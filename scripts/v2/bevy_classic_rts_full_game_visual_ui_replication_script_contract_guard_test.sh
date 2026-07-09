#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1'
  'bevy-classic-rts-full-game-visual-ui-replication.json'
  'bevy-classic-rts-full-game-visual-ui-replication.ppm'
  'classic-rts-full-game-visual-ui-replication'
  'runtime_screen_mode == "player_runtime_full_game_visual_ui_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'SUMMARY_RAW="$(mktemp "${SUMMARY}.raw.XXXXXX")"'
  'SUMMARY_TMP="$(mktemp "${SUMMARY}.tmp.XXXXXX")"'
  'source_contract_count = (.source_contracts | keys | length)'
  'source_path_count = (.source_paths | keys | length)'
  'source_review_contract_count = (.source_review_contracts | keys | length)'
  'source_review_gate_count = (.source_review_gates | keys | length)'
  'source_review_source_count = (.source_review_sources | keys | length)'
  'source_headline_field_count = (.source_headline | keys | length)'
  'single_screen_runtime_layout_count = (.single_screen_runtime_layout | keys | length)'
  'pixel_count_field_count = (.pixel_counts | keys | length)'
  'coverage_surface_count == 18'
  'coverage_surface_name_count == (.coverage_surface_names | length)'
  'command_grid_role_id_count == (.full_game_command_grid_role_ids | length)'
  'command_grid_icon_signature_count == (.full_game_command_grid_icon_signatures | length)'
  'command_grid_state_sample_count == (.full_game_command_grid_state_samples | length)'
  'gate_count == 14'
  'passed_gate_count == 14'
  'failed_gate_count == 0'
  'source_review_contracts.continuous_player_flow == "trnm_rts_evidence_continuous_player_flow_review_v1"'
  'source_contract_gate == true'
  'runtime_screen_chain_gate == true'
  'player_flow_gate == true'
  'player_first_first_contact_screen_readability_gate == true'
  'first_contact_art_direction == "top_down_pixel_rts"'
  'player_first_map_composition_gate == true'
  'player_first_five_second_readability_gate == true'
  'full_game_command_grid_readability_gate == true'
  'full_game_command_grid_active_role == "signal"'
  'full_game_command_grid_unique_icon_signature_count >= 6'
  'full_game_command_grid_sent_slot_count >= 3'
  'player_first_full_game_visual_ui_screen_gate == true'
  'full_game_visual_ui_replication_gate == true'
  'rts_evidence_full_game_visual_ui_replication_review_contract == "trnm_rts_evidence_full_game_visual_ui_replication_review_v1"'
  'rts_evidence_full_game_visual_ui_replication_review_gate == true'
  'internal_rust_full_game_visual_ui_replication_claimed == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_GAME_VISUAL_UI_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing full-game visual/UI replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_GAME_VISUAL_UI_REPLICATION_CONTRACT'
  'native_classic_rts_full_game_visual_ui_replication_evidence_json'
  'classic-rts-full-game-visual-ui-replication'
  'player_runtime_full_game_visual_ui_screen'
  'TRNM_RTS_EVIDENCE_FULL_GAME_VISUAL_UI_REPLICATION_REVIEW_CONTRACT'
  'rts_full_game_visual_ui_replication_review'
  'source_review_contracts'
  'player_first_full_game_visual_ui_screen_gate'
  'classic_draw_first_contact_basin_scene_with_layout'
  'player_first_first_contact_screen_readability_gate'
  'first_contact_art_direction'
  'first_contact_art_direction_pixel_counts'
  'first_contact_five_second_readability_checks'
  'player_first_five_second_readability_gate'
  'player_first_first_contact_lane_ground'
  'player_first_tactical_occluding_panel'
  'title_account_shell'
  'match_setup_start'
  'tactical_viewport'
  'map_minimap_camera'
  'command_grid'
  'session_slot_save_load'
  'open_world_handoff'
  'coverage_surface_count'
  'runtime_screen_chain_gate'
  'player_flow_gate'
  'full_game_command_grid_state_samples'
  'full_game_command_grid_readability_gate'
  'internal_rust_full_game_visual_ui_replication_claimed'
  'screen_for_screen_openra_ui_claimed'
  'third_party_asset_copied'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing full-game visual/UI replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh'
  'bevy-classic-rts-full-game-visual-ui-replication.json'
  'bevy-classic-rts-full-game-visual-ui-replication.ppm'
  'classic_rts_full_game_visual_ui_replication_green'
  'rts_full_game_visual_ui_replication_first_contact_screen_readability_gate == true'
  'rts_full_game_visual_ui_replication_player_first_tactical_composition_gate == true'
  'rts_full_game_visual_ui_replication_player_first_tactical_preview_non_background > 700000'
  'rts_full_game_visual_ui_replication_player_first_tactical_viewport_frame_pixel_count > 8000'
  'rts_full_game_visual_ui_replication_player_first_tactical_status_strip_pixel_count > 10000'
  'rts_full_game_visual_ui_replication_first_contact_lane_ground_pixel_count > 90000'
  'rts_full_game_visual_ui_replication_first_contact_hot_lane_anchor_pixel_count < 1500'
  'rts_full_game_visual_ui_replication_first_contact_combat_flow_pixel_count >= 200'
  'rts_full_game_visual_ui_replication_first_contact_target_rim_pixel_count >= 160'
  'rts_full_game_visual_ui_replication_first_contact_occluding_panel_pixel_count == 0'
  'rts_full_game_visual_ui_replication_command_grid_readability_gate == true'
  'rts_full_game_visual_ui_replication_review_contract'
  'rts_full_game_visual_ui_replication_rts_evidence_review_gate == true'
  'rts_full_game_visual_ui_replication_gate'
  'rts_full_game_visual_ui_replication_surface_count'
  'rts_full_game_visual_ui_replication_source_contract_count'
  'rts_full_game_visual_ui_replication_gate_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing full-game visual/UI replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh'
  'bevy_classic_rts_full_game_visual_ui_replication_script_contract_guard_test.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing full-game visual/UI replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] full-game visual/UI replication gate remains connected to Rust CLI, playtest readiness, release-review CI, and no-external-evidence boundaries"
