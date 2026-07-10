#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_sprite_sheet.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_authored_sprite_sheet_artifact_v1'
  'bevy-authored-sprite-sheet.json'
  'bevy-authored-sprite-sheet.ppm'
  'bevy-authored-sprite-sheet-manifest.json'
  'authored-sprite-sheet'
  'ppm_p3_rgb'
  'P3'
  'frame_count_gate'
  'frame_asset_kind_gate'
  'frame_layer_gate'
  'frame_slot_gate'
  'atlas_write_gate'
  'manifest_write_gate'
  'terrain_tile'
  'road_tile'
  'building_tile'
  'foliage_sprite'
  'water_tile'
  'hud_icon'
  'hud_glyph'
  'actor_sprite'
  'feedback_glyph'
  'tile_sprite_slot'
  'hud_icon_slot'
  'hud_glyph_slot'
  'actor_sprite_slot'
  'actor_shadow_slot'
  'actor_badge_slot'
  'feedback_glyph_slot'
  'local_authored_primitive_manifest_v1'
  'project_owned_internal_placeholder'
  'generated_project_owned_local_ppm_atlas_not_final_external_bitmap_art'
  'android_s5_real_device_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] authored sprite sheet contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored sprite sheet gate writes a local atlas artifact and frame manifest without claiming final external bitmap art"
