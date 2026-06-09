#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-action-executor.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-action-executor"
ACTIONS="$PREVIEW_DIR/bot-planner-action-executor.actions.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-bot-planner-action-executor "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1"
  and .green == true
  and .bot_planner_action_executor_state == "bevy_planner_decisions_execute_as_native_rts_actions_not_openra_runtime_bot"
  and .executor_action_count == 6
  and .accepted_action_count == 6
  and .command_marker_hit_count == 6
  and (.execution_log | length) == 6
  and (.execution_log | all(.source_decision_found == true and .accepted == true and .command_marker_hit == true))
  and (.execution_log | map(.planner_phase) == ([
    "stabilize_macro_workers",
    "scout_resource_beacons",
    "confirm_enemy_pressure_lane",
    "unlock_tier_two_tech",
    "transition_siege_push",
    "terminal_contract_alignment"
  ]))
  and (.execution_log | map(.source_tick) == ([0, 420, 900, 1440, 2160, 3000]))
  and (.action_labels | length) == 6
  and (.action_labels | index("RTS:QUEUE:faction:mirror_guard") != null)
  and (.action_labels | index("RTS:QUEUE:recon:sweep:watchtower_scan@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:objective:claim:relay_beacon@6,5") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:push:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:finish:gate_bulwark@10,3") != null)
  and (.expected_command_markers | length) == 6
  and (.input_sources == ["classic_rts_bot_planner_action_executor_input"])
  and (.action_log_sha256 | type == "string" and length == 64)
  and (.planner_live_decision_log_sha256 | type == "string" and length == 64)
  and (.planner_strategy_checksum_sha256 | type == "string" and length == 64)
  and .planner_live_summary.winner == "Multi2"
  and .planner_live_summary.winner_beacons == 2
  and .planner_live_summary.hold_ticks == 3000
  and .planner_live_summary.final_match_result_state == "victory:bot_terminal:Multi2"
  and .final_runtime_summary.faction_id == "mirror_guard"
  and .final_runtime_summary.objective_capture_percent == 100
  and (.final_runtime_summary.tier_two_tech_ids | index("relay_foundry") != null)
  and .final_runtime_summary.tier_two_push_state == "siege_push_ready:gate_bulwark"
  and .final_runtime_summary.siege_breach_state == "counterplay_won:gate_bulwark"
  and .final_runtime_summary.base_breach_percent == 100
  and (.final_runtime_summary.next_action_ids | index("enter_inner_lane") != null)
  and .final_runtime_summary.input_feedback_event_count == 6
  and .non_background_pixels > 250000
  and .write_preview_gate == true
  and .source_contract_gate == true
  and .source_loop_gate == true
  and .action_mapping_gate == true
  and .executor_acceptance_gate == true
  and .runtime_mutation_gate == true
  and .terminal_source_alignment_gate == true
  and .preview_gate == true
  and .action_log_write_gate == true
  and .action_log_readback_gate == true
  and .action_log_gate == true
  and .boundary_gate == true
  and .bot_planner_action_executor_gate == true
  and .bevy_bot_planner_action_executor_claimed == true
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
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1"
  and .executor_action_count == 6
  and .accepted_action_count == 6
  and .command_marker_hit_count == 6
  and (.execution_log | length) == 6
  and (.execution_log | all(.accepted == true and .command_marker_hit == true))
' "$ACTIONS" >/dev/null

test -s "$PREVIEW_DIR/bot-planner-action-executor.ppm"
test -s "$ACTIONS"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_PLANNER_ACTION_EXECUTOR_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
