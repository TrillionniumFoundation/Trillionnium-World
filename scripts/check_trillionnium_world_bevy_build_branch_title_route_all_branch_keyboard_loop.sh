#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-all-branch-keyboard-loop.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-all-branch-keyboard-loop >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_loop_v1"
  and .build_branch_title_route_keyboard_loop_contract == "trillionnium_world_bevy_build_branch_title_route_keyboard_loop_v1"
  and .green == true
  and .keyboard_loop_contract_green == true
  and .branch_count == 3
  and .all_branch_keyboard_loop_gate == true
  and (.completed_stat_ids | index("force") != null)
  and (.completed_stat_ids | index("agility") != null)
  and (.completed_stat_ids | index("craft") != null)
  and .branch_results.force.green == true
  and .branch_results.force.gates.equip_gate == true
  and .branch_results.force.gates.followup_route_keyboard_gate == true
  and .branch_results.force.gates.followup_completion_keyboard_gate == true
  and .branch_results.force.gates.mastery_route_keyboard_gate == true
  and .branch_results.force.gates.combat_prereq_keyboard_gate == true
  and .branch_results.force.gates.mastery_completion_keyboard_gate == true
  and .branch_results.force.gates.final_done_hint_gate == true
  and .branch_results.force.gates.final_runtime_gate == true
  and .branch_results.force.keyboard_steps.equip_title.event.key == "Enter"
  and .branch_results.force.keyboard_steps.equip_title.event.action == "TITLE:EQUIP:title-force-gate-warden"
  and (.branch_results.force.keyboard_steps.followup_route | length) >= 1
  and all(.branch_results.force.keyboard_steps.followup_route[]; .event.action == "TITLE:ROUTE" and .event.accepted == true)
  and all(.branch_results.force.keyboard_steps.mastery_route[]; .event.action == "TITLE:ROUTE" and .event.accepted == true)
  and (.branch_results.force.keyboard_steps.combat_prereq | length) >= 1
  and all(.branch_results.force.keyboard_steps.combat_prereq[]; .event.key == "KeyJ" and .event.action == "COMBAT:attack" and .event.accepted == true)
  and .branch_results.force.keyboard_steps.complete_mastery.event.action == "COMPLETE"
  and .branch_results.force.final_runtime.current_room_id == "league-coliseum"
  and .branch_results.force.final_runtime.active_build_title_id == "title-force-gate-warden"
  and .branch_results.force.final_runtime.route_director_task_id == "task-force-mastery-guard-trial"
  and (.branch_results.force.final_runtime.completed_task_ids | index("task-force-guard-duty") != null)
  and (.branch_results.force.final_runtime.completed_task_ids | index("task-force-mastery-guard-trial") != null)
  and (.branch_results.force.final_runtime.inventory_items | index("force-mastery-signet") != null)
  and .branch_results.force.final_runtime.objective_status == "build_mastery_challenge_completed:force:task-force-mastery-guard-trial"
  and .branch_results.force.final_runtime.combat_result_state == "victory"
  and .branch_results.agility.green == true
  and .branch_results.agility.gates.equip_gate == true
  and .branch_results.agility.gates.followup_route_keyboard_gate == true
  and .branch_results.agility.gates.followup_completion_keyboard_gate == true
  and .branch_results.agility.gates.mastery_route_keyboard_gate == true
  and .branch_results.agility.gates.combat_prereq_keyboard_gate == true
  and .branch_results.agility.gates.mastery_completion_keyboard_gate == true
  and .branch_results.agility.gates.final_done_hint_gate == true
  and .branch_results.agility.gates.final_runtime_gate == true
  and .branch_results.agility.keyboard_steps.equip_title.event.action == "TITLE:EQUIP:title-agility-relay-runner"
  and (.branch_results.agility.keyboard_steps.combat_prereq | length) == 0
  and .branch_results.agility.final_runtime.current_room_id == "mirror-city-square"
  and .branch_results.agility.final_runtime.active_build_title_id == "title-agility-relay-runner"
  and .branch_results.agility.final_runtime.route_director_task_id == "task-agility-mastery-shortcut-run"
  and (.branch_results.agility.final_runtime.completed_task_ids | index("task-agility-courier-relay") != null)
  and (.branch_results.agility.final_runtime.completed_task_ids | index("task-agility-mastery-shortcut-run") != null)
  and (.branch_results.agility.final_runtime.inventory_items | index("agility-mastery-signet") != null)
  and .branch_results.agility.final_runtime.objective_status == "build_mastery_challenge_completed:agility:task-agility-mastery-shortcut-run"
  and .branch_results.craft.green == true
  and .branch_results.craft.gates.equip_gate == true
  and .branch_results.craft.gates.followup_route_keyboard_gate == true
  and .branch_results.craft.gates.followup_completion_keyboard_gate == true
  and .branch_results.craft.gates.mastery_route_keyboard_gate == true
  and .branch_results.craft.gates.combat_prereq_keyboard_gate == true
  and .branch_results.craft.gates.mastery_completion_keyboard_gate == true
  and .branch_results.craft.gates.final_done_hint_gate == true
  and .branch_results.craft.gates.final_runtime_gate == true
  and .branch_results.craft.keyboard_steps.equip_title.event.action == "TITLE:EQUIP:title-craft-forge-master"
  and (.branch_results.craft.keyboard_steps.combat_prereq | length) == 0
  and .branch_results.craft.final_runtime.current_room_id == "forge-workbench"
  and .branch_results.craft.final_runtime.active_build_title_id == "title-craft-forge-master"
  and .branch_results.craft.final_runtime.route_director_task_id == "task-craft-mastery-client-order"
  and (.branch_results.craft.final_runtime.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.branch_results.craft.final_runtime.completed_task_ids | index("task-craft-mastery-client-order") != null)
  and (.branch_results.craft.final_runtime.inventory_items | index("craft-mastery-signet") != null)
  and .branch_results.craft.final_runtime.objective_status == "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ALL_BRANCH_KEYBOARD_LOOP_GREEN %s\n' "$SUMMARY"
