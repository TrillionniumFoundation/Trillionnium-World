#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_art_pack.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_authored_art_pack_v1'
  'bevy-authored-art-pack.json'
  'authored-art-pack'
  'authored_art_pack_gate'
  'authored_art_pack_policy'
  'trnm_world_authored_art_pack_v1'
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
  'feedback_glyph_slot'
  'local_authored_primitive_manifest_v1'
  'project_owned_internal_placeholder'
  'project_owned_internal_placeholder_manifest_not_external_bitmap_ship_claim'
  'android_s5_real_device_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] authored art pack contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored art pack gate keeps project-owned replacement slots without claiming shipped external bitmap art"
