#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_importer.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_replay_importer_v1'
  'bevy-classic-rts-openra-replay-importer.json'
  'openra-replay-envelope-importer.orarep'
  'openra-replay-imported-metadata.json'
  'openra-replay-imported-order-stream.jsonl'
  'openra-replay-importer-negative-corpus.json'
  'classic-rts-openra-replay-importer'
  '.envelope_path'
  '.metadata_path'
  '.imported_stream_path'
  '.importer_path'
  '.negative_corpus_path'
  '.source_paths.serializer_jsonl'
  '.source_paths.serializer_manifest'
  'openra_replay_envelope_importer_v1_json'
  'openra_replay_envelope_metadata_v1_json'
  'openra_replay_outer_packet_v1'
  'openra_outer_replay_envelope_imported == true'
  'openra_order_payload_decoder_claimed == false'
  'openra_binary_replay_compatible == false'
  'openra_replay_file_claimed == false'
  'openra_runtime_parity_claimed == false'
  'openra_replay_importer_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA replay importer script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_REPLAY_IMPORTER_CONTRACT'
  'native_classic_rts_openra_replay_importer_evidence_json'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_SUMMARY'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_DIR'
  'classic-rts-openra-replay-importer'
  'bevy_owned_openra_outer_replay_envelope_importer_not_full_binary_replay'
  'openra_replay_envelope_importer_v1_json'
  'openra_replay_envelope_metadata_v1_json'
  'OpenRA.Game/FileFormats/ReplayMetadata.cs'
  'OpenRA.Game/Network/ReplayConnection.cs'
  'OpenRA.Game/Network/ReplayRecorder.cs'
  'OpenRA.Game/Network/OrderIO.cs'
  'metadata_reader_gate'
  'outer_packet_gate'
  'imported_stream_gate'
  'negative_corpus_gate'
  'openra_replay_importer_gate'
  'bevy_openra_replay_envelope_importer_claimed'
  'bevy_openra_order_payload_decoder_claimed'
  'bevy_openra_binary_replay_compatible'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA replay importer source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_replay_importer_contract_guard'
  'bevy_classic_rts_openra_replay_importer_gate'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-serializer-fixture.json"'
  'TRNM_OPENRA_ORDER_SERIALIZER_FIXTURE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-serializer-fixture"'
  'bevy_classic_rts_openra_replay_importer_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_replay_importer.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA replay importer line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA replay importer reads an OpenRA outer replay envelope and metadata markers without claiming full binary/runtime parity"
