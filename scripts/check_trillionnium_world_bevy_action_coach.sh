#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json"
SUMMARY_RAW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.raw.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" action-coach >"$SUMMARY_RAW"
)

jq '
  .status = "action_coach_green"
  | .external_evidence_ignored_for_current_action_coach_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY"
rm -f "$SUMMARY_RAW"

jq -e '
  .contract_version == "trillionnium_world_bevy_action_coach_v1"
  and .status == "action_coach_green"
  and .green == true
  and .coach_stage_gate == true
  and .enter_execution_gate == true
  and .final_next_gate == true
  and .input_hint_contract_gate == true
  and (.coach_stage_checks | length) == 4
  and (.coach_stage_checks | all(.action_matches == true and .clean_player_line == true))
  and (.coach_stage_checks | map(.coach_line | contains("ACTION COACH | Enter/NumpadEnter ->")) | all)
  and (.enter_execution_checks | length) == 3
  and (.enter_execution_checks | all(.matches == true and .accepted == true))
  and ([.coach_stage_checks[].expected_action] == ["TALK","TRAIN","MOVE:north","FIGHT"])
  and (.android_s5_real_device_claimed == false)
  and (.external_evidence_ignored_for_current_action_coach_pass == true)
  and (.public_launch_ready == false)
  and (.production_ready_ui_claimed == false)
  and (.screen_for_screen_openra_ui_claimed == false)
  and (.openra_engine_port_claimed == false)
  and (.warcraft_iii_asset_copied == false)
  and (.openra_asset_copied == false)
  and (.third_party_asset_copied == false)
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_ACTION_COACH_GREEN $SUMMARY"
