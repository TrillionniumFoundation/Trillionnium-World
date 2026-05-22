#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-macro-economy-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-macro-economy-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-macro-economy-gap "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_macro_economy_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_macro_economy_gap_state == "bevy_macro_economy_vocabulary_not_openra_native_economy_ai"
  and .bevy_native_macro_economy_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .macro_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("worker_saturation_open") != null)
  and (.stage_summaries | map(.stage) | index("natural_expand_timing") != null)
  and (.stage_summaries | map(.stage) | index("supply_cap_recovery") != null)
  and (.stage_summaries | map(.stage) | index("production_queue_cycle") != null)
  and (.stage_summaries | map(.stage) | index("tech_ramp_spend") != null)
  and (.stage_summaries | map(.stage) | index("resource_deny_rebuild") != null)
  and .macro_signal_count >= 24
  and .worker_saturation_count >= 12
  and .expansion_timing_count >= 3
  and .supply_recovery_count >= 3
  and .production_cycle_count >= 4
  and .tech_ramp_count >= 2
  and .resource_deny_count >= 2
  and .final_macro_state == "deny_rebuild_pressure"
  and .final_rts_ai_pressure_percent >= 80
  and .final_rts_defeat_risk_percent <= 20
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "macro_economy_gap:deny_rebuild_pressure"
  and (.final_command_queue | index("macro_stage:resource_deny_rebuild") != null)
  and (.final_command_queue | index("native_openra_macro_economy_ai:false") != null)
  and (.final_army_production_batch_ids | index("macro_economy:worker_saturation_to_12") != null)
  and (.final_army_production_batch_ids | index("macro_economy:deny_enemy_node_rebuild_army") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .macro_stage_gate == true
  and .macro_signal_gate == true
  and .macro_worker_gate == true
  and .macro_expand_gate == true
  and .macro_supply_gate == true
  and .macro_production_gate == true
  and .macro_tech_gate == true
  and .macro_deny_rebuild_gate == true
  and .bevy_gap_gate == true
  and .openra_macro_economy_target_gate == true
  and .renderer_gate == true
  and .macro_economy_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_MACRO_ECONOMY_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
