#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-harassment-defense-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-harassment-defense-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-bot-harassment-defense-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_harassment_defense_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_harassment_defense_gap_state == "bevy_harassment_defense_vocabulary_not_openra_native_harassment_ai"
  and .bevy_native_harassment_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .harassment_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("worker_line_probe") != null)
  and (.stage_summaries | map(.stage) | index("worker_pullback_split") != null)
  and (.stage_summaries | map(.stage) | index("repair_and_body_block") != null)
  and (.stage_summaries | map(.stage) | index("turret_zone_response") != null)
  and (.stage_summaries | map(.stage) | index("counter_raid_timing") != null)
  and (.stage_summaries | map(.stage) | index("rebuild_route_secure") != null)
  and .harassment_signal_count >= 24
  and .worker_pullback_count >= 4
  and .repair_cycle_count >= 3
  and .static_defense_response_count >= 3
  and .counter_raid_count >= 3
  and .retreat_path_count >= 2
  and .rebuild_secure_count >= 2
  and .final_harassment_state == "counter_raid_rebuild_secured"
  and .final_rts_ai_pressure_percent >= 80
  and .final_rts_defeat_risk_percent <= 20
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "harassment_defense_gap:counter_raid_rebuild_secured"
  and (.final_command_queue | index("harassment_stage:rebuild_route_secure") != null)
  and (.final_command_queue | index("native_openra_harassment_ai:false") != null)
  and (.final_army_production_batch_ids | index("harassment_defense:worker_pullback_split") != null)
  and (.final_army_production_batch_ids | index("harassment_defense:counter_raid_timing") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_bot_harassment_defense_core_subject_actor_ids | length) == 3
  and (.rts_bot_harassment_defense_core_action_labels | length) == 7
  and .rts_bot_harassment_defense_core_frame_order_stream.map_id == "first-contact-basin-bot-harassment-defense"
  and .rts_bot_harassment_defense_core_frame_order_stream.rules_id == "trnm-rts-core-bot-harassment-defense-rules-v1"
  and .rts_bot_harassment_defense_core_frame_order_kind_labels == ["recon", "move", "repair", "build", "attack", "move", "build"]
  and (.rts_bot_harassment_defense_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_bot_harassment_defense_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_bot_harassment_defense_core_frame_order_errors | length) == 0
  and .rts_bot_harassment_defense_core_frame_order_stream_error == null
  and .rts_bot_harassment_defense_core_headless_replay_error == null
  and .rts_bot_harassment_defense_core_headless_applied_order_count == 7
  and .rts_bot_harassment_defense_core_headless_actor_count >= 3
  and .rts_bot_harassment_defense_core_headless_final_frame == 2006
  and .rts_bot_harassment_defense_core_headless_recon_order_count == 1
  and .rts_bot_harassment_defense_core_headless_scout_order_count == 1
  and (.rts_bot_harassment_defense_core_headless_recon_ids | index("worker_line_probe") != null)
  and (.rts_bot_harassment_defense_core_headless_recon_tile_ids | index("4,5") != null)
  and .rts_bot_harassment_defense_core_headless_build_order_count == 2
  and .rts_bot_harassment_defense_core_headless_repair_order_count == 1
  and (.rts_bot_harassment_defense_core_headless_build_rule_ids | index("static_defense_turret") != null)
  and (.rts_bot_harassment_defense_core_headless_build_rule_ids | index("rebuild_route_relay") != null)
  and (.rts_bot_harassment_defense_core_headless_repair_target_ids | index("relay_turret") != null)
  and .rts_bot_harassment_defense_core_headless_attack_order_count == 1
  and .rts_bot_harassment_defense_core_headless_micro_move_order_count == 2
  and (.rts_bot_harassment_defense_core_headless_combat_target_actor_ids | index("enemy_expand_counter_raid") != null)
  and (.rts_bot_harassment_defense_core_headless_combat_target_tile_ids | index("7,4") != null)
  and (.rts_bot_harassment_defense_core_headless_combat_formation_ids | index("worker_pullback_split") != null)
  and (.rts_bot_harassment_defense_core_headless_combat_formation_ids | index("retreat_path_rejoin") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .harassment_stage_gate == true
  and .harassment_signal_gate == true
  and .harassment_worker_gate == true
  and .harassment_repair_gate == true
  and .harassment_static_defense_gate == true
  and .harassment_counter_raid_gate == true
  and .harassment_retreat_gate == true
  and .harassment_rebuild_gate == true
  and .bevy_gap_gate == true
  and .openra_harassment_defense_target_gate == true
  and .renderer_gate == true
  and .harassment_defense_gap_gate == true
  and .rts_bot_harassment_defense_core_frame_order_gate == true
  and .rts_bot_harassment_defense_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_HARASSMENT_DEFENSE_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
