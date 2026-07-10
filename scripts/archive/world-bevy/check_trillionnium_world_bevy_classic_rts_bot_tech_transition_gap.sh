#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tech-transition-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tech-transition-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-bot-tech-transition-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_tech_transition_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_tech_transition_gap_state == "bevy_tech_transition_vocabulary_not_openra_native_tech_switch_ai"
  and .bevy_native_tech_transition_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .tech_transition_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("early_signal_read") != null)
  and (.stage_summaries | map(.stage) | index("counter_tech_switch") != null)
  and (.stage_summaries | map(.stage) | index("anti_air_timing") != null)
  and (.stage_summaries | map(.stage) | index("siege_response_window") != null)
  and (.stage_summaries | map(.stage) | index("upgrade_timing_push") != null)
  and (.stage_summaries | map(.stage) | index("terminal_tech_lock") != null)
  and .tech_transition_signal_count >= 24
  and .signal_read_count >= 3
  and .counter_switch_count >= 3
  and .anti_air_timing_count >= 2
  and .siege_response_count >= 2
  and .upgrade_window_count >= 3
  and .terminal_tech_lock_count >= 2
  and .final_tech_transition_state == "terminal_tech_lock_secured"
  and .final_rts_ai_pressure_percent >= 90
  and .final_rts_defeat_risk_percent <= 15
  and .final_objective_capture_percent >= 95
  and .final_match_result_state == "tech_transition_gap:terminal_tech_lock_secured"
  and (.final_command_queue | index("tech_transition_stage:terminal_tech_lock") != null)
  and (.final_command_queue | index("native_openra_tech_transition_ai:false") != null)
  and (.final_army_production_batch_ids | index("tech_transition:counter_tech_switch") != null)
  and (.final_army_production_batch_ids | index("tech_transition:terminal_tech_lock") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_bot_tech_transition_core_subject_actor_ids | length) == 3
  and (.rts_bot_tech_transition_core_action_labels | length) == 6
  and .rts_bot_tech_transition_core_frame_order_stream.map_id == "first-contact-basin-bot-tech-transition"
  and .rts_bot_tech_transition_core_frame_order_stream.rules_id == "trnm-rts-core-bot-tech-transition-rules-v1"
  and .rts_bot_tech_transition_core_frame_order_kind_labels == ["recon", "research", "train", "upgrade", "attack", "capture"]
  and (.rts_bot_tech_transition_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_bot_tech_transition_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_bot_tech_transition_core_frame_order_errors | length) == 0
  and .rts_bot_tech_transition_core_frame_order_stream_error == null
  and .rts_bot_tech_transition_core_headless_replay_error == null
  and .rts_bot_tech_transition_core_headless_applied_order_count == 6
  and .rts_bot_tech_transition_core_headless_actor_count >= 3
  and .rts_bot_tech_transition_core_headless_final_frame == 2605
  and .rts_bot_tech_transition_core_headless_recon_order_count == 1
  and .rts_bot_tech_transition_core_headless_scan_order_count == 1
  and (.rts_bot_tech_transition_core_headless_recon_ids | index("early_signal_read") != null)
  and (.rts_bot_tech_transition_core_headless_recon_tile_ids | index("4,5") != null)
  and .rts_bot_tech_transition_core_headless_train_order_count == 1
  and (.rts_bot_tech_transition_core_headless_train_rule_ids | index("anti_air_timing") != null)
  and .rts_bot_tech_transition_core_headless_tech_order_count == 2
  and .rts_bot_tech_transition_core_headless_research_order_count == 1
  and .rts_bot_tech_transition_core_headless_upgrade_order_count == 1
  and (.rts_bot_tech_transition_core_headless_researched_rule_ids | index("counter_tech_switch") != null)
  and (.rts_bot_tech_transition_core_headless_upgraded_rule_ids | index("siege_response_window") != null)
  and (.rts_bot_tech_transition_core_headless_source_actor_ids | index("signal_array") != null)
  and (.rts_bot_tech_transition_core_headless_source_actor_ids | index("training_hall") != null)
  and .rts_bot_tech_transition_core_headless_objective_order_count == 1
  and .rts_bot_tech_transition_core_headless_capture_order_count == 1
  and (.rts_bot_tech_transition_core_headless_objective_ids | index("terminal_tech_lock") != null)
  and (.rts_bot_tech_transition_core_headless_objective_tile_ids | index("6,5") != null)
  and (.rts_bot_tech_transition_core_headless_objective_queue_ids | index("objective:claim:terminal_tech_lock@6,5") != null)
  and .rts_bot_tech_transition_core_headless_attack_order_count == 1
  and (.rts_bot_tech_transition_core_headless_combat_target_actor_ids | index("upgrade_timing_push") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .tech_transition_stage_gate == true
  and .tech_transition_signal_gate == true
  and .tech_transition_signal_read_gate == true
  and .tech_transition_counter_gate == true
  and .tech_transition_anti_air_gate == true
  and .tech_transition_siege_gate == true
  and .tech_transition_upgrade_gate == true
  and .tech_transition_terminal_gate == true
  and .bevy_gap_gate == true
  and .openra_tech_transition_target_gate == true
  and .renderer_gate == true
  and .tech_transition_gap_gate == true
  and .rts_bot_tech_transition_core_frame_order_gate == true
  and .rts_bot_tech_transition_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_TECH_TRANSITION_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
