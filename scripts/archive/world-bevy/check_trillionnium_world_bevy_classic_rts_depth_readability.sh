#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-depth-readability.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-depth-readability.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-depth-readability "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_depth_readability_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.depth_event == "depth:foreground_canopy")] | length) == 1
  and ([.stage_summaries[] | select(.depth_event == "depth:behind_silhouette")] | length) == 1
  and ([.stage_summaries[] | select(.depth_event == "depth:building_mask")] | length) == 1
  and ([.stage_summaries[] | select(.depth_event == "depth:target_priority")] | length) == 1
  and ([.stage_summaries[] | select(.depth_event == "depth:path_occlusion")] | length) == 1
  and ([.stage_summaries[] | select(.depth_event == "depth:terrain_cutaway")] | length) == 1
  and .foreground_pixel_count > 120
  and .behind_pixel_count > 120
  and .building_mask_pixel_count > 140
  and .target_priority_pixel_count > 130
  and .path_occlusion_pixel_count > 120
  and .cutaway_pixel_count > 120
  and .foreground_gate == true
  and .behind_gate == true
  and .building_mask_gate == true
  and .target_priority_gate == true
  and .path_occlusion_gate == true
  and .cutaway_gate == true
  and .depth_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_DEPTH_READABILITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
