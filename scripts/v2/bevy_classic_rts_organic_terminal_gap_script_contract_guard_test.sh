#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_organic_terminal_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_organic_terminal_gap_v1'
  'bevy-classic-rts-organic-terminal-gap.json'
  'bevy-classic-rts-organic-terminal-gap.ppm'
  'classic-rts-organic-terminal-gap'
  'bevy_deterministic_observation_not_openra_natural_gameover'
  'bevy_natural_gameover_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_gap_not_closed_gate == true'
  'openra_parity_target_commit == "5f1bf76"'
  'terminal_probe_game_over == true'
  'normal_match_winner_claimed == false'
  'Replay:SurrenderAbsent'
  'organic_terminal_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS organic terminal gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ORGANIC_TERMINAL_GAP_CONTRACT'
  'native_classic_rts_organic_terminal_gap_evidence_json'
  'classic-rts-organic-terminal-gap'
  'terminal_gameover_probe'
  'replay_outcome_probe'
  'bevy_deterministic_observation_not_openra_natural_gameover'
  'terminal_probe_game_over'
  'normal_match_winner_claimed'
  'Replay:SurrenderAbsent'
  'organic_terminal_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS organic terminal gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_organic_terminal_gap.sh'
  'bevy-classic-rts-organic-terminal-gap.json'
  'classic_rts_organic_terminal_gap_green'
  'rts_organic_terminal_gap_stage_count'
  'rts_organic_terminal_gap_openra_gap_not_closed_gate'
  'rts_organic_terminal_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS organic terminal gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS organic terminal gap evidence remains bound to OpenRA natural GameOver/replay target while keeping Bevy parity unclaimed"
