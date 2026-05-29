#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_natural_terminal_contract.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_natural_terminal_contract_v1'
  'bevy-classic-rts-natural-terminal-contract.json'
  'bevy-classic-rts-natural-terminal-contract'
  'classic-rts-natural-terminal-contract'
  'bevy_natural_terminal_contract_v1_not_openra_natural_match'
  'control_2_of_4_flux_beacons_for_3000_ticks'
  'bevy_natural_terminal_contract_claimed == true'
  'bevy_openra_natural_terminal_match_claimed == false'
  'bevy_openra_headless_client_match_claimed == false'
  'natural_terminal_contract_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS natural terminal contract script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NATURAL_TERMINAL_CONTRACT'
  'native_classic_rts_natural_terminal_contract_evidence_json'
  'classic-rts-natural-terminal-contract'
  'bevy_natural_terminal_contract_v1_not_openra_natural_match'
  'terminal_outcome_contract_gate'
  'terminal_rule_contract_gate'
  'headless_terminal_gate'
  'no_openra_natural_terminal_claim_gate'
  'bevy_natural_terminal_contract_claimed'
  'bevy_openra_natural_terminal_match_claimed'
  'natural_terminal_contract_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS natural terminal contract source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_natural_terminal_contract_contract_guard'
  'bevy_classic_rts_natural_terminal_contract_gate'
  'bevy_classic_rts_natural_terminal_contract_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_natural_terminal_contract.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI natural terminal contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS natural terminal contract normalizes organic, observation, replay, and headless terminal outcomes without claiming OpenRA/public-launch readiness"
