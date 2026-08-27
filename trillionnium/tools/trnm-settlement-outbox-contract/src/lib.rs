use std::error::Error;
use std::fmt::{Display, Formatter};

pub const SETTLEMENT_OUTBOX_CONTRACT: &str = "trnm_settlement_outbox_v1";
pub const MAX_SETTLEMENT_ATTEMPTS: u32 = 16;
pub const MAX_LEASE_DURATION_MS: u64 = 5 * 60 * 1_000;
pub const MAX_ID_BYTES: usize = 256;
pub const MAX_ERROR_DETAIL_CHARS: usize = 1_024;
const JOB_ID_PREFIX: &str = "trnm-settlement-outbox-v1:";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementJobKeyV1 {
    pub match_id: String,
    pub campaign_id: String,
    pub intent_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementLeaseV1 {
    pub job_id: String,
    pub owner: String,
    pub generation: u64,
    pub expires_at_ms: u64,
    pub attempt: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementReceiptBindingV1 {
    pub job_id: String,
    pub intent_id: String,
    pub intent_hash: String,
    pub receipt_id: String,
    pub receipt_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettlementJobStateV1 {
    Pending,
    Leased {
        owner: String,
        generation: u64,
        expires_at_ms: u64,
    },
    Retryable {
        next_attempt_at_ms: u64,
        last_error: String,
    },
    Succeeded {
        receipt_id: String,
        receipt_hash: String,
        completed_at_ms: u64,
    },
    DeadLetter {
        reason: String,
        failed_at_ms: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementJobV1 {
    pub contract_version: String,
    pub job_id: String,
    pub key: SettlementJobKeyV1,
    pub expected_campaign_revision: u64,
    pub intent_hash: String,
    pub attempts: u32,
    pub lease_generation: u64,
    pub state: SettlementJobStateV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettlementContractError {
    EmptyField(&'static str),
    FieldTooLong(&'static str),
    SurroundingWhitespace(&'static str),
    InvalidHash(&'static str),
    InvalidJobId,
    InvalidState(&'static str),
    InvalidLeaseDuration,
    LeaseNotAvailable,
    LeaseOwnerMismatch,
    LeaseGenerationMismatch,
    LeaseExpired,
    RetryTimeMustAdvance,
    AttemptsExhausted,
    ReceiptMismatch(&'static str),
    ArithmeticOverflow,
}

impl Display for SettlementContractError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "{field} must not be empty"),
            Self::FieldTooLong(field) => write!(formatter, "{field} exceeds its size limit"),
            Self::SurroundingWhitespace(field) => {
                write!(formatter, "{field} must not contain surrounding whitespace")
            }
            Self::InvalidHash(field) => {
                write!(formatter, "{field} must be exactly 64 lowercase hexadecimal characters")
            }
            Self::InvalidJobId => write!(formatter, "job_id does not match its deterministic key"),
            Self::InvalidState(message) => write!(formatter, "invalid settlement state: {message}"),
            Self::InvalidLeaseDuration => write!(
                formatter,
                "lease duration must be between 1 and {MAX_LEASE_DURATION_MS} milliseconds"
            ),
            Self::LeaseNotAvailable => write!(formatter, "settlement job is not currently leasable"),
            Self::LeaseOwnerMismatch => write!(formatter, "settlement lease owner mismatch"),
            Self::LeaseGenerationMismatch => {
                write!(formatter, "settlement lease generation mismatch")
            }
            Self::LeaseExpired => write!(formatter, "settlement lease has expired"),
            Self::RetryTimeMustAdvance => {
                write!(formatter, "retry time must be later than the failure time")
            }
            Self::AttemptsExhausted => write!(formatter, "settlement attempts are exhausted"),
            Self::ReceiptMismatch(field) => write!(formatter, "settlement receipt {field} mismatch"),
            Self::ArithmeticOverflow => write!(formatter, "settlement counter or timestamp overflow"),
        }
    }
}

impl Error for SettlementContractError {}

impl SettlementJobKeyV1 {
    pub fn validate(&self) -> Result<(), SettlementContractError> {
        validate_identifier("match_id", &self.match_id)?;
        validate_identifier("campaign_id", &self.campaign_id)?;
        validate_identifier("intent_id", &self.intent_id)?;
        Ok(())
    }
}

impl SettlementJobV1 {
    pub fn new(
        key: SettlementJobKeyV1,
        expected_campaign_revision: u64,
        intent_hash: String,
    ) -> Result<Self, SettlementContractError> {
        key.validate()?;
        validate_hash("intent_hash", &intent_hash)?;
        let job_id = deterministic_job_id(&key)?;
        Ok(Self {
            contract_version: SETTLEMENT_OUTBOX_CONTRACT.to_string(),
            job_id,
            key,
            expected_campaign_revision,
            intent_hash,
            attempts: 0,
            lease_generation: 0,
            state: SettlementJobStateV1::Pending,
        })
    }

    pub fn validate(&self) -> Result<(), SettlementContractError> {
        if self.contract_version != SETTLEMENT_OUTBOX_CONTRACT {
            return Err(SettlementContractError::InvalidState(
                "unsupported contract version",
            ));
        }
        self.key.validate()?;
        validate_hash("intent_hash", &self.intent_hash)?;
        if deterministic_job_id(&self.key)? != self.job_id {
            return Err(SettlementContractError::InvalidJobId);
        }
        if self.attempts > MAX_SETTLEMENT_ATTEMPTS {
            return Err(SettlementContractError::InvalidState(
                "attempt count exceeds the contract maximum",
            ));
        }
        if self.lease_generation < u64::from(self.attempts) {
            return Err(SettlementContractError::InvalidState(
                "lease generation is behind the attempt count",
            ));
        }
        match &self.state {
            SettlementJobStateV1::Pending => {
                if self.attempts != 0 || self.lease_generation != 0 {
                    return Err(SettlementContractError::InvalidState(
                        "pending job has lease history",
                    ));
                }
            }
            SettlementJobStateV1::Leased {
                owner,
                generation,
                expires_at_ms,
            } => {
                validate_identifier("lease owner", owner)?;
                if *generation == 0 || *generation != self.lease_generation {
                    return Err(SettlementContractError::InvalidState(
                        "active lease generation is not current",
                    ));
                }
                if *expires_at_ms == 0 || self.attempts == 0 {
                    return Err(SettlementContractError::InvalidState(
                        "active lease has no expiry or attempt",
                    ));
                }
            }
            SettlementJobStateV1::Retryable {
                next_attempt_at_ms,
                last_error,
            } => {
                if self.attempts == 0 || *next_attempt_at_ms == 0 {
                    return Err(SettlementContractError::InvalidState(
                        "retryable job has no attempt or retry time",
                    ));
                }
                validate_detail("last_error", last_error)?;
            }
            SettlementJobStateV1::Succeeded {
                receipt_id,
                receipt_hash,
                completed_at_ms,
            } => {
                validate_identifier("receipt_id", receipt_id)?;
                validate_hash("receipt_hash", receipt_hash)?;
                if self.attempts == 0 || *completed_at_ms == 0 {
                    return Err(SettlementContractError::InvalidState(
                        "succeeded job has no attempt or completion time",
                    ));
                }
            }
            SettlementJobStateV1::DeadLetter {
                reason,
                failed_at_ms,
            } => {
                validate_detail("dead-letter reason", reason)?;
                if self.attempts == 0 || *failed_at_ms == 0 {
                    return Err(SettlementContractError::InvalidState(
                        "dead-letter job has no attempt or failure time",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn lease(
        &mut self,
        owner: &str,
        now_ms: u64,
        duration_ms: u64,
    ) -> Result<SettlementLeaseV1, SettlementContractError> {
        self.validate()?;
        validate_identifier("lease owner", owner)?;
        if duration_ms == 0 || duration_ms > MAX_LEASE_DURATION_MS {
            return Err(SettlementContractError::InvalidLeaseDuration);
        }
        let available = match &self.state {
            SettlementJobStateV1::Pending => true,
            SettlementJobStateV1::Retryable {
                next_attempt_at_ms, ..
            } => now_ms >= *next_attempt_at_ms,
            SettlementJobStateV1::Leased { expires_at_ms, .. } => now_ms >= *expires_at_ms,
            SettlementJobStateV1::Succeeded { .. } | SettlementJobStateV1::DeadLetter { .. } => {
                false
            }
        };
        if !available {
            return Err(SettlementContractError::LeaseNotAvailable);
        }
        if self.attempts >= MAX_SETTLEMENT_ATTEMPTS {
            return Err(SettlementContractError::AttemptsExhausted);
        }
        self.attempts = self
            .attempts
            .checked_add(1)
            .ok_or(SettlementContractError::ArithmeticOverflow)?;
        self.lease_generation = self
            .lease_generation
            .checked_add(1)
            .ok_or(SettlementContractError::ArithmeticOverflow)?;
        let expires_at_ms = now_ms
            .checked_add(duration_ms)
            .ok_or(SettlementContractError::ArithmeticOverflow)?;
        self.state = SettlementJobStateV1::Leased {
            owner: owner.to_string(),
            generation: self.lease_generation,
            expires_at_ms,
        };
        Ok(SettlementLeaseV1 {
            job_id: self.job_id.clone(),
            owner: owner.to_string(),
            generation: self.lease_generation,
            expires_at_ms,
            attempt: self.attempts,
        })
    }

    pub fn record_retryable(
        &mut self,
        owner: &str,
        generation: u64,
        now_ms: u64,
        next_attempt_at_ms: u64,
        error: &str,
    ) -> Result<(), SettlementContractError> {
        self.require_active_lease(owner, generation, now_ms)?;
        if next_attempt_at_ms <= now_ms {
            return Err(SettlementContractError::RetryTimeMustAdvance);
        }
        let bounded_error = bounded_detail(error)?;
        if self.attempts >= MAX_SETTLEMENT_ATTEMPTS {
            self.state = SettlementJobStateV1::DeadLetter {
                reason: bounded_error,
                failed_at_ms: now_ms,
            };
        } else {
            self.state = SettlementJobStateV1::Retryable {
                next_attempt_at_ms,
                last_error: bounded_error,
            };
        }
        Ok(())
    }

    pub fn complete(
        &mut self,
        owner: &str,
        generation: u64,
        now_ms: u64,
        receipt: &SettlementReceiptBindingV1,
    ) -> Result<(), SettlementContractError> {
        validate_receipt_shape(receipt)?;
        self.require_receipt_binding(receipt)?;
        if let SettlementJobStateV1::Succeeded {
            receipt_id,
            receipt_hash,
            ..
        } = &self.state
        {
            if receipt_id == &receipt.receipt_id && receipt_hash == &receipt.receipt_hash {
                return Ok(());
            }
            return Err(SettlementContractError::ReceiptMismatch(
                "terminal receipt",
            ));
        }
        self.require_active_lease(owner, generation, now_ms)?;
        self.state = SettlementJobStateV1::Succeeded {
            receipt_id: receipt.receipt_id.clone(),
            receipt_hash: receipt.receipt_hash.clone(),
            completed_at_ms: now_ms,
        };
        Ok(())
    }

    pub fn dead_letter(
        &mut self,
        owner: &str,
        generation: u64,
        now_ms: u64,
        reason: &str,
    ) -> Result<(), SettlementContractError> {
        self.require_active_lease(owner, generation, now_ms)?;
        self.state = SettlementJobStateV1::DeadLetter {
            reason: bounded_detail(reason)?,
            failed_at_ms: now_ms,
        };
        Ok(())
    }

    pub fn is_available_at(&self, now_ms: u64) -> bool {
        match &self.state {
            SettlementJobStateV1::Pending => self.attempts < MAX_SETTLEMENT_ATTEMPTS,
            SettlementJobStateV1::Retryable {
                next_attempt_at_ms, ..
            } => self.attempts < MAX_SETTLEMENT_ATTEMPTS && now_ms >= *next_attempt_at_ms,
            SettlementJobStateV1::Leased { expires_at_ms, .. } => {
                self.attempts < MAX_SETTLEMENT_ATTEMPTS && now_ms >= *expires_at_ms
            }
            SettlementJobStateV1::Succeeded { .. } | SettlementJobStateV1::DeadLetter { .. } => {
                false
            }
        }
    }

    pub fn state_name(&self) -> &'static str {
        match &self.state {
            SettlementJobStateV1::Pending => "pending",
            SettlementJobStateV1::Leased { .. } => "leased",
            SettlementJobStateV1::Retryable { .. } => "retryable",
            SettlementJobStateV1::Succeeded { .. } => "succeeded",
            SettlementJobStateV1::DeadLetter { .. } => "dead_letter",
        }
    }

    fn require_active_lease(
        &self,
        owner: &str,
        generation: u64,
        now_ms: u64,
    ) -> Result<(), SettlementContractError> {
        let (current_owner, current_generation, expires_at_ms) = match &self.state {
            SettlementJobStateV1::Leased {
                owner,
                generation,
                expires_at_ms,
            } => (owner.as_str(), *generation, *expires_at_ms),
            _ => {
                return Err(SettlementContractError::InvalidState(
                    "operation requires an active lease",
                ))
            }
        };
        if current_owner != owner {
            return Err(SettlementContractError::LeaseOwnerMismatch);
        }
        if current_generation != generation || generation != self.lease_generation {
            return Err(SettlementContractError::LeaseGenerationMismatch);
        }
        if now_ms >= expires_at_ms {
            return Err(SettlementContractError::LeaseExpired);
        }
        Ok(())
    }

    fn require_receipt_binding(
        &self,
        receipt: &SettlementReceiptBindingV1,
    ) -> Result<(), SettlementContractError> {
        if receipt.job_id != self.job_id {
            return Err(SettlementContractError::ReceiptMismatch("job_id"));
        }
        if receipt.intent_id != self.key.intent_id {
            return Err(SettlementContractError::ReceiptMismatch("intent_id"));
        }
        if receipt.intent_hash != self.intent_hash {
            return Err(SettlementContractError::ReceiptMismatch("intent_hash"));
        }
        Ok(())
    }
}

pub fn deterministic_job_id(
    key: &SettlementJobKeyV1,
) -> Result<String, SettlementContractError> {
    key.validate()?;
    let mut canonical = Vec::new();
    append_component(&mut canonical, SETTLEMENT_OUTBOX_CONTRACT)?;
    append_component(&mut canonical, &key.match_id)?;
    append_component(&mut canonical, &key.campaign_id)?;
    append_component(&mut canonical, &key.intent_id)?;
    Ok(format!("{JOB_ID_PREFIX}{}", hex_encode(&canonical)))
}

fn append_component(
    target: &mut Vec<u8>,
    value: &str,
) -> Result<(), SettlementContractError> {
    let length = u32::try_from(value.len()).map_err(|_| {
        SettlementContractError::FieldTooLong("canonical job component")
    })?;
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(value.as_bytes());
    Ok(())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), SettlementContractError> {
    if value.is_empty() {
        return Err(SettlementContractError::EmptyField(field));
    }
    if value.len() > MAX_ID_BYTES {
        return Err(SettlementContractError::FieldTooLong(field));
    }
    if value.trim() != value {
        return Err(SettlementContractError::SurroundingWhitespace(field));
    }
    Ok(())
}

fn validate_hash(
    field: &'static str,
    value: &str,
) -> Result<(), SettlementContractError> {
    if value.len() != 64
        || !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(SettlementContractError::InvalidHash(field));
    }
    Ok(())
}

fn validate_detail(
    field: &'static str,
    value: &str,
) -> Result<(), SettlementContractError> {
    if value.trim().is_empty() {
        return Err(SettlementContractError::EmptyField(field));
    }
    if value.chars().count() > MAX_ERROR_DETAIL_CHARS {
        return Err(SettlementContractError::FieldTooLong(field));
    }
    Ok(())
}

fn bounded_detail(value: &str) -> Result<String, SettlementContractError> {
    if value.trim().is_empty() {
        return Err(SettlementContractError::EmptyField("error detail"));
    }
    Ok(value.chars().take(MAX_ERROR_DETAIL_CHARS).collect())
}

fn validate_receipt_shape(
    receipt: &SettlementReceiptBindingV1,
) -> Result<(), SettlementContractError> {
    if receipt.job_id.is_empty() || receipt.job_id.len() > 2_048 {
        return Err(SettlementContractError::InvalidJobId);
    }
    validate_identifier("receipt intent_id", &receipt.intent_id)?;
    validate_hash("receipt intent_hash", &receipt.intent_hash)?;
    validate_identifier("receipt_id", &receipt.receipt_id)?;
    validate_hash("receipt_hash", &receipt.receipt_hash)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(character: char) -> String {
        std::iter::repeat(character).take(64).collect()
    }

    fn key(intent_id: &str) -> SettlementJobKeyV1 {
        SettlementJobKeyV1 {
            match_id: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(),
            campaign_id: "online-campaign:player-a:slot-main".to_string(),
            intent_id: intent_id.to_string(),
        }
    }

    fn receipt(job: &SettlementJobV1) -> SettlementReceiptBindingV1 {
        SettlementReceiptBindingV1 {
            job_id: job.job_id.clone(),
            intent_id: job.key.intent_id.clone(),
            intent_hash: job.intent_hash.clone(),
            receipt_id: "cex-receipt-1".to_string(),
            receipt_hash: hash('b'),
        }
    }

    #[test]
    fn deterministic_job_id_is_stable_and_unambiguous() {
        let first = deterministic_job_id(&key("intent-a")).unwrap();
        let second = deterministic_job_id(&key("intent-a")).unwrap();
        let third = deterministic_job_id(&key("intent-b")).unwrap();
        assert_eq!(first, second);
        assert_ne!(first, third);
        assert!(first.starts_with(JOB_ID_PREFIX));
    }

    #[test]
    fn new_job_starts_pending_and_valid() {
        let job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        assert_eq!(job.state_name(), "pending");
        assert_eq!(job.attempts, 0);
        assert!(job.is_available_at(0));
        job.validate().unwrap();
    }

    #[test]
    fn lease_increments_attempt_and_generation() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let lease = job.lease("worker-a", 1_000, 5_000).unwrap();
        assert_eq!(lease.attempt, 1);
        assert_eq!(lease.generation, 1);
        assert_eq!(lease.expires_at_ms, 6_000);
        assert_eq!(job.state_name(), "leased");
        job.validate().unwrap();
    }

    #[test]
    fn retry_is_not_available_before_due_time() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let lease = job.lease("worker-a", 1_000, 5_000).unwrap();
        job.record_retryable(
            &lease.owner,
            lease.generation,
            2_000,
            10_000,
            "CEX transport timeout",
        )
        .unwrap();
        assert!(!job.is_available_at(9_999));
        assert!(job.is_available_at(10_000));
        assert_eq!(job.state_name(), "retryable");
    }

    #[test]
    fn expired_lease_can_be_reclaimed_with_higher_generation() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let first = job.lease("worker-a", 1_000, 1_000).unwrap();
        assert!(!job.is_available_at(1_999));
        assert!(job.is_available_at(2_000));
        let second = job.lease("worker-b", 2_000, 1_000).unwrap();
        assert_eq!(second.attempt, 2);
        assert!(second.generation > first.generation);
    }

    #[test]
    fn stale_worker_cannot_complete_after_lease_reclaim() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let first = job.lease("worker-a", 1_000, 1_000).unwrap();
        let second = job.lease("worker-b", 2_000, 1_000).unwrap();
        let binding = receipt(&job);
        let result = job.complete(&first.owner, first.generation, 2_100, &binding);
        assert_eq!(result, Err(SettlementContractError::LeaseOwnerMismatch));
        job.complete(&second.owner, second.generation, 2_100, &binding)
            .unwrap();
    }

    #[test]
    fn mismatched_receipt_fails_closed() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let lease = job.lease("worker-a", 1_000, 5_000).unwrap();
        let mut wrong = receipt(&job);
        wrong.intent_hash = hash('c');
        assert_eq!(
            job.complete(&lease.owner, lease.generation, 2_000, &wrong),
            Err(SettlementContractError::ReceiptMismatch("intent_hash"))
        );
        assert_eq!(job.state_name(), "leased");
    }

    #[test]
    fn exact_duplicate_completion_is_idempotent() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let lease = job.lease("worker-a", 1_000, 5_000).unwrap();
        let receipt = receipt(&job);
        job.complete(&lease.owner, lease.generation, 2_000, &receipt)
            .unwrap();
        job.complete("ignored-after-success", 999, 3_000, &receipt)
            .unwrap();
        assert_eq!(job.state_name(), "succeeded");
        job.validate().unwrap();
    }

    #[test]
    fn sixteenth_failed_attempt_becomes_dead_letter() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        for attempt in 1..=MAX_SETTLEMENT_ATTEMPTS {
            let now = u64::from(attempt) * 10_000;
            let lease = job.lease("worker-a", now, 1_000).unwrap();
            job.record_retryable(
                &lease.owner,
                lease.generation,
                now + 1,
                now + 2_000,
                "bounded failure",
            )
            .unwrap();
        }
        assert_eq!(job.state_name(), "dead_letter");
        assert!(!job.is_available_at(u64::MAX));
        job.validate().unwrap();
    }

    #[test]
    fn explicit_dead_letter_is_terminal() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        let lease = job.lease("worker-a", 1_000, 5_000).unwrap();
        job.dead_letter(
            &lease.owner,
            lease.generation,
            2_000,
            "receipt signature is invalid",
        )
        .unwrap();
        assert_eq!(job.state_name(), "dead_letter");
        assert_eq!(
            job.lease("worker-b", 3_000, 1_000),
            Err(SettlementContractError::LeaseNotAvailable)
        );
    }

    #[test]
    fn corrupted_job_id_is_rejected() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        job.job_id.push('0');
        assert_eq!(job.validate(), Err(SettlementContractError::InvalidJobId));
    }

    #[test]
    fn hashes_must_be_lowercase_hex() {
        assert_eq!(
            SettlementJobV1::new(key("intent-a"), 7, hash('A')),
            Err(SettlementContractError::InvalidHash("intent_hash"))
        );
    }

    #[test]
    fn lease_duration_is_bounded() {
        let mut job = SettlementJobV1::new(key("intent-a"), 7, hash('a')).unwrap();
        assert_eq!(
            job.lease("worker-a", 1_000, 0),
            Err(SettlementContractError::InvalidLeaseDuration)
        );
        assert_eq!(
            job.lease("worker-a", 1_000, MAX_LEASE_DURATION_MS + 1),
            Err(SettlementContractError::InvalidLeaseDuration)
        );
    }
}
