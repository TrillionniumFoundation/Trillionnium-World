#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-override-probe.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-override-probe.ppm"
OVERRIDE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/classic-asset-overrides"
OVERRIDE_FRAME="$OVERRIDE_DIR/actor_player_idle_south.ppm"
mkdir -p "$(dirname "$SUMMARY")" "$OVERRIDE_DIR"

{
  printf 'P3\n16 16\n255\n'
  for y in $(seq 0 15); do
    for x in $(seq 0 15); do
      if [ "$x" -ge 4 ] && [ "$x" -le 11 ] && [ "$y" -ge 2 ] && [ "$y" -le 14 ]; then
        printf '255 0 255\n'
      else
        printf '0 0 0\n'
      fi
    done
  done
} >"$OVERRIDE_FRAME"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR="$OVERRIDE_DIR" \
    "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-asset-override-probe "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_asset_override_probe_v1"
  and .green == true
  and .override_frame_id == "actor_player_idle_south"
  and .override_frame_count >= 1
  and (.override_frame_ids | index("actor_player_idle_south") != null)
  and .override_frame_gate == true
  and .override_probe_color == "ff00ff"
  and .override_probe_pixel_count > 300
  and .non_background_pixels > 300
  and .preview_width == 96
  and .preview_height == 96
  and .draw_gate == true
  and .write_gate == true
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .replacement_boundary_gate == true
  and .x230_low_spec_renderer_target == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
  and (.asset_boundary | contains("not_cex_runtime"))
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_PROBE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
