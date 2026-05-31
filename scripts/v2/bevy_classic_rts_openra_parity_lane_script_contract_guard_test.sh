#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_lane.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_parity_lane_v1'
  'bevy-classic-rts-openra-parity-lane.json'
  'bevy-classic-rts-openra-parity-lane'
  'classic-rts-openra-parity-lane'
  'rules_mod_vocabulary'
  'headless_replay_playback'
  'natural_terminal_contract'
  'bot_skirmish_loop'
  'bevy_openra_runtime_parity_claimed == false'
  'bevy_openra_live_bot_match_claimed == false'
  'openra_parity_lane_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA parity lane script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_PARITY_LANE_CONTRACT'
  'native_classic_rts_openra_parity_lane_evidence_json'
  'classic-rts-openra-parity-lane'
  'bevy_openra_parity_lane_v1_local_runtime_green_not_openra_runtime_parity'
  'rules_mod_vocabulary_gate'
  'owned_replay_lane_gate'
  'headless_playback_lane_gate'
  'natural_terminal_lane_gate'
  'bot_skirmish_lane_gate'
  'replay_headless_consistency_gate'
  'bevy_openra_runtime_parity_claimed'
  'bevy_openra_live_bot_match_claimed'
  'openra_parity_lane_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA parity lane source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_parity_lane_contract_guard'
  'bevy_classic_rts_openra_parity_lane_gate'
  'bevy_classic_rts_openra_parity_lane_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_parity_lane.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA parity lane line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA parity lane composes rules/replay/headless/terminal/bot evidence without claiming OpenRA runtime parity"
