#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_cadence.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_action_cadence_v1'
  'bevy-classic-rts-action-cadence.json'
  'bevy-classic-rts-action-cadence.ppm'
  'classic-rts-action-cadence'
  'windup_gate == true'
  'strike_gate == true'
  'recovery_gate == true'
  'carry_bob_gate == true'
  'idle_breath_gate == true'
  'scene_renderer_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ACTION_CADENCE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS action cadence script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ACTION_CADENCE_CONTRACT'
  'native_classic_rts_action_cadence_evidence_json'
  'classic_draw_rts_action_cadence_marks'
  'CLASSIC_RTS_ACTION_CADENCE_WINDUP_COLOR'
  'CLASSIC_RTS_ACTION_CADENCE_STRIKE_COLOR'
  'CLASSIC_RTS_ACTION_CADENCE_RECOVERY_COLOR'
  'CLASSIC_RTS_ACTION_CADENCE_CARRY_BOB_COLOR'
  'CLASSIC_RTS_ACTION_CADENCE_IDLE_BREATH_COLOR'
  'CLASSIC_RTS_ACTION_CADENCE_SHADOW_SMEAR_COLOR'
  'Original Trillionnium action cadence marks'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS action cadence source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_action_cadence.sh'
  'bevy-classic-rts-action-cadence.json'
  'classic_rts_action_cadence_green'
  'rts_action_cadence_windup_gate'
  'rts_action_cadence_strike_gate'
  'rts_action_cadence_recovery_gate'
  'rts_action_cadence_carry_bob_gate'
  'rts_action_cadence_idle_breath_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS action cadence readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_action_cadence_v1'
  'bevy_classic_rts_action_cadence_contract_guard'
  'bevy_classic_rts_action_cadence_gate'
  'bevy_classic_rts_action_cadence_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_action_cadence.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS action cadence release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS action cadence evidence remains connected to renderer, readiness, release review, and original art policy"
