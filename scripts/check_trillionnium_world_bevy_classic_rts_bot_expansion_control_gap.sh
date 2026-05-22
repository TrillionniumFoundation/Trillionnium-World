#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-expansion-control-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-expansion-control-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-expansion-control-gap "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_expansion_control_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_expansion_control_gap_state == "bevy_expansion_control_vocabulary_not_openra_native_map_control_ai"
  and .bevy_native_expansion_control_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .expansion_control_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("natural_expand_probe") != null)
  and (.stage_summaries | map(.stage) | index("third_node_deny") != null)
  and (.stage_summaries | map(.stage) | index("refinery_pickoff") != null)
  and (.stage_summaries | map(.stage) | index("contain_ring_setup") != null)
  and (.stage_summaries | map(.stage) | index("reexpand_punish") != null)
  and (.stage_summaries | map(.stage) | index("map_control_lock") != null)
  and .expansion_control_signal_count >= 24
  and .natural_probe_count >= 3
  and .third_node_deny_count >= 3
  and .refinery_pickoff_count >= 2
  and .contain_ring_count >= 3
  and .reexpand_punish_count >= 2
  and .map_lock_count >= 2
  and .final_expansion_control_state == "map_control_lock_secured"
  and .final_rts_ai_pressure_percent >= 85
  and .final_rts_defeat_risk_percent <= 20
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "expansion_control_gap:map_control_lock_secured"
  and (.final_command_queue | index("expansion_control_stage:map_control_lock") != null)
  and (.final_command_queue | index("native_openra_expansion_control_ai:false") != null)
  and (.final_army_production_batch_ids | index("expansion_control:third_node_deny") != null)
  and (.final_army_production_batch_ids | index("expansion_control:map_control_lock") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .expansion_control_stage_gate == true
  and .expansion_control_signal_gate == true
  and .expansion_control_natural_gate == true
  and .expansion_control_third_node_gate == true
  and .expansion_control_refinery_gate == true
  and .expansion_control_contain_gate == true
  and .expansion_control_reexpand_gate == true
  and .expansion_control_lock_gate == true
  and .bevy_gap_gate == true
  and .openra_expansion_control_target_gate == true
  and .renderer_gate == true
  and .expansion_control_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_EXPANSION_CONTROL_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
