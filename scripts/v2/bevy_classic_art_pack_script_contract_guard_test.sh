#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh"
RUNNER="$ROOT/scripts/run_trillionnium_world_bevy_client.sh"
RUNNER_STATUS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_art_pack_v1'
  'bevy-classic-art-pack.json'
  'bevy-classic-art-pack.ppm'
  'assets/trnm-world/classic/art-pack-v1'
  'classic-art-pack'
  'model_town_hall'
  'model_waygate'
  'model_training_hall'
  'model_coliseum_stands'
  'model_tree_cluster_large'
  'doodad_rock_cluster'
  'doodad_barrel_stack'
  'doodad_torch'
  'doodad_crystal_cluster'
  'actor_player_idle_south'
  'actor_player_walk_north_1'
  'actor_player_walk_east_1'
  'actor_player_walk_west_1'
  'actor_enemy'
  'actor_enemy_attack'
  'asset_count >= 26'
  'preview_height >= 735'
  'group_counts.doodad >= 4'
  'doodad_art_gate == true'
  'preview_non_background_pixels > 35000'
  'model_detail_gate == true'
  'model_detail_asset_count >= 5'
  'model_unique_color_total >= 45'
  'model_shadow_pixel_count > 300'
  'model_highlight_pixel_count > 120'
  'unit_detail_gate == true'
  'player_unit_detail_asset_count >= 13'
  'enemy_unit_detail_asset_count >= 4'
  'unit_unique_color_total >= 100'
  'unit_shadow_pixel_count > 130'
  'unit_highlight_pixel_count > 100'
  'doodad_detail_gate == true'
  'doodad_detail_asset_count >= 4'
  'doodad_unique_color_total >= 12'
  'doodad_shadow_pixel_count > 20'
  'doodad_detail_pixel_count > 200'
  'replacement_boundary_gate == true'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic art pack script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ART_PACK_CONTRACT'
  'native_classic_art_pack_evidence_json'
  'classic-art-pack'
  'classic-artpack'
  'classic_art_pack_override_specs'
  'classic_art_pack_synthetic_override_specs'
  'classic_art_pack_pixels'
  'classic_draw_art_pack_preview'
  'model_town_hall'
  'model_waygate'
  'model_training_hall'
  'model_coliseum_stands'
  'model_tree_cluster_large'
  'actor_player_idle_south'
  'actor_player_walk_north_1'
  'actor_player_walk_east_1'
  'actor_player_walk_west_1'
  'actor_enemy'
  'actor_enemy_attack'
  'doodad_rock_cluster'
  'doodad_barrel_stack'
  'doodad_torch'
  'doodad_crystal_cluster'
  'town_hall'
  'waygate'
  'training_hall'
  'coliseum'
  'tree_cluster'
  'player_art_gate'
  'enemy_art_gate'
  'model_detail_gate'
  'classic_art_pack_highlight_color'
  'model_detail_asset_gate'
  'unit_detail_gate'
  'unit_detail_asset_gate'
  'player_unit_detail_asset_count'
  'enemy_unit_detail_asset_count'
  'doodad_detail_gate'
  'doodad_detail_asset_gate'
  'classic_art_pack_doodad_detail_color'
  'first real 2.5D override sprites'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic art pack source line: $line" >&2
    exit 1
  fi
done

if ! grep -Fq 'TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR' "$RUNNER" "$RUNNER_STATUS"; then
  echo "[FAIL] runner does not expose the classic art pack override env" >&2
  exit 1
fi

if ! grep -Fq 'art-pack-v1' "$RUNNER" "$RUNNER_STATUS"; then
  echo "[FAIL] runner does not target art-pack-v1" >&2
  exit 1
fi

echo "[PASS] classic art pack keeps the first real Bevy 2.5D assets wired to the runner"
