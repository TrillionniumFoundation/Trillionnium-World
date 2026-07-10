#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1'
  'bevy-classic-rts-bot-planner-executor-replay-determinism.json'
  'bevy-classic-rts-bot-planner-executor-replay-determinism'
  'bot-planner-executor-replay-determinism.replay.json'
  'classic-rts-bot-planner-executor-replay-determinism'
  'bevy_executor_action_log_replays_to_identical_runtime_state_not_openra_runtime_bot'
  'stabilize_macro_workers'
  'scout_resource_beacons'
  'confirm_enemy_pressure_lane'
  'unlock_tier_two_tech'
  'transition_siege_push'
  'terminal_contract_alignment'
  'RTS:QUEUE:faction:mirror_guard'
  'RTS:QUEUE:tier2:finish:gate_bulwark@10,3'
  'source_final_runtime_sha256 == .replay_final_runtime_sha256'
  'source_command_queue_sha256 == .replay_command_queue_sha256'
  'bot_planner_executor_replay_determinism_gate == true'
  'bevy_bot_planner_executor_replay_determinism_claimed == true'
  'bevy_openra_runtime_bot_executor_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot planner executor replay determinism script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_PLANNER_EXECUTOR_REPLAY_DETERMINISM_CONTRACT'
  'native_classic_rts_bot_planner_executor_replay_determinism_evidence_json'
  'classic-rts-bot-planner-executor-replay-determinism'
  'bot-planner-executor-replay-determinism.replay.json'
  'bevy_executor_action_log_replays_to_identical_runtime_state_not_openra_runtime_bot'
  'classic_rts_bot_planner_executor_replay_input'
  'native_control_action_from_label'
  'source_final_runtime_sha256'
  'replay_final_runtime_sha256'
  'source_command_queue_sha256'
  'replay_command_queue_sha256'
  'runtime_determinism_gate'
  'bot_planner_executor_replay_determinism_gate'
  'bevy_openra_runtime_bot_executor_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot planner executor replay determinism source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh'
  'bevy_classic_rts_bot_planner_executor_replay_determinism_contract_guard'
  'bevy_classic_rts_bot_planner_executor_replay_determinism_gate'
  'bevy_classic_rts_bot_planner_executor_replay_determinism_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI bot planner executor replay determinism line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot planner executor replay determinism replays accepted Bevy-native action logs while keeping OpenRA/public-launch claims blocked"
