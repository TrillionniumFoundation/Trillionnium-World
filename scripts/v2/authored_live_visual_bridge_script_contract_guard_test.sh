#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_live_visual_bridge.sh"

required_lines=(
  'trillionnium_world_bevy_authored_live_visual_bridge_v1'
  'bevy-authored-live-visual-bridge.json'
  'trillionnium_world_bevy_authored_render_frame_v1'
  'trillionnium_world_bevy_live_window_screenshot_sequence_v1'
  'check_trillionnium_world_bevy_authored_render_frame.sh'
  'bevy-live-window-screenshot-sequence.json'
  'bevy-authored-render-frame.json'
  'authored_render_frame_gate'
  'live_window_sequence_gate'
  'live_final_frame_nonblank_gate'
  'four_layer_visual_bridge_gate'
  'boundary_gate'
  'map","hud","actor","feedback'
  'correlates_host_side_ppm_render_frame_with_live_window_screenshot_sequence_not_gpu_texture_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] authored live visual bridge contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored live visual bridge correlates render-frame artifact with live-window screenshot evidence"
