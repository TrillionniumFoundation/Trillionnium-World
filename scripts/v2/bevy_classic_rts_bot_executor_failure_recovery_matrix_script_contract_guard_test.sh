#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1'
  'bevy-classic-rts-bot-executor-failure-recovery-matrix.json'
  'bevy-classic-rts-bot-executor-failure-recovery-matrix'
  'bot-executor-failure-recovery-matrix.matrix.json'
  'classic-rts-bot-executor-failure-recovery-matrix'
  'TRNM_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_SUMMARY'
  'TRNM_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_DIR'
  'bevy_executor_rejects_blocked_actions_and_recovers_without_command_queue_pollution_not_openra_runtime_bot'
  'blocked_injection_count == 6'
  'blocked_rejection_count == 6'
  'blocked_command_queue_unchanged_count == 6'
  'recovery_accepted_action_count == 6'
  'recovery_command_delta_match_count == 6'
  'rts_queue_id_required'
  'rts_group_id_required'
  'rts_attack_required_before_ability'
  'rts_invalid_tile:bad-tile'
  'rts_attack_target_required'
  'classic_rts_bot_executor_failure_recovery_matrix_blocked_input'
  'classic_rts_bot_executor_failure_recovery_matrix_recovery_input'
  'bot_executor_failure_recovery_matrix_gate == true'
  'bevy_bot_executor_failure_recovery_matrix_claimed == true'
  'bevy_openra_runtime_bot_executor_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot executor failure recovery matrix script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_EXECUTOR_FAILURE_RECOVERY_MATRIX_CONTRACT'
  'native_classic_rts_bot_executor_failure_recovery_matrix_evidence_json'
  'classic-rts-bot-executor-failure-recovery-matrix'
  'TRNM_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_SUMMARY'
  'TRNM_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_DIR'
  'bot-executor-failure-recovery-matrix.matrix.json'
  'bevy_executor_rejects_blocked_actions_and_recovers_without_command_queue_pollution_not_openra_runtime_bot'
  'classic_rts_bot_executor_failure_recovery_matrix_blocked_input'
  'classic_rts_bot_executor_failure_recovery_matrix_recovery_input'
  'empty_queue_rejected'
  'empty_group_rejected'
  'ability_before_attack_rejected'
  'invalid_tile_rejected'
  'blank_queue_rejected'
  'attack_target_missing_rejected'
  'blocked_rejection_gate'
  'blocked_non_pollution_gate'
  'recovery_acceptance_gate'
  'recovery_runtime_gate'
  'bot_executor_failure_recovery_matrix_gate'
  'bevy_bot_executor_failure_recovery_matrix_claimed'
  'bevy_openra_runtime_bot_executor_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot executor failure recovery matrix source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh'
  'TRNM_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_SUMMARY'
  'TRNM_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_DIR'
  'bevy_classic_rts_bot_executor_failure_recovery_matrix_contract_guard'
  'bevy_classic_rts_bot_executor_failure_recovery_matrix_gate'
  'bevy_classic_rts_bot_executor_failure_recovery_matrix_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI bot executor failure recovery matrix line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot executor failure/recovery matrix rejects blocked native actions and recovers without command queue pollution"
