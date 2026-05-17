#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-mastery-challenges.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-mastery-challenges >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_mastery_challenges_v1"
  and .build_branch_mastery_contract == "trillionnium_world_bevy_build_branch_mastery_v1"
  and .build_branch_followup_completion_contract == "trillionnium_world_bevy_build_branch_followup_completion_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .challenge_unlocked_after_mastery_gate == true
  and .challenge_completion_persisted_gate == true
  and .force_challenge_gate == true
  and .agility_challenge_gate == true
  and .craft_challenge_gate == true
  and .challenge_isolation_gate == true
  and .challenge_task_ids.force == "task-force-mastery-guard-trial"
  and .challenge_task_ids.agility == "task-agility-mastery-shortcut-run"
  and .challenge_task_ids.craft == "task-craft-mastery-client-order"
  and .reward_items.force == "force-mastery-signet"
  and .reward_items.agility == "agility-mastery-signet"
  and .reward_items.craft == "craft-mastery-signet"
  and (.slot_bytes.force > 0)
  and (.slot_bytes.agility > 0)
  and (.slot_bytes.craft > 0)
  and (.quest_texts_after_mastery_restore.force | contains("force:task-force-mastery-guard-trial:unlocked_after_mastery_restore"))
  and (.quest_texts_after_mastery_restore.agility | contains("agility:task-agility-mastery-shortcut-run:unlocked_after_mastery_restore"))
  and (.quest_texts_after_mastery_restore.craft | contains("craft:task-craft-mastery-client-order:unlocked_after_mastery_restore"))
  and (.slot_snapshots_after_challenge.force.completed_task_ids | index("task-force-mastery-guard-trial") != null)
  and (.slot_snapshots_after_challenge.force.active_task_ids | index("task-force-mastery-guard-trial") == null)
  and (.slot_snapshots_after_challenge.force.inventory_items | index("force-mastery-signet") != null)
  and .slot_snapshots_after_challenge.force.objective_status == "build_mastery_challenge_completed:force:task-force-mastery-guard-trial"
  and .slot_snapshots_after_challenge.force.map_scene == "force_mastery_guard_trial_cleared"
  and (.runtime_summaries_after_challenge.force.combat_round_log | map(contains("incoming:4")) | any)
  and (.visible_texts_after_challenge_completion.force_npc | contains("Guard trial cleared"))
  and (.visible_texts_after_challenge_completion.force_enemy | contains("mastery force complete"))
  and (.slot_snapshots_after_challenge.agility.completed_task_ids | index("task-agility-mastery-shortcut-run") != null)
  and (.slot_snapshots_after_challenge.agility.active_task_ids | index("task-agility-mastery-shortcut-run") == null)
  and (.slot_snapshots_after_challenge.agility.inventory_items | index("agility-mastery-signet") != null)
  and .slot_snapshots_after_challenge.agility.objective_status == "build_mastery_challenge_completed:agility:task-agility-mastery-shortcut-run"
  and .runtime_summaries_after_challenge.agility.current_room_id == "mirror-city-square"
  and .runtime_summaries_after_challenge.agility.map_scene == "agility_mastery_shortcut_run_cleared"
  and (.runtime_summaries_after_challenge.agility.route_director_history | map(contains("delivery-dock->mirror-city-square")) | any)
  and (.visible_texts_after_challenge_completion.agility_npc | contains("Shortcut run signed"))
  and (.slot_snapshots_after_challenge.craft.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.slot_snapshots_after_challenge.craft.active_task_ids | index("task-craft-mastery-client-order") == null)
  and (.slot_snapshots_after_challenge.craft.inventory_items | index("craft-mastery-signet") != null)
  and .slot_snapshots_after_challenge.craft.objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and .slot_snapshots_after_challenge.craft.map_scene == "craft_mastery_client_order_cleared"
  and (.runtime_summaries_after_challenge.craft.reward_settlement_text | contains("CRAFT MASTERY +3 coins"))
  and (.visible_texts_after_challenge_completion.craft_npc | contains("Master client order paid"))
  and (.runtime_summaries_after_challenge.force.completed_task_ids | index("task-agility-mastery-shortcut-run") == null)
  and (.runtime_summaries_after_challenge.agility.completed_task_ids | index("task-craft-mastery-client-order") == null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_MASTERY_CHALLENGES_GREEN %s\n' "$SUMMARY"
