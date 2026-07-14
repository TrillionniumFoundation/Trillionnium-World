use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, VecDeque},
    fmt,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use trnm_online_protocol::{OnlineCommandReceipt, OnlineCommandSubmitRequest};
use trnm_rts_protocol::RtsFrameOrder;

pub(super) const ONLINE_COMMAND_JOURNAL_CONTRACT: &str = "trnm_online_command_journal_v1";
pub(super) const MAX_PENDING_EXACT_ATTEMPTS: usize = 16;
pub(super) const MAX_REJECTED_EXACT_ATTEMPTS: usize = 16;
const MAX_JOURNAL_BYTES: u64 = 256 * 1024;
const MAX_REJECTION_REASON_BYTES: usize = 1_024;

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OnlineCommandJournalScope {
    pub(super) match_id: String,
    pub(super) player_id: String,
    pub(super) account_id: String,
}

impl OnlineCommandJournalScope {
    pub(super) fn new(
        match_id: impl Into<String>,
        player_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Self {
        Self {
            match_id: match_id.into(),
            player_id: player_id.into(),
            account_id: account_id.into(),
        }
    }

    fn validate(&self) -> Result<(), String> {
        if !is_uuid(&self.match_id) {
            return Err("online command journal match_id must be a UUID".to_string());
        }
        if !is_portable_identifier(&self.player_id, 128) {
            return Err(
                "online command journal player_id must be 1-128 portable ASCII identifier characters"
                    .to_string(),
            );
        }
        if !is_uuid(&self.account_id) {
            return Err("online command journal account_id must be a UUID".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PendingExactCommandAttempt {
    pub(super) request: OnlineCommandSubmitRequest,
    pub(super) order: RtsFrameOrder,
    pub(super) label: String,
    pub(super) intent_id: String,
    pub(super) attempt: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RejectedExactCommandAttempt {
    pub(super) pending: PendingExactCommandAttempt,
    pub(super) status: u16,
    pub(super) reason: String,
}

impl RejectedExactCommandAttempt {
    fn validate(&self, scope: &OnlineCommandJournalScope) -> Result<(), String> {
        self.pending.validate(scope)?;
        if !(400..500).contains(&self.status) {
            return Err("rejected command status must be an HTTP 4xx status".to_string());
        }
        if self.reason.is_empty()
            || self.reason.len() > MAX_REJECTION_REASON_BYTES
            || self.reason.chars().any(char::is_control)
        {
            return Err(format!(
                "rejected command reason must be 1-{MAX_REJECTION_REASON_BYTES} non-control bytes"
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum JournalStoreError {
    BeforeInstall(String),
    DurabilityUncertain(String),
}

impl JournalStoreError {
    pub(super) fn durability_uncertain(&self) -> bool {
        matches!(self, Self::DurabilityUncertain(_))
    }
}

impl fmt::Display for JournalStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BeforeInstall(message) => formatter.write_str(message),
            Self::DurabilityUncertain(message) => {
                write!(
                    formatter,
                    "online command journal durability is uncertain: {message}"
                )
            }
        }
    }
}

impl PendingExactCommandAttempt {
    fn validate(&self, scope: &OnlineCommandJournalScope) -> Result<(), String> {
        if self.request.player_id != scope.player_id || self.request.account_id != scope.account_id
        {
            return Err("pending command identity does not match journal scope".to_string());
        }
        if self.request.protocol_version.trim().is_empty()
            || self.request.build_id.trim().is_empty()
        {
            return Err("pending command protocol/build identity is empty".to_string());
        }
        if !is_portable_identifier(&self.request.command_id, 160) {
            return Err(
                "pending command_id must be 1-160 portable ASCII identifier characters".to_string(),
            );
        }
        if !is_portable_identifier(&self.intent_id, 160) {
            return Err(
                "pending intent_id must be 1-160 portable ASCII identifier characters".to_string(),
            );
        }
        if self.label.is_empty()
            || self.label.len() > 256
            || self.label.chars().any(char::is_control)
        {
            return Err("pending command label must be 1-256 non-control characters".to_string());
        }
        if self.request.order != self.order {
            return Err("pending exact request/order diverged".to_string());
        }
        self.order
            .validate()
            .map_err(|error| format!("pending order is invalid: {error}"))?;
        if self.request.target_tick != u64::from(self.order.frame) {
            return Err("pending target_tick does not equal order.frame".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OnlineCommandJournal {
    #[serde(skip)]
    path: PathBuf,
    #[serde(skip)]
    lock_file: Option<Arc<File>>,
    contract_version: String,
    pub(super) scope: OnlineCommandJournalScope,
    pub(super) next_input_sequence: u64,
    pub(super) next_receipt_sequence: u64,
    pub(super) last_snapshot_hash: String,
    pub(super) pending_exact_attempts: VecDeque<PendingExactCommandAttempt>,
    #[serde(default)]
    pub(super) rejected_exact_attempts: VecDeque<RejectedExactCommandAttempt>,
}

impl OnlineCommandJournal {
    pub(super) fn load_or_new(
        path: impl Into<PathBuf>,
        scope: OnlineCommandJournalScope,
    ) -> Result<Self, String> {
        scope.validate()?;
        let path = path.into();
        if path.as_os_str().is_empty() {
            return Err("online command journal path is empty".to_string());
        }
        ensure_secure_journal_directory(journal_parent(&path))?;
        let lock_file = acquire_journal_lock(&path)?;
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::new(path, scope, lock_file));
            }
            Err(error) => {
                return Err(format!(
                    "inspect online command journal {}: {error}",
                    path.display()
                ));
            }
        };
        if !metadata.file_type().is_file() {
            return Err(format!(
                "online command journal {} is not a regular file",
                path.display()
            ));
        }
        validate_private_regular_file(&path, &metadata, "online command journal")?;
        if metadata.len() > MAX_JOURNAL_BYTES {
            return Err(format!(
                "online command journal {} exceeds {MAX_JOURNAL_BYTES} bytes",
                path.display()
            ));
        }
        let mut file = File::open(&path)
            .map_err(|error| format!("open online command journal {}: {error}", path.display()))?;
        validate_open_file_identity(&path, &file, "online command journal")?;
        let mut bytes = Vec::with_capacity(usize::try_from(metadata.len()).unwrap_or_default());
        std::io::Read::by_ref(&mut file)
            .take(MAX_JOURNAL_BYTES.saturating_add(1))
            .read_to_end(&mut bytes)
            .map_err(|error| format!("read online command journal {}: {error}", path.display()))?;
        if bytes.len() as u64 > MAX_JOURNAL_BYTES {
            return Err(format!(
                "online command journal {} grew beyond {MAX_JOURNAL_BYTES} bytes while reading",
                path.display()
            ));
        }
        let mut journal = match serde_json::from_slice::<Self>(&bytes) {
            Ok(journal) => journal,
            Err(error) => return Err(quarantine_error(&path, format!("decode failed: {error}"))),
        };
        journal.path = path.clone();
        journal.lock_file = Some(lock_file);
        if let Err(error) = journal.validate() {
            return Err(quarantine_error(&path, error));
        }
        if journal.scope != scope {
            return Err(
                "online command journal scope mismatch: stored match/player/account does not match the active session"
                    .to_string(),
            );
        }
        Ok(journal)
    }

    fn new(path: PathBuf, scope: OnlineCommandJournalScope, lock_file: Arc<File>) -> Self {
        Self {
            path,
            lock_file: Some(lock_file),
            contract_version: ONLINE_COMMAND_JOURNAL_CONTRACT.to_string(),
            scope,
            next_input_sequence: 0,
            next_receipt_sequence: 0,
            last_snapshot_hash: String::new(),
            pending_exact_attempts: VecDeque::new(),
            rejected_exact_attempts: VecDeque::new(),
        }
    }

    pub(super) fn store(&self) -> Result<(), JournalStoreError> {
        self.validate().map_err(JournalStoreError::BeforeInstall)?;
        let parent = journal_parent(&self.path);
        ensure_secure_journal_directory(parent).map_err(JournalStoreError::BeforeInstall)?;
        let payload = serde_json::to_vec_pretty(self).map_err(|error| {
            JournalStoreError::BeforeInstall(format!("encode online command journal: {error}"))
        })?;
        if payload.len() as u64 > MAX_JOURNAL_BYTES {
            return Err(JournalStoreError::BeforeInstall(format!(
                "online command journal exceeds {MAX_JOURNAL_BYTES} bytes"
            )));
        }
        let temp_path = unique_sibling_path(&self.path, "tmp");
        let before_install = (|| {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temp_path).map_err(|error| {
                format!(
                    "create online command journal temporary file {}: {error}",
                    temp_path.display()
                )
            })?;
            file.write_all(&payload).map_err(|error| {
                format!(
                    "write online command journal temporary file {}: {error}",
                    temp_path.display()
                )
            })?;
            file.sync_all().map_err(|error| {
                format!(
                    "sync online command journal temporary file {}: {error}",
                    temp_path.display()
                )
            })?;
            fs::rename(&temp_path, &self.path).map_err(|error| {
                format!(
                    "atomically install online command journal {}: {error}",
                    self.path.display()
                )
            })?;
            Ok(())
        })();
        if let Err(error) = before_install {
            let _ = fs::remove_file(&temp_path);
            return Err(JournalStoreError::BeforeInstall(error));
        }
        File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| {
                JournalStoreError::DurabilityUncertain(format!(
                    "installed {} but syncing directory {} failed: {error}",
                    self.path.display(),
                    parent.display()
                ))
            })
    }

    pub(super) fn advance_input_sequence(&mut self, next: u64) -> Result<(), String> {
        if next < self.next_input_sequence {
            return Err("online input sequence cannot regress".to_string());
        }
        self.next_input_sequence = next;
        Ok(())
    }

    pub(super) fn update_receipt_cursor(
        &mut self,
        next_receipt_sequence: u64,
        last_snapshot_hash: impl Into<String>,
    ) -> Result<(), String> {
        if next_receipt_sequence < self.next_receipt_sequence {
            return Err("online receipt sequence cannot regress".to_string());
        }
        let last_snapshot_hash = last_snapshot_hash.into();
        validate_snapshot_hash(&last_snapshot_hash)?;
        self.next_receipt_sequence = next_receipt_sequence;
        self.last_snapshot_hash = last_snapshot_hash;
        Ok(())
    }

    pub(super) fn enqueue_exact_attempt(
        &mut self,
        pending: PendingExactCommandAttempt,
    ) -> Result<(), String> {
        if self.pending_exact_attempts.len() >= MAX_PENDING_EXACT_ATTEMPTS {
            return Err(format!(
                "online command journal queue is full ({MAX_PENDING_EXACT_ATTEMPTS})"
            ));
        }
        pending.validate(&self.scope)?;
        if self
            .pending_exact_attempts
            .iter()
            .any(|current| current.intent_id == pending.intent_id)
        {
            return Err("pending intent_id is already present".to_string());
        }
        if self
            .pending_exact_attempts
            .iter()
            .any(|current| current.request.command_id == pending.request.command_id)
        {
            return Err("pending command_id is already present".to_string());
        }
        if let Some(input_sequence) = pending.request.input_sequence {
            self.next_input_sequence = self.next_input_sequence.max(
                input_sequence
                    .checked_add(1)
                    .ok_or_else(|| "online input sequence exhausted".to_string())?,
            );
        }
        self.pending_exact_attempts.push_back(pending);
        Ok(())
    }

    pub(super) fn replace_exact_attempt(
        &mut self,
        intent_id: &str,
        replacement: PendingExactCommandAttempt,
    ) -> Result<PendingExactCommandAttempt, String> {
        replacement.validate(&self.scope)?;
        if replacement.intent_id != intent_id {
            return Err("replacement changed the durable intent_id".to_string());
        }
        let index = self
            .pending_exact_attempts
            .iter()
            .position(|pending| pending.intent_id == intent_id)
            .ok_or_else(|| "pending intent_id was not found".to_string())?;
        let current = &self.pending_exact_attempts[index];
        let expected_attempt = current
            .attempt
            .checked_add(1)
            .ok_or_else(|| "online command attempt sequence exhausted".to_string())?;
        if replacement.attempt != expected_attempt {
            return Err("replacement attempt must advance by exactly one".to_string());
        }
        if self
            .pending_exact_attempts
            .iter()
            .enumerate()
            .any(|(candidate_index, pending)| {
                candidate_index != index
                    && pending.request.command_id == replacement.request.command_id
            })
        {
            return Err("replacement command_id is already present".to_string());
        }
        if let Some(input_sequence) = replacement.request.input_sequence {
            self.next_input_sequence = self.next_input_sequence.max(
                input_sequence
                    .checked_add(1)
                    .ok_or_else(|| "online input sequence exhausted".to_string())?,
            );
        }
        let previous = std::mem::replace(&mut self.pending_exact_attempts[index], replacement);
        Ok(previous)
    }

    pub(super) fn acknowledge(
        &mut self,
        receipt: &OnlineCommandReceipt,
    ) -> Result<PendingExactCommandAttempt, String> {
        if receipt.match_id != self.scope.match_id {
            return Err("command receipt match_id does not match journal scope".to_string());
        }
        if receipt.player_id != self.scope.player_id {
            return Err("command receipt player_id does not match journal scope".to_string());
        }
        validate_snapshot_hash(&receipt.snapshot_hash)?;
        let index = self
            .pending_exact_attempts
            .iter()
            .position(|pending| pending.request.command_id == receipt.command_id)
            .ok_or_else(|| "command receipt does not match a pending exact attempt".to_string())?;
        let request = &self.pending_exact_attempts[index].request;
        if receipt.protocol_version != request.protocol_version {
            return Err("command receipt protocol does not match the exact request".to_string());
        }
        if receipt.client_observed_tick != request.client_observed_tick {
            return Err(
                "command receipt client_observed_tick does not match the exact request".to_string(),
            );
        }
        if receipt
            .client_observed_tick
            .is_some_and(|observed| receipt.accepted_tick < observed)
        {
            return Err("command receipt accepted_tick predates the observed tick".to_string());
        }
        if receipt.match_revision < request.expected_match_revision {
            return Err("command receipt match_revision regressed the exact request".to_string());
        }
        let acknowledged_input_sequence = if let Some(input_sequence) = request.input_sequence {
            if input_sequence != receipt.input_sequence {
                return Err(
                    "command receipt input_sequence does not match the exact request".to_string(),
                );
            }
            input_sequence
        } else {
            if request.sequence != receipt.sequence {
                return Err("command receipt sequence does not match the exact request".to_string());
            }
            receipt.sequence
        };
        let next_receipt_sequence = receipt
            .sequence
            .checked_add(1)
            .ok_or_else(|| "online receipt sequence exhausted".to_string())?;
        let next_input_sequence = acknowledged_input_sequence
            .checked_add(1)
            .ok_or_else(|| "online input sequence exhausted".to_string())?;
        let pending = self
            .pending_exact_attempts
            .remove(index)
            .expect("pending index was resolved above");
        if next_receipt_sequence >= self.next_receipt_sequence {
            self.next_receipt_sequence = next_receipt_sequence;
            self.last_snapshot_hash = receipt.snapshot_hash.clone();
        }
        self.next_input_sequence = self.next_input_sequence.max(next_input_sequence);
        Ok(pending)
    }

    pub(super) fn reject(
        &mut self,
        expected: &PendingExactCommandAttempt,
        status: u16,
        reason: impl Into<String>,
    ) -> Result<(), String> {
        let pending = self
            .pending_exact_attempts
            .front()
            .ok_or_else(|| "rejected command has no pending exact attempt".to_string())?;
        if pending != expected {
            return Err("rejected command matched a different exact attempt".to_string());
        }
        let reason = sanitize_rejection_reason(&reason.into());
        let rejected = RejectedExactCommandAttempt {
            pending: expected.clone(),
            status,
            reason,
        };
        rejected.validate(&self.scope)?;
        self.pending_exact_attempts
            .pop_front()
            .expect("pending rejection was resolved above");
        if self.rejected_exact_attempts.len() == MAX_REJECTED_EXACT_ATTEMPTS {
            self.rejected_exact_attempts.pop_front();
        }
        self.rejected_exact_attempts.push_back(rejected);
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        if self.path.as_os_str().is_empty() {
            return Err("online command journal path is empty".to_string());
        }
        if self.lock_file.is_none() {
            return Err("online command journal does not hold its process lock".to_string());
        }
        if self.contract_version != ONLINE_COMMAND_JOURNAL_CONTRACT {
            return Err(format!(
                "unsupported online command journal contract {}",
                self.contract_version
            ));
        }
        self.scope.validate()?;
        validate_snapshot_hash(&self.last_snapshot_hash)?;
        if self.pending_exact_attempts.len() > MAX_PENDING_EXACT_ATTEMPTS {
            return Err(format!(
                "online command journal exceeds {MAX_PENDING_EXACT_ATTEMPTS} pending attempts"
            ));
        }
        if self.rejected_exact_attempts.len() > MAX_REJECTED_EXACT_ATTEMPTS {
            return Err(format!(
                "online command journal exceeds {MAX_REJECTED_EXACT_ATTEMPTS} rejected attempts"
            ));
        }
        let mut intent_ids = BTreeSet::new();
        let mut command_ids = BTreeSet::new();
        for pending in &self.pending_exact_attempts {
            pending.validate(&self.scope)?;
            if pending
                .request
                .input_sequence
                .is_some_and(|input_sequence| input_sequence >= self.next_input_sequence)
            {
                return Err(
                    "pending input_sequence is not covered by next_input_sequence".to_string(),
                );
            }
            if !intent_ids.insert(pending.intent_id.as_str()) {
                return Err("online command journal contains duplicate intent_id".to_string());
            }
            if !command_ids.insert(pending.request.command_id.as_str()) {
                return Err("online command journal contains duplicate command_id".to_string());
            }
        }
        for rejected in &self.rejected_exact_attempts {
            rejected.validate(&self.scope)?;
        }
        Ok(())
    }
}

fn validate_snapshot_hash(value: &str) -> Result<(), String> {
    if value.is_empty()
        || (value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()))
    {
        Ok(())
    } else {
        Err("last_snapshot_hash must be empty or 64 lowercase hexadecimal characters".to_string())
    }
}

fn is_portable_identifier(value: &str, maximum_length: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
}

fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit(),
        })
}

fn acquire_journal_lock(path: &Path) -> Result<Arc<File>, String> {
    let parent = journal_parent(path);
    ensure_secure_journal_directory(parent)?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("online-command-journal");
    let lock_path = parent.join(format!(".{name}.lock"));
    match fs::symlink_metadata(&lock_path) {
        Ok(metadata) => {
            validate_private_regular_file(&lock_path, &metadata, "online command journal lock")?
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(format!(
                "inspect online command journal lock {}: {error}",
                lock_path.display()
            ));
        }
    }
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&lock_path).map_err(|error| {
        format!(
            "open online command journal lock {}: {error}",
            lock_path.display()
        )
    })?;
    validate_open_file_identity(&lock_path, &file, "online command journal lock")?;
    file.try_lock().map_err(|error| {
        format!(
            "online command journal {} is already owned by another client process: {error}",
            path.display()
        )
    })?;
    Ok(Arc::new(file))
}

fn ensure_secure_journal_directory(path: &Path) -> Result<(), String> {
    let existed = match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
                return Err(format!(
                    "online command journal directory {} is not a real directory",
                    path.display()
                ));
            }
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(format!(
                "inspect online command journal directory {}: {error}",
                path.display()
            ));
        }
    };
    if !existed {
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(path).map_err(|error| {
            format!(
                "create online command journal directory {}: {error}",
                path.display()
            )
        })?;
    }
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        format!(
            "inspect online command journal directory {} after creation: {error}",
            path.display()
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return Err(format!(
            "online command journal directory {} is not a real directory",
            path.display()
        ));
    }
    validate_owner(path, &metadata, "online command journal directory")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let mode = metadata.permissions().mode() & 0o777;
        if mode != 0o700 {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|error| {
                format!(
                    "set online command journal directory {} mode 0700: {error}",
                    path.display()
                )
            })?;
            let repaired = fs::symlink_metadata(path).map_err(|error| {
                format!(
                    "reinspect online command journal directory {} after permission repair: {error}",
                    path.display()
                )
            })?;
            if repaired.file_type().is_symlink()
                || !repaired.file_type().is_dir()
                || repaired.uid() != effective_user_id()
                || repaired.permissions().mode() & 0o777 != 0o700
            {
                return Err(format!(
                    "online command journal directory {} did not converge to owner mode 0700",
                    path.display()
                ));
            }
            File::open(path)
                .and_then(|directory| directory.sync_all())
                .map_err(|error| {
                    format!(
                        "sync online command journal directory {} after permission repair: {error}",
                        path.display()
                    )
                })?;
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
    validate_private_owner_and_mode(path, metadata, 0o600, label)
}

#[cfg(unix)]
fn validate_private_owner_and_mode(
    path: &Path,
    metadata: &fs::Metadata,
    expected_mode: u32,
    label: &str,
) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    validate_owner(path, metadata, label)?;
    let mode = metadata.permissions().mode() & 0o777;
    if mode != expected_mode {
        return Err(format!(
            "{label} {} must have mode {expected_mode:04o}, found {mode:04o}",
            path.display()
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn validate_owner(path: &Path, metadata: &fs::Metadata, label: &str) -> Result<(), String> {
    use std::os::unix::fs::MetadataExt;

    if metadata.uid() != effective_user_id() {
        return Err(format!(
            "{label} {} is not owned by this user",
            path.display()
        ));
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_owner(_path: &Path, _metadata: &fs::Metadata, _label: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(not(unix))]
fn validate_private_owner_and_mode(
    _path: &Path,
    _metadata: &fs::Metadata,
    _expected_mode: u32,
    _label: &str,
) -> Result<(), String> {
    Ok(())
}

#[cfg(unix)]
fn effective_user_id() -> u32 {
    unsafe extern "C" {
        fn geteuid() -> u32;
    }
    // SAFETY: geteuid has no preconditions and does not access caller memory.
    unsafe { geteuid() }
}

fn validate_open_file_identity(path: &Path, file: &File, label: &str) -> Result<(), String> {
    let open_metadata = file
        .metadata()
        .map_err(|error| format!("inspect open {label} {}: {error}", path.display()))?;
    validate_private_regular_file(path, &open_metadata, label)?;
    let path_metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("reinspect {label} {}: {error}", path.display()))?;
    validate_private_regular_file(path, &path_metadata, label)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if open_metadata.dev() != path_metadata.dev() || open_metadata.ino() != path_metadata.ino()
        {
            return Err(format!("{label} {} changed while opening", path.display()));
        }
    }
    Ok(())
}

fn sanitize_rejection_reason(value: &str) -> String {
    let mut sanitized = String::new();
    for character in value.chars() {
        let character = if character.is_control() {
            ' '
        } else {
            character
        };
        if sanitized.len().saturating_add(character.len_utf8()) > MAX_REJECTION_REASON_BYTES {
            break;
        }
        sanitized.push(character);
    }
    let sanitized = sanitized.trim();
    if sanitized.is_empty() {
        "HTTP request was rejected".to_string()
    } else {
        sanitized.to_string()
    }
}

fn journal_parent(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn unique_sibling_path(path: &Path, kind: &str) -> PathBuf {
    let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("online-command-journal");
    journal_parent(path).join(format!(
        ".{name}.{kind}-{}-{timestamp}-{sequence}",
        process::id()
    ))
}

fn quarantine_error(path: &Path, reason: String) -> String {
    let quarantine_path = unique_sibling_path(path, "corrupt");
    match fs::rename(path, &quarantine_path) {
        Ok(()) => {
            let parent = journal_parent(path);
            let sync_result = File::open(parent).and_then(|directory| directory.sync_all());
            match sync_result {
                Ok(()) => format!(
                    "online command journal {} is invalid ({reason}); quarantined at {}",
                    path.display(),
                    quarantine_path.display()
                ),
                Err(error) => format!(
                    "online command journal {} is invalid ({reason}); quarantined at {} but directory sync failed: {error}",
                    path.display(),
                    quarantine_path.display()
                ),
            }
        }
        Err(error) => format!(
            "online command journal {} is invalid ({reason}) and quarantine failed: {error}",
            path.display()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use trnm_online_protocol::{ONLINE_AUTHORITY_BUILD, ONLINE_AUTHORITY_PROTOCOL};
    use trnm_rts_protocol::{RtsOrderKind, RtsOrderSource, RtsTile};

    static TEST_PATH_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn test_path(label: &str) -> PathBuf {
        let sequence = TEST_PATH_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir()
            .join(format!(
                "trnm-online-command-journal-{}-{label}-{sequence}",
                process::id()
            ))
            .join("journal.json")
    }

    fn scope() -> OnlineCommandJournalScope {
        OnlineCommandJournalScope::new(
            "00000000-0000-0000-0000-000000000001",
            "player-one",
            "00000000-0000-0000-0000-000000000002",
        )
    }

    fn pending(
        command_id: &str,
        intent_id: &str,
        attempt: u32,
        sequence: u64,
    ) -> PendingExactCommandAttempt {
        let mut order = RtsFrameOrder::new(
            42,
            "player-one",
            vec!["host:hero".to_string()],
            RtsOrderKind::Move,
            RtsOrderSource::LocalInput,
        );
        order.target_tile = Some(RtsTile { x: 3, y: 4 });
        let request = OnlineCommandSubmitRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: "player-one".to_string(),
            account_id: "00000000-0000-0000-0000-000000000002".to_string(),
            command_id: command_id.to_string(),
            sequence,
            input_sequence: Some(sequence),
            expected_match_revision: sequence,
            target_tick: 42,
            client_observed_tick: Some(42),
            order: order.clone(),
        };
        PendingExactCommandAttempt {
            request,
            order,
            label: "Move".to_string(),
            intent_id: intent_id.to_string(),
            attempt,
        }
    }

    fn cleanup(path: &Path) {
        if let Some(root) = path.parent() {
            let _ = fs::remove_dir_all(root);
        }
    }

    #[test]
    fn journal_round_trips_without_credentials() {
        let path = test_path("roundtrip");
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.update_receipt_cursor(0, "a".repeat(64)).unwrap();
        journal
            .enqueue_exact_attempt(pending("native:intent-1:a0", "intent-1", 0, 0))
            .unwrap();
        journal.store().unwrap();

        let bytes = fs::read(&path).unwrap();
        let serialized = String::from_utf8(bytes).unwrap();
        assert!(!serialized.contains("player_session"));
        assert!(!serialized.contains("session_token"));
        drop(journal);
        let loaded = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        assert_eq!(loaded.next_input_sequence, 1);
        assert_eq!(loaded.next_receipt_sequence, 0);
        assert_eq!(loaded.last_snapshot_hash, "a".repeat(64));
        assert_eq!(loaded.pending_exact_attempts.len(), 1);
        assert_eq!(
            loaded.pending_exact_attempts[0],
            pending("native:intent-1:a0", "intent-1", 0, 0)
        );
        cleanup(&path);
    }

    #[test]
    fn truncated_journal_is_quarantined_and_fails_closed() {
        let path = test_path("truncated");
        let journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.store().unwrap();
        drop(journal);
        fs::write(&path, b"{\"contract_version\":").unwrap();

        let error = OnlineCommandJournal::load_or_new(&path, scope()).unwrap_err();
        assert!(error.contains("quarantined"));
        assert!(!path.exists());
        let parent = path.parent().unwrap();
        assert!(fs::read_dir(parent).unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains(".corrupt-")
        }));
        cleanup(&path);
    }

    #[test]
    fn scope_mismatch_is_rejected_without_quarantining_valid_state() {
        let path = test_path("scope-mismatch");
        let journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.store().unwrap();
        drop(journal);
        let other_scope = OnlineCommandJournalScope::new(
            "00000000-0000-0000-0000-000000000003",
            "player-one",
            "00000000-0000-0000-0000-000000000002",
        );

        let error = OnlineCommandJournal::load_or_new(&path, other_scope).unwrap_err();
        assert!(error.contains("scope mismatch"));
        assert!(path.exists());
        cleanup(&path);
    }

    #[test]
    fn exact_pending_attempt_survives_and_acknowledges_by_command_id() {
        let path = test_path("exact-pending");
        let exact = pending("native:intent-7:a0", "intent-7", 0, 7);
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.enqueue_exact_attempt(exact.clone()).unwrap();
        journal.store().unwrap();
        drop(journal);

        let mut loaded = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        assert_eq!(loaded.pending_exact_attempts.front(), Some(&exact));
        let receipt = OnlineCommandReceipt {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            match_id: scope().match_id,
            player_id: scope().player_id,
            command_id: exact.request.command_id.clone(),
            sequence: 12,
            input_sequence: exact.request.input_sequence.unwrap(),
            duplicate: true,
            accepted_tick: exact.request.target_tick,
            client_observed_tick: exact.request.client_observed_tick,
            match_revision: 8,
            snapshot_hash: "b".repeat(64),
        };
        assert_eq!(loaded.acknowledge(&receipt).unwrap(), exact);
        assert!(loaded.pending_exact_attempts.is_empty());
        assert_eq!(loaded.next_receipt_sequence, 13);
        assert_eq!(loaded.next_input_sequence, 8);
        assert_eq!(loaded.last_snapshot_hash, "b".repeat(64));
        cleanup(&path);
    }

    #[test]
    fn replacement_requires_a_new_exact_attempt() {
        let path = test_path("replacement");
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal
            .enqueue_exact_attempt(pending("native:intent-9:a0", "intent-9", 0, 9))
            .unwrap();
        let replacement = pending("native:intent-9:a1", "intent-9", 1, 10);
        let previous = journal
            .replace_exact_attempt("intent-9", replacement.clone())
            .unwrap();
        assert_eq!(previous.attempt, 0);
        assert_eq!(journal.pending_exact_attempts.front(), Some(&replacement));
        cleanup(&path);
    }

    #[test]
    fn journal_is_exclusive_for_the_client_lifetime() {
        let path = test_path("exclusive-lock");
        let journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.store().unwrap();
        let error = OnlineCommandJournal::load_or_new(&path, scope()).unwrap_err();
        assert!(error.contains("already owned"), "{error}");
        drop(journal);
        let reopened = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        drop(reopened);
        cleanup(&path);
    }

    #[test]
    fn journal_enforces_the_sixteen_intent_backpressure_boundary() {
        let path = test_path("queue-boundary");
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        for sequence in 0..MAX_PENDING_EXACT_ATTEMPTS as u64 {
            journal
                .enqueue_exact_attempt(pending(
                    &format!("native:intent-{sequence}:a0"),
                    &format!("intent-{sequence}"),
                    0,
                    sequence,
                ))
                .unwrap();
        }
        assert_eq!(
            journal.pending_exact_attempts.len(),
            MAX_PENDING_EXACT_ATTEMPTS
        );
        let error = journal
            .enqueue_exact_attempt(pending(
                "native:intent-overflow:a0",
                "intent-overflow",
                0,
                MAX_PENDING_EXACT_ATTEMPTS as u64,
            ))
            .unwrap_err();
        assert!(error.contains("queue is full"), "{error}");
        drop(journal);
        cleanup(&path);
    }

    #[test]
    fn permanent_rejection_is_sanitized_persisted_and_does_not_poison_fifo() {
        let path = test_path("rejected-fifo");
        let first = pending("native:intent-0:a0", "intent-0", 0, 0);
        let second = pending("native:intent-1:a0", "intent-1", 0, 1);
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.enqueue_exact_attempt(first.clone()).unwrap();
        journal.enqueue_exact_attempt(second.clone()).unwrap();

        let invalid = journal.reject(&first, 503, "must not mutate").unwrap_err();
        assert!(invalid.contains("4xx"), "{invalid}");
        assert_eq!(journal.pending_exact_attempts.front(), Some(&first));
        assert!(journal.rejected_exact_attempts.is_empty());

        journal
            .reject(
                &first,
                422,
                format!("bad\nrequest\t{}", "界".repeat(MAX_REJECTION_REASON_BYTES)),
            )
            .unwrap();
        journal.store().unwrap();
        drop(journal);

        let loaded = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        assert_eq!(loaded.pending_exact_attempts.front(), Some(&second));
        assert_eq!(loaded.rejected_exact_attempts.len(), 1);
        let rejected = &loaded.rejected_exact_attempts[0];
        assert_eq!(rejected.pending, first);
        assert_eq!(rejected.status, 422);
        assert!(!rejected.reason.chars().any(char::is_control));
        assert!(rejected.reason.len() <= MAX_REJECTION_REASON_BYTES);
        drop(loaded);
        cleanup(&path);
    }

    #[test]
    fn rejected_command_history_is_bounded() {
        let path = test_path("rejected-boundary");
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        for sequence in 0..=MAX_REJECTED_EXACT_ATTEMPTS as u64 {
            let exact = pending(
                &format!("native:intent-{sequence}:a0"),
                &format!("intent-{sequence}"),
                0,
                sequence,
            );
            journal.enqueue_exact_attempt(exact.clone()).unwrap();
            journal.reject(&exact, 409, "conflict").unwrap();
        }
        assert_eq!(
            journal.rejected_exact_attempts.len(),
            MAX_REJECTED_EXACT_ATTEMPTS
        );
        assert_eq!(
            journal
                .rejected_exact_attempts
                .front()
                .unwrap()
                .pending
                .intent_id,
            "intent-1"
        );
        assert_eq!(
            journal
                .rejected_exact_attempts
                .back()
                .unwrap()
                .pending
                .intent_id,
            format!("intent-{MAX_REJECTED_EXACT_ATTEMPTS}")
        );
        drop(journal);
        cleanup(&path);
    }

    #[cfg(unix)]
    #[test]
    fn owner_owned_journal_directory_mode_is_repaired_to_0700() {
        use std::os::unix::fs::PermissionsExt;

        let path = test_path("directory-mode-repair");
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        fs::set_permissions(parent, fs::Permissions::from_mode(0o775)).unwrap();

        let journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        let mode = fs::metadata(parent).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o700);
        drop(journal);
        cleanup(&path);
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_journal_directory_is_rejected() {
        use std::os::unix::fs::symlink;

        let path = test_path("directory-symlink");
        let link = path.parent().unwrap().to_path_buf();
        let target = link.with_extension("target");
        fs::create_dir_all(&target).unwrap();
        symlink(&target, &link).unwrap();

        let error = OnlineCommandJournal::load_or_new(&path, scope()).unwrap_err();
        assert!(error.contains("not a real directory"), "{error}");
        fs::remove_file(&link).unwrap();
        fs::remove_dir_all(&target).unwrap();
    }

    #[test]
    fn legacy_v1_journal_without_rejected_history_still_loads() {
        let path = test_path("legacy-rejected-default");
        let journal = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        journal.store().unwrap();
        drop(journal);

        let mut value =
            serde_json::from_slice::<serde_json::Value>(&fs::read(&path).unwrap()).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .remove("rejected_exact_attempts");
        fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();

        let loaded = OnlineCommandJournal::load_or_new(&path, scope()).unwrap();
        assert!(loaded.rejected_exact_attempts.is_empty());
        drop(loaded);
        cleanup(&path);
    }
}
