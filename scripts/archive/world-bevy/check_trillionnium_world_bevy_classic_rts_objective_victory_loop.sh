#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-objective-victory-loop "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_objective_victory_loop_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_objective_victory_loop_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:ai:skirmish_wave") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:guard_break") != null)
  and (.action_labels | index("RTS:QUEUE:objective:claim:relay_beacon@6,5") != null)
  and (.action_labels | index("RTS:QUEUE:objective:extract:relay_beacon@9,2") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_objective_core_frame_order_gate == true
  and .rts_objective_core_headless_replay_gate == true
  and (.rts_objective_core_frame_orders | length == 5)
  and (.rts_objective_core_frame_order_kind_labels | tostring == "[\"queue\",\"attack\",\"ability\",\"capture\",\"extract\"]")
  and (.rts_objective_core_frame_order_errors | length == 0)
  and .rts_objective_core_frame_order_stream_error == null
  and (.rts_objective_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_objective_core_headless_replay_error == null
  and (.rts_objective_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_objective_core_headless_applied_order_count == 5
  and .rts_objective_core_headless_actor_count >= 4
  and .rts_objective_core_headless_final_frame == 804
  and .rts_objective_core_headless_objective_order_count == 2
  and .rts_objective_core_headless_capture_order_count == 1
  and .rts_objective_core_headless_extract_order_count == 1
  and (.rts_objective_core_headless_objective_ids | index("relay_beacon") != null)
  and (.rts_objective_core_headless_objective_tile_ids | index("6,5") != null)
  and (.rts_objective_core_headless_objective_tile_ids | index("9,2") != null)
  and (.final_objective_tile_ids | length) == 4
  and .final_objective_capture_percent == 100
  and .final_objective_owner_state == "player:relay_beacon"
  and .final_objective_result_state == "victory:relay_beacon_extracted"
  and .final_objective_extraction_tile_id == "9,2"
  and .final_defeat_risk_percent <= 8
  and .final_ai_pressure_percent <= 34
  and (.final_objective_score_delta_log | index("victory:+250xp:+120g") != null)
  and (.final_command_queue | index("objective_claim:relay_beacon@6,5") != null)
  and (.final_command_queue | index("extract:relay_beacon@9,2") != null)
  and (.final_command_queue | index("victory:relay_beacon") != null)
  and .non_background_pixels > 250000
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .victory_pixel_count > 20
  and .defeat_risk_pixel_count > 5
  and .extraction_pixel_count > 40
  and .live_objective_input_gate == true
  and .objective_marker_gate == true
  and .capture_progress_gate == true
  and .victory_resolution_gate == true
  and .defeat_pressure_gate == true
  and .extraction_gate == true
  and .openra_parity_target_commit == "5f1bf76"
  and .openra_parity_target_package == "dist/trillionnium-rts-playtest-20260522T065052Z-5f1bf76.tar.gz"
  and .openra_parity_target_natural_terminal == true
  and .openra_parity_target_winner == "Multi2"
  and .openra_parity_target_replay_outcome == "3W/1L"
  and .openra_parity_target_winner_beacons == 2
  and .openra_parity_target_total_beacons == 4
  and .openra_parity_target_hold_ticks == 3000
  and .bevy_openra_parity_state == "catching_up_not_claimed"
  and .bevy_terminal_parity_claimed == false
  and .bevy_objective_loop_kind == "scripted_live_input_objective_loop"
  and .bevy_objective_controlled_beacons == 2
  and .bevy_objective_total_beacons == 4
  and .bevy_objective_control_ratio_percent == 50
  and .bevy_objective_hold_ticks == 3000
  and .bevy_objective_terminal_rule == "control_2_of_4_flux_beacons_for_3000_ticks"
  and .openra_parity_bridge_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OBJECTIVE_VICTORY_LOOP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
