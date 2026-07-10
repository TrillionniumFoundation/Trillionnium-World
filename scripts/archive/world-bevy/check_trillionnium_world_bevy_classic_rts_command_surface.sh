#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-surface.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-surface.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-command-surface "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_command_surface_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 4
  and (.stage_summaries | map(.surface_event) | index("surface:selection_state") != null)
  and (.stage_summaries | map(.surface_event) | index("surface:command_grid") != null)
  and (.stage_summaries | map(.surface_event) | index("surface:cooldown_disabled") != null)
  and (.stage_summaries | map(.surface_event) | index("surface:target_queue") != null)
  and .selection_frame_pixel_count > 800
  and .ready_pixel_count > 500
  and .disabled_pixel_count > 100
  and .cooldown_pixel_count > 300
  and .target_panel_pixel_count > 400
  and .queue_confirm_pixel_count > 250
  and .group_tab_pixel_count > 200
  and .selection_surface_gate == true
  and .command_grid_surface_gate == true
  and .cooldown_disabled_surface_gate == true
  and .target_queue_surface_gate == true
  and .surface_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_SURFACE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
