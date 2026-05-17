#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-authored-live-visual-bridge.json"
RENDER_SUMMARY="$EVIDENCE_DIR/bevy-authored-render-frame.json"
LIVE_SUMMARY="$EVIDENCE_DIR/bevy-live-window-screenshot-sequence.json"
mkdir -p "$EVIDENCE_DIR"

"$ROOT/scripts/check_trillionnium_world_bevy_authored_render_frame.sh" >/dev/null

test -s "$RENDER_SUMMARY"
test -s "$LIVE_SUMMARY"

FINAL_FRAME="$(jq -r '.final_frame_path' "$LIVE_SUMMARY")"
CONTACT_SHEET="$(jq -r '.contact_sheet_path' "$LIVE_SUMMARY")"
test -s "$FINAL_FRAME"
test -s "$CONTACT_SHEET"

jq -n \
  --slurpfile render "$RENDER_SUMMARY" \
  --slurpfile live "$LIVE_SUMMARY" \
  --arg contract_version "trillionnium_world_bevy_authored_live_visual_bridge_v1" \
  --arg render_summary "$RENDER_SUMMARY" \
  --arg live_summary "$LIVE_SUMMARY" \
  --arg final_frame "$FINAL_FRAME" \
  --arg contact_sheet "$CONTACT_SHEET" \
  '($render[0]) as $r
  | ($live[0]) as $l
  | ($r.layer_samples // []) as $layers
  | ($l.frames // []) as $frames
  | {
      contract_version: $contract_version,
      green: (
        ($r.green == true)
        and ($l.green == true)
        and ($r.render_frame_layer_gate == true)
        and ($r.frame_nonblank_gate == true)
        and ($l.screenshot_nonblank_gate == true)
        and ($l.frame_change_gate == true)
        and ($l.contact_sheet_gate == true)
        and ($l.final_frame_gate == true)
        and (($r.frame_unique_color_count // 0) >= 8)
        and (($frames | length) >= 11)
        and (($frames | map(select(.nonblank == true)) | length) == ($frames | length))
        and (($frames[-1].colors_96x54 // 0) >= 1000)
        and ((["map","hud","actor","feedback"] - ($layers | map(select((.drawn_pixel_count // 0) > 0) | .scene_layer) | unique)) | length == 0)
        and ($r.gpu_upload_claimed == false)
        and ($r.android_s5_real_device_claimed == false)
        and ($l.android_s5_real_device_claimed == false)
      ),
      authored_render_frame_contract: "trillionnium_world_bevy_authored_render_frame_v1",
      live_window_screenshot_contract: "trillionnium_world_bevy_live_window_screenshot_sequence_v1",
      render_frame_summary_path: $render_summary,
      live_window_summary_path: $live_summary,
      final_frame_path: $final_frame,
      contact_sheet_path: $contact_sheet,
      render_frame_green: ($r.green == true),
      live_window_green: ($l.green == true),
      render_frame_bytes: ($r.frame_bytes // 0),
      render_frame_unique_color_count: ($r.frame_unique_color_count // 0),
      live_frame_count: ($frames | length),
      live_final_frame_colors_96x54: ($frames[-1].colors_96x54 // 0),
      live_contact_sheet_colors: ($l.contact_sheet_colors // 0),
      layer_samples: $layers,
      gates: {
        authored_render_frame_gate: ($r.green == true and $r.render_frame_layer_gate == true and $r.frame_nonblank_gate == true),
        live_window_sequence_gate: ($l.green == true and $l.screenshot_nonblank_gate == true and $l.frame_change_gate == true and $l.contact_sheet_gate == true and $l.final_frame_gate == true),
        live_final_frame_nonblank_gate: (($frames | length) >= 11 and (($frames | map(select(.nonblank == true)) | length) == ($frames | length)) and (($frames[-1].colors_96x54 // 0) >= 1000)),
        four_layer_visual_bridge_gate: ((["map","hud","actor","feedback"] - ($layers | map(select((.drawn_pixel_count // 0) > 0) | .scene_layer) | unique)) | length == 0),
        boundary_gate: ($r.gpu_upload_claimed == false and $r.android_s5_real_device_claimed == false and $l.android_s5_real_device_claimed == false)
      },
      asset_boundary: "correlates_host_side_ppm_render_frame_with_live_window_screenshot_sequence_not_gpu_texture_claim",
      source_of_truth: "This bridge requires the authored host-side render-frame artifact and the live-window screenshot sequence artifact to be green at the same acceptance revision, with map/HUD/actor/feedback layer pixels in the authored frame and nonblank changing pixels in the live Bevy window sequence.",
      gpu_upload_claimed: false,
      android_s5_real_device_claimed: false,
      live_osm_ingestion_claimed: false
    }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_authored_live_visual_bridge_v1"
  and .green == true
  and .authored_render_frame_contract == "trillionnium_world_bevy_authored_render_frame_v1"
  and .live_window_screenshot_contract == "trillionnium_world_bevy_live_window_screenshot_sequence_v1"
  and .render_frame_green == true
  and .live_window_green == true
  and .render_frame_bytes > 100000
  and .render_frame_unique_color_count >= 8
  and .live_frame_count >= 11
  and .live_final_frame_colors_96x54 >= 1000
  and .live_contact_sheet_colors > 32
  and .gates.authored_render_frame_gate == true
  and .gates.live_window_sequence_gate == true
  and .gates.live_final_frame_nonblank_gate == true
  and .gates.four_layer_visual_bridge_gate == true
  and .gates.boundary_gate == true
  and .asset_boundary == "correlates_host_side_ppm_render_frame_with_live_window_screenshot_sequence_not_gpu_texture_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_AUTHORED_LIVE_VISUAL_BRIDGE_GREEN $SUMMARY final_frame=$FINAL_FRAME contact_sheet=$CONTACT_SHEET"
