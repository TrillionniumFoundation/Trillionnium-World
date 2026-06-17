#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_replay_reducer.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_order_replay_reducer_v1'
  'bevy-classic-rts-openra-order-replay-reducer.json'
  'openra-order-replay-reducer.json'
  'openra-order-replay-snapshots.jsonl'
  'classic-rts-openra-order-replay-reducer'
  '.reducer_path'
  '.snapshot_path'
  '.source_paths.serializer_jsonl'
  '.source_paths.serializer_manifest'
  'openra_order_stream_reducer_state_v1_json'
  'openra_order_reducer_snapshot_v1'
  'openra_order_replay_reducer_gate == true'
  'bevy_openra_order_replay_reducer_claimed == true'
  'bevy_openra_order_serializer_fixture_claimed == true'
  'bevy_openra_order_serializer_claimed == false'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA order replay reducer script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_REPLAY_REDUCER_CONTRACT'
  'native_classic_rts_openra_order_replay_reducer_evidence_json'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_SUMMARY'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_DIR'
  'classic-rts-openra-order-replay-reducer'
  'bevy_owned_openra_style_order_stream_replayed_by_rust_reducer'
  'openra_order_stream_reducer_state_v1_json'
  'openra_order_reducer_snapshot_v1'
  'replay_reducer_parse_gate'
  'record_payload_sha_gate'
  'replay_reducer_state_gate'
  'openra_order_replay_reducer_gate'
  'bevy_openra_order_replay_reducer_claimed'
  'bevy_openra_order_serializer_fixture_claimed'
  'bevy_openra_network_order_stream_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA order replay reducer source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_order_replay_reducer_contract_guard'
  'bevy_classic_rts_openra_order_replay_reducer_gate'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-serializer-fixture.json"'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-serializer-fixture"'
  'bevy_classic_rts_openra_order_replay_reducer_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_order_replay_reducer.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA order replay reducer line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA order replay reducer consumes deterministic JSONL orders and derives a Rust state digest without claiming OpenRA runtime/network parity"
