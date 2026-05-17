#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-completion-handoff.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-completion-handoff >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_completion_handoff_v1"
  and .build_branch_title_route_accept_contract == "trillionnium_world_bevy_build_branch_title_route_accept_v1"
  and .build_branch_followup_completion_contract == "trillionnium_world_bevy_build_branch_followup_completion_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .title_route_accept_prereq_gate == true
  and .title_route_followup_completion_gate == true
  and .title_route_mastery_handoff_gate == true
  and .save_load_mastery_handoff_gate == true
  and .restore_title_route_handoff_gate == true
  and .button_events.equip_craft_title.availability_before == "enabled_build_title_equip:craft:title-craft-forge-master"
  and .button_events.accept_step_one.action == "TITLE:ROUTE"
  and .button_events.accept_step_one.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .button_events.accept_step_two.action == "TITLE:ROUTE"
  and .button_events.accept_step_two.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:forge-workbench"
  and .button_events.accept_followup_start.action == "TITLE:ROUTE"
  and .button_events.accept_followup_start.availability_before == "enabled_title_route_start:craft:task-craft-forge-batch"
  and .button_events.complete_followup.action == "COMPLETE"
  and .button_events.complete_followup.availability_before == "enabled_build_branch_followup_completion:task-craft-forge-batch"
  and .button_events.accept_mastery_start.action == "TITLE:ROUTE"
  and .button_events.accept_mastery_start.availability_before == "enabled_title_route_start:craft:task-craft-mastery-client-order"
  and .route_completion_states.after_followup_complete.current_node_id == "forge-workbench"
  and .route_completion_states.after_followup_complete.objective_status == "build_branch_followup_completed:craft:task-craft-forge-batch"
  and (.route_completion_states.after_followup_complete.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.route_completion_states.after_followup_complete.inventory_items | index("craft-batch-seal") != null)
  and .route_completion_states.after_followup_complete.route_director_task_id == "task-craft-mastery-client-order"
  and (.route_completion_states.after_followup_complete.quest_panel_text | contains("TITLE ROUTE | active Forge Master recommends craft:task-craft-mastery-client-order via forge_client_trust_anchor"))
  and .route_completion_states.after_followup_complete.title_route_button.enabled == true
  and .route_completion_states.after_followup_complete.title_route_button.reason == "enabled_title_route_start:craft:task-craft-mastery-client-order"
  and .route_completion_states.after_mastery_start.objective_status == "build_mastery_challenge_active:craft:task-craft-mastery-client-order"
  and (.route_completion_states.after_mastery_start.active_task_ids | index("task-craft-mastery-client-order") != null)
  and (.route_completion_states.after_mastery_start.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.route_completion_states.after_mastery_start.route_director_history | index("active_title_route_recommendation:craft:task-craft-mastery-client-order:forge_client_trust_anchor") != null)
  and (.route_completion_states.after_mastery_start.route_director_history | index("active_title_route_accept:craft:task-craft-mastery-client-order:start_task") != null)
  and .route_completion_states.after_mastery_start.title_route_button.enabled == false
  and .route_completion_states.after_mastery_start.title_route_button.reason == "title_route_start_blocked:task_already_active:task-craft-mastery-client-order"
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and .slot_snapshot_after_completion_handoff_save.present == true
  and .slot_snapshot_after_completion_handoff_save.current_node_id == "forge-workbench"
  and .slot_snapshot_after_completion_handoff_save.active_build_title_id == "title-craft-forge-master"
  and .slot_snapshot_after_completion_handoff_save.route_director_task_id == "task-craft-mastery-client-order"
  and (.slot_snapshot_after_completion_handoff_save.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.slot_snapshot_after_completion_handoff_save.active_task_ids | index("task-craft-mastery-client-order") != null)
  and .final_runtime.current_room_id == "forge-workbench"
  and .final_runtime.active_build_title_id == "title-craft-forge-master"
  and .final_runtime.route_director_task_id == "task-craft-mastery-client-order"
  and (.final_runtime.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.final_runtime.active_task_ids | index("task-craft-mastery-client-order") != null)
  and .final_runtime.objective_status == "build_mastery_challenge_active:craft:task-craft-mastery-client-order"
  and .route_completion_states.after_continue.title_route_button.enabled == false
  and .route_completion_states.after_continue.title_route_button.reason == "title_route_start_blocked:task_already_active:task-craft-mastery-client-order"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_COMPLETION_HANDOFF_GREEN %s\n' "$SUMMARY"
