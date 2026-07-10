#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-feedback-strip.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-feedback-strip.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-control-group-command-feedback-strip "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(classic_rts_control_group_command_feedback_strip_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.stage_summaries | length) == 4
  and (.stage_summaries | all(.renderer_path == "classic_draw_scene"))
  and (.stage_summaries | map(.stage) | index("group_26_queued") != null)
  and (.stage_summaries | map(.stage) | index("group_27_override") != null)
  and (.stage_summaries | map(.stage) | index("group_28_formation") != null)
  and (.stage_summaries | map(.stage) | index("group_28_filtered") != null)
  and .group_26_recall_focus_tile == "18,30"
  and .group_26_queued_target_tile == "18,31"
  and (.group_26_member_ids | index("multi0.recall.order.runner") != null)
  and (.group_26_member_ids | index("multi0.recall.order.wing") != null)
  and .group_27_recall_focus_tile == "21,30"
  and .group_27_canceled_target_tile == "21,25"
  and (.group_27_canceled_member_ids | index("multi0.recall.override.runner") != null)
  and (.group_27_canceled_member_ids | index("multi0.recall.override.wing") != null)
  and .group_27_override_final_tile_ids == ["20,30", "22,30"]
  and .group_28_recall_focus_tile == "1,30"
  and .group_28_formation_anchor_tile == "1,31"
  and .group_28_formation_slot_tile_ids == ["1,31", "2,31"]
  and (.group_28_member_ids | index("multi0.recall.formation.runner") != null)
  and (.group_28_member_ids | index("multi0.recall.formation.wing") != null)
  and (.filtered_member_ids | index("missing:multi0.recall.formation.missing") != null)
  and (.filtered_member_ids | index("foreign:map.actor1") != null)
  and (.cleared_old_member_ids | index("old:multi0.recall.formation.old.seed") != null)
  and (.cleared_old_member_ids | index("old:multi0.recall.formation.old.wing") != null)
  and .hud_pixel_count > 900
  and .queue_pixel_count > 900
  and .cancel_pixel_count > 500
  and .final_pixel_count > 500
  and .filtered_pixel_count > 500
  and .cleared_pixel_count > 500
  and .anchor_pixel_count > 500
  and .live_input_gate == true
  and .hud_visual_gate == true
  and .queue_visual_gate == true
  and .cancel_visual_gate == true
  and .final_visual_gate == true
  and .filtered_visual_gate == true
  and .cleared_visual_gate == true
  and .anchor_visual_gate == true
  and .stage_gate == true
  and .group_26_strip_gate == true
  and .group_27_strip_gate == true
  and .group_28_strip_gate == true
  and .filtered_cleared_strip_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_FEEDBACK_STRIP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
