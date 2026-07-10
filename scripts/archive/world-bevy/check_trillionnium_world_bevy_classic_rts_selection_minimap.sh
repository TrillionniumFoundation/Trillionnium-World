#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.ppm"
RAW_SUMMARY="$SUMMARY.raw.$$"
TMP_SUMMARY="$SUMMARY.tmp.$$"
mkdir -p "$(dirname "$SUMMARY")"
cleanup() {
  rm -f "$RAW_SUMMARY" "$TMP_SUMMARY"
}
trap cleanup EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-selection-minimap "$PREVIEW" >"$RAW_SUMMARY"

jq '
  .action_label_count = ((.action_labels // []) | length)
  | .input_source_count = ((.input_sources // []) | length)
  | .stage_summary_count = ((.stage_summaries // []) | length)
  | .stage_name_count = ([.stage_summaries[]?.stage] | length)
  | .final_selected_unit_count = ((.final_selected_unit_ids // []) | length)
  | .final_selection_box_tile_count = ((.final_selection_box_tile_ids // []) | length)
  | .final_control_group_assignment_count = ((.final_control_group_assignments // []) | length)
  | .final_active_control_group_count = ((.final_active_control_group_ids // []) | length)
  | .final_group_route_tile_count = ((.final_group_route_tile_ids // []) | length)
  | .final_command_queue_count = ((.final_command_queue // []) | length)
  | .rts_selection_minimap_core_frame_order_count = ((.rts_selection_minimap_core_frame_orders // []) | length)
  | .rts_selection_minimap_core_frame_order_kind_label_count = ((.rts_selection_minimap_core_frame_order_kind_labels // []) | length)
  | .rts_selection_minimap_core_frame_order_error_count = ((.rts_selection_minimap_core_frame_order_errors // []) | length)
  | .selection_minimap_gate_count = ([.write_gate, .live_selection_minimap_input_gate, .selection_box_gate, .control_group_gate, .minimap_command_gate, .split_route_gate, .rts_selection_minimap_core_frame_order_gate, .rts_selection_minimap_core_headless_replay_gate] | length)
  | .selection_minimap_passed_gate_count = ([.write_gate, .live_selection_minimap_input_gate, .selection_box_gate, .control_group_gate, .minimap_command_gate, .split_route_gate, .rts_selection_minimap_core_frame_order_gate, .rts_selection_minimap_core_headless_replay_gate] | map(select(. == true)) | length)
  | .selection_minimap_failed_gate_count = ([.write_gate, .live_selection_minimap_input_gate, .selection_box_gate, .control_group_gate, .minimap_command_gate, .split_route_gate, .rts_selection_minimap_core_frame_order_gate, .rts_selection_minimap_core_headless_replay_gate] | map(select(. != true)) | length)
' "$RAW_SUMMARY" >"$TMP_SUMMARY"
mv "$TMP_SUMMARY" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_selection_minimap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_selection_minimap_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and .action_label_count == (.action_labels | length)
  and .input_source_count == (.input_sources | length)
  and .stage_summary_count == (.stage_summaries | length)
  and .stage_name_count == ([.stage_summaries[].stage] | length)
  and .final_selected_unit_count == (.final_selected_unit_ids | length)
  and .final_selection_box_tile_count == (.final_selection_box_tile_ids | length)
  and .final_control_group_assignment_count == (.final_control_group_assignments | length)
  and .final_active_control_group_count == (.final_active_control_group_ids | length)
  and .final_group_route_tile_count == (.final_group_route_tile_ids | length)
  and .final_command_queue_count == (.final_command_queue | length)
  and .rts_selection_minimap_core_frame_order_count == (.rts_selection_minimap_core_frame_orders | length)
  and .rts_selection_minimap_core_frame_order_kind_label_count == (.rts_selection_minimap_core_frame_order_kind_labels | length)
  and .rts_selection_minimap_core_frame_order_error_count == (.rts_selection_minimap_core_frame_order_errors | length)
  and (.action_labels | index("RTS:SELECT:box:frontline") != null)
  and (.action_labels | index("RTS:MOVE:minimap:9,2:rally") != null)
  and (.action_labels | index("RTS:SELECT:2") != null)
  and (.action_labels | index("RTS:MOVE:6,5:split") != null)
  and .final_control_group_id == "2"
  and (.final_selected_unit_ids | index("square_guard_patrol") != null)
  and (.final_selected_unit_ids | index("square_creep_wander") != null)
  and (.final_selection_box_tile_ids | index("5,5") != null)
  and (.final_selection_box_tile_ids | index("6,4") != null)
  and (.final_control_group_assignments | index("1:player|square_guard_patrol|square_worker_carry|square_creep_wander") != null)
  and (.final_control_group_assignments | index("2:square_guard_patrol|square_creep_wander") != null)
  and (.final_active_control_group_ids | index("1") != null)
  and (.final_active_control_group_ids | index("2") != null)
  and (.stage_summaries | any(.stage == "minimap_rally"
      and .minimap_command_tile_id == "9,2"
      and .minimap_command_kind == "rally"
      and (.group_route_tile_ids | index("9,2") != null)))
  and .final_minimap_command_tile_id == "6,5"
  and .final_minimap_command_kind == "split"
  and (.final_group_route_tile_ids | length >= 4)
  and (.final_group_route_tile_ids | index("6,4") != null)
  and .final_group_command_state == "split_route:group_2"
  and (.final_command_queue | index("minimap:rally:9,2") != null)
  and (.final_command_queue | any(startswith("box_select:")))
  and (.final_command_queue | any(startswith("split_route:")))
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_selection_minimap_core_frame_orders | length == 2)
  and .rts_selection_minimap_core_frame_order_kind_labels == ["move","move"]
  and (.rts_selection_minimap_core_frame_order_errors | length == 0)
  and .rts_selection_minimap_core_frame_order_stream_error == null
  and (.rts_selection_minimap_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.rts_selection_minimap_core_frame_orders | any(.target_tile.x == 9 and .target_tile.y == 2 and .formation_id == "minimap:rally"))
  and (.rts_selection_minimap_core_frame_orders | any(.target_tile.x == 6 and .target_tile.y == 5 and .formation_id == "split"))
  and .rts_selection_minimap_core_headless_replay_error == null
  and (.rts_selection_minimap_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_selection_minimap_core_headless_applied_order_count == 2
  and .rts_selection_minimap_core_headless_actor_count == 4
  and .rts_selection_minimap_core_headless_final_frame == 713
  and .non_background_pixels > 220000
  and .selection_box_pixel_count > 160
  and .minimap_command_pixel_count > 80
  and .group_two_pixel_count > 20
  and .split_route_pixel_count > 120
  and .live_selection_minimap_input_gate == true
  and .selection_box_gate == true
  and .control_group_gate == true
  and .minimap_command_gate == true
  and .split_route_gate == true
  and .rts_selection_minimap_core_frame_order_gate == true
  and .rts_selection_minimap_core_headless_replay_gate == true
  and .selection_minimap_gate_count == 8
  and .selection_minimap_passed_gate_count == 8
  and .selection_minimap_failed_gate_count == 0
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_MINIMAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
