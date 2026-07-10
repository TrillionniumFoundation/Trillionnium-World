#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-session-save-slot.json"
SLOT="$EVIDENCE_DIR/bevy-session-save-slot.snapshot.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- session-save-slot "$SLOT" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_session_save_slot_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .session_recovery_contract == "trillionnium_world_bevy_session_recovery_v1"
  and .session_recovery_ui_contract == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .input_telemetry_summary_contract == "trillionnium_world_bevy_input_telemetry_summary_v1"
  and .green == true
  and .slot_exists == true
  and .slot_bytes > 512
  and .slot_file_gate == true
  and .slot_restore_gate == true
  and .post_restore_guard_gate == true
  and .slot_continue_gate == true
  and .slot_hud_gate == true
  and (.slot_path | endswith("bevy-session-save-slot.snapshot.json"))
  and .restored_before_continue_summary.total_events == 10
  and .restored_before_continue_summary.accepted_events == 5
  and .restored_before_continue_summary.blocked_events == 5
  and .restored_before_continue_summary.last_action_label == "COMPLETE"
  and .post_restore_blocked_event.action == "EQUIP:bandit_sash"
  and .post_restore_blocked_event.accepted == false
  and .post_restore_blocked_event.availability_before == "item_not_in_bag:bandit_sash"
  and .post_restore_blocked_event.core_state_unchanged_when_disabled == true
  and .post_restore_equip_event.action == "EQUIP"
  and .post_restore_equip_event.accepted == true
  and .final_input_telemetry_summary.total_events == 12
  and .final_input_telemetry_summary.accepted_events == 6
  and .final_input_telemetry_summary.blocked_events == 6
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and (.final_session_text | contains("SESSION RECOVERED"))
  and (.final_session_text | contains("checkpoint events 12 accepted 6 blocked 6"))
  and (.final_session_text | contains("guard EQUIP:bandit_sash:item_not_in_bag:bandit_sash"))
  and (.final_event_log_text | contains("INPUT SUMMARY total 12 accepted 6 blocked 6 keyboard 12 buttons 0"))
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and (.first_playable.input_feedback_history | length == 10)
' "$SLOT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SESSION_SAVE_SLOT_GREEN %s slot=%s\n' "$SUMMARY" "$SLOT"
