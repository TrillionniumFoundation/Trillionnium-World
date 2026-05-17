#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-route-action-hint.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-route-action-hint >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_route_action_hint_v1"
  and .build_branch_title_route_action_focus_input_contract == "trillionnium_world_bevy_build_branch_title_route_action_focus_input_v1"
  and .green == true
  and .action_focus_input_contract_green == true
  and .equip_hint_gate == true
  and .route_hint_gate == true
  and .complete_hint_gate == true
  and .blocked_hint_gate == true
  and .done_hint_gate == true
  and (.hint_samples.equip.character_detail_text | contains("TITLE ROUTE CONFIRM | key Enter/NumpadEnter action TITLE:EQUIP:title-force-gate-warden enabled true reason enabled_build_title_equip:force:title-force-gate-warden"))
  and (.hint_samples.equip.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> TITLE:EQUIP:title-force-gate-warden [enabled_build_title_equip:force:title-force-gate-warden]"))
  and (.hint_samples.route.character_detail_text | contains("TITLE ROUTE CONFIRM | key Enter/NumpadEnter action TITLE:ROUTE enabled true reason enabled_title_route_step:craft:task-craft-forge-batch:starter-studio"))
  and (.hint_samples.route.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> TITLE:ROUTE [enabled_title_route_step:craft:task-craft-forge-batch:starter-studio]"))
  and (.hint_samples.complete.character_detail_text | contains("TITLE ROUTE CONFIRM | key Enter/NumpadEnter action COMPLETE enabled true reason enabled_build_branch_followup_completion:task-craft-forge-batch"))
  and (.hint_samples.complete.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> COMPLETE [enabled_build_branch_followup_completion:task-craft-forge-batch]"))
  and (.hint_samples.blocked.character_detail_text | contains("TITLE ROUTE CONFIRM | key Enter/NumpadEnter action COMPLETE enabled false reason build_mastery_force_victory_required:task-force-mastery-guard-trial"))
  and (.hint_samples.blocked.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> COMPLETE [build_mastery_force_victory_required:task-force-mastery-guard-trial]"))
  and (.hint_samples.done.character_detail_text | contains("TITLE ROUTE CONFIRM | key Enter/NumpadEnter action DONE enabled false reason all_title_routes_complete"))
  and (.hint_samples.done.input_hint_text | contains("TITLE ROUTE CONFIRM: Enter/NumpadEnter -> none [all_title_routes_complete]"))
  and (.hint_samples.equip.focus_buttons[] | select(.action_label == "TITLE:EQUIP:title-force-gate-warden" and .visual_state == "title_route_dashboard_focus")) != null
  and (.hint_samples.blocked.focus_buttons[] | select(.action_label == "COMPLETE" and .visual_state == "title_route_dashboard_blocked")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_ROUTE_ACTION_HINT_GREEN %s\n' "$SUMMARY"
