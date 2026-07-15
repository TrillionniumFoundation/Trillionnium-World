use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
const ACK_TOMBSTONE_CONTRACT: &str = "trnm_published_tick_ack_tombstone_v2";
const ABANDONMENT_TOMBSTONE_CONTRACT: &str = "trnm_published_tick_abandonment_tombstone_v1";
const LEGACY_ACK_MANIFEST_CONTRACT: &str = "trnm_published_tick_ack_manifest_v1";
const COLD_WITNESS_MANIFEST_CONTRACT: &str = "trnm_published_tick_cold_witness_manifest_v2";
const JOURNAL_QUEUE_CAPACITY: usize = 128;
const MAX_RECORDS: usize = 10_000;
const MAX_JOURNAL_ROOT_ENTRIES: usize = MAX_RECORDS + 16;
const MAX_RECORD_BYTES: u64 = 16 * 1024;
const MAX_ACK_TOMBSTONE_BYTES: u64 = 24 * 1024;
const MAX_ABANDONMENT_TOMBSTONE_BYTES: u64 = 24 * 1024;
const MAX_ACK_MANIFEST_BYTES: u64 = 32 * 1024;
#[cfg(test)]
const MAX_ACK_TOMBSTONE_PAGE_SIZE: usize = 512;
const MAX_ACK_TOMBSTONES_PER_SHARD: usize = 4_096;
const ACK_TOMBSTONE_DIRECTORY: &str = "acknowledged";
const ABANDONMENT_TOMBSTONE_DIRECTORY: &str = "abandoned";
const ACK_MANIFEST_FILE: &str = ".published-tick-ack-manifest.json";

/// The last actor state that was allowed to cross the public publication
/// boundary. This is deliberately a local-host recovery record, not a
/// replicated durability claim.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Debug)]
struct DurableDatabaseHighWater {
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: BTreeMap<String, u64>,
}

/// Durable cold evidence that an exact terminal high-water was committed to
/// PostgreSQL and acknowledged. Unlike a hot high-water, this record is never
/// rewritten by actor progress and is not subject to `MAX_RECORDS`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PublishedTickAckTombstone {
    pub contract_version: String,
    pub journal_seal_sequence: u64,
    pub high_water: PublishedTickHighWater,
    pub result_hash: String,
    pub settlement_state: String,
    pub acknowledged_at_unix_ms: u64,
    pub database_system_identifier: String,
    pub database_timeline_id: u32,
    pub database_wal_lsn: String,
}

#[derive(Clone, Debug)]
pub(crate) struct PublishedTickAckTombstoneInput {
    pub high_water: PublishedTickHighWater,
    pub result_hash: String,
    pub settlement_state: String,
    pub acknowledged_at_unix_ms: u64,
    pub database_system_identifier: String,
    pub database_timeline_id: u32,
    pub database_wal_lsn: String,
}

impl PublishedTickAckTombstone {
    fn new(
        input: PublishedTickAckTombstoneInput,
        journal_seal_sequence: u64,
    ) -> Result<Self, String> {
        let tombstone = Self {
            contract_version: ACK_TOMBSTONE_CONTRACT.to_string(),
            journal_seal_sequence,
            high_water: input.high_water,
            result_hash: input.result_hash,
            settlement_state: input.settlement_state,
            acknowledged_at_unix_ms: input.acknowledged_at_unix_ms,
            database_system_identifier: input.database_system_identifier,
            database_timeline_id: input.database_timeline_id,
            database_wal_lsn: input.database_wal_lsn,
        };
        tombstone.validate()?;
        Ok(tombstone)
    }

    fn validate(&self) -> Result<(), String> {
        if self.contract_version != ACK_TOMBSTONE_CONTRACT {
            return Err(format!(
                "unsupported published-tick ACK tombstone contract {}",
                self.contract_version
            ));
        }
        if self.journal_seal_sequence == 0 {
            return Err("published-tick ACK journal seal sequence must be positive".to_string());
        }
        self.high_water.validate()?;
        if self.high_water.phase != "complete" || !self.high_water.receipts_replayable {
            return Err(
                "published-tick ACK tombstone requires a replayable terminal high-water"
                    .to_string(),
            );
        }
        if !is_lowercase_sha256(&self.result_hash) {
            return Err(
                "published-tick ACK result hash must be 64 lowercase hexadecimal characters"
                    .to_string(),
            );
        }
        if !matches!(self.settlement_state.as_str(), "pending" | "settled") {
            return Err(
                "published-tick ACK settlement state must be pending or settled".to_string(),
            );
        }
        if self.acknowledged_at_unix_ms == 0 {
            return Err(
                "published-tick ACK acknowledgement timestamp must be positive".to_string(),
            );
        }
        validate_database_lineage(
            &self.database_system_identifier,
            self.database_timeline_id,
            &self.database_wal_lsn,
            "published-tick ACK",
        )
    }
}

/// Durable cold evidence that an exact running high-water was deliberately
/// retired after PostgreSQL atomically moved the match to `failed_closed`.
/// This is intentionally distinct from a terminal ACK: no terminal result was
/// published and no ACK may be inferred from this witness.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PublishedTickAbandonmentTombstone {
    pub contract_version: String,
    pub journal_seal_sequence: u64,
    pub high_water: PublishedTickHighWater,
    pub failure_reason: String,
    pub abandoned_at_unix_ms: u64,
    pub database_system_identifier: String,
    pub database_timeline_id: u32,
    pub database_wal_lsn: String,
}

#[derive(Clone, Debug)]
pub(crate) struct PublishedTickAbandonmentTombstoneInput {
    pub high_water: PublishedTickHighWater,
    pub failure_reason: String,
    pub abandoned_at_unix_ms: u64,
    pub database_system_identifier: String,
    pub database_timeline_id: u32,
    pub database_wal_lsn: String,
}

impl PublishedTickAbandonmentTombstone {
    fn new(
        input: PublishedTickAbandonmentTombstoneInput,
        journal_seal_sequence: u64,
    ) -> Result<Self, String> {
        let tombstone = Self {
            contract_version: ABANDONMENT_TOMBSTONE_CONTRACT.to_string(),
            journal_seal_sequence,
            high_water: input.high_water,
            failure_reason: input.failure_reason,
            abandoned_at_unix_ms: input.abandoned_at_unix_ms,
            database_system_identifier: input.database_system_identifier,
            database_timeline_id: input.database_timeline_id,
            database_wal_lsn: input.database_wal_lsn,
        };
        tombstone.validate()?;
        Ok(tombstone)
    }

    fn validate(&self) -> Result<(), String> {
        if self.contract_version != ABANDONMENT_TOMBSTONE_CONTRACT {
            return Err(format!(
                "unsupported published-tick abandonment tombstone contract {}",
                self.contract_version
            ));
        }
        if self.journal_seal_sequence == 0 {
            return Err(
                "published-tick abandonment journal seal sequence must be positive".to_string(),
            );
        }
        self.high_water.validate()?;
        if self.high_water.phase != "running" || !self.high_water.receipts_replayable {
            return Err(
                "published-tick abandonment tombstone requires a replayable running high-water"
                    .to_string(),
            );
        }
        if self.failure_reason.trim().is_empty()
            || self.failure_reason.len() > 1_024
            || self.failure_reason.chars().any(char::is_control)
        {
            return Err(
                "published-tick abandonment failure reason must be 1..=1024 non-control bytes"
                    .to_string(),
            );
        }
        if self.abandoned_at_unix_ms == 0 {
            return Err("published-tick abandonment timestamp must be positive".to_string());
        }
        validate_database_lineage(
            &self.database_system_identifier,
            self.database_timeline_id,
            &self.database_wal_lsn,
            "published-tick abandonment",
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PublishedTickColdWitness {
    TerminalAck(PublishedTickAckTombstone),
    FailedClosedAbandonment(PublishedTickAbandonmentTombstone),
}

impl PublishedTickColdWitness {
    fn validate(&self) -> Result<(), String> {
        match self {
            Self::TerminalAck(tombstone) => tombstone.validate(),
            Self::FailedClosedAbandonment(tombstone) => tombstone.validate(),
        }
    }

    fn high_water(&self) -> &PublishedTickHighWater {
        match self {
            Self::TerminalAck(tombstone) => &tombstone.high_water,
            Self::FailedClosedAbandonment(tombstone) => &tombstone.high_water,
        }
    }

    fn journal_seal_sequence(&self) -> u64 {
        match self {
            Self::TerminalAck(tombstone) => tombstone.journal_seal_sequence,
            Self::FailedClosedAbandonment(tombstone) => tombstone.journal_seal_sequence,
        }
    }

    fn database_system_identifier(&self) -> &str {
        match self {
            Self::TerminalAck(tombstone) => &tombstone.database_system_identifier,
            Self::FailedClosedAbandonment(tombstone) => &tombstone.database_system_identifier,
        }
    }

    fn database_timeline_id(&self) -> u32 {
        match self {
            Self::TerminalAck(tombstone) => tombstone.database_timeline_id,
            Self::FailedClosedAbandonment(tombstone) => tombstone.database_timeline_id,
        }
    }

    fn database_wal_lsn(&self) -> &str {
        match self {
            Self::TerminalAck(tombstone) => &tombstone.database_wal_lsn,
            Self::FailedClosedAbandonment(tombstone) => &tombstone.database_wal_lsn,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct LegacyAckTombstoneManifest {
    contract_version: String,
    journal_owner_id: Uuid,
    physical_host_id: String,
    tombstone_count: u64,
    committed_seal_sequence: u64,
    database_system_identifier: Option<String>,
    database_timeline_id: Option<u32>,
    latest_tombstone: Option<PublishedTickAckTombstone>,
    latest_tombstone_sha256: Option<String>,
}

impl LegacyAckTombstoneManifest {
    fn validate(&self, owner: &JournalOwnerManifest) -> Result<(), String> {
        if self.contract_version != LEGACY_ACK_MANIFEST_CONTRACT
            || self.journal_owner_id != owner.journal_owner_id
            || self.physical_host_id != owner.physical_host_id
            || self.tombstone_count != self.committed_seal_sequence
        {
            return Err(
                "published-tick legacy ACK manifest identity or sequence is invalid".to_string(),
            );
        }
        validate_legacy_manifest_payload(self)
    }

    fn normalize(self, owner: &JournalOwnerManifest) -> Result<AckTombstoneManifest, String> {
        self.validate(owner)?;
        let normalized = AckTombstoneManifest {
            contract_version: COLD_WITNESS_MANIFEST_CONTRACT.to_string(),
            journal_owner_id: self.journal_owner_id,
            physical_host_id: self.physical_host_id,
            terminal_tombstone_count: self.tombstone_count,
            abandonment_tombstone_count: 0,
            committed_seal_sequence: self.committed_seal_sequence,
            database_system_identifier: self.database_system_identifier,
            database_timeline_id: self.database_timeline_id,
            latest_witness: self
                .latest_tombstone
                .map(PublishedTickColdWitness::TerminalAck),
            latest_witness_sha256: self.latest_tombstone_sha256,
            legacy_latest_semantics: true,
        };
        normalized.validate(owner)?;
        Ok(normalized)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AckTombstoneManifest {
    contract_version: String,
    journal_owner_id: Uuid,
    physical_host_id: String,
    terminal_tombstone_count: u64,
    abandonment_tombstone_count: u64,
    committed_seal_sequence: u64,
    database_system_identifier: Option<String>,
    database_timeline_id: Option<u32>,
    latest_witness: Option<PublishedTickColdWitness>,
    latest_witness_sha256: Option<String>,
    // Legacy v1 selected its sentinel by WAL/tie-key order, so its latest
    // tombstone may precede the committed sequence. This is deliberately
    // process-local: every manifest serialized as v2 must use exact latest
    // seal-sequence semantics.
    #[serde(skip)]
    legacy_latest_semantics: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AckManifestOnDisk {
    Legacy(LegacyAckTombstoneManifest),
    Current(AckTombstoneManifest),
}

impl AckTombstoneManifest {
    fn empty(owner: &JournalOwnerManifest) -> Self {
        Self {
            contract_version: COLD_WITNESS_MANIFEST_CONTRACT.to_string(),
            journal_owner_id: owner.journal_owner_id,
            physical_host_id: owner.physical_host_id.clone(),
            terminal_tombstone_count: 0,
            abandonment_tombstone_count: 0,
            committed_seal_sequence: 0,
            database_system_identifier: None,
            database_timeline_id: None,
            latest_witness: None,
            latest_witness_sha256: None,
            legacy_latest_semantics: false,
        }
    }

    fn cold_witness_count(&self) -> Result<u64, String> {
        self.terminal_tombstone_count
            .checked_add(self.abandonment_tombstone_count)
            .ok_or_else(|| "published-tick cold witness count overflow".to_string())
    }

    fn validate(&self, owner: &JournalOwnerManifest) -> Result<(), String> {
        if self.contract_version != COLD_WITNESS_MANIFEST_CONTRACT
            || self.journal_owner_id != owner.journal_owner_id
            || self.physical_host_id != owner.physical_host_id
            || self.cold_witness_count()? != self.committed_seal_sequence
        {
            return Err(
                "published-tick cold witness manifest identity or sequence is invalid".to_string(),
            );
        }
        if self.committed_seal_sequence == 0 {
            if self.database_system_identifier.is_some()
                || self.database_timeline_id.is_some()
                || self.latest_witness.is_some()
                || self.latest_witness_sha256.is_some()
            {
                return Err(
                    "empty published-tick cold witness manifest contains lineage or latest state"
                        .to_string(),
                );
            }
            return Ok(());
        }
        let system_identifier = self.database_system_identifier.as_deref().ok_or_else(|| {
            "published-tick cold witness manifest database system identifier is missing".to_string()
        })?;
        if !is_canonical_positive_u64(system_identifier)
            || self
                .database_timeline_id
                .is_none_or(|timeline| timeline == 0)
        {
            return Err(
                "published-tick cold witness manifest database lineage is invalid".to_string(),
            );
        }
        let latest = self.latest_witness.as_ref().ok_or_else(|| {
            "published-tick cold witness manifest latest witness is missing".to_string()
        })?;
        latest.validate()?;
        if (self.legacy_latest_semantics
            && latest.journal_seal_sequence() > self.committed_seal_sequence)
            || (!self.legacy_latest_semantics
                && latest.journal_seal_sequence() != self.committed_seal_sequence)
            || latest.high_water().journal_owner_id != self.journal_owner_id
            || latest.high_water().physical_host_id != self.physical_host_id
            || latest.database_system_identifier() != system_identifier
            || Some(latest.database_timeline_id()) != self.database_timeline_id
            || self
                .latest_witness_sha256
                .as_deref()
                .is_none_or(|hash| !is_lowercase_sha256(hash))
        {
            return Err(
                "published-tick cold witness manifest latest witness is inconsistent".to_string(),
            );
        }
        Ok(())
    }
}

fn validate_legacy_manifest_payload(manifest: &LegacyAckTombstoneManifest) -> Result<(), String> {
    if manifest.tombstone_count == 0 {
        if manifest.database_system_identifier.is_some()
            || manifest.database_timeline_id.is_some()
            || manifest.latest_tombstone.is_some()
            || manifest.latest_tombstone_sha256.is_some()
        {
            return Err(
                "empty published-tick legacy ACK manifest contains lineage or latest state"
                    .to_string(),
            );
        }
        return Ok(());
    }
    let system_identifier = manifest
        .database_system_identifier
        .as_deref()
        .ok_or_else(|| {
            "published-tick legacy ACK manifest database system identifier is missing".to_string()
        })?;
    if !is_canonical_positive_u64(system_identifier)
        || manifest
            .database_timeline_id
            .is_none_or(|timeline| timeline == 0)
    {
        return Err("published-tick legacy ACK manifest database lineage is invalid".to_string());
    }
    let latest = manifest.latest_tombstone.as_ref().ok_or_else(|| {
        "published-tick legacy ACK manifest latest tombstone is missing".to_string()
    })?;
    latest.validate()?;
    if latest.journal_seal_sequence > manifest.committed_seal_sequence
        || latest.high_water.journal_owner_id != manifest.journal_owner_id
        || latest.high_water.physical_host_id != manifest.physical_host_id
        || latest.database_system_identifier != system_identifier
        || Some(latest.database_timeline_id) != manifest.database_timeline_id
        || manifest
            .latest_tombstone_sha256
            .as_deref()
            .is_none_or(|hash| !is_lowercase_sha256(hash))
    {
        return Err(
            "published-tick legacy ACK manifest latest tombstone is inconsistent".to_string(),
        );
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct JournalOwnerManifest {
    contract_version: String,
    journal_owner_id: Uuid,
    physical_host_id: String,
}

#[derive(Clone)]
pub(crate) struct PublishedTickJournal {
    root: Arc<PathBuf>,
    requests: mpsc::Sender<JournalRequest>,
    records: Arc<Mutex<BTreeMap<Uuid, PublishedTickHighWater>>>,
    ack_manifest: Arc<Mutex<AckTombstoneManifest>>,
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
        durable_database: DurableDatabaseHighWater,
        completion: oneshot::Sender<Result<(), String>>,
    },
    Compact {
        poison_generation: u64,
        active_matches: BTreeSet<Uuid>,
        completion: oneshot::Sender<Result<(), String>>,
    },
    SealTerminalAck {
        poison_generation: u64,
        input: Box<PublishedTickAckTombstoneInput>,
        completion: oneshot::Sender<Result<PublishedTickAckTombstone, String>>,
    },
    SealAbandonment {
        poison_generation: u64,
        input: Box<PublishedTickAbandonmentTombstoneInput>,
        completion: oneshot::Sender<Result<PublishedTickAbandonmentTombstone, String>>,
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
    ack_manifest: Arc<Mutex<AckTombstoneManifest>>,
    owner: Arc<JournalOwnerManifest>,
    _process_lock: Arc<File>,
    _host_identity_lock: Arc<File>,
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
        let mut ack_manifest = load_or_create_ack_manifest(&root, &owner)?;
        validate_manifest_latest_tombstone(&root, &owner, &ack_manifest)?;
        let records = Arc::new(Mutex::new(load_hot_records(
            &root,
            &owner,
            &mut ack_manifest,
        )?));
        let root = Arc::new(root);
        let ack_manifest = Arc::new(Mutex::new(ack_manifest));
        let (requests, receiver) = mpsc::channel(JOURNAL_QUEUE_CAPACITY);
        let failed_closed = Arc::new(AtomicBool::new(false));
        let poison_generation = Arc::new(AtomicU64::new(0));
        let (fatal_shutdown, _) = watch::channel(false);
        #[cfg(test)]
        let test_pause_next_record = Arc::new(Mutex::new(None));
        let worker_root = root.clone();
        let worker_records = records.clone();
        let worker_ack_manifest = ack_manifest.clone();
        let worker_owner = owner.clone();
        let worker_lock = process_lock.clone();
        let worker_host_identity_lock = host_identity_lock.clone();
        let worker_failed_closed = failed_closed.clone();
        let worker_poison_generation = poison_generation.clone();
        let worker_fatal_shutdown = fatal_shutdown.clone();
        #[cfg(test)]
        let worker_test_pause = test_pause_next_record.clone();
        tokio::spawn(async move {
            run_writer(
                JournalWriterContext {
                    root: (*worker_root).clone(),
                    records: worker_records,
                    ack_manifest: worker_ack_manifest,
                    owner: worker_owner,
                    _process_lock: worker_lock,
                    _host_identity_lock: worker_host_identity_lock,
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
            root,
            requests,
            records,
            ack_manifest,
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
                durable_database: DurableDatabaseHighWater {
                    next_sequence: durable_db_next_sequence,
                    match_revision: durable_db_match_revision,
                    next_input_sequences: durable_db_next_input_sequences,
                },
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

    pub(crate) fn ack_tombstone(
        &self,
        match_id: Uuid,
    ) -> Result<Option<PublishedTickAckTombstone>, String> {
        let manifest = self
            .ack_manifest
            .lock()
            .map_err(|_| "published-tick ACK manifest lock is poisoned".to_string())?
            .clone();
        match load_cold_witness_if_exists(&self.root, &self.owner, &manifest, match_id)? {
            Some(PublishedTickColdWitness::TerminalAck(tombstone)) => {
                validate_tombstone_against_manifest(&tombstone, &manifest)?;
                Ok(Some(tombstone))
            }
            Some(PublishedTickColdWitness::FailedClosedAbandonment(_)) | None => Ok(None),
        }
    }

    /// Returns at most `limit` tombstones strictly after `after_match_id` in
    /// UUID order. Callers must page explicitly; no runtime path is required
    /// to enumerate the complete cold history.
    #[cfg(test)]
    pub(crate) fn ack_tombstones_page(
        &self,
        after_match_id: Option<Uuid>,
        limit: usize,
    ) -> Result<Vec<PublishedTickAckTombstone>, String> {
        if limit == 0 || limit > MAX_ACK_TOMBSTONE_PAGE_SIZE {
            return Err(format!(
                "published-tick ACK tombstone page size must be 1..={MAX_ACK_TOMBSTONE_PAGE_SIZE}"
            ));
        }
        let manifest = self
            .ack_manifest
            .lock()
            .map_err(|_| "published-tick ACK manifest lock is poisoned".to_string())?
            .clone();
        scan_ack_tombstone_page(&self.root, &self.owner, &manifest, after_match_id, limit)
    }

    pub(crate) fn ack_tombstone_count(&self) -> Result<usize, String> {
        self.ack_manifest
            .lock()
            .map_err(|_| "published-tick ACK manifest lock is poisoned".to_string())
            .and_then(|manifest| {
                usize::try_from(manifest.terminal_tombstone_count)
                    .map_err(|_| "published-tick ACK tombstone count exceeds usize".to_string())
            })
    }

    pub(crate) fn abandonment_tombstone_count(&self) -> Result<usize, String> {
        self.ack_manifest
            .lock()
            .map_err(|_| "published-tick cold witness manifest lock is poisoned".to_string())
            .and_then(|manifest| {
                usize::try_from(manifest.abandonment_tombstone_count).map_err(|_| {
                    "published-tick abandonment tombstone count exceeds usize".to_string()
                })
            })
    }

    pub(crate) fn cold_witness_count(&self) -> Result<usize, String> {
        self.ack_manifest
            .lock()
            .map_err(|_| "published-tick cold witness manifest lock is poisoned".to_string())
            .and_then(|manifest| manifest.cold_witness_count())
            .and_then(|count| {
                usize::try_from(count)
                    .map_err(|_| "published-tick cold witness count exceeds usize".to_string())
            })
    }

    /// O(1) rollback sentinel across both acknowledged terminal matches and
    /// exact failed-closed abandonments.
    pub(crate) fn latest_cold_witness(&self) -> Result<Option<PublishedTickColdWitness>, String> {
        self.ack_manifest
            .lock()
            .map_err(|_| "published-tick cold witness manifest lock is poisoned".to_string())
            .map(|manifest| manifest.latest_witness.clone())
    }

    pub(crate) fn abandonment_tombstone(
        &self,
        match_id: Uuid,
    ) -> Result<Option<PublishedTickAbandonmentTombstone>, String> {
        let manifest = self
            .ack_manifest
            .lock()
            .map_err(|_| "published-tick cold witness manifest lock is poisoned".to_string())?
            .clone();
        match load_cold_witness_if_exists(&self.root, &self.owner, &manifest, match_id)? {
            Some(PublishedTickColdWitness::FailedClosedAbandonment(tombstone)) => {
                validate_abandonment_against_manifest(&tombstone, &manifest)?;
                Ok(Some(tombstone))
            }
            Some(PublishedTickColdWitness::TerminalAck(_)) | None => Ok(None),
        }
    }

    /// Seal only after the exact PostgreSQL terminal ACK transaction has
    /// committed. The database-derived timestamp and mandatory post-commit
    /// WAL flush LSN are supplied by the caller.
    pub(crate) async fn seal_terminal_ack(
        &self,
        input: PublishedTickAckTombstoneInput,
    ) -> Result<PublishedTickAckTombstone, String> {
        let poison_generation = self.operational_generation()?;
        let validated = PublishedTickAckTombstone::new(input.clone(), 1)?;
        if validated.high_water.journal_owner_id != self.owner.journal_owner_id
            || validated.high_water.physical_host_id != self.owner.physical_host_id
        {
            return Err(
                "published-tick ACK tombstone does not belong to this host journal".to_string(),
            );
        }
        let (completion, completed) = oneshot::channel();
        if self
            .requests
            .send(JournalRequest::SealTerminalAck {
                poison_generation,
                input: Box::new(input),
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
                    "published-tick journal writer stopped while sealing terminal ACK".to_string(),
                );
            }
        };
        self.require_generation(poison_generation)?;
        result
    }

    /// Seal only after PostgreSQL has committed an exact guarded
    /// `running -> failed_closed` transition for this high-water.
    pub(crate) async fn seal_abandonment(
        &self,
        input: PublishedTickAbandonmentTombstoneInput,
    ) -> Result<PublishedTickAbandonmentTombstone, String> {
        let poison_generation = self.operational_generation()?;
        let validated = PublishedTickAbandonmentTombstone::new(input.clone(), 1)?;
        if validated.high_water.journal_owner_id != self.owner.journal_owner_id
            || validated.high_water.physical_host_id != self.owner.physical_host_id
        {
            return Err(
                "published-tick abandonment tombstone does not belong to this host journal"
                    .to_string(),
            );
        }
        let (completion, completed) = oneshot::channel();
        if self
            .requests
            .send(JournalRequest::SealAbandonment {
                poison_generation,
                input: Box::new(input),
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
                    "published-tick journal writer stopped while sealing abandonment".to_string(),
                );
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
        ack_manifest,
        owner,
        _process_lock,
        _host_identity_lock,
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
                durable_database,
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
                let worker_ack_manifest = ack_manifest.clone();
                let worker_owner = owner.clone();
                let worker_process_lock = _process_lock.clone();
                let worker_host_identity_lock = _host_identity_lock.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let _lock_guards = (worker_process_lock, worker_host_identity_lock);
                    persist_record(
                        &worker_root,
                        &worker_records,
                        &worker_ack_manifest,
                        &worker_owner,
                        *record,
                        durable_database,
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
                let worker_process_lock = _process_lock.clone();
                let worker_host_identity_lock = _host_identity_lock.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let _lock_guards = (worker_process_lock, worker_host_identity_lock);
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
            JournalRequest::SealTerminalAck {
                poison_generation: request_generation,
                input,
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
                let worker_ack_manifest = ack_manifest.clone();
                let worker_owner = owner.clone();
                let worker_process_lock = _process_lock.clone();
                let worker_host_identity_lock = _host_identity_lock.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let _lock_guards = (worker_process_lock, worker_host_identity_lock);
                    seal_terminal_ack_tombstone(
                        &worker_root,
                        &worker_records,
                        &worker_ack_manifest,
                        &worker_owner,
                        *input,
                    )
                })
                .await
                .unwrap_or_else(|error| {
                    Err(JournalWriteError::DurabilityUncertain(format!(
                        "published-tick ACK seal task panicked or was cancelled: {error}"
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
            JournalRequest::SealAbandonment {
                poison_generation: request_generation,
                input,
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
                let worker_ack_manifest = ack_manifest.clone();
                let worker_owner = owner.clone();
                let worker_process_lock = _process_lock.clone();
                let worker_host_identity_lock = _host_identity_lock.clone();
                let result = tokio::task::spawn_blocking(move || {
                    let _lock_guards = (worker_process_lock, worker_host_identity_lock);
                    seal_abandonment_tombstone(
                        &worker_root,
                        &worker_records,
                        &worker_ack_manifest,
                        &worker_owner,
                        *input,
                    )
                })
                .await
                .unwrap_or_else(|error| {
                    Err(JournalWriteError::DurabilityUncertain(format!(
                        "published-tick abandonment seal task panicked or was cancelled: {error}"
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
    ack_manifest: &Mutex<AckTombstoneManifest>,
    owner: &JournalOwnerManifest,
    record: PublishedTickHighWater,
    durable_database: DurableDatabaseHighWater,
) -> Result<(), JournalWriteError> {
    record.validate().map_err(JournalWriteError::Rejected)?;
    if record.journal_owner_id != owner.journal_owner_id
        || record.physical_host_id != owner.physical_host_id
    {
        return Err(JournalWriteError::Rejected(
            "published-tick record does not belong to this host journal".to_string(),
        ));
    }
    if record.next_sequence > durable_database.next_sequence
        || record.match_revision > durable_database.match_revision
    {
        return Err(JournalWriteError::Rejected(
            "published-tick high-water cursor is ahead of durable database authority".to_string(),
        ));
    }
    if record
        .next_input_sequences
        .iter()
        .any(|(player_id, cursor)| {
            durable_database
                .next_input_sequences
                .get(player_id)
                .is_none_or(|durable| cursor > durable)
        })
    {
        return Err(JournalWriteError::Rejected(
            "published-tick member cursor is ahead of durable database authority".to_string(),
        ));
    }
    let (current, hot_record_count) = {
        let records = records.lock().map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?;
        (records.get(&record.match_id).cloned(), records.len())
    };
    let manifest = ack_manifest
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick cold witness manifest lock is poisoned".to_string(),
            )
        })?
        .clone();
    if let Some(witness) = load_cold_witness_if_exists(root, owner, &manifest, record.match_id)
        .map_err(JournalWriteError::DurabilityUncertain)?
    {
        validate_cold_witness_against_manifest(&witness, &manifest)
            .map_err(JournalWriteError::DurabilityUncertain)?;
        return Err(JournalWriteError::Rejected(
            "published-tick record cannot recreate or advance a durable cold witness".to_string(),
        ));
    }
    if let Some(current) = current.as_ref() {
        validate_forward_progress(current, &record).map_err(JournalWriteError::Rejected)?;
    } else {
        validate_hot_record_capacity(hot_record_count, true)
            .map_err(JournalWriteError::Rejected)?;
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
    let previous = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .insert(record.match_id, record);
    if current.is_none() && previous.is_some() {
        return Err(JournalWriteError::DurabilityUncertain(
            "published-tick record appeared while installing a new hot high-water".to_string(),
        ));
    }
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

fn validate_hot_record_capacity(
    hot_record_count: usize,
    adding_new_record: bool,
) -> Result<(), String> {
    if (adding_new_record && hot_record_count >= MAX_RECORDS)
        || (!adding_new_record && hot_record_count > MAX_RECORDS)
    {
        return Err(format!(
            "published-tick journal reached its bounded {MAX_RECORDS}-match hot retention limit"
        ));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut digest_hex = String::with_capacity(64);
    for byte in digest {
        write!(&mut digest_hex, "{byte:02x}")
            .expect("writing a SHA-256 digest into a String cannot fail");
    }
    digest_hex
}

fn validate_tombstone_against_manifest(
    tombstone: &PublishedTickAckTombstone,
    manifest: &AckTombstoneManifest,
) -> Result<(), String> {
    validate_cold_witness_against_manifest(
        &PublishedTickColdWitness::TerminalAck(tombstone.clone()),
        manifest,
    )
}

fn validate_abandonment_against_manifest(
    tombstone: &PublishedTickAbandonmentTombstone,
    manifest: &AckTombstoneManifest,
) -> Result<(), String> {
    validate_cold_witness_against_manifest(
        &PublishedTickColdWitness::FailedClosedAbandonment(tombstone.clone()),
        manifest,
    )
}

fn validate_cold_witness_against_manifest(
    witness: &PublishedTickColdWitness,
    manifest: &AckTombstoneManifest,
) -> Result<(), String> {
    witness.validate()?;
    let kind_count = match witness {
        PublishedTickColdWitness::TerminalAck(_) => manifest.terminal_tombstone_count,
        PublishedTickColdWitness::FailedClosedAbandonment(_) => {
            manifest.abandonment_tombstone_count
        }
    };
    if kind_count == 0
        || witness.journal_seal_sequence() > manifest.committed_seal_sequence
        || witness.high_water().journal_owner_id != manifest.journal_owner_id
        || witness.high_water().physical_host_id != manifest.physical_host_id
        || manifest.database_system_identifier.as_deref()
            != Some(witness.database_system_identifier())
        || manifest.database_timeline_id != Some(witness.database_timeline_id())
    {
        return Err(
            "published-tick cold witness is not covered by the durable manifest".to_string(),
        );
    }
    Ok(())
}

fn advance_manifest_for_witness(
    manifest: &mut AckTombstoneManifest,
    owner: &JournalOwnerManifest,
    witness: &PublishedTickColdWitness,
    witness_sha256: &str,
) -> Result<(), String> {
    manifest.validate(owner)?;
    witness.validate()?;
    if !is_lowercase_sha256(witness_sha256)
        || witness.journal_seal_sequence()
            != manifest
                .committed_seal_sequence
                .checked_add(1)
                .ok_or_else(|| "published-tick cold witness seal sequence overflow".to_string())?
        || witness.high_water().journal_owner_id != owner.journal_owner_id
        || witness.high_water().physical_host_id != owner.physical_host_id
    {
        return Err("published-tick cold witness cannot advance this manifest".to_string());
    }

    if manifest.committed_seal_sequence == 0 {
        manifest.database_system_identifier =
            Some(witness.database_system_identifier().to_string());
        manifest.database_timeline_id = Some(witness.database_timeline_id());
    } else if manifest.database_system_identifier.as_deref()
        != Some(witness.database_system_identifier())
        || manifest.database_timeline_id != Some(witness.database_timeline_id())
    {
        return Err(
            "published-tick cold witness database incarnation changed within one host journal"
                .to_string(),
        );
    }

    let candidate_wal_lsn =
        parse_postgres_wal_lsn(witness.database_wal_lsn()).ok_or_else(|| {
            "published-tick cold witness database WAL LSN is not canonical PostgreSQL LSN text"
                .to_string()
        })?;
    if let Some(current) = manifest.latest_witness.as_ref() {
        let current_wal_lsn =
            parse_postgres_wal_lsn(current.database_wal_lsn()).ok_or_else(|| {
                "published-tick cold witness manifest WAL LSN is not canonical PostgreSQL LSN text"
                    .to_string()
            })?;
        if candidate_wal_lsn < current_wal_lsn {
            return Err("published-tick cold witness WAL LSN cannot regress".to_string());
        }
    }
    // The sentinel is the exact latest durable seal, not the maximum of a
    // WAL/tie-key ordering. Equal WAL LSNs are valid across independent
    // exact transactions and the newly committed sequence must still win.
    manifest.latest_witness = Some(witness.clone());
    manifest.latest_witness_sha256 = Some(witness_sha256.to_string());
    match witness {
        PublishedTickColdWitness::TerminalAck(_) => {
            manifest.terminal_tombstone_count = manifest
                .terminal_tombstone_count
                .checked_add(1)
                .ok_or_else(|| "published-tick ACK tombstone count overflow".to_string())?;
        }
        PublishedTickColdWitness::FailedClosedAbandonment(_) => {
            manifest.abandonment_tombstone_count = manifest
                .abandonment_tombstone_count
                .checked_add(1)
                .ok_or_else(|| "published-tick abandonment tombstone count overflow".to_string())?;
        }
    }
    manifest.committed_seal_sequence = witness.journal_seal_sequence();
    manifest.legacy_latest_semantics = false;
    manifest.validate(owner)
}

fn advance_manifest_for_tombstone(
    manifest: &mut AckTombstoneManifest,
    owner: &JournalOwnerManifest,
    tombstone: &PublishedTickAckTombstone,
    tombstone_sha256: &str,
) -> Result<(), String> {
    advance_manifest_for_witness(
        manifest,
        owner,
        &PublishedTickColdWitness::TerminalAck(tombstone.clone()),
        tombstone_sha256,
    )
}

fn advance_manifest_for_abandonment(
    manifest: &mut AckTombstoneManifest,
    owner: &JournalOwnerManifest,
    tombstone: &PublishedTickAbandonmentTombstone,
    tombstone_sha256: &str,
) -> Result<(), String> {
    advance_manifest_for_witness(
        manifest,
        owner,
        &PublishedTickColdWitness::FailedClosedAbandonment(tombstone.clone()),
        tombstone_sha256,
    )
}

fn tombstone_matches_input(
    tombstone: &PublishedTickAckTombstone,
    input: &PublishedTickAckTombstoneInput,
) -> Result<bool, String> {
    let stored_lsn = parse_postgres_wal_lsn(&tombstone.database_wal_lsn).ok_or_else(|| {
        "published-tick ACK database WAL LSN is not canonical PostgreSQL LSN text".to_string()
    })?;
    let retry_lsn = parse_postgres_wal_lsn(&input.database_wal_lsn).ok_or_else(|| {
        "published-tick ACK retry WAL LSN is not canonical PostgreSQL LSN text".to_string()
    })?;
    Ok(tombstone.high_water == input.high_water
        && tombstone.result_hash == input.result_hash
        && (tombstone.settlement_state == input.settlement_state
            || (tombstone.settlement_state == "pending" && input.settlement_state == "settled"))
        && tombstone.acknowledged_at_unix_ms == input.acknowledged_at_unix_ms
        && tombstone.database_system_identifier == input.database_system_identifier
        && tombstone.database_timeline_id == input.database_timeline_id
        && retry_lsn >= stored_lsn)
}

fn abandonment_matches_input(
    tombstone: &PublishedTickAbandonmentTombstone,
    input: &PublishedTickAbandonmentTombstoneInput,
) -> Result<bool, String> {
    let stored_lsn = parse_postgres_wal_lsn(&tombstone.database_wal_lsn).ok_or_else(|| {
        "published-tick abandonment database WAL LSN is not canonical PostgreSQL LSN text"
            .to_string()
    })?;
    let retry_lsn = parse_postgres_wal_lsn(&input.database_wal_lsn).ok_or_else(|| {
        "published-tick abandonment retry WAL LSN is not canonical PostgreSQL LSN text".to_string()
    })?;
    Ok(tombstone.high_water == input.high_water
        && tombstone.failure_reason == input.failure_reason
        && tombstone.abandoned_at_unix_ms == input.abandoned_at_unix_ms
        && tombstone.database_system_identifier == input.database_system_identifier
        && tombstone.database_timeline_id == input.database_timeline_id
        && retry_lsn >= stored_lsn)
}

fn install_manifest_update(
    root: &Path,
    manifest_lock: &Mutex<AckTombstoneManifest>,
    previous: &AckTombstoneManifest,
    next: AckTombstoneManifest,
) -> Result<(), JournalWriteError> {
    persist_ack_manifest(root, &next)?;
    let mut current = manifest_lock.lock().map_err(|_| {
        JournalWriteError::DurabilityUncertain(
            "published-tick ACK manifest lock is poisoned".to_string(),
        )
    })?;
    if &*current != previous {
        return Err(JournalWriteError::DurabilityUncertain(
            "published-tick ACK manifest changed while installing a seal".to_string(),
        ));
    }
    *current = next;
    Ok(())
}

fn seal_terminal_ack_tombstone(
    root: &Path,
    records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
    ack_manifest: &Mutex<AckTombstoneManifest>,
    owner: &JournalOwnerManifest,
    input: PublishedTickAckTombstoneInput,
) -> Result<PublishedTickAckTombstone, JournalWriteError> {
    PublishedTickAckTombstone::new(input.clone(), 1).map_err(JournalWriteError::Rejected)?;
    if input.high_water.journal_owner_id != owner.journal_owner_id
        || input.high_water.physical_host_id != owner.physical_host_id
    {
        return Err(JournalWriteError::Rejected(
            "published-tick ACK tombstone does not belong to this host journal".to_string(),
        ));
    }
    let match_id = input.high_water.match_id;
    let manifest = ack_manifest
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick ACK manifest lock is poisoned".to_string(),
            )
        })?
        .clone();
    manifest
        .validate(owner)
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let current_high_water = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .get(&match_id)
        .cloned();

    if load_abandonment_tombstone_if_exists(root, owner, &manifest, match_id)
        .map_err(JournalWriteError::DurabilityUncertain)?
        .is_some()
    {
        return Err(JournalWriteError::DurabilityUncertain(
            "published-tick match has both terminal ACK and abandonment cold-witness intent"
                .to_string(),
        ));
    }

    if let Some((existing, tombstone_sha256)) =
        load_ack_tombstone_if_exists_with_sha(root, owner, match_id)
            .map_err(JournalWriteError::DurabilityUncertain)?
    {
        if !tombstone_matches_input(&existing, &input)
            .map_err(JournalWriteError::DurabilityUncertain)?
        {
            return Err(JournalWriteError::Rejected(
                "published-tick ACK seal does not match the durable tombstone".to_string(),
            ));
        }
        if existing.journal_seal_sequence
            == manifest
                .committed_seal_sequence
                .checked_add(1)
                .ok_or_else(|| {
                    JournalWriteError::DurabilityUncertain(
                        "published-tick ACK seal sequence overflow".to_string(),
                    )
                })?
        {
            if current_high_water.as_ref() != Some(&existing.high_water) {
                return Err(JournalWriteError::DurabilityUncertain(
                    "uncommitted published-tick ACK tombstone lacks its exact hot witness"
                        .to_string(),
                ));
            }
            let mut next_manifest = manifest.clone();
            advance_manifest_for_tombstone(&mut next_manifest, owner, &existing, &tombstone_sha256)
                .map_err(JournalWriteError::DurabilityUncertain)?;
            install_manifest_update(root, ack_manifest, &manifest, next_manifest)?;
        } else {
            validate_tombstone_against_manifest(&existing, &manifest)
                .map_err(JournalWriteError::DurabilityUncertain)?;
        }
        if let Some(current) = current_high_water {
            if current != existing.high_water {
                return Err(JournalWriteError::DurabilityUncertain(
                    "published-tick hot record conflicts with its durable ACK tombstone"
                        .to_string(),
                ));
            }
            remove_exact_hot_record(root, records, &current)?;
        }
        return Ok(existing);
    }

    let Some(current) = current_high_water else {
        return Err(JournalWriteError::Rejected(
            "published-tick ACK seal requires the exact terminal hot high-water".to_string(),
        ));
    };
    if current != input.high_water {
        return Err(JournalWriteError::Rejected(
            "published-tick ACK seal does not match the current terminal high-water".to_string(),
        ));
    }
    let seal_sequence = manifest
        .committed_seal_sequence
        .checked_add(1)
        .ok_or_else(|| {
            JournalWriteError::DurabilityUncertain(
                "published-tick ACK seal sequence overflow".to_string(),
            )
        })?;
    let tombstone = PublishedTickAckTombstone::new(input, seal_sequence)
        .map_err(JournalWriteError::Rejected)?;
    let payload = serde_json::to_vec_pretty(&tombstone).map_err(|error| {
        JournalWriteError::Rejected(format!("encode published-tick ACK tombstone: {error}"))
    })?;
    if payload.len() as u64 > MAX_ACK_TOMBSTONE_BYTES {
        return Err(JournalWriteError::Rejected(format!(
            "published-tick ACK tombstone exceeds {MAX_ACK_TOMBSTONE_BYTES} bytes"
        )));
    }
    let tombstone_sha256 = sha256_hex(&payload);
    let mut next_manifest = manifest.clone();
    advance_manifest_for_tombstone(&mut next_manifest, owner, &tombstone, &tombstone_sha256)
        .map_err(JournalWriteError::Rejected)?;
    let shard = ensure_ack_tombstone_shard(root, match_id)?;
    let target = ack_tombstone_path(root, match_id);
    match fs::symlink_metadata(&target) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "unindexed published-tick ACK tombstone already exists at {}",
                target.display()
            )));
        }
        Err(error) => {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "inspect published-tick ACK tombstone {}: {error}",
                target.display()
            )));
        }
    }

    // Crash contract: make cold evidence durable, then make its manifest
    // membership durable, and only then unlink the hot witness.
    atomic_install(&shard, &target, &payload)?;
    install_manifest_update(root, ack_manifest, &manifest, next_manifest)?;
    remove_exact_hot_record(root, records, &current)?;
    Ok(tombstone)
}

fn seal_abandonment_tombstone(
    root: &Path,
    records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
    ack_manifest: &Mutex<AckTombstoneManifest>,
    owner: &JournalOwnerManifest,
    input: PublishedTickAbandonmentTombstoneInput,
) -> Result<PublishedTickAbandonmentTombstone, JournalWriteError> {
    PublishedTickAbandonmentTombstone::new(input.clone(), 1)
        .map_err(JournalWriteError::Rejected)?;
    if input.high_water.journal_owner_id != owner.journal_owner_id
        || input.high_water.physical_host_id != owner.physical_host_id
    {
        return Err(JournalWriteError::Rejected(
            "published-tick abandonment tombstone does not belong to this host journal".to_string(),
        ));
    }
    let match_id = input.high_water.match_id;
    let manifest = ack_manifest
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick cold witness manifest lock is poisoned".to_string(),
            )
        })?
        .clone();
    manifest
        .validate(owner)
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let current_high_water = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .get(&match_id)
        .cloned();

    if load_ack_tombstone_if_exists(root, owner, match_id)
        .map_err(JournalWriteError::DurabilityUncertain)?
        .is_some()
    {
        return Err(JournalWriteError::DurabilityUncertain(
            "published-tick match has both abandonment and terminal ACK cold-witness intent"
                .to_string(),
        ));
    }

    if let Some((existing, tombstone_sha256)) =
        load_abandonment_tombstone_if_exists_with_sha(root, owner, &manifest, match_id)
            .map_err(JournalWriteError::DurabilityUncertain)?
    {
        if !abandonment_matches_input(&existing, &input)
            .map_err(JournalWriteError::DurabilityUncertain)?
        {
            return Err(JournalWriteError::Rejected(
                "published-tick abandonment seal does not match the durable tombstone".to_string(),
            ));
        }
        if existing.journal_seal_sequence
            == manifest
                .committed_seal_sequence
                .checked_add(1)
                .ok_or_else(|| {
                    JournalWriteError::DurabilityUncertain(
                        "published-tick abandonment seal sequence overflow".to_string(),
                    )
                })?
        {
            if current_high_water.as_ref() != Some(&existing.high_water) {
                return Err(JournalWriteError::DurabilityUncertain(
                    "uncommitted published-tick abandonment tombstone lacks its exact hot witness"
                        .to_string(),
                ));
            }
            let mut next_manifest = manifest.clone();
            advance_manifest_for_abandonment(
                &mut next_manifest,
                owner,
                &existing,
                &tombstone_sha256,
            )
            .map_err(JournalWriteError::DurabilityUncertain)?;
            install_manifest_update(root, ack_manifest, &manifest, next_manifest)?;
        } else {
            validate_abandonment_against_manifest(&existing, &manifest)
                .map_err(JournalWriteError::DurabilityUncertain)?;
        }
        if let Some(current) = current_high_water {
            if current != existing.high_water {
                return Err(JournalWriteError::DurabilityUncertain(
                    "published-tick hot record conflicts with its durable abandonment tombstone"
                        .to_string(),
                ));
            }
            remove_exact_hot_record(root, records, &current)?;
        }
        return Ok(existing);
    }

    let Some(current) = current_high_water else {
        return Err(JournalWriteError::Rejected(
            "published-tick abandonment seal requires the exact running hot high-water".to_string(),
        ));
    };
    if current != input.high_water {
        return Err(JournalWriteError::Rejected(
            "published-tick abandonment seal does not match the current running high-water"
                .to_string(),
        ));
    }
    let seal_sequence = manifest
        .committed_seal_sequence
        .checked_add(1)
        .ok_or_else(|| {
            JournalWriteError::DurabilityUncertain(
                "published-tick abandonment seal sequence overflow".to_string(),
            )
        })?;
    let tombstone = PublishedTickAbandonmentTombstone::new(input, seal_sequence)
        .map_err(JournalWriteError::Rejected)?;
    let payload = serde_json::to_vec_pretty(&tombstone).map_err(|error| {
        JournalWriteError::Rejected(format!(
            "encode published-tick abandonment tombstone: {error}"
        ))
    })?;
    if payload.len() as u64 > MAX_ABANDONMENT_TOMBSTONE_BYTES {
        return Err(JournalWriteError::Rejected(format!(
            "published-tick abandonment tombstone exceeds {MAX_ABANDONMENT_TOMBSTONE_BYTES} bytes"
        )));
    }
    let tombstone_sha256 = sha256_hex(&payload);
    let mut next_manifest = manifest.clone();
    advance_manifest_for_abandonment(&mut next_manifest, owner, &tombstone, &tombstone_sha256)
        .map_err(JournalWriteError::Rejected)?;
    let shard = ensure_abandonment_tombstone_shard(root, match_id)?;
    let target = abandonment_tombstone_path(root, match_id);
    match fs::symlink_metadata(&target) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "unindexed published-tick abandonment tombstone already exists at {}",
                target.display()
            )));
        }
        Err(error) => {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "inspect published-tick abandonment tombstone {}: {error}",
                target.display()
            )));
        }
    }

    // Crash contract: cold fsync -> unified manifest fsync -> exact hot unlink.
    atomic_install(&shard, &target, &payload)?;
    install_manifest_update(root, ack_manifest, &manifest, next_manifest)?;
    remove_exact_hot_record(root, records, &current)?;
    Ok(tombstone)
}

fn remove_exact_hot_record(
    root: &Path,
    records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
    expected: &PublishedTickHighWater,
) -> Result<(), JournalWriteError> {
    let path = record_path(root, expected.match_id);
    fs::remove_file(&path).map_err(|error| {
        JournalWriteError::DurabilityUncertain(format!(
            "remove sealed terminal published-tick record {}: {error}",
            path.display()
        ))
    })?;
    sync_directory(root).map_err(JournalWriteError::DurabilityUncertain)?;
    let removed = records
        .lock()
        .map_err(|_| {
            JournalWriteError::DurabilityUncertain(
                "published-tick journal record lock is poisoned".to_string(),
            )
        })?
        .remove(&expected.match_id);
    if removed.as_ref() != Some(expected) {
        return Err(JournalWriteError::DurabilityUncertain(
            "published-tick hot record changed while sealing its ACK tombstone".to_string(),
        ));
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

fn load_or_create_ack_manifest(
    root: &Path,
    owner: &JournalOwnerManifest,
) -> Result<AckTombstoneManifest, String> {
    let path = root.join(ACK_MANIFEST_FILE);
    let ack_root = root.join(ACK_TOMBSTONE_DIRECTORY);
    let abandonment_root = root.join(ABANDONMENT_TOMBSTONE_DIRECTORY);
    let mut legacy_on_disk = false;
    let manifest = match fs::symlink_metadata(&path) {
        Ok(_) => {
            let on_disk: AckManifestOnDisk = read_private_json(
                &path,
                "published-tick cold witness manifest",
                MAX_ACK_MANIFEST_BYTES,
            )?;
            match on_disk {
                AckManifestOnDisk::Legacy(legacy) => {
                    legacy_on_disk = true;
                    legacy.normalize(owner)?
                }
                AckManifestOnDisk::Current(current) => current,
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            for cold_root in [&ack_root, &abandonment_root] {
                match fs::symlink_metadata(cold_root) {
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Ok(_) => {
                        return Err(
                            "published-tick cold witness directory exists without its durable manifest"
                                .to_string(),
                        );
                    }
                    Err(error) => {
                        return Err(format!(
                            "inspect published-tick cold witness directory {}: {error}",
                            cold_root.display()
                        ));
                    }
                }
            }
            let manifest = AckTombstoneManifest::empty(owner);
            persist_ack_manifest(root, &manifest).map_err(JournalWriteError::message)?;
            manifest
        }
        Err(error) => {
            return Err(format!(
                "inspect published-tick cold witness manifest {}: {error}",
                path.display()
            ));
        }
    };
    manifest.validate(owner)?;
    if legacy_on_disk {
        match fs::symlink_metadata(&abandonment_root) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => {
                validate_existing_secure_directory(
                    &abandonment_root,
                    "published-tick abandonment tombstone directory",
                )?;
                let mut entries = fs::read_dir(&abandonment_root).map_err(|error| {
                    format!(
                        "read published-tick abandonment directory {}: {error}",
                        abandonment_root.display()
                    )
                })?;
                if entries
                    .next()
                    .transpose()
                    .map_err(|error| {
                        format!(
                            "read published-tick abandonment directory entry {}: {error}",
                            abandonment_root.display()
                        )
                    })?
                    .is_some()
                {
                    return Err(
                        "legacy published-tick ACK manifest cannot cover a non-empty abandonment witness tree"
                            .to_string(),
                    );
                }
            }
            Err(error) => {
                return Err(format!(
                    "inspect published-tick abandonment directory {}: {error}",
                    abandonment_root.display()
                ));
            }
        }
    }
    match fs::symlink_metadata(&ack_root) {
        Ok(_) => {
            validate_existing_secure_directory(&ack_root, "published-tick ACK tombstone directory")?
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if manifest.terminal_tombstone_count != 0 {
                return Err(
                    "non-empty published-tick ACK manifest lost its tombstone directory"
                        .to_string(),
                );
            }
            ensure_secure_child_directory(root, &ack_root)?;
        }
        Err(error) => {
            return Err(format!(
                "inspect published-tick ACK directory {}: {error}",
                ack_root.display()
            ));
        }
    }
    match fs::symlink_metadata(&abandonment_root) {
        Ok(_) => validate_existing_secure_directory(
            &abandonment_root,
            "published-tick abandonment tombstone directory",
        )?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if manifest.abandonment_tombstone_count != 0 {
                return Err(
                    "non-empty published-tick cold witness manifest lost its abandonment directory"
                        .to_string(),
                );
            }
            // Reading a real v1 layout is non-mutating so a release rollback
            // can still start the old binary until the first v2 abandonment
            // witness makes the filesystem contract intentionally one-way.
            if !legacy_on_disk {
                ensure_secure_child_directory(root, &abandonment_root)?;
            }
        }
        Err(error) => {
            return Err(format!(
                "inspect published-tick abandonment directory {}: {error}",
                abandonment_root.display()
            ));
        }
    }
    Ok(manifest)
}

fn persist_ack_manifest(
    root: &Path,
    manifest: &AckTombstoneManifest,
) -> Result<(), JournalWriteError> {
    let payload = serde_json::to_vec_pretty(manifest).map_err(|error| {
        JournalWriteError::Rejected(format!("encode published-tick ACK manifest: {error}"))
    })?;
    if payload.len() as u64 > MAX_ACK_MANIFEST_BYTES {
        return Err(JournalWriteError::Rejected(format!(
            "published-tick ACK manifest exceeds {MAX_ACK_MANIFEST_BYTES} bytes"
        )));
    }
    atomic_install(root, &root.join(ACK_MANIFEST_FILE), &payload)
}

fn validate_manifest_latest_tombstone(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &AckTombstoneManifest,
) -> Result<(), String> {
    manifest.validate(owner)?;
    let Some(expected) = manifest.latest_witness.as_ref() else {
        return Ok(());
    };
    let (actual, actual_sha256) = load_cold_witness_if_exists_with_sha(
        root,
        owner,
        manifest,
        expected.high_water().match_id,
    )?
    .ok_or_else(|| {
        "published-tick cold witness manifest latest witness file is missing".to_string()
    })?;
    if &actual != expected
        || manifest.latest_witness_sha256.as_deref() != Some(actual_sha256.as_str())
    {
        return Err(
            "published-tick cold witness manifest latest witness does not match durable storage"
                .to_string(),
        );
    }
    validate_cold_witness_against_manifest(&actual, manifest)
}

fn load_hot_records(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &mut AckTombstoneManifest,
) -> Result<BTreeMap<Uuid, PublishedTickHighWater>, String> {
    manifest.validate(owner)?;
    let mut records = BTreeMap::new();
    let mut removed_root_entry = false;
    let mut root_entry_count = 0usize;
    for entry in fs::read_dir(root)
        .map_err(|error| format!("read published-tick journal {}: {error}", root.display()))?
    {
        root_entry_count += 1;
        if root_entry_count > MAX_JOURNAL_ROOT_ENTRIES {
            return Err(format!(
                "published-tick journal root exceeds its {MAX_JOURNAL_ROOT_ENTRIES}-entry startup bound"
            ));
        }
        let entry = entry.map_err(|error| format!("read published-tick entry: {error}"))?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| "published-tick journal contains a non-UTF8 filename".to_string())?;
        if matches!(
            name,
            ".published-tick.lock" | ".published-tick-owner.json" | ACK_MANIFEST_FILE
        ) {
            continue;
        }
        if name == ACK_TOMBSTONE_DIRECTORY {
            validate_existing_secure_directory(&path, "published-tick ACK tombstone directory")?;
            continue;
        }
        if name == ABANDONMENT_TOMBSTONE_DIRECTORY {
            validate_existing_secure_directory(
                &path,
                "published-tick abandonment tombstone directory",
            )?;
            continue;
        }
        if is_atomic_temp_name(name) {
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "inspect published-tick atomic temporary {}: {error}",
                    path.display()
                )
            })?;
            validate_private_regular_file(&path, &metadata, "published-tick atomic temporary")?;
            fs::remove_file(&path).map_err(|error| {
                format!(
                    "remove published-tick atomic temporary {}: {error}",
                    path.display()
                )
            })?;
            removed_root_entry = true;
            continue;
        }
        let Some(match_id) = hot_record_match_id(name) else {
            return Err(format!(
                "published-tick journal contains unexpected entry {}",
                path.display()
            ));
        };
        let record: PublishedTickHighWater =
            read_private_json(&path, "published-tick record", MAX_RECORD_BYTES)?;
        record.validate()?;
        if record.match_id != match_id
            || record.journal_owner_id != owner.journal_owner_id
            || record.physical_host_id != owner.physical_host_id
        {
            return Err(format!(
                "published-tick record {} has the wrong filename or host journal identity",
                path.display()
            ));
        }
        if let Some((witness, witness_sha256)) =
            load_cold_witness_if_exists_with_sha(root, owner, manifest, match_id)?
        {
            if witness.high_water() != &record {
                return Err(format!(
                    "published-tick hot record for match {match_id} conflicts with its cold witness"
                ));
            }
            if witness.journal_seal_sequence()
                == manifest
                    .committed_seal_sequence
                    .checked_add(1)
                    .ok_or_else(|| {
                        "published-tick cold witness seal sequence overflow".to_string()
                    })?
            {
                let mut next_manifest = manifest.clone();
                advance_manifest_for_witness(&mut next_manifest, owner, &witness, &witness_sha256)?;
                persist_ack_manifest(root, &next_manifest).map_err(JournalWriteError::message)?;
                *manifest = next_manifest;
            } else if witness.journal_seal_sequence() == manifest.committed_seal_sequence
                && ((manifest.legacy_latest_semantics
                    && matches!(&witness, PublishedTickColdWitness::TerminalAck(_)))
                    || (manifest.latest_witness.as_ref() == Some(&witness)
                        && manifest.latest_witness_sha256.as_deref()
                            == Some(witness_sha256.as_str())))
            {
                // V1 chose latest_tombstone by WAL/tie-key order. An honest
                // same-LSN sequence-N seal could therefore commit the v1
                // manifest while its sentinel still named sequence N-1. The
                // exact hot + terminal cold sequence-N overlap is nevertheless
                // its manifest-fsync/hot-unlink crash boundary. V2 never takes
                // this branch without exact latest witness/hash membership.
                validate_cold_witness_against_manifest(&witness, manifest)?;
            } else {
                return Err(format!(
                    "published-tick hot record for match {match_id} overlaps a cold witness that is not the manifest latest crash boundary"
                ));
            }
            fs::remove_file(&path).map_err(|error| {
                format!(
                    "remove redundant sealed hot record {}: {error}",
                    path.display()
                )
            })?;
            removed_root_entry = true;
            continue;
        }
        if records.insert(match_id, record).is_some() {
            return Err("duplicate published-tick match record".to_string());
        }
        validate_hot_record_capacity(records.len(), false)?;
    }
    if removed_root_entry {
        sync_directory(root)?;
    }
    Ok(records)
}

fn load_ack_tombstone_if_exists(
    root: &Path,
    owner: &JournalOwnerManifest,
    match_id: Uuid,
) -> Result<Option<PublishedTickAckTombstone>, String> {
    load_ack_tombstone_if_exists_with_sha(root, owner, match_id)
        .map(|record| record.map(|(tombstone, _)| tombstone))
}

fn load_ack_tombstone_if_exists_with_sha(
    root: &Path,
    owner: &JournalOwnerManifest,
    match_id: Uuid,
) -> Result<Option<(PublishedTickAckTombstone, String)>, String> {
    let (first, second) = ack_shard_components(match_id);
    let ack_root = root.join(ACK_TOMBSTONE_DIRECTORY);
    validate_existing_secure_directory(&ack_root, "published-tick ACK tombstone directory")?;
    let first_path = ack_root.join(&first);
    match fs::symlink_metadata(&first_path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(_) => {
            validate_existing_secure_directory(&first_path, "published-tick ACK first-level shard")?
        }
        Err(error) => {
            return Err(format!(
                "inspect published-tick ACK shard {}: {error}",
                first_path.display()
            ));
        }
    }
    let second_path = first_path.join(&second);
    match fs::symlink_metadata(&second_path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(_) => validate_existing_secure_directory(
            &second_path,
            "published-tick ACK second-level shard",
        )?,
        Err(error) => {
            return Err(format!(
                "inspect published-tick ACK shard {}: {error}",
                second_path.display()
            ));
        }
    }
    let path = ack_tombstone_path(root, match_id);
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(_) => {}
        Err(error) => {
            return Err(format!(
                "inspect published-tick ACK tombstone {}: {error}",
                path.display()
            ));
        }
    }
    let (tombstone, sha256): (PublishedTickAckTombstone, String) = read_private_json_with_sha(
        &path,
        "published-tick ACK tombstone",
        MAX_ACK_TOMBSTONE_BYTES,
    )?;
    tombstone.validate()?;
    if tombstone.high_water.match_id != match_id
        || tombstone.high_water.journal_owner_id != owner.journal_owner_id
        || tombstone.high_water.physical_host_id != owner.physical_host_id
    {
        return Err(format!(
            "published-tick ACK tombstone {} has the wrong filename or host journal identity",
            path.display()
        ));
    }
    Ok(Some((tombstone, sha256)))
}

fn load_abandonment_tombstone_if_exists(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &AckTombstoneManifest,
    match_id: Uuid,
) -> Result<Option<PublishedTickAbandonmentTombstone>, String> {
    load_abandonment_tombstone_if_exists_with_sha(root, owner, manifest, match_id)
        .map(|record| record.map(|(tombstone, _)| tombstone))
}

fn load_abandonment_tombstone_if_exists_with_sha(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &AckTombstoneManifest,
    match_id: Uuid,
) -> Result<Option<(PublishedTickAbandonmentTombstone, String)>, String> {
    let (first, second) = ack_shard_components(match_id);
    let abandonment_root = root.join(ABANDONMENT_TOMBSTONE_DIRECTORY);
    match fs::symlink_metadata(&abandonment_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if manifest.abandonment_tombstone_count == 0 {
                return Ok(None);
            }
            return Err(
                "non-empty published-tick cold witness manifest lost its abandonment directory"
                    .to_string(),
            );
        }
        Ok(_) => validate_existing_secure_directory(
            &abandonment_root,
            "published-tick abandonment tombstone directory",
        )?,
        Err(error) => {
            return Err(format!(
                "inspect published-tick abandonment directory {}: {error}",
                abandonment_root.display()
            ));
        }
    }
    let first_path = abandonment_root.join(&first);
    match fs::symlink_metadata(&first_path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(_) => validate_existing_secure_directory(
            &first_path,
            "published-tick abandonment first-level shard",
        )?,
        Err(error) => {
            return Err(format!(
                "inspect published-tick abandonment shard {}: {error}",
                first_path.display()
            ));
        }
    }
    let second_path = first_path.join(&second);
    match fs::symlink_metadata(&second_path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(_) => validate_existing_secure_directory(
            &second_path,
            "published-tick abandonment second-level shard",
        )?,
        Err(error) => {
            return Err(format!(
                "inspect published-tick abandonment shard {}: {error}",
                second_path.display()
            ));
        }
    }
    let path = abandonment_tombstone_path(root, match_id);
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Ok(_) => {}
        Err(error) => {
            return Err(format!(
                "inspect published-tick abandonment tombstone {}: {error}",
                path.display()
            ));
        }
    }
    let (tombstone, sha256): (PublishedTickAbandonmentTombstone, String) =
        read_private_json_with_sha(
            &path,
            "published-tick abandonment tombstone",
            MAX_ABANDONMENT_TOMBSTONE_BYTES,
        )?;
    tombstone.validate()?;
    if tombstone.high_water.match_id != match_id
        || tombstone.high_water.journal_owner_id != owner.journal_owner_id
        || tombstone.high_water.physical_host_id != owner.physical_host_id
    {
        return Err(format!(
            "published-tick abandonment tombstone {} has the wrong filename or host journal identity",
            path.display()
        ));
    }
    Ok(Some((tombstone, sha256)))
}

fn load_cold_witness_if_exists(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &AckTombstoneManifest,
    match_id: Uuid,
) -> Result<Option<PublishedTickColdWitness>, String> {
    load_cold_witness_if_exists_with_sha(root, owner, manifest, match_id)
        .map(|record| record.map(|(witness, _)| witness))
}

fn load_cold_witness_if_exists_with_sha(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &AckTombstoneManifest,
    match_id: Uuid,
) -> Result<Option<(PublishedTickColdWitness, String)>, String> {
    let terminal = load_ack_tombstone_if_exists_with_sha(root, owner, match_id)?;
    let abandonment =
        load_abandonment_tombstone_if_exists_with_sha(root, owner, manifest, match_id)?;
    match (terminal, abandonment) {
        (Some(_), Some(_)) => Err(format!(
            "published-tick match {match_id} has mutually exclusive terminal and abandonment cold witnesses"
        )),
        (Some((tombstone, sha256)), None) => Ok(Some((
            PublishedTickColdWitness::TerminalAck(tombstone),
            sha256,
        ))),
        (None, Some((tombstone, sha256))) => Ok(Some((
            PublishedTickColdWitness::FailedClosedAbandonment(tombstone),
            sha256,
        ))),
        (None, None) => Ok(None),
    }
}

#[cfg(test)]
fn scan_ack_tombstone_page(
    root: &Path,
    owner: &JournalOwnerManifest,
    manifest: &AckTombstoneManifest,
    after_match_id: Option<Uuid>,
    limit: usize,
) -> Result<Vec<PublishedTickAckTombstone>, String> {
    manifest.validate(owner)?;
    let ack_root = root.join(ACK_TOMBSTONE_DIRECTORY);
    validate_existing_secure_directory(&ack_root, "published-tick ACK tombstone directory")?;
    let first_shards =
        list_hex_shard_directories(&ack_root, "published-tick ACK first-level shard")?;
    let mut page = Vec::with_capacity(limit);
    for (first_name, first_path) in first_shards {
        let second_shards =
            list_hex_shard_directories(&first_path, "published-tick ACK second-level shard")?;
        for (second_name, second_path) in second_shards {
            let mut entries = Vec::new();
            for entry in fs::read_dir(&second_path).map_err(|error| {
                format!(
                    "read published-tick ACK shard {}: {error}",
                    second_path.display()
                )
            })? {
                let entry = entry
                    .map_err(|error| format!("read published-tick ACK tombstone entry: {error}"))?;
                let path = entry.path();
                let name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .ok_or_else(|| {
                        "published-tick ACK shard contains a non-UTF8 filename".to_string()
                    })?;
                if is_atomic_temp_name(name) {
                    return Err(format!(
                        "published-tick ACK shard contains an unresolved atomic temporary {}",
                        path.display()
                    ));
                }
                let match_id = ack_tombstone_match_id(name).ok_or_else(|| {
                    format!(
                        "published-tick ACK shard contains unexpected entry {}",
                        path.display()
                    )
                })?;
                let (expected_first, expected_second) = ack_shard_components(match_id);
                if first_name != expected_first || second_name != expected_second {
                    return Err(format!(
                        "published-tick ACK tombstone is stored in the wrong shard: {}",
                        path.display()
                    ));
                }
                entries.push((match_id, path));
                if entries.len() > MAX_ACK_TOMBSTONES_PER_SHARD {
                    return Err(format!(
                        "published-tick ACK shard {} exceeds its {MAX_ACK_TOMBSTONES_PER_SHARD}-record bound",
                        second_path.display()
                    ));
                }
            }
            entries.sort_by_key(|(match_id, _)| *match_id);
            for (match_id, _) in entries {
                if after_match_id.is_some_and(|after| match_id <= after) {
                    continue;
                }
                let tombstone =
                    load_ack_tombstone_if_exists(root, owner, match_id)?.ok_or_else(|| {
                        format!("published-tick ACK tombstone {match_id} disappeared during audit")
                    })?;
                validate_tombstone_against_manifest(&tombstone, manifest)?;
                page.push(tombstone);
                if page.len() == limit {
                    return Ok(page);
                }
            }
        }
    }
    Ok(page)
}

#[cfg(test)]
fn list_hex_shard_directories(
    parent: &Path,
    label: &str,
) -> Result<Vec<(String, PathBuf)>, String> {
    let mut shards = Vec::new();
    for entry in fs::read_dir(parent)
        .map_err(|error| format!("read {label} directory {}: {error}", parent.display()))?
    {
        let entry = entry.map_err(|error| format!("read {label} entry: {error}"))?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| format!("{label} contains a non-UTF8 name"))?;
        if !is_lowercase_hex_shard_name(name) {
            return Err(format!(
                "{label} contains unexpected entry {}",
                path.display()
            ));
        }
        validate_existing_secure_directory(&path, label)?;
        shards.push((name.to_string(), path));
        if shards.len() > 256 {
            return Err(format!("{label} directory exceeds its 256-shard bound"));
        }
    }
    shards.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(shards)
}

fn ensure_ack_tombstone_shard(root: &Path, match_id: Uuid) -> Result<PathBuf, JournalWriteError> {
    let (first, second) = ack_shard_components(match_id);
    let ack_root = root.join(ACK_TOMBSTONE_DIRECTORY);
    validate_existing_secure_directory(&ack_root, "published-tick ACK tombstone directory")
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let first_path = ack_root.join(&first);
    ensure_secure_child_directory(&ack_root, &first_path)
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let second_path = first_path.join(&second);
    ensure_secure_child_directory(&first_path, &second_path)
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let mut count = 0usize;
    for entry in fs::read_dir(&second_path).map_err(|error| {
        JournalWriteError::DurabilityUncertain(format!(
            "read published-tick ACK shard {}: {error}",
            second_path.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!(
                "read published-tick ACK shard entry: {error}"
            ))
        })?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                JournalWriteError::DurabilityUncertain(
                    "published-tick ACK shard contains a non-UTF8 filename".to_string(),
                )
            })?;
        if is_atomic_temp_name(name) {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "published-tick ACK shard contains an unresolved atomic temporary {}",
                path.display()
            )));
        }
        let existing_id = ack_tombstone_match_id(name).ok_or_else(|| {
            JournalWriteError::DurabilityUncertain(format!(
                "published-tick ACK shard contains unexpected entry {}",
                path.display()
            ))
        })?;
        let (expected_first, expected_second) = ack_shard_components(existing_id);
        if expected_first != first || expected_second != second {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "published-tick ACK tombstone is stored in the wrong shard: {}",
                path.display()
            )));
        }
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!(
                "inspect published-tick ACK tombstone {}: {error}",
                path.display()
            ))
        })?;
        validate_private_regular_file(&path, &metadata, "published-tick ACK tombstone")
            .map_err(JournalWriteError::DurabilityUncertain)?;
        count += 1;
        if count >= MAX_ACK_TOMBSTONES_PER_SHARD {
            return Err(JournalWriteError::Rejected(format!(
                "published-tick ACK shard reached its {MAX_ACK_TOMBSTONES_PER_SHARD}-record bound"
            )));
        }
    }
    Ok(second_path)
}

fn ensure_abandonment_tombstone_shard(
    root: &Path,
    match_id: Uuid,
) -> Result<PathBuf, JournalWriteError> {
    let (first, second) = ack_shard_components(match_id);
    let abandonment_root = root.join(ABANDONMENT_TOMBSTONE_DIRECTORY);
    match fs::symlink_metadata(&abandonment_root) {
        Ok(_) => validate_existing_secure_directory(
            &abandonment_root,
            "published-tick abandonment tombstone directory",
        )
        .map_err(JournalWriteError::DurabilityUncertain)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            ensure_secure_child_directory(root, &abandonment_root)
                .map_err(JournalWriteError::DurabilityUncertain)?;
        }
        Err(error) => {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "inspect published-tick abandonment directory {}: {error}",
                abandonment_root.display()
            )));
        }
    }
    let first_path = abandonment_root.join(&first);
    ensure_secure_child_directory(&abandonment_root, &first_path)
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let second_path = first_path.join(&second);
    ensure_secure_child_directory(&first_path, &second_path)
        .map_err(JournalWriteError::DurabilityUncertain)?;
    let mut count = 0usize;
    for entry in fs::read_dir(&second_path).map_err(|error| {
        JournalWriteError::DurabilityUncertain(format!(
            "read published-tick abandonment shard {}: {error}",
            second_path.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!(
                "read published-tick abandonment shard entry: {error}"
            ))
        })?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                JournalWriteError::DurabilityUncertain(
                    "published-tick abandonment shard contains a non-UTF8 filename".to_string(),
                )
            })?;
        if is_atomic_temp_name(name) {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "published-tick abandonment shard contains an unresolved atomic temporary {}",
                path.display()
            )));
        }
        let existing_id = abandonment_tombstone_match_id(name).ok_or_else(|| {
            JournalWriteError::DurabilityUncertain(format!(
                "published-tick abandonment shard contains unexpected entry {}",
                path.display()
            ))
        })?;
        let (expected_first, expected_second) = ack_shard_components(existing_id);
        if expected_first != first || expected_second != second {
            return Err(JournalWriteError::DurabilityUncertain(format!(
                "published-tick abandonment tombstone is stored in the wrong shard: {}",
                path.display()
            )));
        }
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            JournalWriteError::DurabilityUncertain(format!(
                "inspect published-tick abandonment tombstone {}: {error}",
                path.display()
            ))
        })?;
        validate_private_regular_file(&path, &metadata, "published-tick abandonment tombstone")
            .map_err(JournalWriteError::DurabilityUncertain)?;
        count += 1;
        if count >= MAX_ACK_TOMBSTONES_PER_SHARD {
            return Err(JournalWriteError::Rejected(format!(
                "published-tick abandonment shard reached its {MAX_ACK_TOMBSTONES_PER_SHARD}-record bound"
            )));
        }
    }
    Ok(second_path)
}

fn ack_shard_components(match_id: Uuid) -> (String, String) {
    let simple = match_id.simple().to_string();
    (simple[0..2].to_string(), simple[2..4].to_string())
}

fn hot_record_match_id(name: &str) -> Option<Uuid> {
    let id = name.strip_prefix("published-")?.strip_suffix(".json")?;
    let parsed = Uuid::parse_str(id).ok()?;
    (parsed.to_string() == id).then_some(parsed)
}

fn ack_tombstone_match_id(name: &str) -> Option<Uuid> {
    let id = name.strip_prefix("acknowledged-")?.strip_suffix(".json")?;
    let parsed = Uuid::parse_str(id).ok()?;
    (parsed.to_string() == id).then_some(parsed)
}

fn abandonment_tombstone_match_id(name: &str) -> Option<Uuid> {
    let id = name.strip_prefix("abandoned-")?.strip_suffix(".json")?;
    let parsed = Uuid::parse_str(id).ok()?;
    (parsed.to_string() == id).then_some(parsed)
}

fn is_atomic_temp_name(name: &str) -> bool {
    let Some(rest) = name.strip_prefix(".published-") else {
        return false;
    };
    let Some((pid, id)) = rest.split_once(".tmp-") else {
        return false;
    };
    if pid.is_empty()
        || (pid.len() > 1 && pid.starts_with('0'))
        || !pid.bytes().all(|byte| byte.is_ascii_digit())
        || pid.parse::<u32>().ok().is_none_or(|pid| pid == 0)
    {
        return false;
    }
    Uuid::parse_str(id).is_ok_and(|parsed| parsed.to_string() == id)
}

#[cfg(test)]
fn is_lowercase_hex_shard_name(name: &str) -> bool {
    name.len() == 2
        && name
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(test)]
fn load_records(
    root: &Path,
    owner: &JournalOwnerManifest,
) -> Result<BTreeMap<Uuid, PublishedTickHighWater>, String> {
    let mut manifest = load_or_create_ack_manifest(root, owner)?;
    validate_manifest_latest_tombstone(root, owner, &manifest)?;
    load_hot_records(root, owner, &mut manifest)
}

fn read_private_json<T: DeserializeOwned>(
    path: &Path,
    label: &str,
    max_bytes: u64,
) -> Result<T, String> {
    read_private_json_with_sha(path, label, max_bytes).map(|(value, _)| value)
}

fn read_private_json_with_sha<T: DeserializeOwned>(
    path: &Path,
    label: &str,
    max_bytes: u64,
) -> Result<(T, String), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("inspect {label} {}: {error}", path.display()))?;
    validate_private_regular_file(path, &metadata, label)?;
    if metadata.len() > max_bytes {
        return Err(format!(
            "{label} {} exceeds {max_bytes} bytes",
            path.display()
        ));
    }
    let mut file =
        File::open(path).map_err(|error| format!("open {label} {}: {error}", path.display()))?;
    validate_open_file_identity(path, &file, label)?;
    let opened_metadata = file
        .metadata()
        .map_err(|error| format!("inspect opened {label} {}: {error}", path.display()))?;
    validate_private_regular_file(path, &opened_metadata, label)?;
    if !same_file_identity(&metadata, &opened_metadata) || metadata.len() != opened_metadata.len() {
        return Err(format!("{label} {} changed while opening", path.display()));
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    (&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| format!("read {label} {}: {error}", path.display()))?;
    if bytes.len() as u64 > max_bytes {
        return Err(format!(
            "{label} {} exceeds {max_bytes} bytes while reading",
            path.display()
        ));
    }
    let opened_after = file
        .metadata()
        .map_err(|error| format!("reinspect opened {label} {}: {error}", path.display()))?;
    let path_after = fs::symlink_metadata(path)
        .map_err(|error| format!("reinspect {label} {}: {error}", path.display()))?;
    validate_private_regular_file(path, &path_after, label)?;
    if !same_file_identity(&metadata, &opened_after)
        || !same_file_identity(&metadata, &path_after)
        || opened_after.len() != metadata.len()
        || path_after.len() != metadata.len()
        || bytes.len() as u64 != metadata.len()
    {
        return Err(format!("{label} {} changed while reading", path.display()));
    }
    let value = serde_json::from_slice(&bytes)
        .map_err(|error| format!("decode {label} {}: {error}", path.display()))?;
    Ok((value, sha256_hex(&bytes)))
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
            let owner: JournalOwnerManifest =
                read_private_json(&path, "published-tick owner manifest", MAX_RECORD_BYTES)?;
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

fn is_lowercase_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn is_canonical_positive_u64(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 20
        && !value.starts_with('0')
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && value.parse::<u64>().is_ok_and(|parsed| parsed > 0)
}

fn validate_database_lineage(
    system_identifier: &str,
    timeline_id: u32,
    wal_lsn: &str,
    label: &str,
) -> Result<(), String> {
    if !is_canonical_positive_u64(system_identifier) {
        return Err(format!(
            "{label} database system identifier must be canonical positive u64 text"
        ));
    }
    if timeline_id == 0 {
        return Err(format!(
            "{label} database timeline identifier must be positive"
        ));
    }
    if parse_postgres_wal_lsn(wal_lsn).is_none_or(|lsn| lsn == 0) {
        return Err(format!(
            "{label} database WAL LSN must be canonical positive PostgreSQL LSN text"
        ));
    }
    Ok(())
}

fn parse_postgres_wal_lsn(value: &str) -> Option<u64> {
    fn parse_canonical_hex_part(part: &str) -> Option<u32> {
        if part.is_empty()
            || part.len() > 8
            || (part != "0" && part.starts_with('0'))
            || !part
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'A'..=b'F'))
        {
            return None;
        }
        u32::from_str_radix(part, 16).ok()
    }

    let (high, low) = value.split_once('/')?;
    if low.contains('/') {
        return None;
    }
    let high = parse_canonical_hex_part(high)?;
    let low = parse_canonical_hex_part(low)?;
    Some((u64::from(high) << 32) | u64::from(low))
}

fn record_path(root: &Path, match_id: Uuid) -> PathBuf {
    root.join(format!("published-{match_id}.json"))
}

fn ack_tombstone_path(root: &Path, match_id: Uuid) -> PathBuf {
    let (first, second) = ack_shard_components(match_id);
    root.join(ACK_TOMBSTONE_DIRECTORY)
        .join(first)
        .join(second)
        .join(format!("acknowledged-{match_id}.json"))
}

fn abandonment_tombstone_path(root: &Path, match_id: Uuid) -> PathBuf {
    let (first, second) = ack_shard_components(match_id);
    root.join(ABANDONMENT_TOMBSTONE_DIRECTORY)
        .join(first)
        .join(second)
        .join(format!("abandoned-{match_id}.json"))
}

fn atomic_install(parent: &Path, target: &Path, payload: &[u8]) -> Result<(), JournalWriteError> {
    if target.parent() != Some(parent) {
        return Err(JournalWriteError::Rejected(format!(
            "atomic published-tick target {} is not inside {}",
            target.display(),
            parent.display()
        )));
    }
    let temp = parent.join(format!(
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
        sync_directory(parent).map_err(JournalWriteError::DurabilityUncertain)
    })();
    if result.is_err() && fs::remove_file(&temp).is_ok() {
        let _ = sync_directory(parent);
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
    let lock_root = shared_host_identity_lock_root()?;
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

#[cfg(target_os = "linux")]
fn shared_host_identity_lock_root() -> Result<PathBuf, String> {
    let per_user_runtime_root = PathBuf::from(format!("/run/user/{}", effective_user_id()));
    let metadata = fs::symlink_metadata(&per_user_runtime_root).map_err(|error| {
        format!(
            "the shared physical-host journal lock requires {}: {error}",
            per_user_runtime_root.display()
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return Err(format!(
            "the shared physical-host journal runtime root {} must be a real directory",
            per_user_runtime_root.display()
        ));
    }
    validate_owner(
        &per_user_runtime_root,
        &metadata,
        "published-tick shared runtime root",
    )?;
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o700 {
            return Err(format!(
                "published-tick shared runtime root {} must have mode 0700, found {mode:04o}",
                per_user_runtime_root.display()
            ));
        }
    }
    let lock_root = per_user_runtime_root.join("trnm-published-tick-host-locks");
    ensure_secure_directory(&lock_root)?;
    Ok(lock_root)
}

#[cfg(all(unix, not(target_os = "linux")))]
fn shared_host_identity_lock_root() -> Result<PathBuf, String> {
    let lock_root = std::env::temp_dir().join(format!(
        "trnm-published-tick-host-locks-{}",
        effective_user_id()
    ));
    ensure_secure_directory(&lock_root)?;
    Ok(lock_root)
}

#[cfg(not(unix))]
fn shared_host_identity_lock_root() -> Result<PathBuf, String> {
    let lock_root = std::env::temp_dir().join("trnm-published-tick-host-locks");
    ensure_secure_directory(&lock_root)?;
    Ok(lock_root)
}

fn validate_existing_secure_directory(path: &Path, label: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("inspect {label} {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return Err(format!(
            "{label} {} is not a real directory",
            path.display()
        ));
    }
    validate_owner(path, &metadata, label)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o700 {
            return Err(format!(
                "{label} {} must have mode 0700, found {mode:04o}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn ensure_secure_child_directory(parent: &Path, path: &Path) -> Result<(), String> {
    if path.parent() != Some(parent) {
        return Err(format!(
            "published-tick child directory {} is not directly inside {}",
            path.display(),
            parent.display()
        ));
    }
    validate_existing_secure_directory(parent, "published-tick parent directory")?;
    match fs::symlink_metadata(path) {
        Ok(_) => validate_existing_secure_directory(path, "published-tick child directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            builder
                .create(path)
                .map_err(|error| format!("create {}: {error}", path.display()))?;
            sync_directory(parent)?;
            sync_directory(path)?;
            validate_existing_secure_directory(path, "published-tick child directory")
        }
        Err(error) => Err(format!("inspect {}: {error}", path.display())),
    }
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

fn same_file_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        left.dev() == right.dev() && left.ino() == right.ino()
    }
    #[cfg(not(unix))]
    {
        let _ = (left, right);
        true
    }
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
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o600 {
            return Err(format!(
                "{label} {} must have mode 0600, found {mode:04o}",
                path.display()
            ));
        }
        if metadata.nlink() != 1 {
            return Err(format!(
                "{label} {} must have exactly one filesystem link",
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

    fn persist_test_record(
        root: &Path,
        records: &Mutex<BTreeMap<Uuid, PublishedTickHighWater>>,
        record: PublishedTickHighWater,
        durable_db_next_sequence: u64,
        durable_db_match_revision: u64,
    ) -> Result<(), JournalWriteError> {
        let owner = JournalOwnerManifest {
            contract_version: JOURNAL_OWNER_CONTRACT.to_string(),
            journal_owner_id: record.journal_owner_id,
            physical_host_id: record.physical_host_id.clone(),
        };
        let manifest = Mutex::new(
            load_or_create_ack_manifest(root, &owner).map_err(JournalWriteError::Rejected)?,
        );
        persist_record(
            root,
            records,
            &manifest,
            &owner,
            record,
            DurableDatabaseHighWater {
                next_sequence: durable_db_next_sequence,
                match_revision: durable_db_match_revision,
                next_input_sequences: input_cursors(),
            },
        )
    }

    fn ack_input(
        high_water: PublishedTickHighWater,
        acknowledged_at_unix_ms: u64,
    ) -> PublishedTickAckTombstoneInput {
        PublishedTickAckTombstoneInput {
            high_water,
            result_hash: "ab".repeat(32),
            settlement_state: "pending".to_string(),
            acknowledged_at_unix_ms,
            database_system_identifier: "72623859790382856".to_string(),
            database_timeline_id: 7,
            database_wal_lsn: "0/16B6C50".to_string(),
        }
    }

    fn abandonment_input(
        high_water: PublishedTickHighWater,
        abandoned_at_unix_ms: u64,
    ) -> PublishedTickAbandonmentTombstoneInput {
        PublishedTickAbandonmentTombstoneInput {
            high_water,
            failure_reason: "authority shutdown after exact durable checkpoint".to_string(),
            abandoned_at_unix_ms,
            database_system_identifier: "72623859790382856".to_string(),
            database_timeline_id: 7,
            database_wal_lsn: "0/16B6C50".to_string(),
        }
    }

    #[test]
    fn crash_reopen_recovers_exact_high_water() {
        let root = temp_dir("reopen");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let match_id = Uuid::new_v4();
        let expected = record(&owner, "instance-a", match_id, Uuid::new_v4(), 7, 91);
        persist_test_record(&root, &records, expected.clone(), 4, 5).unwrap();

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
    fn missing_or_corrupt_ack_manifest_fails_closed_without_scanning_repair() {
        let root = temp_dir("manifest-fail-closed");
        let owner = owner(&root, "host-a");
        load_or_create_ack_manifest(&root, &owner).unwrap();
        let manifest_path = root.join(ACK_MANIFEST_FILE);
        fs::remove_file(&manifest_path).unwrap();
        sync_directory(&root).unwrap();
        assert!(load_or_create_ack_manifest(&root, &owner)
            .unwrap_err()
            .contains("exists without its durable manifest"));

        fs::remove_dir_all(root.join(ACK_TOMBSTONE_DIRECTORY)).unwrap();
        atomic_install(&root, &manifest_path, b"{not-json").unwrap();
        assert!(load_or_create_ack_manifest(&root, &owner)
            .unwrap_err()
            .contains("decode published-tick cold witness manifest"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn exact_root_atomic_temporary_is_cleaned_but_lookalike_is_rejected() {
        let root = temp_dir("atomic-temp");
        let owner = owner(&root, "host-a");
        load_or_create_ack_manifest(&root, &owner).unwrap();
        let exact = root.join(format!(".published-123.tmp-{}", Uuid::new_v4()));
        atomic_install(&root, &exact, b"unfinished").unwrap();
        assert!(load_records(&root, &owner).unwrap().is_empty());
        assert!(!exact.exists());

        let lookalike = root.join(format!(".published-0123.tmp-{}", Uuid::new_v4()));
        atomic_install(&root, &lookalike, b"unfinished").unwrap();
        assert!(load_records(&root, &owner)
            .unwrap_err()
            .contains("unexpected entry"));
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
        persist_test_record(&root, &records, current.clone(), 4, 5).unwrap();

        let rollback = record(&owner, "instance-a", match_id, generation, 3, 99);
        assert!(persist_test_record(&root, &records, rollback, 4, 5)
            .unwrap_err()
            .message()
            .contains("cannot regress"));
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
        assert!(persist_test_record(&root, &records, ahead, 8, 5)
            .unwrap_err()
            .message()
            .contains("ahead of durable database"));
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
        persist_test_record(&root, &records, current.clone(), 4, 5).unwrap();

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
        persist_test_record(&root, &records, adopted, 4, 5).unwrap();

        let advanced = record(&owner, "instance-c", match_id, Uuid::new_v4(), 20, 51);
        assert!(persist_test_record(&root, &records, advanced, 4, 5)
            .unwrap_err()
            .message()
            .contains("must first adopt"));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn physical_host_lock_uses_a_shared_private_runtime_directory() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};

        let lock_root = shared_host_identity_lock_root().unwrap();
        assert!(lock_root.is_absolute());
        assert!(!lock_root.starts_with("/tmp"));
        let runtime_root = PathBuf::from(format!("/run/user/{}", effective_user_id()));
        assert!(lock_root.starts_with(runtime_root));
        let metadata = fs::symlink_metadata(&lock_root).unwrap();
        assert!(metadata.file_type().is_dir());
        assert_eq!(metadata.uid(), effective_user_id());
        assert_eq!(metadata.permissions().mode() & 0o777, 0o700);
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
    async fn terminal_ack_seal_reopens_as_cold_pitr_rollback_witness() {
        let root = temp_dir("terminal-tombstone");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("terminal-tombstone")).unwrap();
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
            .record(terminal.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        let tombstone = journal
            .seal_terminal_ack(ack_input(terminal.clone(), 1_700_000_000_000))
            .await
            .unwrap();
        assert_eq!(tombstone.high_water, terminal);
        assert!(journal.high_water(match_id).unwrap().is_none());
        assert_eq!(
            journal.ack_tombstone(match_id).unwrap(),
            Some(tombstone.clone())
        );
        assert!(journal.recorded_match_ids().unwrap().is_empty());
        assert_eq!(journal.ack_tombstone_count().unwrap(), 1);
        assert_eq!(
            journal.latest_cold_witness().unwrap(),
            Some(PublishedTickColdWitness::TerminalAck(tombstone.clone()))
        );
        assert!(!record_path(&root, match_id).exists());
        assert!(ack_tombstone_path(&root, match_id).exists());

        let manifest = journal.ack_manifest.lock().unwrap().clone();
        validate_manifest_latest_tombstone(&root, &journal.owner, &manifest).unwrap();
        assert!(load_records(&root, &journal.owner).unwrap().is_empty());
        assert_eq!(
            load_ack_tombstone_if_exists(&root, &journal.owner, match_id).unwrap(),
            Some(tombstone)
        );
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn abandonment_seal_is_cold_idempotent_and_does_not_claim_terminal_ack() {
        let root = temp_dir("abandonment-tombstone");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("abandonment-tombstone")).unwrap();
        let match_id = Uuid::new_v4();
        let running = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                Uuid::new_v4(),
                1,
                34,
                "running",
            ))
            .unwrap();
        journal
            .record(running.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        let input = abandonment_input(running.clone(), 1_700_000_000_100);
        let tombstone = journal.seal_abandonment(input.clone()).await.unwrap();
        assert_eq!(tombstone.high_water, running);
        assert_eq!(journal.seal_abandonment(input).await.unwrap(), tombstone);
        assert!(journal.high_water(match_id).unwrap().is_none());
        assert!(journal.ack_tombstone(match_id).unwrap().is_none());
        assert_eq!(
            journal.abandonment_tombstone(match_id).unwrap(),
            Some(tombstone.clone())
        );
        assert_eq!(journal.ack_tombstone_count().unwrap(), 0);
        assert_eq!(journal.abandonment_tombstone_count().unwrap(), 1);
        assert_eq!(journal.cold_witness_count().unwrap(), 1);
        assert_eq!(
            journal.latest_cold_witness().unwrap(),
            Some(PublishedTickColdWitness::FailedClosedAbandonment(
                tombstone.clone()
            ))
        );
        assert!(!record_path(&root, match_id).exists());
        assert!(abandonment_tombstone_path(&root, match_id).exists());
        let manifest = journal.ack_manifest.lock().unwrap().clone();
        validate_manifest_latest_tombstone(&root, &journal.owner, &manifest).unwrap();
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reopen_reconciles_exact_hot_and_uncommitted_abandonment_cold_file() {
        let root = temp_dir("abandonment-dual-file-crash");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let match_id = Uuid::new_v4();
        let running = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            running.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let tombstone = PublishedTickAbandonmentTombstone::new(
            abandonment_input(running.clone(), 1_700_000_000_101),
            1,
        )
        .unwrap();
        let payload = serde_json::to_vec_pretty(&tombstone).unwrap();
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &abandonment_tombstone_path(&root, match_id),
            &payload,
        )
        .unwrap();

        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let reopened = load_hot_records(&root, &owner, &mut reopened_manifest).unwrap();
        assert!(reopened.is_empty());
        assert_eq!(reopened_manifest.terminal_tombstone_count, 0);
        assert_eq!(reopened_manifest.abandonment_tombstone_count, 1);
        assert_eq!(reopened_manifest.committed_seal_sequence, 1);
        validate_manifest_latest_tombstone(&root, &owner, &reopened_manifest).unwrap();
        assert!(!record_path(&root, match_id).exists());
        assert_eq!(
            load_abandonment_tombstone_if_exists(&root, &owner, &reopened_manifest, match_id,)
                .unwrap(),
            Some(tombstone)
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reopen_removes_exact_hot_after_committed_abandonment_manifest() {
        let root = temp_dir("abandonment-manifest-before-unlink");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let match_id = Uuid::new_v4();
        let running = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            running.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let tombstone = seal_abandonment_tombstone(
            &root,
            &records,
            &manifest,
            &owner,
            abandonment_input(running.clone(), 1_700_000_000_111),
        )
        .unwrap();
        let committed_manifest = manifest.lock().unwrap().clone();
        assert_eq!(tombstone.journal_seal_sequence, 1);
        assert_eq!(
            committed_manifest.latest_witness,
            Some(PublishedTickColdWitness::FailedClosedAbandonment(tombstone))
        );
        assert_eq!(
            committed_manifest.committed_seal_sequence,
            committed_manifest
                .latest_witness
                .as_ref()
                .unwrap()
                .journal_seal_sequence()
        );
        validate_manifest_latest_tombstone(&root, &owner, &committed_manifest).unwrap();
        // Recreate the exact durable state after manifest fsync and before
        // the honest sealer unlinks its hot witness.
        atomic_install(
            &root,
            &record_path(&root, match_id),
            &serde_json::to_vec_pretty(&running).unwrap(),
        )
        .unwrap();

        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let reopened = load_hot_records(&root, &owner, &mut reopened_manifest).unwrap();
        assert!(reopened.is_empty());
        assert!(!record_path(&root, match_id).exists());
        assert_eq!(reopened_manifest.abandonment_tombstone_count, 1);
        assert_eq!(reopened_manifest.committed_seal_sequence, 1);
        assert_eq!(reopened_manifest, committed_manifest);
        validate_manifest_latest_tombstone(&root, &owner, &reopened_manifest).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reopen_removes_exact_hot_after_committed_terminal_manifest() {
        let root = temp_dir("terminal-manifest-before-unlink");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let prior_match_id = Uuid::new_v4();
        let prior_running = record(&owner, "instance-a", prior_match_id, Uuid::new_v4(), 3, 76);
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            prior_running.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let prior_abandonment = seal_abandonment_tombstone(
            &root,
            &records,
            &manifest,
            &owner,
            abandonment_input(prior_running, 1_700_000_000_112),
        )
        .unwrap();
        assert_eq!(prior_abandonment.journal_seal_sequence, 1);
        let match_id = Uuid::new_v4();
        let mut terminal = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        terminal.phase = "complete".to_string();
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            terminal.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let tombstone = seal_terminal_ack_tombstone(
            &root,
            &records,
            &manifest,
            &owner,
            ack_input(terminal.clone(), 1_700_000_000_112),
        )
        .unwrap();
        let committed_manifest = manifest.lock().unwrap().clone();
        assert_eq!(tombstone.journal_seal_sequence, 2);
        assert_eq!(
            tombstone.database_wal_lsn,
            prior_abandonment.database_wal_lsn
        );
        assert_eq!(
            committed_manifest.latest_witness,
            Some(PublishedTickColdWitness::TerminalAck(tombstone))
        );
        assert_eq!(
            committed_manifest.committed_seal_sequence,
            committed_manifest
                .latest_witness
                .as_ref()
                .unwrap()
                .journal_seal_sequence()
        );
        validate_manifest_latest_tombstone(&root, &owner, &committed_manifest).unwrap();
        // This is the same-LSN, cross-kind manifest-fsync/hot-unlink crash
        // boundary that previously left latest_witness on sequence 1.
        atomic_install(
            &root,
            &record_path(&root, match_id),
            &serde_json::to_vec_pretty(&terminal).unwrap(),
        )
        .unwrap();

        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let reopened = load_hot_records(&root, &owner, &mut reopened_manifest).unwrap();
        assert!(reopened.is_empty());
        assert!(!record_path(&root, match_id).exists());
        assert_eq!(reopened_manifest.terminal_tombstone_count, 1);
        assert_eq!(reopened_manifest.abandonment_tombstone_count, 1);
        assert_eq!(reopened_manifest.committed_seal_sequence, 2);
        assert_eq!(reopened_manifest, committed_manifest);
        validate_manifest_latest_tombstone(&root, &owner, &reopened_manifest).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn old_committed_cold_file_cannot_consume_a_reintroduced_hot_witness() {
        let root = temp_dir("old-cold-overlap");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let first_match = Uuid::new_v4();
        let mut first = record(&owner, "instance-a", first_match, Uuid::new_v4(), 3, 77);
        first.phase = "complete".to_string();
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            first.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        seal_terminal_ack_tombstone(
            &root,
            &records,
            &manifest,
            &owner,
            ack_input(first.clone(), 1_700_000_000_113),
        )
        .unwrap();

        let second_match = Uuid::new_v4();
        let second = record(&owner, "instance-a", second_match, Uuid::new_v4(), 4, 78);
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            second.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let mut second_input = abandonment_input(second, 1_700_000_000_114);
        second_input.database_wal_lsn = "0/16B6C51".to_string();
        seal_abandonment_tombstone(&root, &records, &manifest, &owner, second_input).unwrap();
        atomic_install(
            &root,
            &record_path(&root, first_match),
            &serde_json::to_vec_pretty(&first).unwrap(),
        )
        .unwrap();

        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        assert!(load_hot_records(&root, &owner, &mut reopened_manifest)
            .unwrap_err()
            .contains("not the manifest latest crash boundary"));
        assert!(record_path(&root, first_match).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn conflicting_hot_and_abandonment_cold_fails_closed() {
        let root = temp_dir("abandonment-dual-file-conflict");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let match_id = Uuid::new_v4();
        let running = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            running.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let mut conflicting = running;
        conflicting.snapshot_hash = "cd".repeat(32);
        let tombstone = PublishedTickAbandonmentTombstone::new(
            abandonment_input(conflicting, 1_700_000_000_102),
            1,
        )
        .unwrap();
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &abandonment_tombstone_path(&root, match_id),
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        assert!(load_hot_records(&root, &owner, &mut reopened_manifest)
            .unwrap_err()
            .contains("conflicts with its cold witness"));
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn real_legacy_v1_layout_stays_unchanged_until_first_abandonment_seal() {
        let root = temp_dir("legacy-layout-non-mutating");
        let host = unique_host("legacy-layout-non-mutating");
        let owner = owner(&root, &host);
        ensure_secure_child_directory(&root, &root.join(ACK_TOMBSTONE_DIRECTORY)).unwrap();
        let legacy = LegacyAckTombstoneManifest {
            contract_version: LEGACY_ACK_MANIFEST_CONTRACT.to_string(),
            journal_owner_id: owner.journal_owner_id,
            physical_host_id: owner.physical_host_id.clone(),
            tombstone_count: 0,
            committed_seal_sequence: 0,
            database_system_identifier: None,
            database_timeline_id: None,
            latest_tombstone: None,
            latest_tombstone_sha256: None,
        };
        let payload = serde_json::to_vec_pretty(&legacy).unwrap();
        atomic_install(&root, &root.join(ACK_MANIFEST_FILE), &payload).unwrap();

        let normalized = load_or_create_ack_manifest(&root, &owner).unwrap();
        assert_eq!(normalized.cold_witness_count().unwrap(), 0);
        assert!(!root.join(ABANDONMENT_TOMBSTONE_DIRECTORY).exists());
        assert_eq!(fs::read(root.join(ACK_MANIFEST_FILE)).unwrap(), payload);

        let journal = PublishedTickJournal::open(root.clone(), host).unwrap();
        assert!(!root.join(ABANDONMENT_TOMBSTONE_DIRECTORY).exists());
        let match_id = Uuid::new_v4();
        let running = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                Uuid::new_v4(),
                1,
                10,
                "running",
            ))
            .unwrap();
        journal
            .record(running.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        assert!(!root.join(ABANDONMENT_TOMBSTONE_DIRECTORY).exists());
        journal
            .seal_abandonment(abandonment_input(running, 1_700_000_000_102))
            .await
            .unwrap();
        assert!(root.join(ABANDONMENT_TOMBSTONE_DIRECTORY).is_dir());
        let on_disk: AckManifestOnDisk = read_private_json(
            &root.join(ACK_MANIFEST_FILE),
            "published-tick cold witness manifest",
            MAX_ACK_MANIFEST_BYTES,
        )
        .unwrap();
        assert!(matches!(on_disk, AckManifestOnDisk::Current(_)));
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn legacy_v1_manifest_is_read_without_rewrite_then_upgraded_on_new_seal() {
        let root = temp_dir("legacy-manifest-upgrade");
        let host = unique_host("legacy-manifest-upgrade");
        let owner = owner(&root, &host);
        load_or_create_ack_manifest(&root, &owner).unwrap();
        let terminal_match = Uuid::parse_str("ffffffff-ffff-4fff-8fff-ffffffffffff").unwrap();
        let mut terminal = record(&owner, "instance-a", terminal_match, Uuid::new_v4(), 3, 77);
        terminal.phase = "complete".to_string();
        let terminal_tombstone =
            PublishedTickAckTombstone::new(ack_input(terminal, 1_700_000_000_103), 1).unwrap();
        let terminal_payload = serde_json::to_vec_pretty(&terminal_tombstone).unwrap();
        let terminal_sha256 = sha256_hex(&terminal_payload);
        let shard = ensure_ack_tombstone_shard(&root, terminal_match).unwrap();
        atomic_install(
            &shard,
            &ack_tombstone_path(&root, terminal_match),
            &terminal_payload,
        )
        .unwrap();
        let second_terminal_match =
            Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap();
        let mut second_terminal = record(
            &owner,
            "instance-a",
            second_terminal_match,
            Uuid::new_v4(),
            3,
            78,
        );
        second_terminal.phase = "complete".to_string();
        let second_terminal_tombstone = PublishedTickAckTombstone::new(
            ack_input(second_terminal.clone(), 1_700_000_000_104),
            2,
        )
        .unwrap();
        assert_eq!(
            second_terminal_tombstone.database_wal_lsn,
            terminal_tombstone.database_wal_lsn
        );
        let second_payload = serde_json::to_vec_pretty(&second_terminal_tombstone).unwrap();
        let second_shard = ensure_ack_tombstone_shard(&root, second_terminal_match).unwrap();
        atomic_install(
            &second_shard,
            &ack_tombstone_path(&root, second_terminal_match),
            &second_payload,
        )
        .unwrap();
        let legacy = LegacyAckTombstoneManifest {
            contract_version: LEGACY_ACK_MANIFEST_CONTRACT.to_string(),
            journal_owner_id: owner.journal_owner_id,
            physical_host_id: owner.physical_host_id.clone(),
            tombstone_count: 2,
            committed_seal_sequence: 2,
            database_system_identifier: Some(terminal_tombstone.database_system_identifier.clone()),
            database_timeline_id: Some(terminal_tombstone.database_timeline_id),
            // V1 used WAL/tie-key maximum semantics, so an honest same-LSN
            // seal at sequence 2 could leave sequence 1 as its sentinel.
            latest_tombstone: Some(terminal_tombstone.clone()),
            latest_tombstone_sha256: Some(terminal_sha256),
        };
        let legacy_payload = serde_json::to_vec_pretty(&legacy).unwrap();
        atomic_install(&root, &root.join(ACK_MANIFEST_FILE), &legacy_payload).unwrap();
        // V1 sequence 2 committed with its same-LSN sentinel still on
        // sequence 1, then the process crashed before unlinking sequence 2's
        // exact hot witness.
        atomic_install(
            &root,
            &record_path(&root, second_terminal_match),
            &serde_json::to_vec_pretty(&second_terminal).unwrap(),
        )
        .unwrap();

        let normalized = load_or_create_ack_manifest(&root, &owner).unwrap();
        assert_eq!(normalized.terminal_tombstone_count, 2);
        assert_eq!(normalized.abandonment_tombstone_count, 0);
        assert_eq!(normalized.committed_seal_sequence, 2);
        assert_eq!(
            normalized
                .latest_witness
                .as_ref()
                .unwrap()
                .journal_seal_sequence(),
            1
        );
        assert!(normalized.legacy_latest_semantics);
        assert_eq!(
            fs::read(root.join(ACK_MANIFEST_FILE)).unwrap(),
            legacy_payload
        );
        assert!(record_path(&root, second_terminal_match).exists());

        let journal = PublishedTickJournal::open(root.clone(), host).unwrap();
        assert_eq!(journal.ack_tombstone_count().unwrap(), 2);
        assert!(journal.high_water(second_terminal_match).unwrap().is_none());
        assert!(!record_path(&root, second_terminal_match).exists());
        let running_match = Uuid::new_v4();
        let running = journal
            .new_record(record_input(
                "instance-a",
                running_match,
                Uuid::new_v4(),
                4,
                78,
                "running",
            ))
            .unwrap();
        journal
            .record(running.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        let abandonment = journal
            .seal_abandonment(abandonment_input(running, 1_700_000_000_105))
            .await
            .unwrap();
        assert_eq!(abandonment.journal_seal_sequence, 3);
        assert_eq!(journal.ack_tombstone_count().unwrap(), 2);
        assert_eq!(journal.abandonment_tombstone_count().unwrap(), 1);
        assert_eq!(journal.cold_witness_count().unwrap(), 3);
        assert_eq!(
            journal.latest_cold_witness().unwrap(),
            Some(PublishedTickColdWitness::FailedClosedAbandonment(
                abandonment
            ))
        );
        let on_disk: AckManifestOnDisk = read_private_json(
            &root.join(ACK_MANIFEST_FILE),
            "published-tick cold witness manifest",
            MAX_ACK_MANIFEST_BYTES,
        )
        .unwrap();
        let AckManifestOnDisk::Current(upgraded) = on_disk else {
            panic!("legacy manifest was not upgraded on the next seal")
        };
        assert_eq!(upgraded.terminal_tombstone_count, 2);
        assert_eq!(upgraded.abandonment_tombstone_count, 1);
        assert_eq!(upgraded.committed_seal_sequence, 3);
        assert_eq!(
            upgraded
                .latest_witness
                .as_ref()
                .unwrap()
                .journal_seal_sequence(),
            upgraded.committed_seal_sequence
        );
        assert!(!upgraded.legacy_latest_semantics);
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn legacy_v1_manifest_cannot_hide_an_abandonment_witness_tree() {
        let root = temp_dir("legacy-manifest-abandonment-rollback");
        let owner = owner(&root, "host-a");
        load_or_create_ack_manifest(&root, &owner).unwrap();
        let match_id = Uuid::new_v4();
        let running = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        let tombstone = PublishedTickAbandonmentTombstone::new(
            abandonment_input(running, 1_700_000_000_105),
            1,
        )
        .unwrap();
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &abandonment_tombstone_path(&root, match_id),
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        let legacy = LegacyAckTombstoneManifest {
            contract_version: LEGACY_ACK_MANIFEST_CONTRACT.to_string(),
            journal_owner_id: owner.journal_owner_id,
            physical_host_id: owner.physical_host_id.clone(),
            tombstone_count: 0,
            committed_seal_sequence: 0,
            database_system_identifier: None,
            database_timeline_id: None,
            latest_tombstone: None,
            latest_tombstone_sha256: None,
        };
        atomic_install(
            &root,
            &root.join(ACK_MANIFEST_FILE),
            &serde_json::to_vec_pretty(&legacy).unwrap(),
        )
        .unwrap();

        assert!(load_or_create_ack_manifest(&root, &owner)
            .unwrap_err()
            .contains("cannot cover a non-empty abandonment witness tree"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reopen_reconciles_exact_hot_and_cold_crash_state() {
        let root = temp_dir("dual-file-crash");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let match_id = Uuid::new_v4();
        let mut terminal = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        terminal.phase = "complete".to_string();
        terminal.validate().unwrap();
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            terminal.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let tombstone =
            PublishedTickAckTombstone::new(ack_input(terminal.clone(), 1_700_000_000_001), 1)
                .unwrap();
        let payload = serde_json::to_vec_pretty(&tombstone).unwrap();
        let shard = ensure_ack_tombstone_shard(&root, match_id).unwrap();
        atomic_install(&shard, &ack_tombstone_path(&root, match_id), &payload).unwrap();

        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let reopened = load_hot_records(&root, &owner, &mut reopened_manifest).unwrap();
        assert!(reopened.is_empty());
        assert_eq!(reopened_manifest.terminal_tombstone_count, 1);
        assert_eq!(reopened_manifest.abandonment_tombstone_count, 0);
        assert_eq!(reopened_manifest.committed_seal_sequence, 1);
        validate_manifest_latest_tombstone(&root, &owner, &reopened_manifest).unwrap();
        assert!(!record_path(&root, match_id).exists());
        assert_eq!(
            load_ack_tombstone_if_exists(&root, &owner, match_id).unwrap(),
            Some(tombstone)
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn conflicting_hot_and_cold_crash_state_fails_closed() {
        let root = temp_dir("dual-file-conflict");
        let owner = owner(&root, "host-a");
        let records = Mutex::new(BTreeMap::new());
        let manifest = Mutex::new(load_or_create_ack_manifest(&root, &owner).unwrap());
        let match_id = Uuid::new_v4();
        let mut terminal = record(&owner, "instance-a", match_id, Uuid::new_v4(), 3, 77);
        terminal.phase = "complete".to_string();
        persist_record(
            &root,
            &records,
            &manifest,
            &owner,
            terminal.clone(),
            DurableDatabaseHighWater {
                next_sequence: 4,
                match_revision: 5,
                next_input_sequences: input_cursors(),
            },
        )
        .unwrap();
        let mut conflicting = terminal.clone();
        conflicting.snapshot_hash = "cd".repeat(32);
        let tombstone =
            PublishedTickAckTombstone::new(ack_input(conflicting, 1_700_000_000_002), 1).unwrap();
        let shard = ensure_ack_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &ack_tombstone_path(&root, match_id),
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        let mut reopened_manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        assert!(load_hot_records(&root, &owner, &mut reopened_manifest)
            .unwrap_err()
            .contains("conflicts with its cold witness"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cold_history_does_not_consume_the_ten_thousand_hot_slots() {
        let simulated_cold_history = MAX_RECORDS * 100;
        assert!(simulated_cold_history > MAX_RECORDS);
        assert!(validate_hot_record_capacity(MAX_RECORDS, false).is_ok());
        assert!(validate_hot_record_capacity(MAX_RECORDS - 1, true).is_ok());
        assert!(validate_hot_record_capacity(MAX_RECORDS, true).is_err());
        assert!(validate_hot_record_capacity(MAX_RECORDS + 1, false).is_err());
    }

    #[tokio::test]
    async fn tombstoned_match_cannot_recreate_a_hot_record() {
        let root = temp_dir("tombstone-no-recreate");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("tombstone-no-recreate")).unwrap();
        let match_id = Uuid::new_v4();
        let terminal = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                Uuid::new_v4(),
                1,
                10,
                "complete",
            ))
            .unwrap();
        journal
            .record(terminal.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        journal
            .seal_terminal_ack(ack_input(terminal.clone(), 1_700_000_000_003))
            .await
            .unwrap();
        let error = journal
            .record(terminal, 4, 5, input_cursors())
            .await
            .unwrap_err();
        assert!(error.contains("durable cold witness"));
        assert!(journal.high_water(match_id).unwrap().is_none());
        assert!(journal.is_operational());
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn abandoned_match_cannot_recreate_a_hot_record() {
        let root = temp_dir("abandonment-no-recreate");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("abandonment-no-recreate"))
                .unwrap();
        let match_id = Uuid::new_v4();
        let running = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                Uuid::new_v4(),
                1,
                10,
                "running",
            ))
            .unwrap();
        journal
            .record(running.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        journal
            .seal_abandonment(abandonment_input(running.clone(), 1_700_000_000_105))
            .await
            .unwrap();
        let error = journal
            .record(running, 4, 5, input_cursors())
            .await
            .unwrap_err();
        assert!(error.contains("durable cold witness"));
        assert!(journal.high_water(match_id).unwrap().is_none());
        assert!(journal.is_operational());
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn terminal_and_abandonment_files_for_one_match_are_mutually_exclusive() {
        let root = temp_dir("mutually-exclusive-cold");
        let owner = owner(&root, "host-a");
        load_or_create_ack_manifest(&root, &owner).unwrap();
        let match_id = Uuid::new_v4();
        let running = record(&owner, "instance-a", match_id, Uuid::new_v4(), 1, 10);
        let mut terminal = running.clone();
        terminal.phase = "complete".to_string();
        let ack = PublishedTickAckTombstone::new(ack_input(terminal, 100), 1).unwrap();
        let abandonment =
            PublishedTickAbandonmentTombstone::new(abandonment_input(running, 101), 2).unwrap();
        let ack_shard = ensure_ack_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &ack_shard,
            &ack_tombstone_path(&root, match_id),
            &serde_json::to_vec_pretty(&ack).unwrap(),
        )
        .unwrap();
        let abandonment_shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &abandonment_shard,
            &abandonment_tombstone_path(&root, match_id),
            &serde_json::to_vec_pretty(&abandonment).unwrap(),
        )
        .unwrap();
        let manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        assert!(
            load_cold_witness_if_exists(&root, &owner, &manifest, match_id)
                .unwrap_err()
                .contains("mutually exclusive")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn manifest_latest_validation_rejects_two_cold_witness_kinds_for_one_match() {
        let root = temp_dir("mutually-exclusive-cold-reopen");
        let host = unique_host("mutually-exclusive-cold-reopen");
        let journal = PublishedTickJournal::open(root.clone(), host.clone()).unwrap();
        let match_id = Uuid::new_v4();
        let generation = Uuid::new_v4();
        let mut terminal = record(&journal.owner, "instance-a", match_id, generation, 1, 10);
        terminal.phase = "complete".to_string();
        journal
            .record(terminal.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        journal
            .seal_terminal_ack(ack_input(terminal, 100))
            .await
            .unwrap();
        let running = record(&journal.owner, "instance-a", match_id, generation, 1, 10);
        let abandonment =
            PublishedTickAbandonmentTombstone::new(abandonment_input(running, 101), 2).unwrap();
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &abandonment_tombstone_path(&root, match_id),
            &serde_json::to_vec_pretty(&abandonment).unwrap(),
        )
        .unwrap();
        let manifest = journal.ack_manifest.lock().unwrap().clone();
        let error = validate_manifest_latest_tombstone(&root, &journal.owner, &manifest)
            .expect_err("manifest validation must reject mutually exclusive cold witnesses");
        assert!(error.contains("mutually exclusive"));
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn cold_tombstones_are_retained_and_explicitly_paged_in_uuid_order() {
        let root = temp_dir("retained-page");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("retained-page")).unwrap();
        let first_match = Uuid::new_v4();
        let second_match = Uuid::new_v4();
        let first = journal
            .new_record(record_input(
                "instance-a",
                first_match,
                Uuid::new_v4(),
                1,
                10,
                "complete",
            ))
            .unwrap();
        let second = journal
            .new_record(record_input(
                "instance-a",
                second_match,
                Uuid::new_v4(),
                1,
                11,
                "complete",
            ))
            .unwrap();
        journal
            .record(first.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        journal
            .seal_terminal_ack(ack_input(first, 100))
            .await
            .unwrap();
        journal
            .record(second.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        let mut second_ack = ack_input(second, 200);
        second_ack.database_wal_lsn = "0/16B6C51".to_string();
        journal.seal_terminal_ack(second_ack).await.unwrap();

        assert!(journal.ack_tombstone(first_match).unwrap().is_some());
        assert!(journal.ack_tombstone(second_match).unwrap().is_some());
        assert_eq!(
            journal
                .latest_cold_witness()
                .unwrap()
                .unwrap()
                .high_water()
                .match_id,
            second_match
        );
        assert!(ack_tombstone_path(&root, first_match).exists());
        assert!(ack_tombstone_path(&root, second_match).exists());
        assert_eq!(journal.ack_tombstone_count().unwrap(), 2);
        let page = journal.ack_tombstones_page(None, 2).unwrap();
        assert_eq!(page.len(), 2);
        assert!(page[0].high_water.match_id < page[1].high_water.match_id);
        let second_page = journal
            .ack_tombstones_page(Some(page[0].high_water.match_id), 1)
            .unwrap();
        assert_eq!(second_page, vec![page[1].clone()]);
        assert!(journal
            .ack_tombstones_page(None, MAX_ACK_TOMBSTONE_PAGE_SIZE + 1)
            .is_err());
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ack_tombstone_requires_canonical_database_lineage() {
        let root = temp_dir("ack-lineage");
        let owner = owner(&root, "host-a");
        let mut terminal = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        terminal.phase = "complete".to_string();
        let mut input = ack_input(terminal.clone(), 100);
        input.database_system_identifier = "072623859790382856".to_string();
        assert!(PublishedTickAckTombstone::new(input, 1).is_err());
        let mut input = ack_input(terminal.clone(), 100);
        input.database_timeline_id = 0;
        assert!(PublishedTickAckTombstone::new(input, 1).is_err());
        let mut input = ack_input(terminal, 100);
        input.database_wal_lsn = "0/016B6C50".to_string();
        assert!(PublishedTickAckTombstone::new(input, 1).is_err());
        let mut terminal = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        terminal.phase = "complete".to_string();
        let mut input = ack_input(terminal, 100);
        input.database_wal_lsn = "0/0".to_string();
        assert!(PublishedTickAckTombstone::new(input, 1).is_err());
        assert_eq!(parse_postgres_wal_lsn("1/ABCDEF01"), Some(0x1_abcdef01));
        assert!(parse_postgres_wal_lsn("1/abcdef01").is_none());
        assert!(parse_postgres_wal_lsn("100000000/0").is_none());
        assert!(parse_postgres_wal_lsn("1/00000000").is_none());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn abandonment_tombstone_requires_strict_schema_reason_and_database_lineage() {
        let root = temp_dir("abandonment-lineage");
        let owner = owner(&root, "host-a");
        let running = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        let mut input = abandonment_input(running.clone(), 100);
        input.failure_reason = "\n".to_string();
        assert!(PublishedTickAbandonmentTombstone::new(input, 1).is_err());
        let mut input = abandonment_input(running.clone(), 100);
        input.database_system_identifier = "072623859790382856".to_string();
        assert!(PublishedTickAbandonmentTombstone::new(input, 1).is_err());
        let mut input = abandonment_input(running.clone(), 100);
        input.database_timeline_id = 0;
        assert!(PublishedTickAbandonmentTombstone::new(input, 1).is_err());
        let mut input = abandonment_input(running.clone(), 100);
        input.database_wal_lsn = "0/016B6C50".to_string();
        assert!(PublishedTickAbandonmentTombstone::new(input, 1).is_err());
        let mut input = abandonment_input(running, 100);
        input.database_wal_lsn = "0/0".to_string();
        assert!(PublishedTickAbandonmentTombstone::new(input, 1).is_err());

        let valid = PublishedTickAbandonmentTombstone::new(
            abandonment_input(
                record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1),
                100,
            ),
            1,
        )
        .unwrap();
        let mut value = serde_json::to_value(&valid).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("unexpected".to_string(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<PublishedTickAbandonmentTombstone>(value).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cross_kind_latest_order_is_stable_and_wal_or_timeline_regression_fails_closed() {
        let root = temp_dir("cross-kind-lineage-order");
        let owner = owner(&root, "host-a");
        let mut manifest = AckTombstoneManifest::empty(&owner);
        let mut terminal_high_water =
            record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 10);
        terminal_high_water.phase = "complete".to_string();
        let terminal = PublishedTickColdWitness::TerminalAck(
            PublishedTickAckTombstone::new(ack_input(terminal_high_water, 100), 1).unwrap(),
        );
        advance_manifest_for_witness(&mut manifest, &owner, &terminal, &"a".repeat(64)).unwrap();

        let abandonment = PublishedTickColdWitness::FailedClosedAbandonment(
            PublishedTickAbandonmentTombstone::new(
                abandonment_input(
                    record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 11),
                    101,
                ),
                2,
            )
            .unwrap(),
        );
        advance_manifest_for_witness(&mut manifest, &owner, &abandonment, &"b".repeat(64)).unwrap();
        assert_eq!(manifest.latest_witness, Some(abandonment.clone()));
        let mut stale_v2 = manifest.clone();
        stale_v2.latest_witness = Some(terminal.clone());
        stale_v2.latest_witness_sha256 = Some("a".repeat(64));
        assert!(stale_v2
            .validate(&owner)
            .unwrap_err()
            .contains("latest witness is inconsistent"));

        let mut same_lsn_terminal_input = ack_input(
            {
                let mut high_water =
                    record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 12);
                high_water.phase = "complete".to_string();
                high_water
            },
            102,
        );
        same_lsn_terminal_input.database_wal_lsn = "0/16B6C50".to_string();
        let same_lsn_terminal = PublishedTickColdWitness::TerminalAck(
            PublishedTickAckTombstone::new(same_lsn_terminal_input, 3).unwrap(),
        );
        advance_manifest_for_witness(&mut manifest, &owner, &same_lsn_terminal, &"c".repeat(64))
            .unwrap();
        assert_eq!(manifest.committed_seal_sequence, 3);
        assert_eq!(manifest.latest_witness, Some(same_lsn_terminal));
        assert_eq!(manifest.latest_witness_sha256, Some("c".repeat(64)));

        let mut regressed_input = abandonment_input(
            record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 13),
            103,
        );
        regressed_input.database_wal_lsn = "0/16B6C4F".to_string();
        let regressed = PublishedTickColdWitness::FailedClosedAbandonment(
            PublishedTickAbandonmentTombstone::new(regressed_input, 4).unwrap(),
        );
        assert!(
            advance_manifest_for_witness(&mut manifest, &owner, &regressed, &"d".repeat(64),)
                .unwrap_err()
                .contains("WAL LSN cannot regress")
        );

        let mut changed_timeline_input = abandonment_input(
            record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 14),
            104,
        );
        changed_timeline_input.database_timeline_id += 1;
        changed_timeline_input.database_wal_lsn = "0/16B6C51".to_string();
        let changed_timeline = PublishedTickColdWitness::FailedClosedAbandonment(
            PublishedTickAbandonmentTombstone::new(changed_timeline_input, 4).unwrap(),
        );
        assert!(advance_manifest_for_witness(
            &mut manifest,
            &owner,
            &changed_timeline,
            &"e".repeat(64),
        )
        .unwrap_err()
        .contains("database incarnation changed"));
        assert_eq!(manifest.committed_seal_sequence, 3);
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
        persist_test_record(
            &root,
            &records,
            record(&owner, "instance-a", match_id, Uuid::new_v4(), 1, 1),
            4,
            5,
        )
        .unwrap();
        let path = record_path(&root, match_id);
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(load_records(&root, &owner)
            .unwrap_err()
            .contains("must have mode 0600"));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn insecure_ack_tombstone_mode_fails_closed_on_reopen() {
        use std::os::unix::fs::PermissionsExt;

        let root = temp_dir("ack-mode");
        let owner = owner(&root, "host-a");
        load_or_create_ack_manifest(&root, &owner).unwrap();
        let mut terminal = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        terminal.phase = "complete".to_string();
        let tombstone = PublishedTickAckTombstone::new(ack_input(terminal, 100), 1).unwrap();
        let path = ack_tombstone_path(&root, tombstone.high_water.match_id);
        let shard = ensure_ack_tombstone_shard(&root, tombstone.high_water.match_id).unwrap();
        atomic_install(
            &shard,
            &path,
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(
            load_ack_tombstone_if_exists(&root, &owner, tombstone.high_water.match_id)
                .unwrap_err()
                .contains("must have mode 0600")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn insecure_abandonment_tombstone_mode_fails_closed_on_read() {
        use std::os::unix::fs::PermissionsExt;

        let root = temp_dir("abandonment-mode");
        let owner = owner(&root, "host-a");
        let manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let running = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        let tombstone =
            PublishedTickAbandonmentTombstone::new(abandonment_input(running, 100), 1).unwrap();
        let match_id = tombstone.high_water.match_id;
        let path = abandonment_tombstone_path(&root, match_id);
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &path,
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(
            load_abandonment_tombstone_if_exists(&root, &owner, &manifest, match_id)
                .unwrap_err()
                .contains("must have mode 0600")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_abandonment_tombstone_fails_closed_on_read() {
        use std::os::unix::fs::symlink;

        let root = temp_dir("abandonment-symlink");
        let owner = owner(&root, "host-a");
        let manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let running = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        let tombstone =
            PublishedTickAbandonmentTombstone::new(abandonment_input(running, 100), 1).unwrap();
        let match_id = tombstone.high_water.match_id;
        let target = root.join("abandonment-target.json");
        atomic_install(
            &root,
            &target,
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        symlink(&target, abandonment_tombstone_path(&root, match_id)).unwrap();
        sync_directory(&shard).unwrap();
        assert!(
            load_abandonment_tombstone_if_exists(&root, &owner, &manifest, match_id)
                .unwrap_err()
                .contains("is not a regular file")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn hardlinked_abandonment_tombstone_fails_closed_on_read() {
        let root = temp_dir("abandonment-hardlink");
        let owner = owner(&root, "host-a");
        let manifest = load_or_create_ack_manifest(&root, &owner).unwrap();
        let running = record(&owner, "instance-a", Uuid::new_v4(), Uuid::new_v4(), 1, 1);
        let tombstone =
            PublishedTickAbandonmentTombstone::new(abandonment_input(running, 100), 1).unwrap();
        let match_id = tombstone.high_water.match_id;
        let path = abandonment_tombstone_path(&root, match_id);
        let shard = ensure_abandonment_tombstone_shard(&root, match_id).unwrap();
        atomic_install(
            &shard,
            &path,
            &serde_json::to_vec_pretty(&tombstone).unwrap(),
        )
        .unwrap();
        fs::hard_link(&path, root.join("abandonment-extra-link.json")).unwrap();
        assert!(
            load_abandonment_tombstone_if_exists(&root, &owner, &manifest, match_id)
                .unwrap_err()
                .contains("must have exactly one filesystem link")
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn committed_abandonment_directory_loss_blocks_hot_revival() {
        let root = temp_dir("abandonment-directory-loss");
        let journal =
            PublishedTickJournal::open(root.clone(), unique_host("abandonment-directory-loss"))
                .unwrap();
        let match_id = Uuid::new_v4();
        let running = journal
            .new_record(record_input(
                "instance-a",
                match_id,
                Uuid::new_v4(),
                1,
                10,
                "running",
            ))
            .unwrap();
        journal
            .record(running.clone(), 4, 5, input_cursors())
            .await
            .unwrap();
        journal
            .seal_abandonment(abandonment_input(running.clone(), 100))
            .await
            .unwrap();
        fs::remove_dir_all(root.join(ABANDONMENT_TOMBSTONE_DIRECTORY)).unwrap();
        assert!(journal
            .abandonment_tombstone(match_id)
            .unwrap_err()
            .contains("lost its abandonment directory"));
        let error = journal
            .record(running, 4, 5, input_cursors())
            .await
            .unwrap_err();
        assert!(
            error.contains("lost its abandonment directory")
                || error.contains("failed-closed generation"),
            "unexpected fail-closed error: {error}"
        );
        assert!(!journal.is_operational());
        assert!(!record_path(&root, match_id).exists());
        drop(journal);
        fs::remove_dir_all(root).unwrap();
    }
}
