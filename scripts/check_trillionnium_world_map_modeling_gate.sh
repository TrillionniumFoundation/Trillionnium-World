#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/map-modeling-gate.json"

mkdir -p "$ACCEPTANCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -q -p trnm-world-server -- map-modeling-gate >"$SUMMARY_FILE"
)

jq -e '
  .contract_version == "trillionnium_world_map_modeling_gate_v1"
  and .status == "fixture_map_modeling_gate_green_with_public_data_blockers"
  and .fixture_only == true
  and .live_ingestion_enabled == false
  and .runtime_clients_fetch_public_osm_directly == false
  and .public_network_ready == false
  and .layer_counts.buildings >= 20
  and .layer_counts.roads >= 20
  and .layer_counts.greenery >= 5
  and .layer_counts.terrain >= 4
  and .gates.building_modeling_gate == true
  and .gates.road_modeling_gate == true
  and .gates.greenery_modeling_gate == true
  and .gates.terrain_modeling_gate == true
  and .gates.no_live_ingestion_gate == true
  and .gates.all_layers_modeled == true
  and ([.modeling_layers.buildings[].asset_class] | index("building_mass_from_map_pack_node"))
  and ([.modeling_layers.roads[].asset_class] | index("road_path_from_map_pack_edge"))
  and ([.modeling_layers.greenery[].asset_class] | index("greenery_cluster_from_map_pack_tags"))
  and ([.modeling_layers.terrain[].mesh_role] | index("water_and_bank_surface"))
  and (.modeling_policy.production_data_rule | contains("signed production map_pack artifacts"))
  and (.required_next_evidence | index("building_footprint_derivation_report"))
  and (.required_next_evidence | index("road_graph_derivation_report"))
  and (.required_next_evidence | index("greenery_landuse_derivation_report"))
  and (.required_next_evidence | index("terrain_mesh_derivation_report"))
' "$SUMMARY_FILE" >/dev/null

printf 'TRILLIONNIUM_WORLD_MAP_MODELING_GATE_READY %s\n' "$SUMMARY_FILE"
