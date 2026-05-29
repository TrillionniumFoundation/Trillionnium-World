#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-planner-live-autonomous-bot-loop.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-planner-live-autonomous-bot-loop"
DECISIONS="$PREVIEW_DIR/planner-live-autonomous-bot-loop.decisions.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-planner-live-autonomous-bot-loop "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop_v1"
  and .green == true
  and .planner_live_loop_state == "bevy_planner_drives_live_autonomous_bot_timeline_not_openra_bot_match"
  and .planner_phase_count == 6
  and .live_stage_count == 6
  and .replayable_decision_count == 6
  and (.decision_log | length) == 6
  and (.decision_log | map(.planner_phase) | index("stabilize_macro_workers") != null)
  and (.decision_log | map(.planner_phase) | index("scout_resource_beacons") != null)
  and (.decision_log | map(.planner_phase) | index("confirm_enemy_pressure_lane") != null)
  and (.decision_log | map(.planner_phase) | index("unlock_tier_two_tech") != null)
  and (.decision_log | map(.planner_phase) | index("transition_siege_push") != null)
  and (.decision_log | map(.planner_phase) | index("terminal_contract_alignment") != null)
  and (.decision_log | map(.live_stage) | index("spawn_and_mine") != null)
  and (.decision_log | map(.live_stage) | index("scout_beacons") != null)
  and (.decision_log | map(.live_stage) | index("first_beacon_capture") != null)
  and (.decision_log | map(.live_stage) | index("army_production_rally") != null)
  and (.decision_log | map(.live_stage) | index("beacon_fight") != null)
  and (.decision_log | map(.live_stage) | index("terminal_resolution") != null)
  and (.decision_log | all(.planner_phase_found == true and .live_stage_found == true))
  and (.decision_log | map(.tick) == ([0, 420, 900, 1440, 2160, 3000]))
  and (.decision_log_sha256 | type == "string" and length == 64)
  and (.planner_strategy_checksum_sha256 | type == "string" and length == 64)
  and .planner_summary.source_signal_count >= 120
  and .planner_summary.macro_final == "deny_rebuild_pressure"
  and .planner_summary.map_intel_final == "rotate_pressure_confirmed_beacon"
  and .planner_summary.tech_transition_final == "terminal_tech_lock_secured"
  and .planner_summary.terminal_winner == "Multi2"
  and .autonomous_summary.loop_kind == "deterministic_autonomous_bot_skirmish_timeline"
  and .autonomous_summary.winner == "Multi2"
  and .autonomous_summary.winner_beacons == 2
  and .autonomous_summary.total_beacons == 4
  and .autonomous_summary.hold_ticks == 3000
  and .autonomous_summary.final_match_result_state == "victory:bot_terminal:Multi2"
  and .autonomous_summary.final_objective_status == "autonomous_bot_terminal_complete:Multi2:2_of_4"
  and .source_contract_gate == true
  and .source_green_gate == true
  and .planner_source_gate == true
  and .autonomous_source_gate == true
  and .decision_mapping_gate == true
  and .live_timeline_gate == true
  and .terminal_alignment_gate == true
  and .decision_log_write_gate == true
  and .decision_log_readback_gate == true
  and .decision_log_replay_gate == true
  and .preview_gate == true
  and .no_forced_hook_gate == true
  and .boundary_gate == true
  and .planner_live_autonomous_bot_loop_gate == true
  and .bevy_planner_live_autonomous_bot_loop_claimed == true
  and .bevy_openra_live_bot_match_claimed == false
  and .bevy_openra_bot_ai_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop_v1"
  and .decision_count == 6
  and (.decisions | length) == 6
  and (.decisions | all(.planner_phase_found == true and .live_stage_found == true))
' "$DECISIONS" >/dev/null

test -s "$PREVIEW_DIR/autonomous-bot-skirmish.ppm"
test -s "$DECISIONS"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PLANNER_LIVE_AUTONOMOUS_BOT_LOOP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
