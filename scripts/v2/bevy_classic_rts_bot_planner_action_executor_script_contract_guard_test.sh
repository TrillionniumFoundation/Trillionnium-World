#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1'
  'bevy-classic-rts-bot-planner-action-executor.json'
  'bevy-classic-rts-bot-planner-action-executor'
  'bot-planner-action-executor.actions.json'
  'classic-rts-bot-planner-action-executor'
  'bevy_planner_decisions_execute_as_native_rts_actions_not_openra_runtime_bot'
  'stabilize_macro_workers'
  'scout_resource_beacons'
  'confirm_enemy_pressure_lane'
  'unlock_tier_two_tech'
  'transition_siege_push'
  'terminal_contract_alignment'
  'RTS:QUEUE:faction:mirror_guard'
  'RTS:QUEUE:tier2:finish:gate_bulwark@10,3'
  'bot_planner_action_executor_gate == true'
  'bevy_bot_planner_action_executor_claimed == true'
  'bevy_openra_runtime_bot_executor_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot planner action executor script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_PLANNER_ACTION_EXECUTOR_CONTRACT'
  'native_classic_rts_bot_planner_action_executor_evidence_json'
  'classic-rts-bot-planner-action-executor'
  'bot-planner-action-executor.actions.json'
  'bevy_planner_decisions_execute_as_native_rts_actions_not_openra_runtime_bot'
  'classic_rts_bot_planner_action_executor_input'
  'execute_faction_base'
  'execute_recon_sweep'
  'execute_beacon_claim'
  'execute_relay_foundry_unlock'
  'execute_gate_bulwark_push'
  'execute_final_gate_break'
  'action_log_sha256'
  'executor_acceptance_gate'
  'runtime_mutation_gate'
  'terminal_source_alignment_gate'
  'bot_planner_action_executor_gate'
  'bevy_openra_runtime_bot_executor_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot planner action executor source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh'
  'bevy_classic_rts_bot_planner_action_executor_contract_guard'
  'bevy_classic_rts_bot_planner_action_executor_gate'
  'bevy_classic_rts_bot_planner_action_executor_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI bot planner action executor line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot planner action executor translates planner decisions into accepted Bevy-native RTS actions while keeping OpenRA/public-launch claims blocked"
