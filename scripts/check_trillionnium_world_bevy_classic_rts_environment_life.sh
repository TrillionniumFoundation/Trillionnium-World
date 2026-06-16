#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-environment-life.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-environment-life.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-environment-life "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_environment_life_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.environment_event) | index("environment:tree_sway") != null)
  and (.stage_summaries | map(.environment_event) | index("environment:torch_flicker") != null)
  and (.stage_summaries | map(.environment_event) | index("environment:water_shimmer") != null)
  and (.stage_summaries | map(.environment_event) | index("environment:banner_flutter") != null)
  and (.stage_summaries | map(.environment_event) | index("environment:resource_glint") != null)
  and (.stage_summaries | map(.environment_event) | index("environment:ambient_dust") != null)
  and .tree_sway_pixel_count > 160
  and .torch_flicker_pixel_count > 120
  and .water_shimmer_pixel_count > 120
  and .banner_flutter_pixel_count > 160
  and .resource_glint_pixel_count > 120
  and .ambient_dust_pixel_count > 120
  and .tree_sway_gate == true
  and .torch_flicker_gate == true
  and .water_shimmer_gate == true
  and .banner_flutter_gate == true
  and .resource_glint_gate == true
  and .ambient_dust_gate == true
  and .environment_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENVIRONMENT_LIFE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
