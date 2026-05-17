#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-input-telemetry-hud.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- input-telemetry-hud >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_input_telemetry_hud_v1"
  and .input_telemetry_summary_contract == "trillionnium_world_bevy_input_telemetry_summary_v1"
  and .input_feedback_loop_contract == "trillionnium_world_bevy_input_feedback_loop_v1"
  and .keyboard_input_guard_contract == "trillionnium_world_bevy_keyboard_input_guard_v1"
  and .green == true
  and .hud_contract_gate == true
  and .hud_recent_input_gate == true
  and .sample_summary_gate == true
  and .final_runtime_gate == true
  and (.event_log_text | contains("INPUT SUMMARY total 11 accepted 6 blocked 5 keyboard 11 buttons 0"))
  and (.event_log_text | contains("last keyboard EQUIP enabled_after_reward_claim"))
  and (.event_log_text | contains("keyboard EQUIP accepted (enabled_after_reward_claim)"))
  and (.event_log_text | contains("keyboard COMPLETE accepted (enabled_after_combat)"))
  and .input_telemetry_summary.total_events == 11
  and .input_telemetry_summary.accepted_events == 6
  and .input_telemetry_summary.blocked_events == 5
  and .input_telemetry_summary.keyboard_events == 11
  and .input_telemetry_summary.bevy_button_events == 0
  and .input_telemetry_summary.last_input_source == "keyboard"
  and .input_telemetry_summary.last_action_label == "EQUIP"
  and .input_telemetry_summary.last_accepted == true
  and .input_telemetry_summary.last_reason == "enabled_after_reward_claim"
  and .final_sample.input_telemetry_summary.total_events == 11
  and .final_sample.input_telemetry_summary.accepted_events == 6
  and .final_sample.input_telemetry_summary.blocked_events == 5
  and .final_sample.input_telemetry_summary.keyboard_events == 11
  and .final_sample.input_telemetry_summary.bevy_button_events == 0
  and .final_sample.input_telemetry_summary.last_input_source == "keyboard"
  and .final_sample.input_telemetry_summary.last_action_label == "EQUIP"
  and .final_sample.input_telemetry_summary.last_accepted == true
  and .final_sample.input_telemetry_summary.last_reason == "enabled_after_reward_claim"
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_INPUT_TELEMETRY_HUD_GREEN %s\n' "$SUMMARY"
