use crate::{
    canonical::{safe_token, CanonicalWriter, REQUEST_DOMAIN, RESULT_DOMAIN},
    digest::{sha256, Digest32},
    error::{StableErrorCode, TransitionFailure},
    CONTRACT_VERSION,
};

pub const MAX_STATE_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_COMMAND_BYTES: usize = 256 * 1024;
pub const MAX_OUTCOME_BYTES: usize = 1024 * 1024;
pub const MAX_REPLAY_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_STEPS: u64 = 10_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudget {
    pub max_steps: u64,
    pub max_output_bytes: u64,
    pub max_replay_bytes: u64,
}

impl ResourceBudget {
    pub const fn conservative_default() -> Self {
        Self {
            max_steps: 1_000_000,
            max_output_bytes: 2 * 1024 * 1024,
            max_replay_bytes: 8 * 1024 * 1024,
        }
    }

    pub fn validate(self) -> Result<(), TransitionFailure> {
        if self.max_steps == 0
            || self.max_steps > MAX_STEPS
            || self.max_output_bytes == 0
            || self.max_output_bytes > (MAX_STATE_BYTES + MAX_OUTCOME_BYTES) as u64
            || self.max_replay_bytes == 0
            || self.max_replay_bytes > MAX_REPLAY_BYTES as u64
        {
            return Err(TransitionFailure::new(
                StableErrorCode::InvalidResourceBudget,
                "resource budget is zero or exceeds the contract ceiling",
            ));
        }
        Ok(())
    }
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self::conservative_default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionRequest {
    pub contract_version: String,
    pub ruleset_revision: String,
    pub content_revision: String,
    pub transition_id: String,
    pub state_canonical: Vec<u8>,
    pub command_canonical: Vec<u8>,
    pub budget: ResourceBudget,
}

impl TransitionRequest {
    pub fn new(
        ruleset_revision: impl Into<String>,
        content_revision: impl Into<String>,
        transition_id: impl Into<String>,
        state_canonical: Vec<u8>,
        command_canonical: Vec<u8>,
        budget: ResourceBudget,
    ) -> Self {
        Self {
            contract_version: CONTRACT_VERSION.to_string(),
            ruleset_revision: ruleset_revision.into(),
            content_revision: content_revision.into(),
            transition_id: transition_id.into(),
            state_canonical,
            command_canonical,
            budget,
        }
    }

    pub fn validate(&self) -> Result<(), TransitionFailure> {
        if self.contract_version != CONTRACT_VERSION {
            return Err(TransitionFailure::new(
                StableErrorCode::UnsupportedContractVersion,
                "contract version is not supported",
            ));
        }
        if !safe_token(&self.ruleset_revision, 128) {
            return Err(TransitionFailure::new(
                StableErrorCode::UnknownRulesetRevision,
                "ruleset revision is empty or not a canonical token",
            ));
        }
        if !safe_token(&self.content_revision, 128) {
            return Err(TransitionFailure::new(
                StableErrorCode::InvalidContentRevision,
                "content revision is empty or not a canonical token",
            ));
        }
        if !safe_token(&self.transition_id, 192) {
            return Err(TransitionFailure::new(
                StableErrorCode::InvalidTransitionId,
                "transition id is empty or not a canonical token",
            ));
        }
        if self.state_canonical.is_empty() || self.state_canonical.len() > MAX_STATE_BYTES {
            return Err(TransitionFailure::new(
                StableErrorCode::MalformedState,
                "canonical state is empty or exceeds the contract ceiling",
            ));
        }
        if self.command_canonical.is_empty() || self.command_canonical.len() > MAX_COMMAND_BYTES {
            return Err(TransitionFailure::new(
                StableErrorCode::MalformedCommand,
                "canonical command is empty or exceeds the contract ceiling",
            ));
        }
        self.budget.validate()
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut writer = CanonicalWriter::with_domain(REQUEST_DOMAIN);
        writer.field("contract", &self.contract_version);
        writer.field("ruleset", &self.ruleset_revision);
        writer.field("content", &self.content_revision);
        writer.field("transition", &self.transition_id);
        writer.u64_field("max_steps", self.budget.max_steps);
        writer.u64_field("max_output_bytes", self.budget.max_output_bytes);
        writer.u64_field("max_replay_bytes", self.budget.max_replay_bytes);
        writer.bytes_field("state", &self.state_canonical);
        writer.bytes_field("command", &self.command_canonical);
        writer.finish()
    }

    pub fn request_hash(&self) -> Digest32 {
        sha256(&self.canonical_bytes())
    }

    pub fn state_before_hash(&self) -> Digest32 {
        sha256(&self.state_canonical)
    }

    pub fn command_hash(&self) -> Digest32 {
        sha256(&self.command_canonical)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EngineOutput {
    pub state_after_canonical: Vec<u8>,
    pub outcome_canonical: Vec<u8>,
    pub replay_canonical: Vec<u8>,
    pub steps_used: u64,
}

impl EngineOutput {
    pub fn output_bytes(&self) -> u64 {
        (self.state_after_canonical.len() + self.outcome_canonical.len()) as u64
    }

    pub fn replay_bytes(&self) -> u64 {
        self.replay_canonical.len() as u64
    }

    pub fn validate_against(&self, budget: ResourceBudget) -> Result<(), TransitionFailure> {
        if self.state_after_canonical.is_empty()
            || self.state_after_canonical.len() > MAX_STATE_BYTES
            || self.outcome_canonical.len() > MAX_OUTCOME_BYTES
            || self.replay_canonical.len() > MAX_REPLAY_BYTES
        {
            return Err(TransitionFailure::new(
                StableErrorCode::OutputTooLarge,
                "engine output violates the contract size ceiling",
            ));
        }
        if self.steps_used > budget.max_steps
            || self.output_bytes() > budget.max_output_bytes
            || self.replay_bytes() > budget.max_replay_bytes
        {
            return Err(TransitionFailure::new(
                StableErrorCode::ResourceBudgetExceeded,
                "engine output exceeds the request resource budget",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionDisposition {
    Applied(EngineOutput),
    Rejected(TransitionFailure),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionReceipt {
    pub contract_version: String,
    pub ruleset_revision: String,
    pub content_revision: String,
    pub transition_id: String,
    pub request_hash: Digest32,
    pub state_before_hash: Digest32,
    pub command_hash: Digest32,
    pub applied: bool,
    pub error_code: Option<StableErrorCode>,
    pub state_after_hash: Digest32,
    pub outcome_hash: Digest32,
    pub replay_hash: Digest32,
    pub steps_used: u64,
    pub output_bytes: u64,
    pub replay_bytes: u64,
    pub transition_hash: Digest32,
    /// Diagnostics are intentionally not committed by transition_hash.
    pub diagnostic: Option<String>,
}

impl TransitionReceipt {
    pub(crate) fn applied(request: &TransitionRequest, output: EngineOutput) -> Self {
        let mut receipt = Self {
            contract_version: request.contract_version.clone(),
            ruleset_revision: request.ruleset_revision.clone(),
            content_revision: request.content_revision.clone(),
            transition_id: request.transition_id.clone(),
            request_hash: request.request_hash(),
            state_before_hash: request.state_before_hash(),
            command_hash: request.command_hash(),
            applied: true,
            error_code: None,
            state_after_hash: sha256(&output.state_after_canonical),
            outcome_hash: sha256(&output.outcome_canonical),
            replay_hash: sha256(&output.replay_canonical),
            steps_used: output.steps_used,
            output_bytes: output.output_bytes(),
            replay_bytes: output.replay_bytes(),
            transition_hash: Digest32::ZERO,
            diagnostic: None,
        };
        receipt.transition_hash = sha256(&receipt.canonical_bytes_without_transition_hash());
        receipt
    }

    pub(crate) fn rejected(request: &TransitionRequest, failure: TransitionFailure) -> Self {
        let mut receipt = Self {
            contract_version: request.contract_version.clone(),
            ruleset_revision: request.ruleset_revision.clone(),
            content_revision: request.content_revision.clone(),
            transition_id: request.transition_id.clone(),
            request_hash: request.request_hash(),
            state_before_hash: request.state_before_hash(),
            command_hash: request.command_hash(),
            applied: false,
            error_code: Some(failure.code),
            state_after_hash: Digest32::ZERO,
            outcome_hash: Digest32::ZERO,
            replay_hash: Digest32::ZERO,
            steps_used: 0,
            output_bytes: 0,
            replay_bytes: 0,
            transition_hash: Digest32::ZERO,
            diagnostic: Some(failure.diagnostic),
        };
        receipt.transition_hash = sha256(&receipt.canonical_bytes_without_transition_hash());
        receipt
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = self.canonical_bytes_without_transition_hash();
        bytes.extend_from_slice(b"transition_hash=");
        bytes.extend_from_slice(self.transition_hash.to_hex().as_bytes());
        bytes.push(b'\n');
        bytes
    }

    fn canonical_bytes_without_transition_hash(&self) -> Vec<u8> {
        let mut writer = CanonicalWriter::with_domain(RESULT_DOMAIN);
        writer.field("contract", &self.contract_version);
        writer.field("ruleset", &self.ruleset_revision);
        writer.field("content", &self.content_revision);
        writer.field("transition", &self.transition_id);
        writer.field("request_hash", &self.request_hash.to_hex());
        writer.field("state_before_hash", &self.state_before_hash.to_hex());
        writer.field("command_hash", &self.command_hash.to_hex());
        writer.field("disposition", if self.applied { "applied" } else { "rejected" });
        writer.field(
            "error_code",
            self.error_code.map_or("none", StableErrorCode::as_str),
        );
        writer.field("state_after_hash", &self.state_after_hash.to_hex());
        writer.field("outcome_hash", &self.outcome_hash.to_hex());
        writer.field("replay_hash", &self.replay_hash.to_hex());
        writer.u64_field("steps_used", self.steps_used);
        writer.u64_field("output_bytes", self.output_bytes);
        writer.u64_field("replay_bytes", self.replay_bytes);
        writer.finish()
    }

    pub fn verify_self_consistency(&self) -> bool {
        sha256(&self.canonical_bytes_without_transition_hash()) == self.transition_hash
            && self.applied == self.error_code.is_none()
            && (self.applied
                || (self.state_after_hash == Digest32::ZERO
                    && self.outcome_hash == Digest32::ZERO
                    && self.replay_hash == Digest32::ZERO
                    && self.steps_used == 0
                    && self.output_bytes == 0
                    && self.replay_bytes == 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> TransitionRequest {
        TransitionRequest::new(
            "first_contact_v1",
            "content_2026_08_27",
            "vector-0001",
            b"state-v1".to_vec(),
            b"command-v1".to_vec(),
            ResourceBudget {
                max_steps: 100,
                max_output_bytes: 4096,
                max_replay_bytes: 4096,
            },
        )
    }

    #[test]
    fn canonical_request_vector_is_exact_and_ordered() {
        assert_eq!(
            String::from_utf8(request().canonical_bytes()).unwrap(),
            concat!(
                "TRNM-WORLD-RULES-REQUEST/1\n",
                "contract=trnm_world_rules_v1\n",
                "ruleset=first_contact_v1\n",
                "content=content_2026_08_27\n",
                "transition=vector-0001\n",
                "max_steps=100\n",
                "max_output_bytes=4096\n",
                "max_replay_bytes=4096\n",
                "state=73746174652d7631\n",
                "command=636f6d6d616e642d7631\n",
            )
        );
    }

    #[test]
    fn request_rejects_online_style_or_ambiguous_tokens() {
        let mut invalid = request();
        invalid.transition_id = "player=session".to_string();
        assert_eq!(
            invalid.validate().unwrap_err().code,
            StableErrorCode::InvalidTransitionId
        );
        invalid = request();
        invalid.ruleset_revision = "first contact".to_string();
        assert_eq!(
            invalid.validate().unwrap_err().code,
            StableErrorCode::UnknownRulesetRevision
        );
    }

    #[test]
    fn diagnostics_do_not_change_rejected_commitment() {
        let first = TransitionReceipt::rejected(
            &request(),
            TransitionFailure::new(StableErrorCode::DomainRejected, "english diagnostic"),
        );
        let second = TransitionReceipt::rejected(
            &request(),
            TransitionFailure::new(StableErrorCode::DomainRejected, "localized diagnostic"),
        );
        assert_eq!(first.transition_hash, second.transition_hash);
        assert_ne!(first.diagnostic, second.diagnostic);
        assert!(first.verify_self_consistency());
    }
}
