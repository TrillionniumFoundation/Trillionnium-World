#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-terminal-observation-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-terminal-observation-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-terminal-observation-gap "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_terminal_observation_gap_v1"
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
  and .bevy_terminal_observation_gap_state == "bevy_terminal_observation_vocabulary_not_natural_openra_match"
  and .bevy_natural_terminal_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_terminal_readiness_target_commit == "174525a"
  and .openra_terminal_probe_target_commit == "bf42eb1"
  and .openra_strategic_terminal_target_commit == "9e08464"
  and .openra_target_terminal_rule == "StrategicVictoryConditions:control_2_of_4_flux_beacons_for_3000_ticks"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("readiness_rule_check") != null)
  and (.stage_summaries | map(.stage) | index("terminal_observation_probe") != null)
  and (.stage_summaries | map(.stage) | index("outcome_classification") != null)
  and .final_match_result_state == "victory:terminal_observation:Multi2"
  and .final_objective_status == "terminal_observed:Multi2:2_of_4"
  and .terminal_victory_rules_ready == true
  and .strategic_victory_conditions_seen == true
  and .terminal_probe_game_over == true
  and .terminal_probe_winner_observed == true
  and .terminal_probe_loser_count == 3
  and .terminal_probe_controlled == false
  and .forced_capture_hook_enabled == false
  and .forced_surrender_hook_enabled == false
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .stage_gate == true
  and .terminal_readiness_gate == true
  and .terminal_observation_gate == true
  and .openra_target_gate == true
  and .bevy_gap_gate == true
  and .renderer_gate == true
  and .terminal_observation_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TERMINAL_OBSERVATION_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
