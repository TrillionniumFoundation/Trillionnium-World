#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_shell_meta_ui_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-shell-meta-ui-replication'
  'bevy-classic-rts-shell-meta-ui-replication.json'
  'bevy-classic-rts-shell-meta-ui-replication.ppm'
  'trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1'
  'trillionnium_world_bevy_account_title_flow_v1'
  'trillionnium_world_bevy_title_menu_v1'
  'trillionnium_world_bevy_character_create_v1'
  'trillionnium_world_bevy_session_slot_menu_v1'
  'trillionnium_world_bevy_session_save_slot_v1'
  'trillionnium_world_bevy_session_slot_confirm_v1'
  'trillionnium_world_bevy_session_load_resume_v1'
  'trillionnium_world_bevy_session_recovery_ui_v1'
  'trillionnium_world_bevy_pause_menu_v1'
  'trillionnium_world_bevy_settings_menu_v1'
  'trillionnium_world_bevy_input_telemetry_hud_v1'
  'trillionnium_world_bevy_visible_button_hit_test_map_v1'
  'trillionnium_world_bevy_first_minute_onboarding_v1'
  'shell_meta_surface_count == 12'
  'source_contract_count == (.source_contracts | keys | length)'
  'source_path_count == (.source_paths | keys | length)'
  'source_headline_field_count == (.source_headline | keys | length)'
  'runtime_screen_layout_count == (.runtime_screen_layout | keys | length)'
  'shell_meta_pixel_count_field_count == (.shell_meta_pixel_counts | keys | length)'
  'shell_meta_player_first_pixel_count_field_count == (.shell_meta_player_first_pixel_counts | keys | length)'
  'shell_meta_surface_name_count == (.shell_meta_surface_names | length)'
  'shell_meta_slot_id_count == (.shell_meta_slot_ids | length)'
  'shell_meta_source_surface_count == (.shell_meta_source_surfaces | length)'
  'gate_count == 20'
  'passed_gate_count == 20'
  'failed_gate_count == 0'
  'runtime_screen_mode == "player_runtime_shell_meta_screen"'
  'runtime_screen_gate == true'
  'player_first_shell_meta_screen_gate == true'
  'shell_meta_player_first_pixel_counts.player_first_shell_meta_surface_non_background > 450000'
  'evidence_board_only == false'
  'shell_meta_ui_replication_gate == true'
  'external_evidence_ignored_for_current_replication_pass == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SHELL_META_UI_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing shell/meta UI replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SHELL_META_UI_REPLICATION_CONTRACT'
  'native_classic_rts_shell_meta_ui_replication_evidence_json'
  'TRNM RUST/BEVY SHELL + META RUNTIME SURFACE'
  'player_runtime_shell_meta_screen'
  'player_first_shell_meta_screen_gate'
  'shell_meta_player_first_pixel_counts'
  'evidence_board_only'
  'top account/login/continue CTA strip'
  'visible save slots with selected slot A'
  'pause, settings, input HUD, and hit-test cards'
  'bottom create-to-continue gameplay route'
  'native_classic_rts_full_screen_ui_replication_evidence_json'
  'native_account_title_flow_evidence_json'
  'native_title_menu_evidence_json'
  'native_character_create_evidence_json'
  'native_session_slot_menu_evidence_json'
  'native_session_save_slot_evidence_json'
  'native_session_slot_confirm_evidence_json'
  'native_session_load_resume_evidence_json'
  'native_session_recovery_ui_evidence_json'
  'native_pause_menu_evidence_json'
  'native_settings_menu_evidence_json'
  'native_input_telemetry_hud_evidence_json'
  'native_visible_button_hit_test_map_evidence_json'
  'native_first_minute_onboarding_evidence_json'
  'TITLE / ACCOUNT'
  'CHARACTER CREATE'
  'SESSION SLOT MENU'
  'SAVE SLOT FILE'
  'SAVE / LOAD CONFIRM'
  'LOAD / RESUME CTA'
  'SESSION RECOVERY'
  'PAUSE / RESUME'
  'SETTINGS'
  'INPUT HUD'
  'BUTTON HIT TEST'
  'FIRST-MINUTE HANDOFF'
  'shell_meta_ui_replication_gate'
  'external_evidence_ignored_for_current_replication_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing shell/meta UI replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_shell_meta_ui_replication.sh'
  'rts_shell_meta_ui_replication'
  'classic_rts_shell_meta_ui_replication_green'
  'bevy-classic-rts-shell-meta-ui-replication.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing shell/meta UI replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_shell_meta_ui_replication.sh'
  'bevy_classic_rts_shell_meta_ui_replication_script_contract_guard_test.sh'
  'bevy_classic_rts_shell_meta_ui_replication_gate'
  'trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing shell/meta UI replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS shell/meta UI replication gate remains connected to Rust CLI, shell UI sources, playtest readiness, release-review CI, and no-external-evidence boundaries"
