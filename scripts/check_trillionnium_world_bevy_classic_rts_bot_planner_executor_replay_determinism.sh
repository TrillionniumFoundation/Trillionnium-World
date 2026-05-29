#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-executor-replay-determinism.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-executor-replay-determinism"
REPLAY_LOG="$PREVIEW_DIR/bot-planner-executor-replay-determinism.replay.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-planner-executor-replay-determinism "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1"
  and .green == true
  and .bot_planner_executor_replay_determinism_state == "bevy_executor_action_log_replays_to_identical_runtime_state_not_openra_runtime_bot"
  and .source_executor_action_count == 6
  and .replay_action_count == 6
  and .accepted_replay_action_count == 6
  and .replay_command_marker_hit_count == 6
  and .command_delta_match_count == 6
  and (.replay_execution_log | length) == 6
  and (.replay_execution_log | all(.action_label_parse_gate == true and .accepted == true and .command_marker_hit == true and .command_delta_match == true))
  and (.replay_execution_log | map(.source_planner_phase) == ([
    "stabilize_macro_workers",
    "scout_resource_beacons",
    "confirm_enemy_pressure_lane",
    "unlock_tier_two_tech",
    "transition_siege_push",
    "terminal_contract_alignment"
  ]))
  and (.replay_execution_log | map(.source_tick) == ([0, 420, 900, 1440, 2160, 3000]))
  and (.replayed_action_labels | length) == 6
  and (.replayed_action_labels | index("RTS:QUEUE:faction:mirror_guard") != null)
  and (.replayed_action_labels | index("RTS:QUEUE:recon:sweep:watchtower_scan@7,4") != null)
  and (.replayed_action_labels | index("RTS:QUEUE:objective:claim:relay_beacon@6,5") != null)
  and (.replayed_action_labels | index("RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost") != null)
  and (.replayed_action_labels | index("RTS:QUEUE:tier2:push:gate_bulwark@10,3") != null)
  and (.replayed_action_labels | index("RTS:QUEUE:tier2:finish:gate_bulwark@10,3") != null)
  and (.replay_input_sources == ["classic_rts_bot_planner_executor_replay_input"])
  and (.source_action_log_sha256 | type == "string" and length == 64)
  and (.replay_log_sha256 | type == "string" and length == 64)
  and (.planner_live_decision_log_sha256 | type == "string" and length == 64)
  and (.planner_strategy_checksum_sha256 | type == "string" and length == 64)
  and (.source_final_runtime_sha256 | type == "string" and length == 64)
  and (.replay_final_runtime_sha256 | type == "string" and length == 64)
  and .source_final_runtime_sha256 == .replay_final_runtime_sha256
  and (.source_command_queue_sha256 | type == "string" and length == 64)
  and (.replay_command_queue_sha256 | type == "string" and length == 64)
  and .source_command_queue_sha256 == .replay_command_queue_sha256
  and .source_final_runtime_summary == .replay_final_runtime_summary
  and .replay_final_runtime_summary.faction_id == "mirror_guard"
  and .replay_final_runtime_summary.objective_capture_percent == 100
  and (.replay_final_runtime_summary.tier_two_tech_ids | index("relay_foundry") != null)
  and .replay_final_runtime_summary.tier_two_push_state == "siege_push_ready:gate_bulwark"
  and .replay_final_runtime_summary.siege_breach_state == "counterplay_won:gate_bulwark"
  and .replay_final_runtime_summary.base_breach_percent == 100
  and .replay_final_runtime_summary.match_result_state == "siege_breakthrough:inner_lane"
  and (.replay_final_runtime_summary.next_action_ids | index("enter_inner_lane") != null)
  and .replay_final_runtime_summary.input_feedback_event_count == 6
  and .non_background_pixels > 250000
  and .write_preview_gate == true
  and .source_contract_gate == true
  and .source_executor_gate == true
  and .source_action_log_readback_gate == true
  and .replay_mapping_gate == true
  and .replay_acceptance_gate == true
  and .runtime_determinism_gate == true
  and .preview_gate == true
  and .replay_log_write_gate == true
  and .replay_log_readback_gate == true
  and .replay_log_gate == true
  and .boundary_gate == true
  and .bot_planner_executor_replay_determinism_gate == true
  and .bevy_bot_planner_executor_replay_determinism_claimed == true
  and .bevy_openra_runtime_bot_executor_claimed == false
  and .bevy_openra_live_bot_match_claimed == false
  and .bevy_openra_bot_ai_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1"
  and .replay_action_count == 6
  and .accepted_replay_action_count == 6
  and .replay_command_marker_hit_count == 6
  and .command_delta_match_count == 6
  and .source_final_runtime_sha256 == .replay_final_runtime_sha256
  and .source_command_queue_sha256 == .replay_command_queue_sha256
  and (.execution_log | length) == 6
  and (.execution_log | all(.accepted == true and .command_marker_hit == true and .command_delta_match == true))
' "$REPLAY_LOG" >/dev/null

test -s "$PREVIEW_DIR/bot-planner-executor-replay-determinism.ppm"
test -s "$REPLAY_LOG"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_PLANNER_EXECUTOR_REPLAY_DETERMINISM_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
