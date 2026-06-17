#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness_v1'
  'bevy-classic-rts-openra-headless-comparison-harness.json'
  'openra-headless-comparison-harness.json'
  'openra-headless-comparison-mismatch-matrix.json'
  'classic-rts-openra-headless-comparison-harness'
  '.source_paths.reducer_state'
  '.source_paths.replay_summary_adapter'
  'openra_headless_comparison_harness_gate == true'
  'bevy_openra_headless_comparison_harness_claimed == true'
  'bevy_openra_order_replay_reducer_claimed == true'
  'bevy_openra_replay_summary_adapter_claimed == true'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
  'winner_mismatch_probe'
  'headless_wgpu_toggle_probe'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA headless comparison harness script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_HEADLESS_COMPARISON_HARNESS_CONTRACT'
  'native_classic_rts_openra_headless_comparison_harness_evidence_json'
  'TRNM_OPENRA_ORDER_REPLAY_REDUCER_SUMMARY'
  'TRNM_OPENRA_ORDER_REPLAY_REDUCER_DIR'
  'TRNM_OPENRA_REPLAY_COMPAT_ADAPTER_SUMMARY'
  'TRNM_OPENRA_REPLAY_COMPAT_ADAPTER_DIR'
  'classic-rts-openra-headless-comparison-harness'
  'bevy_owned_openra_style_reducer_compared_with_headless_replay_harness'
  'openra_headless_comparison_harness_v1_json'
  'comparison_alignment_gate'
  'headless_comparison_gate'
  'mismatch_matrix_gate'
  'openra_headless_comparison_harness_gate'
  'bevy_openra_headless_comparison_harness_claimed'
  'bevy_openra_order_replay_reducer_claimed'
  'bevy_openra_replay_summary_adapter_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA headless comparison harness source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_headless_comparison_harness_contract_guard'
  'bevy_classic_rts_openra_headless_comparison_harness_gate'
  'TRNM_OPENRA_ORDER_REPLAY_REDUCER_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-replay-reducer.json"'
  'TRNM_OPENRA_ORDER_REPLAY_REDUCER_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-replay-reducer"'
  'TRNM_OPENRA_REPLAY_COMPAT_ADAPTER_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-compat-adapter.json"'
  'TRNM_OPENRA_REPLAY_COMPAT_ADAPTER_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-compat-adapter"'
  'bevy_classic_rts_openra_headless_comparison_harness_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA headless comparison harness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA headless comparison harness aligns Rust order reducer state with headless replay summary and detects corrupted comparison probes"
