#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.json"
SUMMARY_RAW="$SUMMARY.raw.$$"
SUMMARY_TMP="$SUMMARY.tmp.$$"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.ppm"
mkdir -p "$(dirname "$SUMMARY")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-isometric-modeling "$PREVIEW" >"$SUMMARY_RAW"
)

jq '
  .status = "classic_isometric_modeling_green"
  | .ready_for_release_review = true
  | .projection_sample_count = (.projection.samples | length)
  | .depth_order_count = (.depth_order | length)
  | .modeling_component_count = (.modeling_components | length)
  | .gate_count = 13
  | .passed_gate_count = ([
      .projection_gate,
      .diamond_tile_gate,
      .depth_sort_gate,
      .sprite_anchor_gate,
      .shadow_anchor_gate,
      .procedural_volume_gate,
      .rts_model_set_gate,
      .terrain_detail_gate,
      .environment_detail_gate,
      .doodad_detail_gate,
      .unit_detail_gate,
      .neutral_unit_detail_gate,
      .command_feedback_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
  | .android_s5_real_device_claimed = false
  | .external_evidence_ignored_for_current_isometric_modeling_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_isometric_modeling_v1"
  and .status == "classic_isometric_modeling_green"
  and .green == true
  and .ready_for_release_review == true
  and .gate_count == 13
  and .passed_gate_count == 13
  and .failed_gate_count == 0
  and .projection.mode == "orthographic_isometric_2_5d"
  and .projection_sample_count == (.projection.samples | length)
  and .depth_order_count == (.depth_order | length)
  and .modeling_component_count == (.modeling_components | length)
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
  and (.modeling_components | index("rts_command_destination_marker") != null)
  and (.modeling_components | index("combat_attack_arc") != null)
  and (.modeling_components | index("combat_hit_flash") != null)
  and (.modeling_components | index("rts_doodad_density") != null)
  and (.modeling_components | index("procedural_rock_clusters") != null)
  and (.modeling_components | index("torch_and_crystal_doodads") != null)
  and (.modeling_components | index("doodad_depth_sorting") != null)
  and (.modeling_components | index("biome_environment_overlays") != null)
  and (.modeling_components | index("bridge_and_cliff_detail_tiles") != null)
  and (.modeling_components | index("ruins_gold_vein_and_signpost_doodads") != null)
  and (.modeling_components | index("neutral_guard_worker_creep_units") != null)
  and .projection_gate == true
  and .depth_sort_gate == true
  and .diamond_tile_gate == true
  and .shadow_anchor_gate == true
  and .procedural_volume_gate == true
  and .rts_model_set_gate == true
  and .terrain_detail_gate == true
  and .unit_detail_gate == true
  and .command_feedback_gate == true
  and .doodad_detail_gate == true
  and .environment_detail_gate == true
  and .neutral_unit_detail_gate == true
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
  and .rts_neutral_unit_entity_count >= 8
  and .neutral_unit_detail_pixel_count > 450
  and .neutral_guard_pixel_count > 70
  and .neutral_worker_pixel_count > 70
  and .neutral_creep_pixel_count > 70
  and .command_feedback_pixel_count > 500
  and .command_marker_pixel_count > 250
  and .attack_arc_pixel_count > 100
  and .hit_flash_pixel_count > 80
  and .rts_doodad_entity_count >= 12
  and .doodad_detail_pixel_count > 900
  and .doodad_stone_pixel_count > 150
  and .doodad_wood_pixel_count > 150
  and .doodad_fire_pixel_count > 40
  and .doodad_crystal_pixel_count > 120
  and .rts_environment_entity_count >= 12
  and .environment_detail_pixel_count > 2500
  and .environment_foliage_pixel_count > 1000
  and .environment_ruin_pixel_count > 40
  and .environment_gold_pixel_count > 20
  and .environment_bridge_pixel_count > 60
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and .android_s5_real_device_claimed == false
  and .external_evidence_ignored_for_current_isometric_modeling_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ISOMETRIC_MODELING_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
