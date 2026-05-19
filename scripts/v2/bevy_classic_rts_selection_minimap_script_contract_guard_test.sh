#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_selection_minimap_v1'
  'bevy-classic-rts-selection-minimap.json'
  'bevy-classic-rts-selection-minimap.ppm'
  'classic-rts-selection-minimap'
  'input_path == "apply_live_native_action_with_source(classic_rts_selection_minimap_input)"'
  'RTS:SELECT:box:frontline'
  'RTS:MOVE:minimap:9,2:rally'
  'RTS:SELECT:2'
  'RTS:MOVE:6,5:split'
  'selection_box_gate == true'
  'control_group_gate == true'
  'minimap_command_gate == true'
  'split_route_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS selection/minimap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_MINIMAP_CONTRACT'
  'native_classic_rts_selection_minimap_evidence_json'
  'classic-rts-selection-minimap'
  'classic_rts_selection_minimap_input'
  'rts_selection_box_tile_ids'
  'rts_control_group_assignments'
  'rts_active_control_group_ids'
  'rts_minimap_command_tile_id'
  'rts_minimap_command_kind'
  'rts_group_route_tile_ids'
  'rts_group_command_state'
  'classic_rts_selection_box_tiles'
  'classic_rts_group_two_units'
  'CLASSIC_RTS_SELECTION_BOX_COLOR'
  'CLASSIC_RTS_MINIMAP_COMMAND_COLOR'
  'CLASSIC_RTS_GROUP_TWO_COLOR'
  'CLASSIC_RTS_SPLIT_ROUTE_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS selection/minimap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_selection_minimap.sh'
  'bevy-classic-rts-selection-minimap.json'
  'classic_rts_selection_minimap_green'
  'rts_selection_box_gate'
  'rts_control_group_gate'
  'rts_minimap_command_gate'
  'rts_split_route_gate'
  'rts_selection_minimap_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS selection/minimap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS selection/minimap evidence remains connected to live box-select, control-group, minimap command, split-route runtime state, renderer overlays, and readiness"
