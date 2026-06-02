#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.ppm"
SLOT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication-slots"
mkdir -p "$(dirname "$SUMMARY")" "$SLOT_DIR"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-shell-meta-ui-replication "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"
  and .status == "classic_rts_shell_meta_ui_replication_green"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .source_contracts.full_screen_ui_replication == "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1"
  and .source_contracts.account_title_flow == "trillionnium_world_bevy_account_title_flow_v1"
  and .source_contracts.title_menu == "trillionnium_world_bevy_title_menu_v1"
  and .source_contracts.character_create == "trillionnium_world_bevy_character_create_v1"
  and .source_contracts.session_slot_menu == "trillionnium_world_bevy_session_slot_menu_v1"
  and .source_contracts.session_save_slot == "trillionnium_world_bevy_session_save_slot_v1"
  and .source_contracts.session_slot_confirm == "trillionnium_world_bevy_session_slot_confirm_v1"
  and .source_contracts.session_load_resume == "trillionnium_world_bevy_session_load_resume_v1"
  and .source_contracts.session_recovery_ui == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .source_contracts.pause_menu == "trillionnium_world_bevy_pause_menu_v1"
  and .source_contracts.settings_menu == "trillionnium_world_bevy_settings_menu_v1"
  and .source_contracts.input_telemetry_hud == "trillionnium_world_bevy_input_telemetry_hud_v1"
  and .source_contracts.visible_button_hit_test_map == "trillionnium_world_bevy_visible_button_hit_test_map_v1"
  and .source_contracts.first_minute_onboarding == "trillionnium_world_bevy_first_minute_onboarding_v1"
  and .shell_meta_surface_count == 12
  and (.shell_meta_surface_names | index("TITLE / ACCOUNT") != null)
  and (.shell_meta_surface_names | index("CHARACTER CREATE") != null)
  and (.shell_meta_surface_names | index("SESSION SLOT MENU") != null)
  and (.shell_meta_surface_names | index("SAVE SLOT FILE") != null)
  and (.shell_meta_surface_names | index("SAVE / LOAD CONFIRM") != null)
  and (.shell_meta_surface_names | index("LOAD / RESUME CTA") != null)
  and (.shell_meta_surface_names | index("SESSION RECOVERY") != null)
  and (.shell_meta_surface_names | index("PAUSE / RESUME") != null)
  and (.shell_meta_surface_names | index("SETTINGS") != null)
  and (.shell_meta_surface_names | index("INPUT HUD") != null)
  and (.shell_meta_surface_names | index("BUTTON HIT TEST") != null)
  and (.shell_meta_surface_names | index("FIRST-MINUTE HANDOFF") != null)
  and (.shell_meta_slot_ids | index("account_title_flow") != null)
  and (.shell_meta_slot_ids | index("character_create") != null)
  and (.shell_meta_slot_ids | index("session_slot_menu") != null)
  and (.shell_meta_slot_ids | index("save_slot_file") != null)
  and (.shell_meta_slot_ids | index("slot_confirm") != null)
  and (.shell_meta_slot_ids | index("load_resume_overlay") != null)
  and (.shell_meta_slot_ids | index("recovery_panel") != null)
  and (.shell_meta_slot_ids | index("pause_menu") != null)
  and (.shell_meta_slot_ids | index("settings_menu") != null)
  and (.shell_meta_slot_ids | index("input_telemetry_hud") != null)
  and (.shell_meta_slot_ids | index("hit_test_map") != null)
  and (.shell_meta_slot_ids | index("onboarding_handoff") != null)
  and .shell_meta_pixel_counts.board > 80000
  and .shell_meta_pixel_counts.account_title > 1000
  and .shell_meta_pixel_counts.character_create > 1000
  and .shell_meta_pixel_counts.session_slot_menu > 1000
  and .shell_meta_pixel_counts.save_slot_file > 1000
  and .shell_meta_pixel_counts.save_load_confirm > 1000
  and .shell_meta_pixel_counts.load_resume_cta > 1000
  and .shell_meta_pixel_counts.session_recovery > 1000
  and .shell_meta_pixel_counts.pause_resume > 1000
  and .shell_meta_pixel_counts.settings > 1000
  and .shell_meta_pixel_counts.input_hud > 1000
  and .shell_meta_pixel_counts.button_hit_test > 1000
  and .shell_meta_pixel_counts.first_minute_handoff > 1000
  and .shell_meta_pixel_counts.highlight > 2000
  and .source_headline.full_screen_surface_count == 10
  and .source_headline.account_session_bound == true
  and .source_headline.title_slot_a_bytes > 512
  and .source_headline.character_name == "Mira"
  and .source_headline.slot_menu_target_count == 10
  and .source_headline.save_slot_bytes > 512
  and .source_headline.settings_volume == 5
  and .source_headline.input_keyboard_events >= 10
  and .source_headline.onboarding_final_node == "league-coliseum"
  and .full_screen_ui_replication_gate == true
  and .account_title_gate == true
  and .title_menu_gate == true
  and .character_create_gate == true
  and .session_slot_menu_gate == true
  and .session_save_slot_gate == true
  and .session_slot_confirm_gate == true
  and .session_load_resume_gate == true
  and .session_recovery_gate == true
  and .pause_menu_gate == true
  and .settings_menu_gate == true
  and .input_hud_gate == true
  and .visible_hit_test_gate == true
  and .first_minute_onboarding_gate == true
  and .no_external_boundary_gate == true
  and .shell_meta_preview_gate == true
  and .source_preview_gate == true
  and .shell_meta_ui_replication_gate == true
  and .internal_shell_meta_ui_replication_claimed == true
  and .external_evidence_ignored_for_current_replication_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SHELL_META_UI_REPLICATION_GREEN %s %s slot_dir=%s\n' "$SUMMARY" "$PREVIEW" "$SLOT_DIR"
