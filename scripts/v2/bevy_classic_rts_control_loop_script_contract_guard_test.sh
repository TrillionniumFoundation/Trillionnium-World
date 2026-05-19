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
  'move:7,4'
  'formation:diamond'
  'attack:arena_creep_attack'
  'selection_marker_pixel_count > 500'
  'formation_line_pixel_count > 200'
  'command_marker_pixel_count > 600'
  'attack_feedback_pixel_count > 180'
  'selection_gate == true'
  'command_queue_gate == true'
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
  'classic_parse_rts_tile'
  'classic_rts_control_group_entities'
  'classic_draw_iso_rts_selection_marker'
  'classic_draw_iso_rts_formation_line'
  'CLASSIC_ISO_CONTROL_GROUP_COLOR'
  'CLASSIC_ISO_FORMATION_LINE_COLOR'
  'selection_marker_pixel_count'
  'formation_line_pixel_count'
  'command_marker_pixel_count'
  'attack_feedback_pixel_count'
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
  'rts_control_loop_move_selected_unit_count'
  'rts_control_loop_attack_selected_unit_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS control loop readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control loop keeps selection, command queue, attack feedback, and readiness gates connected"
