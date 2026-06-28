#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_loop.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_loop_v1'
  'bevy-classic-rts-control-loop.json'
  'bevy-classic-rts-control-loop.ppm'
  'classic-rts-control-loop'
  'preview_width == 1280'
  'preview_height == 360'
  'move_selected_unit_count >= 4'
  'attack_selected_unit_count >= 4'
  'move_command_queue_count == (.move_command_queue | length)'
  'attack_command_queue_count == (.attack_command_queue | length)'
  'move_visible_tile_count == (.move_visible_tile_ids | length)'
  'attack_visible_tile_count == (.attack_visible_tile_ids | length)'
  'control_loop_gate_count == 9'
  'control_loop_passed_gate_count == 9'
  'control_loop_failed_gate_count == 0'
  'move:7,4'
  'formation:diamond'
  'attack:arena_creep_attack'
  'selection_marker_pixel_count > 500'
  'formation_line_pixel_count > 200'
  'command_marker_pixel_count > 600'
  'attack_feedback_pixel_count > 180'
  'strategy_panel_pixel_count > 4000'
  'minimap_pixel_count > 2800'
  'fog_pixel_count > 400'
  'vision_pixel_count > 120'
  'resource_hud_pixel_count > 120'
  'production_queue_pixel_count > 900'
  'move_training_progress_percent >= 50'
  'attack_build_progress_percent >= 50'
  'unit_health_card_pixel_count > 280'
  'ability_command_pixel_count > 800'
  'target_health_pixel_count > 60'
  'attack_active_ability_id == "focus_fire"'
  'selection_gate == true'
  'command_queue_gate == true'
  'strategy_hud_gate == true'
  'macro_loop_gate == true'
  'tactical_combat_gate == true'
  'gameplay_surface_gate == true'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control loop script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_LOOP_CONTRACT'
  'native_classic_rts_control_loop_evidence_json'
  'classic-rts-control-loop'
  'classic-rts-control'
  'rts_control_group_id'
  'rts_selected_unit_ids'
  'rts_command_queue'
  'rts_command_destination_tile'
  'rts_attack_target_id'
  'rts_visible_tile_ids'
  'rts_fogged_tile_ids'
  'rts_production_queue'
  'rts_build_queue'
  'rts_resource_spend_log'
  'rts_unit_health_percents'
  'rts_ability_command_ids'
  'rts_ability_cooldown_percents'
  'rts_active_ability_id'
  'rts_target_health_percent'
  'rts_combat_event_log'
  'classic_parse_rts_tile'
  'classic_rts_control_group_entities'
  'classic_rts_visible_tiles'
  'classic_rts_fogged_tiles'
  'classic_draw_iso_rts_selection_marker'
  'classic_draw_iso_rts_formation_line'
  'classic_draw_rts_strategy_overlay'
  'classic_draw_rts_queue_slot'
  'classic_draw_rts_ability_slot'
  'CLASSIC_ISO_CONTROL_GROUP_COLOR'
  'CLASSIC_ISO_FORMATION_LINE_COLOR'
  'CLASSIC_RTS_MINIMAP_TERRAIN_COLOR'
  'CLASSIC_RTS_MINIMAP_FOG_COLOR'
  'CLASSIC_RTS_RESOURCE_LUMBER_COLOR'
  'CLASSIC_RTS_PRODUCTION_PROGRESS_COLOR'
  'CLASSIC_RTS_UNIT_CARD_HEALTH_COLOR'
  'CLASSIC_RTS_ABILITY_COOLDOWN_COLOR'
  'CLASSIC_RTS_ACTIVE_ABILITY_COLOR'
  'selection_marker_pixel_count'
  'formation_line_pixel_count'
  'command_marker_pixel_count'
  'attack_feedback_pixel_count'
  'strategy_panel_pixel_count'
  'minimap_pixel_count'
  'fog_pixel_count'
  'vision_pixel_count'
  'resource_hud_pixel_count'
  'production_queue_pixel_count'
  'unit_health_card_pixel_count'
  'ability_command_pixel_count'
  'target_health_pixel_count'
  'strategy_hud_gate'
  'macro_loop_gate'
  'tactical_combat_gate'
  'gameplay_surface_gate'
  'RTS group 1 moving to waypoint'
  'RTS attack order accepted'
  'not_cex_runtime'
  'wgpu_required'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control loop source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_control_loop.sh'
  'bevy-classic-rts-control-loop.json'
  'bevy-classic-rts-control-loop.ppm'
  'classic_rts_control_loop_green'
  'rts_control_loop_selection_gate'
  'rts_control_loop_command_queue_gate'
  'rts_control_loop_gameplay_surface_gate'
  'rts_control_loop_strategy_hud_gate'
  'rts_control_loop_macro_loop_gate'
  'rts_control_loop_tactical_combat_gate'
  'rts_control_loop_move_selected_unit_count'
  'rts_control_loop_attack_selected_unit_count'
  'rts_control_loop_minimap_pixel_count'
  'rts_control_loop_fog_pixel_count'
  'rts_control_loop_vision_pixel_count'
  'rts_control_loop_production_queue_pixel_count'
  'rts_control_loop_unit_health_card_pixel_count'
  'rts_control_loop_ability_command_pixel_count'
  'rts_control_loop_target_health_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS control loop readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control loop keeps selection, command queue, attack feedback, minimap/resource HUD, and readiness gates connected"
