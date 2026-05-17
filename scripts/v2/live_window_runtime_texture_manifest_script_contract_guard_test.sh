#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh"

required_lines=(
  'trillionnium_world_bevy_live_window_screenshot_sequence_v1'
  'trillionnium_world_bevy_runtime_texture_asset_v1'
  'check_trillionnium_world_bevy_runtime_texture_asset.sh'
  'bevy-runtime-texture-asset.json'
  'bevy-runtime-texture-asset-manifest.json'
  'TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_MANIFEST'
  'TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_SHA256'
  'TRNM_WORLD_BEVY_RUNTIME_PROBE_PATH'
  'runtime_texture_sprite_asset_binding_gate'
  'runtime_texture_sprite_bound_surface_count'
  'trillionnium_world_bevy_sprite_asset_binding_v1'
  'runtime_texture_manifest_sha256'
  'runtime_texture_manifest_hash_gate'
  'runtime_texture_launch_env_gate'
  'runtime_texture_handle_gate'
  'bevy_image_handle::trnm_world_authored_sprite_sheet_v1'
  'bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] live-window runtime texture manifest contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] live-window screenshot sequence requires runtime texture asset manifest hash and handle ids before launch"
