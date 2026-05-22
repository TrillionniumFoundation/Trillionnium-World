#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-tactical-micro-gap "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_tactical_micro_gap_state == "bevy_tactical_micro_vocabulary_not_openra_native_combat_ai"
  and .bevy_native_combat_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .micro_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("target_priority_probe") != null)
  and (.stage_summaries | map(.stage) | index("focus_fire_commit") != null)
  and (.stage_summaries | map(.stage) | index("kite_and_stutter_step") != null)
  and (.stage_summaries | map(.stage) | index("flank_angle_split") != null)
  and (.stage_summaries | map(.stage) | index("ability_timing_window") != null)
  and (.stage_summaries | map(.stage) | index("low_health_pullback_regroup") != null)
  and .micro_signal_count >= 24
  and .target_swap_count >= 3
  and .focus_fire_order_count >= 3
  and .kite_step_count >= 3
  and .flank_angle_count >= 2
  and .ability_timing_count >= 2
  and .low_health_pullback_count >= 2
  and .final_micro_state == "pullback_regroup_reattack"
  and .final_rts_ai_pressure_percent >= 70
  and .final_rts_defeat_risk_percent <= 20
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "tactical_micro_gap:pullback_regroup_reattack"
  and (.final_command_queue | index("micro_stage:low_health_pullback_regroup") != null)
  and (.final_command_queue | index("native_openra_combat_ai:false") != null)
  and (.final_army_production_batch_ids | index("micro_control:focus_fire_low_armor_striker") != null)
  and (.final_army_production_batch_ids | index("micro_control:pull_redline_units_regroup_reattack") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .micro_stage_gate == true
  and .micro_signal_gate == true
  and .micro_target_gate == true
  and .micro_focus_gate == true
  and .micro_kite_gate == true
  and .micro_flank_gate == true
  and .micro_ability_gate == true
  and .micro_pullback_gate == true
  and .bevy_gap_gate == true
  and .openra_tactical_micro_target_gate == true
  and .renderer_gate == true
  and .tactical_micro_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_TACTICAL_MICRO_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
