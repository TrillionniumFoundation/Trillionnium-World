#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-accept.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-accept >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_accept_v1"
  and .build_branch_title_route_recommendation_contract == "trillionnium_world_bevy_build_branch_title_route_recommendation_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .title_route_seed_gate == true
  and .title_route_button_gate == true
  and .title_route_step_one_gate == true
  and .title_route_step_two_gate == true
  and .title_route_start_gate == true
  and .save_load_route_accept_gate == true
  and .restore_route_accept_gate == true
  and .button_events.equip_craft_title.accepted == true
  and .button_events.equip_craft_title.availability_before == "enabled_build_title_equip:craft:title-craft-forge-master"
  and .button_events.accept_step_one.action == "TITLE:ROUTE"
  and .button_events.accept_step_one.accepted == true
  and .button_events.accept_step_one.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .button_events.accept_step_two.action == "TITLE:ROUTE"
  and .button_events.accept_step_two.accepted == true
  and .button_events.accept_step_two.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:forge-workbench"
  and .button_events.accept_start_task.action == "TITLE:ROUTE"
  and .button_events.accept_start_task.accepted == true
  and .button_events.accept_start_task.availability_before == "enabled_title_route_start:craft:task-craft-forge-batch"
  and .route_accept_states.after_craft_equipped.title_route_button.enabled == true
  and .route_accept_states.after_craft_equipped.title_route_button.reason == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .route_accept_states.after_step_one.current_node_id == "starter-studio"
  and .route_accept_states.after_step_one.current_room_id == "starter-studio"
  and .route_accept_states.after_step_one.route_director_next_room_id == "forge-workbench"
  and (.route_accept_states.after_step_one.route_director_history | index("active_title_route_accept:craft:task-craft-forge-batch:move:starter-studio") != null)
  and .route_accept_states.after_step_two.current_node_id == "forge-workbench"
  and .route_accept_states.after_step_two.current_room_id == "forge-workbench"
  and .route_accept_states.after_step_two.title_route_button.reason == "enabled_title_route_start:craft:task-craft-forge-batch"
  and (.route_accept_states.after_step_two.route_director_history | index("active_title_route_accept:craft:task-craft-forge-batch:move:forge-workbench") != null)
  and .route_accept_states.after_start_task.current_node_id == "forge-workbench"
  and (.route_accept_states.after_start_task.active_task_ids | index("task-craft-forge-batch") != null)
  and .route_accept_states.after_start_task.objective_status == "build_branch_followup_active:craft:task-craft-forge-batch"
  and .route_accept_states.after_start_task.title_route_button.enabled == false
  and .route_accept_states.after_start_task.title_route_button.reason == "title_route_start_blocked:task_already_active:task-craft-forge-batch"
  and (.route_accept_states.after_start_task.route_director_history | index("active_title_route_accept:craft:task-craft-forge-batch:start_task") != null)
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and .slot_snapshot_after_route_accept_save.present == true
  and .slot_snapshot_after_route_accept_save.contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .slot_snapshot_after_route_accept_save.current_node_id == "forge-workbench"
  and .slot_snapshot_after_route_accept_save.active_build_title_id == "title-craft-forge-master"
  and (.slot_snapshot_after_route_accept_save.active_task_ids | index("task-craft-forge-batch") != null)
  and .final_runtime.current_room_id == "forge-workbench"
  and .final_runtime.active_build_title_id == "title-craft-forge-master"
  and .final_runtime.route_director_task_id == "task-craft-forge-batch"
  and (.final_runtime.active_task_ids | index("task-craft-forge-batch") != null)
  and .final_runtime.objective_status == "build_branch_followup_active:craft:task-craft-forge-batch"
  and .route_accept_states.after_continue.title_route_button.enabled == false
  and .route_accept_states.after_continue.title_route_button.reason == "title_route_start_blocked:task_already_active:task-craft-forge-batch"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ACCEPT_GREEN %s\n' "$SUMMARY"
