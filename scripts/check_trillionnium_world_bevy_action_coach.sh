#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- action-coach >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_action_coach_v1"
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
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_ACTION_COACH_GREEN $SUMMARY"
