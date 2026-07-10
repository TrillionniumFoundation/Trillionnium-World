#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_texture_atlas_binding.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_authored_texture_atlas_binding_v1'
  'bevy-authored-texture-atlas-binding.json'
  'bevy-authored-texture-atlas-binding-manifest.json'
  'authored-texture-atlas-binding'
  'texture-atlas-binding'
  'runtime_texture_handle::trnm_world_authored_sprite_sheet_v1'
  'uv_rect_gate'
  'runtime_target_gate'
  'material_slot_gate'
  'replacement_slot_gate'
  'map_tile_renderer'
  'hud_renderer'
  'actor_renderer'
  'feedback_renderer'
  'world_tile_material'
  'hud_icon_material'
  'actor_sprite_material'
  'feedback_glyph_material'
  'tile_sprite_slot'
  'hud_icon_slot'
  'hud_glyph_slot'
  'actor_sprite_slot'
  'actor_shadow_slot'
  'actor_badge_slot'
  'feedback_glyph_slot'
  'nearest_pixel_art'
  'local_authored_primitive_manifest_v1'
  'project_owned_internal_placeholder'
  'runtime_texture_atlas_binding_for_generated_local_ppm_not_final_external_art'
  'android_s5_real_device_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] authored texture atlas binding contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored texture atlas binding gate maps atlas frames into runtime texture/material slots"
