#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.ppm"
RAW_SUMMARY="$SUMMARY.raw.$$"
TMP_SUMMARY="$SUMMARY.tmp.$$"
mkdir -p "$(dirname "$SUMMARY")"
cleanup() {
  rm -f "$RAW_SUMMARY" "$TMP_SUMMARY"
}
trap cleanup EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-control-loop "$PREVIEW" >"$RAW_SUMMARY"

jq '
  .move_command_queue_count = ((.move_command_queue // []) | length)
  | .attack_command_queue_count = ((.attack_command_queue // []) | length)
  | .move_production_queue_count = ((.move_production_queue // []) | length)
  | .attack_production_queue_count = ((.attack_production_queue // []) | length)
  | .move_build_queue_count = ((.move_build_queue // []) | length)
  | .attack_build_queue_count = ((.attack_build_queue // []) | length)
  | .move_resource_spend_log_count = ((.move_resource_spend_log // []) | length)
  | .attack_resource_spend_log_count = ((.attack_resource_spend_log // []) | length)
  | .move_combat_event_log_count = ((.move_combat_event_log // []) | length)
  | .attack_combat_event_log_count = ((.attack_combat_event_log // []) | length)
  | .move_visible_tile_count = ((.move_visible_tile_ids // []) | length)
  | .attack_visible_tile_count = ((.attack_visible_tile_ids // []) | length)
  | .move_fogged_tile_count = ((.move_fogged_tile_ids // []) | length)
  | .attack_fogged_tile_count = ((.attack_fogged_tile_ids // []) | length)
  | .move_ability_command_count = ((.move_ability_command_ids // []) | length)
  | .attack_ability_command_count = ((.attack_ability_command_ids // []) | length)
  | .move_ability_cooldown_count = ((.move_ability_cooldown_percents // []) | length)
  | .attack_ability_cooldown_count = ((.attack_ability_cooldown_percents // []) | length)
  | .control_loop_gate_count = ([.write_gate, .mirror_scene_gate, .coliseum_scene_gate, .selection_gate, .command_queue_gate, .strategy_hud_gate, .macro_loop_gate, .tactical_combat_gate, .gameplay_surface_gate] | length)
  | .control_loop_passed_gate_count = ([.write_gate, .mirror_scene_gate, .coliseum_scene_gate, .selection_gate, .command_queue_gate, .strategy_hud_gate, .macro_loop_gate, .tactical_combat_gate, .gameplay_surface_gate] | map(select(. == true)) | length)
  | .control_loop_failed_gate_count = ([.write_gate, .mirror_scene_gate, .coliseum_scene_gate, .selection_gate, .command_queue_gate, .strategy_hud_gate, .macro_loop_gate, .tactical_combat_gate, .gameplay_surface_gate] | map(select(. != true)) | length)
' "$RAW_SUMMARY" >"$TMP_SUMMARY"
mv "$TMP_SUMMARY" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_control_loop_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 360
  and .write_gate == true
  and .mirror_scene_gate == true
  and .coliseum_scene_gate == true
  and .non_background_pixels > 120000
  and .control_group_id == "1"
  and .move_selected_unit_count >= 4
  and .attack_selected_unit_count >= 4
  and .move_command_queue_count == (.move_command_queue | length)
  and .attack_command_queue_count == (.attack_command_queue | length)
  and .move_production_queue_count == (.move_production_queue | length)
  and .attack_production_queue_count == (.attack_production_queue | length)
  and .move_build_queue_count == (.move_build_queue | length)
  and .attack_build_queue_count == (.attack_build_queue | length)
  and .move_resource_spend_log_count == (.move_resource_spend_log | length)
  and .attack_resource_spend_log_count == (.attack_resource_spend_log | length)
  and .move_combat_event_log_count == (.move_combat_event_log | length)
  and .attack_combat_event_log_count == (.attack_combat_event_log | length)
  and .move_visible_tile_count == (.move_visible_tile_ids | length)
  and .attack_visible_tile_count == (.attack_visible_tile_ids | length)
  and .move_fogged_tile_count == (.move_fogged_tile_ids | length)
  and .attack_fogged_tile_count == (.attack_fogged_tile_ids | length)
  and .move_ability_command_count == (.move_ability_command_ids | length)
  and .attack_ability_command_count == (.attack_ability_command_ids | length)
  and .move_ability_cooldown_count == (.move_ability_cooldown_percents | length)
  and .attack_ability_cooldown_count == (.attack_ability_cooldown_percents | length)
  and (.move_command_queue | index("select_group_1") != null)
  and (.move_command_queue | index("move:7,4") != null)
  and (.move_command_queue | index("formation:diamond") != null)
  and (.attack_command_queue | index("select_group_1") != null)
  and (.attack_command_queue | index("attack:arena_creep_attack") != null)
  and .attack_target_id == "arena_creep_attack"
  and .selection_marker_pixel_count > 500
  and .formation_line_pixel_count > 200
  and .command_marker_pixel_count > 600
  and .attack_feedback_pixel_count > 180
  and .strategy_panel_pixel_count > 4000
  and .minimap_pixel_count > 2800
  and .fog_pixel_count > 400
  and .vision_pixel_count > 120
  and .resource_hud_pixel_count > 120
  and .production_queue_pixel_count > 900
  and (.move_production_queue | index("train:worker") != null)
  and (.move_build_queue | index("build:scout_tower") != null)
  and (.attack_build_queue | index("upgrade:training_hall") != null)
  and .move_training_progress_percent >= 50
  and .attack_build_progress_percent >= 50
  and .unit_health_card_pixel_count > 280
  and .ability_command_pixel_count > 800
  and .target_health_pixel_count > 60
  and .attack_target_health_percent < 60
  and .attack_active_ability_id == "focus_fire"
  and (.attack_ability_command_ids | index("focus_fire") != null)
  and (.attack_combat_event_log | index("damage:28") != null)
  and .selection_gate == true
  and .command_queue_gate == true
  and .strategy_hud_gate == true
  and .macro_loop_gate == true
  and .tactical_combat_gate == true
  and .gameplay_surface_gate == true
  and .control_loop_gate_count == 9
  and .control_loop_passed_gate_count == 9
  and .control_loop_failed_gate_count == 0
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_LOOP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
