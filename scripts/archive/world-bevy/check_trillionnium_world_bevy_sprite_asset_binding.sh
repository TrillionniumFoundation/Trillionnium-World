#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-asset-binding.json"
RUNTIME_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset.json"
RUNTIME_MANIFEST="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-runtime-texture-asset-manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_asset_store_registration.sh" >/dev/null
test -s "$RUNTIME_SUMMARY"
test -s "$RUNTIME_MANIFEST"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" sprite-asset-binding "$RUNTIME_SUMMARY" "$RUNTIME_MANIFEST" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_sprite_asset_binding_v1"
  and .green == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .runtime_texture_manifest_probe_contract == "trillionnium_world_bevy_runtime_texture_manifest_probe_v1"
  and .asset_store_registration_contract == "trillionnium_world_bevy_asset_store_registration_v1"
  and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
  and .runtime_summary_gate == true
  and .asset_store_registration_gate == true
  and .app_handle_resource_gate == true
  and .sprite_component_binding_gate == true
  and .texture_atlas_index_gate == true
  and .visible_sprite_surface_binding_gate == true
  and .runtime_manifest_surface_match_gate == true
  and .boundary_gate == true
  and (.runtime_manifest_sha256 | length) == 64
  and .asset_store_registration.green == true
  and .asset_store_registration.asset_store_registered_gate == true
  and .asset_store_registration.bevy_image_store_registration_gate == true
  and .asset_store_registration.texture_atlas_layout_store_registration_gate == true
  and .asset_store_registration.render_world_asset_usage_requested == true
  and .sprite_binding_lookup.green == true
  and .sprite_binding_lookup.binding_count >= 32
  and .bound_sprite_surface_count >= 24
  and (.scene_layers | index("map"))
  and (.scene_layers | index("hud"))
  and (.scene_layers | index("actor"))
  and (.scene_layers | index("feedback"))
  and (.material_slots | index("world_tile_material"))
  and (.material_slots | index("hud_icon_material"))
  and (.material_slots | index("actor_sprite_material"))
  and (.material_slots | index("feedback_glyph_material"))
  and (.bound_surfaces_sample | length) >= 8
  and (.bound_surfaces_sample[0].sprite_image_asset_id_debug | contains("index"))
  and (.bound_surfaces_sample[0].sprite_texture_atlas_layout_asset_id_debug | contains("index"))
  and .runtime_probe.runtime_texture_asset_store_registration_gate == true
  and (.host_log_line | contains("TRNM_WORLD_BEVY_SPRITE_ASSET_BINDING"))
  and .asset_boundary == "bevy_sprite_components_bound_to_registered_image_and_texture_atlas_layout_not_gpu_upload_claim"
  and .host_side_sprite_asset_binding_claimed == true
  and .render_world_asset_usage_requested == true
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_SPRITE_ASSET_BINDING_GREEN $SUMMARY"
