#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_formation_move_preview_v1'
  'bevy-classic-rts-formation-move-preview.json'
  'bevy-classic-rts-formation-move-preview.ppm'
  'classic-rts-formation-move-preview'
  'destination_ghost_gate == true'
  'wedge_spacing_gate == true'
  'line_reflow_gate == true'
  'collision_avoidance_gate == true'
  'split_avoidance_gate == true'
  'commit_spacing_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FORMATION_MOVE_PREVIEW_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS formation move preview script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FORMATION_MOVE_PREVIEW_CONTRACT'
  'native_classic_rts_formation_move_preview_evidence_json'
  'classic_draw_rts_formation_move_preview_overlay'
  'classic_rts_formation_move_preview_stage'
  'CLASSIC_RTS_FORMATION_PREVIEW_GHOST_COLOR'
  'CLASSIC_RTS_FORMATION_PREVIEW_PATH_COLOR'
  'CLASSIC_RTS_FORMATION_PREVIEW_SLOT_COLOR'
  'CLASSIC_RTS_FORMATION_PREVIEW_COLLISION_COLOR'
  'CLASSIC_RTS_FORMATION_PREVIEW_DISPERSE_COLOR'
  'Original Trillionnium formation move preview overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS formation move preview source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh'
  'bevy-classic-rts-formation-move-preview.json'
  'classic_rts_formation_move_preview_green'
  'rts_formation_move_preview_live_input_gate'
  'rts_formation_move_preview_destination_ghost_gate'
  'rts_formation_move_preview_wedge_spacing_gate'
  'rts_formation_move_preview_line_reflow_gate'
  'rts_formation_move_preview_collision_avoidance_gate'
  'rts_formation_move_preview_split_avoidance_gate'
  'rts_formation_move_preview_commit_spacing_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS formation move preview readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_formation_move_preview_v1'
  'bevy_classic_rts_formation_move_preview_contract_guard'
  'bevy_classic_rts_formation_move_preview_gate'
  'bevy_classic_rts_formation_move_preview_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS formation move preview release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS formation move preview evidence remains connected to renderer, CLI, readiness, release-review, live input, group formation, pathing, collision avoidance, and original art policy"
