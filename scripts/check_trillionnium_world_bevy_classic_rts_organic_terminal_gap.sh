#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-organic-terminal-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-organic-terminal-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-organic-terminal-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_organic_terminal_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .no_live_player_input_gate == true
  and .bot_slot_count == 4
  and (.bevy_bot_player_ids | length) == 4
  and .bevy_terminal_winner == "Multi2"
  and .bevy_terminal_winner_beacons == 2
  and .bevy_terminal_total_beacons == 4
  and .bevy_terminal_hold_ticks == 3000
  and .bevy_terminal_observation_gap_state == "bevy_deterministic_observation_not_openra_natural_gameover"
  and .bevy_natural_gameover_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_parity_target_commit == "5f1bf76"
  and .openra_parity_target_natural_terminal == true
  and .openra_parity_target_replay_outcome == "3W/1L"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("terminal_gameover_probe") != null)
  and (.stage_summaries | map(.stage) | index("replay_outcome_probe") != null)
  and .final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .final_objective_status == "organic_terminal_observed:Multi2:2_of_4"
  and .terminal_probe_controlled == false
  and .terminal_probe_game_over == true
  and .terminal_probe_winner_observed == true
  and .terminal_probe_loser_observed == true
  and .normal_match_winner_claimed == false
  and .winner_count >= 1
  and .loser_count >= 1
  and (.replay_outcome_log | index("Outcome:Multi2:Won") != null)
  and (.replay_outcome_log | index("Outcome:Multi0:Lost") != null)
  and (.replay_outcome_log | index("Replay:SurrenderAbsent") != null)
  and .forced_capture_hook_enabled == false
  and .forced_surrender_hook_enabled == false
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .stage_gate == true
  and .observation_report_gate == true
  and .openra_target_gate == true
  and .bevy_gap_gate == true
  and .renderer_gate == true
  and .organic_terminal_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ORGANIC_TERMINAL_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
