#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_expansion_counterattack.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_expansion_counterattack_v1'
  'bevy-classic-rts-expansion-counterattack.json'
  'bevy-classic-rts-expansion-counterattack.ppm'
  'classic-rts-expansion-counterattack'
  'input_path == "apply_live_native_action_with_source(classic_rts_expansion_counterattack_input)"'
  'RTS:QUEUE:expansion:claim:forest_relay@9,2'
  'RTS:QUEUE:expansion:build:relay_outpost@9,2'
  'RTS:QUEUE:expansion:workers:gold_line@9,2'
  'RTS:QUEUE:expansion:defend:counter_wave@8,3'
  'expansion_claim_gate == true'
  'expansion_build_gate == true'
  'expansion_worker_income_gate == true'
  'counterattack_gate == true'
  'defense_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS expansion script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_EXPANSION_COUNTERATTACK_CONTRACT'
  'native_classic_rts_expansion_counterattack_evidence_json'
  'classic-rts-expansion-counterattack'
  'classic_rts_expansion_counterattack_input'
  'rts_expansion_structure_ids'
  'rts_expansion_worker_unit_ids'
  'rts_expansion_income_per_minute'
  'rts_expansion_resource_log'
  'rts_enemy_counterattack_unit_ids'
  'rts_enemy_counterattack_route_tile_ids'
  'rts_expansion_defense_state'
  'CLASSIC_RTS_EXPANSION_BASE_COLOR'
  'CLASSIC_RTS_EXPANSION_WORKER_COLOR'
  'CLASSIC_RTS_EXPANSION_INCOME_COLOR'
  'CLASSIC_RTS_COUNTERATTACK_COLOR'
  'CLASSIC_RTS_EXPANSION_DEFENSE_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS expansion source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_expansion_counterattack.sh'
  'bevy-classic-rts-expansion-counterattack.json'
  'classic_rts_expansion_counterattack_green'
  'rts_expansion_counterattack_live_input_gate'
  'rts_expansion_counterattack_commander_dependency_gate'
  'rts_expansion_counterattack_claim_gate'
  'rts_expansion_counterattack_build_gate'
  'rts_expansion_counterattack_worker_income_gate'
  'rts_expansion_counterattack_counterattack_gate'
  'rts_expansion_counterattack_defense_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS expansion readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS expansion counterattack evidence remains connected to commander progression, second-base economy, enemy counter-wave, aura defense, and readiness"
