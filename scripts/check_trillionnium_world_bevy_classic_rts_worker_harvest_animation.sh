#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-worker-harvest-animation.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-worker-harvest-animation.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-worker-harvest-animation "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_worker_harvest_animation_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.harvest_event) | index("harvest_anim:approach") != null)
  and (.stage_summaries | map(.harvest_event) | index("harvest_anim:tool_swing") != null)
  and (.stage_summaries | map(.harvest_event) | index("harvest_anim:resource_pop") != null)
  and (.stage_summaries | map(.harvest_event) | index("harvest_anim:carry_load") != null)
  and (.stage_summaries | map(.harvest_event) | index("harvest_anim:dropoff_burst") != null)
  and (.stage_summaries | map(.harvest_event) | index("harvest_anim:return_path") != null)
  and .approach_pixel_count > 120
  and .tool_swing_pixel_count > 100
  and .resource_pop_pixel_count > 120
  and .carry_load_pixel_count > 120
  and .dropoff_burst_pixel_count > 120
  and .return_path_pixel_count > 120
  and .approach_gate == true
  and .tool_swing_gate == true
  and .resource_pop_gate == true
  and .carry_load_gate == true
  and .dropoff_burst_gate == true
  and .return_path_gate == true
  and .harvest_stage_gate == true
  and .economy_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_WORKER_HARVEST_ANIMATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
