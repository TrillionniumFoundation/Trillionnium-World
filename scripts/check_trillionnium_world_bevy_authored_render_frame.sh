#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-render-frame.json"
ATLAS="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.ppm"
MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet-manifest.json"
BINDING="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-texture-atlas-binding-manifest.json"
CONSUMPTION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-consumption-manifest.json"
APPLICATION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-application-manifest.json"
FRAME="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-render-frame.ppm"
FRAME_MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-render-frame-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- authored-render-frame "$ATLAS" "$MANIFEST" "$BINDING" "$CONSUMPTION" "$APPLICATION" "$FRAME" "$FRAME_MANIFEST" >"$SUMMARY"
)

test -s "$ATLAS"
test -s "$MANIFEST"
test -s "$BINDING"
test -s "$CONSUMPTION"
test -s "$APPLICATION"
test -s "$FRAME"
test -s "$FRAME_MANIFEST"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null
head -n 1 "$FRAME" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_render_frame_v1"
  and .green == true
  and .authored_material_application_contract == "trillionnium_world_bevy_authored_material_application_v1"
  and .frame_format == "ppm_p3_rgb"
  and .frame_bytes > 100000
  and .frame_manifest_bytes > 1024
  and .frame_unique_color_count >= 8
  and .application_count >= 32
  and .render_frame_application_count_gate == true
  and .render_frame_layer_gate == true
  and .render_frame_material_slot_gate == true
  and .render_frame_pipeline_gate == true
  and .render_frame_application_mode_gate == true
  and .frame_header_gate == true
  and .frame_nonblank_gate == true
  and .render_frame_boundary_gate == true
  and .frame_manifest_write_gate == true
  and .frame_manifest_parse_gate == true
  and (.scene_layers | index("map") != null)
  and (.scene_layers | index("hud") != null)
  and (.scene_layers | index("actor") != null)
  and (.scene_layers | index("feedback") != null)
  and (.material_slots | index("world_tile_material") != null)
  and (.material_slots | index("hud_icon_material") != null)
  and (.material_slots | index("actor_sprite_material") != null)
  and (.material_slots | index("feedback_glyph_material") != null)
  and (.render_pipelines | index("bevy_sprite_2d_material_pipeline") != null)
  and (.render_pipelines | index("bevy_text2d_sprite_icon_material_pipeline") != null)
  and .asset_boundary == "host_side_ppm_visual_frame_from_material_application_not_gpu_or_device_screenshot"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_render_frame_v1"
  and .authored_material_application_contract == "trillionnium_world_bevy_authored_material_application_v1"
  and .frame_format == "ppm_p3_rgb"
  and .frame_size.width == 384
  and .frame_size.height == 216
  and .frame_bytes > 100000
  and .frame_unique_color_count >= 8
  and .application_count >= 32
  and (.layer_samples | length) == 4
  and ([.layer_samples[] | select(.scene_layer == "map" and .drawn_pixel_count > 0)] | length) == 1
  and ([.layer_samples[] | select(.scene_layer == "hud" and .drawn_pixel_count > 0)] | length) == 1
  and ([.layer_samples[] | select(.scene_layer == "actor" and .drawn_pixel_count > 0)] | length) == 1
  and ([.layer_samples[] | select(.scene_layer == "feedback" and .drawn_pixel_count > 0)] | length) == 1
  and .asset_boundary == "host_side_ppm_visual_frame_from_material_application_not_gpu_or_device_screenshot"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
' "$FRAME_MANIFEST" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_RENDER_FRAME_GREEN $SUMMARY $FRAME $FRAME_MANIFEST"
