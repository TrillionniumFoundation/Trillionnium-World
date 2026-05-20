#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-locomotion-blend.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-locomotion-blend.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-locomotion-blend "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_locomotion_blend_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.locomotion_event == "locomotion:path_commit")] | length) == 1
  and ([.stage_summaries[] | select(.locomotion_event == "locomotion:footstep_left")] | length) == 1
  and ([.stage_summaries[] | select(.locomotion_event == "locomotion:footstep_right")] | length) == 1
  and ([.stage_summaries[] | select(.locomotion_event == "locomotion:turn_arc")] | length) == 1
  and ([.stage_summaries[] | select(.locomotion_event == "locomotion:formation_slide")] | length) == 1
  and ([.stage_summaries[] | select(.locomotion_event == "locomotion:arrival_brake")] | length) == 1
  and .path_pixel_count > 120
  and .left_step_pixel_count > 80
  and .right_step_pixel_count > 80
  and .turn_pixel_count > 100
  and .slide_pixel_count > 100
  and .brake_pixel_count > 100
  and .path_gate == true
  and .left_step_gate == true
  and .right_step_gate == true
  and .turn_gate == true
  and .slide_gate == true
  and .brake_gate == true
  and .locomotion_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LOCOMOTION_BLEND_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
