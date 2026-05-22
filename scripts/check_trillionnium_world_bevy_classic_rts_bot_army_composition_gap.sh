#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-army-composition-gap "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_army_composition_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_army_composition_gap_state == "bevy_army_composition_vocabulary_not_openra_native_unit_mix_ai"
  and .bevy_native_army_composition_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .army_composition_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("opening_unit_mix_read") != null)
  and (.stage_summaries | map(.stage) | index("frontline_backline_ratio") != null)
  and (.stage_summaries | map(.stage) | index("counter_mix_swap") != null)
  and (.stage_summaries | map(.stage) | index("reinforce_supply_curve") != null)
  and (.stage_summaries | map(.stage) | index("specialist_timing_window") != null)
  and (.stage_summaries | map(.stage) | index("terminal_composition_lock") != null)
  and .army_composition_signal_count >= 24
  and .unit_mix_read_count >= 3
  and .frontline_ratio_count >= 3
  and .counter_mix_swap_count >= 3
  and .reinforce_curve_count >= 3
  and .specialist_timing_count >= 2
  and .composition_lock_count >= 2
  and .final_army_composition_state == "terminal_composition_lock_secured"
  and .final_rts_ai_pressure_percent >= 90
  and .final_rts_defeat_risk_percent <= 15
  and .final_objective_capture_percent >= 95
  and .final_match_result_state == "army_composition_gap:terminal_composition_lock_secured"
  and (.final_command_queue | index("army_composition_stage:terminal_composition_lock") != null)
  and (.final_command_queue | index("native_openra_army_composition_ai:false") != null)
  and (.final_army_production_batch_ids | index("army_composition:counter_mix_swap") != null)
  and (.final_army_production_batch_ids | index("army_composition:terminal_composition_lock") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .army_composition_stage_gate == true
  and .army_composition_signal_gate == true
  and .army_composition_unit_mix_gate == true
  and .army_composition_ratio_gate == true
  and .army_composition_counter_gate == true
  and .army_composition_reinforce_gate == true
  and .army_composition_specialist_gate == true
  and .army_composition_lock_gate == true
  and .bevy_gap_gate == true
  and .openra_army_composition_target_gate == true
  and .renderer_gate == true
  and .army_composition_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_ARMY_COMPOSITION_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
