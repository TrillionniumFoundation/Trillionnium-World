#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-production-interaction-polish'
  'bevy-classic-rts-production-interaction-polish.json'
  'bevy-classic-rts-production-interaction-polish.ppm'
  'SUMMARY_RAW="$(mktemp'
  'SUMMARY_TMP="$(mktemp'
  '.source_paths.production_ui_skin'
  'trillionnium_world_bevy_classic_rts_production_interaction_polish_v1'
  'trillionnium_world_bevy_classic_rts_production_ui_skin_v1'
  'source_contract_count'
  'source_path_count'
  'runtime_screen_layout_count'
  'interaction_pixel_count_field_count'
  'interaction_surface_name_count'
  'interaction_replacement_slot_count'
  'interaction_source_surface_count'
  'trillionnium_world_bevy_classic_rts_command_affordance_v1'
  'trillionnium_world_bevy_classic_rts_selection_command_feedback_v1'
  'trillionnium_world_bevy_classic_rts_build_lifecycle_v1'
  'trillionnium_world_bevy_classic_rts_scrollable_map_v1'
  'trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1'
  'production_interaction_polish_gate == true'
  'gate_count == 12'
  'passed_gate_count == 12'
  'failed_gate_count == 0'
  'runtime_screen_mode == "player_runtime_command_interaction_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'player_first_command_interaction_screen_gate == true'
  'player_first_command_interaction_view_non_background'
  'player_runtime_production_hud_skin_screen'
  'no_copy_boundary_gate == true'
  'screen_for_screen_openra_ui_claimed == false'
  'production_ready_interaction_ui_shipped == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_INTERACTION_POLISH_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing production interaction polish script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_INTERACTION_POLISH_CONTRACT'
  'native_classic_rts_production_interaction_polish_evidence_json'
  'TRNM_PRODUCTION_UI_SKIN_SUMMARY'
  'TRNM_PRODUCTION_UI_SKIN_PREVIEW'
  'TRNM PRODUCTION INTERACTION POLISH'
  'native_classic_rts_production_ui_skin_evidence_json'
  'native_classic_rts_command_affordance_evidence_json'
  'native_classic_rts_selection_command_feedback_evidence_json'
  'native_classic_rts_build_lifecycle_evidence_json'
  'native_classic_rts_scrollable_map_evidence_json'
  'native_classic_rts_command_queue_path_preview_evidence_json'
  'drag_marquee_skin_slot'
  'right_click_marker_skin_slot'
  'attack_cursor_skin_slot'
  'build_ghost_skin_slot'
  'queued_path_skin_slot'
  'scroll_minimap_skin_slot'
  'production_interaction_polish_preview_gate'
  'player_first_command_interaction_screen_gate'
  'interaction_pixel_counts'
  'player_runtime_command_interaction_screen'
  'runtime_screen_layout'
  'ui_skin_runtime_screen_gate'
  'evidence_board_only'
  'production_interaction_polish_gate'
  'no_copy_boundary_gate'
  'screen_for_screen_openra_ui_claimed'
  'project_owned_internal_interaction_feedback_skin_slots'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing production interaction polish source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh'
  'rts_production_interaction_polish'
  'classic_rts_production_interaction_polish_green'
  'bevy-classic-rts-production-interaction-polish.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing production interaction polish readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh'
  'bevy_classic_rts_production_interaction_polish_script_contract_guard_test.sh'
  'bevy_classic_rts_production_interaction_polish_gate'
  'TRNM_PRODUCTION_UI_SKIN_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.json"'
  'TRNM_PRODUCTION_UI_SKIN_PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.ppm"'
  'trillionnium_world_bevy_classic_rts_production_interaction_polish_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing production interaction polish release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS production interaction polish gate remains connected to Rust CLI, production UI skin, command feedback reducers, no-copy policy, playtest readiness, and release-review CI"
