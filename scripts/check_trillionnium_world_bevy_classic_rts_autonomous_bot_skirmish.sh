#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-autonomous-bot-skirmish.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-autonomous-bot-skirmish.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-autonomous-bot-skirmish "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_autonomous_bot_skirmish_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .no_live_player_input_gate == true
  and .bot_slot_count == 4
  and (.bevy_bot_player_ids | length) == 4
  and .bevy_terminal_winner == "Multi2"
  and .bevy_terminal_winner_beacons == 2
  and .bevy_terminal_total_beacons == 4
  and .bevy_terminal_hold_ticks == 3000
  and .bevy_autonomous_loop_kind == "deterministic_autonomous_bot_skirmish_timeline"
  and .bevy_terminal_parity_claimed == false
  and .openra_parity_target_commit == "5f1bf76"
  and .openra_parity_target_natural_terminal == true
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_autonomous_bot_core_frame_order_stream.map_id == "first-contact-basin-autonomous-bot-skirmish"
  and .rts_autonomous_bot_core_frame_order_stream.rules_id == "trnm-rts-core-autonomous-bot-skirmish-rules-v1"
  and .rts_autonomous_bot_core_frame_order_kind_labels == ["harvest", "recon", "capture", "train", "attack", "capture", "capture"]
  and .rts_autonomous_bot_core_frame_ticks == [0, 420, 900, 1440, 2160, 2400, 3000]
  and (.rts_autonomous_bot_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_autonomous_bot_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_autonomous_bot_core_frame_order_errors | length) == 0
  and .rts_autonomous_bot_core_frame_order_stream_error == null
  and .rts_autonomous_bot_core_headless_replay_error == null
  and .rts_autonomous_bot_core_headless_applied_order_count == 7
  and .rts_autonomous_bot_core_headless_player_count >= 3
  and .rts_autonomous_bot_core_headless_actor_count == 4
  and .rts_autonomous_bot_core_headless_final_frame == 3000
  and .rts_autonomous_bot_core_headless_harvest_actor_order_count == 1
  and .rts_autonomous_bot_core_headless_recon_order_count == 1
  and .rts_autonomous_bot_core_headless_train_order_count == 1
  and .rts_autonomous_bot_core_headless_attack_order_count == 1
  and .rts_autonomous_bot_core_headless_capture_order_count == 3
  and (.rts_autonomous_bot_core_headless_objective_ids | index("relay_beacon_1") != null)
  and (.rts_autonomous_bot_core_headless_objective_ids | index("relay_beacon_2") != null)
  and (.rts_autonomous_bot_core_headless_objective_ids | index("relay_beacon_3") != null)
  and (.rts_autonomous_bot_core_headless_objective_tile_ids | index("7,5") != null)
  and (.rts_autonomous_bot_core_headless_train_rule_ids | index("trnm.forge_bastion") != null)
  and (.rts_autonomous_bot_core_headless_recon_ids | index("beacon_ring") != null)
  and (.rts_autonomous_bot_core_headless_combat_target_actor_ids | index("beacon_lane_fight") != null)
  and (.stage_summaries | length) == 6
  and (.final_objective_tile_ids | length) == 4
  and .final_objective_owner_state == "bot:Multi2:beacons=2"
  and .final_objective_result_state == "terminal_victory:Multi2:2_of_4_beacons"
  and (.final_objective_score_delta_log | index("beacon2:Multi2") != null)
  and (.final_objective_score_delta_log | index("beacon3:Multi2") != null)
  and .final_match_result_state == "victory:bot_terminal:Multi2"
  and .final_objective_status == "autonomous_bot_terminal_complete:Multi2:2_of_4"
  and (.final_ai_wave_unit_ids | length) == 4
  and (.final_enemy_pressure_wave_unit_ids | length) >= 3
  and (.final_resource_delta_log | index("Multi2:objective:+250") != null)
  and (.final_army_production_batch_ids | length) >= 3
  and (.final_army_spawned_unit_ids | length) >= 5
  and (.final_army_rally_tile_ids | length) >= 5
  and (.final_army_composition_log | length) >= 5
  and .final_army_supply_used >= 14
  and .final_army_supply_cap >= 22
  and (.final_combat_event_log | index("Outcome:Multi2:Won") != null)
  and (.final_command_queue | index("bot_economy:mine:Multi0,Multi1,Multi2,Multi3") != null)
  and (.final_command_queue | index("bot_production:scout+warden+striker") != null)
  and (.final_command_queue | index("bot_combat:contest_beacon_lane") != null)
  and (.final_command_queue | index("bot_terminal_hold:Multi2:2_of_4@3000") != null)
  and .forced_capture_hook_enabled == false
  and .forced_surrender_hook_enabled == false
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .autonomous_timeline_gate == true
  and .bot_roster_gate == true
  and .economy_gate == true
  and .production_gate == true
  and .combat_gate == true
  and .terminal_gate == true
  and .renderer_gate == true
  and .autonomous_bot_skirmish_gate == true
  and .rts_autonomous_bot_core_frame_order_gate == true
  and .rts_autonomous_bot_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_AUTONOMOUS_BOT_SKIRMISH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
