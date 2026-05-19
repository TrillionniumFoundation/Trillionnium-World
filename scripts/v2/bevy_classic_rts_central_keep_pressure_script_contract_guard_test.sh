#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_pressure.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_central_keep_pressure_v1'
  'bevy-classic-rts-central-keep-pressure.json'
  'bevy-classic-rts-central-keep-pressure.ppm'
  'classic-rts-central-keep-pressure'
  'input_path == "apply_live_native_action_with_source(classic_rts_central_keep_pressure_input)"'
  'RTS:QUEUE:tier2:keep_route:central_keep@13,3'
  'RTS:QUEUE:tier2:keep_shield:mirror_ward@13,3'
  'RTS:QUEUE:tier2:keep_guard:warden_line@12,3'
  'RTS:QUEUE:tier2:keep_siege:final_line@12,4'
  'RTS:QUEUE:tier2:keep_pressure:central_keep@13,3'
  'inner_lane_dependency_gate == true'
  'keep_route_gate == true'
  'keep_shield_gate == true'
  'keep_guard_gate == true'
  'keep_siege_line_gate == true'
  'keep_pressure_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS central keep script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CENTRAL_KEEP_PRESSURE_CONTRACT'
  'native_classic_rts_central_keep_pressure_evidence_json'
  'classic-rts-central-keep-pressure'
  'classic_rts_central_keep_pressure_input'
  'rts_central_keep_target_ids'
  'rts_central_keep_route_tile_ids'
  'rts_keep_shield_percent'
  'rts_boss_guard_unit_ids'
  'rts_player_siege_line_tile_ids'
  'rts_central_keep_state'
  'CLASSIC_RTS_KEEP_ROUTE_COLOR'
  'CLASSIC_RTS_KEEP_SHIELD_COLOR'
  'CLASSIC_RTS_KEEP_GUARD_COLOR'
  'CLASSIC_RTS_KEEP_SIEGE_LINE_COLOR'
  'CLASSIC_RTS_KEEP_PRESSURE_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS central keep source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_central_keep_pressure.sh'
  'bevy-classic-rts-central-keep-pressure.json'
  'classic_rts_central_keep_pressure_green'
  'rts_central_keep_pressure_live_input_gate'
  'rts_central_keep_pressure_inner_lane_dependency_gate'
  'rts_central_keep_pressure_route_gate'
  'rts_central_keep_pressure_shield_gate'
  'rts_central_keep_pressure_guard_gate'
  'rts_central_keep_pressure_siege_line_gate'
  'rts_central_keep_pressure_pressure_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS central keep readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS central keep pressure evidence remains connected to secured inner lane dependency, keep route, shield read, boss guard line, siege formation, pressure lock, and readiness"
