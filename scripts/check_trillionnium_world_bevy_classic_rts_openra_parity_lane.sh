#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-lane.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-lane"
REPLAY="$PREVIEW_DIR/openra-parity-lane.trnm-replay.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-parity-lane "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_parity_lane_v1"
  and .green == true
  and .lane_state == "bevy_openra_parity_lane_v1_local_runtime_green_not_openra_runtime_parity"
  and .lane_axis_count == 6
  and (.lane_axes | length) == 6
  and (.lane_axes | all(.gate == true))
  and (.lane_axes | map(.axis) | index("rules_mod_vocabulary") != null)
  and (.lane_axes | map(.axis) | index("comparison_bridge") != null)
  and (.lane_axes | map(.axis) | index("owned_replay_file") != null)
  and (.lane_axes | map(.axis) | index("headless_replay_playback") != null)
  and (.lane_axes | map(.axis) | index("natural_terminal_contract") != null)
  and (.lane_axes | map(.axis) | index("bot_skirmish_loop") != null)
  and .source_contracts.openra_like_core == "trillionnium_world_bevy_classic_rts_openra_like_core_v1"
  and .source_contracts.openra_parity_bridge == "trillionnium_world_bevy_classic_rts_openra_parity_bridge_v1"
  and .source_contracts.owned_replay_file == "trillionnium_world_bevy_classic_rts_owned_replay_file_v1"
  and .source_contracts.headless_replay_playback == "trillionnium_world_bevy_classic_rts_headless_replay_playback_v1"
  and .source_contracts.natural_terminal_contract == "trillionnium_world_bevy_classic_rts_natural_terminal_contract_v1"
  and .source_contracts.planner_live_autonomous_bot_loop == "trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop_v1"
  and .lane_summary.runtime_model == "rust_bevy_owned_openra_like_rts_core"
  and .lane_summary.rules_count >= 10
  and .lane_summary.actor_template_count >= 39
  and .lane_summary.order_count >= 10
  and .lane_summary.bridge_axis_count == 4
  and .lane_summary.replay_format == "trnm_owned_replay_v1_json"
  and (.lane_summary.replay_file_sha256 | type == "string" and length == 64)
  and .lane_summary.recorded_input_count >= 6
  and .lane_summary.headless_playback_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and (.lane_summary.final_checkpoint_sha256 | type == "string" and length == 64)
  and .lane_summary.terminal_winner == "Multi2"
  and .lane_summary.terminal_hold_ticks == 3000
  and .lane_summary.bot_loop_state == "bevy_planner_drives_live_autonomous_bot_timeline_not_openra_bot_match"
  and .lane_summary.bot_loop_winner == "Multi2"
  and .lane_summary.bot_loop_decision_count == 6
  and .source_contract_gate == true
  and .source_green_gate == true
  and .rules_mod_vocabulary_gate == true
  and .comparison_bridge_gate == true
  and .owned_replay_lane_gate == true
  and .headless_playback_lane_gate == true
  and .natural_terminal_lane_gate == true
  and .bot_skirmish_lane_gate == true
  and .replay_headless_consistency_gate == true
  and .no_openra_parity_claim_gate == true
  and .boundary_gate == true
  and .openra_parity_lane_gate == true
  and .bevy_openra_parity_lane_evidence_claimed == true
  and .bevy_openra_runtime_parity_claimed == false
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_natural_terminal_match_claimed == false
  and .bevy_openra_live_bot_match_claimed == false
  and .bevy_openra_bot_ai_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$REPLAY"
test -s "$PREVIEW_DIR/openra-parity-bridge/organic-terminal-gap.ppm"
test -s "$PREVIEW_DIR/natural-terminal-contract/organic-terminal.ppm"
test -s "$PREVIEW_DIR/planner-live-autonomous-bot-loop/autonomous-bot-skirmish.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_PARITY_LANE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
