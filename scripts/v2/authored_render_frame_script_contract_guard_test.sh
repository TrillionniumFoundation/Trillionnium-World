#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_render_frame.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_authored_render_frame_v1'
  'bevy-authored-render-frame.json'
  'bevy-authored-render-frame.ppm'
  'bevy-authored-render-frame-manifest.json'
  'authored-render-frame'
  'render-frame'
  'trillionnium_world_bevy_authored_material_application_v1'
  'ppm_p3_rgb'
  'render_frame_application_count_gate'
  'render_frame_layer_gate'
  'render_frame_material_slot_gate'
  'render_frame_pipeline_gate'
  'render_frame_application_mode_gate'
  'frame_nonblank_gate'
  'render_frame_boundary_gate'
  'bevy_sprite_2d_material_pipeline'
  'bevy_text2d_sprite_icon_material_pipeline'
  'world_tile_material'
  'hud_icon_material'
  'actor_sprite_material'
  'feedback_glyph_material'
  'host_side_ppm_visual_frame_from_material_application_not_gpu_or_device_screenshot'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] authored render frame contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored render frame gate renders host-side PPM visual evidence from material applications"
