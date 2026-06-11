#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-bot-map-intel-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_map_intel_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_map_intel_gap_state == "bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai"
  and .bevy_native_shroud_memory_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .intel_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("initial_scout_sweep") != null)
  and (.stage_summaries | map(.stage) | index("fog_memory_stamp") != null)
  and (.stage_summaries | map(.stage) | index("expansion_threat_inference") != null)
  and (.stage_summaries | map(.stage) | index("enemy_tech_read") != null)
  and (.stage_summaries | map(.stage) | index("hidden_army_prediction") != null)
  and (.stage_summaries | map(.stage) | index("rotate_pressure_reveal") != null)
  and .intel_signal_count >= 24
  and .scout_sweep_count >= 3
  and .fog_memory_stamp_count >= 4
  and .expansion_threat_count >= 3
  and .enemy_tech_read_count >= 2
  and .hidden_army_prediction_count >= 2
  and .pressure_rotation_count >= 2
  and .final_intel_state == "rotate_pressure_confirmed_beacon"
  and .final_rts_ai_pressure_percent >= 80
  and .final_rts_defeat_risk_percent <= 20
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "map_intel_gap:rotate_pressure_confirmed_beacon"
  and (.final_command_queue | index("intel_stage:rotate_pressure_reveal") != null)
  and (.final_command_queue | index("native_openra_shroud_memory_ai:false") != null)
  and (.final_army_production_batch_ids | index("map_intel:fog_memory_last_seen_grid") != null)
  and (.final_army_production_batch_ids | index("map_intel:rotate_pressure_to_confirmed_beacon") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_bot_map_intel_core_subject_actor_ids | length) == 3
  and (.rts_bot_map_intel_core_action_labels | length) == 6
  and .rts_bot_map_intel_core_frame_order_stream.map_id == "first-contact-basin-bot-map-intel"
  and .rts_bot_map_intel_core_frame_order_stream.rules_id == "trnm-rts-core-bot-map-intel-rules-v1"
  and .rts_bot_map_intel_core_frame_order_kind_labels == ["recon", "recon", "recon", "recon", "recon", "move"]
  and (.rts_bot_map_intel_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_bot_map_intel_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_bot_map_intel_core_frame_order_errors | length) == 0
  and .rts_bot_map_intel_core_frame_order_stream_error == null
  and .rts_bot_map_intel_core_headless_replay_error == null
  and .rts_bot_map_intel_core_headless_applied_order_count == 6
  and .rts_bot_map_intel_core_headless_actor_count >= 3
  and .rts_bot_map_intel_core_headless_final_frame == 1605
  and .rts_bot_map_intel_core_headless_recon_order_count == 5
  and .rts_bot_map_intel_core_headless_scout_order_count == 2
  and .rts_bot_map_intel_core_headless_mark_order_count == 1
  and .rts_bot_map_intel_core_headless_sweep_order_count == 1
  and .rts_bot_map_intel_core_headless_scan_order_count == 1
  and (.rts_bot_map_intel_core_headless_recon_ids | index("three_lane_scout_sweep") != null)
  and (.rts_bot_map_intel_core_headless_recon_ids | index("fog_memory_last_seen_grid") != null)
  and (.rts_bot_map_intel_core_headless_recon_ids | index("natural_expand_threat") != null)
  and (.rts_bot_map_intel_core_headless_recon_ids | index("enemy_signal_array_tech") != null)
  and (.rts_bot_map_intel_core_headless_recon_ids | index("hidden_army_fog_gap") != null)
  and (.rts_bot_map_intel_core_headless_recon_tile_ids | index("5,5") != null)
  and (.rts_bot_map_intel_core_headless_recon_tile_ids | index("8,4") != null)
  and .rts_bot_map_intel_core_headless_micro_move_order_count == 1
  and (.rts_bot_map_intel_core_headless_combat_target_tile_ids | index("9,5") != null)
  and (.rts_bot_map_intel_core_headless_combat_formation_ids | index("rotate_pressure_to_confirmed_beacon") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .intel_stage_gate == true
  and .intel_signal_gate == true
  and .intel_scout_gate == true
  and .intel_fog_memory_gate == true
  and .intel_expansion_gate == true
  and .intel_tech_gate == true
  and .intel_hidden_army_gate == true
  and .intel_rotation_gate == true
  and .bevy_gap_gate == true
  and .openra_map_intel_target_gate == true
  and .renderer_gate == true
  and .map_intel_gap_gate == true
  and .rts_bot_map_intel_core_frame_order_gate == true
  and .rts_bot_map_intel_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_MAP_INTEL_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
