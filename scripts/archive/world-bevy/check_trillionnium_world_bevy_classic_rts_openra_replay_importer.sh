#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-importer.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-importer"
ENVELOPE="$PREVIEW_DIR/openra-replay-envelope-importer.orarep"
METADATA="$PREVIEW_DIR/openra-replay-imported-metadata.json"
IMPORTED_STREAM="$PREVIEW_DIR/openra-replay-imported-order-stream.jsonl"
IMPORTER="$PREVIEW_DIR/openra-replay-importer.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-replay-importer-negative-corpus.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-replay-importer "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_replay_importer_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_outer_replay_envelope_importer_not_full_binary_replay"
  and .source_contracts.openra_order_serializer_fixture == "trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture_v1"
  and (.envelope_sha256 | type == "string" and length == 64)
  and (.importer_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .importer_summary.importer_schema == "openra_replay_envelope_importer_v1_json"
  and .importer_summary.metadata_schema == "openra_replay_envelope_metadata_v1_json"
  and .importer_summary.extension == ".orarep"
  and .importer_summary.meta_start_marker == -1
  and .importer_summary.meta_end_marker == -2
  and .importer_summary.meta_version == 1
  and .importer_summary.source_stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .importer_summary.source_record_schema == "openra_order_stream_record_v1"
  and .importer_summary.source_record_count >= 20
  and .importer_summary.imported_record_count == .importer_summary.source_record_count
  and .importer_summary.imported_packet_count == .importer_summary.source_record_count
  and (.importer_summary.source_stream_sha256 | type == "string" and length == 64)
  and .importer_summary.imported_stream_sha256 == .importer_summary.source_stream_sha256
  and (.importer_summary.source_replay_sha256 | type == "string" and length == 64)
  and .importer_summary.vocabulary_count >= 9
  and .importer_summary.final_frame >= 3000
  and .importer_summary.winner == "Multi2"
  and .importer_summary.negative_case_count >= 6
  and .importer_summary.detected_negative_case_count == .importer_summary.negative_case_count
  and .source_contract_gate == true
  and .source_green_gate == true
  and .record_parse_gate == true
  and .source_sequence_gate == true
  and .source_frame_monotonic_gate == true
  and .source_payload_sha_gate == true
  and .source_stream_parse_gate == true
  and .valid_envelope_parse_gate == true
  and .metadata_reader_gate == true
  and .outer_packet_gate == true
  and .imported_stream_gate == true
  and .negative_corpus_gate == true
  and .importer_file_gate == true
  and .compatibility_boundary_gate == true
  and .openra_replay_importer_gate == true
  and .bevy_openra_replay_envelope_importer_claimed == true
  and .bevy_openra_replay_metadata_reader_claimed == true
  and .bevy_openra_order_serializer_fixture_claimed == true
  and .bevy_openra_command_vocabulary_adapter_claimed == true
  and .bevy_openra_binary_replay_compatible == false
  and .bevy_openra_order_payload_decoder_claimed == false
  and .bevy_openra_order_serializer_claimed == false
  and .bevy_openra_network_order_stream_claimed == false
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_runtime_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

ENVELOPE="$(jq -er '.envelope_path' "$SUMMARY")"
METADATA="$(jq -er '.metadata_path' "$SUMMARY")"
IMPORTED_STREAM="$(jq -er '.imported_stream_path' "$SUMMARY")"
IMPORTER="$(jq -er '.importer_path' "$SUMMARY")"
NEGATIVE_CORPUS="$(jq -er '.negative_corpus_path' "$SUMMARY")"
SOURCE_STREAM="$(jq -er '.source_paths.serializer_jsonl' "$SUMMARY")"
SOURCE_MANIFEST="$(jq -er '.source_paths.serializer_manifest' "$SUMMARY")"

jq -e '
  .metadata_schema == "openra_replay_envelope_metadata_v1_json"
  and .extension == ".orarep"
  and .meta_start_marker == -1
  and .meta_end_marker == -2
  and .meta_version == 1
  and (.outer_record_layout | length) == 3
  and (.openra_reference_files | length) >= 4
  and .source_stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .source_record_schema == "openra_order_stream_record_v1"
  and .source_record_count >= 20
  and (.source_stream_sha256 | type == "string" and length == 64)
  and (.source_replay_sha256 | type == "string" and length == 64)
  and .vocabulary_count >= 9
  and .final_frame >= 3000
  and .winner == "Multi2"
  and .compatibility.openra_outer_replay_envelope_imported == true
  and .compatibility.openra_metadata_marker_reader == true
  and .compatibility.openra_order_payload_decoder_claimed == false
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_replay_file_claimed == false
  and .compatibility.openra_network_order_stream_claimed == false
  and .compatibility.openra_runtime_parity_claimed == false
' "$METADATA" >/dev/null

jq -e '
  .importer_schema == "openra_replay_envelope_importer_v1_json"
  and (.envelope_sha256 | type == "string" and length == 64)
  and .summary.imported_packet_count == .summary.source_record_count
  and .summary.imported_record_count == .summary.source_record_count
  and .summary.imported_stream_sha256 == .summary.source_stream_sha256
  and .summary.meta_start_marker == -1
  and .summary.meta_end_marker == -2
  and .summary.meta_version == 1
  and .summary.final_frame >= 3000
  and .summary.winner == "Multi2"
  and .summary.detected_negative_case_count == .summary.negative_case_count
  and (.packet_summary | length) == .summary.imported_packet_count
  and all(.packet_summary[]; .packet_schema == "openra_replay_outer_packet_v1" and (.packet_sha256 | length == 64))
' "$IMPORTER" >/dev/null

jq -s -e '
  length >= 20
  and (to_entries | all(.[]; .value.sequence == .key))
  and all(.[]; .record_schema == "openra_order_stream_record_v1")
  and (.[0].order == "StartGame")
  and (map(.order) | index("ReplayOutcome") != null)
  and (map(.order) | index("Outcome") != null)
  and (map(.frame) | max) >= 3000
' "$IMPORTED_STREAM" >/dev/null

jq -e '
  length >= 6
  and all(.[]; .detected == true)
  and (map(.case) | index("bad_meta_end_marker") != null)
  and (map(.case) | index("bad_meta_start_marker") != null)
  and (map(.case) | index("unsupported_meta_version") != null)
  and (map(.case) | index("metadata_length_overflow") != null)
  and (map(.case) | index("packet_frame_regression") != null)
  and (map(.case) | index("packet_payload_hash_mismatch") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .manifest_schema == "openra_order_serializer_fixture_manifest_v1_json"
  and .stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .record_schema == "openra_order_stream_record_v1"
  and .compatibility.openra_style_order_stream_fixture == true
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_network_order_stream_claimed == false
' "$SOURCE_MANIFEST" >/dev/null

cmp -s "$SOURCE_STREAM" "$IMPORTED_STREAM"
test -s "$ENVELOPE"
test -s "$METADATA"
test -s "$IMPORTED_STREAM"
test -s "$IMPORTER"
test -s "$NEGATIVE_CORPUS"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_REPLAY_IMPORTER_GREEN %s %s %s\n' "$SUMMARY" "$ENVELOPE" "$IMPORTED_STREAM"
