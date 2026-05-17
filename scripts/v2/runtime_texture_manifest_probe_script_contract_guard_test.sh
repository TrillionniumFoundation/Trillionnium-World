#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_manifest_probe.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_runtime_texture_manifest_probe_v1'
  'bevy-runtime-texture-manifest-probe.json'
  'runtime-texture-manifest-probe'
  'runtime-texture-probe'
  'trillionnium_world_bevy_runtime_texture_asset_v1'
  'trillionnium_world_bevy_runtime_probe_v1'
  'check_trillionnium_world_bevy_runtime_texture_asset.sh'
  'TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_MANIFEST'
  'TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_SHA256'
  'NativeRuntimeTextureManifestProbe'
  'runtime_texture_manifest_probe_from_env'
  'runtime_texture_manifest_probe_gate'
  'app_resource_registered_gate'
  'manifest_hash_gate'
  'handle_gate'
  'bevy_image_handle::trnm_world_authored_sprite_sheet_v1'
  'bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1'
  'TRNM_WORLD_BEVY_RUNTIME_TEXTURE_MANIFEST_PROBE'
  'bevy_app_resource_reads_runtime_texture_manifest_env_and_reports_hash_not_gpu_upload_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] runtime texture manifest probe contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] runtime texture manifest probe is inserted into Bevy app resources and runtime probe output without GPU/device claims"
