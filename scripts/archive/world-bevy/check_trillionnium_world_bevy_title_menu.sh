#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-title-menu.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-title-menu-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- title-menu "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_title_menu_v1"
  and .session_load_resume_contract == "trillionnium_world_bevy_session_load_resume_v1"
  and .session_slot_confirm_contract == "trillionnium_world_bevy_session_slot_confirm_v1"
  and .character_create_contract == "trillionnium_world_bevy_character_create_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .title_boot_gate == true
  and .title_locked_input_gate == true
  and .title_continue_missing_gate == true
  and .title_new_game_gate == true
  and .character_confirm_gate == true
  and .title_slot_save_gate == true
  and .title_with_slot_gate == true
  and .title_continue_resume_gate == true
  and .title_continue_button_gate == true
  and .title_load_resume_gate == true
  and .post_continue_equip_gate == true
  and .initial_title_sample.runtime.session_title_menu_visible == true
  and .initial_title_sample.runtime.session_title_input_locked == true
  and (.initial_title_sample.title_menu_text | contains("TITLE ACTIVE"))
' "$SUMMARY" >/dev/null

jq -e '
  (.initial_title_input_sample.availability
    | any(.action == "TALK" and .enabled == false and .reason == "title_menu_choice_required"))
  and (.initial_title_input_sample.availability
    | any(.action == "TITLE:NEW" and .enabled == true and .reason == "enabled_title_new_game"))
  and (.initial_title_input_sample.availability
    | any(.action == "TITLE:CONTINUE" and .enabled == false and .reason == "title_continue_slot_missing:A"))
  and .title_new_state.visual_state == "onboarding_next_button"
  and .title_continue_missing_state.reason == "title_continue_slot_missing:A"
  and .talk_locked_state.reason == "title_menu_choice_required"
  and .title_locked_talk_event.action == "TALK"
  and .title_locked_talk_event.accepted == false
  and .title_locked_talk_event.availability_before == "title_menu_choice_required"
  and .title_continue_missing_event.action == "TITLE:CONTINUE"
  and .title_continue_missing_event.accepted == false
  and .title_continue_missing_event.availability_before == "title_continue_slot_missing:A"
  and .title_new_event.action == "TITLE:NEW"
  and .title_new_event.accepted == true
  and .title_new_event.availability_before == "enabled_title_new_game"
  and .after_title_new_sample.runtime.session_title_menu_visible == false
  and .after_title_new_sample.runtime.session_character_create_visible == true
  and .after_title_new_sample.runtime.session_character_create_input_locked == true
  and (.after_title_new_sample.character_create_text | contains("CREATE ACTIVE"))
  and .character_confirm_event.action == "CREATE:CONFIRM"
  and .character_confirm_event.accepted == true
  and .character_confirm_event.availability_before == "enabled_character_create_confirm"
  and .after_character_confirm_sample.runtime.session_character_create_visible == false
  and .after_character_confirm_sample.runtime.session_title_boot_state == "new_game_started"
  and .after_character_confirm_sample.character_display_name == "Ari"
  and .save_selected_a_event.action == "SAVE:SELECTED"
  and .save_selected_a_event.accepted == true
  and (.after_title_slot_save_sample.session_slot_text | contains("A:saved"))
  and .title_open_with_slot_event.action == "TITLE:OPEN"
  and .title_open_with_slot_event.accepted == true
  and .after_title_open_with_slot_sample.runtime.session_title_input_locked == true
  and .title_continue_state.reason == "enabled_title_continue_slot:A"
  and .title_load_state.reason == "enabled_title_load_slot:A"
  and .title_continue_event.action == "TITLE:CONTINUE"
  and .title_continue_event.accepted == true
  and .title_continue_event.availability_before == "enabled_title_continue_slot:A"
  and .after_title_continue_sample.runtime.session_resume_input_locked == true
  and .after_title_continue_sample.runtime.session_title_input_locked == false
  and .after_title_continue_sample.runtime.session_title_last_slot_id == "A"
  and (.after_title_continue_sample.session_resume_text | contains("RESUME ACTIVE"))
  and .continue_after_title_continue_event.action == "CONTINUE:SESSION"
  and .continue_after_title_continue_event.accepted == true
  and .after_title_continue_resume_sample.runtime.session_resume_input_locked == false
  and .after_title_continue_resume_sample.runtime.session_title_boot_state == "game_active"
  and .title_reopen_event.action == "TITLE:OPEN"
  and .title_reopen_event.accepted == true
  and .title_load_event.action == "TITLE:LOAD"
  and .title_load_event.accepted == true
  and .title_load_event.availability_before == "enabled_title_load_slot:A"
  and .after_title_load_sample.runtime.session_resume_input_locked == true
  and .after_title_load_sample.runtime.session_resume_source_slot_id == "A"
  and .continue_after_title_load_event.action == "CONTINUE:SESSION"
  and .continue_after_title_load_event.accepted == true
  and .after_title_load_continue_sample.runtime.session_resume_input_locked == false
  and .post_continue_guard_event.action == "EQUIP:bandit_sash"
  and .post_continue_guard_event.accepted == false
  and .post_continue_guard_event.availability_before == "item_not_in_bag:bandit_sash"
  and .post_continue_guard_event.core_state_unchanged_when_disabled == true
  and .post_continue_equip_event.action == "EQUIP"
  and .post_continue_equip_event.accepted == true
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and .first_playable.session_title_menu_visible == false
  and .first_playable.session_title_boot_state == "new_game_started"
  and .first_playable.session_character_create_name == "Ari"
  and .first_playable.session_character_create_archetype == "balanced"
  and .first_playable.session_resume_input_locked == false
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_TITLE_MENU_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
