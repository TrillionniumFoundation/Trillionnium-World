#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure_v1'
  'bevy-classic-rts-enemy-base-tech-pressure.json'
  'bevy-classic-rts-enemy-base-tech-pressure.ppm'
  'classic-rts-enemy-base-tech-pressure'
  'input_path == "apply_live_native_action_with_source(classic_rts_enemy_base_tech_pressure_input)"'
  'RTS:QUEUE:enemy:tech:shadow_lattice@enemy_barracks'
  'RTS:QUEUE:enemy:train:raider_wave@enemy_barracks'
  'RTS:QUEUE:counter:research:sentinel_lantern@signal_spire'
  'RTS:QUEUE:counter:fortify:watch_tower@7,4'
  'intel_dependency_gate == true'
  'enemy_tech_gate == true'
  'enemy_production_gate == true'
  'player_counter_gate == true'
  'defense_ready_gate == true'
  'pressure_warning_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS enemy-base tech-pressure script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENEMY_BASE_TECH_PRESSURE_CONTRACT'
  'native_classic_rts_enemy_base_tech_pressure_evidence_json'
  'classic-rts-enemy-base-tech-pressure'
  'classic_rts_enemy_base_tech_pressure_input'
  'rts_enemy_base_tech_ids'
  'rts_enemy_production_queue'
  'rts_enemy_pressure_wave_unit_ids'
  'rts_player_counter_tech_ids'
  'rts_player_defense_structure_ids'
  'rts_enemy_pressure_warning_percent'
  'rts_enemy_base_pressure_state'
  'CLASSIC_RTS_ENEMY_TECH_COLOR'
  'CLASSIC_RTS_ENEMY_PRODUCTION_COLOR'
  'CLASSIC_RTS_PLAYER_COUNTER_TECH_COLOR'
  'CLASSIC_RTS_DEFENSE_READY_COLOR'
  'CLASSIC_RTS_PRESSURE_WARNING_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS enemy-base tech-pressure source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure.sh'
  'bevy-classic-rts-enemy-base-tech-pressure.json'
  'classic_rts_enemy_base_tech_pressure_green'
  'rts_enemy_base_tech_pressure_live_input_gate'
  'rts_enemy_base_tech_pressure_intel_dependency_gate'
  'rts_enemy_base_tech_pressure_enemy_tech_gate'
  'rts_enemy_base_tech_pressure_enemy_production_gate'
  'rts_enemy_base_tech_pressure_player_counter_gate'
  'rts_enemy_base_tech_pressure_defense_ready_gate'
  'rts_enemy_base_tech_pressure_warning_gate'
  'rts_enemy_base_tech_pressure_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS enemy-base tech-pressure readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS enemy-base tech-pressure evidence remains connected to scout intel, enemy tech escalation, production pressure, counter research, defense readiness, renderer overlays, and readiness"
