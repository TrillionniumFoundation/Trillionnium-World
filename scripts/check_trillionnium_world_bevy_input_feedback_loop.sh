#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-input-feedback-loop.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- input-feedback-loop >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_input_feedback_loop_v1"
  and .keyboard_input_guard_contract == "trillionnium_world_bevy_keyboard_input_guard_v1"
  and .contextual_button_guard_contract == "trillionnium_world_bevy_contextual_button_guard_v1"
  and .green == true
  and .button_blocked_feedback_gate == true
  and .button_accepted_feedback_gate == true
  and .keyboard_blocked_feedback_gate == true
  and .keyboard_accepted_feedback_gate == true
  and .event_log_feedback_gate == true
  and .feedback_history_cap_gate == true
  and (.button_feedback_history | length) >= 3
  and (.keyboard_feedback_history | length) >= 11
  and (.button_feedback_history[] | select(.input_source == "bevy_button" and .action_label == "TRAIN" and .accepted == false and .reason == "talk_required_before_training"))
  and (.button_feedback_history[] | select(.input_source == "bevy_button" and .action_label == "TALK" and .accepted == true and .reason == "enabled_at_mentor_tile"))
  and (.keyboard_feedback_history[] | select(.input_source == "keyboard" and .action_label == "MOVE:north" and .accepted == false and .reason == "training_required_before_route_move"))
  and (.keyboard_feedback_history[] | select(.input_source == "keyboard" and .action_label == "MOVE:north" and .accepted == true and .reason == "enabled_route_step_north"))
  and (.keyboard_feedback_history[] | select(.input_source == "keyboard" and .action_label == "EQUIP" and .accepted == true and .reason == "enabled_after_reward_claim"))
  and (.keyboard_final_runtime.equipment_ready == true)
  and (.keyboard_final_runtime.objective_status == "first_playable_loop_complete")
  and ([.button_samples[].event_log_text] | any(. != null and contains("bevy_button TRAIN blocked")))
  and ([.keyboard_samples[].event_log_text] | any(. != null and contains("keyboard MOVE:north accepted")))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_INPUT_FEEDBACK_LOOP_GREEN %s\n' "$SUMMARY"
