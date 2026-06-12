#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-multi-front-pressure-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-multi-front-pressure-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-multi-front-pressure-gap "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_multi_front_pressure_gap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_action_count == 0
  and .bevy_bot_multi_front_pressure_gap_state == "bevy_multi_front_pressure_vocabulary_not_openra_native_split_map_ai"
  and .bevy_native_multi_front_ai_claimed == false
  and .bevy_openra_parity_claimed == false
  and .openra_gap_not_closed_gate == true
  and .openra_bot_economy_tech_target_commit == "f6c47d9"
  and .openra_bot_beacon_pressure_target_commit == "2b6f25b"
  and .openra_organic_bot_terminal_target_commit == "5f1bf76"
  and .multi_front_stage_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("dual_scout_lane_probe") != null)
  and (.stage_summaries | map(.stage) | index("decoy_beacon_pressure") != null)
  and (.stage_summaries | map(.stage) | index("main_force_rotate") != null)
  and (.stage_summaries | map(.stage) | index("reinforce_cross_map") != null)
  and (.stage_summaries | map(.stage) | index("simultaneous_expand_hit") != null)
  and (.stage_summaries | map(.stage) | index("collapse_to_terminal") != null)
  and .multi_front_signal_count >= 24
  and .split_lane_count >= 2
  and .decoy_pressure_count >= 3
  and .rotation_count >= 3
  and .reinforce_join_count >= 3
  and .simultaneous_hit_count >= 2
  and .terminal_collapse_count >= 2
  and .final_multi_front_state == "terminal_collapse_secured"
  and .final_rts_ai_pressure_percent >= 80
  and .final_rts_defeat_risk_percent <= 20
  and .final_objective_capture_percent >= 90
  and .final_match_result_state == "multi_front_pressure_gap:terminal_collapse_secured"
  and (.final_command_queue | index("multi_front_stage:collapse_to_terminal") != null)
  and (.final_command_queue | index("native_openra_multi_front_ai:false") != null)
  and (.final_army_production_batch_ids | index("multi_front:decoy_beacon_pressure") != null)
  and (.final_army_production_batch_ids | index("multi_front:collapse_to_terminal") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_bot_multi_front_core_subject_actor_ids | length) == 3
  and (.rts_bot_multi_front_core_action_labels | length) == 7
  and .rts_bot_multi_front_core_frame_order_stream.map_id == "first-contact-basin-bot-multi-front-pressure"
  and .rts_bot_multi_front_core_frame_order_stream.rules_id == "trnm-rts-core-bot-multi-front-pressure-rules-v1"
  and .rts_bot_multi_front_core_frame_order_kind_labels == ["recon", "move", "attack", "move", "move", "attack", "move"]
  and (.rts_bot_multi_front_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.rts_bot_multi_front_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and (.rts_bot_multi_front_core_frame_order_errors | length) == 0
  and .rts_bot_multi_front_core_frame_order_stream_error == null
  and .rts_bot_multi_front_core_headless_replay_error == null
  and .rts_bot_multi_front_core_headless_applied_order_count == 7
  and .rts_bot_multi_front_core_headless_actor_count >= 3
  and .rts_bot_multi_front_core_headless_final_frame == 2206
  and .rts_bot_multi_front_core_headless_recon_order_count == 1
  and .rts_bot_multi_front_core_headless_scout_order_count == 1
  and (.rts_bot_multi_front_core_headless_recon_ids | index("dual_scout_lane_probe") != null)
  and (.rts_bot_multi_front_core_headless_recon_tile_ids | index("4,5") != null)
  and .rts_bot_multi_front_core_headless_attack_order_count == 2
  and .rts_bot_multi_front_core_headless_micro_move_order_count == 4
  and (.rts_bot_multi_front_core_headless_combat_target_actor_ids | index("decoy_beacon_pressure") != null)
  and (.rts_bot_multi_front_core_headless_combat_target_actor_ids | index("simultaneous_expand_hit") != null)
  and (.rts_bot_multi_front_core_headless_combat_target_tile_ids | index("9,2") != null)
  and (.rts_bot_multi_front_core_headless_combat_formation_ids | index("split_lane_probe") != null)
  and (.rts_bot_multi_front_core_headless_combat_formation_ids | index("main_force_rotate") != null)
  and (.rts_bot_multi_front_core_headless_combat_formation_ids | index("reinforce_cross_map") != null)
  and (.rts_bot_multi_front_core_headless_combat_formation_ids | index("collapse_to_terminal") != null)
  and .non_background_pixels > 250000
  and .ai_wave_pixel_count > 80
  and .ai_pressure_pixel_count > 120
  and .ai_counter_pixel_count > 80
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .match_result_pixel_count > 20
  and .multi_front_stage_gate == true
  and .multi_front_signal_gate == true
  and .multi_front_split_gate == true
  and .multi_front_decoy_gate == true
  and .multi_front_rotation_gate == true
  and .multi_front_reinforce_gate == true
  and .multi_front_simultaneous_gate == true
  and .multi_front_terminal_gate == true
  and .bevy_gap_gate == true
  and .openra_multi_front_pressure_target_gate == true
  and .renderer_gate == true
  and .multi_front_pressure_gap_gate == true
  and .rts_bot_multi_front_core_frame_order_gate == true
  and .rts_bot_multi_front_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_MULTI_FRONT_PRESSURE_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
