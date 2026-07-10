#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-session-slot-confirm.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-session-confirm-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- session-slot-confirm "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_session_slot_confirm_v1"
  and .session_slot_menu_contract == "trillionnium_world_bevy_session_slot_menu_v1"
  and .session_slot_buttons_contract == "trillionnium_world_bevy_session_slot_buttons_v1"
  and .session_recovery_ui_contract == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .initial_selected_gate == true
  and .selected_highlight_gate == true
  and .empty_load_selected_gate == true
  and .save_selected_gate == true
  and .overwrite_prompt_gate == true
  and .confirm_overwrite_gate == true
  and .load_selected_restore_gate == true
  and .continue_after_load_gate == true
  and .post_restore_guard_gate == true
  and .post_restore_continue_gate == true
  and .final_hud_gate == true
  and .slot_file_gate == true
  and .first_slot_a_bytes > 512
  and .pending_slot_a_bytes == .first_slot_a_bytes
  and .confirmed_slot_a_bytes >= .first_slot_a_bytes
  and .select_b_event.action == "SLOT:B"
  and .select_b_event.accepted == true
  and .select_b_event.availability_before == "enabled_select_slot:B"
  and .slot_b_selected_state.visual_state == "selected_slot"
  and .slot_b_selected_text.text == "* SLOT B"
  and .load_selected_b_event.action == "LOAD:SELECTED"
  and .load_selected_b_event.accepted == false
  and .load_selected_b_event.availability_before == "session_slot_missing:B"
  and .save_selected_a_event.action == "SAVE:SELECTED"
  and .save_selected_a_event.accepted == true
  and .save_selected_a_event.availability_before == "enabled_save_selected_slot:A"
  and .save_selected_a_pending_event.action == "SAVE:SELECTED"
  and .save_selected_a_pending_event.accepted == true
  and .save_selected_a_pending_event.availability_before == "enabled_overwrite_prompt:A"
  and .confirm_pending_state.visual_state == "overwrite_pending"
  and .cancel_pending_state.visual_state == "overwrite_pending"
  and .confirm_overwrite_event.action == "CONFIRM:OVERWRITE"
  and .confirm_overwrite_event.accepted == true
  and .confirm_overwrite_event.availability_before == "enabled_confirm_overwrite:A"
  and .load_selected_a_event.action == "LOAD:SELECTED"
  and .load_selected_a_event.accepted == true
  and .load_selected_a_event.availability_before == "enabled_session_slot_found:A"
  and (.after_load_selected_a_sample.session_recovery_text | contains("SESSION RECOVERED"))
  and (.after_load_selected_a_sample.session_recovery_text | contains("last bevy_button LOAD:SELECTED"))
  and .after_load_selected_a_sample.runtime.current_room_id == "league-coliseum"
  and .after_load_selected_a_sample.runtime.map_scene == "arena_outdoor"
  and .after_load_selected_a_sample.runtime.objective_status == "first_task_reward_ready"
  and (.after_load_selected_a_sample.session_resume_text | contains("RESUME ACTIVE"))
  and .after_load_selected_a_sample.runtime.session_resume_input_locked == true
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
  and .final_input_telemetry_summary.accepted_events == 10
  and .final_input_telemetry_summary.blocked_events == 2
  and .final_input_telemetry_summary.keyboard_events == 5
  and .final_input_telemetry_summary.bevy_button_events == 7
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and (.final_session_text | contains("SESSION RECOVERED"))
  and (.final_session_text | contains("checkpoint events 12 accepted 10 blocked 2"))
  and (.final_session_text | contains("guard EQUIP:bandit_sash:item_not_in_bag:bandit_sash"))
  and (.final_slot_text | contains("selected:A pending:none"))
  and (.final_slot_text | contains("SELECTED A:saved"))
  and (.final_slot_text | contains("B:empty LOAD session_slot_missing:B"))
  and (.final_event_log_text | contains("INPUT SUMMARY total 12 accepted 10 blocked 2 keyboard 5 buttons 7"))
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and (.first_playable.input_feedback_history | length == 12)
  and (.first_playable.input_feedback_history[] | select(.action_label == "SAVE:SELECTED" and .reason == "enabled_overwrite_prompt:A"))
  and .first_playable.session_selected_slot_id == "A"
  and .first_playable.session_overwrite_pending_slot_id == null
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

test ! -e "$SLOT_DIR/bevy-session-slot-b.snapshot.json"
test ! -e "$SLOT_DIR/bevy-session-slot-c.snapshot.json"

printf 'TRILLIONNIUM_WORLD_BEVY_SESSION_SLOT_CONFIRM_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
