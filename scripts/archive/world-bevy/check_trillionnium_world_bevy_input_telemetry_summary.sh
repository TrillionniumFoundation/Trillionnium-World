#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-input-telemetry-summary.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- input-telemetry-summary >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_input_telemetry_summary_v1"
  and .input_replay_telemetry_contract == "trillionnium_world_bevy_input_replay_telemetry_v1"
  and .input_feedback_loop_contract == "trillionnium_world_bevy_input_feedback_loop_v1"
  and .green == true
  and .button_summary_gate == true
  and .keyboard_summary_gate == true
  and .blocked_reason_summary_gate == true
  and .accepted_action_summary_gate == true
  and .sample_summary_gate == true
  and .button_summary.total_events == 3
  and .button_summary.accepted_events == 2
  and .button_summary.blocked_events == 1
  and .button_summary.bevy_button_events == 3
  and .button_summary.last_action_label == "TRAIN"
  and .button_summary.last_accepted == true
  and .button_summary.last_reason == "enabled_after_dialogue_choice"
  and .keyboard_summary.total_events == 11
  and .keyboard_summary.accepted_events == 6
  and .keyboard_summary.blocked_events == 5
  and .keyboard_summary.keyboard_events == 11
  and .keyboard_summary.last_action_label == "EQUIP"
  and .keyboard_summary.last_accepted == true
  and .keyboard_summary.last_reason == "enabled_after_reward_claim"
  and (.keyboard_summary.blocked_reasons | index("training_required_before_route_move")) != null
  and (.keyboard_summary.blocked_reasons | index("item_not_in_bag:bandit_sash")) != null
  and (.keyboard_summary.accepted_action_labels | index("MOVE:north")) != null
  and (.keyboard_summary.accepted_action_labels | index("EQUIP")) != null
  and .keyboard_summary.final_objective_status == "first_playable_loop_complete"
  and .keyboard_summary.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_INPUT_TELEMETRY_SUMMARY_GREEN %s\n' "$SUMMARY"
