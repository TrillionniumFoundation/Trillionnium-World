use crate::{bound_diagnostic_utf8, DIAGNOSTIC_FALLBACK, DIAGNOSTIC_MAX_UTF8_BYTES};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StableErrorCode {
    UnsupportedContractVersion,
    UnknownRulesetRevision,
    InvalidContentRevision,
    InvalidTransitionId,
    MalformedState,
    MalformedCommand,
    InvalidResourceBudget,
    ResourceBudgetExceeded,
    DomainRejected,
    OutputTooLarge,
    NondeterministicResult,
    InternalContractError,
}

impl StableErrorCode {
    pub const ALL: [Self; 12] = [
        Self::UnsupportedContractVersion,
        Self::UnknownRulesetRevision,
        Self::InvalidContentRevision,
        Self::InvalidTransitionId,
        Self::MalformedState,
        Self::MalformedCommand,
        Self::InvalidResourceBudget,
        Self::ResourceBudgetExceeded,
        Self::DomainRejected,
        Self::OutputTooLarge,
        Self::NondeterministicResult,
        Self::InternalContractError,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedContractVersion => "unsupported_contract_version",
            Self::UnknownRulesetRevision => "unknown_ruleset_revision",
            Self::InvalidContentRevision => "invalid_content_revision",
            Self::InvalidTransitionId => "invalid_transition_id",
            Self::MalformedState => "malformed_state",
            Self::MalformedCommand => "malformed_command",
            Self::InvalidResourceBudget => "invalid_resource_budget",
            Self::ResourceBudgetExceeded => "resource_budget_exceeded",
            Self::DomainRejected => "domain_rejected",
            Self::OutputTooLarge => "output_too_large",
            Self::NondeterministicResult => "nondeterministic_result",
            Self::InternalContractError => "internal_contract_error",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.as_str() == value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionFailure {
    pub code: StableErrorCode,
    /// Human-readable diagnostics are deliberately excluded from canonical
    /// result commitments. Adapters may redact or localize this value without
    /// changing authoritative deterministic facts. The generated boundary
    /// still normalizes controls and caps UTF-8 bytes so implementations agree.
    pub diagnostic: String,
}

impl TransitionFailure {
    pub fn new(code: StableErrorCode, diagnostic: impl Into<String>) -> Self {
        let diagnostic = diagnostic.into();
        Self {
            code,
            diagnostic: bound_diagnostic_utf8(&diagnostic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_round_trip_without_aliases() {
        for code in StableErrorCode::ALL {
            assert_eq!(StableErrorCode::parse(code.as_str()), Some(code));
        }
        assert_eq!(StableErrorCode::parse(""), None);
        assert_eq!(StableErrorCode::parse("UNKNOWN_RULESET_REVISION"), None);
    }

    #[test]
    fn stable_error_strings_are_unique_and_snake_case() {
        let mut values = StableErrorCode::ALL
            .into_iter()
            .map(StableErrorCode::as_str)
            .collect::<Vec<_>>();
        let original_length = values.len();
        values.sort_unstable();
        values.dedup();
        assert_eq!(values.len(), original_length);
        assert!(values.iter().all(|value| value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
        })));
    }

    #[test]
    fn diagnostics_share_the_exact_utf8_byte_boundary() {
        let diagnostic = format!("{}界", "a".repeat(DIAGNOSTIC_MAX_UTF8_BYTES - 1));
        let failure = TransitionFailure::new(StableErrorCode::DomainRejected, diagnostic);
        assert!(failure.diagnostic.len() <= DIAGNOSTIC_MAX_UTF8_BYTES);
        assert!(failure.diagnostic.ends_with('a'));
        assert!(!failure.diagnostic.ends_with('界'));

        let controls = TransitionFailure::new(
            StableErrorCode::DomainRejected,
            "failure\nreason\r\t",
        );
        assert_eq!(controls.diagnostic, "failure reason");

        let empty = TransitionFailure::new(StableErrorCode::DomainRejected, "\n\r\t");
        assert_eq!(empty.diagnostic, DIAGNOSTIC_FALLBACK);
    }
}
