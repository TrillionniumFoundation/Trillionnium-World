#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-control-loop "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_loop_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 360
  and .write_gate == true
  and .mirror_scene_gate == true
  and .coliseum_scene_gate == true
  and .non_background_pixels > 120000
  and .control_group_id == "1"
  and .move_selected_unit_count >= 4
  and .attack_selected_unit_count >= 4
  and (.move_command_queue | index("select_group_1") != null)
  and (.move_command_queue | index("move:7,4") != null)
  and (.move_command_queue | index("formation:diamond") != null)
  and (.attack_command_queue | index("select_group_1") != null)
  and (.attack_command_queue | index("attack:arena_creep_attack") != null)
  and .attack_target_id == "arena_creep_attack"
  and .selection_marker_pixel_count > 500
  and .formation_line_pixel_count > 200
  and .command_marker_pixel_count > 600
  and .attack_feedback_pixel_count > 180
  and .strategy_panel_pixel_count > 4000
  and .minimap_pixel_count > 2800
  and .resource_hud_pixel_count > 120
  and .selection_gate == true
  and .command_queue_gate == true
  and .strategy_hud_gate == true
  and .gameplay_surface_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_LOOP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
