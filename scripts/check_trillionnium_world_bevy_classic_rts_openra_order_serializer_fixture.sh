#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-serializer-fixture.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-serializer-fixture"
STREAM="$PREVIEW_DIR/openra-order-stream-fixture.jsonl"
MANIFEST="$PREVIEW_DIR/openra-order-serializer-fixture.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-order-serializer-fixture "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_style_order_serializer_fixture_not_openra_order_stream"
  and .source_contracts.openra_command_vocab_adapter == "trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter_v1"
  and (.serializer_sha256 | type == "string" and length == 64)
  and (.manifest_sha256 | type == "string" and length == 64)
  and .serializer_summary.stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .serializer_summary.record_schema == "openra_order_stream_record_v1"
  and (.serializer_summary.source_command_adapter_payload_sha256 | type == "string" and length == 64)
  and (.serializer_summary.source_command_adapter_file_sha256 | type == "string" and length == 64)
  and (.serializer_summary.source_replay_sha256 | type == "string" and length == 64)
  and .serializer_summary.vocabulary_count >= 9
  and .serializer_summary.serialized_record_count >= 20
  and .serializer_summary.roundtrip_record_count == .serializer_summary.serialized_record_count
  and .serializer_summary.checkpoint_order_count >= 6
  and .serializer_summary.event_order_count >= 10
  and .serializer_summary.first_order == "StartGame"
  and .serializer_summary.final_frame >= 3000
  and .serializer_summary.winner == "Multi2"
  and .source_contract_gate == true
  and .source_green_gate == true
  and .source_serializer_input_gate == true
  and .serialized_record_count_gate == true
  and .serialized_vocabulary_gate == true
  and .serializer_file_gate == true
  and .manifest_gate == true
  and .roundtrip_parse_gate == true
  and .roundtrip_record_schema_gate == true
  and .roundtrip_sequence_gate == true
  and .roundtrip_frame_monotonic_gate == true
  and .roundtrip_payload_sha_gate == true
  and .roundtrip_gate == true
  and .compatibility_boundary_gate == true
  and .openra_order_serializer_fixture_gate == true
  and .bevy_openra_order_serializer_fixture_claimed == true
  and .bevy_openra_command_vocabulary_adapter_claimed == true
  and .bevy_openra_binary_replay_compatible == false
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

STREAM="$(jq -er '.serializer_path' "$SUMMARY")"
MANIFEST="$(jq -er '.manifest_path' "$SUMMARY")"
COMMAND_ADAPTER="$(jq -er '.source_paths.command_adapter_json' "$SUMMARY")"

jq -e '
  .manifest_schema == "openra_order_serializer_fixture_manifest_v1_json"
  and .stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .record_schema == "openra_order_stream_record_v1"
  and (.serializer_sha256 | type == "string" and length == 64)
  and (.source_command_adapter_file_sha256 | type == "string" and length == 64)
  and (.source_command_adapter_payload_sha256 | type == "string" and length == 64)
  and (.source_replay_sha256 | type == "string" and length == 64)
  and .record_count >= 20
  and .checkpoint_order_count >= 6
  and .event_order_count >= 10
  and .vocabulary_count >= 9
  and .first_order == "StartGame"
  and .final_frame >= 3000
  and .winner == "Multi2"
  and .compatibility.openra_style_order_stream_fixture == true
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_order_serializer_claimed == false
  and .compatibility.openra_network_order_stream_claimed == false
  and .compatibility.openra_runtime_parity_claimed == false
' "$MANIFEST" >/dev/null

jq -s -e '
  length >= 20
  and (to_entries | all(.[]; .value.sequence == .key))
  and all(.[]; .record_schema == "openra_order_stream_record_v1")
  and all(.[]; (.payload_sha256 | type == "string" and length == 64))
  and (.[0].order == "StartGame")
  and (map(.order) | index("SyncFrame") != null)
  and (map(.order) | index("BotOrder") != null)
  and (map(.order) | index("TerminalProbe") != null)
  and (map(.order) | index("GameOver") != null)
  and (map(.order) | index("ReplayOutcome") != null)
  and (map(.order) | index("Winner") != null)
  and (map(.order) | index("Losers") != null)
  and (map(.order) | index("Outcome") != null)
  and (map(select(.order == "Outcome")) | length) == 4
  and (map(select(.order == "Winner" and (.source_order.source_event | contains("Multi2")))) | length) == 1
  and (map(.frame) | max) >= 3000
' "$STREAM" >/dev/null

test -s "$STREAM"
test -s "$MANIFEST"
test -s "$COMMAND_ADAPTER"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_SERIALIZER_FIXTURE_GREEN %s %s %s\n' "$SUMMARY" "$STREAM" "$MANIFEST"
