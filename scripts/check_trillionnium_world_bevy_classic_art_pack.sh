#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack.ppm"
OVERRIDE_DIR="$ROOT/assets/trnm-world/classic/art-pack-v1"
mkdir -p "$(dirname "$SUMMARY")" "$OVERRIDE_DIR"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-art-pack "$OVERRIDE_DIR" "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_art_pack_v1"
  and .green == true
  and .asset_count >= 62
  and .override_frame_count >= 62
  and .preview_width == 640
  and .preview_height >= 1680
  and .preview_write_gate == true
  and .preview_non_background_pixels > 35000
  and .group_counts.town_hall == 1
  and .group_counts.waygate == 1
  and .group_counts.training_hall == 1
  and .group_counts.coliseum == 1
  and .group_counts.tree_cluster == 1
  and .group_counts.doodad >= 8
  and .group_counts.terrain >= 11
  and .group_counts.world_prop >= 9
  and .group_counts.vfx >= 6
  and .group_counts.player >= 13
  and .group_counts.enemy >= 4
  and .group_counts.neutral_unit >= 6
  and .required_model_gate == true
  and .player_art_gate == true
  and .enemy_art_gate == true
  and .neutral_unit_art_gate == true
  and .doodad_art_gate == true
  and .terrain_art_gate == true
  and .world_prop_art_gate == true
  and .vfx_art_gate == true
  and .model_detail_gate == true
  and .model_detail_asset_count >= 5
  and .model_unique_color_total >= 45
  and .model_shadow_pixel_count > 300
  and .model_highlight_pixel_count > 120
  and .unit_detail_gate == true
  and .player_unit_detail_asset_count >= 13
  and .enemy_unit_detail_asset_count >= 4
  and .unit_unique_color_total >= 100
  and .unit_shadow_pixel_count > 130
  and .unit_highlight_pixel_count > 100
  and .neutral_unit_detail_gate == true
  and .neutral_unit_detail_asset_count >= 6
  and .neutral_unit_unique_color_total >= 42
  and .neutral_unit_shadow_pixel_count > 48
  and .neutral_unit_highlight_pixel_count > 24
  and .neutral_unit_detail_pixel_count > 360
  and .doodad_detail_gate == true
  and .doodad_detail_asset_count >= 8
  and .doodad_unique_color_total >= 24
  and .doodad_shadow_pixel_count > 40
  and .doodad_detail_pixel_count > 420
  and .terrain_detail_gate == true
  and .terrain_detail_asset_count >= 11
  and .terrain_unique_color_total >= 44
  and .terrain_detail_pixel_count > 1350
  and .world_prop_detail_gate == true
  and .world_prop_detail_asset_count >= 9
  and .world_prop_unique_color_total >= 31
  and .world_prop_detail_pixel_count > 800
  and .vfx_detail_gate == true
  and .vfx_detail_asset_count >= 6
  and .vfx_unique_color_total >= 18
  and .vfx_detail_pixel_count > 700
  and .replacement_boundary_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and (.asset_groups | index("town_hall") != null)
  and (.asset_groups | index("waygate") != null)
  and (.asset_groups | index("training_hall") != null)
  and (.asset_groups | index("coliseum") != null)
  and (.asset_groups | index("tree_cluster") != null)
  and (.asset_groups | index("doodad") != null)
  and (.asset_groups | index("terrain") != null)
  and (.asset_groups | index("world_prop") != null)
  and (.asset_groups | index("vfx") != null)
  and (.asset_groups | index("player") != null)
  and (.asset_groups | index("enemy") != null)
  and (.asset_groups | index("neutral_unit") != null)
  and (.override_frame_ids | index("model_town_hall") != null)
  and (.override_frame_ids | index("model_waygate") != null)
  and (.override_frame_ids | index("model_training_hall") != null)
  and (.override_frame_ids | index("model_coliseum_stands") != null)
  and (.override_frame_ids | index("model_tree_cluster_large") != null)
  and (.override_frame_ids | index("doodad_rock_cluster") != null)
  and (.override_frame_ids | index("doodad_barrel_stack") != null)
  and (.override_frame_ids | index("doodad_torch") != null)
  and (.override_frame_ids | index("doodad_crystal_cluster") != null)
  and (.override_frame_ids | index("doodad_bush_cluster") != null)
  and (.override_frame_ids | index("doodad_ruins_column") != null)
  and (.override_frame_ids | index("doodad_gold_vein") != null)
  and (.override_frame_ids | index("doodad_signpost") != null)
  and (.override_frame_ids | index("tile_grass_a") != null)
  and (.override_frame_ids | index("tile_grass_b") != null)
  and (.override_frame_ids | index("tile_road") != null)
  and (.override_frame_ids | index("tile_water") != null)
  and (.override_frame_ids | index("tile_wall") != null)
  and (.override_frame_ids | index("tile_roof") != null)
  and (.override_frame_ids | index("tile_arena") != null)
  and (.override_frame_ids | index("tile_cliff_edge") != null)
  and (.override_frame_ids | index("tile_bridge") != null)
  and (.override_frame_ids | index("tile_forest_floor") != null)
  and (.override_frame_ids | index("tile_shadow_edge") != null)
  and (.override_frame_ids | index("actor_mentor_talk") != null)
  and (.override_frame_ids | index("actor_vendor_idle") != null)
  and (.override_frame_ids | index("prop_training_dummy") != null)
  and (.override_frame_ids | index("prop_reward") != null)
  and (.override_frame_ids | index("prop_arena_gate") != null)
  and (.override_frame_ids | index("prop_market_stall") != null)
  and (.override_frame_ids | index("prop_banner") != null)
  and (.override_frame_ids | index("marker_objective") != null)
  and (.override_frame_ids | index("marker_interaction") != null)
  and (.override_frame_ids | index("rts_command_destination_marker") != null)
  and (.override_frame_ids | index("combat_attack_arc") != null)
  and (.override_frame_ids | index("combat_hit_flash") != null)
  and (.override_frame_ids | index("rts_unit_selection_ring") != null)
  and (.override_frame_ids | index("unit_health_bar") != null)
  and (.override_frame_ids | index("rts_foundation_shadow") != null)
  and (.override_frame_ids | index("actor_player_idle_south") != null)
  and (.override_frame_ids | index("actor_player_walk_north_1") != null)
  and (.override_frame_ids | index("actor_player_walk_east_1") != null)
  and (.override_frame_ids | index("actor_player_walk_west_1") != null)
  and (.override_frame_ids | index("actor_enemy") != null)
  and (.override_frame_ids | index("actor_enemy_attack") != null)
  and (.override_frame_ids | index("actor_guard_idle") != null)
  and (.override_frame_ids | index("actor_guard_attack") != null)
  and (.override_frame_ids | index("actor_worker_idle") != null)
  and (.override_frame_ids | index("actor_worker_carry") != null)
  and (.override_frame_ids | index("actor_creep_idle") != null)
  and (.override_frame_ids | index("actor_creep_attack") != null)
  and (.written_assets[] | select(.frame_id == "model_town_hall") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_waygate") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_training_hall") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_coliseum_stands") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_tree_cluster_large") | .model_detail_asset_gate == true)
  and ([.written_assets[] | select(.group == "player" and .unit_detail_asset_gate == true)] | length >= 13)
  and ([.written_assets[] | select(.group == "enemy" and .unit_detail_asset_gate == true)] | length >= 4)
  and ([.written_assets[] | select(.group == "neutral_unit" and .neutral_unit_detail_asset_gate == true)] | length >= 6)
  and ([.written_assets[] | select(.group == "doodad" and .doodad_detail_asset_gate == true)] | length >= 8)
  and ([.written_assets[] | select(.group == "terrain" and .terrain_detail_asset_gate == true)] | length >= 11)
  and ([.written_assets[] | select(.group == "world_prop" and .world_prop_detail_asset_gate == true)] | length >= 9)
  and ([.written_assets[] | select(.group == "vfx" and .vfx_detail_asset_gate == true)] | length >= 6)
' "$SUMMARY" >/dev/null

for asset in model_town_hall model_waygate model_training_hall model_coliseum_stands model_tree_cluster_large doodad_rock_cluster doodad_barrel_stack doodad_torch doodad_crystal_cluster doodad_bush_cluster doodad_ruins_column doodad_gold_vein doodad_signpost tile_grass_a tile_grass_b tile_road tile_water tile_wall tile_roof tile_arena tile_cliff_edge tile_bridge tile_forest_floor tile_shadow_edge actor_mentor_talk actor_vendor_idle prop_training_dummy prop_reward prop_arena_gate prop_market_stall prop_banner marker_objective marker_interaction rts_command_destination_marker combat_attack_arc combat_hit_flash rts_unit_selection_ring unit_health_bar rts_foundation_shadow actor_player_idle_south actor_player_walk_north_1 actor_player_walk_east_1 actor_player_walk_west_1 actor_enemy actor_enemy_attack actor_guard_idle actor_guard_attack actor_worker_idle actor_worker_carry actor_creep_idle actor_creep_attack; do
  test -s "$OVERRIDE_DIR/$asset.ppm"
done
test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ART_PACK_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
