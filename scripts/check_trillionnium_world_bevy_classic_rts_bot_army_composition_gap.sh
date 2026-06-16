#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-bot-army-composition-gap "$PREVIEW" >"$SUMMARY"

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
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_bot_army_composition_core_subject_actor_ids | length) == 3
  and (.rts_bot_army_composition_core_action_labels | length) == 6
  and .rts_bot_army_composition_core_frame_order_stream.map_id == "first-contact-basin-bot-army-composition"
  and .rts_bot_army_composition_core_frame_order_stream.rules_id == "trnm-rts-core-bot-army-composition-rules-v1"
  and .rts_bot_army_composition_core_frame_order_kind_labels == ["recon", "train", "train", "train", "ability", "capture"]
  and (.rts_bot_army_composition_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_bot_army_composition_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_bot_army_composition_core_frame_order_errors | length) == 0
  and .rts_bot_army_composition_core_frame_order_stream_error == null
  and .rts_bot_army_composition_core_headless_replay_error == null
  and .rts_bot_army_composition_core_headless_applied_order_count == 6
  and .rts_bot_army_composition_core_headless_actor_count >= 3
  and .rts_bot_army_composition_core_headless_final_frame == 2805
  and .rts_bot_army_composition_core_headless_recon_order_count == 1
  and .rts_bot_army_composition_core_headless_scout_order_count == 1
  and (.rts_bot_army_composition_core_headless_recon_ids | index("opening_unit_mix_read") != null)
  and (.rts_bot_army_composition_core_headless_recon_tile_ids | index("4,5") != null)
  and .rts_bot_army_composition_core_headless_train_order_count == 3
  and (.rts_bot_army_composition_core_headless_train_rule_ids | index("frontline_backline_ratio") != null)
  and (.rts_bot_army_composition_core_headless_train_rule_ids | index("counter_mix_swap") != null)
  and (.rts_bot_army_composition_core_headless_train_rule_ids | index("reinforce_supply_curve") != null)
  and .rts_bot_army_composition_core_headless_ability_order_count == 1
  and (.rts_bot_army_composition_core_headless_ability_rule_ids | index("specialist_timing_window") != null)
  and (.rts_bot_army_composition_core_headless_ability_target_actor_ids | index("signal_array") != null)
  and .rts_bot_army_composition_core_headless_objective_order_count == 1
  and .rts_bot_army_composition_core_headless_capture_order_count == 1
  and (.rts_bot_army_composition_core_headless_objective_ids | index("terminal_composition_lock") != null)
  and (.rts_bot_army_composition_core_headless_objective_tile_ids | index("6,5") != null)
  and (.rts_bot_army_composition_core_headless_objective_queue_ids | index("objective:claim:terminal_composition_lock@6,5") != null)
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
  and .rts_bot_army_composition_core_frame_order_gate == true
  and .rts_bot_army_composition_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_ARMY_COMPOSITION_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
