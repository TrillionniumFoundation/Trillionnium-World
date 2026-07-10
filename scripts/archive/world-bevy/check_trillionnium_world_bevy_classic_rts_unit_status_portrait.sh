#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-status-portrait.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-status-portrait.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-unit-status-portrait "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_unit_status_portrait_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.status_event) | index("unit_status_portrait:worker") != null)
  and (.stage_summaries | map(.status_event) | index("unit_status_portrait:guard") != null)
  and (.stage_summaries | map(.status_event) | index("unit_status_portrait:commander") != null)
  and (.stage_summaries | map(.status_event) | index("unit_status_portrait:creep_target") != null)
  and (.stage_summaries | map(.status_event) | index("unit_status_portrait:structure") != null)
  and (.stage_summaries | map(.status_event) | index("unit_status_portrait:multi_select") != null)
  and .portrait_frame_pixel_count > 1200
  and .health_bar_pixel_count > 300
  and .mana_bar_pixel_count > 240
  and .xp_bar_pixel_count > 200
  and .buff_badge_pixel_count > 160
  and .role_badge_pixel_count > 600
  and .queue_badge_pixel_count > 500
  and .portrait_frame_gate == true
  and .health_bar_gate == true
  and .mana_bar_gate == true
  and .xp_bar_gate == true
  and .buff_badge_gate == true
  and .role_badge_gate == true
  and .queue_badge_gate == true
  and .status_stage_gate == true
  and .status_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_STATUS_PORTRAIT_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
