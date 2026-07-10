#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-history.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-history.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-control-group-command-history "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_group_command_history_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(classic_rts_control_group_command_history_input)"
  and .input_action_count == 3
  and .accepted_input_count == 3
  and (.stage_summaries | length) == 3
  and (.stage_summaries | all(.renderer_path == "classic_draw_scene"))
  and (.stage_summaries | map(.stage) | index("fresh_history_appended") != null)
  and (.stage_summaries | map(.stage) | index("dimmed_history_retained") != null)
  and (.stage_summaries | map(.stage) | index("cleared_history_retained") != null)
  and .history_entry_count == 3
  and .history_group_ids == ["26", "27", "28"]
  and .active_strip_lifecycle_states == ["fresh", "dimmed", "cleared"]
  and .group_26_queued_target_tile == "18,31"
  and (.group_26_member_ids | index("multi0.recall.order.runner") != null)
  and (.group_26_member_ids | index("multi0.recall.order.wing") != null)
  and .group_27_canceled_target_tile == "21,25"
  and .group_27_override_final_tile_ids == ["20,30", "22,30"]
  and (.group_27_member_ids | index("multi0.recall.override.runner") != null)
  and (.group_27_member_ids | index("multi0.recall.override.wing") != null)
  and .group_28_formation_anchor_tile == "1,31"
  and .group_28_formation_slot_tile_ids == ["1,31", "2,31"]
  and (.group_28_member_ids | index("multi0.recall.formation.runner") != null)
  and (.group_28_member_ids | index("multi0.recall.formation.wing") != null)
  and (.history_entries | map(.group_id) | index("26") != null)
  and (.history_entries | map(.group_id) | index("27") != null)
  and (.history_entries | map(.group_id) | index("28") != null)
  and .history_frame_pixel_count > 1200
  and .history_row_pixel_count > 8000
  and .history_badge_pixel_count > 900
  and .history_age_pixel_count > 700
  and .history_retained_pixel_count > 350
  and .cleared_ready_pixel_count > 350
  and .cleared_active_stale_pixel_count == 0
  and .live_input_gate == true
  and .stage_gate == true
  and .history_visual_gate == true
  and .history_entry_gate == true
  and .cleared_history_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
