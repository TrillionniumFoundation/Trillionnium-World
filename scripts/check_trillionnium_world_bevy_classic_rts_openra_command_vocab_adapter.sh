#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-command-vocab-adapter.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-command-vocab-adapter"
ADAPTER="$PREVIEW_DIR/openra-command-vocabulary-adapter.json"
REPLAY="$PREVIEW_DIR/openra-replay-compat-adapter/openra-parity-lane/openra-parity-lane.trnm-replay.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-command-vocab-adapter "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter_v1"
  and .green == true
  and .adapter_state == "bevy_owned_replay_to_openra_style_command_vocabulary_adapter_not_binary_openra_replay"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and .source_contracts.owned_replay_file == "trillionnium_world_bevy_classic_rts_owned_replay_file_v1"
  and (.command_adapter_sha256 | type == "string" and length == 64)
  and .command_adapter_summary.schema == "openra_replay_command_vocab_adapter_v1_json"
  and .command_adapter_summary.source_replay_format == "trnm_owned_replay_v1_json"
  and (.command_adapter_summary.source_replay_sha256 | type == "string" and length == 64)
  and .command_adapter_summary.vocabulary_count >= 9
  and .command_adapter_summary.checkpoint_order_count >= 6
  and .command_adapter_summary.event_order_count >= 10
  and .command_adapter_summary.final_tick >= 3000
  and .command_adapter_summary.winner == "Multi2"
  and .command_adapter_summary.has_start_game == true
  and .command_adapter_summary.has_game_over == true
  and .command_adapter_summary.has_outcome_orders == true
  and .source_contract_gate == true
  and .source_green_gate == true
  and .replay_payload_gate == true
  and .command_vocabulary_gate == true
  and .checkpoint_command_gate == true
  and .event_command_gate == true
  and .outcome_command_gate == true
  and .compatibility_boundary_gate == true
  and .command_adapter_file_gate == true
  and .openra_command_vocab_adapter_gate == true
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

jq -e '
  .adapter_schema == "openra_replay_command_vocab_adapter_v1_json"
  and .source_replay_format == "trnm_owned_replay_v1_json"
  and (.source_replay_sha256 | type == "string" and length == 64)
  and (.command_vocabulary | index("StartGame") != null)
  and (.command_vocabulary | index("SyncFrame") != null)
  and (.command_vocabulary | index("BotOrder") != null)
  and (.command_vocabulary | index("TerminalProbe") != null)
  and (.command_vocabulary | index("GameOver") != null)
  and (.command_vocabulary | index("ReplayOutcome") != null)
  and (.command_vocabulary | index("Outcome") != null)
  and .start_game_order.order == "StartGame"
  and (.start_game_order.slots | length) == 4
  and (.checkpoint_orders | length) >= 6
  and (.checkpoint_orders | map(select(.order == "GameOver")) | length) >= 1
  and (.checkpoint_orders | map(select(.order == "ReplayOutcome")) | length) >= 1
  and (.event_orders | length) >= 10
  and (.event_orders | map(select(.order == "BotOrder")) | length) >= 4
  and (.event_orders | map(select(.order == "TerminalProbe")) | length) >= 4
  and (.event_orders | map(select(.order == "Winner" and (.source_event | contains("Multi2")))) | length) == 1
  and (.event_orders | map(select(.order == "Outcome")) | length) == 4
  and .timeline.final_tick >= 3000
  and .outcome.winner == "Multi2"
  and .compatibility.openra_command_vocabulary_schema_mapped == true
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_order_serializer_claimed == false
  and .compatibility.openra_network_order_stream_claimed == false
  and .compatibility.openra_runtime_parity_claimed == false
' "$ADAPTER" >/dev/null

test -s "$ADAPTER"
test -s "$REPLAY"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_COMMAND_VOCAB_ADAPTER_GREEN %s %s\n' "$SUMMARY" "$ADAPTER"
