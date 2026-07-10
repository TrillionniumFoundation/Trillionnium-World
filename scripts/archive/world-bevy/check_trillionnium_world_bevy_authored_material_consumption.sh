#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-consumption.json"
ATLAS="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet.ppm"
MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-sprite-sheet-manifest.json"
BINDING="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-texture-atlas-binding-manifest.json"
CONSUMPTION="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-authored-material-consumption-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" authored-material-consumption "$ATLAS" "$MANIFEST" "$BINDING" "$CONSUMPTION" >"$SUMMARY"
)

test -s "$ATLAS"
test -s "$MANIFEST"
test -s "$BINDING"
test -s "$CONSUMPTION"
head -n 1 "$ATLAS" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_material_consumption_v1"
  and .green == true
  and .authored_texture_atlas_binding_contract == "trillionnium_world_bevy_authored_texture_atlas_binding_v1"
  and .texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1"
  and .consumption_count >= 32
  and .consumption_bytes > 8192
  and .material_consumption_count_gate == true
  and .scene_surface_match_gate == true
  and .runtime_target_consumption_gate == true
  and .material_slot_consumption_gate == true
  and .replacement_slot_consumption_gate == true
  and .scene_layer_consumption_gate == true
  and .consumer_component_gate == true
  and .uv_rect_consumption_gate == true
  and .source_boundary_gate == true
  and .consumption_write_gate == true
  and .consumption_parse_gate == true
  and (.scene_layers | index("map") != null)
  and (.scene_layers | index("hud") != null)
  and (.scene_layers | index("actor") != null)
  and (.scene_layers | index("feedback") != null)
  and (.runtime_targets | index("map_tile_renderer") != null)
  and (.runtime_targets | index("hud_renderer") != null)
  and (.runtime_targets | index("actor_renderer") != null)
  and (.runtime_targets | index("feedback_renderer") != null)
  and (.material_slots | index("world_tile_material") != null)
  and (.material_slots | index("hud_icon_material") != null)
  and (.material_slots | index("actor_sprite_material") != null)
  and (.material_slots | index("feedback_glyph_material") != null)
  and (.consumer_components | index("Sprite+BevyWorldTileRpgSurface+BevyWorldAuthoredArtAssetSurface") != null)
  and (.consumer_components | index("Text2d/Sprite+BevyWorldAuthoredArtAssetSurface") != null)
  and (.consumer_components | index("Sprite+BevyWorldVisibleActorRuntime+BevyWorldAuthoredArtAssetSurface") != null)
  and (.consumer_components | index("Sprite/Text2d+BevyWorldAuthoredArtAssetSurface") != null)
  and (.consumption_modes | index("runtime_texture_handle_material_slot_uv_rect") != null)
  and (.source_origins | index("local_authored_primitive_manifest_v1") != null)
  and (.license_scopes | index("project_owned_internal_placeholder") != null)
  and .asset_boundary == "scene_material_consumes_generated_local_atlas_binding_not_final_external_art"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_material_consumption_v1"
  and .authored_texture_atlas_binding_contract == "trillionnium_world_bevy_authored_texture_atlas_binding_v1"
  and .texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1"
  and .consumption_count >= 32
  and (.consumptions | length) >= 32
  and (.consumptions[0].texture_handle_id == "runtime_texture_handle::trnm_world_authored_sprite_sheet_v1")
  and (.consumptions[0].scene_surface_match == true)
  and (.consumptions[0].uv_rect.u0 >= 0)
  and (.consumptions[0].uv_rect.u1 > .consumptions[0].uv_rect.u0)
  and .asset_boundary == "scene_material_consumes_generated_local_atlas_binding_not_final_external_art"
  and .android_s5_real_device_claimed == false
' "$CONSUMPTION" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_MATERIAL_CONSUMPTION_GREEN $SUMMARY $CONSUMPTION"
