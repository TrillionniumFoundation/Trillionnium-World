#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-pause-menu.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-pause-menu-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- pause-menu "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_pause_menu_v1"
  and .session_load_resume_contract == "trillionnium_world_bevy_session_load_resume_v1"
  and .session_slot_confirm_contract == "trillionnium_world_bevy_session_slot_confirm_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .pause_open_gate == true
  and .paused_input_gate == true
  and .resume_gate == true
  and .paused_save_load_gate == true
  and .load_to_resume_gate == true
  and .resume_continue_gate == true
  and .post_continue_guard_gate == true
  and .post_continue_equip_gate == true
  and .final_hud_gate == true
  and .pause_open_event.action == "PAUSE:MENU"
  and .pause_open_event.accepted == true
  and .pause_open_event.availability_before == "enabled_open_pause_menu"
  and .after_pause_open_sample.runtime.session_pause_menu_visible == true
  and .after_pause_open_sample.runtime.session_pause_input_locked == true
  and (.after_pause_open_sample.pause_menu_text | contains("PAUSE ACTIVE"))
  and .pause_resume_state.visual_state == "pause_resume_required"
  and .pause_resume_state.enabled == true
  and .pause_button_state.reason == "pause_menu_already_open"
  and .paused_equip_state.reason == "pause_menu_resume_required"
  and .paused_input_event.action == "EQUIP"
  and .paused_input_event.accepted == false
  and .paused_input_event.availability_before == "pause_menu_resume_required"
  and .paused_input_event.core_state_unchanged_when_disabled == true
  and .pause_resume_event.action == "RESUME:MENU"
  and .pause_resume_event.accepted == true
  and .pause_resume_event.availability_before == "enabled_pause_resume"
  and .after_pause_resume_sample.runtime.session_pause_input_locked == false
  and (.after_pause_resume_sample.pause_menu_text | contains("PAUSE READY"))
  and .second_pause_event.action == "PAUSE:MENU"
  and .second_pause_event.accepted == true
  and .save_selected_a_event.action == "SAVE:SELECTED"
  and .save_selected_a_event.accepted == true
  and .save_selected_a_event.availability_before == "enabled_save_selected_slot:A"
  and .load_selected_a_event.action == "LOAD:SELECTED"
  and .load_selected_a_event.accepted == true
  and .load_selected_a_event.availability_before == "enabled_session_slot_found:A"
  and .after_paused_load_sample.runtime.session_resume_input_locked == true
  and .after_paused_load_sample.runtime.session_pause_input_locked == false
  and .after_paused_load_sample.runtime.session_resume_source_slot_id == "A"
  and (.after_paused_load_sample.session_resume_text | contains("RESUME ACTIVE"))
  and (.after_paused_load_sample.pause_menu_text | contains("PAUSE READY"))
  and .continue_state.visual_state == "onboarding_next_button"
  and .load_resume_locked_input_event.action == "EQUIP"
  and .load_resume_locked_input_event.accepted == false
  and .load_resume_locked_input_event.availability_before == "session_resume_continue_required"
  and .continue_after_load_event.action == "CONTINUE:SESSION"
  and .continue_after_load_event.accepted == true
  and .after_continue_sample.runtime.session_resume_input_locked == false
  and .after_continue_sample.runtime.session_pause_input_locked == false
  and .post_continue_guard_event.action == "EQUIP:bandit_sash"
  and .post_continue_guard_event.accepted == false
  and .post_continue_guard_event.availability_before == "item_not_in_bag:bandit_sash"
  and .post_continue_guard_event.core_state_unchanged_when_disabled == true
  and .post_continue_equip_event.action == "EQUIP"
  and .post_continue_equip_event.accepted == true
  and .post_continue_equip_event.availability_before == "enabled_after_reward_claim"
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and (.final_pause_text | contains("PAUSE READY"))
  and (.final_resume_text | contains("RESUME READY"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and .first_playable.session_pause_menu_visible == true
  and .first_playable.session_pause_input_locked == true
  and .first_playable.session_selected_slot_id == "A"
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_PAUSE_MENU_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
