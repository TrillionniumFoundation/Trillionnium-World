#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scrollable-map.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scrollable-map.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-scrollable-map "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_scrollable_map_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_scrollable_map_overlay"
  and .input_path == "apply_rts_scrollable_map_camera_input(classic_rts_scrollable_map_input)"
  and .input_handler == "update_native_rts_scrollable_map_camera"
  and .projection_path == "apply_native_rts_scrollable_map_view"
  and .surface_role_filter == "is_scrollable_map_surface_role"
  and .native_runtime_path == "update_native_rts_scrollable_map_camera+apply_native_rts_scrollable_map_view"
  and .input_action_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("keyboard_pan") != null)
  and (.stage_summaries | map(.stage) | index("edge_scroll") != null)
  and (.stage_summaries | map(.stage) | index("middle_mouse_drag") != null)
  and (.stage_summaries | map(.stage) | index("wheel_zoom") != null)
  and (.stage_summaries | map(.stage) | index("minimap_jump") != null)
  and (.stage_summaries | map(.stage) | index("bounds_clamp") != null)
  and (.stage_summaries | map(select(.stage == "minimap_jump"))[0].minimap_tile_id == "minimap_cursor_jump")
  and (.stage_summaries | map(select(.stage == "bounds_clamp"))[0].clamped == true)
  and .camera_frame_pixel_count > 4000
  and .edge_pixel_count > 1000
  and .drag_pixel_count > 250
  and .zoom_pixel_count > 900
  and .minimap_pixel_count > 600
  and .clamp_pixel_count > 1000
  and .frame_gate == true
  and .edge_gate == true
  and .drag_gate == true
  and .zoom_gate == true
  and .minimap_gate == true
  and .clamp_gate == true
  and .stage_gate == true
  and .keyboard_pan_gate == true
  and .edge_scroll_gate == true
  and .drag_pan_gate == true
  and .wheel_zoom_gate == true
  and .minimap_jump_gate == true
  and .boundary_clamp_gate == true
  and .map_layer_projection_gate == true
  and .hud_fixed_gate == true
  and .camera_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SCROLLABLE_MAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
