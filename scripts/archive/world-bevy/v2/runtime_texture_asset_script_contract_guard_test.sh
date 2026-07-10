#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_runtime_texture_asset_v1'
  'bevy-runtime-texture-asset.json'
  'bevy-runtime-texture-asset-manifest.json'
  'runtime-texture-asset'
  'authored-runtime-texture-asset'
  'trillionnium_world_bevy_authored_material_application_v1'
  'bevy_image_handle::trnm_world_authored_sprite_sheet_v1'
  'bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1'
  'bevy::image::Image'
  'bevy::sprite::TextureAtlasLayout'
  'Rgba8UnormSrgb'
  'ImageSampler::nearest_pixel_art'
  'Handle<Image> + TextureAtlasLayout + texture_atlas_index attached_to_visible_sprite_surface'
  'runtime_image_descriptor_gate'
  'texture_atlas_layout_gate'
  'material_handle_gate'
  'sprite_binding_gate'
  'scene_layer_asset_gate'
  'runtime_asset_manifest_write_gate'
  'host_side_asset_registration_claimed'
  'host_side_bevy_image_texture_atlas_handle_registration_not_gpu_upload_or_device_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] runtime texture asset contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] runtime texture asset gate maps authored atlas evidence into Bevy Image/TextureAtlasLayout handle registrations without GPU/device claims"
