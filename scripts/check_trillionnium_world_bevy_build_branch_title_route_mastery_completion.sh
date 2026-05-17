#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-mastery-completion.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-mastery-completion >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_mastery_completion_v1"
  and .build_branch_title_route_completion_handoff_contract == "trillionnium_world_bevy_build_branch_title_route_completion_handoff_v1"
  and .build_branch_mastery_challenges_contract == "trillionnium_world_bevy_build_branch_mastery_challenges_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .title_route_mastery_prereq_gate == true
  and .mastery_completion_gate == true
  and .save_load_mastery_completion_gate == true
  and .restore_title_route_complete_gate == true
  and .route_complete_reason == "title_route_complete:craft"
  and .button_events.accept_step_one.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .button_events.accept_step_two.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:forge-workbench"
  and .button_events.accept_followup_start.availability_before == "enabled_title_route_start:craft:task-craft-forge-batch"
  and .button_events.complete_followup.availability_before == "enabled_build_branch_followup_completion:task-craft-forge-batch"
  and .button_events.accept_mastery_start.availability_before == "enabled_title_route_start:craft:task-craft-mastery-client-order"
  and .button_events.complete_mastery.action == "COMPLETE"
  and .button_events.complete_mastery.accepted == true
  and .button_events.complete_mastery.availability_before == "enabled_build_mastery_challenge_completion:task-craft-mastery-client-order"
  and .route_mastery_states.after_mastery_complete.current_node_id == "forge-workbench"
  and .route_mastery_states.after_mastery_complete.objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and (.route_mastery_states.after_mastery_complete.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.route_mastery_states.after_mastery_complete.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.route_mastery_states.after_mastery_complete.active_task_ids | index("task-craft-mastery-client-order") == null)
  and (.route_mastery_states.after_mastery_complete.inventory_items | index("craft-mastery-signet") != null)
  and (.route_mastery_states.after_mastery_complete.loot_history | index("build_mastery_challenge_reward:task-craft-mastery-client-order:craft-mastery-signet") != null)
  and (.route_mastery_states.after_mastery_complete.quest_panel_text | contains("TITLE ROUTE | active Forge Master complete craft via forge_client_trust_anchor"))
  and (.route_mastery_states.after_mastery_complete.reward_settlement_text | contains("CRAFT MASTERY CHALLENGE | master client order cleared"))
  and .route_mastery_states.after_mastery_complete.title_route_button.enabled == false
  and .route_mastery_states.after_mastery_complete.title_route_button.reason == "title_route_complete:craft"
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and .slot_snapshot_after_mastery_completion_save.present == true
  and .slot_snapshot_after_mastery_completion_save.current_node_id == "forge-workbench"
  and .slot_snapshot_after_mastery_completion_save.active_build_title_id == "title-craft-forge-master"
  and .slot_snapshot_after_mastery_completion_save.route_director_task_id == "task-craft-mastery-client-order"
  and (.slot_snapshot_after_mastery_completion_save.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.slot_snapshot_after_mastery_completion_save.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.slot_snapshot_after_mastery_completion_save.active_task_ids | index("task-craft-mastery-client-order") == null)
  and (.slot_snapshot_after_mastery_completion_save.inventory_items | index("craft-mastery-signet") != null)
  and .final_runtime.current_room_id == "forge-workbench"
  and .final_runtime.active_build_title_id == "title-craft-forge-master"
  and .final_runtime.route_director_task_id == "task-craft-mastery-client-order"
  and (.final_runtime.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.final_runtime.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.final_runtime.active_task_ids | index("task-craft-mastery-client-order") == null)
  and (.final_runtime.inventory_items | index("craft-mastery-signet") != null)
  and .final_runtime.objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and .route_mastery_states.after_continue.title_route_button.enabled == false
  and .route_mastery_states.after_continue.title_route_button.reason == "title_route_complete:craft"
  and (.route_mastery_states.after_continue.quest_panel_text | contains("TITLE ROUTE | active Forge Master complete craft via forge_client_trust_anchor"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_MASTERY_COMPLETION_GREEN %s\n' "$SUMMARY"
