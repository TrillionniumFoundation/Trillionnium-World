#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1'
  'bevy-classic-rts-camera-minimap-sync.json'
  'bevy-classic-rts-camera-minimap-sync.ppm'
  'classic-rts-camera-minimap-sync'
  'large_map.map_width_tiles == 34'
  'large_map_minimap_gate == true'
  'viewport_sync_gate == true'
  'fog_reveal_gate == true'
  'selection_follow_gate == true'
  'control_group_sync_gate == true'
  'route_projection_gate == true'
  'zoom_rect_sync_gate == true'
  'minimap_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMERA_MINIMAP_SYNC_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS camera minimap sync script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMERA_MINIMAP_SYNC_CONTRACT'
  'native_classic_rts_camera_minimap_sync_evidence_json'
  'classic_draw_rts_camera_minimap_sync_overlay'
  'rts_camera_minimap_viewport_rect'
  'rts_camera_minimap_revealed_tiles'
  'rts_camera_minimap_selection_follow_step'
  'CLASSIC_RTS_LARGE_MAP_WIDTH_TILES'
  'CLASSIC_RTS_LARGE_MAP_MAX_X'
  'CLASSIC_RTS_CAMERA_SYNC_VIEWPORT_COLOR'
  'CLASSIC_RTS_CAMERA_SYNC_FOG_COLOR'
  'CLASSIC_RTS_CAMERA_SYNC_REVEAL_COLOR'
  'CLASSIC_RTS_CAMERA_SYNC_SELECTION_COLOR'
  'CLASSIC_RTS_CAMERA_SYNC_ROUTE_COLOR'
  'Original Trillionnium camera/minimap sync overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS camera minimap sync source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh'
  'bevy-classic-rts-camera-minimap-sync.json'
  'classic_rts_camera_minimap_sync_green'
  'rts_camera_minimap_sync_viewport_sync_gate'
  'rts_camera_minimap_sync_fog_reveal_gate'
  'rts_camera_minimap_sync_selection_follow_gate'
  'rts_camera_minimap_sync_control_group_sync_gate'
  'rts_camera_minimap_sync_route_projection_gate'
  'rts_camera_minimap_sync_zoom_rect_sync_gate'
  'rts_camera_minimap_sync_minimap_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS camera minimap sync readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1'
  'bevy_classic_rts_camera_minimap_sync_contract_guard'
  'bevy_classic_rts_camera_minimap_sync_gate'
  'bevy_classic_rts_camera_minimap_sync_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS camera minimap sync release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS camera minimap sync evidence remains connected to renderer, CLI, readiness, release-review, minimap runtime, and original art policy"
