#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-selection-minimap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_selection_minimap_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_selection_minimap_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
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
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_MINIMAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
