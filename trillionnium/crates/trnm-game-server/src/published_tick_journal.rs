use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{mpsc, oneshot, watch};
use uuid::Uuid;

const JOURNAL_CONTRACT: &str = "trnm_published_tick_high_water_v2";
const JOURNAL_OWNER_CONTRACT: &str = "trnm_published_tick_journal_owner_v1";
const JOURNAL_QUEUE_CAPACITY: usize = 128;
const MAX_RECORDS: usize = 10_000;
const MAX_RECORD_BYTES: u64 = 16 * 1024;

/// The last actor state that was allowed to cross the public publication
/// boundary. This is deliberately a local-host recovery record, not a
/// replicated durability claim.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct PublishedTickHighWater {
    pub contract_version: String,
    pub journal_owner_id: Uuid,
    pub instance_id: String,
    pub physical_host_id: String,
    pub match_id: Uuid,
    pub actor_generation: Uuid,
    pub actor_epoch: i64,
    pub tick: u64,
    pub next_sequence: u64,
    pub match_revision: u64,
    pub next_input_sequences: BTreeMap<String, u64>,
    pub phase: String,
    pub receipts_replayable: bool,
    pub snapshot_hash: String,
    pub recorded_at_unix_ms: u64,
}

impl PublishedTickHighWater {
    pub(crate) fn new(
        journal_owner_id: Uuid,
        physical_host_id: String,
        input: PublishedTickRecordInput,
    ) -> Result<Self, String> {
        let recorded_at_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("published-tick journal system clock is invalid: {error}"))?
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        let record = Self {
            contract_version: JOURNAL_CONTRACT.to_string(),
            journal_owner_id,
            instance_id: input.instance_id,
            physical_host_id,
            match_id: input.match_id,
            actor_generation: input.actor_generation,
            actor_epoch: input.actor_epoch,
            tick: input.tick,
            next_sequence: input.next_sequence,
            match_revision: input.match_revision,
            next_input_sequences: input.next_input_sequences,
            phase: input.phase,
            receipts_replayable: input.receipts_replayable,
            snapshot_hash: input.snapshot_hash,
            recorded_at_unix_ms,
        };
        record.validate()?;
        Ok(record)
    }

    fn validate(&self) -> Result<(), String> {
        if self.contract_version != JOURNAL_CONTRACT {
            return Err(format!(
                "unsupported published-tick journal contract {}",
                self.contract_version
            ));
        }
        if self.journal_owner_id.is_nil()
            || self.match_id.is_nil()
            || self.actor_generation.is_nil()
            || self.actor_epoch <= 0
            || !is_portable_identity(&self.instance_id)
            || !is_portable_identity(&self.physical_host_id)
        {
            return Err(
                "published-tick record requires journal/host/instance identity, non-nil match/generation and positive actor epoch".to_string(),
            );
        }
        if self.snapshot_hash.len() != 64
            || !self
                .snapshot_hash
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(
                "published-tick snapshot hash must be 64 lowercase hexadecimal characters"
                    .to_string(),
            );
        }
        if self.next_input_sequences.is_empty()
            || self.next_input_sequences.len() > 64
            || self
                .next_input_sequences
                .keys()
                .any(|player_id| !is_portable_identity(player_id))
        {
            return Err(
                "published-tick input cursors require 1..=64 portable player identities"
                    .to_string(),
            );
        }
        if !matches!(self.phase.as_str(), "running" | "complete") {
            return Err("published-tick phase must be running or complete".to_string());
        }
        if self.phase == "complete" && !self.receipts_replayable {
            return Err(
                "terminal published-tick records must make receipts replayable".to_string(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PublishedTickRecordInput {
    pub instance_id: String,
    pub match_id: Uuid,
    pub actor_generation: Uuid,
    pub actor_epoch: i64,
    pub tick: u64,
    pub next_sequence: u64,
    pub match_revision: u64,
    pub next_input_sequences: BTreeMap<String, u64>,
    pub phase: String,
    pub receipts_replayable: bool,
    pub snapshot_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct JournalOwnerManifest {
    contract_version: String,
    journal_owner_id: Uuid,
    physical_host_id: String,
}

#[derive(Clone)]
pub(crate) struct PublishedTickJournal {
    requests: mpsc::Sender<JournalRequest>,
    records: Arc<Mutex<BTreeMap<Uuid, PublishedTickHighWater>>>,
    owner: Arc<JournalOwnerManifest>,
    failed_closed: Arc<AtomicBool>,
    poison_generation: Arc<AtomicU64>,
    fatal_shutdown: watch::Sender<bool>,
    #[cfg(test)]
    test_pause_next_record: Arc<Mutex<Option<JournalWriterTestPause>>>,
    // The lock is intentionally held for the complete server lifetime. The
    // writer task owns another Arc so dropping a transient handle cannot
    // release it while a write is still in progress.
    _process_lock: Arc<File>,
    _host_identity_lock: Arc<File>,
}

enum JournalRequest {
    Record {
        poison_generation: u64,
        record: Box<PublishedTickHighWater>,
        durable_db_next_sequence: u64,
        durable_db_match_revision: u64,
        durable_db_next_input_sequences: BTreeMap<String, u64>,
        completion: oneshot::Sender<Result<(), String>>,
    },
    Compact {
        poison_generation: u64,
        active_matches: BTreeSet<Uuid>,
        completion: oneshot::Sender<Result<(), String>>,
    },
    Retire {
        poison_generation: u64,
        match_id: Uuid,
        actor_generation: Uuid,
        completion: oneshot::Sender<Result<(), String>>,
    },
}

#[cfg(test)]
struct JournalWriterTestPause {
    entered: oneshot::Sender<()>,
    release: oneshot::Receiver<()>,
}

struct JournalWriterContext {
    root: PathBuf,
    records: Arc<Mutex<BTreeMap<Uuid, PublishedTickHighWater>>>,
    _process_lock: Arc<File>,
    failed_closed: Arc<AtomicBool>,
    poison_generation: Arc<AtomicU64>,
    fatal_shutdown: watch::Sender<bool>,
    #[cfg(test)]
    test_pause_next_record: Arc<Mutex<Option<JournalWriterTestPause>>>,
}

#[derive(Debug)]
enum JournalWriteError {
    Rejected(String),
    DurabilityUncertain(String),
}

impl JournalWriteError {
    fn message(self) -> String {
        match self {
            Self::Rejected(message) | Self::DurabilityUncertain(message) => message,
        }
    }

    fn is_fatal(&self) -> bool {
        matches!(self, Self::DurabilityUncertain(_))
    }
}

impl PublishedTickJournal {
    pub(crate) fn open(root: PathBuf, physical_host_id: String) -> Result<Self, String> {
        let host_identity_lock = acquire_host_identity_lock(&physical_host_id)?;
        ensure_secure_directory(&root)?;
        let process_lock = acquire_process_lock(&root)?;
        let owner = Arc::new(load_or_create_owner(&root, physical_host_id)?);
        let records = Arc::new(Mutex::new(load_records(&root, &owner)?));
        let (requests, receiver) = mpsc::channel(JOURNAL_QUEUE_CAPACITY);
        let failed_closed = Arc::new(AtomicBool::new(false));
        let poison_generation = Arc::new(AtomicU64::new(0));
        let (fatal_shutdown, _) = watch::channel(false);
        #[cfg(test)]
        let test_pause_next_record = Arc::new(Mutex::new(None));
        let worker_records = records.clone();
        let worker_lock = process_lock.clone();
        let worker_failed_closed = failed_closed.clone();
        let worker_poison_generation = poison_generation.clone();
        let worker_fatal_shutdown = fatal_shutdown.clone();
        #[cfg(test)]
        let worker_test_pause = test_pause_next_record.clone();
        tokio::spawn(async move {
            run_writer(
                JournalWriterContext {
                    root,
                    records: worker_records,
                    _process_lock: worker_lock,
                    failed_closed: worker_failed_closed,
                    poison_generation: worker_poison_generation,
                    fatal_shutdown: worker_fatal_shutdown,
                    #[cfg(test)]
                    test_pause_next_record: worker_test_pause,
                },
                receiver,
            )
            .await;
        });
        Ok(Self {
            requests,
            records,
            owner,
            failed_closed,
            poison_generation,
            fatal_shutdown,
            #[cfg(test)]
            test_pause_next_record,
            _process_lock: process_lock,
            _host_identity_lock: host_identity_lock,
        })
    }

    pub(crate) fn new_record(
        &self,
        input: PublishedTickRecordInput,
    ) -> Result<PublishedTickHighWater, String> {
        PublishedTickHighWater::new(
            self.owner.journal_owner_id,
            self.owner.physical_host_id.clone(),
            input,
        )
    }

    pub(crate) fn high_water(
        &self,
        match_id: Uuid,
    ) -> Result<Option<PublishedTickHighWater>, String> {
        self.records
            .lock()
            .map_err(|_| "published-tick journal record lock is poisoned".to_string())
            .map(|records| records.get(&match_id).cloned())
    }

    pub(crate) fn fail_closed(&self) {
        poison_journal(
            &self.failed_closed,
            &self.poison_generation,
            &self.fatal_shutdown,
        );
    }

    pub(crate) fn fatal_shutdown(&self) -> watch::Receiver<bool> {
        self.fatal_shutdown.subscribe()
    }

    pub(crate) fn is_operational(&self) -> bool {
        !self.failed_closed.load(Ordering::Acquire)
    }

    fn operational_generation(&self) -> Result<u64, String> {
        let generation = self.poison_generation.load(Ordering::Acquire);
        if request_generation_is_operational(
            &self.failed_closed,
            &self.poison_generation,
            generation,
        ) {
            Ok(generation)
        } else {
            Err("published-tick journal is failed closed after uncertain durability".to_string())
        }
    }

    fn require_generation(&self, generation: u64) -> Result<(), String> {
        if request_generation_is_operational(
            &self.failed_closed,
            &self.poison_generation,
            generation,
        ) {
            Ok(())
        } else {
            Err("published-tick journal request crossed a failed-closed generation".to_string())
        }
    }

    #[cfg(test)]
    fn pause_next_record_for_test(&self) -> (oneshot::Receiver<()>, oneshot::Sender<()>) {
        let (entered_tx, entered_rx) = oneshot::channel();
        let (release_tx, release_rx) = oneshot::channel();
        *self
            .test_pause_next_record
            .lock()
            .expect("journal test pause lock") = Some(JournalWriterTestPause {
            entered: entered_tx,
            release: release_rx,
        });
        (entered_rx, release_tx)
    }

    pub(crate) async fn record(
        &self,
        record: PublishedTickHighWater,
        durable_db_next_sequence: u64,
        durable_db_match_revision: u64,
        durable_db_next_input_sequences: BTreeMap<String, u64>,
    ) -> Result<(), String> {
        let poison_generation = self.operational_generation()?;
        if record.journal_owner_id != self.owner.journal_owner_id
            || record.physical_host_id != self.owner.physical_host_id
        {
            return Err("published-tick record does not belong to this host journal".to_string());
        }
        if record.next_sequence > durable_db_next_sequence
            || record.match_revision > durable_db_match_revision
        {
            return Err(
                "published-tick high-water cursor is ahead of durable database authority"
                    .to_string(),
            );
        }
        let (completion, completed) = oneshot::channel();
        if self
            .requests
            .send(JournalRequest::Record {
                poison_generation,
                record: Box::new(record),
                durable_db_next_sequence,
                durable_db_match_revision,
                durable_db_next_input_sequences,
                completion,
            })
            .await
            .is_err()
        {
            self.require_generation(poison_generation)?;
            return Err("published-tick journal writer is unavailable".to_string());
        }
        let result = match completed.await {
            Ok(result) => result,
            Err(_) => {
                self.require_generation(poison_generation)?;
                return Err(
                    "published-tick journal writer stopped before acknowledgement".to_string(),
                );
            }
        };
        self.require_generation(poison_generation)?;
        result
    }

    pub(crate) async fn compact_to_active(
        &self,
        active_matches: BTreeSet<Uuid>,
    ) -> Result<(), String> {
        let poison_generation = self.operational_generation()?;
        let (completion, completed) = oneshot::channel();
        if self
            .requests
            .send(JournalRequest::Compact {
                poison_generation,
                active_matches,
                completion,
            })
            .await
            .is_err()
        {
            self.require_generation(poison_generation)?;
            return Err("published-tick journal writer is unavailable".to_string());
        }
        let result = match completed.await {
            Ok(result) => result,
            Err(_) => {
                self.require_generation(poison_generation)?;
                return Err("published-tick journal writer stopped during compaction".to_string());
            }
        };
        self.require_generation(poison_generation)?;
        result
    }

    pub(crate) fn recorded_match_ids(&self) -> Result<Vec<Uuid>, String> {
        self.records
            .lock()
            .map_err(|_| "published-tick journal record lock is poisoned".to_string())
            .map(|records| records.keys().copied().collect())
    }

    pub(crate) async fn retire(
        &self,
        match_id: Uuid,
        actor_generation: Uuid,
    ) -> Result<(), String> {
        let poison_generation = self.operational_generation()?;
        let (completion, completed) = oneshot::channel();
        if self
            .requests
            .send(JournalRequest::Retire {
                poison_generation,
                match_id,
                actor_generation,
                completion,
            })
            .await
            .is_err()
        {
            self.require_generation(poison_generation)?;
            return Err("published-tick journal writer is unavailable".to_string());
        }
        let result = match completed.await {
            Ok(result) => result,
            Err(_) => {
                self.require_generation(poison_generation)?;
                return Err("published-tick journal writer stopped during retirement".to_string());
            }
        };
        self.require_generation(poison_generation)?;
        result
    }
}

fn poison_journal(
    failed_closed: &AtomicBool,
    poison_generation: &AtomicU64,
    fatal_shutdown: &watch::Sender<bool>,
) {
    if !failed_closed.swap(true, Ordering::AcqRel) {
        poison_generation.fetch_add(1, Ordering::AcqRel);
        fatal_shutdown.send_replace(true);
    }
}

fn request_generation_is_operational(
    failed_closed: &AtomicBool,
    poison_generation: &AtomicU64,
    request_generation: u64,
) -> bool {
    !failed_closed.load(Ordering::Acquire)
        && poison_generation.load(Ordering::Acquire) == request_generation
}

async fn run_writer(context: JournalWriterContext, mut requests: mpsc::Receiver<JournalRequest>) {
    let JournalWriterContext {
        root,
        records,
        _process_lock,
        failed_closed,
        poison_generation,
        fatal_shutdown,
        #[cfg(test)]
        test_pause_next_record,
    } = context;
    while let Some(request) = requests.recv().await {
        match request {
            JournalRequest::Record {
                poison_generation: request_generation,
                record,
                durable_db_next_sequence,
                durable_db_match_revision,
                durable_db_next_input_sequences,
                completion,
            } => {
                if !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                ) {
                    let _ = completion.send(Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    ));
                    break;
                }
                #[cfg(test)]
                let test_pause = {
                    test_pause_next_record
                        .lock()
                        .expect("journal test pause lock")
                        .take()
                };
                #[cfg(test)]
                if let Some(pause) = test_pause {
                    let _ = pause.entered.send(());
                    let _ = pause.release.await;
                }
                if !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                ) {
                    let _ = completion.send(Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    ));
                    break;
                }
                let worker_root = root.clone();
                let worker_records = records.clone();
                let result = tokio::task::spawn_blocking(move || {
                    persist_record(
                        &worker_root,
                        &worker_records,
                        *record,
                        durable_db_next_sequence,
                        durable_db_match_revision,
                        durable_db_next_input_sequences,
                    )
                })
                .await
                .unwrap_or_else(|error| {
                    Err(JournalWriteError::DurabilityUncertain(format!(
                        "published-tick journal writer task panicked or was cancelled: {error}"
                    )))
                });
                let fatal = result.as_ref().is_err_and(JournalWriteError::is_fatal);
                if fatal {
                    poison_journal(&failed_closed, &poison_generation, &fatal_shutdown);
                }
                let crossed_generation = !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                );
                let completion_result = if crossed_generation {
                    Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    )
                } else {
                    result.map_err(JournalWriteError::message)
                };
                let _ = completion.send(completion_result);
                if fatal || crossed_generation {
                    break;
                }
            }
            JournalRequest::Compact {
                poison_generation: request_generation,
                active_matches,
                completion,
            } => {
                if !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                ) {
                    let _ = completion.send(Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    ));
                    break;
                }
                let worker_root = root.clone();
                let worker_records = records.clone();
                let result = tokio::task::spawn_blocking(move || {
                    compact_records(&worker_root, &worker_records, &active_matches)
                })
                .await
                .unwrap_or_else(|error| {
                    Err(JournalWriteError::DurabilityUncertain(format!(
                        "published-tick journal compaction task panicked or was cancelled: {error}"
                    )))
                });
                let fatal = result.as_ref().is_err_and(JournalWriteError::is_fatal);
                if fatal {
                    poison_journal(&failed_closed, &poison_generation, &fatal_shutdown);
                }
                let crossed_generation = !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                );
                let completion_result = if crossed_generation {
                    Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    )
                } else {
                    result.map_err(JournalWriteError::message)
                };
                let _ = completion.send(completion_result);
                if fatal || crossed_generation {
                    break;
                }
            }
            JournalRequest::Retire {
                poison_generation: request_generation,
                match_id,
                actor_generation,
                completion,
            } => {
                if !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                ) {
                    let _ = completion.send(Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    ));
                    break;
                }
                let worker_root = root.clone();
                let worker_records = records.clone();
                let result = tokio::task::spawn_blocking(move || {
                    retire_record(&worker_root, &worker_records, match_id, actor_generation)
                })
                .await
                .unwrap_or_else(|error| {
                    Err(JournalWriteError::DurabilityUncertain(format!(
                        "published-tick retirement task panicked or was cancelled: {error}"
                    )))
                });
                let fatal = result.as_ref().is_err_and(JournalWriteError::is_fatal);
                if fatal {
                    poison_journal(&failed_closed, &poison_generation, &fatal_shutdown);
                }
                let crossed_generation = !request_generation_is_operational(
                    &failed_closed,
                    &poison_generation,
                    request_generation,
                );
                let completion_result = if crossed_generation {
                    Err(
                        "published-tick journal request crossed a failed-closed generation"
                            .to_string(),
                    )
                } else {
                    result.map_err(JournalWriteError::message)
                };
                let _ = completion.send(completion_result);
                if fatal || crossed_generation {
                    break;
                }
            }
        }
    }
}

fn persist_record(
    root: &Path,
    records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
    record: PublishedTickHighWater,
    durable_db_next_sequence: u64,
    durable_db_match_revision: u64,
    durable_db_next_input_sequences: BTreeMap<String, u64>,
) -> Result<(), JournalWriteError> {
    record.validate().map_err(JournalWriteError::Rejected)?;
    if record.next_sequence > durable_db_next_sequence
        || record.match_revision > durable_db_match_revision
    {
        return Err(JournalWriteError::Rejected(
            "published-tick high-water cursor is ahead of durable database authority".to_string(),
        ));
    }
    if record
        .next_input_sequences
        .iter()
        .any(|(player_id, cursor)| {
            durable_db_next_input_sequences
                .get(player_id)
                .is_none_or(|durable| cursor > durable)
        })
    {
        return Err(JournalWriteError::Rejected(
            "published-tick member cursor is ahead of durable database authority".to_string(),
        ));
    }
    let current = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .get(&record.match_id)
        .cloned();
    if let Some(current) = current.as_ref() {
        validate_forward_progress(current, &record).map_err(JournalWriteError::Rejected)?;
    } else if records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .len()
        >= MAX_RECORDS
    {
        return Err(JournalWriteError::Rejected(format!(
            "published-tick journal reached its bounded {MAX_RECORDS}-match retention limit"
        )));
    }

    let payload = serde_json::to_vec_pretty(&record).map_err(|error| {
        JournalWriteError::Rejected(format!("encode published-tick high-water: {error}"))
    })?;
    if payload.len() as u64 > MAX_RECORD_BYTES {
        return Err(JournalWriteError::Rejected(format!(
            "published-tick record exceeds {MAX_RECORD_BYTES} bytes"
        )));
    }
    atomic_install(root, &record_path(root, record.match_id), &payload)?;
    records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .insert(record.match_id, record);
    Ok(())
}

fn validate_forward_progress(
    current: &PublishedTickHighWater,
    next: &PublishedTickHighWater,
) -> Result<(), String> {
    if next.journal_owner_id != current.journal_owner_id
        || next.physical_host_id != current.physical_host_id
    {
        return Err("published-tick record crossed its host journal identity".to_string());
    }
    if (next.instance_id == current.instance_id && next.actor_epoch < current.actor_epoch)
        || next.tick < current.tick
        || next.next_sequence < current.next_sequence
        || next.match_revision < current.match_revision
        || current
            .next_input_sequences
            .iter()
            .any(|(player_id, cursor)| {
                next.next_input_sequences
                    .get(player_id)
                    .is_none_or(|next_cursor| next_cursor < cursor)
            })
        || (current.phase == "complete" && next.phase != "complete")
    {
        return Err("published-tick high-water cannot regress".to_string());
    }
    if (next.instance_id != current.instance_id
        || next.actor_generation != current.actor_generation)
        && (next.tick != current.tick
            || next.next_sequence != current.next_sequence
            || next.match_revision != current.match_revision
            || next.next_input_sequences != current.next_input_sequences
            || next.phase != current.phase
            || next.receipts_replayable != current.receipts_replayable
            || next.snapshot_hash != current.snapshot_hash)
    {
        return Err(
            "a replacement actor generation must first adopt the exact durable high-water"
                .to_string(),
        );
    }
    if next.tick == current.tick
        && next.next_sequence == current.next_sequence
        && next.match_revision == current.match_revision
        && next.next_input_sequences == current.next_input_sequences
        && next.phase == current.phase
        && next.snapshot_hash != current.snapshot_hash
    {
        return Err("published-tick hash changed without cursor or tick progress".to_string());
    }
    Ok(())
}

fn compact_records(
    root: &Path,
    records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
    active_matches: &BTreeSet<Uuid>,
) -> Result<(), JournalWriteError> {
    let stale = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .keys()
        .filter(|match_id| !active_matches.contains(match_id))
        .copied()
        .collect::<Vec<_>>();
    for match_id in &stale {
        let path = record_path(root, *match_id);
        match fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(JournalWriteError::DurabilityUncertain(format!(
                    "remove stale published-tick record {}: {error}",
                    path.display()
                )));
            }
        }
    }
    if !stale.is_empty() {
        sync_directory(root).map_err(JournalWriteError::DurabilityUncertain)?;
        let mut records = records.lock().map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?;
        for match_id in stale {
            records.remove(&match_id);
        }
    }
    Ok(())
}

fn retire_record(
    root: &Path,
    records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
    match_id: Uuid,
    actor_generation: Uuid,
) -> Result<(), JournalWriteError> {
    let current = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .get(&match_id)
        .cloned();
    let Some(current) = current else {
        return Ok(());
    };
    if current.actor_generation != actor_generation {
        return Err(JournalWriteError::Rejected(
            "published-tick retirement cannot remove a replacement actor generation".to_string(),
        ));
    }
    if current.phase != "complete" || !current.receipts_replayable {
        return Err(JournalWriteError::Rejected(
            "published-tick retirement requires an acknowledged terminal record".to_string(),
        ));
    }
    let path = record_path(root, match_id);
    match fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "remove terminal published-tick record {}: {error}",
                path.display()
            )));
        }
    }
    sync_directory(root).map_err(JournalWriteError::DurabilityUncertain)?;
    records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .remove(&match_id);
    Ok(())
}

fn load_records(
    root: &Path,
    owner: &JournalOwnerManifest,
) -> Result<BTreeMap<Uuid, PublishedTickHighWater>, String> {
    let mut records = BTreeMap::new();
    for entry in fs::read_dir(root)
        .map_err(|error| format!("read published-tick journal {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("read published-tick entry: {error}"))?;
        let path = entry.path();
        let name = match path.file_name().and_then(|value| value.to_str()) {
            Some(name) => name,
            None => return Err("published-tick journal contains a non-UTF8 filename".to_string()),
        };
        if name == ".published-tick.lock"
            || name == ".published-tick-owner.json"
            || name.starts_with(".published-")
        {
            continue;
        }
        if !name.starts_with("published-") || !name.ends_with(".json") {
            return Err(format!(
                "published-tick journal contains unexpected entry {}",
                path.display()
            ));
        }
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("inspect {}: {error}", path.display()))?;
        validate_private_regular_file(&path, &metadata, "published-tick record")?;
        if metadata.len() > MAX_RECORD_BYTES {
            return Err(format!(
                "published-tick record {} exceeds {MAX_RECORD_BYTES} bytes",
                path.display()
            ));
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        File::open(&path)
            .and_then(|mut file| file.read_to_end(&mut bytes))
            .map_err(|error| format!("read {}: {error}", path.display()))?;
        let record: PublishedTickHighWater = serde_json::from_slice(&bytes)
            .map_err(|error| format!("decode published-tick record {}: {error}", path.display()))?;
        record.validate()?;
        if record.journal_owner_id != owner.journal_owner_id
            || record.physical_host_id != owner.physical_host_id
        {
            return Err(format!(
                "published-tick record {} belongs to a different host journal",
                path.display()
            ));
        }
        if path != record_path(root, record.match_id) {
            return Err(format!(
                "published-tick record filename does not match its match id: {}",
                path.display()
            ));
        }
        if records.insert(record.match_id, record).is_some() {
            return Err("duplicate published-tick match record".to_string());
        }
        if records.len() > MAX_RECORDS {
            return Err(format!(
                "published-tick journal exceeds its {MAX_RECORDS}-match retention limit"
            ));
        }
    }
    Ok(records)
}

fn load_or_create_owner(
    root: &Path,
    physical_host_id: String,
) -> Result<JournalOwnerManifest, String> {
    if !is_portable_identity(&physical_host_id) {
        return Err("physical_host_id is not a bounded portable identity".to_string());
    }
    let path = root.join(".published-tick-owner.json");
    match fs::symlink_metadata(&path) {
        Ok(metadata) => {
            validate_private_regular_file(&path, &metadata, "published-tick owner manifest")?;
            if metadata.len() > MAX_RECORD_BYTES {
                return Err("published-tick owner manifest is oversized".to_string());
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("read published-tick owner manifest: {error}"))?;
            let owner: JournalOwnerManifest = serde_json::from_slice(&bytes)
                .map_err(|error| format!("decode published-tick owner manifest: {error}"))?;
            if owner.contract_version != JOURNAL_OWNER_CONTRACT
                || owner.journal_owner_id.is_nil()
                || !is_portable_identity(&owner.physical_host_id)
            {
                return Err("published-tick owner manifest is invalid".to_string());
            }
            if owner.physical_host_id != physical_host_id {
                return Err(
                    "published-tick journal belongs to a different physical host identity"
                        .to_string(),
                );
            }
            Ok(owner)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let owner = JournalOwnerManifest {
                contract_version: JOURNAL_OWNER_CONTRACT.to_string(),
                journal_owner_id: Uuid::new_v4(),
                physical_host_id,
            };
            let payload = serde_json::to_vec_pretty(&owner)
                .map_err(|error| format!("encode published-tick owner manifest: {error}"))?;
            atomic_install(root, &path, &payload).map_err(JournalWriteError::message)?;
            Ok(owner)
        }
        Err(error) => Err(format!(
            "inspect published-tick owner manifest {}: {error}",
            path.display()
        )),
    }
}

fn is_portable_identity(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
}

fn record_path(root: &Path, match_id: Uuid) -> PathBuf {
    root.join(format!("published-{match_id}.json"))
}

fn atomic_install(root: &Path, target: &Path, payload: &[u8]) -> Result<(), JournalWriteError> {
    let temp = root.join(format!(
        ".published-{}.tmp-{}",
        std::process::id(),
        Uuid::new_v4()
    ));
    let result = (|| {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&temp).map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!("create {}: {error}", temp.display()))
        })?;
        file.write_all(payload).map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!("write {}: {error}", temp.display()))
        })?;
        file.sync_all().map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!("sync {}: {error}", temp.display()))
        })?;
        fs::rename(&temp, target).map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!(
                "atomically install published-tick record {}: {error}",
                target.display()
            ))
        })?;
        sync_directory(root).map_err(JournalWriteError::DurabilityUncertain)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

fn sync_directory(path: &Path) -> Result<(), String> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("sync published-tick directory {}: {error}", path.display()))
}

fn acquire_process_lock(root: &Path) -> Result<Arc<File>, String> {
    let path = root.join(".published-tick.lock");
    if let Ok(metadata) = fs::symlink_metadata(&path) {
        validate_private_regular_file(&path, &metadata, "published-tick process lock")?;
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&path).map_err(|error| {
        format!(
            "open published-tick process lock {}: {error}",
            path.display()
        )
    })?;
    validate_open_file_identity(&path, &file, "published-tick process lock")?;
    file.try_lock().map_err(|error| {
        format!(
            "published-tick journal {} is already owned by another server process: {error}",
            root.display()
        )
    })?;
    Ok(Arc::new(file))
}

fn acquire_host_identity_lock(physical_host_id: &str) -> Result<Arc<File>, String> {
    if !is_portable_identity(physical_host_id) {
        return Err("physical_host_id is not a bounded portable identity".to_string());
    }
    let digest = Sha256::digest(physical_host_id.as_bytes());
    let mut digest_hex = String::with_capacity(64);
    for byte in digest {
        write!(&mut digest_hex, "{byte:02x}")
            .expect("writing a SHA-256 digest into a String cannot fail");
    }
    #[cfg(unix)]
    let lock_root = PathBuf::from("/tmp");
    #[cfg(not(unix))]
    let lock_root = std::env::temp_dir();
    let path = lock_root.join(format!(".trnm-published-tick-host-{digest_hex}.lock"));
    if let Ok(metadata) = fs::symlink_metadata(&path) {
        validate_private_regular_file(&path, &metadata, "published-tick host identity lock")?;
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&path).map_err(|error| {
        format!(
            "open published-tick host identity lock {}: {error}",
            path.display()
        )
    })?;
    validate_open_file_identity(&path, &file, "published-tick host identity lock")?;
    file.try_lock().map_err(|error| {
        format!(
            "physical host identity {physical_host_id} already owns another published-tick journal directory: {error}"
        )
    })?;
    Ok(Arc::new(file))
}

fn ensure_secure_directory(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
                return Err(format!(
                    "published-tick journal {} is not a real directory",
                    path.display()
                ));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut builder = fs::DirBuilder::new();
            builder.recursive(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            builder
                .create(path)
                .map_err(|error| format!("create {}: {error}", path.display()))?;
        }
        Err(error) => return Err(format!("inspect {}: {error}", path.display())),
    }
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("inspect {} after creation: {error}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return Err(format!(
            "published-tick journal {} is not a real directory",
            path.display()
        ));
    }
    validate_owner(path, &metadata, "published-tick journal directory")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o700 {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                .map_err(|error| format!("set {} mode 0700: {error}", path.display()))?;
            let repaired = fs::symlink_metadata(path)
                .map_err(|error| format!("reinspect {}: {error}", path.display()))?;
            if repaired.file_type().is_symlink()
                || !repaired.file_type().is_dir()
                || repaired.uid() != effective_user_id()
                || repaired.permissions().mode() & 0o777 != 0o700
            {
                return Err(format!(
                    "published-tick journal {} did not converge to owner mode 0700",
                    path.display()
                ));
            }
            sync_directory(path)?;
        }
    }
    Ok(())
}

fn validate_open_file_identity(path: &Path, file: &File, label: &str) -> Result<(), String> {
    let path_metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("inspect {label} {}: {error}", path.display()))?;
    validate_private_regular_file(path, &path_metadata, label)?;
    let file_metadata = file
        .metadata()
        .map_err(|error| format!("inspect opened {label} {}: {error}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if path_metadata.dev() != file_metadata.dev() || path_metadata.ino() != file_metadata.ino()
        {
            return Err(format!("{label} {} changed while opening", path.display()));
        }
    }
    Ok(())
}

fn validate_private_regular_file(
    path: &Path,
    metadata: &fs::Metadata,
    label: &str,
) -> Result<(), String> {
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(format!("{label} {} is not a regular file", path.display()));
    }
    validate_owner(path, metadata, label)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o600 {
            return Err(format!(
                "{label} {} must have mode 0600, found {mode:04o}",
                path.display()
            ));
        }
    }
    Ok(())
}

#[cfg(unix)]
fn validate_owner(path: &Path, metadata: &fs::Metadata, label: &str) -> Result<(), String> {
    use std::os::unix::fs::MetadataExt;
    if metadata.uid() != effective_user_id() {
        return Err(format!("{label} {} has a different owner", path.display()));
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_owner(_path: &Path, _metadata: &fs::Metadata, _label: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(unix)]
fn effective_user_id() -> u32 {
    unsafe extern "C" {
        fn geteuid() -> u32;
    }
    // SAFETY: geteuid has no preconditions and returns the current process uid.
    unsafe { geteuid() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "trnm-published-tick-{label}-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ))
    }

    fn input_cursors() -> BTreeMap<String, u64> {
        BTreeMap::from([("player-a".to_string(), 2), ("player-b".to_string(), 2)])
    }

    fn owner(root: &Path, physical_host_id: &str) -> JournalOwnerManifest {
        ensure_secure_directory(root).unwrap();
        load_or_create_owner(root, physical_host_id.to_string()).unwrap()
    }

    fn unique_host(label: &str) -> String {
        format!("host-{label}-{}", Uuid::new_v4())
    }

    fn record(
        owner: &JournalOwnerManifest,
        instance_id: &str,
        match_id: Uuid,
        generation: Uuid,
        epoch: i64,
        tick: u64,
    ) -> PublishedTickHighWater {
        PublishedTickHighWater::new(
            owner.journal_owner_id,
            owner.physical_host_id.clone(),
            PublishedTickRecordInput {
                instance_id: instance_id.to_string(),
                match_id,
                actor_generation: generation,
                actor_epoch: epoch,
                tick,
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
                phase: "running".to_string(),
                receipts_replayable: true,
                snapshot_hash: format!("{:064x}", tick),
            },
        )
        .unwrap()
    }

    fn record_input(
        instance_id: &str,
        match_id: Uuid,
        generation: Uuid,
        epoch: i64,
        tick: u64,
        phase: &str,
    ) -> PublishedTickRecordInput {
        PublishedTickRecordInput {
            instance_id: instance_id.to_string(),
            match_id,
            actor_generation: generation,
            actor_epoch: epoch,
            tick,
            next_sequence: 4,
            match_revision: 5,
            next_input_sequences: input_cursors(),
            phase: phase.to_string(),
            receipts_replayable: true,
            snapshot_hash: format!("{:064x}", tick),
        }
    }

    #[test]
    fn crash_reopen_recovers_exact_high_water() {
        let root = temp_dir("reopen");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let match_id = Uuid::new_v4();
        let expected = record(&owner, "instance-a", match_id, Uuid::new_v4(), 7, 91);
        persist_record(&root, &records, expected.clone(), 4, 5, input_cursors()).unwrap();

        let reopened = load_records(&root, &owner).unwrap();
        assert_eq!(reopened.get(&match_id), Some(&expected));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn corruption_fails_closed_on_reopen() {
        let root = temp_dir("corrupt");
        let owner = owner(&root, "host-a");
        let path = record_path(&root, Uuid::new_v4());
        atomic_install(&root, &path, b"{not-json").unwrap();
        let error = load_records(&root, &owner).unwrap_err();
        assert!(error.contains("decode published-tick record"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn tick_or_cursor_rollback_is_rejected_without_replacing_record() {
        let root = temp_dir("rollback");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let match_id = Uuid::new_v4();
        let generation = Uuid::new_v4();
        let current = record(&owner, "instance-a", match_id, generation, 3, 100);
        persist_record(&root, &records, current.clone(), 4, 5, input_cursors()).unwrap();

        let rollback = record(&owner, "instance-a", match_id, generation, 3, 99);
        assert!(
            persist_record(&root, &records, rollback, 4, 5, input_cursors())
                .unwrap_err()
                .message()
                .contains("cannot regress")
        );
        assert_eq!(
            load_records(&root, &owner).unwrap().get(&match_id),
            Some(&current)
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cursor_beyond_database_authority_is_rejected() {
        let root = temp_dir("db-cursor");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let mut ahead = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 2, 10);
        ahead.next_sequence = 9;
        assert!(
            persist_record(&root, &records, ahead, 8, 5, input_cursors())
                .unwrap_err()
                .message()
                .contains("ahead of durable database")
        );
        assert!(records.lock().unwrap().is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn generation_replacement_must_adopt_exact_high_water() {
        let root = temp_dir("generation");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let match_id = Uuid::new_v4();
        let current = record(&owner, "instance-a", match_id, Uuid::new_v4(), 9, 50);
        persist_record(&root, &records, current.clone(), 4, 5, input_cursors()).unwrap();

        let adopted = PublishedTickHighWater::new(
            owner.journal_owner_id,
            owner.physical_host_id.clone(),
            PublishedTickRecordInput {
                instance_id: "instance-b".to_string(),
                match_id,
                actor_generation: Uuid::new_v4(),
                actor_epoch: 1,
                tick: current.tick,
                next_sequence: current.next_sequence,
                match_revision: current.match_revision,
                next_input_sequences: current.next_input_sequences.clone(),
                phase: current.phase.clone(),
                receipts_replayable: current.receipts_replayable,
                snapshot_hash: current.snapshot_hash.clone(),
            },
        )
        .unwrap();
        persist_record(&root, &records, adopted, 4, 5, input_cursors()).unwrap();

        let advanced = record(&owner, "instance-c", match_id, Uuid::new_v4(), 20, 51);
        assert!(
            persist_record(&root, &records, advanced, 4, 5, input_cursors())
                .unwrap_err()
                .message()
                .contains("must first adopt")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn second_server_process_owner_is_rejected() {
        let root = temp_dir("process-lock");
        let host = unique_host("process-lock");
        let first = PublishedTickJournal::open(root.clone(), host.clone()).unwrap();
        let error = match PublishedTickJournal::open(root.clone(), host) {
            Ok(_) => panic!("second journal owner unexpectedly acquired the process lock"),
            Err(error) => error,
        };
        assert!(error.contains("already owns another published-tick journal directory"));
        drop(first);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn same_physical_host_cannot_open_two_different_journal_directories() {
        let first_root = temp_dir("host-lock-first");
        let second_root = temp_dir("host-lock-second");
        let host = unique_host("directory-lock");
        let first = PublishedTickJournal::open(first_root.clone(), host.clone()).unwrap();
        let error = match PublishedTickJournal::open(second_root.clone(), host) {
            Ok(_) => panic!("same host identity unexpectedly opened a second journal root"),
            Err(error) => error,
        };
        assert!(error.contains("already owns another published-tick journal directory"));
        drop(first);
        fs::remove_dir_all(first_root).unwrap();
        if second_root.exists() {
            fs::remove_dir_all(second_root).unwrap();
        }
    }

    #[tokio::test]
    async fn logical_rejection_does_not_poison_other_journal_writes() {
        let root = temp_dir("logical-isolation");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("logical-isolation")).unwrap();
        let match_id = Uuid::new_v4();
        let generation = Uuid::new_v4();
        let first = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                generation,
                1,
                10,
                "running",
            ))
            .unwrap();
        journal.record(first, 4, 5, input_cursors()).await.unwrap();
        let rollback = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                generation,
                1,
                9,
                "running",
            ))
            .unwrap();
        assert!(journal
            .record(rollback, 4, 5, input_cursors())
            .await
            .unwrap_err()
            .contains("cannot regress"));
        let next_match = Uuid::new_v4();
        let later = journal
            .new_record(record_input(
                "instance-a",
                next_match,
                Uuid::new_v4(),
                1,
                1,
                "running",
            ))
            .unwrap();
        journal.record(later, 4, 5, input_cursors()).await.unwrap();
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn terminal_acknowledgement_is_retired_with_directory_durability() {
        let root = temp_dir("retire");
        let journal = PublishedTickJournal::open(root.clone(), unique_host("retire")).unwrap();
        let match_id = Uuid::new_v4();
        let generation = Uuid::new_v4();
        let terminal = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                generation,
                1,
                10,
                "complete",
            ))
            .unwrap();
        journal
            .record(terminal, 4, 5, input_cursors())
            .await
            .unwrap();
        journal.retire(match_id, generation).await.unwrap();
        assert!(journal.high_water(match_id).unwrap().is_none());
        assert!(!record_path(&root, match_id).exists());
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn poison_generation_rejects_blocked_and_already_queued_records() {
        let root = temp_dir("poison-generation");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("poison-generation")).unwrap();
        let first_match = Uuid::new_v4();
        let second_match = Uuid::new_v4();
        let first = journal
            .new_record(record_input(
                "instance-a",
                first_match,
                Uuid::new_v4(),
                1,
                1,
                "running",
            ))
            .unwrap();
        let second = journal
            .new_record(record_input(
                "instance-a",
                second_match,
                Uuid::new_v4(),
                1,
                1,
                "running",
            ))
            .unwrap();
        let (entered, release) = journal.pause_next_record_for_test();
        let fatal_shutdown = journal.fatal_shutdown();
        let first_journal = journal.clone();
        let first_task =
            tokio::spawn(async move { first_journal.record(first, 4, 5, input_cursors()).await });
        entered.await.unwrap();
        let second_journal = journal.clone();
        let second_task =
            tokio::spawn(async move { second_journal.record(second, 4, 5, input_cursors()).await });
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while journal.requests.capacity() == JOURNAL_QUEUE_CAPACITY {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        journal.fail_closed();
        assert!(*fatal_shutdown.borrow());
        release.send(()).unwrap();
        let first_error = first_task.await.unwrap().unwrap_err();
        let second_error = second_task.await.unwrap().unwrap_err();
        assert!(first_error.contains("failed-closed") || first_error.contains("failed closed"));
        assert!(second_error.contains("failed-closed") || second_error.contains("failed closed"));
        assert!(journal.high_water(first_match).unwrap().is_none());
        assert!(journal.high_water(second_match).unwrap().is_none());
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn uncertain_filesystem_failure_closes_the_global_writer() {
        use std::os::unix::fs::PermissionsExt;

        let root = temp_dir("fatal-io");
        let journal = PublishedTickJournal::open(root.clone(), unique_host("fatal-io")).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o500)).unwrap();
        let failed = journal
            .new_record(record_input(
                "instance-a",
                Uuid::new_v4(),
                Uuid::new_v4(),
                1,
                1,
                "running",
            ))
            .unwrap();
        let first_error = journal
            .record(failed, 4, 5, input_cursors())
            .await
            .unwrap_err();
        assert!(
            first_error.contains("create")
                || first_error.contains("install")
                || first_error.contains("failed closed")
                || first_error.contains("failed-closed")
        );
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        let later = journal
            .new_record(record_input(
                "instance-a",
                Uuid::new_v4(),
                Uuid::new_v4(),
                1,
                1,
                "running",
            ))
            .unwrap();
        assert!(journal
            .record(later, 4, 5, input_cursors())
            .await
            .unwrap_err()
            .contains("failed closed"));
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn host_owner_manifest_rejects_a_different_physical_host() {
        let root = temp_dir("host-owner");
        owner(&root, "host-a");
        let error = load_or_create_owner(&root, "host-b".to_string()).unwrap_err();
        assert!(error.contains("different physical host identity"));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn insecure_record_mode_fails_closed_on_reopen() {
        use std::os::unix::fs::PermissionsExt;

        let root = temp_dir("mode");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let match_id = Uuid::new_v4();
        persist_record(
            &root,
            &records,
            record(&owner, "instance-a", match_id, Uuid::new_v4(), 1, 1),
            4,
            5,
            input_cursors(),
        )
        .unwrap();
        let path = record_path(&root, match_id);
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(load_records(&root, &owner)
            .unwrap_err()
            .contains("must have mode 0600"));
        fs::remove_dir_all(root).unwrap();
    }
}
