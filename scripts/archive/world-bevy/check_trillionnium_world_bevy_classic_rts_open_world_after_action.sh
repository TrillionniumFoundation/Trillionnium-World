#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-open-world-after-action.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-open-world-after-action.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-open-world-after-action "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_open_world_after_action_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .write_gate == true
  and .runtime_screen_mode == "player_runtime_open_world_after_action_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .input_path == "apply_live_native_action_with_source(classic_rts_open_world_after_action_input)"
  and .input_action_count == 3
  and .accepted_input_count == 3
  and (.action_labels | index("RTS:QUEUE:tier2:open_world:after_action@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:open_world_route:league-coliseum@12,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:open_world_resume:league-coliseum@12,3") != null)
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_outdoor"
  and .final_route_director_task_id == "task-fixture-first-route"
  and .final_route_director_target_room_id == "league-coliseum"
  and .final_route_director_next_room_id == null
  and (.final_route_director_history | index("rts_open_world_after_action:league-coliseum:arrived") != null)
  and (.final_open_world_route_tile_ids | length) >= 5
  and (.final_open_world_route_tile_ids | index("13,3") != null)
  and (.final_open_world_route_tile_ids | index("9,2") != null)
  and (.final_open_world_panel_ids | index("task_panel:task-fixture-first-route") != null)
  and (.final_open_world_task_ids | index("task-fixture-first-route") != null)
  and .final_open_world_handoff_state == "resumed:league-coliseum"
  and .final_open_world_resume_room_id == "league-coliseum"
  and (.final_contextual_action_labels | index("COMBAT:attack") != null)
  and (.final_active_task_ids | index("task-fixture-first-route") != null)
  and .final_objective_status == "open_world_after_action_ready"
  and (.final_next_action_ids | index("open_world_after_action") != null)
  and (.final_base_assault_reward_log | index("open_world_after_action:+route_resume_ready") != null)
  and (.final_command_queue | index("tier2_open_world_resume:league-coliseum@12,3:room=league-coliseum") != null)
  and .non_background_pixels > 140000
  and .open_world_route_pixel_count > 40
  and .open_world_panel_pixel_count > 30
  and .open_world_resume_pixel_count > 20
  and .open_world_after_action_pixel_counts.player_first_open_world_view_non_background > 250000
  and .open_world_after_action_pixel_counts.player_first_open_world_view_frame > 8000
  and .open_world_after_action_pixel_counts.player_first_open_world_status_strip > 20000
  and .open_world_after_action_pixel_counts.player_first_open_world_route_panel > 90000
  and .open_world_after_action_pixel_counts.player_first_open_world_timeline > 10000
  and .live_open_world_input_gate == true
  and .restoration_dependency_gate == true
  and .open_world_route_gate == true
  and .open_world_panel_gate == true
  and .open_world_resume_gate == true
  and .command_gate == true
  and .player_first_open_world_after_action_screen_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPEN_WORLD_AFTER_ACTION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
