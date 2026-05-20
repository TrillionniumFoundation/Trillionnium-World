#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-model-depth.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-model-depth.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-unit-model-depth "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_unit_model_depth_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.focus == "guard_armor_rim")] | length) >= 1
  and ([.stage_summaries[] | select(.focus == "worker_tool_pack")] | length) >= 1
  and ([.stage_summaries[] | select(.focus == "creep_horned_silhouette")] | length) >= 1
  and .rim_pixel_count > 220
  and .armor_pixel_count > 140
  and .role_prop_pixel_count > 110
  and .face_shade_pixel_count > 60
  and .ground_contact_pixel_count > 140
  and .layer_shadow_pixel_count > 160
  and .rim_gate == true
  and .armor_gate == true
  and .role_prop_gate == true
  and .face_shade_gate == true
  and .ground_contact_gate == true
  and .layer_shadow_gate == true
  and .scene_renderer_gate == true
  and .role_coverage_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_MODEL_DEPTH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
