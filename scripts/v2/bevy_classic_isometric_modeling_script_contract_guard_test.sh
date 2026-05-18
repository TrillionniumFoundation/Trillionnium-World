#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_isometric_modeling_v1'
  'bevy-classic-isometric-modeling.json'
  'bevy-classic-isometric-modeling.ppm'
  'classic-isometric-modeling'
  'orthographic_isometric_2_5d'
  'diamond_terrain_tiles'
  'y_depth_sorted_sprite_entities'
  'actor_footprint_shadows'
  'procedural_building_volumes'
  'tree_canopy_occlusion'
  'enlarged_actor_billboards'
  'multi_tile_rts_buildings'
  'warcraft_like_silhouette_set'
  'magic_gate_model'
  'terrain_road_overlay'
  'water_highlight_tiles'
  'raised_tile_cliff_faces'
  'rts_foundation_shadows'
  'rts_unit_selection_rings'
  'unit_health_bars'
  'player_enemy_mentor_silhouettes'
  'unit_depth_overlays'
  'rts_command_destination_marker'
  'combat_attack_arc'
  'combat_hit_flash'
  'procedural_volume_gate'
  'rts_model_set_gate'
  'terrain_detail_gate'
  'unit_detail_gate'
  'command_feedback_gate'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing isometric modeling script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ISOMETRIC_MODELING_CONTRACT'
  'native_classic_isometric_modeling_evidence_json'
  'classic_draw_isometric_scene'
  'classic_iso_project'
  'classic_draw_iso_diamond'
  'classic_draw_iso_shadow'
  'classic_draw_iso_prism'
  'classic_draw_iso_procedural_model'
  'classic_draw_iso_terrain_detail'
  'classic_draw_iso_unit_overlay'
  'classic_draw_iso_command_feedback'
  'classic_scene_rts_model_entities'
  'procedural_model_pixel_count'
  'rts_building_pixel_count'
  'rts_model_entity_count'
  'terrain_detail_pixel_count'
  'terrain_road_pixel_count'
  'terrain_water_pixel_count'
  'terrain_cliff_pixel_count'
  'terrain_foundation_pixel_count'
  'unit_detail_pixel_count'
  'unit_ring_pixel_count'
  'unit_health_pixel_count'
  'unit_silhouette_pixel_count'
  'command_feedback_pixel_count'
  'command_marker_pixel_count'
  'attack_arc_pixel_count'
  'hit_flash_pixel_count'
  'Warcraft-style 2.5D model'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing isometric modeling source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic isometric modeling script keeps the Warcraft-style 2.5D contract"
