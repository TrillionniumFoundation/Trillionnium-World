#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_payload_decoder.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1'
  'bevy-classic-rts-openra-order-payload-decoder.json'
  'openra-order-payload-codec.bin'
  'openra-order-payload-decoded-stream.jsonl'
  'openra-order-payload-decoder-manifest.json'
  'openra-order-payload-decoder-negative-corpus.json'
  'classic-rts-openra-order-payload-decoder'
  'openra_order_payload_decoder_v1_json'
  'openra_order_payload_codec_v1_bin'
  'openra_style_order_payload_codec == true'
  'openra_style_order_payload_decoder == true'
  'openra_native_order_payload_decoder_claimed == false'
  'openra_binary_replay_compatible == false'
  'openra_network_order_stream_claimed == false'
  'openra_runtime_parity_claimed == false'
  'openra_order_payload_decoder_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA order payload decoder script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_PAYLOAD_DECODER_CONTRACT'
  'native_classic_rts_openra_order_payload_decoder_evidence_json'
  'classic-rts-openra-order-payload-decoder'
  'bevy_owned_openra_style_order_payload_codec_decoder_not_native_orderio'
  'openra_order_payload_decoder_v1_json'
  'openra_order_payload_codec_v1_bin'
  'TRNMOPR1'
  'payload_codec_file_gate'
  'decoded_stream_gate'
  'manifest_gate'
  'negative_corpus_gate'
  'openra_order_payload_decoder_gate'
  'bevy_openra_order_payload_codec_claimed'
  'bevy_openra_order_payload_decoder_claimed'
  'bevy_openra_native_order_payload_decoder_claimed'
  'bevy_openra_runtime_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA order payload decoder source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_order_payload_decoder_contract_guard'
  'bevy_classic_rts_openra_order_payload_decoder_gate'
  'bevy_classic_rts_openra_order_payload_decoder_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_order_payload_decoder.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA order payload decoder line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA order payload decoder bridges importer payload bytes to decoded JSONL without claiming native OpenRA OrderIO/runtime parity"
