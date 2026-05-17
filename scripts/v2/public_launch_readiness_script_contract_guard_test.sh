#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_readiness.sh"

required_lines=(
  'trillionnium_world_public_launch_readiness_v1'
  'public-launch-readiness.json'
  'do_not_claim_public_launch_ready_without_native_bevy_local_playability_texture_sampling_render_asset_eligibility_real_device_map_pack_cohort_commercial_multi_node_and_public_deploy_evidence'
  'bevy-build-branch-title-route-all-branch-keyboard-replay.json'
  'bevy-action-coach.json'
  'bevy-player-hud-debug-layer.json'
  'bevy-live-window-screenshot-sequence.json'
  'bevy-sprite-texture-sampling.json'
  'bevy-live-window-sampled-texture-correlation.json'
  'bevy-render-asset-eligibility.json'
  'native_bevy_sprite_texture_sampling_contract'
  'native_bevy_live_window_sampled_texture_correlation_contract'
  'native_bevy_render_asset_eligibility_contract'
  'trillionnium_world_bevy_sprite_texture_sampling_v1'
  'trillionnium_world_bevy_live_window_sampled_texture_correlation_v1'
  'trillionnium_world_bevy_render_asset_eligibility_v1'
  'four_layer_texture_sampling_gate'
  'four_layer_sampled_live_correlation_gate'
  'render_asset_usage_gate'
  'sprite_render_reference_gate'
  'host_side_cpu_texture_sampling_not_gpu_upload_or_android_real_device'
  'host_side_sampled_texture_to_live_window_correlation_not_android_real_device'
  'host_side_render_asset_eligibility_not_render_world_extraction_or_gpu_upload'
  's5_real_device_matrix'
  'production_map_pack_public_evidence'
  'first_beta_cohort_evidence'
  'commercial_launch_drill_evidence'
  'multi_node_or_live_traffic_latency_evidence'
  'public_network_live_exposure_evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch readiness script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch readiness script consumes local Bevy texture/render gates and preserves external public-launch blockers"
