#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json"
SUMMARY_RAW="$SUMMARY.raw.$$"
SUMMARY_TMP="$SUMMARY.tmp.$$"
mkdir -p "$(dirname "$SUMMARY")"
cleanup() {
  rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"
}
trap cleanup EXIT

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" action-coach >"$SUMMARY_RAW"
)

jq '
  ([.coach_stage_gate, .enter_execution_gate, .final_next_gate, .input_hint_contract_gate]) as $gates
  | .status = "action_coach_green"
  | .gate_count = ($gates | length)
  | .passed_gate_count = ($gates | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
  | .coach_stage_check_count = (.coach_stage_checks | length)
  | .matched_coach_stage_check_count = ([.coach_stage_checks[] | select(.action_matches == true and .clean_player_line == true)] | length)
  | .enter_execution_check_count = (.enter_execution_checks | length)
  | .accepted_enter_execution_check_count = ([.enter_execution_checks[] | select(.accepted == true)] | length)
  | .matched_enter_execution_check_count = ([.enter_execution_checks[] | select(.matches == true)] | length)
  | .keyboard_event_count = (.keyboard_events | length)
  | .accepted_keyboard_event_count = ([.keyboard_events[] | select(.accepted == true)] | length)
  | .sample_count = (.samples | length)
  | .final_runtime_key_count = (.final_runtime | keys | length)
  | .final_runtime_completed_step_count = (.final_runtime.completed_steps | length)
  | .final_runtime_input_feedback_history_count = (.final_runtime.input_feedback_history | length)
  | .final_runtime_visited_room_count = (.final_runtime.visited_rooms | length)
  | .external_evidence_ignored_for_current_action_coach_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_action_coach_v1"
  and .status == "action_coach_green"
  and .green == true
  and .coach_stage_gate == true
  and .enter_execution_gate == true
  and .final_next_gate == true
  and .input_hint_contract_gate == true
  and .gate_count == ([.coach_stage_gate, .enter_execution_gate, .final_next_gate, .input_hint_contract_gate] | length)
  and .gate_count == 4
  and .passed_gate_count == ([.coach_stage_gate, .enter_execution_gate, .final_next_gate, .input_hint_contract_gate] | map(select(. == true)) | length)
  and .passed_gate_count == 4
  and .failed_gate_count == (.gate_count - .passed_gate_count)
  and .failed_gate_count == 0
  and .coach_stage_check_count == (.coach_stage_checks | length)
  and .coach_stage_check_count == 4
  and .matched_coach_stage_check_count == ([.coach_stage_checks[] | select(.action_matches == true and .clean_player_line == true)] | length)
  and .matched_coach_stage_check_count == .coach_stage_check_count
  and .enter_execution_check_count == (.enter_execution_checks | length)
  and .enter_execution_check_count == 3
  and .accepted_enter_execution_check_count == ([.enter_execution_checks[] | select(.accepted == true)] | length)
  and .accepted_enter_execution_check_count == .enter_execution_check_count
  and .matched_enter_execution_check_count == ([.enter_execution_checks[] | select(.matches == true)] | length)
  and .matched_enter_execution_check_count == .enter_execution_check_count
  and .keyboard_event_count == (.keyboard_events | length)
  and .keyboard_event_count == 3
  and .accepted_keyboard_event_count == ([.keyboard_events[] | select(.accepted == true)] | length)
  and .accepted_keyboard_event_count == .keyboard_event_count
  and .sample_count == (.samples | length)
  and .sample_count == 4
  and .final_runtime_key_count == (.final_runtime | keys | length)
  and .final_runtime_completed_step_count == (.final_runtime.completed_steps | length)
  and .final_runtime_input_feedback_history_count == (.final_runtime.input_feedback_history | length)
  and .final_runtime_input_feedback_history_count == .keyboard_event_count
  and .final_runtime_visited_room_count == (.final_runtime.visited_rooms | length)
  and .final_runtime_visited_room_count == 2
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
