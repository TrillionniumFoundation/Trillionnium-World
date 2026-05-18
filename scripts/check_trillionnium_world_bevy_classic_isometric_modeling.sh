#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-isometric-modeling "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_isometric_modeling_v1"
  and .green == true
  and .projection.mode == "orthographic_isometric_2_5d"
  and (.modeling_components | index("diamond_terrain_tiles") != null)
  and (.modeling_components | index("y_depth_sorted_sprite_entities") != null)
  and (.modeling_components | index("actor_footprint_shadows") != null)
  and (.modeling_components | index("procedural_building_volumes") != null)
  and (.modeling_components | index("tree_canopy_occlusion") != null)
  and (.modeling_components | index("enlarged_actor_billboards") != null)
  and (.modeling_components | index("multi_tile_rts_buildings") != null)
  and (.modeling_components | index("warcraft_like_silhouette_set") != null)
  and (.modeling_components | index("magic_gate_model") != null)
  and (.modeling_components | index("terrain_road_overlay") != null)
  and (.modeling_components | index("water_highlight_tiles") != null)
  and (.modeling_components | index("raised_tile_cliff_faces") != null)
  and (.modeling_components | index("rts_foundation_shadows") != null)
  and (.modeling_components | index("rts_unit_selection_rings") != null)
  and (.modeling_components | index("unit_health_bars") != null)
  and (.modeling_components | index("player_enemy_mentor_silhouettes") != null)
  and (.modeling_components | index("unit_depth_overlays") != null)
  and .projection_gate == true
  and .depth_sort_gate == true
  and .diamond_tile_gate == true
  and .shadow_anchor_gate == true
  and .procedural_volume_gate == true
  and .rts_model_set_gate == true
  and .terrain_detail_gate == true
  and .unit_detail_gate == true
  and .sprite_anchor_gate == true
  and .preview_width == 640
  and .preview_height == 360
  and .unique_color_count >= 36
  and .non_background_pixels > 80000
  and .shadow_pixel_count > 250
  and .procedural_model_pixel_count > 5000
  and .canopy_pixel_count > 2500
  and .procedural_model_pixel_count > 10000
  and .canopy_pixel_count > 4000
  and .rts_model_entity_count >= 3
  and .rts_building_pixel_count > 1500
  and .terrain_detail_pixel_count > 6000
  and .terrain_road_pixel_count > 1000
  and .terrain_water_pixel_count > 300
  and .terrain_cliff_pixel_count > 1000
  and .terrain_foundation_pixel_count > 500
  and .unit_detail_pixel_count > 900
  and .unit_ring_pixel_count > 250
  and .unit_health_pixel_count > 90
  and .unit_silhouette_pixel_count > 500
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ISOMETRIC_MODELING_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
