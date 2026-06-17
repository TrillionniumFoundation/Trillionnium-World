#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-replay-reducer.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-order-replay-reducer"
REDUCER="$PREVIEW_DIR/openra-order-replay-reducer.json"
SNAPSHOTS="$PREVIEW_DIR/openra-order-replay-snapshots.jsonl"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-order-replay-reducer "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_order_replay_reducer_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_style_order_stream_replayed_by_rust_reducer"
  and .source_contracts.openra_order_serializer_fixture == "trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture_v1"
  and (.reducer_state_sha256 | type == "string" and length == 64)
  and (.snapshot_sha256 | type == "string" and length == 64)
  and .reducer_summary.stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .reducer_summary.record_schema == "openra_order_stream_record_v1"
  and (.reducer_summary.source_stream_sha256 | type == "string" and length == 64)
  and (.reducer_summary.source_manifest_file_sha256 | type == "string" and length == 64)
  and .reducer_summary.record_count >= 20
  and .reducer_summary.snapshot_count >= 6
  and .reducer_summary.final_frame >= 3000
  and .reducer_summary.winner == "Multi2"
  and .reducer_summary.loser_count == 3
  and .reducer_summary.outcome_order_count == 4
  and (.reducer_summary.trace_sha256 | type == "string" and length == 64)
  and .reducer_summary.state_schema == "openra_order_stream_reducer_state_v1_json"
  and .reducer_summary.phase == "replay_complete"
  and .reducer_summary.last_stage == "replay_outcome_probe"
  and (.reducer_summary.final_match_result_state | contains("Multi2"))
  and .source_contract_gate == true
  and .source_green_gate == true
  and .source_stream_input_gate == true
  and .record_parse_gate == true
  and .record_schema_gate == true
  and .sequence_gate == true
  and .frame_monotonic_gate == true
  and .record_payload_sha_gate == true
  and .replay_reducer_parse_gate == true
  and .replay_reducer_state_gate == true
  and .reducer_file_gate == true
  and .snapshot_file_gate == true
  and .compatibility_boundary_gate == true
  and .openra_order_replay_reducer_gate == true
  and .bevy_openra_order_replay_reducer_claimed == true
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

REDUCER="$(jq -er '.reducer_path' "$SUMMARY")"
SNAPSHOTS="$(jq -er '.snapshot_path' "$SUMMARY")"
STREAM="$(jq -er '.source_paths.serializer_jsonl' "$SUMMARY")"
MANIFEST="$(jq -er '.source_paths.serializer_manifest' "$SUMMARY")"

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

jq -s -e '
  length >= 20
  and (to_entries | all(.[]; .value.sequence == .key))
  and all(.[]; .record_schema == "openra_order_stream_record_v1")
  and (.[0].order == "StartGame")
  and (map(.order) | index("ReplayOutcome") != null)
  and (map(.order) | index("Outcome") != null)
  and (map(.frame) | max) >= 3000
' "$STREAM" >/dev/null

jq -e '
  .manifest_schema == "openra_order_serializer_fixture_manifest_v1_json"
  and .stream_schema == "openra_order_stream_fixture_v1_jsonl"
  and .compatibility.openra_style_order_stream_fixture == true
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_network_order_stream_claimed == false
' "$MANIFEST" >/dev/null

test -s "$REDUCER"
test -s "$SNAPSHOTS"
test -s "$STREAM"
test -s "$MANIFEST"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ORDER_REPLAY_REDUCER_GREEN %s %s %s\n' "$SUMMARY" "$REDUCER" "$SNAPSHOTS"
