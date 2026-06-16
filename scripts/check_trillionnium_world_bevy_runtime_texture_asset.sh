#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset.json"
ATLAS="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.ppm"
MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet-manifest.json"
BINDING="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-texture-atlas-binding-manifest.json"
CONSUMPTION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-consumption-manifest.json"
APPLICATION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-application-manifest.json"
RUNTIME_ASSET="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" runtime-texture-asset "$ATLAS" "$MANIFEST" "$BINDING" "$CONSUMPTION" "$APPLICATION" "$RUNTIME_ASSET" >"$SUMMARY"
)

test -s "$ATLAS"
test -s "$MANIFEST"
test -s "$BINDING"
test -s "$CONSUMPTION"
test -s "$APPLICATION"
test -s "$RUNTIME_ASSET"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .green == true
  and .authored_material_application_contract == "trillionnium_world_bevy_authored_material_application_v1"
  and .image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
  and .texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
  and .atlas_bytes > 50000
  and .runtime_asset_bytes > 8192
  and .frame_count >= 32
  and .sprite_binding_count >= 32
  and .material_asset_count == 4
  and .material_application_gate == true
  and .atlas_file_gate == true
  and .manifest_frame_gate == true
  and .runtime_image_descriptor_gate == true
  and .texture_atlas_layout_gate == true
  and .material_handle_gate == true
  and .sprite_binding_gate == true
  and .scene_layer_asset_gate == true
  and .boundary_gate == true
  and .runtime_asset_manifest_write_gate == true
  and .runtime_asset_parse_gate == true
  and .image_asset_descriptor.asset_kind == "bevy::image::Image"
  and .image_asset_descriptor.runtime_format == "Rgba8UnormSrgb"
  and .image_asset_descriptor.sampler == "ImageSampler::nearest_pixel_art"
  and .texture_atlas_layout_descriptor.asset_kind == "bevy::sprite::TextureAtlasLayout"
  and .texture_atlas_layout_descriptor.frame_count >= 32
  and (.scene_layers | index("map") != null)
  and (.scene_layers | index("hud") != null)
  and (.scene_layers | index("actor") != null)
  and (.scene_layers | index("feedback") != null)
  and (.material_asset_handles | index("bevy_material_handle::world_tile_material::trnm_world_authored_sprite_sheet_v1") != null)
  and (.material_asset_handles | index("bevy_material_handle::hud_icon_material::trnm_world_authored_sprite_sheet_v1") != null)
  and (.material_asset_handles | index("bevy_material_handle::actor_sprite_material::trnm_world_authored_sprite_sheet_v1") != null)
  and (.material_asset_handles | index("bevy_material_handle::feedback_glyph_material::trnm_world_authored_sprite_sheet_v1") != null)
  and (.binding_modes | index("Handle<Image> + TextureAtlasLayout + texture_atlas_index attached_to_visible_sprite_surface") != null)
  and .asset_boundary == "host_side_bevy_image_texture_atlas_handle_registration_not_gpu_upload_or_device_claim"
  and .host_side_asset_registration_claimed == true
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .authored_material_application_contract == "trillionnium_world_bevy_authored_material_application_v1"
  and .image_asset_descriptor.asset_kind == "bevy::image::Image"
  and .texture_atlas_layout_descriptor.asset_kind == "bevy::sprite::TextureAtlasLayout"
  and (.material_asset_manifest | length) == 4
  and (.sprite_bindings | length) >= 32
  and (.sprite_bindings[0].image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1")
  and (.sprite_bindings[0].texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1")
  and (.sprite_bindings[0].binding_mode == "Handle<Image> + TextureAtlasLayout + texture_atlas_index attached_to_visible_sprite_surface")
  and (.sprite_bindings[0].gpu_upload_claimed == false)
  and .asset_boundary == "host_side_bevy_image_texture_atlas_handle_registration_not_gpu_upload_or_device_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$RUNTIME_ASSET" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_GREEN $SUMMARY $RUNTIME_ASSET"
