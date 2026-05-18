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
  and .asset_count >= 22
  and .override_frame_count >= 22
  and .preview_width == 640
  and .preview_height == 420
  and .preview_write_gate == true
  and .preview_non_background_pixels > 35000
  and .group_counts.town_hall == 1
  and .group_counts.waygate == 1
  and .group_counts.training_hall == 1
  and .group_counts.coliseum == 1
  and .group_counts.tree_cluster == 1
  and .group_counts.player >= 13
  and .group_counts.enemy >= 4
  and .required_model_gate == true
  and .player_art_gate == true
  and .enemy_art_gate == true
  and .model_detail_gate == true
  and .model_detail_asset_count >= 5
  and .model_unique_color_total >= 45
  and .model_shadow_pixel_count > 300
  and .model_highlight_pixel_count > 120
  and .replacement_boundary_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and (.asset_groups | index("town_hall") != null)
  and (.asset_groups | index("waygate") != null)
  and (.asset_groups | index("training_hall") != null)
  and (.asset_groups | index("coliseum") != null)
  and (.asset_groups | index("tree_cluster") != null)
  and (.asset_groups | index("player") != null)
  and (.asset_groups | index("enemy") != null)
  and (.override_frame_ids | index("model_town_hall") != null)
  and (.override_frame_ids | index("model_waygate") != null)
  and (.override_frame_ids | index("model_training_hall") != null)
  and (.override_frame_ids | index("model_coliseum_stands") != null)
  and (.override_frame_ids | index("model_tree_cluster_large") != null)
  and (.override_frame_ids | index("actor_player_idle_south") != null)
  and (.override_frame_ids | index("actor_player_walk_north_1") != null)
  and (.override_frame_ids | index("actor_player_walk_east_1") != null)
  and (.override_frame_ids | index("actor_player_walk_west_1") != null)
  and (.override_frame_ids | index("actor_enemy") != null)
  and (.override_frame_ids | index("actor_enemy_attack") != null)
  and (.written_assets[] | select(.frame_id == "model_town_hall") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_waygate") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_training_hall") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_coliseum_stands") | .model_detail_asset_gate == true)
  and (.written_assets[] | select(.frame_id == "model_tree_cluster_large") | .model_detail_asset_gate == true)
' "$SUMMARY" >/dev/null

for asset in model_town_hall model_waygate model_training_hall model_coliseum_stands model_tree_cluster_large actor_player_idle_south actor_player_walk_north_1 actor_player_walk_east_1 actor_player_walk_west_1 actor_enemy actor_enemy_attack; do
  test -s "$OVERRIDE_DIR/$asset.ppm"
done
test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ART_PACK_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
