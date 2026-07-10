#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_authored_material_application.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_authored_material_application_v1'
  'bevy-authored-material-application.json'
  'bevy-authored-material-application-manifest.json'
  'authored-material-application'
  'material-application'
  'trillionnium_world_bevy_authored_material_consumption_v1'
  'runtime_texture_handle::trnm_world_authored_sprite_sheet_v1'
  'material_application_count_gate'
  'texture_handle_application_gate'
  'uv_rect_application_gate'
  'scene_layer_application_gate'
  'material_slot_application_gate'
  'runtime_target_application_gate'
  'render_pipeline_gate'
  'replacement_policy_gate'
  'runtime_application_boundary_gate'
  'bevy_sprite_2d_material_pipeline'
  'bevy_text2d_sprite_icon_material_pipeline'
  'apply_texture_handle_material_slot_uv_rect_to_visible_scene_surface'
  'replace_placeholder_sprite_color_with_authored_texture_region'
  'generated_palette_fallback_if_texture_missing'
  'host_side_runtime_application_claimed'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'host_side_material_application_plan_for_generated_local_atlas_not_gpu_or_device_claim'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] authored material application contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] authored material application gate maps scene consumers into host-side render application plans"
