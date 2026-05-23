#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-ui-modeling-readiness.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-ui-modeling-readiness"
mkdir -p "$PREVIEW_DIR" "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-map-ui-modeling-readiness "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1"
  and .green == true
  and .preview_count == 6
  and .source_contracts.visual_fidelity == "trillionnium_world_bevy_classic_rts_visual_fidelity_v1"
  and .source_contracts.command_affordance == "trillionnium_world_bevy_classic_rts_command_affordance_v1"
  and .source_contracts.scrollable_map == "trillionnium_world_bevy_classic_rts_scrollable_map_v1"
  and .source_contracts.camera_minimap_sync == "trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1"
  and .source_contracts.structure_modeling == "trillionnium_world_bevy_classic_rts_structure_modeling_v1"
  and .source_contracts.environment_life == "trillionnium_world_bevy_classic_rts_environment_life_v1"
  and .visual_gate == true
  and .command_gate == true
  and .scroll_gate == true
  and .camera_gate == true
  and .structure_gate == true
  and .environment_gate == true
  and .source_policy_gate == true
  and .preview_gate == true
  and .visual_fidelity_pixels.fidelity_panel > 16000
  and .visual_fidelity_pixels.model_edge > 1200
  and .command_affordance_pixels.drag_marquee > 80
  and .command_affordance_pixels.hotkey > 200
  and .map_camera_pixels.scroll_minimap > 600
  and .map_camera_pixels.camera_viewport > 2400
  and .map_camera_pixels.camera_fog > 8000
  and .map_camera_pixels.camera_route > 900
  and .modeling_pixels.foundation_shadow > 220
  and .modeling_pixels.scaffold > 300
  and .modeling_pixels.production_glow > 120
  and .modeling_pixels.tree_sway > 160
  and .modeling_pixels.water_shimmer > 120
  and .modeling_pixels.ambient_dust > 120
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/visual-fidelity.ppm"
test -s "$PREVIEW_DIR/command-affordance.ppm"
test -s "$PREVIEW_DIR/scrollable-map.ppm"
test -s "$PREVIEW_DIR/camera-minimap-sync.ppm"
test -s "$PREVIEW_DIR/structure-modeling.ppm"
test -s "$PREVIEW_DIR/environment-life.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MAP_UI_MODELING_READINESS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
