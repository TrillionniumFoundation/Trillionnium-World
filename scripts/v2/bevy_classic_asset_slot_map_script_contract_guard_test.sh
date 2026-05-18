#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_slot_map.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_asset_slot_map_v1'
  'bevy-classic-asset-slot-map.json'
  'classic-asset-slot-map'
  'slot_count >= 58'
  'category_counts.terrain'
  'category_counts.unit'
  'category_counts.building'
  'category_counts.doodad'
  'category_counts.vfx_ui'
  'manifest_frame_slot_count >= 43'
  'procedural_model_slot_count >= 5'
  'doodad_slot_count >= 4'
  'vfx_slot_count >= 6'
  'manifest_frame_slots_gate == true'
  'procedural_slots_gate == true'
  'replacement_boundary_gate == true'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
  'not_cex_runtime'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing asset slot map script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_SLOT_MAP_CONTRACT'
  'native_classic_asset_slot_map_evidence_json'
  'classic-asset-slot-map'
  'classic-slots'
  'manifest_frame'
  'procedural_isometric_model'
  'procedural_doodad'
  'procedural_vfx_ui'
  'model_town_hall'
  'model_training_hall'
  'model_waygate'
  'model_coliseum_stands'
  'model_tree_cluster_large'
  'doodad_rock_cluster'
  'doodad_barrel_stack'
  'doodad_torch'
  'doodad_crystal_cluster'
  'rts_command_destination_marker'
  'combat_attack_arc'
  'combat_hit_flash'
  'unit_health_bar'
  'replace_this_slot_in_trnm_world_bevy_assets_only'
  'not_cex_runtime'
  'wgpu_required'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing asset slot map source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic asset slot map keeps the Bevy-owned replacement contract"
