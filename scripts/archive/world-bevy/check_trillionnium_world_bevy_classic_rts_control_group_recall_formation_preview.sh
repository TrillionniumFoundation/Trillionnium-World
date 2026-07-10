#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-recall-formation-preview.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-recall-formation-preview.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-control-group-recall-formation-preview "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_control_group_recall_formation_preview_overlay"
  and .input_path == "apply_live_native_action_with_source(classic_rts_control_group_recall_formation_preview_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.stage_summaries | length) == 4
  and (.stage_summaries | map(.stage) | index("recall_focus_hud") != null)
  and (.stage_summaries | map(.stage) | index("formation_anchor_slots") != null)
  and (.stage_summaries | map(.stage) | index("queued_valid_members") != null)
  and (.stage_summaries | map(.stage) | index("filtered_invalid") != null)
  and .final_control_group_id == "28"
  and .final_recall_focus_tile == "1,30"
  and .final_formation_anchor_tile == "1,31"
  and .final_formation_slot_tile_ids == ["1,31", "2,31"]
  and (.final_selected_unit_ids | index("multi0.recall.formation.runner") != null)
  and (.final_selected_unit_ids | index("multi0.recall.formation.wing") != null)
  and (.final_filtered_member_ids | index("missing:multi0.recall.formation.missing") != null)
  and (.final_filtered_member_ids | index("foreign:map.actor1") != null)
  and (.final_cleared_old_member_ids | index("old:multi0.recall.formation.old.seed") != null)
  and (.final_cleared_old_member_ids | index("old:multi0.recall.formation.old.wing") != null)
  and .hud_pixel_count > 650
  and .focus_pixel_count > 350
  and .anchor_pixel_count > 350
  and .slot_pixel_count > 180
  and .queued_pixel_count > 1500
  and .filtered_pixel_count > 1000
  and .cleared_pixel_count > 400
  and .live_input_gate == true
  and .hud_visual_gate == true
  and .focus_visual_gate == true
  and .anchor_visual_gate == true
  and .slot_visual_gate == true
  and .queued_visual_gate == true
  and .filtered_visual_gate == true
  and .cleared_visual_gate == true
  and .stage_gate == true
  and .recall_hud_gate == true
  and .formation_anchor_gate == true
  and .queued_member_gate == true
  and .filtered_member_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_RECALL_FORMATION_PREVIEW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
