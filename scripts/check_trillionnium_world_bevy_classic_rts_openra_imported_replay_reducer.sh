#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-reducer.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-reducer"
REDUCER="$PREVIEW_DIR/openra-imported-replay-reducer.json"
SNAPSHOTS="$PREVIEW_DIR/openra-imported-replay-snapshots.jsonl"
COMPARISON="$PREVIEW_DIR/openra-imported-replay-reducer-comparison.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-imported-replay-reducer-negative-corpus.json"
IMPORTED_STREAM="$PREVIEW_DIR/openra-order-payload-decoder/openra-order-payload-decoded-stream.jsonl"
PAYLOAD_DECODER="$PREVIEW_DIR/openra-order-payload-decoder/openra-order-payload-decoder.json"
PAYLOAD_MANIFEST="$PREVIEW_DIR/openra-order-payload-decoder/openra-order-payload-decoder-manifest.json"
IMPORTER="$PREVIEW_DIR/openra-order-payload-decoder/openra-replay-importer/openra-replay-importer.json"
BASELINE_REDUCER="$PREVIEW_DIR/openra-order-replay-reducer/openra-order-replay-reducer.json"
BASELINE_SNAPSHOTS="$PREVIEW_DIR/openra-order-replay-reducer/openra-order-replay-snapshots.jsonl"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-imported-replay-reducer "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_decoded_payload_stream_replayed_by_rust_reducer"
  and .source_contracts.openra_order_payload_decoder == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .source_contracts.openra_replay_importer == "trillionnium_world_bevy_classic_rts_openra_replay_importer_v1"
  and .source_contracts.openra_order_replay_reducer == "trillionnium_world_bevy_classic_rts_openra_order_replay_reducer_v1"
  and (.reducer_state_sha256 | type == "string" and length == 64)
  and (.snapshot_sha256 | type == "string" and length == 64)
  and (.comparison_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .imported_reducer_summary.stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .imported_reducer_summary.record_schema == "openra_order_stream_record_v1"
  and (.imported_reducer_summary.imported_stream_sha256 | type == "string" and length == 64)
  and .imported_reducer_summary.decoded_stream_sha256 == .imported_reducer_summary.imported_stream_sha256
  and (.imported_reducer_summary.source_replay_sha256 | type == "string" and length == 64)
  and .imported_reducer_summary.record_count >= 20
  and .imported_reducer_summary.snapshot_count >= 6
  and .imported_reducer_summary.baseline_snapshot_count == .imported_reducer_summary.snapshot_count
  and .imported_reducer_summary.final_frame >= 3000
  and .imported_reducer_summary.winner == "Multi2"
  and .imported_reducer_summary.loser_count == 3
  and .imported_reducer_summary.outcome_order_count == 4
  and (.imported_reducer_summary.trace_sha256 | type == "string" and length == 64)
  and .imported_reducer_summary.state_schema == "openra_order_stream_reducer_state_v1_json"
  and .imported_reducer_summary.phase == "replay_complete"
  and .imported_reducer_summary.last_stage == "replay_outcome_probe"
  and (.imported_reducer_summary.final_match_result_state | contains("Multi2"))
  and .imported_reducer_summary.comparison_count >= 12
  and .imported_reducer_summary.mismatch_count == 0
  and .imported_reducer_summary.negative_case_count >= 6
  and .imported_reducer_summary.detected_negative_case_count == .imported_reducer_summary.negative_case_count
  and .source_contract_gate == true
  and .source_green_gate == true
  and .imported_stream_input_gate == true
  and .record_parse_gate == true
  and .record_schema_gate == true
  and .sequence_gate == true
  and .frame_monotonic_gate == true
  and .record_payload_sha_gate == true
  and .imported_reducer_parse_gate == true
  and .imported_reducer_state_gate == true
  and .reducer_file_gate == true
  and .snapshot_file_gate == true
  and .comparison_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .openra_imported_replay_reducer_gate == true
  and .bevy_openra_imported_replay_reducer_claimed == true
  and .bevy_openra_order_payload_codec_claimed == true
  and .bevy_openra_order_payload_decoder_claimed == true
  and .bevy_openra_native_order_payload_decoder_claimed == false
  and .bevy_openra_replay_envelope_importer_claimed == true
  and .bevy_openra_order_replay_reducer_claimed == true
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
  .state_schema == "openra_order_stream_reducer_state_v1_json"
  and .phase == "replay_complete"
  and (.slots | length) == 4
  and .human_slot == "Multi0"
  and (.bot_slots | length) == 3
  and (.map_uid | type == "string" and length == 64)
  and (.rules_uid | type == "string" and length == 64)
  and .final_frame >= 3000
  and .last_stage == "replay_outcome_probe"
  and .last_probe_state == "replay"
  and .final_controlled_beacons == 2
  and .winner == "Multi2"
  and (.losers | length) == 3
  and (.outcomes | length) == 4
  and .order_counts.StartGame == 1
  and .order_counts.SyncFrame >= 4
  and .order_counts.BotOrder >= 4
  and .order_counts.TerminalProbe >= 4
  and .order_counts.GameOver >= 1
  and .order_counts.ReplayOutcome == 1
  and .order_counts.Winner == 1
  and .order_counts.Losers == 1
  and .order_counts.Outcome == 4
  and (.trace_sha256 | type == "string" and length == 64)
' "$REDUCER" >/dev/null

jq -s -e '
  length >= 6
  and all(.[]; .snapshot_schema == "openra_order_reducer_snapshot_v1")
  and (.[0].frame == 0)
  and (map(.stage) | index("room_boot") != null)
  and (map(.stage) | index("terminal_gameover_probe") != null)
  and (map(.stage) | index("replay_outcome_probe") != null)
  and (map(.controlled_beacons) | max) == 2
  and (.[length - 1].match_result_state | contains("Multi2"))
' "$SNAPSHOTS" >/dev/null

jq -e '
  .comparison_schema == "openra_imported_replay_reducer_comparison_v1_json"
  and .comparison_count >= 12
  and .mismatch_count == 0
  and all(.rows[]; .aligned == true)
  and (.rows | map(.field) | index("state_sha256") != null)
  and (.rows | map(.field) | index("snapshot_sha256") != null)
  and (.rows | map(.field) | index("trace_sha256") != null)
  and (.rows | map(.field) | index("snapshot_count") != null)
' "$COMPARISON" >/dev/null

jq -e '
  length >= 6
  and all(.[]; .detected == true)
  and (map(.case) | index("dropped_imported_record") != null)
  and (map(.case) | index("sequence_gap") != null)
  and (map(.case) | index("frame_regression") != null)
  and (map(.case) | index("payload_hash_mismatch") != null)
  and (map(.case) | index("record_schema_mismatch") != null)
  and (map(.case) | index("stream_sha_mismatch") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .decoder_schema == "openra_order_payload_decoder_v1_json"
  and .summary.decoded_record_count >= 20
  and .summary.decoded_stream_sha256 == .summary.source_stream_sha256
  and .summary.detected_negative_case_count == .summary.negative_case_count
' "$PAYLOAD_DECODER" >/dev/null

jq -e '
  .manifest_schema == "openra_order_payload_decoder_manifest_v1_json"
  and .codec_schema == "openra_order_payload_codec_v1_bin"
  and .compatibility.openra_style_order_payload_decoder == true
  and .compatibility.openra_native_order_payload_decoder_claimed == false
  and .compatibility.openra_binary_replay_compatible == false
' "$PAYLOAD_MANIFEST" >/dev/null

jq -s -e '
  length >= 20
  and (to_entries | all(.[]; .value.sequence == .key))
  and all(.[]; .record_schema == "openra_order_stream_record_v1")
  and (.[0].order == "StartGame")
  and (map(.order) | index("ReplayOutcome") != null)
  and (map(.order) | index("Outcome") != null)
  and (map(.frame) | max) >= 3000
' "$IMPORTED_STREAM" >/dev/null

cmp -s "$REDUCER" "$BASELINE_REDUCER"
cmp -s "$SNAPSHOTS" "$BASELINE_SNAPSHOTS"
test -s "$IMPORTER"
test -s "$PAYLOAD_DECODER"
test -s "$PAYLOAD_MANIFEST"
test -s "$BASELINE_REDUCER"
test -s "$BASELINE_SNAPSHOTS"
test -s "$REDUCER"
test -s "$SNAPSHOTS"
test -s "$COMPARISON"
test -s "$NEGATIVE_CORPUS"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REDUCER_GREEN %s %s %s\n' "$SUMMARY" "$REDUCER" "$COMPARISON"
