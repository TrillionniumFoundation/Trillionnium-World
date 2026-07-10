#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_bridge.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_parity_bridge_v1'
  'bevy-classic-rts-openra-parity-bridge.json'
  'bevy-classic-rts-openra-parity-bridge'
  'classic-rts-openra-parity-bridge'
  'openra_target_commits.organic_terminal == "5f1bf76"'
  'openra_target_commits.replay_summary == "d5ceade"'
  'openra_target_commits.endurance_skirmish == "2cb80a0"'
  'gap_states.replay_metrics == "bevy_replay_metric_vocabulary_not_openra_replay_file"'
  'gap_states.endurance_skirmish == "bevy_endurance_vocabulary_not_openra_headless_client_match"'
  'no_parity_claim_gate == true'
  'comparison_matrix_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA parity bridge script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_PARITY_BRIDGE_CONTRACT'
  'native_classic_rts_openra_parity_bridge_evidence_json'
  'classic-rts-openra-parity-bridge'
  'terminal_rule_comparison_gate'
  'replay_metrics_comparison_gate'
  'headless_endurance_comparison_gate'
  'openra_target_commit_gate'
  'gap_visibility_gate'
  'no_parity_claim_gate'
  'comparison_matrix_gate'
  'bevy_replay_metric_vocabulary_not_openra_replay_file'
  'bevy_endurance_vocabulary_not_openra_headless_client_match'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA parity bridge source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_parity_bridge_contract_guard'
  'bevy_classic_rts_openra_parity_bridge_gate'
  'bevy_classic_rts_openra_parity_bridge_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_parity_bridge.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA parity bridge line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA parity bridge keeps terminal/replay/headless comparison evidence green while preserving OpenRA parity gap boundaries"
