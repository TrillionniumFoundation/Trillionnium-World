#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-sequence.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-sequence.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-action-sequence "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_action_sequence_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.phase_event == "sequence:idle")] | length) == 1
  and ([.stage_summaries[] | select(.phase_event == "sequence:windup")] | length) == 1
  and ([.stage_summaries[] | select(.phase_event == "sequence:strike")] | length) == 1
  and ([.stage_summaries[] | select(.phase_event == "sequence:recovery")] | length) == 1
  and ([.stage_summaries[] | select(.phase_event == "sequence:carry_up")] | length) == 1
  and ([.stage_summaries[] | select(.phase_event == "sequence:carry_down")] | length) == 1
  and .idle_pixel_count > 80
  and .windup_pixel_count > 120
  and .strike_pixel_count > 180
  and .recovery_pixel_count > 120
  and .carry_up_pixel_count > 50
  and .carry_down_pixel_count > 50
  and .frame_ghost_pixel_count > 120
  and .idle_gate == true
  and .windup_gate == true
  and .strike_gate == true
  and .recovery_gate == true
  and .carry_up_gate == true
  and .carry_down_gate == true
  and .frame_ghost_gate == true
  and .sequence_phase_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ACTION_SEQUENCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
