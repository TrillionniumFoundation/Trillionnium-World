#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_material_consumption.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_authored_material_consumption_v1'
  'bevy-authored-material-consumption.json'
  'bevy-authored-material-consumption-manifest.json'
  'authored-material-consumption'
  'material-consumption'
  'trillionnium_world_bevy_authored_texture_atlas_binding_v1'
  'runtime_texture_handle::trnm_world_authored_sprite_sheet_v1'
  'scene_surface_match_gate'
  'runtime_target_consumption_gate'
  'material_slot_consumption_gate'
  'scene_layer_consumption_gate'
  'consumer_component_gate'
  'uv_rect_consumption_gate'
  'map_tile_renderer'
  'hud_renderer'
  'actor_renderer'
  'feedback_renderer'
  'world_tile_material'
  'hud_icon_material'
  'actor_sprite_material'
  'feedback_glyph_material'
  'Sprite+BevyWorldTileRpgSurface+BevyWorldAuthoredArtAssetSurface'
  'Text2d/Sprite+BevyWorldAuthoredArtAssetSurface'
  'Sprite+BevyWorldVisibleActorRuntime+BevyWorldAuthoredArtAssetSurface'
  'Sprite/Text2d+BevyWorldAuthoredArtAssetSurface'
  'runtime_texture_handle_material_slot_uv_rect'
  'local_authored_primitive_manifest_v1'
  'project_owned_internal_placeholder'
  'scene_material_consumes_generated_local_atlas_binding_not_final_external_art'
  'android_s5_real_device_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] authored material consumption contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored material consumption gate maps texture bindings into visible Bevy scene consumers"
