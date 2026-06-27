#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh"

required_lines=(
  'trillionnium_world_bevy_live_window_sampled_texture_correlation_v1'
  'bevy-live-window-sampled-texture-correlation.json'
  'trillionnium_world_bevy_sprite_texture_sampling_v1'
  'trillionnium_world_bevy_live_window_texture_correlation_v1'
  'check_trillionnium_world_bevy_sprite_texture_sampling.sh'
  'check_trillionnium_world_bevy_live_window_texture_correlation.sh'
  'bevy-sprite-texture-sampling.json'
  'bevy-live-window-texture-correlation.json'
  'ready_for_release_review'
  'sampled_layer_count'
  'sampled_layer_counts'
  'sampled_material_slot_count'
  'sampled_material_slot_counts'
  'layer_correlation_count'
  'sampled_layer_count == (.sampled_layer_counts | keys | length)'
  'sampled_material_slot_count == (.sampled_material_slot_counts | keys | length)'
  'layer_correlation_count == (.layer_correlations | length)'
  'same_runtime_manifest_hash_gate'
  'sampled_texture_nonblank_gate'
  'four_layer_sampled_live_correlation_gate'
  'live_window_pixels_correlated_to_cpu_sampled_bevy_texture_atlas_not_gpu_upload_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] live-window sampled texture correlation contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] live-window sampled texture correlation ties CPU sampled Bevy atlas regions to live-window pixel evidence without GPU/device claims"
