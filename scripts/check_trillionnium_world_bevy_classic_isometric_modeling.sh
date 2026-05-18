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
  and .projection_gate == true
  and .depth_sort_gate == true
  and .diamond_tile_gate == true
  and .shadow_anchor_gate == true
  and .procedural_volume_gate == true
  and .sprite_anchor_gate == true
  and .preview_width == 640
  and .preview_height == 360
  and .unique_color_count >= 36
  and .non_background_pixels > 80000
  and .shadow_pixel_count > 250
  and .procedural_model_pixel_count > 5000
  and .canopy_pixel_count > 2500
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ISOMETRIC_MODELING_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
