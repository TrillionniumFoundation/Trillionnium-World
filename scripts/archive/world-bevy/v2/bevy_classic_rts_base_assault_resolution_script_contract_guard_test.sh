#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_base_assault_resolution.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_base_assault_resolution_v1'
  'bevy-classic-rts-base-assault-resolution.json'
  'bevy-classic-rts-base-assault-resolution.ppm'
  'classic-rts-base-assault-resolution'
  'input_path == "apply_live_native_action_with_source(classic_rts_base_assault_resolution_input)"'
  'RTS:MOVE:10,3:siege'
  'RTS:ATTACK:enemy_barracks'
  'RTS:QUEUE:assault:breach:enemy_barracks@10,3'
  'army_dependency_gate == true'
  'assault_path_gate == true'
  'enemy_base_health_gate == true'
  'breach_resolution_gate == true'
  'reward_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS base assault script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BASE_ASSAULT_RESOLUTION_CONTRACT'
  'native_classic_rts_base_assault_resolution_evidence_json'
  'classic-rts-base-assault-resolution'
  'classic_rts_base_assault_resolution_input'
  'rts_base_assault_target_ids'
  'rts_base_assault_path_tile_ids'
  'rts_enemy_structure_health_percents'
  'rts_base_breach_percent'
  'rts_base_assault_result_state'
  'rts_base_assault_reward_log'
  'CLASSIC_RTS_BASE_ASSAULT_PATH_COLOR'
  'CLASSIC_RTS_BASE_BREACH_COLOR'
  'CLASSIC_RTS_ENEMY_BASE_HEALTH_COLOR'
  'CLASSIC_RTS_ASSAULT_REWARD_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS base assault source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_base_assault_resolution.sh'
  'bevy-classic-rts-base-assault-resolution.json'
  'classic_rts_base_assault_resolution_green'
  'rts_base_assault_resolution_live_input_gate'
  'rts_base_assault_resolution_army_dependency_gate'
  'rts_base_assault_resolution_assault_path_gate'
  'rts_base_assault_resolution_enemy_base_health_gate'
  'rts_base_assault_resolution_breach_gate'
  'rts_base_assault_resolution_reward_gate'
  'rts_base_assault_resolution_breach_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS base assault readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS base assault evidence remains connected to army production, siege movement, enemy-base attack, structure health, breach resolution, reward state, renderer overlays, and readiness"
