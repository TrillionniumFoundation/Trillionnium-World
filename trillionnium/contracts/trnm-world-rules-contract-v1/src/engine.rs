use crate::{
    error::{StableErrorCode, TransitionFailure},
    model::{EngineOutput, TransitionDisposition, TransitionReceipt, TransitionRequest},
};

/// The engine boundary is intentionally game-domain-only. It has no player
/// session, admission, global sequence, archive-root, signing, wallet, or Chain
/// finality surface. Those responsibilities stay with their owning systems.
pub trait WorldRulesEngine {
    fn supports(&self, ruleset_revision: &str, content_revision: &str) -> bool;

    fn execute(&self, request: &TransitionRequest) -> Result<EngineOutput, TransitionFailure>;
}

pub fn execute_transition<E: WorldRulesEngine>(
    engine: &E,
    request: &TransitionRequest,
) -> Result<TransitionReceipt, TransitionFailure> {
    request.validate()?;
    let disposition = evaluate(engine, request);
    Ok(receipt_from_disposition(request, disposition))
}

/// Executes the same immutable input twice and fails closed when deterministic
/// facts differ. This is suitable for fixture/shadow verification; production
/// adapters may execute once and rely on cross-runtime shadow comparison.
pub fn execute_transition_verified<E: WorldRulesEngine>(
    engine: &E,
    request: &TransitionRequest,
) -> Result<TransitionReceipt, TransitionFailure> {
    request.validate()?;
    let first = evaluate(engine, request);
    let second = evaluate(engine, request);
    if !same_deterministic_facts(&first, &second) {
        return Ok(TransitionReceipt::rejected(
            request,
            TransitionFailure::new(
                StableErrorCode::NondeterministicResult,
                "identical request produced different deterministic facts",
            ),
        ));
    }
    Ok(receipt_from_disposition(request, first))
}

fn evaluate<E: WorldRulesEngine>(
    engine: &E,
    request: &TransitionRequest,
) -> TransitionDisposition {
    if !engine.supports(&request.ruleset_revision, &request.content_revision) {
        return TransitionDisposition::Rejected(TransitionFailure::new(
            StableErrorCode::UnknownRulesetRevision,
            "engine does not advertise this ruleset/content pair",
        ));
    }
    match engine.execute(request) {
        Ok(output) => match output.validate_against(request.budget) {
            Ok(()) => TransitionDisposition::Applied(output),
            Err(failure) => TransitionDisposition::Rejected(failure),
        },
        Err(failure) => TransitionDisposition::Rejected(failure),
    }
}

fn receipt_from_disposition(
    request: &TransitionRequest,
    disposition: TransitionDisposition,
) -> TransitionReceipt {
    match disposition {
        TransitionDisposition::Applied(output) => TransitionReceipt::applied(request, output),
        TransitionDisposition::Rejected(failure) => TransitionReceipt::rejected(request, failure),
    }
}

fn same_deterministic_facts(
    first: &TransitionDisposition,
    second: &TransitionDisposition,
) -> bool {
    match (first, second) {
        (TransitionDisposition::Applied(first), TransitionDisposition::Applied(second)) => {
            first == second
        }
        (TransitionDisposition::Rejected(first), TransitionDisposition::Rejected(second)) => {
            first.code == second.code
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ResourceBudget;
    use core::cell::Cell;

    struct EchoEngine;

    impl WorldRulesEngine for EchoEngine {
        fn supports(&self, ruleset_revision: &str, content_revision: &str) -> bool {
            ruleset_revision == "first_contact_v1" && content_revision == "content_2026_08_27"
        }

        fn execute(&self, request: &TransitionRequest) -> Result<EngineOutput, TransitionFailure> {
            let mut state = request.state_canonical.clone();
            state.extend_from_slice(b"|");
            state.extend_from_slice(&request.command_canonical);
            Ok(EngineOutput {
                state_after_canonical: state,
                outcome_canonical: b"outcome:victory".to_vec(),
                replay_canonical: b"replay:frame-0".to_vec(),
                steps_used: 3,
            })
        }
    }

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
    fn verified_execution_commits_only_deterministic_facts() {
        let receipt = execute_transition_verified(&EchoEngine, &request()).unwrap();
        assert!(receipt.applied);
        assert_eq!(receipt.steps_used, 3);
        assert_eq!(receipt.error_code, None);
        assert!(receipt.verify_self_consistency());
        assert_eq!(
            receipt.transition_hash,
            execute_transition_verified(&EchoEngine, &request())
                .unwrap()
                .transition_hash
        );
    }

    struct UnsupportedEngine;

    impl WorldRulesEngine for UnsupportedEngine {
        fn supports(&self, _: &str, _: &str) -> bool {
            false
        }

        fn execute(&self, _: &TransitionRequest) -> Result<EngineOutput, TransitionFailure> {
            panic!("unsupported engine must not execute")
        }
    }

    #[test]
    fn unknown_ruleset_fails_closed_with_stable_code() {
        let receipt = execute_transition(&UnsupportedEngine, &request()).unwrap();
        assert!(!receipt.applied);
        assert_eq!(
            receipt.error_code,
            Some(StableErrorCode::UnknownRulesetRevision)
        );
        assert!(receipt.verify_self_consistency());
    }

    struct FlakyEngine {
        call: Cell<u8>,
    }

    impl WorldRulesEngine for FlakyEngine {
        fn supports(&self, _: &str, _: &str) -> bool {
            true
        }

        fn execute(&self, _: &TransitionRequest) -> Result<EngineOutput, TransitionFailure> {
            let next = self.call.get().wrapping_add(1);
            self.call.set(next);
            Ok(EngineOutput {
                state_after_canonical: vec![next],
                outcome_canonical: b"outcome".to_vec(),
                replay_canonical: b"replay".to_vec(),
                steps_used: 1,
            })
        }
    }

    #[test]
    fn nondeterministic_engine_is_rejected_not_partially_accepted() {
        let engine = FlakyEngine { call: Cell::new(0) };
        let receipt = execute_transition_verified(&engine, &request()).unwrap();
        assert!(!receipt.applied);
        assert_eq!(
            receipt.error_code,
            Some(StableErrorCode::NondeterministicResult)
        );
        assert_eq!(receipt.state_after_hash, crate::Digest32::ZERO);
    }

    struct OversizedEngine;

    impl WorldRulesEngine for OversizedEngine {
        fn supports(&self, _: &str, _: &str) -> bool {
            true
        }

        fn execute(&self, request: &TransitionRequest) -> Result<EngineOutput, TransitionFailure> {
            Ok(EngineOutput {
                state_after_canonical: vec![1; request.budget.max_output_bytes as usize + 1],
                outcome_canonical: Vec::new(),
                replay_canonical: b"replay".to_vec(),
                steps_used: 1,
            })
        }
    }

    #[test]
    fn resource_budget_breach_has_no_committed_game_output() {
        let receipt = execute_transition(&OversizedEngine, &request()).unwrap();
        assert!(!receipt.applied);
        assert_eq!(
            receipt.error_code,
            Some(StableErrorCode::ResourceBudgetExceeded)
        );
        assert_eq!(receipt.state_after_hash, crate::Digest32::ZERO);
    }
}
