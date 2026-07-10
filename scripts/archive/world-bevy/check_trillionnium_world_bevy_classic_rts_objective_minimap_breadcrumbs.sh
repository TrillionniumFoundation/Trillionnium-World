#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-minimap-breadcrumbs.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-minimap-breadcrumbs.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-objective-minimap-breadcrumbs "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_objective_minimap_breadcrumbs_v1"
  and .green == true
  and .campaign_handoff_contract == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1"
  and .campaign_handoff_green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and (.required_action_breadcrumbs | index("objective:claim:relay_beacon@6,5") != null)
  and (.required_action_breadcrumbs | index("tier2:open_world_resume:league-coliseum@12,3") != null)
  and (.required_minimap_breadcrumbs | index("minimap:rally:9,2") != null)
  and (.required_minimap_breadcrumbs | index("rts_open_world_after_action:league-coliseum:arrived") != null)
  and (.final_next_action_ids | index("secure_expansion") != null)
  and (.final_next_action_ids | index("open_world_after_action") != null)
  and (.final_next_action_ids | index("resume_world_route") != null)
  and (.final_route_director_path | index("mirror-city-square") != null)
  and (.final_route_director_path | index("league-coliseum") != null)
  and (.final_route_director_history | index("route_director:task-fixture-first-route:mirror-city-square->league-coliseum") != null)
  and (.final_route_director_history | index("rts_open_world_after_action:league-coliseum:route_ready") != null)
  and (.final_route_director_history | index("rts_open_world_after_action:league-coliseum:arrived") != null)
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_outdoor"
  and .final_objective_status == "open_world_after_action_ready"
  and .victory_pixel_count > 20
  and .expansion_pixel_count > 60
  and .keep_pixel_count > 40
  and .restoration_pixel_count > 20
  and .open_world_pixel_count > 60
  and .action_label_gate == true
  and .command_queue_gate == true
  and .next_action_gate == true
  and .route_director_gate == true
  and .objective_breadcrumb_gate == true
  and .minimap_breadcrumb_gate == true
  and .milestone_gate == true
  and .ui_continuity_gate == true
  and .native_client_boundary_gate == true
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OBJECTIVE_MINIMAP_BREADCRUMBS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
