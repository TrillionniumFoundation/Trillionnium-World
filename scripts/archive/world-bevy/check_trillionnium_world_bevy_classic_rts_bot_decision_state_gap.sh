#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-bot-decision-state-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_decision_state_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_decision_gap_state == "bevy_bot_decision_vocabulary_not_openra_native_bot_ai"
  and .bevy_native_bot_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_bot_decision_core_frame_order_gate == true
  and .rts_bot_decision_core_headless_replay_gate == true
  and .rts_bot_decision_core_frame_order_kind_labels == ["harvest", "recon", "capture", "research", "attack", "move"]
  and .rts_bot_decision_core_headless_applied_order_count == 6
  and .rts_bot_decision_core_headless_actor_count >= 3
  and .rts_bot_decision_core_headless_final_frame == 1005
  and .rts_bot_decision_core_headless_harvest_actor_order_count >= 3
  and .rts_bot_decision_core_headless_scout_order_count == 1
  and .rts_bot_decision_core_headless_capture_order_count == 1
  and .rts_bot_decision_core_headless_research_order_count == 1
  and .rts_bot_decision_core_headless_attack_order_count == 1
  and .rts_bot_decision_core_headless_micro_move_order_count == 1
  and (.rts_bot_decision_core_headless_recon_ids | index("beacon_ring") != null)
  and (.rts_bot_decision_core_headless_objective_ids | index("relay_beacon") != null)
  and (.rts_bot_decision_core_headless_researched_rule_ids | index("signal_array") != null)
  and (.rts_bot_decision_core_headless_combat_target_actor_ids | index("counter_push") != null)
  and (.rts_bot_decision_core_headless_combat_target_tile_ids | index("8,4") != null)
  and .bot_decision_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("economy_seed") != null)
  and (.stage_summaries | map(.stage) | index("scout_objectives") != null)
  and (.stage_summaries | map(.stage) | index("capture_beacon") != null)
  and (.stage_summaries | map(.stage) | index("tech_switch") != null)
  and (.stage_summaries | map(.stage) | index("defend_counter") != null)
  and (.stage_summaries | map(.stage) | index("attack_commit_with_counter_repath") != null)
  and .decision_signal_count >= 18
  and .economy_decision_count >= 3
  and .objective_decision_count >= 4
  and .combat_decision_count >= 4
  and .tech_decision_count >= 2
  and .final_bot_decision_state == "attack_commit_with_counter_repath"
  and .final_rts_ai_pressure_percent >= 70
  and .final_rts_defeat_risk_percent <= 35
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "bot_decision_gap:attack_commit_with_counter_repath"
  and (.final_command_queue | index("decision:combat:attack_commit_with_counter_repath") != null)
  and (.final_command_queue | index("parity_claim:false") != null)
  and (.final_army_production_batch_ids | index("batch:tech:signal+skimmer+bastion") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .bot_decision_stage_gate == true
  and .bot_decision_signal_gate == true
  and .bot_decision_economy_gate == true
  and .bot_decision_scout_gate == true
  and .bot_decision_capture_gate == true
  and .bot_decision_tech_gate == true
  and .bot_decision_counter_gate == true
  and .bot_decision_attack_gate == true
  and .bot_decision_retreat_gate == true
  and .bevy_gap_gate == true
  and .openra_bot_decision_target_gate == true
  and .renderer_gate == true
  and .bot_decision_state_gap_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_DECISION_STATE_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
