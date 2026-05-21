#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-camera-minimap-sync "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_camera_minimap_sync_overlay"
  and .input_path == "apply_rts_scrollable_map_camera_input(classic_rts_camera_minimap_sync_input)"
  and .runtime_path == "apply_rts_scrollable_map_camera_input+rts_camera_minimap_viewport_rect+rts_camera_minimap_revealed_tiles"
  and .selection_follow_path == "rts_camera_minimap_selection_follow_step"
  and .native_runtime_path == "update_native_rts_scrollable_map_camera+apply_native_rts_scrollable_map_view+rts_camera_minimap_viewport_rect"
  and .input_action_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("viewport_rect") != null)
  and (.stage_summaries | map(.stage) | index("fog_reveal") != null)
  and (.stage_summaries | map(.stage) | index("selection_follow") != null)
  and (.stage_summaries | map(.stage) | index("control_group_recall") != null)
  and (.stage_summaries | map(.stage) | index("route_projection") != null)
  and (.stage_summaries | map(.stage) | index("zoom_sync") != null)
  and (.stage_summaries | map(select(.stage == "selection_follow"))[0].selected_unit_id == "mirror_captain")
  and (.stage_summaries | map(select(.stage == "control_group_recall"))[0].control_group_id == "2")
  and (.stage_summaries | map(select(.stage == "route_projection"))[0].minimap_tile_id == "minimap_route_target")
  and .revealed_tile_union_count >= 12
  and .viewport_pixel_count > 2400
  and .fog_pixel_count > 8000
  and .reveal_pixel_count > 800
  and .selection_pixel_count > 1000
  and .route_pixel_count > 900
  and .viewport_visual_gate == true
  and .fog_visual_gate == true
  and .reveal_visual_gate == true
  and .selection_visual_gate == true
  and .route_visual_gate == true
  and .stage_gate == true
  and .viewport_sync_gate == true
  and .fog_reveal_gate == true
  and .selection_follow_gate == true
  and .control_group_sync_gate == true
  and .route_projection_gate == true
  and .zoom_rect_sync_gate == true
  and .minimap_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMERA_MINIMAP_SYNC_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
