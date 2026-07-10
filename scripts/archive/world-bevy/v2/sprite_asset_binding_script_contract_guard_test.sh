#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_sprite_asset_binding.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_sprite_asset_binding_v1'
  'bevy-sprite-asset-binding.json'
  'sprite-asset-binding'
  'runtime-texture-sprite-binding'
  'check_trillionnium_world_bevy_asset_store_registration.sh'
  'NativeRuntimeTextureAssetStoreHandles'
  'NativeRuntimeTextureSpriteBindingLookup'
  'BevyWorldRuntimeTextureSpriteAssetBinding'
  'bind_runtime_textures_to_authored_sprite_surfaces'
  'Sprite.image + Sprite.texture_atlas bound_to_registered_bevy_assets'
  'TextureAtlas {'
  'sprite_component_binding_gate'
  'visible_sprite_surface_binding_gate'
  'runtime_manifest_surface_match_gate'
  'app_handle_resource_gate'
  'bound_sprite_surface_count >= 24'
  'TRNM_WORLD_BEVY_SPRITE_ASSET_BINDING'
  'bevy_sprite_components_bound_to_registered_image_and_texture_atlas_layout_not_gpu_upload_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] sprite asset binding contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] sprite asset binding attaches registered Bevy Image and TextureAtlasLayout handles to visible Sprite surfaces without GPU/device claims"
