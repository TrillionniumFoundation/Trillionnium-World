#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_sequence.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_action_sequence_v1'
  'bevy-classic-rts-action-sequence.json'
  'bevy-classic-rts-action-sequence.ppm'
  'classic-rts-action-sequence'
  'idle_gate == true'
  'windup_gate == true'
  'strike_gate == true'
  'recovery_gate == true'
  'carry_up_gate == true'
  'carry_down_gate == true'
  'sequence_phase_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ACTION_SEQUENCE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS action sequence script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ACTION_SEQUENCE_CONTRACT'
  'native_classic_rts_action_sequence_evidence_json'
  'classic_rts_action_sequence_phase'
  'classic_draw_rts_action_sequence_marks'
  'CLASSIC_RTS_ACTION_SEQUENCE_IDLE_COLOR'
  'CLASSIC_RTS_ACTION_SEQUENCE_WINDUP_COLOR'
  'CLASSIC_RTS_ACTION_SEQUENCE_STRIKE_COLOR'
  'CLASSIC_RTS_ACTION_SEQUENCE_RECOVERY_COLOR'
  'CLASSIC_RTS_ACTION_SEQUENCE_CARRY_UP_COLOR'
  'CLASSIC_RTS_ACTION_SEQUENCE_CARRY_DOWN_COLOR'
  'Original Trillionnium action sequence overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS action sequence source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_action_sequence.sh'
  'bevy-classic-rts-action-sequence.json'
  'classic_rts_action_sequence_green'
  'rts_action_sequence_idle_gate'
  'rts_action_sequence_windup_gate'
  'rts_action_sequence_strike_gate'
  'rts_action_sequence_recovery_gate'
  'rts_action_sequence_carry_up_gate'
  'rts_action_sequence_carry_down_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS action sequence readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_action_sequence_v1'
  'bevy_classic_rts_action_sequence_contract_guard'
  'bevy_classic_rts_action_sequence_gate'
  'bevy_classic_rts_action_sequence_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_action_sequence.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS action sequence release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS action sequence evidence remains connected to renderer, readiness, release review, and original art policy"
