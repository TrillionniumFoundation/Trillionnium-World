#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-command-feedback.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-command-feedback.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-selection-command-feedback "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.feedback_event) | index("selection_command_feedback:marquee_start") != null)
  and (.stage_summaries | map(.feedback_event) | index("selection_command_feedback:selection_confirm") != null)
  and (.stage_summaries | map(.feedback_event) | index("selection_command_feedback:rally_preview") != null)
  and (.stage_summaries | map(.feedback_event) | index("selection_command_feedback:move_line") != null)
  and (.stage_summaries | map(.feedback_event) | index("selection_command_feedback:attack_lock") != null)
  and (.stage_summaries | map(.feedback_event) | index("selection_command_feedback:invalid_order") != null)
  and .marquee_pixel_count > 350
  and .confirm_pixel_count > 260
  and .rally_pixel_count > 280
  and .move_pixel_count > 300
  and .attack_pixel_count > 320
  and .error_pixel_count > 420
  and .ack_pixel_count > 240
  and .marquee_gate == true
  and .confirm_gate == true
  and .rally_gate == true
  and .move_gate == true
  and .attack_gate == true
  and .error_gate == true
  and .ack_gate == true
  and .feedback_stage_gate == true
  and .command_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_COMMAND_FEEDBACK_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
