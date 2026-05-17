#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-all-branch-completion.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-all-branch-completion >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_all_branch_completion_v1"
  and .build_branch_title_route_mastery_completion_contract == "trillionnium_world_bevy_build_branch_title_route_mastery_completion_v1"
  and .build_branch_title_route_completion_handoff_contract == "trillionnium_world_bevy_build_branch_title_route_completion_handoff_v1"
  and .build_branch_mastery_challenges_contract == "trillionnium_world_bevy_build_branch_mastery_challenges_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .branch_count == 3
  and .all_branch_completion_gate == true
  and (.completed_stat_ids | index("force") != null)
  and (.completed_stat_ids | index("agility") != null)
  and (.completed_stat_ids | index("craft") != null)
  and .branch_results.force.green == true
  and .branch_results.force.gates.equip_gate == true
  and .branch_results.force.gates.followup_route_reaches_task_gate == true
  and .branch_results.force.gates.followup_completion_gate == true
  and .branch_results.force.gates.mastery_route_reaches_task_gate == true
  and .branch_results.force.gates.mastery_prereq_gate == true
  and .branch_results.force.gates.mastery_completion_gate == true
  and .branch_results.force.gates.save_load_gate == true
  and .branch_results.force.gates.restore_complete_gate == true
  and .branch_results.force.title_id == "title-force-gate-warden"
  and .branch_results.force.followup_task_id == "task-force-guard-duty"
  and .branch_results.force.mastery_task_id == "task-force-mastery-guard-trial"
  and .branch_results.force.reward_item_id == "force-mastery-signet"
  and .branch_results.force.title_route_complete_reason == "title_route_complete:force"
  and .branch_results.force.final_runtime.current_room_id == "league-coliseum"
  and .branch_results.force.final_runtime.route_director_task_id == "task-force-mastery-guard-trial"
  and (.branch_results.force.final_runtime.completed_task_ids | index("task-force-guard-duty") != null)
  and (.branch_results.force.final_runtime.completed_task_ids | index("task-force-mastery-guard-trial") != null)
  and (.branch_results.force.final_runtime.active_task_ids | index("task-force-mastery-guard-trial") == null)
  and (.branch_results.force.final_runtime.inventory_items | index("force-mastery-signet") != null)
  and .branch_results.force.final_runtime.objective_status == "build_mastery_challenge_completed:force:task-force-mastery-guard-trial"
  and .branch_results.force.route_samples.after_continue.title_route_button.enabled == false
  and .branch_results.force.route_samples.after_continue.title_route_button.reason == "title_route_complete:force"
  and (.branch_results.force.route_samples.after_continue.quest_panel_text | contains("TITLE ROUTE | active Gate Warden complete force via arena_gate_reputation_anchor"))
  and .branch_results.force.slot_snapshot_after_save.current_room_id == "league-coliseum"
  and .branch_results.agility.green == true
  and .branch_results.agility.gates.equip_gate == true
  and .branch_results.agility.gates.followup_route_reaches_task_gate == true
  and .branch_results.agility.gates.followup_completion_gate == true
  and .branch_results.agility.gates.mastery_route_reaches_task_gate == true
  and .branch_results.agility.gates.mastery_prereq_gate == true
  and .branch_results.agility.gates.mastery_completion_gate == true
  and .branch_results.agility.gates.save_load_gate == true
  and .branch_results.agility.gates.restore_complete_gate == true
  and .branch_results.agility.title_id == "title-agility-relay-runner"
  and .branch_results.agility.followup_task_id == "task-agility-courier-relay"
  and .branch_results.agility.mastery_task_id == "task-agility-mastery-shortcut-run"
  and .branch_results.agility.reward_item_id == "agility-mastery-signet"
  and .branch_results.agility.title_route_complete_reason == "title_route_complete:agility"
  and .branch_results.agility.final_runtime.current_room_id == "mirror-city-square"
  and .branch_results.agility.final_runtime.route_director_task_id == "task-agility-mastery-shortcut-run"
  and (.branch_results.agility.final_runtime.completed_task_ids | index("task-agility-courier-relay") != null)
  and (.branch_results.agility.final_runtime.completed_task_ids | index("task-agility-mastery-shortcut-run") != null)
  and (.branch_results.agility.final_runtime.active_task_ids | index("task-agility-mastery-shortcut-run") == null)
  and (.branch_results.agility.final_runtime.inventory_items | index("agility-mastery-signet") != null)
  and .branch_results.agility.final_runtime.objective_status == "build_mastery_challenge_completed:agility:task-agility-mastery-shortcut-run"
  and .branch_results.agility.route_samples.after_continue.title_route_button.enabled == false
  and .branch_results.agility.route_samples.after_continue.title_route_button.reason == "title_route_complete:agility"
  and (.branch_results.agility.route_samples.after_continue.quest_panel_text | contains("TITLE ROUTE | active Relay Runner complete agility via relay_route_priority_anchor"))
  and .branch_results.agility.slot_snapshot_after_save.current_room_id == "mirror-city-square"
  and .branch_results.craft.green == true
  and .branch_results.craft.gates.equip_gate == true
  and .branch_results.craft.gates.followup_route_reaches_task_gate == true
  and .branch_results.craft.gates.followup_completion_gate == true
  and .branch_results.craft.gates.mastery_route_reaches_task_gate == true
  and .branch_results.craft.gates.mastery_prereq_gate == true
  and .branch_results.craft.gates.mastery_completion_gate == true
  and .branch_results.craft.gates.save_load_gate == true
  and .branch_results.craft.gates.restore_complete_gate == true
  and .branch_results.craft.title_id == "title-craft-forge-master"
  and .branch_results.craft.followup_task_id == "task-craft-forge-batch"
  and .branch_results.craft.mastery_task_id == "task-craft-mastery-client-order"
  and .branch_results.craft.reward_item_id == "craft-mastery-signet"
  and .branch_results.craft.title_route_complete_reason == "title_route_complete:craft"
  and .branch_results.craft.final_runtime.current_room_id == "forge-workbench"
  and .branch_results.craft.final_runtime.route_director_task_id == "task-craft-mastery-client-order"
  and (.branch_results.craft.final_runtime.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.branch_results.craft.final_runtime.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.branch_results.craft.final_runtime.active_task_ids | index("task-craft-mastery-client-order") == null)
  and (.branch_results.craft.final_runtime.inventory_items | index("craft-mastery-signet") != null)
  and .branch_results.craft.final_runtime.objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and .branch_results.craft.route_samples.after_continue.title_route_button.enabled == false
  and .branch_results.craft.route_samples.after_continue.title_route_button.reason == "title_route_complete:craft"
  and (.branch_results.craft.route_samples.after_continue.quest_panel_text | contains("TITLE ROUTE | active Forge Master complete craft via forge_client_trust_anchor"))
  and .branch_results.craft.slot_snapshot_after_save.current_room_id == "forge-workbench"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ALL_BRANCH_COMPLETION_GREEN %s\n' "$SUMMARY"
