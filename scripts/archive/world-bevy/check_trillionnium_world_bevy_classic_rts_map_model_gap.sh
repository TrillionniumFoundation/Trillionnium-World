#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-model-gap.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-model-gap.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-map-model-gap "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_map_model_gap_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.map_model_event) | index("map_model_gap:lane_topology") != null)
  and (.stage_summaries | map(.map_model_event) | index("map_model_gap:resource_expansion") != null)
  and (.stage_summaries | map(.map_model_event) | index("map_model_gap:height_choke") != null)
  and (.stage_summaries | map(.map_model_event) | index("map_model_gap:structure_silhouette") != null)
  and (.stage_summaries | map(.map_model_event) | index("map_model_gap:unit_role_readability") != null)
  and (.stage_summaries | map(.map_model_event) | index("map_model_gap:fog_depth_cutaway") != null)
  and ([.stage_summaries[] | select(.lane_count >= 3)] | length) == 6
  and ([.stage_summaries[] | select(.resource_cluster_count >= 3)] | length) == 6
  and ([.stage_summaries[] | select(.height_zone_count >= 3)] | length) == 6
  and ([.stage_summaries[] | select(.choke_count >= 3)] | length) == 6
  and ([.stage_summaries[] | select(.structure_silhouette_count >= 3)] | length) == 6
  and ([.stage_summaries[] | select(.unit_role_marker_count >= 4)] | length) == 6
  and ([.stage_summaries[] | select(.occlusion_layer_count >= 3)] | length) == 6
  and .lane_pixel_count > 4000
  and .resource_pixel_count > 1000
  and .height_pixel_count > 3000
  and .choke_pixel_count > 1000
  and .structure_pixel_count > 3000
  and .unit_role_pixel_count > 1000
  and .occlusion_pixel_count > 2000
  and .lane_gate == true
  and .resource_gate == true
  and .height_gate == true
  and .choke_gate == true
  and .structure_silhouette_gate == true
  and .unit_role_gate == true
  and .occlusion_gate == true
  and .map_model_stage_gate == true
  and .map_topology_gate == true
  and .model_readability_gate == true
  and .scene_renderer_gate == true
  and .openra_gap_not_closed_gate == true
  and .openra_parity_target_commit == "5f1bf76"
  and .bevy_openra_parity_state == "map_model_catching_up_not_claimed"
  and .bevy_openra_parity_claimed == false
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MAP_MODEL_GAP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
