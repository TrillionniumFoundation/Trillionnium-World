#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-replay-metrics-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-replay-metrics-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-replay-metrics-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_replay_metrics_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_replay_metrics_gap_state == "bevy_replay_metric_vocabulary_not_openra_replay_file"
  and .bevy_replay_file_claimed == false
  and .bevy_replay_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_replay_summary_target_commit == "d5ceade"
  and .openra_battle_outcome_target_commit == "9b2664b"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("startgame_order") != null)
  and (.stage_summaries | map(.stage) | index("outcome_summary") != null)
  and .replay_startgame_order == true
  and (.replay_client_slots | length) == 4
  and .replay_human_slot == "Multi0"
  and .replay_bot_type == "trnm-rush"
  and .replay_bot_mentions >= 3
  and .replay_actor_order_tokens >= 12
  and .replay_unique_actor_token_count >= 6
  and (.replay_unique_actor_tokens | index("trnm.worker") != null)
  and (.replay_unique_actor_tokens | index("trnm.striker") != null)
  and .replay_economy_tokens >= 12
  and .replay_tech_tokens >= 6
  and .replay_combat_tokens >= 12
  and .configured_seconds >= 55
  and .elapsed_seconds >= 55
  and .outcome_signal == "sustained_engagement_no_terminal_victory"
  and .terminal_victory_detected == false
  and .terminal_victory_rules_ready == true
  and .winner_claimed == false
  and .final_match_result_state == "sustained_engagement_no_terminal_victory"
  and (.final_command_queue | index("replay_startgame_order:true") != null)
  and (.final_command_queue | index("battle_outcome:winner_claimed:false") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .match_result_pixel_count > 20
  and .replay_metrics_stage_gate == true
  and .replay_roster_gate == true
  and .replay_token_gate == true
  and .battle_outcome_summary_gate == true
  and .bevy_gap_gate == true
  and .openra_replay_metrics_target_gate == true
  and .renderer_gate == true
  and .replay_metrics_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_REPLAY_METRICS_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
