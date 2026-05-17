#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-session-recovery.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- session-recovery >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_session_recovery_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .input_telemetry_hud_contract == "trillionnium_world_bevy_input_telemetry_hud_v1"
  and .input_telemetry_summary_contract == "trillionnium_world_bevy_input_telemetry_summary_v1"
  and .input_feedback_loop_contract == "trillionnium_world_bevy_input_feedback_loop_v1"
  and .keyboard_input_guard_contract == "trillionnium_world_bevy_keyboard_input_guard_v1"
  and .green == true
  and .snapshot_round_trip_gate == true
  and .feedback_history_restore_gate == true
  and .post_restore_guard_gate == true
  and .post_restore_summary_gate == true
  and .hud_recovery_gate == true
  and .final_runtime_gate == true
  and .snapshot_json_bytes > 512
  and .before_snapshot_summary.total_events == 10
  and .before_snapshot_summary.accepted_events == 5
  and .before_snapshot_summary.blocked_events == 5
  and .before_snapshot_summary.last_action_label == "COMPLETE"
  and .restored_before_continue_summary.total_events == 10
  and .restored_before_continue_summary.accepted_events == 5
  and .restored_before_continue_summary.blocked_events == 5
  and .restored_before_continue_summary.last_action_label == "COMPLETE"
  and .restored_before_continue_summary.last_reason == "enabled_after_combat"
  and .post_restore_blocked_event.action == "EQUIP:bandit_sash"
  and .post_restore_blocked_event.accepted == false
  and .post_restore_blocked_event.availability_before == "item_not_in_bag:bandit_sash"
  and .post_restore_blocked_event.core_state_unchanged_when_disabled == true
  and .post_restore_equip_event.action == "EQUIP"
  and .post_restore_equip_event.accepted == true
  and .post_restore_equip_event.availability_before == "enabled_after_reward_claim"
  and .final_input_telemetry_summary.total_events == 12
  and .final_input_telemetry_summary.accepted_events == 6
  and .final_input_telemetry_summary.blocked_events == 6
  and .final_input_telemetry_summary.keyboard_events == 12
  and .final_input_telemetry_summary.bevy_button_events == 0
  and .final_input_telemetry_summary.last_input_source == "keyboard"
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and .final_input_telemetry_summary.last_accepted == true
  and .final_input_telemetry_summary.last_reason == "enabled_after_reward_claim"
  and (.final_event_log_text | contains("INPUT SUMMARY total 12 accepted 6 blocked 6 keyboard 12 buttons 0"))
  and (.final_event_log_text | contains("last keyboard EQUIP enabled_after_reward_claim"))
  and (.final_event_log_text | contains("keyboard EQUIP:bandit_sash blocked (item_not_in_bag:bandit_sash)"))
  and (.final_event_log_text | contains("keyboard EQUIP accepted (enabled_after_reward_claim)"))
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SESSION_RECOVERY_GREEN %s\n' "$SUMMARY"
