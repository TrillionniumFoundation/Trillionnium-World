#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-preview.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-preview.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-formation-move-preview "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_formation_move_preview_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_formation_move_preview_overlay"
  and .input_path == "apply_live_native_action_with_source(classic_rts_formation_move_preview_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("destination_ghost") != null)
  and (.stage_summaries | map(.stage) | index("wedge_spacing") != null)
  and (.stage_summaries | map(.stage) | index("line_reflow") != null)
  and (.stage_summaries | map(.stage) | index("collision_avoidance") != null)
  and (.stage_summaries | map(.stage) | index("split_avoidance") != null)
  and (.stage_summaries | map(.stage) | index("commit_spacing") != null)
  and (.stage_summaries | map(select(.stage == "destination_ghost"))[0].command_destination_tile == "8,4")
  and (.stage_summaries | map(select(.stage == "collision_avoidance"))[0].blocked_tile_ids | index("7,4") != null)
  and (.stage_summaries | map(select(.stage == "split_avoidance"))[0].group_command_state == "split_route:group_2")
  and (.stage_summaries | map(select(.stage == "commit_spacing"))[0].command_destination_tile == "9,2")
  and .ghost_pixel_count > 1200
  and .path_pixel_count > 500
  and .slot_pixel_count > 250
  and .collision_pixel_count > 250
  and .disperse_pixel_count > 120
  and .commit_pixel_count > 160
  and .live_input_gate == true
  and .ghost_visual_gate == true
  and .path_visual_gate == true
  and .slot_visual_gate == true
  and .collision_visual_gate == true
  and .disperse_visual_gate == true
  and .commit_visual_gate == true
  and .stage_gate == true
  and .destination_ghost_gate == true
  and .wedge_spacing_gate == true
  and .line_reflow_gate == true
  and .collision_avoidance_gate == true
  and .split_avoidance_gate == true
  and .commit_spacing_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FORMATION_MOVE_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
