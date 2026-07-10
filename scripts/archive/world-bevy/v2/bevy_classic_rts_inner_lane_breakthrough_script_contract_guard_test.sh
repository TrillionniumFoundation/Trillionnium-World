#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_inner_lane_breakthrough.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_inner_lane_breakthrough_v1'
  'bevy-classic-rts-inner-lane-breakthrough.json'
  'bevy-classic-rts-inner-lane-breakthrough.ppm'
  'classic-rts-inner-lane-breakthrough'
  'input_path == "apply_live_native_action_with_source(classic_rts_inner_lane_breakthrough_input)"'
  'RTS:QUEUE:tier2:inner_route:inner_lane@11,2'
  'RTS:QUEUE:tier2:inner_gate:inner_latch@11,3'
  'RTS:QUEUE:tier2:inner_supply:relay_convoy@9,3'
  'RTS:QUEUE:tier2:inner_split:flank_team@10,4'
  'RTS:QUEUE:tier2:inner_clear:second_line@11,3'
  'RTS:QUEUE:tier2:inner_secure:signal_core@12,3'
  'siege_breach_dependency_gate == true'
  'inner_route_gate == true'
  'inner_gate_gate == true'
  'supply_convoy_gate == true'
  'split_squad_gate == true'
  'second_line_clear_gate == true'
  'signal_core_secure_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS inner lane script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_INNER_LANE_BREAKTHROUGH_CONTRACT'
  'native_classic_rts_inner_lane_breakthrough_evidence_json'
  'classic-rts-inner-lane-breakthrough'
  'classic_rts_inner_lane_breakthrough_input'
  'rts_inner_lane_tile_ids'
  'rts_inner_gate_ids'
  'rts_inner_defender_unit_ids'
  'rts_supply_convoy_ids'
  'rts_split_squad_tile_ids'
  'rts_inner_objective_state'
  'CLASSIC_RTS_INNER_ROUTE_COLOR'
  'CLASSIC_RTS_INNER_GATE_COLOR'
  'CLASSIC_RTS_INNER_DEFENDER_COLOR'
  'CLASSIC_RTS_INNER_SUPPLY_COLOR'
  'CLASSIC_RTS_INNER_SPLIT_COLOR'
  'CLASSIC_RTS_INNER_CORE_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS inner lane source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_inner_lane_breakthrough.sh'
  'bevy-classic-rts-inner-lane-breakthrough.json'
  'classic_rts_inner_lane_breakthrough_green'
  'rts_inner_lane_breakthrough_live_input_gate'
  'rts_inner_lane_breakthrough_siege_breach_dependency_gate'
  'rts_inner_lane_breakthrough_route_gate'
  'rts_inner_lane_breakthrough_gate_gate'
  'rts_inner_lane_breakthrough_supply_gate'
  'rts_inner_lane_breakthrough_split_gate'
  'rts_inner_lane_breakthrough_clear_gate'
  'rts_inner_lane_breakthrough_secure_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS inner lane readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS inner lane breakthrough evidence remains connected to siege-breach dependency, inner route, gate lock, supply convoy, split squad, defender clear, signal core secure, and readiness"
