#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-creep-camp-terrain-route.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-creep-camp-terrain-route.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-creep-camp-terrain-route "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_creep_camp_terrain_route_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_creep_camp_terrain_route_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:scout:creep_camp@8,3") != null)
  and (.action_labels | index("RTS:MOVE:8,3:wedge") != null)
  and (.action_labels | index("RTS:ATTACK:forest_creep_camp") != null)
  and (.action_labels | index("RTS:ABILITY:guard_break") != null)
  and (.action_labels | index("RTS:QUEUE:camp:clear:forest_creep_camp@8,3") != null)
  and (.final_creep_camp_tile_ids | length) >= 4
  and (.final_creep_camp_unit_ids | length) >= 3
  and .final_creep_camp_state == "cleared:forest_creep_camp"
  and (.final_terrain_route_tile_ids | length) >= 4
  and (.final_terrain_choke_tile_ids | length) >= 3
  and (.final_expansion_tile_ids | length) >= 3
  and .final_scout_reveal_percent == 100
  and .final_target_health_percent <= 18
  and (.final_command_queue | index("scout:forest_creep_camp@8,3") != null)
  and (.final_command_queue | index("camp_clear:forest_creep_camp@8,3") != null)
  and (.final_command_queue | index("expansion:forest_relay@9,2") != null)
  and .non_background_pixels > 250000
  and .camp_pixel_count > 100
  and .terrain_route_pixel_count > 80
  and .choke_pixel_count > 40
  and .expansion_pixel_count > 50
  and .scout_reveal_pixel_count > 20
  and .live_creep_camp_input_gate == true
  and .terrain_route_gate == true
  and .choke_gate == true
  and .camp_clear_gate == true
  and .scout_reveal_gate == true
  and .expansion_route_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CREEP_CAMP_TERRAIN_ROUTE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
