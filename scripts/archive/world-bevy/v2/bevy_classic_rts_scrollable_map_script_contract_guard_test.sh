#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_scrollable_map.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_scrollable_map_v1'
  'bevy-classic-rts-scrollable-map.json'
  'bevy-classic-rts-scrollable-map.ppm'
  'classic-rts-scrollable-map'
  'large_map.map_width_tiles == 34'
  'large_map_coordinate_gate == true'
  'keyboard_pan_gate == true'
  'edge_scroll_gate == true'
  'drag_pan_gate == true'
  'wheel_zoom_gate == true'
  'minimap_jump_gate == true'
  'boundary_clamp_gate == true'
  'map_layer_projection_gate == true'
  'hud_fixed_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SCROLLABLE_MAP_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS scrollable map script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SCROLLABLE_MAP_CONTRACT'
  'native_classic_rts_scrollable_map_evidence_json'
  'classic_draw_rts_scrollable_map_overlay'
  'update_native_rts_scrollable_map_camera'
  'apply_native_rts_scrollable_map_view'
  'BevyRtsScrollableMapCamera'
  'BevyWorldScrollableMapAnchor'
  'rts_scrollable_map_camera_config'
  'apply_rts_scrollable_map_camera_input'
  'CLASSIC_RTS_LARGE_MAP_WIDTH_TILES'
  'CLASSIC_RTS_LARGE_MAP_MAX_X'
  'CLASSIC_RTS_SCROLL_CAMERA_FRAME_COLOR'
  'CLASSIC_RTS_SCROLL_EDGE_COLOR'
  'CLASSIC_RTS_SCROLL_DRAG_COLOR'
  'CLASSIC_RTS_SCROLL_ZOOM_COLOR'
  'CLASSIC_RTS_SCROLL_MINIMAP_COLOR'
  'CLASSIC_RTS_SCROLL_CLAMP_COLOR'
  'Original Trillionnium scrollable-map camera overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS scrollable map source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_scrollable_map.sh'
  'bevy-classic-rts-scrollable-map.json'
  'classic_rts_scrollable_map_green'
  'rts_scrollable_map_keyboard_pan_gate'
  'rts_scrollable_map_edge_scroll_gate'
  'rts_scrollable_map_drag_pan_gate'
  'rts_scrollable_map_wheel_zoom_gate'
  'rts_scrollable_map_minimap_jump_gate'
  'rts_scrollable_map_boundary_clamp_gate'
  'rts_scrollable_map_map_layer_projection_gate'
  'rts_scrollable_map_hud_fixed_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS scrollable map readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_scrollable_map_v1'
  'bevy_classic_rts_scrollable_map_contract_guard'
  'bevy_classic_rts_scrollable_map_gate'
  'bevy_classic_rts_scrollable_map_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_scrollable_map.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS scrollable map release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS scrollable map evidence remains connected to renderer, CLI, readiness, release-review, runtime camera input, map projection, and original art policy"
