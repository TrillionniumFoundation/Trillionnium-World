#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-action-focus.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-action-focus >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_action_focus_v1"
  and .build_branch_title_route_action_dashboard_contract == "trillionnium_world_bevy_build_branch_title_route_action_dashboard_v1"
  and .green == true
  and .action_dashboard_contract_green == true
  and .equip_focus_gate == true
  and .route_focus_gate == true
  and .complete_focus_gate == true
  and .force_prereq_blocked_focus_gate == true
  and .done_no_focus_gate == true
  and .focus_samples.equip.contextual_primary_action_label == "TITLE:EQUIP:title-force-gate-warden"
  and (.focus_samples.equip.focus_buttons[] | select(
    .action_label == "TITLE:EQUIP:title-force-gate-warden"
    and .visual_state == "title_route_dashboard_focus"
    and .enabled == true
    and .primary == true
    and .source == "title_route_action_dashboard"
    and .reason == "enabled_build_title_equip:force:title-force-gate-warden"
  )) != null
  and (.focus_samples.equip.button_texts[] | select(
    .action_label == "TITLE:EQUIP:title-force-gate-warden"
    and .text == ">> GATE TITLE"
  )) != null
  and (.focus_samples.equip.character_detail_text | contains("TITLE ROUTE FOCUS | force:followup_ready reward force-mastery-signet action TITLE:EQUIP:title-force-gate-warden enabled true reason enabled_build_title_equip:force:title-force-gate-warden"))
  and .focus_samples.route.contextual_primary_action_label == "TITLE:ROUTE"
  and (.focus_samples.route.focus_buttons[] | select(
    .action_label == "TITLE:ROUTE"
    and .visual_state == "title_route_dashboard_focus"
    and .enabled == true
    and .primary == true
    and .source == "title_route_action_dashboard"
    and .reason == "enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"
  )) != null
  and (.focus_samples.route.button_texts[] | select(
    .action_label == "TITLE:ROUTE"
    and .text == ">> TITLE ROUTE"
  )) != null
  and (.focus_samples.route.character_detail_text | contains("TITLE ROUTE FOCUS | craft:followup_ready reward craft-mastery-signet action TITLE:ROUTE enabled true reason enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"))
  and .focus_samples.complete.contextual_primary_action_label == "COMPLETE"
  and (.focus_samples.complete.focus_buttons[] | select(
    .action_label == "COMPLETE"
    and .visual_state == "title_route_dashboard_focus"
    and .enabled == true
    and .primary == true
    and .source == "title_route_action_dashboard"
    and .reason == "enabled_build_branch_followup_completion:task-craft-forge-batch"
  )) != null
  and (.focus_samples.complete.button_texts[] | select(
    .action_label == "COMPLETE"
    and .text == ">> C TASK"
  )) != null
  and (.focus_samples.complete.character_detail_text | contains("TITLE ROUTE FOCUS | craft:followup_active reward craft-mastery-signet action COMPLETE enabled true reason enabled_build_branch_followup_completion:task-craft-forge-batch"))
  and (.focus_samples.force_prereq.focus_buttons[] | select(
    .action_label == "COMPLETE"
    and .visual_state == "title_route_dashboard_blocked"
    and .enabled == false
    and .primary == false
    and .source == "title_route_action_dashboard"
    and .reason == "build_mastery_force_victory_required:task-force-mastery-guard-trial"
  )) != null
  and (.focus_samples.force_prereq.button_texts[] | select(
    .action_label == "COMPLETE"
    and .text == "! C TASK"
  )) != null
  and (.focus_samples.force_prereq.character_detail_text | contains("TITLE ROUTE FOCUS | force:mastery_active reward force-mastery-signet action COMPLETE enabled false reason build_mastery_force_victory_required:task-force-mastery-guard-trial"))
  and (.focus_samples.done.focus_buttons | length) == 0
  and (.focus_samples.done.character_detail_text | contains("TITLE ROUTE FOCUS | none action DONE enabled false reason all_title_routes_complete"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ACTION_FOCUS_GREEN %s\n' "$SUMMARY"
