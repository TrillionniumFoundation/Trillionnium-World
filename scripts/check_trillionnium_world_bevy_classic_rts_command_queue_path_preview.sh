#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-queue-path-preview.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-queue-path-preview.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-command-queue-path-preview "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay"
  and .input_path == "apply_live_native_action_with_source(classic_rts_command_queue_path_preview_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("queue_stack") != null)
  and (.stage_summaries | map(.stage) | index("shift_waypoints") != null)
  and (.stage_summaries | map(.stage) | index("rally_chain") != null)
  and (.stage_summaries | map(.stage) | index("attack_focus") != null)
  and (.stage_summaries | map(.stage) | index("build_reservation") != null)
  and (.stage_summaries | map(.stage) | index("cancel_repath") != null)
  and (.stage_summaries | map(select(.stage == "rally_chain"))[0].minimap_command_kind == "rally")
  and (.stage_summaries | map(select(.stage == "attack_focus"))[0].attack_target_id == "arena_creep_attack")
  and (.stage_summaries | map(select(.stage == "build_reservation"))[0].building_blueprint_id == "watch_tower")
  and (.stage_summaries | map(select(.stage == "cancel_repath"))[0].cancelled_structure_ids | index("watch_tower") != null)
  and .queue_slot_pixel_count > 1200
  and .path_pixel_count > 400
  and .waypoint_pixel_count > 400
  and .target_pixel_count > 300
  and .reservation_pixel_count > 250
  and .cancel_pixel_count > 250
  and .live_input_gate == true
  and .queue_slot_visual_gate == true
  and .path_visual_gate == true
  and .waypoint_visual_gate == true
  and .target_visual_gate == true
  and .reservation_visual_gate == true
  and .cancel_visual_gate == true
  and .stage_gate == true
  and .queue_stack_gate == true
  and .shift_waypoint_gate == true
  and .rally_chain_gate == true
  and .attack_focus_gate == true
  and .build_reservation_gate == true
  and .cancel_repath_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_QUEUE_PATH_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
