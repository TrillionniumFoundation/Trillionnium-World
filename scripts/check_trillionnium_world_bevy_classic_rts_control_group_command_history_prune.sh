#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-history-prune.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-history-prune.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-control-group-command-history-prune "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_group_command_history_prune_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(classic_rts_control_group_command_history_prune_input)"
  and .input_action_count == 3
  and .accepted_input_count == 3
  and (.stage_summaries | length) == 3
  and (.stage_summaries | all(.renderer_path == "classic_draw_scene"))
  and (.stage_summaries | map(.stage) | index("overflow_input_pruned") != null)
  and (.stage_summaries | map(.stage) | index("recent_three_retained") != null)
  and (.stage_summaries | map(.stage) | index("cleared_history_bounded") != null)
  and (.stage_summaries | all(.history_entry_count == 3))
  and (.stage_summaries | all(.pruned_entry_count == 2))
  and (.stage_summaries | all(.history_overflow_row_count == 0))
  and (.stage_summaries | all(.stale_group_25_visible == false))
  and (.stage_summaries | all(.active_strip_cleared == true))
  and (.stage_summaries | all(.active_stale_signal_pixel_count == 0))
  and .history_entry_count == 3
  and .history_capacity == 3
  and .history_overflow_row_count == 0
  and .retained_history_group_ids == ["26", "27", "28"]
  and .pruned_history_group_ids == ["25", "24"]
  and .pruned_entry_count == 2
  and .stale_group_25_visible == false
  and .active_strip_lifecycle_states == ["cleared"]
  and .group_26_queued_target_tile == "18,31"
  and .group_27_canceled_target_tile == "21,25"
  and .group_27_override_final_tile_ids == ["20,30", "22,30"]
  and .group_28_formation_anchor_tile == "1,31"
  and .group_28_formation_slot_tile_ids == ["1,31", "2,31"]
  and .group_25_pruned_target_tile == "17,30"
  and .group_24_pruned_target_tile == "16,29"
  and (.history_entries | map(.group_id) | index("26") != null)
  and (.history_entries | map(.group_id) | index("27") != null)
  and (.history_entries | map(.group_id) | index("28") != null)
  and (.history_entries | map(.group_id) | index("25") == null)
  and (.pruned_history_entries | map(.group_id) | index("25") != null)
  and (.pruned_history_entries | map(.group_id) | index("24") != null)
  and .history_frame_pixel_count > 1200
  and .history_row_pixel_count > 8000
  and .history_row_pixel_count < 90000
  and .history_badge_pixel_count > 900
  and .history_age_pixel_count > 700
  and .history_retained_pixel_count > 350
  and .history_pruned_pixel_count > 250
  and .history_limit_pixel_count > 150
  and .cleared_ready_pixel_count > 350
  and .cleared_active_stale_pixel_count == 0
  and .live_input_gate == true
  and .stage_gate == true
  and .history_visual_gate == true
  and .history_prune_visual_gate == true
  and .retained_entry_gate == true
  and .pruned_entry_gate == true
  and .no_overflow_gate == true
  and .cleared_history_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_COMMAND_HISTORY_PRUNE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
