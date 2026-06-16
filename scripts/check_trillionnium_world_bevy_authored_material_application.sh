#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-application.json"
ATLAS="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.ppm"
MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet-manifest.json"
BINDING="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-texture-atlas-binding-manifest.json"
CONSUMPTION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-consumption-manifest.json"
APPLICATION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-application-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" authored-material-application "$ATLAS" "$MANIFEST" "$BINDING" "$CONSUMPTION" "$APPLICATION" >"$SUMMARY"
)

test -s "$ATLAS"
test -s "$MANIFEST"
test -s "$BINDING"
test -s "$CONSUMPTION"
test -s "$APPLICATION"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_material_application_v1"
  and .green == true
  and .authored_material_consumption_contract == "trillionnium_world_bevy_authored_material_consumption_v1"
  and .texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1"
  and .application_count >= 32
  and .application_bytes > 8192
  and .material_application_count_gate == true
  and .texture_handle_application_gate == true
  and .uv_rect_application_gate == true
  and .scene_layer_application_gate == true
  and .material_slot_application_gate == true
  and .runtime_target_application_gate == true
  and .render_pipeline_gate == true
  and .replacement_policy_gate == true
  and .runtime_application_boundary_gate == true
  and .source_boundary_gate == true
  and .application_write_gate == true
  and .application_parse_gate == true
  and (.scene_layers | index("map") != null)
  and (.scene_layers | index("hud") != null)
  and (.scene_layers | index("actor") != null)
  and (.scene_layers | index("feedback") != null)
  and (.render_pipelines | index("bevy_sprite_2d_material_pipeline") != null)
  and (.render_pipelines | index("bevy_text2d_sprite_icon_material_pipeline") != null)
  and (.application_modes | index("apply_texture_handle_material_slot_uv_rect_to_visible_scene_surface") != null)
  and (.replacement_policies | index("replace_placeholder_sprite_color_with_authored_texture_region") != null)
  and (.fallback_policies | index("generated_palette_fallback_if_texture_missing") != null)
  and .asset_boundary == "host_side_material_application_plan_for_generated_local_atlas_not_gpu_or_device_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_material_application_v1"
  and .authored_material_consumption_contract == "trillionnium_world_bevy_authored_material_consumption_v1"
  and .texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1"
  and .application_count >= 32
  and (.applications | length) >= 32
  and (.applications[0].texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1")
  and (.applications[0].material_application_mode == "apply_texture_handle_material_slot_uv_rect_to_visible_scene_surface")
  and (.applications[0].replacement_policy == "replace_placeholder_sprite_color_with_authored_texture_region")
  and (.applications[0].fallback_policy == "generated_palette_fallback_if_texture_missing")
  and (.applications[0].gpu_upload_claimed == false)
  and (.applications[0].android_s5_real_device_claimed == false)
  and .asset_boundary == "host_side_material_application_plan_for_generated_local_atlas_not_gpu_or_device_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
' "$APPLICATION" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_MATERIAL_APPLICATION_GREEN $SUMMARY $APPLICATION"
