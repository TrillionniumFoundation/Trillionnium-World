#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-native-bot-ai-planner.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-native-bot-ai-planner"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-native-bot-ai-planner "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_native_bot_ai_planner_v1"
  and .green == true
  and .planner_strategy_state == "bevy_native_bot_ai_planner_v1_macro_intel_tech_closed_not_openra_bot_parity"
  and .planner_phase_count == 6
  and (.strategy_phases | length) == 6
  and (.strategy_phases | map(.phase) | index("scout_resource_beacons") != null)
  and (.strategy_phases | map(.phase) | index("stabilize_macro_workers") != null)
  and (.strategy_phases | map(.phase) | index("confirm_enemy_pressure_lane") != null)
  and (.strategy_phases | map(.phase) | index("unlock_tier_two_tech") != null)
  and (.strategy_phases | map(.phase) | index("transition_siege_push") != null)
  and (.strategy_phases | map(.phase) | index("terminal_contract_alignment") != null)
  and (.strategy_phases | all(.gate == true))
  and (.strategy_checksum_sha256 | type == "string" and length == 64)
  and .source_signal_count >= 120
  and .macro_summary.stage_count == 6
  and .macro_summary.signal_count >= 24
  and .macro_summary.final_state == "deny_rebuild_pressure"
  and .macro_summary.final_match_result_state == "macro_economy_gap:deny_rebuild_pressure"
  and .macro_summary.final_objective_capture_percent >= 90
  and .map_intel_summary.stage_count == 6
  and .map_intel_summary.signal_count >= 24
  and .map_intel_summary.final_state == "rotate_pressure_confirmed_beacon"
  and .map_intel_summary.final_match_result_state == "map_intel_gap:rotate_pressure_confirmed_beacon"
  and .map_intel_summary.final_objective_capture_percent >= 90
  and .tech_transition_summary.stage_count == 6
  and .tech_transition_summary.signal_count >= 24
  and .tech_transition_summary.final_state == "terminal_tech_lock_secured"
  and .tech_transition_summary.final_match_result_state == "tech_transition_gap:terminal_tech_lock_secured"
  and .tech_transition_summary.final_objective_capture_percent >= 95
  and .terminal_contract_summary.terminal_state == "bevy_natural_terminal_contract_v1_not_openra_natural_match"
  and .terminal_contract_summary.winner == "Multi2"
  and .terminal_contract_summary.winner_beacons == 2
  and .terminal_contract_summary.total_beacons == 4
  and .terminal_contract_summary.hold_ticks == 3000
  and .source_contract_gate == true
  and .source_green_gate == true
  and .owned_replay_file_gate == true
  and .owned_replay_summary.recorded_input_count == 6
  and .owned_replay_summary.playback_checkpoint_count == 6
  and .owned_replay_summary.checksum_mismatch_count == 0
  and (.owned_replay_summary.final_playback_checkpoint_sha256 | type == "string" and length == 64)
  and .macro_economy_phase_gate == true
  and .map_intel_phase_gate == true
  and .tech_transition_phase_gate == true
  and .terminal_contract_gate == true
  and .planner_pressure_gate == true
  and .planner_phase_gate == true
  and .planner_replayability_gate == true
  and .preview_gate == true
  and .no_openra_bot_parity_claim_gate == true
  and .boundary_gate == true
  and .native_bot_ai_planner_gate == true
  and .bevy_native_bot_ai_planner_claimed == true
  and .bevy_openra_bot_ai_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/bot-macro-economy-gap.ppm"
test -s "$PREVIEW_DIR/bot-map-intel-gap.ppm"
test -s "$PREVIEW_DIR/bot-tech-transition-gap.ppm"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NATIVE_BOT_AI_PLANNER_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
