#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-payload-decoder.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-payload-decoder"
PAYLOAD_CODEC="$PREVIEW_DIR/openra-order-payload-codec.bin"
DECODED_STREAM="$PREVIEW_DIR/openra-order-payload-decoded-stream.jsonl"
DECODER="$PREVIEW_DIR/openra-order-payload-decoder.json"
MANIFEST="$PREVIEW_DIR/openra-order-payload-decoder-manifest.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-order-payload-decoder-negative-corpus.json"
IMPORTER_STREAM="$PREVIEW_DIR/openra-replay-importer/openra-replay-imported-order-stream.jsonl"
IMPORTER="$PREVIEW_DIR/openra-replay-importer/openra-replay-importer.json"
METADATA="$PREVIEW_DIR/openra-replay-importer/openra-replay-imported-metadata.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-order-payload-decoder "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_style_order_payload_codec_decoder_not_native_orderio"
  and .source_contracts.openra_replay_importer == "trillionnium_world_bevy_classic_rts_openra_replay_importer_v1"
  and (.payload_codec_sha256 | type == "string" and length == 64)
  and (.decoded_stream_sha256 | type == "string" and length == 64)
  and (.decoder_sha256 | type == "string" and length == 64)
  and (.manifest_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .decoder_summary.decoder_schema == "openra_order_payload_decoder_v1_json"
  and .decoder_summary.codec_schema == "openra_order_payload_codec_v1_bin"
  and .decoder_summary.source_stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .decoder_summary.record_schema == "openra_order_stream_record_v1"
  and .decoder_summary.source_record_count >= 20
  and .decoder_summary.decoded_record_count == .decoder_summary.source_record_count
  and .decoder_summary.decoded_stream_sha256 == .decoder_summary.source_stream_sha256
  and (.decoder_summary.payload_codec_sha256 | type == "string" and length == 64)
  and (.decoder_summary.source_replay_sha256 | type == "string" and length == 64)
  and .decoder_summary.vocabulary_count >= 9
  and .decoder_summary.final_frame >= 3000
  and .decoder_summary.winner == "Multi2"
  and .decoder_summary.negative_case_count >= 8
  and .decoder_summary.detected_negative_case_count == .decoder_summary.negative_case_count
  and .source_contract_gate == true
  and .source_green_gate == true
  and .record_parse_gate == true
  and .source_stream_parse_gate == true
  and .valid_payload_decode_gate == true
  and .payload_codec_file_gate == true
  and .manifest_gate == true
  and .decoded_stream_gate == true
  and .decoder_file_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .openra_order_payload_decoder_gate == true
  and .bevy_openra_order_payload_codec_claimed == true
  and .bevy_openra_order_payload_decoder_claimed == true
  and .bevy_openra_native_order_payload_decoder_claimed == false
  and .bevy_openra_replay_envelope_importer_claimed == true
  and .bevy_openra_binary_replay_compatible == false
  and .bevy_openra_order_serializer_claimed == false
  and .bevy_openra_network_order_stream_claimed == false
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_runtime_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .manifest_schema == "openra_order_payload_decoder_manifest_v1_json"
  and .codec_schema == "openra_order_payload_codec_v1_bin"
  and .decoder_schema == "openra_order_payload_decoder_v1_json"
  and .magic == "TRNMOPR1"
  and .codec_version == 1
  and .source_stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .source_record_schema == "openra_order_stream_record_v1"
  and (.source_stream_sha256 | type == "string" and length == 64)
  and .decoded_stream_sha256 == .source_stream_sha256
  and (.payload_codec_sha256 | type == "string" and length == 64)
  and .record_count >= 20
  and (.payload_record_layout | length) == 7
  and .order_tag_map.StartGame == 1
  and .order_tag_map.ReplayOutcome == 6
  and .order_tag_map.Outcome == 9
  and .source_kind_tag_map.start_game == 1
  and .source_kind_tag_map.checkpoint == 2
  and .source_kind_tag_map.event == 3
  and .compatibility.openra_style_order_payload_codec == true
  and .compatibility.openra_style_order_payload_decoder == true
  and .compatibility.openra_native_order_payload_decoder_claimed == false
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_network_order_stream_claimed == false
  and .compatibility.openra_runtime_parity_claimed == false
' "$MANIFEST" >/dev/null

jq -e '
  .decoder_schema == "openra_order_payload_decoder_v1_json"
  and (.summary.payload_codec_sha256 | type == "string" and length == 64)
  and .summary.decoded_record_count == .summary.source_record_count
  and .summary.decoded_stream_sha256 == .summary.source_stream_sha256
  and .summary.final_frame >= 3000
  and .summary.winner == "Multi2"
  and .summary.detected_negative_case_count == .summary.negative_case_count
' "$DECODER" >/dev/null

jq -s -e '
  length >= 20
  and (to_entries | all(.[]; .value.sequence == .key))
  and all(.[]; .record_schema == "openra_order_stream_record_v1")
  and (.[0].order == "StartGame")
  and (map(.order) | index("ReplayOutcome") != null)
  and (map(.order) | index("Outcome") != null)
  and (map(.frame) | max) >= 3000
' "$DECODED_STREAM" >/dev/null

jq -e '
  length >= 8
  and all(.[]; .detected == true)
  and (map(.case) | index("bad_magic") != null)
  and (map(.case) | index("unsupported_version") != null)
  and (map(.case) | index("truncated_header") != null)
  and (map(.case) | index("payload_length_overrun") != null)
  and (map(.case) | index("sequence_mismatch") != null)
  and (map(.case) | index("frame_regression") != null)
  and (map(.case) | index("order_tag_mismatch") != null)
  and (map(.case) | index("payload_sha_mismatch") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .metadata_schema == "openra_replay_envelope_metadata_v1_json"
  and .compatibility.openra_outer_replay_envelope_imported == true
  and .compatibility.openra_order_payload_decoder_claimed == false
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_runtime_parity_claimed == false
' "$METADATA" >/dev/null

jq -e '
  .importer_schema == "openra_replay_envelope_importer_v1_json"
  and .summary.imported_stream_sha256 == .summary.source_stream_sha256
' "$IMPORTER" >/dev/null

cmp -s "$IMPORTER_STREAM" "$DECODED_STREAM"
test -s "$PAYLOAD_CODEC"
test -s "$DECODED_STREAM"
test -s "$DECODER"
test -s "$MANIFEST"
test -s "$NEGATIVE_CORPUS"
test -s "$IMPORTER"
test -s "$METADATA"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_PAYLOAD_DECODER_GREEN %s %s %s\n' "$SUMMARY" "$PAYLOAD_CODEC" "$DECODED_STREAM"
