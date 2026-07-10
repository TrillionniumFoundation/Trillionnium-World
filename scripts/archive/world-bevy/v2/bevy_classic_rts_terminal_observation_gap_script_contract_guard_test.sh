#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_terminal_observation_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_terminal_observation_gap_v1'
  'bevy-classic-rts-terminal-observation-gap.json'
  'bevy-classic-rts-terminal-observation-gap.ppm'
  'classic-rts-terminal-observation-gap'
  'bevy_terminal_observation_vocabulary_not_natural_openra_match'
  'bevy_natural_terminal_parity_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_gap_not_closed_gate == true'
  'openra_terminal_readiness_target_commit == "174525a"'
  'openra_terminal_probe_target_commit == "bf42eb1"'
  'openra_strategic_terminal_target_commit == "9e08464"'
  'terminal_probe_game_over == true'
  'terminal_probe_controlled == false'
  'terminal_observation_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS terminal observation gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TERMINAL_OBSERVATION_GAP_CONTRACT'
  'native_classic_rts_terminal_observation_gap_evidence_json'
  'classic-rts-terminal-observation-gap'
  'readiness_rule_check'
  'terminal_observation_probe'
  'outcome_classification'
  'bevy_terminal_observation_vocabulary_not_natural_openra_match'
  'OPENRA_TERMINAL_READINESS_COMMIT'
  'OPENRA_TERMINAL_PROBE_COMMIT'
  'OPENRA_STRATEGIC_TERMINAL_COMMIT'
  'terminal_observation_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS terminal observation gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_terminal_observation_gap.sh'
  'bevy-classic-rts-terminal-observation-gap.json'
  'classic_rts_terminal_observation_gap_green'
  'rts_terminal_observation_gap_stage_count'
  'rts_terminal_observation_gap_openra_gap_not_closed_gate'
  'rts_terminal_observation_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS terminal observation gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS terminal observation gap evidence remains bound to OpenRA terminal readiness/probe/strategic victory while keeping Bevy parity unclaimed"
