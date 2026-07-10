#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture_v1'
  'bevy-classic-rts-openra-order-serializer-fixture.json'
  'openra-order-stream-fixture.jsonl'
  'openra-order-serializer-fixture.json'
  'classic-rts-openra-order-serializer-fixture'
  '.serializer_path'
  '.manifest_path'
  '.source_paths.command_adapter_json'
  'openra_order_stream_fixture_v1_jsonl'
  'openra_order_stream_record_v1'
  'ReplayOutcome'
  'openra_order_serializer_fixture_gate == true'
  'bevy_openra_order_serializer_fixture_claimed == true'
  'bevy_openra_order_serializer_claimed == false'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA order serializer fixture script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_SERIALIZER_FIXTURE_CONTRACT'
  'native_classic_rts_openra_order_serializer_fixture_evidence_json'
  'TRNM_OPENRA_COMMAND_VOCAB_ADAPTER_SUMMARY'
  'TRNM_OPENRA_COMMAND_VOCAB_ADAPTER_DIR'
  'classic-rts-openra-order-serializer-fixture'
  'bevy_owned_openra_style_order_serializer_fixture_not_openra_order_stream'
  'openra_order_stream_fixture_v1_jsonl'
  'openra_order_stream_record_v1'
  'ReplayOutcome'
  'serialized_vocabulary_gate'
  'roundtrip_payload_sha_gate'
  'openra_order_serializer_fixture_gate'
  'bevy_openra_order_serializer_fixture_claimed'
  'bevy_openra_order_serializer_claimed'
  'bevy_openra_network_order_stream_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA order serializer fixture source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_order_serializer_fixture_contract_guard'
  'bevy_classic_rts_openra_order_serializer_fixture_gate'
  'TRNM_OPENRA_COMMAND_VOCAB_ADAPTER_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-command-vocab-adapter.json"'
  'TRNM_OPENRA_COMMAND_VOCAB_ADAPTER_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-command-vocab-adapter"'
  'bevy_classic_rts_openra_order_serializer_fixture_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA order serializer fixture line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA order serializer fixture writes deterministic JSONL orders and roundtrips without claiming OpenRA runtime/network parity"
