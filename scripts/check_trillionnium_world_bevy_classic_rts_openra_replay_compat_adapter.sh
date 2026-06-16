#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-compat-adapter.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-replay-compat-adapter"
ADAPTER="$PREVIEW_DIR/openra-replay-summary-adapter.json"
REPLAY="$PREVIEW_DIR/openra-parity-lane/openra-parity-lane.trnm-replay.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-replay-compat-adapter "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and .green == true
  and .adapter_state == "bevy_owned_replay_to_openra_style_summary_adapter_not_binary_openra_replay"
  and .source_contracts.openra_parity_lane == "trillionnium_world_bevy_classic_rts_openra_parity_lane_v1"
  and .source_contracts.owned_replay_file == "trillionnium_world_bevy_classic_rts_owned_replay_file_v1"
  and .source_contracts.headless_replay_playback == "trillionnium_world_bevy_classic_rts_headless_replay_playback_v1"
  and .source_contracts.natural_terminal_contract == "trillionnium_world_bevy_classic_rts_natural_terminal_contract_v1"
  and (.adapter_sha256 | type == "string" and length == 64)
  and .adapter_summary.schema == "openra_replay_summary_adapter_v1_json"
  and .adapter_summary.source_replay_format == "trnm_owned_replay_v1_json"
  and (.adapter_summary.source_replay_sha256 | type == "string" and length == 64)
  and (.adapter_summary.map_uid | type == "string" and length == 64)
  and (.adapter_summary.rules_uid | type == "string" and length == 64)
  and .adapter_summary.start_game_order == "StartGame"
  and .adapter_summary.slot_count == 4
  and .adapter_summary.recorded_input_count >= 6
  and .adapter_summary.combat_event_count >= 10
  and .adapter_summary.winner == "Multi2"
  and .adapter_summary.final_tick >= 3000
  and .adapter_summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and .source_contract_gate == true
  and .source_green_gate == true
  and .replay_parse_gate == true
  and .summary_schema_gate == true
  and .replay_timeline_gate == true
  and .headless_adapter_gate == true
  and .terminal_adapter_gate == true
  and .compatibility_boundary_gate == true
  and .adapter_file_gate == true
  and .openra_replay_compat_adapter_gate == true
  and .bevy_openra_replay_summary_adapter_claimed == true
  and .bevy_openra_binary_replay_compatible == false
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
  .adapter_schema == "openra_replay_summary_adapter_v1_json"
  and .source_replay_format == "trnm_owned_replay_v1_json"
  and (.source_replay_sha256 | type == "string" and length == 64)
  and .start_game.order == "StartGame"
  and (.start_game.slots | length) == 4
  and .start_game.human_slot == "Multi0"
  and (.start_game.bot_slots | length) == 3
  and .timeline.recorded_input_count >= 6
  and (.timeline.stages | index("replay_outcome_probe") != null)
  and .outcome.winner == "Multi2"
  and .outcome.winner_count == 1
  and .outcome.loser_count == 3
  and .outcome.final_tick >= 3000
  and .headless.mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and .headless.rendered_frame_count == 0
  and .headless.wgpu_required == false
  and .terminal_contract.winner == "Multi2"
  and .terminal_contract.hold_ticks == 3000
  and .compatibility.openra_replay_summary_schema_mapped == true
  and .compatibility.openra_binary_replay_compatible == false
  and .compatibility.openra_replay_file_claimed == false
  and .compatibility.openra_headless_client_match_claimed == false
  and .compatibility.openra_runtime_parity_claimed == false
' "$ADAPTER" >/dev/null

test -s "$ADAPTER"
test -s "$REPLAY"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_REPLAY_COMPAT_ADAPTER_GREEN %s %s\n' "$SUMMARY" "$ADAPTER"
