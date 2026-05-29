#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1'
  'bevy-classic-rts-multi-match-bot-executor-evaluation.json'
  'bevy-classic-rts-multi-match-bot-executor-evaluation'
  'multi-match-bot-executor-evaluation.matches.json'
  'classic-rts-multi-match-bot-executor-evaluation'
  'bevy_executor_action_log_runs_across_multiple_deterministic_match_variants_not_openra_ladder'
  'forest_relay'
  'ridge_watch'
  'marsh_gate'
  'market_ruins'
  'total_replay_action_count == 24'
  'runtime_sha_match_count == 4'
  'command_queue_sha_match_count == 4'
  'classic_rts_multi_match_bot_executor_evaluation_input'
  'multi_match_bot_executor_evaluation_gate == true'
  'bevy_multi_match_bot_executor_evaluation_claimed == true'
  'bevy_openra_runtime_bot_executor_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS multi-match bot executor evaluation script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_CONTRACT'
  'native_classic_rts_multi_match_bot_executor_evaluation_evidence_json'
  'classic-rts-multi-match-bot-executor-evaluation'
  'multi-match-bot-executor-evaluation.matches.json'
  'bevy_executor_action_log_runs_across_multiple_deterministic_match_variants_not_openra_ladder'
  'classic_rts_multi_match_bot_executor_evaluation_input'
  'seed_2026052901_forest_relay'
  'seed_2026052902_ridge_watch'
  'seed_2026052903_marsh_gate'
  'seed_2026052904_market_ruins'
  'variant_diversity_gate'
  'multi_match_acceptance_gate'
  'multi_match_runtime_gate'
  'multi_match_bot_executor_evaluation_gate'
  'bevy_multi_match_bot_executor_evaluation_claimed'
  'bevy_openra_runtime_bot_executor_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS multi-match bot executor evaluation source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh'
  'bevy_classic_rts_multi_match_bot_executor_evaluation_contract_guard'
  'bevy_classic_rts_multi_match_bot_executor_evaluation_gate'
  'bevy_classic_rts_multi_match_bot_executor_evaluation_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI multi-match bot executor evaluation line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS multi-match bot executor evaluation runs the Bevy executor action log across deterministic variants while keeping OpenRA/public-launch claims blocked"
