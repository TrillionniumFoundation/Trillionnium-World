#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-session-slot-menu.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-session-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- session-slot-menu "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_session_slot_menu_v1"
  and .session_slot_buttons_contract == "trillionnium_world_bevy_session_slot_buttons_v1"
  and .session_recovery_ui_contract == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .visible_empty_slot_gate == true
  and .save_a_gate == true
  and .overwrite_a_gate == true
  and .empty_load_b_gate == true
  and .load_a_restore_gate == true
  and .continue_after_load_gate == true
  and .post_restore_guard_gate == true
  and .post_restore_continue_gate == true
  and .final_hud_gate == true
  and .slot_status_gate == true
  and .first_slot_a_bytes > 512
  and .overwrite_slot_a_bytes >= .first_slot_a_bytes
  and .save_a_event.action == "SAVE:A"
  and .save_a_event.accepted == true
  and .save_a_event.availability_before == "enabled_session_checkpoint:A"
  and .overwrite_a_event.action == "SAVE:A"
  and .overwrite_a_event.accepted == true
  and .overwrite_a_event.availability_before == "enabled_session_checkpoint:A"
  and .load_b_empty_event.action == "LOAD:B"
  and .load_b_empty_event.accepted == false
  and .load_b_empty_event.availability_before == "session_slot_missing:B"
  and .load_a_event.action == "LOAD:A"
  and .load_a_event.accepted == true
  and .load_a_event.availability_before == "enabled_session_slot_found:A"
  and (.empty_slot_sample.session_slot_text | contains("A:empty LOAD session_slot_missing:A"))
  and (.empty_slot_sample.session_slot_text | contains("B:empty LOAD session_slot_missing:B"))
  and (.after_save_a_sample.session_slot_text | contains("A:saved"))
  and (.after_load_b_empty_sample.session_slot_text | contains("B:empty LOAD session_slot_missing:B"))
  and (.after_load_a_sample.session_recovery_text | contains("SESSION RECOVERED"))
  and (.after_load_a_sample.session_recovery_text | contains("last bevy_button LOAD:A"))
  and .after_load_a_sample.input_telemetry_summary.total_events == 12
  and .after_load_a_sample.input_telemetry_summary.accepted_events == 7
  and .after_load_a_sample.input_telemetry_summary.blocked_events == 5
  and .after_load_a_sample.input_telemetry_summary.bevy_button_events == 2
  and (.after_load_a_sample.session_resume_text | contains("RESUME ACTIVE"))
  and .after_load_a_sample.runtime.session_resume_input_locked == true
  and .continue_after_load_event.action == "CONTINUE:SESSION"
  and .continue_after_load_event.accepted == true
  and .continue_after_load_event.availability_before == "enabled_session_resume_continue"
  and .after_continue_sample.runtime.session_resume_input_locked == false
  and .post_restore_blocked_event.action == "EQUIP:bandit_sash"
  and .post_restore_blocked_event.accepted == false
  and .post_restore_blocked_event.availability_before == "item_not_in_bag:bandit_sash"
  and .post_restore_blocked_event.core_state_unchanged_when_disabled == true
  and .post_restore_equip_event.action == "EQUIP"
  and .post_restore_equip_event.accepted == true
  and .final_input_telemetry_summary.total_events == 12
  and .final_input_telemetry_summary.accepted_events == 9
  and .final_input_telemetry_summary.blocked_events == 3
  and .final_input_telemetry_summary.keyboard_events == 9
  and .final_input_telemetry_summary.bevy_button_events == 3
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and (.final_session_text | contains("SESSION RECOVERED"))
  and (.final_session_text | contains("checkpoint events 12 accepted 9 blocked 3"))
  and (.final_slot_text | contains("A:saved"))
  and (.final_slot_text | contains("B:empty LOAD session_slot_missing:B"))
  and (.final_slot_text | contains("C:empty LOAD session_slot_missing:C"))
  and (.final_event_log_text | contains("INPUT SUMMARY total 12 accepted 9 blocked 3 keyboard 9 buttons 3"))
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and (.first_playable.input_feedback_history | length == 11)
  and (.first_playable.input_feedback_history[] | select(.action_label == "SAVE:A" and .accepted == true))
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

test ! -e "$SLOT_DIR/bevy-session-slot-b.snapshot.json"
test ! -e "$SLOT_DIR/bevy-session-slot-c.snapshot.json"

printf 'TRILLIONNIUM_WORLD_BEVY_SESSION_SLOT_MENU_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
