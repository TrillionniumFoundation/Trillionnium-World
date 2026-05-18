#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack_scene_probe.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_art_pack_scene_probe_v1'
  'bevy-classic-art-pack-scene-probe.json'
  'bevy-classic-art-pack-scene-probe.ppm'
  'classic-art-pack-scene-probe'
  'assets/trnm-world/classic/art-pack-v1'
  'override_presence_gate == true'
  'color_probe_gate == true'
  'terrain_override_presence_gate == true'
  'terrain_color_probe_gate == true'
  'world_prop_override_presence_gate == true'
  'world_prop_color_probe_gate == true'
  'vfx_override_presence_gate == true'
  'vfx_color_probe_gate == true'
  'town_hall_color_count > 20'
  'waygate_color_count > 20'
  'tree_color_count > 20'
  'coliseum_color_count > 20'
  'player_color_count > 20'
  'enemy_attack_color_count > 20'
  'terrain_grass_color_count > 600'
  'terrain_road_color_count > 100'
  'terrain_water_color_count > 40'
  'terrain_wall_roof_color_count > 80'
  'world_prop_runtime_color_count > 900'
  'command_marker_color_count > 200'
  'attack_arc_color_count > 100'
  'hit_flash_color_count > 80'
  'replacement_boundary_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic art pack scene probe script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ART_PACK_SCENE_PROBE_CONTRACT'
  'native_classic_art_pack_scene_probe_evidence_json'
  'classic-art-pack-scene-probe'
  'classic-artpack-scene'
  'classic_copy_pixels'
  'mirror_city_square'
  'league_coliseum'
  'model_town_hall'
  'model_waygate'
  'model_coliseum_stands'
  'model_tree_cluster_large'
  'actor_player_walk_east_1'
  'actor_enemy_attack'
  'tile_grass_a'
  'tile_grass_b'
  'tile_road'
  'tile_water'
  'tile_wall'
  'tile_roof'
  'tile_arena'
  'actor_mentor_talk'
  'prop_market_stall'
  'prop_arena_gate'
  'prop_reward'
  'prop_banner'
  'rts_command_destination_marker'
  'combat_attack_arc'
  'combat_hit_flash'
  'town_hall_color_count'
  'waygate_color_count'
  'tree_color_count'
  'coliseum_color_count'
  'player_color_count'
  'enemy_attack_color_count'
  'terrain_grass_color_count'
  'terrain_road_color_count'
  'terrain_water_color_count'
  'terrain_wall_roof_color_count'
  'terrain_override_presence_gate'
  'terrain_color_probe_gate'
  'world_prop_runtime_color_count'
  'world_prop_override_presence_gate'
  'world_prop_color_probe_gate'
  'command_marker_color_count'
  'attack_arc_color_count'
  'hit_flash_color_count'
  'vfx_override_presence_gate'
  'vfx_color_probe_gate'
  'live isometric scene panels'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic art pack scene probe source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic art pack scene probe proves the art pack reaches runtime frames"
