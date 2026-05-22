#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-terminal-loop.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-terminal-loop.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-terminal-loop "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_terminal_loop_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .write_gate == true
  and .input_action_count == 0
  and .no_live_player_input_gate == true
  and .bot_slot_count == 4
  and (.bevy_bot_player_ids | length) == 4
  and .bevy_terminal_winner == "Multi2"
  and .bevy_terminal_winner_beacons == 2
  and .bevy_terminal_total_beacons == 4
  and .bevy_terminal_hold_ticks == 3000
  and .bevy_terminal_rule == "bot_control_2_of_4_flux_beacons_for_3000_ticks"
  and .bevy_terminal_loop_kind == "deterministic_bot_terminal_rule_simulation"
  and .bevy_terminal_parity_claimed == false
  and .openra_parity_target_commit == "5f1bf76"
  and .openra_parity_target_natural_terminal == true
  and (.stage_summaries | length) == 4
  and (.final_objective_tile_ids | length) == 4
  and .final_objective_owner_state == "bot:Multi2:beacons=2"
  and .final_objective_result_state == "terminal_victory:Multi2:2_of_4_beacons"
  and (.final_objective_score_delta_log | index("beacon2:Multi2") != null)
  and (.final_objective_score_delta_log | index("beacon3:Multi2") != null)
  and .final_match_result_state == "victory:bot_terminal:Multi2"
  and .final_objective_status == "bot_terminal_complete:Multi2:2_of_4"
  and (.final_ai_wave_unit_ids | length) == 4
  and (.final_command_queue | index("bot_terminal_hold:Multi2:2_of_4@3000") != null)
  and .forced_capture_hook_enabled == false
  and .forced_surrender_hook_enabled == false
  and .non_background_pixels > 150000
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .bot_roster_gate == true
  and .beacon_rule_gate == true
  and .terminal_hold_gate == true
  and .terminal_result_gate == true
  and .renderer_gate == true
  and .bevy_terminal_rule_simulation_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_TERMINAL_LOOP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
