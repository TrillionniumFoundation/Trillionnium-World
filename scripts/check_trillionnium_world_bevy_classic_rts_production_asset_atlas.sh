#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-production-asset-atlas "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_asset_atlas_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .source_contracts.production_art_replication == "trillionnium_world_bevy_classic_rts_production_art_replication_v1"
  and .source_contracts.authored_sprite_sheet == "trillionnium_world_bevy_authored_sprite_sheet_artifact_v1"
  and .source_contracts.authored_texture_atlas_binding == "trillionnium_world_bevy_authored_texture_atlas_binding_v1"
  and .source_contracts.runtime_texture_asset == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .production_family_count >= 9
  and .authored_surface_count >= 120
  and .authored_export_ready_count == .authored_surface_count
  and .atlas_family_count == 10
  and (.atlas_family_names | index("TERRAIN TILES") != null)
  and (.atlas_family_names | index("BUILDINGS") != null)
  and (.atlas_family_names | index("PLAYER UNITS") != null)
  and (.atlas_family_names | index("HUD ICONS") != null)
  and (.atlas_family_names | index("FEEDBACK VFX") != null)
  and .atlas_frame_count >= 32
  and .sprite_binding_count >= 32
  and .material_asset_count == 4
  and .atlas_bytes > 50000
  and .runtime_asset_bytes > 8192
  and (.runtime_scene_layers | index("map") != null)
  and (.runtime_scene_layers | index("hud") != null)
  and (.runtime_scene_layers | index("actor") != null)
  and (.runtime_scene_layers | index("feedback") != null)
  and (.runtime_material_slots | index("world_tile_material") != null)
  and (.runtime_material_slots | index("hud_icon_material") != null)
  and (.runtime_material_slots | index("actor_sprite_material") != null)
  and (.runtime_material_slots | index("feedback_glyph_material") != null)
  and (.binding_runtime_targets | index("map_tile_renderer") != null)
  and (.binding_runtime_targets | index("hud_renderer") != null)
  and (.binding_runtime_targets | index("actor_renderer") != null)
  and (.binding_runtime_targets | index("feedback_renderer") != null)
  and (.binding_replacement_slots | index("tile_sprite_slot") != null)
  and (.binding_replacement_slots | index("actor_sprite_slot") != null)
  and (.binding_replacement_slots | index("hud_icon_slot") != null)
  and (.binding_replacement_slots | index("feedback_glyph_slot") != null)
  and .atlas_board_pixel_count > 80000
  and .terrain_tile_pixel_count > 1500
  and .road_tile_pixel_count > 1500
  and .water_tile_pixel_count > 1500
  and .foliage_sprite_pixel_count > 1500
  and .building_sprite_pixel_count > 1500
  and .player_unit_sprite_pixel_count > 1500
  and .enemy_unit_sprite_pixel_count > 1500
  and .neutral_unit_sprite_pixel_count > 1500
  and .hud_icon_pixel_count > 2000
  and .feedback_vfx_pixel_count > 1500
  and .runtime_binding_lane_pixel_count > 8000
  and .uv_rect_pixel_count > 6000
  and .production_art_replication_gate == true
  and .sprite_sheet_gate == true
  and .texture_atlas_binding_gate == true
  and .runtime_texture_asset_gate == true
  and .production_asset_atlas_preview_gate == true
  and .production_asset_atlas_gate == true
  and .no_copy_boundary_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .final_external_bitmap_art_shipped == false
  and .production_ready_art_shipped == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .gpu_upload_claimed == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ASSET_ATLAS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
