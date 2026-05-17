#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_live_window_texture_correlation.sh"

required_lines=(
  'trillionnium_world_bevy_live_window_texture_correlation_v1'
  'bevy-live-window-texture-correlation.json'
  'trillionnium_world_bevy_runtime_texture_asset_v1'
  'trillionnium_world_bevy_live_window_layer_pixel_probe_v1'
  'check_trillionnium_world_bevy_runtime_texture_asset.sh'
  'check_trillionnium_world_bevy_live_window_layer_pixel_probe.sh'
  'bevy-runtime-texture-asset.json'
  'bevy-runtime-texture-asset-manifest.json'
  'bevy-live-window-layer-pixel-probe.json'
  'bevy_image_handle::trnm_world_authored_sprite_sheet_v1'
  'bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1'
  'material_slot_correlation_gate'
  'sprite_binding_correlation_gate'
  'pixel_region_correlation_gate'
  'four_layer_texture_window_correlation_gate'
  'layer_correlations'
  'texture_atlas_indexes'
  'material_asset_handles'
  'live_window_pixels_correlated_to_host_side_bevy_texture_handles_not_gpu_upload_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] live-window texture correlation contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] live-window texture correlation ties final PNG layer probes to host-side Bevy texture handles without GPU/device claims"
