#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-hotkey-feedback.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-hotkey-feedback.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-control-group-hotkey-feedback "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(classic_rts_control_group_hotkey_feedback_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.hotkey_event) | index("control_group_hotkey_feedback:assign_group") != null)
  and (.stage_summaries | map(.hotkey_event) | index("control_group_hotkey_feedback:recall_group") != null)
  and (.stage_summaries | map(.hotkey_event) | index("control_group_hotkey_feedback:double_tap_camera") != null)
  and (.stage_summaries | map(.hotkey_event) | index("control_group_hotkey_feedback:idle_worker_ping") != null)
  and (.stage_summaries | map(.hotkey_event) | index("control_group_hotkey_feedback:production_hotkey") != null)
  and (.stage_summaries | map(.hotkey_event) | index("control_group_hotkey_feedback:ability_hotkey_ack") != null)
  and (.final_active_control_group_ids | length) >= 4
  and (.final_production_queue | length) >= 4
  and .final_active_ability_id == "guard_break"
  and .assign_pixel_count > 1000
  and .recall_pixel_count > 450
  and .camera_pixel_count > 900
  and .idle_pixel_count > 900
  and .production_pixel_count > 700
  and .ability_pixel_count > 700
  and .assign_gate == true
  and .recall_gate == true
  and .camera_gate == true
  and .idle_gate == true
  and .production_gate == true
  and .ability_gate == true
  and .hotkey_stage_gate == true
  and .hotkey_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_HOTKEY_FEEDBACK_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
