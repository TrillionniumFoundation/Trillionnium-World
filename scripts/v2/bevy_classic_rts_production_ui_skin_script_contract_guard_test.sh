#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_ui_skin.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-production-ui-skin'
  'bevy-classic-rts-production-ui-skin.json'
  'bevy-classic-rts-production-ui-skin.ppm'
  'trillionnium_world_bevy_classic_rts_production_ui_skin_v1'
  'trillionnium_world_bevy_classic_rts_production_asset_atlas_v1'
  'trillionnium_world_bevy_classic_rts_command_surface_v1'
  'trillionnium_world_bevy_classic_rts_selection_minimap_v1'
  'trillionnium_world_bevy_classic_rts_unit_status_portrait_v1'
  'trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1'
  'production_ui_skin_gate == true'
  'runtime_screen_mode == "player_runtime_production_hud_skin_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'bottom player HUD chrome and resource strip'
  'no_copy_boundary_gate == true'
  'screen_for_screen_openra_ui_claimed == false'
  'production_ready_ui_shipped == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_UI_SKIN_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing production UI skin script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_UI_SKIN_CONTRACT'
  'native_classic_rts_production_ui_skin_evidence_json'
  'TRNM NATIVE PRODUCTION UI SKIN'
  'native_classic_rts_production_asset_atlas_evidence_json'
  'native_classic_rts_command_surface_evidence_json'
  'native_classic_rts_selection_minimap_evidence_json'
  'native_classic_rts_unit_status_portrait_evidence_json'
  'native_classic_rts_selection_command_feedback_evidence_json'
  'native_classic_rts_ability_tooltip_telegraph_evidence_json'
  'native_classic_rts_control_group_hotkey_feedback_evidence_json'
  'production_ui_skin_preview_gate'
  'player_runtime_production_hud_skin_screen'
  'runtime_screen_layout'
  'evidence_board_only'
  'production_ui_skin_gate'
  'no_copy_boundary_gate'
  'screen_for_screen_openra_ui_claimed'
  'project_owned_internal_ui_skin_replacement_slots'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing production UI skin source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_ui_skin.sh'
  'rts_production_ui_skin'
  'classic_rts_production_ui_skin_green'
  'bevy-classic-rts-production-ui-skin.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing production UI skin readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_ui_skin.sh'
  'bevy_classic_rts_production_ui_skin_script_contract_guard_test.sh'
  'bevy_classic_rts_production_ui_skin_gate'
  'trillionnium_world_bevy_classic_rts_production_ui_skin_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing production UI skin release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS production UI skin gate remains connected to Rust CLI, production asset atlas, gameplay UI surfaces, no-copy policy, playtest readiness, and release-review CI"
