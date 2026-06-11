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
  and .drag_select_filter_selection_marker_pixel_count > 400
  and .drag_select_filter_stamp_pixel_count > 80
  and .drag_select_filter_sample.accepted == true
  and .drag_select_filter_sample.input_source == "classic_rts_mouse_drag"
  and .drag_select_filter_sample.start_tile_id == "2,2"
  and .drag_select_filter_sample.current_tile_id == "10,5"
  and .drag_select_filter_sample.action_label == "RTS:SELECT:drag:2,2->10,5"
  and .drag_select_filter_sample.preview_player_label == "DRAG SELECT 5 UNITS 2,2->10,5"
  and (.drag_select_filter_sample.selected_unit_ids | length == 5)
  and (.drag_select_filter_sample.selected_unit_ids | index("player") != null)
  and (.drag_select_filter_sample.selected_unit_ids | index("square_guard_front") != null)
  and (.drag_select_filter_sample.selected_unit_ids | index("square_guard_patrol") != null)
  and (.drag_select_filter_sample.selected_unit_ids | index("square_worker_carry") != null)
  and (.drag_select_filter_sample.selected_unit_ids | index("square_worker_harvest") != null)
  and (.drag_select_filter_sample.selected_unit_ids | index("square_creep_wander") == null)
  and (.drag_select_filter_sample.preview_rejected_unit_ids | length == 1)
  and (.drag_select_filter_sample.preview_rejected_unit_ids | index("square_creep_wander") != null)
  and (.drag_select_filter_sample.selected_unit_allegiances | all(. == "player"))
  and (.drag_select_filter_sample.selected_unit_priorities | tostring == "[0,1,2,3,4]")
  and (.drag_select_filter_sample.command_queue | any(startswith("drag_select:")))
  and (.drag_select_filter_sample.command_queue | index("select_group_1") != null)
  and .drag_select_filter_sample.command_stamp_player_label == "DRAG SELECT SENT 5 UNITS"
  and .drag_select_filter_gate == true
  and .unit_click_select_marker_pixel_count > 80
  and .unit_click_select_stamp_pixel_count > 80
  and .unit_click_select_sample.accepted == true
  and .unit_click_select_sample.input_source == "classic_rts_mouse_viewport"
  and .unit_click_select_sample.tile_id == "5,4"
  and .unit_click_select_sample.action_label == "RTS:SELECT:unit:player"
  and .unit_click_select_sample.group_id == "1"
  and .unit_click_select_sample.group_command_state == "unit_selected:player@5,4"
  and (.unit_click_select_sample.selected_unit_ids | length == 1)
  and (.unit_click_select_sample.selected_unit_ids | index("player") != null)
  and (.unit_click_select_sample.selection_tile_ids | length == 1)
  and (.unit_click_select_sample.selection_tile_ids | index("5,4") != null)
  and (.unit_click_select_sample.command_queue | index("unit_select:player@5,4") != null)
  and (.unit_click_select_sample.command_queue | index("select_group_1") != null)
  and .unit_click_select_sample.command_stamp_player_label == "MAP SELECT SENT 1 UNIT"
  and .selection_clear_stamp_pixel_count > 80
  and .selection_clear_command_disabled_pixel_count > 500
  and .selection_clear_residual_marker_pixel_count < 80
  and .selection_clear_gate == true
  and (.selection_clear_samples | length == 2)
  and (.selection_clear_samples | all(
    .accepted == true
    and (.selected_unit_ids | length == 0)
    and (.selection_tile_ids | length == 0)
    and .group_id == null
    and (.active_control_group_ids | length == 0)
    and .move_command_available == false
    and .move_command_availability_reason == "rts_group_selection_required"
    and .command_slot_move_available == false
    and .selection_marker_pixel_count < 80
    and .command_disabled_pixel_count > 500
  ))
  and (.selection_clear_samples | any(
    .stage == "empty_viewport_clear"
    and .tile_id == "4,3"
    and .action_label == "RTS:SELECT:clear:empty@4,3"
    and .group_command_state == "selection_cleared:empty@4,3"
    and (.command_queue | index("selection_clear:empty@4,3") != null)
    and .command_stamp_player_label == "MAP SELECTION CLEARED"
  ))
  and (.selection_clear_samples | any(
    .stage == "hostile_viewport_clear"
    and .tile_id == "9,4"
    and .action_label == "RTS:SELECT:clear:hostile:square_creep_wander@9,4"
    and .group_command_state == "selection_cleared:hostile:square_creep_wander@9,4"
    and (.command_queue | index("selection_clear:hostile:square_creep_wander@9,4") != null)
    and .command_stamp_player_label == "MAP SELECTION CLEARED HOSTILE"
  ))
  and .right_click_target_attack_marker_pixel_count > 20
  and .right_click_target_stamp_pixel_count > 80
  and .right_click_target_selection_marker_pixel_count > 120
  and .right_click_target_hover_sample.action_label == "RTS:ATTACK:square_creep_wander"
  and .right_click_target_hover_sample.player_label == "MAP ATTACK READY SQUARE CREEP WANDER"
  and .right_click_target_hover_sample.cursor_kind == "attack"
  and .right_click_target_hover_sample.cursor_player_label == "MAP CURSOR ATTACK READY"
  and .right_click_target_hover_sample.target_preview_kind == "attack"
  and .right_click_target_hover_sample.target_preview_source_tile_id == "5,4"
  and .right_click_target_hover_sample.target_preview_attack_pixel_count > 80
  and .right_click_target_hover_sample.target_preview_path_pixel_count > 80
  and .right_click_target_sample.accepted == true
  and .right_click_target_sample.input_source == "classic_rts_mouse_viewport"
  and .right_click_target_sample.stage == "drag_filter_then_right_click_hostile"
  and .right_click_target_sample.tile_id == "9,4"
  and .right_click_target_sample.action_label == "RTS:ATTACK:square_creep_wander"
  and .right_click_target_sample.target_id == "square_creep_wander"
  and .right_click_target_sample.command_destination_tile == "9,4"
  and (.right_click_target_sample.selected_unit_ids | length == 5)
  and (.right_click_target_sample.selected_unit_ids | index("square_creep_wander") == null)
  and (.right_click_target_sample.target_priority_ids | index("square_creep_wander") != null)
  and (.right_click_target_sample.target_priority_ids | index("forest_creep_camp") != null)
  and (.right_click_target_sample.combat_event_log | index("target_acquired:square_creep_wander") != null)
  and (.right_click_target_sample.command_queue | any(startswith("drag_select:")))
  and (.right_click_target_sample.command_queue | index("attack:square_creep_wander") != null)
  and .right_click_target_sample.command_stamp_tile_id == "9,4"
  and .right_click_target_sample.command_stamp_target_id == "square_creep_wander"
  and .right_click_target_sample.command_stamp_player_label == "MAP ATTACK SENT SQUARE CREEP WANDER"
  and .right_click_target_attack_gate == true
  and (.right_click_target_samples | length == 4)
  and (.right_click_target_hover_samples | length == 4)
  and .right_click_target_preview_path_pixel_count > 300
  and .right_click_target_preview_attack_pixel_count > 80
  and .right_click_target_preview_follow_pixel_count > 80
  and .right_click_target_preview_harvest_pixel_count > 80
  and .right_click_target_preview_gate == true
  and .right_click_target_follow_stamp_pixel_count > 80
  and .right_click_target_harvest_stamp_pixel_count > 80
  and .right_click_execution_feedback_frame_pixel_count > 800
  and .right_click_execution_feedback_path_pixel_count > 300
  and .right_click_execution_feedback_target_pixel_count > 80
  and .right_click_execution_feedback_follow_pixel_count > 80
  and .right_click_execution_feedback_harvest_pixel_count > 80
  and .right_click_execution_feedback_viewport_marker_pixel_count > 500
  and .right_click_execution_feedback_label_pixel_count > 700
  and .right_click_execution_feedback_player_label_gate == true
  and (.right_click_target_hover_samples | any(
    .stage == "right_click_empty_move"
    and .action_label == "RTS:MOVE:4,3:line"
    and .player_label == "MAP MOVE READY 4,3"
    and .cursor_kind == "move"
    and .target_preview_kind == "move"
    and .target_preview_source_tile_id == "5,4"
    and .target_preview_path_pixel_count > 160
  ))
  and (.right_click_target_samples | any(
    .stage == "right_click_empty_move"
    and .accepted == true
    and .action_label == "RTS:MOVE:4,3:line"
    and .command_destination_tile == "4,3"
    and .group_command_state == "route:line:4,3"
    and (.command_queue | index("move:4,3") != null)
    and (.command_queue | index("formation:line") != null)
    and .command_stamp_kind == "move"
    and .command_stamp_player_label == "MAP MOVE SENT 4,3"
    and .command_stamp_pixel_count > 80
    and .execution_feedback_kind == "move"
    and .execution_feedback_renderer_path == "classic_draw_scene+classic_draw_rts_command_execution_feedback_overlay"
    and .execution_feedback_source_tile_id == "5,4"
    and .execution_feedback_destination_tile_id == "4,3"
    and .execution_feedback_player_label == "MOVE EXECUTING 4,3"
    and .execution_feedback_label_pixel_count > 160
    and .execution_feedback_path_pixel_count > 140
    and .execution_feedback_viewport_marker_pixel_count > 80
  ))
  and (.right_click_target_hover_samples | any(
    .stage == "right_click_friendly_follow"
    and .action_label == "RTS:MOVE:5,4:follow:player"
    and .player_label == "MAP FOLLOW READY PLAYER"
    and .cursor_kind == "follow"
    and .target_preview_kind == "follow"
    and .target_preview_source_tile_id == "5,4"
    and .target_preview_follow_pixel_count > 80
    and .target_preview_path_pixel_count > 80
  ))
  and (.right_click_target_samples | any(
    .stage == "right_click_friendly_follow"
    and .accepted == true
    and .action_label == "RTS:MOVE:5,4:follow:player"
    and .command_destination_tile == "5,4"
    and .group_command_state == "follow:player@5,4"
    and .unit_response_state == "following:player"
    and (.command_queue | index("follow:player@5,4") != null)
    and (.command_queue | index("feedback:follow@5,4:player") != null)
    and .command_stamp_kind == "follow"
    and .command_stamp_target_id == "player"
    and .command_stamp_player_label == "MAP FOLLOW SENT PLAYER"
    and .execution_feedback_kind == "follow"
    and .execution_feedback_renderer_path == "classic_draw_scene+classic_draw_rts_command_execution_feedback_overlay"
    and .execution_feedback_source_tile_id == "5,4"
    and .execution_feedback_follow_pixel_count > 80
    and .execution_feedback_player_label == "FOLLOWING PLAYER"
    and .execution_feedback_label_pixel_count > 160
    and .execution_feedback_viewport_marker_pixel_count > 80
  ))
  and (.right_click_target_hover_samples | any(
    .stage == "right_click_resource_harvest"
    and .action_label == "RTS:QUEUE:harvest:gold_vein"
    and .player_label == "MAP HARVEST READY GOLD VEIN"
    and .cursor_kind == "harvest"
    and .target_preview_kind == "harvest"
    and .target_preview_source_tile_id == "5,4"
    and .target_preview_harvest_pixel_count > 80
    and .target_preview_path_pixel_count > 80
  ))
  and (.right_click_target_samples | any(
    .stage == "right_click_resource_harvest"
    and .accepted == true
    and .action_label == "RTS:QUEUE:harvest:gold_vein"
    and .command_destination_tile == "3,3"
    and .minimap_command_kind == "harvest"
    and .minimap_command_tile_id == "3,3"
    and (.harvest_node_ids | index("gold_vein") != null)
    and (.worker_assignment_ids | length >= 2)
    and .economy_state == "harvesting:gold_vein"
    and (.command_queue | index("harvest:gold_vein->town_hall") != null)
    and (.command_queue | index("feedback:harvest_assigned:gold_vein") != null)
    and .command_stamp_kind == "harvest"
    and .command_stamp_tile_id == "3,3"
    and .command_stamp_target_id == "gold_vein"
    and .command_stamp_player_label == "MAP HARVEST SENT GOLD VEIN 3,3"
    and .execution_feedback_kind == "harvest"
    and .execution_feedback_renderer_path == "classic_draw_scene+classic_draw_rts_command_execution_feedback_overlay"
    and .execution_feedback_source_tile_id == "5,4"
    and .execution_feedback_destination_tile_id == "3,3"
    and .execution_feedback_dropoff_structure_id == "town_hall"
    and .execution_feedback_player_label == "HARVEST GOLD VEIN TO TOWN HALL"
    and .execution_feedback_label_pixel_count > 160
    and .execution_feedback_harvest_pixel_count > 80
    and .execution_feedback_viewport_marker_pixel_count > 80
  ))
  and (.right_click_target_samples | any(
    .stage == "drag_filter_then_right_click_hostile"
    and .execution_feedback_kind == "attack"
    and .execution_feedback_renderer_path == "classic_draw_scene+classic_draw_rts_command_execution_feedback_overlay"
    and .execution_feedback_source_tile_id == "5,4"
    and .execution_feedback_destination_tile_id == "9,4"
    and .execution_feedback_target_id == "square_creep_wander"
    and .execution_feedback_player_label == "ATTACK FOCUS SQUARE CREEP WANDER"
    and .execution_feedback_label_pixel_count > 160
    and .execution_feedback_target_pixel_count > 80
    and .execution_feedback_viewport_marker_pixel_count > 80
  ))
  and .right_click_target_move_gate == true
  and .right_click_target_follow_gate == true
  and .right_click_target_harvest_gate == true
  and .right_click_execution_feedback_gate == true
  and .right_click_target_semantics_gate == true
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_core_frame_order_gate == true
  and (.rts_core_frame_orders | length == 4)
  and (.rts_core_frame_order_errors | length == 0)
  and .rts_core_frame_order_stream_error == null
  and (.rts_core_frame_order_stream.orders | length == 4)
  and (.rts_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.rts_core_frame_order_kind_labels | tostring == "[\"move\",\"attack\",\"follow\",\"harvest\"]")
  and (.rts_core_frame_orders | any(
    .kind == "move"
    and .raw_command_label == "RTS:MOVE:4,3:line"
    and .target_tile.x == 4
    and .target_tile.y == 3
    and .formation_id == "line"
    and .source == "local_input"
  ))
  and (.rts_core_frame_orders | any(
    .kind == "attack"
    and .raw_command_label == "RTS:ATTACK:square_creep_wander"
    and .target_actor_id == "square_creep_wander"
    and .source == "local_input"
  ))
  and (.rts_core_frame_orders | any(
    .kind == "follow"
    and .raw_command_label == "RTS:MOVE:5,4:follow:player"
    and .target_actor_id == "player"
    and .target_tile.x == 5
    and .target_tile.y == 4
    and .formation_id == "follow"
  ))
  and (.rts_core_frame_orders | any(
    .kind == "harvest"
    and .raw_command_label == "RTS:QUEUE:harvest:gold_vein"
    and .target_actor_id == "gold_vein"
    and .queued == true
  ))
  and .unit_shift_select_marker_pixel_count > 80
  and .unit_shift_select_stamp_pixel_count > 80
  and (.unit_shift_select_samples | length == 3)
  and (.unit_shift_select_samples | any(
    .stage == "shift_add_patrol"
    and .accepted == true
    and .shift_pressed == true
    and .tile_id == "7,5"
    and .action_label == "RTS:SELECT:shift:unit:square_guard_patrol"
    and .group_command_state == "unit_shift_added:square_guard_patrol@7,5:count:2"
    and (.selected_unit_ids | length == 2)
    and (.selected_unit_ids | index("player") != null)
    and (.selected_unit_ids | index("square_guard_patrol") != null)
    and (.selection_tile_ids | index("5,4") != null)
    and (.selection_tile_ids | index("7,5") != null)
    and (.command_queue | index("unit_shift_add:square_guard_patrol@7,5") != null)
    and .command_stamp_player_label == "MAP SHIFT SELECT SENT 2 UNITS"
  ))
  and (.unit_shift_select_samples | any(
    .stage == "shift_remove_player"
    and .accepted == true
    and .shift_pressed == true
    and .tile_id == "5,4"
    and .action_label == "RTS:SELECT:shift:unit:player"
    and .group_command_state == "unit_shift_removed:player@5,4:count:1"
    and (.selected_unit_ids | length == 1)
    and (.selected_unit_ids | index("square_guard_patrol") != null)
    and (.selection_tile_ids | length == 1)
    and (.selection_tile_ids | index("7,5") != null)
    and (.command_queue | index("unit_shift_remove:player@5,4") != null)
    and .command_stamp_player_label == "MAP SHIFT SELECT SENT 1 UNIT"
  ))
  and (.unit_shift_select_samples | all(.command_stamp_player_label | (contains("feedback") or contains("rts_")) | not))
  and .unit_double_click_select_marker_pixel_count > 80
  and .unit_double_click_select_stamp_pixel_count > 80
  and .unit_double_click_select_sample.accepted == true
  and .unit_double_click_select_sample.input_source == "classic_rts_mouse_viewport"
  and .unit_double_click_select_sample.tile_id == "5,4"
  and .unit_double_click_select_sample.unit_class == "guard"
  and .unit_double_click_select_sample.action_label == "RTS:SELECT:double:unit:player"
  and .unit_double_click_select_sample.group_id == "1"
  and .unit_double_click_select_sample.group_command_state == "unit_double_selected:guard:count:3"
  and (.unit_double_click_select_sample.selected_unit_ids | length == 3)
  and (.unit_double_click_select_sample.selected_unit_ids | index("player") != null)
  and (.unit_double_click_select_sample.selected_unit_ids | index("square_guard_front") != null)
  and (.unit_double_click_select_sample.selected_unit_ids | index("square_guard_patrol") != null)
  and (.unit_double_click_select_sample.selection_tile_ids | length == 2)
  and (.unit_double_click_select_sample.selection_tile_ids | index("5,4") != null)
  and (.unit_double_click_select_sample.selection_tile_ids | index("7,5") != null)
  and (.unit_double_click_select_sample.control_group_assignments | index("1:double:guard:player|square_guard_front|square_guard_patrol") != null)
  and (.unit_double_click_select_sample.command_queue | index("unit_double_select:guard:player|square_guard_front|square_guard_patrol") != null)
  and .unit_double_click_select_sample.command_stamp_player_label == "MAP DOUBLE SELECT SENT 3 UNITS"
  and .control_group_hotkey_marker_pixel_count > 80
  and .control_group_hotkey_stamp_pixel_count > 80
  and .control_group_slot_pixel_count > 20
  and .control_group_slot_visual_gate == true
  and (.control_group_hotkey_samples | length == 5)
  and (.control_group_hotkey_samples | any(
    .stage == "ctrl_assign_group_5"
    and .accepted == true
    and .input_source == "classic_rts_hotkey"
    and .action_label == "RTS:SELECT:assign:5"
    and .group_id == "5"
    and .group_command_state == "group_5_assigned:2units"
    and (.selected_unit_ids | length == 2)
    and (.selected_unit_ids | index("player") != null)
    and (.selected_unit_ids | index("square_guard_patrol") != null)
    and (.control_group_assignments | index("5:player|square_guard_patrol") != null)
    and (.control_group_slot_summaries | length == 10)
    and ([.control_group_slot_summaries[] | select(.slot == "5") | .member_count][0] == 2)
    and ([.control_group_slot_summaries[] | select(.slot == "5") | .active][0] == true)
    and (.command_queue | index("control_group_assign:5:player|square_guard_patrol") != null)
    and .command_stamp_player_label == "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
  ))
  and (.control_group_hotkey_samples | any(
    .stage == "recall_group_5"
    and .accepted == true
    and .action_label == "RTS:SELECT:recall:5"
    and .group_id == "5"
    and .group_command_state == "group_5_recalled:2units"
    and (.selected_unit_ids | length == 2)
    and (.selected_unit_ids | index("player") != null)
    and (.selected_unit_ids | index("square_guard_patrol") != null)
    and (.command_queue | index("control_group_recall:5:player|square_guard_patrol") != null)
    and .command_stamp_player_label == "HOTKEY GROUP 5 RECALLED 2 UNITS"
  ))
  and (.control_group_hotkey_samples | any(
    .stage == "double_tap_camera_group_5"
    and .accepted == true
    and .action_label == "RTS:SELECT:camera:5"
    and .group_id == "5"
    and .group_command_state == "camera_snap:group_5"
    and .camera_focus_tile_id == "5,4"
    and (.command_queue | index("control_group_camera:5@5,4") != null)
    and .command_stamp_player_label == "HOTKEY GROUP 5 CAMERA SNAP"
  ))
  and (.control_group_hotkey_samples | any(
    .stage == "ctrl_shift_append_group_5"
    and .accepted == true
    and .action_label == "RTS:SELECT:append:5"
    and .group_id == "5"
    and .group_command_state == "group_5_appended:3units:1new"
    and (.selected_unit_ids | length == 3)
    and (.selected_unit_ids | index("player") != null)
    and (.selected_unit_ids | index("square_guard_front") != null)
    and (.selected_unit_ids | index("square_guard_patrol") != null)
    and (.control_group_assignments | index("5:append:player|square_guard_patrol|square_guard_front") != null)
    and ([.control_group_slot_summaries[] | select(.slot == "5") | .member_count][0] == 3)
    and ([.control_group_slot_summaries[] | select(.slot == "10") | .key_label][0] == "0")
    and (.command_queue | index("control_group_append:5:player|square_guard_patrol|square_guard_front") != null)
    and .command_stamp_player_label == "HOTKEY GROUP 5 APPENDED 3 UNITS"
  ))
  and (.control_group_hotkey_samples | any(
    .stage == "shift_recall_add_group_5"
    and .accepted == true
    and .action_label == "RTS:SELECT:recall_add:5"
    and .group_id == "5"
    and .group_command_state == "group_5_recall_added:4units"
    and (.selected_unit_ids | length == 4)
    and (.selected_unit_ids | index("square_worker_carry") != null)
    and (.selected_unit_ids | index("player") != null)
    and (.selected_unit_ids | index("square_guard_front") != null)
    and (.selected_unit_ids | index("square_guard_patrol") != null)
    and ([.control_group_slot_summaries[] | select(.slot == "5") | .member_count][0] == 3)
    and ([.control_group_slot_summaries[] | select(.slot == "5") | .occupied][0] == true)
    and (.command_queue | index("control_group_recall_add:5:square_worker_carry|player|square_guard_patrol|square_guard_front") != null)
    and .command_stamp_player_label == "HOTKEY GROUP 5 ADDED 4 UNITS"
  ))
  and (.control_group_hotkey_samples | all(.command_stamp_player_label | (contains("feedback") or contains("rts_")) | not))
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
  and .viewport_world_input_sample.stage == "camera_focus_viewport_world_input"
  and .viewport_world_input_sample.boot_focus_tile_id == "5,4"
  and .viewport_world_input_sample.boot_tile_id == "4,3"
  and .viewport_world_input_sample.shifted_focus_tile_id == "22,20"
  and .viewport_world_input_sample.shifted_tile_id == "21,19"
  and .viewport_world_input_sample.shifted_action_label == "RTS:MOVE:21,19:line"
  and .viewport_world_input_sample.shifted_hover_player_label == "MAP MOVE READY 21,19"
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
  and (.stage_summaries | any(.stage == "queue_waypoint" and .queue_path_preview_stage == "shift_waypoints" and .queue_path_preview_stage_marker == "command_queue_path_preview:shift_waypoints" and .queue_path_preview_renderer_path == "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay" and .queue_path_preview_input_path == "apply_live_native_action_with_source(classic_rts_live_input)" and .queue_path_preview_waypoint_pixel_count > 80))
  and (.stage_summaries | any(.stage == "hold_position" and .queue_path_preview_stage == "queue_stack" and .queue_path_preview_stage_marker == "command_queue_path_preview:queue_stack" and .queue_path_preview_renderer_path == "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay" and .queue_path_preview_path_pixel_count > 80))
  and (.stage_summaries | any(.stage == "patrol_route" and .queue_path_preview_stage == "rally_chain" and .queue_path_preview_stage_marker == "command_queue_path_preview:rally_chain" and .queue_path_preview_renderer_path == "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay" and .queue_path_preview_waypoint_pixel_count > 80))
  and (.stage_summaries | any(.stage == "attack_move" and .queue_path_preview_stage == "attack_focus" and .queue_path_preview_stage_marker == "command_queue_path_preview:attack_focus" and .queue_path_preview_renderer_path == "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay" and .queue_path_preview_target_pixel_count > 80))
  and (.stage_summaries | any(.stage == "stop_order" and .queue_path_preview_stage == "cancel_repath" and .queue_path_preview_stage_marker == "command_queue_path_preview:cancel_repath" and .queue_path_preview_renderer_path == "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay" and .queue_path_preview_cancel_pixel_count > 80))
  and .live_command_queue_path_preview_slot_pixel_count > 1200
  and .live_command_queue_path_preview_path_pixel_count > 400
  and .live_command_queue_path_preview_waypoint_pixel_count > 200
  and .live_command_queue_path_preview_target_pixel_count > 80
  and .live_command_queue_path_preview_cancel_pixel_count > 80
  and .live_command_queue_path_preview_shift_waypoints_gate == true
  and .live_command_queue_path_preview_queue_stack_gate == true
  and .live_command_queue_path_preview_rally_chain_gate == true
  and .live_command_queue_path_preview_attack_focus_gate == true
  and .live_command_queue_path_preview_cancel_repath_gate == true
  and .live_command_queue_path_preview_gate == true
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
  and .viewport_world_input_gate == true
  and .drag_select_preview_gate == true
  and .drag_select_commit_gate == true
  and .unit_click_select_gate == true
  and .unit_shift_select_gate == true
  and .unit_double_click_select_gate == true
  and .control_group_hotkey_gate == true
  and .command_stamp_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_INPUT_SEQUENCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
