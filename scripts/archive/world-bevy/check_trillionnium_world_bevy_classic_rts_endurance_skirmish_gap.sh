#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-endurance-skirmish-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-endurance-skirmish-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-endurance-skirmish-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_endurance_skirmish_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_endurance_skirmish_gap_state == "bevy_endurance_vocabulary_not_openra_headless_client_match"
  and .bevy_headless_match_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_endurance_skirmish_target_commit == "2cb80a0"
  and .openra_longrun_skirmish_target_commit == "5227d99"
  and .openra_multibot_autostart_target_commit == "4b966c1"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("room_autostart") != null)
  and (.stage_summaries | map(.stage) | index("endurance_summary") != null)
  and .endurance_startgame_order == true
  and .endurance_autostart_order == true
  and (.endurance_client_slots | length) == 4
  and .endurance_bot_type == "trnm-rush"
  and .configured_seconds >= 120
  and .elapsed_seconds >= 120
  and .min_active_units >= 4
  and .peak_active_units >= 24
  and .contested_beacon_peak >= 2
  and .economy_event_count >= 12
  and .combat_event_count >= 20
  and .tech_event_count >= 6
  and .terminal_victory_rules_ready == true
  and .terminal_victory_detected == false
  and .winner_claimed == false
  and .outcome_signal == "sustained_engagement_no_terminal_victory"
  and .final_match_result_state == "endurance_sustained_engagement_no_terminal_victory"
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .match_result_pixel_count > 20
  and .endurance_stage_gate == true
  and .endurance_roster_gate == true
  and .endurance_duration_gate == true
  and .endurance_pressure_gate == true
  and .battle_outcome_gate == true
  and .bevy_gap_gate == true
  and .openra_endurance_target_gate == true
  and .renderer_gate == true
  and .endurance_skirmish_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENDURANCE_SKIRMISH_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
