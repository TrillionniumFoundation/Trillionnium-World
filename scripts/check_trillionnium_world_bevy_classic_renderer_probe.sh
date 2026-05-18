#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json"
FRAME="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    cargo run -p trnm-world-bevy -- classic-renderer-probe "$FRAME" >"$SUMMARY"
)

test -s "$SUMMARY"
test -s "$FRAME"
head -n 1 "$FRAME" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_renderer_probe_v1"
  and .green == true
  and .frame_format == "ppm_p3_rgb"
  and .frame_width == 640
  and .frame_height == 360
  and .frame_bytes > 100000
  and .unique_color_count >= 32
  and .non_background_pixels > 80000
  and .hud_text_pixels > 4000
  and .hud_panel_pixels > 4000
  and .player_frame_id == "actor_player_walk_east_1"
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .frame_nonblank_gate == true
  and .hud_probe_gate == true
  and .player_frame_color_gate == true
  and .scene_frame_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RENDERER_PROBE_GREEN %s %s\n' "$SUMMARY" "$FRAME"
