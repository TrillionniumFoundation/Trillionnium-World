#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-live-input-sequence "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_live_input_sequence_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 2520
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_live_input)"
  and .input_action_count == 10
  and .accepted_input_count == 10
  and (.input_sources | index("classic_rts_live_input") != null)
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:train:guard") != null)
  and (.action_labels | index("RTS:MOVE:7,4:diamond") != null)
  and (.action_labels | index("RTS:MOVE:9,4:shift_waypoint") != null)
  and (.action_labels | index("RTS:MOVE:6,5:hold") != null)
  and (.action_labels | index("RTS:MOVE:9,4:patrol") != null)
  and (.action_labels | index("RTS:MOVE:10,3:attack_move") != null)
  and (.action_labels | index("RTS:MOVE:10,3:stop") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:focus_fire") != null)
  and .non_background_pixels > 300000
  and .selection_marker_pixel_count > 1000
  and .command_marker_pixel_count > 600
  and .production_queue_pixel_count > 1000
  and .ability_command_pixel_count > 800
  and .target_health_pixel_count > 60
  and .hover_preview_pixel_count > 40
  and .context_cursor_pixel_count > 80
  and .drag_select_preview_pixel_count > 80
  and .command_stamp_pixel_count > 120
  and (.stage_summaries | any(.stage == "move_formation" and .command_stamp_player_label == "MAP MOVE SENT 7,4" and .command_stamp_tile_id == "7,4"))
  and (.stage_summaries | any(.stage == "attack_target" and .command_stamp_player_label == "MAP ATTACK SENT ARENA CREEP ATTACK" and .command_stamp_target_id == "arena_creep_attack"))
  and (.stage_summaries | any(.stage == "cast_focus_fire" and .command_stamp_player_label == "COMMAND ABILITY SENT FOCUS FIRE" and .command_stamp_kind == "ability"))
  and (.stage_summaries | all(.command_stamp_player_label | (contains("feedback") or contains("rts_")) | not))
  and (.drag_select_preview_samples | length == 1)
  and (.drag_select_preview_samples | any(
    .input_source == "classic_rts_mouse_drag"
    and .start_tile_id == "2,2"
    and .current_tile_id == "6,4"
    and .player_label == "DRAG SELECT 2 UNITS 2,2->6,4"
    and (.selection_tile_ids | length >= 12)
    and (.candidate_unit_ids | length >= 2)
  ))
  and (.drag_select_preview_samples | all(.player_label | (contains("feedback") or contains("rts_")) | not))
  and .drag_select_commit_selection_marker_pixel_count > 250
  and .drag_select_commit_stamp_pixel_count > 80
  and .drag_select_commit_sample.accepted == true
  and .drag_select_commit_sample.input_source == "classic_rts_mouse_drag"
  and .drag_select_commit_sample.group_id == "1"
  and .drag_select_commit_sample.group_command_state == "drag:2,2->6,4"
  and (.drag_select_commit_sample.selected_unit_ids | length == 2)
  and (.drag_select_commit_sample.selected_unit_ids | index("player") != null)
  and (.drag_select_commit_sample.selected_unit_ids | index("square_guard_front") != null)
  and (.drag_select_commit_sample.selection_tile_ids | length == 15)
  and (.drag_select_commit_sample.command_queue | any(startswith("drag_select:")))
  and (.drag_select_commit_sample.command_queue | index("select_group_1") != null)
  and .drag_select_commit_sample.command_stamp_player_label == "DRAG SELECT SENT 2 UNITS"
  and (.hover_samples | length == 4)
  and (.hover_samples | any(.player_label == "MAP MOVE READY 4,3"))
  and (.hover_samples | any(.player_label | startswith("SIDEBAR QUEUE READY WATCH TOWER")))
  and (.hover_samples | any(.player_label | startswith("COMMAND BAR ABILITY READY")))
  and (.hover_samples | any(.player_label == "MINIMAP RALLY READY 5,2"))
  and (.hover_samples | all(.player_label | (contains("feedback") or contains("rts_")) | not))
  and (.context_cursor_samples | length == 4)
  and (.context_cursor_samples | any(.player_label == "MAP CURSOR MOVE READY" and .cursor_kind == "move" and .allowed == true))
  and (.context_cursor_samples | any(.player_label == "SIDEBAR CURSOR BUILD READY" and .cursor_kind == "build" and .allowed == true))
  and (.context_cursor_samples | any(.player_label == "COMMAND BAR CURSOR ABILITY READY" and .cursor_kind == "ability" and .allowed == true))
  and (.context_cursor_samples | any(.player_label == "MINIMAP CURSOR RALLY READY" and .cursor_kind == "rally" and .allowed == true))
  and (.context_cursor_samples | all(.player_label | (contains("feedback") or contains("rts_")) | not))
  and .final_hover_source == "classic_rts_mouse_minimap"
  and .final_hover_tile_id == "5,2"
  and .final_hover_player_label == "MINIMAP RALLY READY 5,2"
  and .final_context_cursor_source == "classic_rts_mouse_minimap"
  and .final_context_cursor_tile_id == "5,2"
  and .final_context_cursor_kind == "rally"
  and .final_context_cursor_player_label == "MINIMAP CURSOR RALLY READY"
  and .final_context_cursor_allowed == true
  and .final_command_stamp_kind == "ability"
  and .final_command_stamp_tile_id == "6,5"
  and .final_command_stamp_target_id == "arena_creep_attack"
  and .final_command_stamp_player_label == "COMMAND ABILITY SENT FOCUS FIRE"
  and (.final_command_queue | index("select_group_1") != null)
  and (.final_command_queue | index("move:7,4") != null)
  and (.final_command_queue | index("formation:diamond") != null)
  and (.final_command_queue | index("move:9,4") != null)
  and (.final_command_queue | index("formation:shift_waypoint") != null)
  and (.final_command_queue | index("formation:hold") != null)
  and (.final_command_queue | index("formation:patrol") != null)
  and (.final_command_queue | index("formation:attack_move") != null)
  and (.final_command_queue | index("formation:stop") != null)
  and (.final_command_queue | index("stop:10,3") != null)
  and (.final_command_queue | index("command_queue_path_preview:shift_waypoints") != null)
  and (.final_command_queue | index("command_queue_path_preview:queue_stack") != null)
  and (.final_command_queue | index("command_queue_path_preview:rally_chain") != null)
  and (.final_command_queue | index("command_queue_path_preview:attack_focus") != null)
  and (.final_command_queue | index("command_queue_path_preview:cancel_repath") != null)
  and (.final_command_queue | index("feedback:diamond@7,4") != null)
  and (.final_command_queue | index("feedback:train_queued:guard") != null)
  and (.final_command_queue | index("feedback:waypoint_queued@9,4") != null)
  and (.final_command_queue | index("feedback:hold_position@6,5") != null)
  and (.final_command_queue | index("feedback:patrol_route@9,4") != null)
  and (.final_command_queue | any(startswith("feedback:attack_move@10,3:")))
  and (.final_command_queue | index("feedback:stop_hold@10,3") != null)
  and (.final_command_queue | index("attack:arena_creep_attack") != null)
  and (.final_command_queue | index("ability:focus_fire") != null)
  and .command_feedback_chip_count >= 6
  and (.final_production_queue | index("train:guard") != null)
  and .final_attack_target_id == "arena_creep_attack"
  and .final_active_ability_id == "focus_fire"
  and .final_target_health_percent < 60
  and (.final_combat_event_log | index("damage:28") != null)
  and .live_input_gate == true
  and .selection_live_gate == true
  and .production_live_gate == true
  and .production_feedback_chip_gate == true
  and .move_live_gate == true
  and .waypoint_live_gate == true
  and .hold_live_gate == true
  and .patrol_live_gate == true
  and .attack_move_live_gate == true
  and .stop_live_gate == true
  and .attack_live_gate == true
  and .ability_live_gate == true
  and .command_feedback_chip_gate == true
  and .hover_preview_gate == true
  and .context_cursor_gate == true
  and .drag_select_preview_gate == true
  and .drag_select_commit_gate == true
  and .command_stamp_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_INPUT_SEQUENCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
