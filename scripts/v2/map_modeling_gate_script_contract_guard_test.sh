#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_map_modeling_gate.sh"

required_lines=(
  'trillionnium_world_map_modeling_gate_v1'
  'map-modeling-gate.json'
  'map-modeling-gate'
  'fixture_map_modeling_gate_green_with_public_data_blockers'
  'fixture_only == true'
  'live_ingestion_enabled == false'
  'runtime_clients_fetch_public_osm_directly == false'
  'public_network_ready == false'
  'building_modeling_gate'
  'road_modeling_gate'
  'greenery_modeling_gate'
  'terrain_modeling_gate'
  'building_mass_from_map_pack_node'
  'road_path_from_map_pack_edge'
  'greenery_cluster_from_map_pack_tags'
  'water_and_bank_surface'
  'signed production map_pack artifacts'
  'building_footprint_derivation_report'
  'road_graph_derivation_report'
  'greenery_landuse_derivation_report'
  'terrain_mesh_derivation_report'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] map modeling gate script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] map modeling gate keeps buildings, roads, greenery, terrain, signed map_pack boundary, and no live ingestion"
