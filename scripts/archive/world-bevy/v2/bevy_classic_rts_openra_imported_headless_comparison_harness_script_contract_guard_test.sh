#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1'
  'bevy-classic-rts-openra-imported-headless-comparison-harness.json'
  'openra-imported-headless-comparison-harness.json'
  'openra-imported-headless-comparison-mismatch-matrix.json'
  'classic-rts-openra-imported-headless-comparison-harness'
  'IMPORTED_REDUCER="$(jq -r '\''.source_paths.imported_reducer_state'\'' "$SUMMARY")"'
  'REPLAY_ADAPTER="$(jq -r '\''.source_paths.replay_summary_adapter'\'' "$SUMMARY")"'
  'openra_imported_headless_comparison_harness_v1_json'
  'openra_imported_headless_comparison_harness_gate == true'
  'bevy_openra_imported_headless_comparison_harness_claimed == true'
  'bevy_openra_imported_replay_reducer_claimed == true'
  'bevy_openra_order_payload_decoder_claimed == true'
  'bevy_openra_native_order_payload_decoder_claimed == false'
  'bevy_openra_replay_summary_adapter_claimed == true'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA imported headless comparison harness script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_HEADLESS_COMPARISON_HARNESS_CONTRACT'
  'native_classic_rts_openra_imported_headless_comparison_harness_evidence_json'
  'classic-rts-openra-imported-headless-comparison-harness'
  'bevy_owned_openra_imported_reducer_compared_with_headless_replay_harness'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REDUCER_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_PAYLOAD_DECODER_CONTRACT'
  'openra_imported_headless_comparison_harness_v1_json'
  'imported_headless_comparison_gate'
  'openra_imported_headless_comparison_harness_gate'
  'TRNM_OPENRA_IMPORTED_REPLAY_REDUCER_SUMMARY'
  'TRNM_OPENRA_REPLAY_COMPAT_ADAPTER_SUMMARY'
  'bevy_openra_imported_headless_comparison_harness_claimed'
  'bevy_openra_imported_replay_reducer_claimed'
  'bevy_openra_order_payload_decoder_claimed'
  'bevy_openra_runtime_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA imported headless comparison harness source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_imported_headless_comparison_harness_contract_guard'
  'bevy_classic_rts_openra_imported_headless_comparison_harness_gate'
  'TRNM_OPENRA_IMPORTED_REPLAY_REDUCER_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-reducer.json"'
  'TRNM_OPENRA_REPLAY_COMPAT_ADAPTER_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-compat-adapter.json"'
  'bevy_classic_rts_openra_imported_headless_comparison_harness_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA imported headless comparison harness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA imported headless comparison harness aligns decoded imported reducer evidence with the headless replay summary without claiming OpenRA runtime/network parity"
