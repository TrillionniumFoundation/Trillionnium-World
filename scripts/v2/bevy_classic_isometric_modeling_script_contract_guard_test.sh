#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_isometric_modeling_v1'
  'bevy-classic-isometric-modeling.json'
  'bevy-classic-isometric-modeling.ppm'
  'classic-isometric-modeling'
  'orthographic_isometric_2_5d'
  'diamond_terrain_tiles'
  'y_depth_sorted_sprite_entities'
  'actor_footprint_shadows'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing isometric modeling script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ISOMETRIC_MODELING_CONTRACT'
  'native_classic_isometric_modeling_evidence_json'
  'classic_draw_isometric_scene'
  'classic_iso_project'
  'classic_draw_iso_diamond'
  'classic_draw_iso_shadow'
  'Warcraft-style 2.5D model'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing isometric modeling source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic isometric modeling script keeps the Warcraft-style 2.5D contract"
