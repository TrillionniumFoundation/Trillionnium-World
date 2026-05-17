#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-keyboard-loop.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-keyboard-loop >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_keyboard_loop_v1"
  and .build_branch_title_route_action_hint_contract == "trillionnium_world_bevy_build_branch_title_route_action_hint_v1"
  and .green == true
  and .action_hint_contract_green == true
  and .equip_keyboard_gate == true
  and .route_step_keyboard_gate == true
  and .followup_keyboard_gate == true
  and .mastery_keyboard_gate == true
  and .final_done_hint_gate == true
  and .final_runtime_gate == true
  and .keyboard_steps.equip_craft.event.key == "Enter"
  and .keyboard_steps.equip_craft.event.action == "TITLE:EQUIP:title-craft-forge-master"
  and .keyboard_steps.equip_craft.event.accepted == true
  and .keyboard_steps.equip_craft.event.availability_before == "enabled_build_title_equip:craft:title-craft-forge-master"
  and (.keyboard_steps.equip_craft.before.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> TITLE:EQUIP:title-craft-forge-master [enabled_build_title_equip:craft:title-craft-forge-master]"))
  and .keyboard_steps.equip_craft.after.active_build_title_id == "title-craft-forge-master"
  and .keyboard_steps.route_to_starter.event.key == "Enter"
  and .keyboard_steps.route_to_starter.event.action == "TITLE:ROUTE"
  and .keyboard_steps.route_to_starter.event.accepted == true
  and .keyboard_steps.route_to_starter.event.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .keyboard_steps.route_to_starter.after.current_room_id == "starter-studio"
  and .keyboard_steps.route_to_forge.event.key == "NumpadEnter"
  and .keyboard_steps.route_to_forge.event.action == "TITLE:ROUTE"
  and .keyboard_steps.route_to_forge.event.accepted == true
  and .keyboard_steps.route_to_forge.event.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:forge-workbench"
  and .keyboard_steps.route_to_forge.after.current_room_id == "forge-workbench"
  and .keyboard_steps.start_followup.event.action == "TITLE:ROUTE"
  and .keyboard_steps.start_followup.event.availability_before == "enabled_title_route_start:craft:task-craft-forge-batch"
  and (.keyboard_steps.start_followup.after.active_task_ids | index("task-craft-forge-batch") != null)
  and .keyboard_steps.complete_followup.event.action == "COMPLETE"
  and .keyboard_steps.complete_followup.event.availability_before == "enabled_build_branch_followup_completion:task-craft-forge-batch"
  and (.keyboard_steps.complete_followup.after.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.keyboard_steps.complete_followup.after.inventory_items | index("craft-batch-seal") != null)
  and .keyboard_steps.start_mastery.event.action == "TITLE:ROUTE"
  and .keyboard_steps.start_mastery.event.availability_before == "enabled_title_route_start:craft:task-craft-mastery-client-order"
  and (.keyboard_steps.start_mastery.after.active_task_ids | index("task-craft-mastery-client-order") != null)
  and .keyboard_steps.complete_mastery.event.action == "COMPLETE"
  and .keyboard_steps.complete_mastery.event.availability_before == "enabled_build_mastery_challenge_completion:task-craft-mastery-client-order"
  and (.keyboard_steps.complete_mastery.after.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.keyboard_steps.complete_mastery.after.inventory_items | index("craft-mastery-signet") != null)
  and (.keyboard_steps.complete_mastery.after.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> none [all_title_routes_complete]"))
  and .final_runtime.current_room_id == "forge-workbench"
  and .final_runtime.active_build_title_id == "title-craft-forge-master"
  and .final_runtime.route_director_task_id == "task-craft-mastery-client-order"
  and (.final_runtime.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.final_runtime.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.final_runtime.active_task_ids | index("task-craft-mastery-client-order") == null)
  and (.final_runtime.inventory_items | index("craft-mastery-signet") != null)
  and .final_runtime.objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_KEYBOARD_LOOP_GREEN %s\n' "$SUMMARY"
