#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-settings-menu.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-settings-menu-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- settings-menu "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_settings_menu_v1"
  and .pause_menu_contract == "trillionnium_world_bevy_pause_menu_v1"
  and .session_load_resume_contract == "trillionnium_world_bevy_session_load_resume_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .pre_pause_settings_gate == true
  and .open_settings_gate == true
  and .adjust_settings_gate == true
  and .settings_save_gate == true
  and .settings_restore_gate == true
  and .resume_continue_gate == true
  and .post_continue_guard_gate == true
  and .post_continue_equip_gate == true
  and .final_hud_gate == true
  and .settings_before_pause_event.action == "SETTINGS:OPEN"
  and .settings_before_pause_event.accepted == false
  and .settings_before_pause_event.availability_before == "pause_menu_required_before_settings"
  and .pause_open_event.action == "PAUSE:MENU"
  and .pause_open_event.accepted == true
  and .settings_open_event.action == "SETTINGS:OPEN"
  and .settings_open_event.accepted == true
  and .settings_open_event.availability_before == "enabled_open_settings_menu"
  and .after_settings_open_sample.runtime.session_pause_input_locked == true
  and .after_settings_open_sample.runtime.session_settings_menu_visible == true
  and (.after_settings_open_sample.settings_menu_text | contains("SETTINGS ACTIVE"))
  and .settings_open_state.reason == "settings_menu_already_open"
  and .settings_back_state.visual_state == "settings_back_required"
  and .low_motion_event.accepted == true
  and .volume_down_event_1.accepted == true
  and .volume_down_event_2.accepted == true
  and .input_mode_event.accepted == true
  and .after_settings_adjusted_sample.runtime.session_settings_low_motion_enabled == true
  and .after_settings_adjusted_sample.runtime.session_settings_volume_level == 5
  and .after_settings_adjusted_sample.runtime.session_settings_input_mode == "keyboard_only"
  and (.after_settings_adjusted_sample.settings_menu_text | contains("volume 5/10"))
  and (.after_settings_adjusted_sample.settings_menu_text | contains("low_motion on"))
  and (.after_settings_adjusted_sample.settings_menu_text | contains("keyboard_only"))
  and .save_selected_a_event.action == "SAVE:SELECTED"
  and .save_selected_a_event.accepted == true
  and .after_settings_saved_sample.runtime.session_settings_menu_visible == true
  and .low_motion_mutation_event.accepted == true
  and .volume_up_mutation_event.accepted == true
  and .input_mode_mutation_event.accepted == true
  and .after_settings_mutated_sample.runtime.session_settings_low_motion_enabled == false
  and .after_settings_mutated_sample.runtime.session_settings_volume_level == 6
  and .after_settings_mutated_sample.runtime.session_settings_input_mode == "button_only"
  and .load_selected_a_event.action == "LOAD:SELECTED"
  and .load_selected_a_event.accepted == true
  and .after_settings_load_sample.runtime.session_resume_input_locked == true
  and .after_settings_load_sample.runtime.session_settings_menu_visible == false
  and .after_settings_load_sample.runtime.session_settings_low_motion_enabled == true
  and .after_settings_load_sample.runtime.session_settings_volume_level == 5
  and .after_settings_load_sample.runtime.session_settings_input_mode == "keyboard_only"
  and (.after_settings_load_sample.settings_menu_text | contains("SETTINGS READY"))
  and .continue_after_load_event.action == "CONTINUE:SESSION"
  and .continue_after_load_event.accepted == true
  and .after_continue_sample.runtime.session_resume_input_locked == false
  and .after_continue_sample.runtime.session_settings_low_motion_enabled == true
  and .after_continue_sample.runtime.session_settings_volume_level == 5
  and .after_continue_sample.runtime.session_settings_input_mode == "keyboard_only"
  and .post_continue_guard_event.action == "EQUIP:bandit_sash"
  and .post_continue_guard_event.accepted == false
  and .post_continue_guard_event.availability_before == "item_not_in_bag:bandit_sash"
  and .post_continue_guard_event.core_state_unchanged_when_disabled == true
  and .post_continue_equip_event.action == "EQUIP"
  and .post_continue_equip_event.accepted == true
  and .final_input_telemetry_summary.last_action_label == "EQUIP"
  and .final_runtime.objective_status == "first_playable_loop_complete"
  and .final_runtime.equipment_ready == true
  and (.final_settings_text | contains("volume 5/10"))
  and (.final_settings_text | contains("low_motion on"))
  and (.final_settings_text | contains("keyboard_only"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and .first_playable.session_pause_menu_visible == true
  and .first_playable.session_pause_input_locked == true
  and .first_playable.session_settings_menu_visible == true
  and .first_playable.session_settings_low_motion_enabled == true
  and .first_playable.session_settings_volume_level == 5
  and .first_playable.session_settings_input_mode == "keyboard_only"
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SETTINGS_MENU_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
