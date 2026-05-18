#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    cargo run -p trnm-world-bevy -- classic-scene-preview "$PREVIEW" >"$SUMMARY"
)

test -s "$SUMMARY"
test -s "$PREVIEW"
head -n 1 "$PREVIEW" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_scene_preview_v1"
  and .green == true
  and .preview_format == "ppm_p3_rgb"
  and .preview_width == 1280
  and .preview_height == 720
  and .preview_bytes > 100000
  and .unique_color_count >= 24
  and .non_background_pixels > 80000
  and .overlay_text_pixel_count > 2000
  and .preview_nonblank_gate == true
  and .overlay_text_gate == true
  and .direction_frame_gate == true
  and .renderer_manifest_gate == true
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .frame_count >= 43
  and .panel_count == 4
  and ([.panel_summaries[].player_frame_id] | unique | length) == 4
  and ([.panel_summaries[].scene_id] | index("mirror_city_square") != null)
  and ([.panel_summaries[].scene_id] | index("mentor_training_room") != null)
  and ([.panel_summaries[].scene_id] | index("league_coliseum") != null)
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_SCENE_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
