#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_asset_store_registration.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_asset_store_registration_v1'
  'bevy-asset-store-registration.json'
  'asset-store-registration'
  'runtime-texture-asset-store-registration'
  'check_trillionnium_world_bevy_runtime_texture_manifest_probe.sh'
  'NativeRuntimeTextureAssetStoreRegistration'
  'register_runtime_texture_asset_store_from_probe'
  'Assets<Image>'
  'Assets<TextureAtlasLayout>'
  'Image::new'
  'TextureAtlasLayout::from_grid'
  'RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD'
  'runtime_texture_asset_store_registration_gate'
  'bevy_image_store_registration_gate'
  'texture_atlas_layout_store_registration_gate'
  'asset_store_registered_gate'
  'image_data_bytes == 131072'
  'TRNM_WORLD_BEVY_ASSET_STORE_REGISTRATION'
  'bevy_assets_main_world_image_and_texture_atlas_layout_registration_not_gpu_upload_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] asset store registration contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] asset store registration inserts Bevy Image and TextureAtlasLayout assets without GPU/device claims"
