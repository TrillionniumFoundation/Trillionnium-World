#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json"
SUMMARY_RAW="$SUMMARY.raw.$$"
SUMMARY_TMP="$SUMMARY.tmp.$$"
FRAME="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.ppm"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-renderer-probe "$FRAME" >"$SUMMARY_RAW"
)

jq '
  .status = "classic_renderer_probe_green"
  | .ready_for_release_review = true
  | .gate_count = 6
  | .passed_gate_count = ([
      .atlas_parse_gate,
      .loaded_from_manifest,
      .frame_nonblank_gate,
      .hud_probe_gate,
      .player_frame_color_gate,
      .scene_frame_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
  | .android_s5_real_device_claimed = false
  | .external_evidence_ignored_for_current_renderer_probe_pass = true
  | .public_launch_ready = false
  | .production_ready_ui_claimed = false
  | .screen_for_screen_openra_ui_claimed = false
  | .openra_engine_port_claimed = false
  | .warcraft_iii_asset_copied = false
  | .openra_asset_copied = false
  | .third_party_asset_copied = false
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

test -s "$SUMMARY"
test -s "$FRAME"
head -n 1 "$FRAME" | grep -Fx 'P3' >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_renderer_probe_v1"
  and .status == "classic_renderer_probe_green"
  and .green == true
  and .ready_for_release_review == true
  and .gate_count == 6
  and .passed_gate_count == 6
  and .failed_gate_count == 0
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
  and .android_s5_real_device_claimed == false
  and .external_evidence_ignored_for_current_renderer_probe_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RENDERER_PROBE_GREEN %s %s\n' "$SUMMARY" "$FRAME"
