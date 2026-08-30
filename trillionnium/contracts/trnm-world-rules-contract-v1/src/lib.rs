#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

mod boundary_generated;
mod canonical;
mod digest;
mod engine;
mod error;
mod model;

pub use boundary_generated::{
    bound_diagnostic_utf8, is_forbidden_authority_key, DIAGNOSTIC_FALLBACK,
    DIAGNOSTIC_MAX_UTF8_BYTES, FORBIDDEN_AUTHORITY_KEYS,
};
pub use digest::{hex_encode, sha256, Digest32};
pub use engine::{execute_transition, execute_transition_verified, WorldRulesEngine};
pub use error::{StableErrorCode, TransitionFailure};
pub use model::{
    EngineOutput, ResourceBudget, TransitionDisposition, TransitionReceipt, TransitionRequest,
    MAX_COMMAND_BYTES, MAX_OUTCOME_BYTES, MAX_REPLAY_BYTES, MAX_STATE_BYTES, MAX_STEPS,
};

pub const CONTRACT_VERSION: &str = "trnm_world_rules_v1";
pub const CONTRACT_RELEASE: &str = "trnm_world_rules_v1@1.0.0-alpha.1";
pub const CANONICAL_ENCODING: &str = "trnm-canonical-lines-v1";
pub const HASH_ALGORITHM: &str = "sha256";

/// This package intentionally cannot represent online admission, player
/// sessions, canonical global event order, archive roots, signing keys, Chain
/// finality, or wallet settlement. It is a pure deterministic game-rules
/// boundary for World and an adapter conformance target for Nakama.
pub const AUTHORITY_SCOPE: &str = "deterministic_world_rules_only";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_contract_identity_is_explicit_and_versioned() {
        assert_eq!(CONTRACT_VERSION, "trnm_world_rules_v1");
        assert!(CONTRACT_RELEASE.starts_with(CONTRACT_VERSION));
        assert_eq!(CANONICAL_ENCODING, "trnm-canonical-lines-v1");
        assert_eq!(HASH_ALGORITHM, "sha256");
        assert_eq!(AUTHORITY_SCOPE, "deterministic_world_rules_only");
    }

    #[test]
    fn public_model_has_no_online_authority_material() {
        let request = TransitionRequest::new(
            "ruleset_v1",
            "content_v1",
            "transition-1",
            b"state".to_vec(),
            b"command".to_vec(),
            ResourceBudget::default(),
        );
        let canonical = String::from_utf8(request.canonical_bytes()).unwrap();
        for forbidden in FORBIDDEN_AUTHORITY_KEYS {
            assert!(!canonical.contains(forbidden));
            assert!(is_forbidden_authority_key(forbidden));
            assert!(is_forbidden_authority_key(&forbidden.to_ascii_uppercase()));
        }
    }

    #[test]
    fn generated_registry_includes_every_cross_runtime_authority_class() {
        for required in [
            "participant_roster",
            "global_event_sequence",
            "match_version",
            "command_idempotency_key",
            "completion_signature",
            "wallet",
            "settlement",
        ] {
            assert!(FORBIDDEN_AUTHORITY_KEYS.contains(&required));
        }
        assert_eq!(DIAGNOSTIC_MAX_UTF8_BYTES, 256);
        assert_eq!(DIAGNOSTIC_FALLBACK, "request rejected");
    }
}
