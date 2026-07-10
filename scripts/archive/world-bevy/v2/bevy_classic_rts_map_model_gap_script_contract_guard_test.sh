#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_map_model_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_map_model_gap_v1'
  'bevy-classic-rts-map-model-gap.json'
  'bevy-classic-rts-map-model-gap.ppm'
  'classic-rts-map-model-gap'
  'map_model_gap:lane_topology'
  'map_model_gap:resource_expansion'
  'map_model_gap:height_choke'
  'map_model_gap:structure_silhouette'
  'map_model_gap:unit_role_readability'
  'map_model_gap:fog_depth_cutaway'
  'map_topology_gate == true'
  'model_readability_gate == true'
  'openra_gap_not_closed_gate == true'
  'bevy_openra_parity_claimed == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MAP_MODEL_GAP_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS map/model gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MAP_MODEL_GAP_CONTRACT'
  'native_classic_rts_map_model_gap_evidence_json'
  'classic-rts-map-model-gap'
  'map_model_gap:lane_topology'
  'map_model_gap:resource_expansion'
  'map_model_gap:height_choke'
  'map_model_gap:structure_silhouette'
  'map_model_gap:unit_role_readability'
  'map_model_gap:fog_depth_cutaway'
  'map_model_catching_up_not_claimed'
  'Original Trillionnium map/model gap overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS map/model gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_map_model_gap.sh'
  'bevy-classic-rts-map-model-gap.json'
  'classic_rts_map_model_gap_green'
  'rts_map_model_gap_lane_gate'
  'rts_map_model_gap_map_topology_gate'
  'rts_map_model_gap_model_readability_gate'
  'rts_map_model_gap_openra_gap_not_closed_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS map/model gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS map/model gap evidence remains bound to terrain topology, resources, height/chokes, structures, unit roles, occlusion, readiness, and non-parity claim"
