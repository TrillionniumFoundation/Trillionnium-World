#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack-scene-probe.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack-scene-probe.ppm"
OVERRIDE_DIR="$ROOT/assets/trnm-world/classic/art-pack-v1"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-art-pack-scene-probe "$OVERRIDE_DIR" "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_art_pack_scene_probe_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 360
  and .override_presence_gate == true
  and .color_probe_gate == true
  and .terrain_override_presence_gate == true
  and .terrain_color_probe_gate == true
  and .world_prop_override_presence_gate == true
  and .world_prop_color_probe_gate == true
  and .neutral_unit_override_presence_gate == true
  and .neutral_unit_color_probe_gate == true
  and .environment_override_presence_gate == true
  and .environment_detail_color_probe_gate == true
  and .vfx_override_presence_gate == true
  and .vfx_color_probe_gate == true
  and .replacement_boundary_gate == true
  and .mirror_scene_gate == true
  and .coliseum_scene_gate == true
  and .non_background_pixels > 120000
  and .town_hall_color_count > 20
  and .waygate_color_count > 20
  and .tree_color_count > 20
  and .coliseum_color_count > 20
  and .player_color_count > 20
  and .enemy_attack_color_count > 20
  and .terrain_grass_color_count > 600
  and .terrain_road_color_count > 100
  and .terrain_water_color_count > 40
  and .terrain_wall_roof_color_count > 80
  and .world_prop_runtime_color_count > 900
  and .neutral_unit_runtime_color_count > 350
  and .environment_detail_color_count > 2000
  and .command_marker_color_count > 200
  and .attack_arc_color_count > 100
  and .hit_flash_color_count > 80
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ART_PACK_SCENE_PROBE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
