#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_live_window_layer_pixel_probe.sh"

required_lines=(
  'trillionnium_world_bevy_live_window_layer_pixel_probe_v1'
  'bevy-live-window-layer-pixel-probe.json'
  'trillionnium_world_bevy_authored_live_visual_bridge_v1'
  'trillionnium_world_bevy_live_window_screenshot_sequence_v1'
  'check_trillionnium_world_bevy_authored_live_visual_bridge.sh'
  'bevy-live-window-screenshot-sequence.json'
  'bevy-authored-render-frame.json'
  'map_playfield_pixels'
  'hud_pixels'
  'actor_activity_pixels'
  'feedback_action_pixels'
  'bridge_gate'
  'live_window_sequence_gate'
  'region_probe_gate'
  'four_layer_pixel_probe_gate'
  'boundary_gate'
  'live_window_png_region_probe_correlated_with_authored_visual_bridge_not_gpu_texture_claim'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] live-window layer pixel probe contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] live-window layer pixel probe samples map, HUD, actor, and feedback regions without claiming GPU or Android S5 evidence"
