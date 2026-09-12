//! Deterministic World commands with candidate-only publication.
//!
//! The private engine preserves the existing decision vocabulary. The public
//! boundary publishes state only on acceptance. Authentication, ordering,
//! durable idempotency and database commit remain the caller's responsibility.

mod engine;
pub use engine::*;

use trnm_world_domain::{WorldState, WorldTrillionniumCharacter};

/// A separate execution-policy identity; command/outcome value formats are unchanged.
/// Promotion requires new exact-component and replay evidence for this policy.
pub const WORLD_COMMAND_PUBLICATION_CONTRACT: &str =
    "trillionnium_world_command_publication_v2";

fn staged<S: Clone, D>(state: &mut S, evaluate: impl FnOnce(&mut S) -> (D, bool)) -> D {
    let mut candidate = state.clone();
    let (decision, accepted) = evaluate(&mut candidate);
    if accepted {
        *state = candidate;
    }
    decision
}

/// Apply an admitted command without publishing a rejected candidate.
pub fn apply_command(state: &mut WorldState, command: WorldCommand) -> WorldCommandDecision {
    staged(state, |candidate| {
        let decision = engine::apply_command(candidate, command);
        let accepted = decision.accepted;
        (decision, accepted)
    })
}

/// Defaults and every helper mutation occur on a private candidate. A rejected
/// response carries the original character, never the discarded candidate.
pub fn apply_tactics_command(
    character: &mut WorldTrillionniumCharacter,
    request: WorldTacticsCommandRequest,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    let mut outcome = staged(character, |candidate| {
        let outcome = engine::apply_tactics_command(candidate, request, now_epoch);
        let accepted = outcome.accepted;
        (outcome, accepted)
    });
    outcome.character = character.clone();
    outcome
}

#[cfg(test)]
mod publication_tests {
    use super::*;

    fn request(command: &str) -> WorldTacticsCommandRequest {
        WorldTacticsCommandRequest {
            command: command.to_string(),
            unit_id: "lord".to_string(),
            ..WorldTacticsCommandRequest::default()
        }
    }

    #[test]
    fn staged_rejection_discards_all_mutations() {
        let mut state = vec![1_u64, 2];
        let before = state.clone();
        let result = staged(&mut state, |candidate| {
            candidate.clear();
            candidate.push(99);
            ("rejected", false)
        });
        assert_eq!(result, "rejected");
        assert_eq!(state, before);
    }

    #[test]
    fn staged_acceptance_publishes_the_complete_candidate() {
        let mut state = vec![1_u64];
        staged(&mut state, |candidate| {
            candidate.extend([2, 3]);
            ((), true)
        });
        assert_eq!(state, vec![1, 2, 3]);
    }

    #[test]
    fn unwinding_before_publication_preserves_input() {
        let mut state = vec![1_u64, 2];
        let before = state.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            staged(&mut state, |candidate| -> ((), bool) {
                candidate.clear();
                panic!("test-only failure before publication");
            });
        }));
        assert!(result.is_err());
        assert_eq!(state, before);
    }

    #[test]
    fn tactics_rejections_preserve_full_state_and_response_state() {
        let mut cases = vec![request("unknown_command")];
        let mut training = request("train_skill");
        training.skill_id = Some("unknown_skill".to_string());
        cases.push(training);
        let mut mentor = request("train_skill");
        mentor.skill_id = Some("basic_unarmed".to_string());
        mentor.npc_id = Some("not_the_matching_mentor".to_string());
        cases.push(mentor);
        let mut item = request("equip_item");
        item.item_id = Some("unknown_item".to_string());
        cases.push(item);
        let mut slot = request("equip_item");
        slot.item_id = Some("route-guard-staff".to_string());
        slot.target_slot = Some("not_a_catalog_slot".to_string());
        cases.push(slot);
        let mut attack = request("attack");
        attack.skill_id = Some("unknown_skill".to_string());
        cases.push(attack);
        for command in cases {
            for epoch in [1, 100, 10_000] {
                let mut character = WorldTrillionniumCharacter::default_for("local-player");
                character.title.clear();
                character.updated_at_epoch = 0;
                character.skill_ids.clear();
                let before = serde_json::to_vec(&character).unwrap();
                let outcome = apply_tactics_command(&mut character, command.clone(), epoch);
                assert!(!outcome.accepted, "{}", command.command);
                assert_eq!(serde_json::to_vec(&character).unwrap(), before);
                assert_eq!(serde_json::to_vec(&outcome.character).unwrap(), before);
            }
        }
    }

    #[test]
    fn repeated_rejections_do_not_normalize_or_advance_state() {
        let mut character = WorldTrillionniumCharacter::default_for("local-player");
        character.title.clear();
        character.updated_at_epoch = 0;
        let before = serde_json::to_vec(&character).unwrap();
        for _ in 0..16 {
            let outcome = apply_tactics_command(&mut character, request("unknown"), 100);
            assert!(!outcome.accepted);
            assert_eq!(serde_json::to_vec(&character).unwrap(), before);
            assert_eq!(serde_json::to_vec(&outcome.character).unwrap(), before);
        }
    }

    #[test]
    fn accepted_tactics_preserve_existing_engine_outputs() {
        for name in ["talk_npc", "offer_task", "complete_task", "train_skill"] {
            let mut expected = WorldTrillionniumCharacter::default_for("local-player");
            let mut actual = expected.clone();
            let expected_outcome = engine::apply_tactics_command(&mut expected, request(name), 10);
            let actual_outcome = apply_tactics_command(&mut actual, request(name), 10);
            assert!(expected_outcome.accepted, "{name}");
            assert_eq!(actual, expected);
            assert_eq!(actual_outcome, expected_outcome);
        }
    }

    #[test]
    fn rejected_world_commands_preserve_full_serialized_state() {
        let cases = [
            WorldCommand::Move {
                actor_id: "missing-actor".to_string(),
                direction: "east".to_string(),
            },
            WorldCommand::Move {
                actor_id: "local-player".to_string(),
                direction: "south".to_string(),
            },
            WorldCommand::TalkNpc {
                actor_id: "local-player".to_string(),
                npc_id: "missing-npc".to_string(),
            },
            WorldCommand::TrainSkill {
                actor_id: "local-player".to_string(),
                npc_id: "missing-npc".to_string(),
                skill_id: "unknown".to_string(),
            },
            WorldCommand::CompleteTask {
                actor_id: "local-player".to_string(),
                task_id: "missing-task".to_string(),
            },
        ];
        for command in cases {
            let mut state = WorldState::fixture();
            let before = serde_json::to_vec(&state).unwrap();
            assert!(!apply_command(&mut state, command).accepted);
            assert_eq!(serde_json::to_vec(&state).unwrap(), before);
        }
    }

    #[test]
    fn accepted_world_commands_preserve_existing_engine_outputs() {
        for direction in ["east", "6", "wait"] {
            let mut actual = WorldState::fixture();
            let mut expected = actual.clone();
            let command = WorldCommand::Move {
                actor_id: "local-player".to_string(),
                direction: direction.to_string(),
            };
            let expected_decision = engine::apply_command(&mut expected, command.clone());
            let actual_decision = apply_command(&mut actual, command);
            assert!(actual_decision.accepted);
            assert_eq!(actual, expected);
            assert_eq!(actual_decision, expected_decision);
        }
    }
}
