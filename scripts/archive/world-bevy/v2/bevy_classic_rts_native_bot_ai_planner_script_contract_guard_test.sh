#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_native_bot_ai_planner.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_native_bot_ai_planner_v1'
  'bevy-classic-rts-native-bot-ai-planner.json'
  'bevy-classic-rts-native-bot-ai-planner'
  'classic-rts-native-bot-ai-planner'
  'bevy_native_bot_ai_planner_v1_macro_intel_tech_closed_not_openra_bot_parity'
  'scout_resource_beacons'
  'stabilize_macro_workers'
  'confirm_enemy_pressure_lane'
  'unlock_tier_two_tech'
  'transition_siege_push'
  'terminal_contract_alignment'
  'strategy_checksum_sha256'
  'owned_replay_file_gate == true'
  'native_bot_ai_planner_gate == true'
  'bevy_native_bot_ai_planner_claimed == true'
  'bevy_openra_bot_ai_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS native bot AI planner script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NATIVE_BOT_AI_PLANNER_CONTRACT'
  'native_classic_rts_native_bot_ai_planner_evidence_json'
  'classic-rts-native-bot-ai-planner'
  'bevy_native_bot_ai_planner_v1_macro_intel_tech_closed_not_openra_bot_parity'
  'scout_resource_beacons'
  'stabilize_macro_workers'
  'confirm_enemy_pressure_lane'
  'unlock_tier_two_tech'
  'transition_siege_push'
  'terminal_contract_alignment'
  'strategy_checksum_sha256'
  'owned_replay_file_gate'
  'macro_economy_phase_gate'
  'map_intel_phase_gate'
  'tech_transition_phase_gate'
  'terminal_contract_gate'
  'native_bot_ai_planner_gate'
  'bevy_openra_bot_ai_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS native bot AI planner source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_native_bot_ai_planner.sh'
  'bevy_classic_rts_native_bot_ai_planner_contract_guard'
  'bevy_classic_rts_native_bot_ai_planner_gate'
  'bevy_classic_rts_native_bot_ai_planner_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_rts_native_bot_ai_planner_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI native bot AI planner line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS native bot AI planner composes macro economy, map intel, tech transition, and terminal contract evidence without claiming OpenRA bot parity or public-launch readiness"
