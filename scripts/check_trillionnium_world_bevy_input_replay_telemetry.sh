#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-input-replay-telemetry.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- input-replay-telemetry >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_input_replay_telemetry_v1"
  and .input_feedback_loop_contract == "trillionnium_world_bevy_input_feedback_loop_v1"
  and .keyboard_input_guard_contract == "trillionnium_world_bevy_keyboard_input_guard_v1"
  and .green == true
  and .parse_gate == true
  and .event_match_gate == true
  and .source_order_gate == true
  and .blocked_replay_gate == true
  and .accepted_replay_gate == true
  and .button_replay_signature_gate == true
  and .keyboard_replay_signature_gate == true
  and (.button_feedback_history | length) == (.button_replay_events | length)
  and (.keyboard_feedback_history | length) == (.keyboard_replay_events | length)
  and (.keyboard_feedback_history | length) >= 11
  and ([.keyboard_replay_events[] | select(.action_label == "MOVE:north" and .expected_accepted == false and .replay_accepted == false and .replay_reason == "training_required_before_route_move")] | length) >= 1
  and ([.keyboard_replay_events[] | select(.action_label == "EQUIP:bandit_sash" and .expected_accepted == false and .replay_accepted == false and .replay_reason == "item_not_in_bag:bandit_sash")] | length) >= 1
  and ([.keyboard_replay_events[] | select(.action_label == "EQUIP" and .expected_accepted == true and .replay_accepted == true and .replay_reason == "enabled_after_reward_claim")] | length) >= 1
  and (.keyboard_replay_final_runtime.equipment_ready == true)
  and (.keyboard_replay_final_runtime.objective_status == "first_playable_loop_complete")
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_INPUT_REPLAY_TELEMETRY_GREEN %s\n' "$SUMMARY"
