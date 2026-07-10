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
  '.preview_paths.owned_replay_file'
  '.preview_paths.openra_parity_bridge'
  '.preview_paths.natural_terminal_contract'
  '.preview_paths.planner_live_autonomous_bot_loop'
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
  'TRNM_OPENRA_LIKE_CORE_SUMMARY'
  'TRNM_OPENRA_PARITY_BRIDGE_SUMMARY'
  'TRNM_OPENRA_PARITY_BRIDGE_DIR'
  'TRNM_OWNED_REPLAY_FILE_SUMMARY'
  'TRNM_OWNED_REPLAY_FILE_PATH'
  'TRNM_HEADLESS_REPLAY_PLAYBACK_SUMMARY'
  'TRNM_NATURAL_TERMINAL_CONTRACT_SUMMARY'
  'TRNM_NATURAL_TERMINAL_CONTRACT_DIR'
  'TRNM_PLANNER_LIVE_AUTONOMOUS_BOT_LOOP_SUMMARY'
  'TRNM_PLANNER_LIVE_AUTONOMOUS_BOT_LOOP_DIR'
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
  'TRNM_OPENRA_LIKE_CORE_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-like-core.json"'
  'TRNM_OPENRA_PARITY_BRIDGE_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-bridge.json"'
  'TRNM_OPENRA_PARITY_BRIDGE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-bridge"'
  'TRNM_OWNED_REPLAY_FILE_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.json"'
  'TRNM_OWNED_REPLAY_FILE_PATH="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.trnm-replay.json"'
  'TRNM_HEADLESS_REPLAY_PLAYBACK_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-headless-replay-playback.json"'
  'TRNM_NATURAL_TERMINAL_CONTRACT_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-natural-terminal-contract.json"'
  'TRNM_NATURAL_TERMINAL_CONTRACT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-natural-terminal-contract"'
  'TRNM_PLANNER_LIVE_AUTONOMOUS_BOT_LOOP_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-planner-live-autonomous-bot-loop.json"'
  'TRNM_PLANNER_LIVE_AUTONOMOUS_BOT_LOOP_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-planner-live-autonomous-bot-loop"'
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
