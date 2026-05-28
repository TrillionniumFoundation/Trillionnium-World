#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-recall-override-preview.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-recall-override-preview.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-control-group-recall-override-preview "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_group_recall_override_preview_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_control_group_recall_override_preview_overlay"
  and .input_path == "apply_live_native_action_with_source(classic_rts_control_group_recall_override_preview_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.stage_summaries | length) == 4
  and (.stage_summaries | map(.stage) | index("group_26_recall_focus") != null)
  and (.stage_summaries | map(.stage) | index("group_26_queued_order") != null)
  and (.stage_summaries | map(.stage) | index("group_27_override_cancel") != null)
  and (.stage_summaries | map(.stage) | index("group_27_final_filtered") != null)
  and .group_26_recall_focus_tile == "18,30"
  and .group_26_queued_target_tile == "18,31"
  and (.group_26_member_ids | index("multi0.recall.order.runner") != null)
  and (.group_26_member_ids | index("multi0.recall.order.wing") != null)
  and .group_27_recall_focus_tile == "21,30"
  and .group_27_canceled_target_tile == "21,25"
  and (.group_27_canceled_member_ids | index("multi0.recall.override.runner") != null)
  and (.group_27_canceled_member_ids | index("multi0.recall.override.wing") != null)
  and .group_27_override_final_tile_ids == ["20,30", "22,30"]
  and (.group_27_filtered_member_ids | index("missing:multi0.recall.override.missing") != null)
  and (.group_27_filtered_member_ids | index("foreign:map.actor1") != null)
  and (.group_27_cleared_old_member_ids | index("old:multi0.recall.override.old.seed") != null)
  and (.group_27_cleared_old_member_ids | index("old:multi0.recall.override.old.wing") != null)
  and .hud_pixel_count > 700
  and .queue_pixel_count > 1200
  and .cancel_pixel_count > 700
  and .final_pixel_count > 700
  and .filtered_pixel_count > 400
  and .cleared_pixel_count > 250
  and .live_input_gate == true
  and .hud_visual_gate == true
  and .queue_visual_gate == true
  and .cancel_visual_gate == true
  and .final_visual_gate == true
  and .filtered_visual_gate == true
  and .cleared_visual_gate == true
  and .stage_gate == true
  and .group_26_recall_gate == true
  and .group_26_queued_gate == true
  and .group_27_override_gate == true
  and .group_27_filtered_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_RECALL_OVERRIDE_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
