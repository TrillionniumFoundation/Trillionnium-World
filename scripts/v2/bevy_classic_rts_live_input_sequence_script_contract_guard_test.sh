#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_live_input_sequence.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_live_input_sequence_v1'
  'bevy-classic-rts-live-input-sequence.json'
  'bevy-classic-rts-live-input-sequence.ppm'
  'classic-rts-live-input-sequence'
  'input_path == "apply_live_native_action_with_source(classic_rts_live_input)"'
  'accepted_input_count == 5'
  'RTS:SELECT:1'
  'RTS:QUEUE:train:guard'
  'RTS:MOVE:7,4:diamond'
  'RTS:ATTACK:arena_creep_attack'
  'RTS:ABILITY:focus_fire'
  'live_input_gate == true'
  'selection_live_gate == true'
  'production_live_gate == true'
  'move_live_gate == true'
  'attack_live_gate == true'
  'ability_live_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS live input script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_INPUT_SEQUENCE_CONTRACT'
  'native_classic_rts_live_input_sequence_evidence_json'
  'classic-rts-live-input-sequence'
  'RtsSelectControlGroup'
  'RtsQueueProduction'
  'RtsMoveCommand'
  'RtsAttackCommand'
  'RtsAbilityCommand'
  'apply_live_native_action_with_source'
  'classic_rts_live_input'
  'apply_classic_rts_select_group_runtime'
  'apply_classic_rts_queue_runtime'
  'apply_classic_rts_move_runtime'
  'apply_classic_rts_attack_runtime'
  'apply_classic_rts_ability_runtime'
  'selection_live_gate'
  'production_live_gate'
  'move_live_gate'
  'attack_live_gate'
  'ability_live_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS live input source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_live_input_sequence.sh'
  'bevy-classic-rts-live-input-sequence.json'
  'classic_rts_live_input_sequence_green'
  'rts_live_input_live_input_gate'
  'rts_live_input_ability_live_gate'
  'rts_live_input_accepted_input_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS live input readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS live input sequence remains connected to native action input, evidence, and readiness"
