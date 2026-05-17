#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-action-focus-input.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-action-focus-input >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_action_focus_input_v1"
  and .build_branch_title_route_action_focus_contract == "trillionnium_world_bevy_build_branch_title_route_action_focus_v1"
  and .green == true
  and .action_focus_contract_green == true
  and .equip_enter_focus_gate == true
  and .route_enter_focus_gate == true
  and .numpad_enter_focus_gate == true
  and .complete_enter_focus_gate == true
  and .blocked_enter_focus_gate == true
  and .input_samples.equip_enter.event.key == "Enter"
  and .input_samples.equip_enter.event.action == "TITLE:EQUIP:title-force-gate-warden"
  and .input_samples.equip_enter.event.accepted == true
  and .input_samples.equip_enter.event.availability_before == "enabled_build_title_equip:force:title-force-gate-warden"
  and .input_samples.equip_enter.last_input_feedback.input_source == "keyboard"
  and .input_samples.equip_enter.last_input_feedback.action_label == "TITLE:EQUIP:title-force-gate-warden"
  and .input_samples.equip_enter.runtime.active_build_title_id == "title-force-gate-warden"
  and (.input_samples.equip_enter.before_focus_buttons[] | select(
    .action_label == "TITLE:EQUIP:title-force-gate-warden"
    and .visual_state == "title_route_dashboard_focus"
    and .source == "title_route_action_dashboard"
  )) != null
  and .input_samples.route_enter.event.key == "Enter"
  and .input_samples.route_enter.event.action == "TITLE:ROUTE"
  and .input_samples.route_enter.event.accepted == true
  and .input_samples.route_enter.event.availability_before == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  and .input_samples.route_enter.last_input_feedback.input_source == "keyboard"
  and .input_samples.route_enter.last_input_feedback.action_label == "TITLE:ROUTE"
  and .input_samples.route_enter.runtime.current_room_id == "starter-studio"
  and (.input_samples.route_enter.before_focus_buttons[] | select(
    .action_label == "TITLE:ROUTE"
    and .visual_state == "title_route_dashboard_focus"
    and .source == "title_route_action_dashboard"
  )) != null
  and .input_samples.numpad_route_enter.event.key == "NumpadEnter"
  and .input_samples.numpad_route_enter.event.action == "TITLE:ROUTE"
  and .input_samples.numpad_route_enter.event.accepted == true
  and .input_samples.numpad_route_enter.runtime.current_room_id == "starter-studio"
  and .input_samples.numpad_route_enter.last_input_feedback.action_label == "TITLE:ROUTE"
  and .input_samples.complete_enter.event.key == "Enter"
  and .input_samples.complete_enter.event.action == "COMPLETE"
  and .input_samples.complete_enter.event.accepted == true
  and .input_samples.complete_enter.event.availability_before == "enabled_build_branch_followup_completion:task-craft-forge-batch"
  and .input_samples.complete_enter.last_input_feedback.action_label == "COMPLETE"
  and (.input_samples.complete_enter.runtime.completed_task_ids | index("task-craft-forge-batch") != null)
  and (.input_samples.complete_enter.before_focus_buttons[] | select(
    .action_label == "COMPLETE"
    and .visual_state == "title_route_dashboard_focus"
    and .source == "title_route_action_dashboard"
  )) != null
  and .input_samples.blocked_enter.event.key == "Enter"
  and .input_samples.blocked_enter.event.action == "COMPLETE"
  and .input_samples.blocked_enter.event.accepted == false
  and .input_samples.blocked_enter.event.availability_before == "build_mastery_force_victory_required:task-force-mastery-guard-trial"
  and .input_samples.blocked_enter.last_input_feedback.input_source == "keyboard"
  and .input_samples.blocked_enter.last_input_feedback.action_label == "COMPLETE"
  and .input_samples.blocked_enter.last_input_feedback.accepted == false
  and .input_samples.blocked_enter.last_input_feedback.reason == "build_mastery_force_victory_required:task-force-mastery-guard-trial"
  and (.input_samples.blocked_enter.runtime.completed_task_ids | index("task-force-mastery-guard-trial") == null)
  and (.input_samples.blocked_enter.before_focus_buttons[] | select(
    .action_label == "COMPLETE"
    and .visual_state == "title_route_dashboard_blocked"
    and .source == "title_route_action_dashboard"
  )) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ACTION_FOCUS_INPUT_GREEN %s\n' "$SUMMARY"
