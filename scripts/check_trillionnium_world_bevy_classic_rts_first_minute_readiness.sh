#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-minute-readiness.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-minute-readiness.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-first-minute-readiness "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_minute_readiness_v1"
  and .green == true
  and .campaign_entry_contract == "trillionnium_world_bevy_classic_rts_campaign_entry_v1"
  and .objective_minimap_breadcrumbs_contract == "trillionnium_world_bevy_classic_rts_objective_minimap_breadcrumbs_v1"
  and .campaign_handoff_contract == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1"
  and .preview_width == 1920
  and .preview_height == 1080
  and .runtime_screen_mode == "player_runtime_first_minute_readiness_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .runtime_screen_layout.campaign_start_resume == "title campaign start/continue/replay into local save-slot restore"
  and .runtime_screen_layout.primary_tactical_viewport == "large restored route tactical state from the first-minute handoff"
  and .runtime_screen_layout.route_progression == "right-side route rail and bottom timeline replace the old 4x4 contact sheet"
  and (.title_actions | index("CAMPAIGN:START") != null)
  and (.title_actions | index("CAMPAIGN:CONTINUE") != null)
  and (.title_actions | index("CAMPAIGN:REPLAY") != null)
  and .input_action_count == 73
  and .start_input_count == 73
  and .replay_input_count == 73
  and .campaign_slot_bytes > 20000
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_outdoor"
  and .final_open_world_handoff_state == "resumed:league-coliseum"
  and .final_contextual_primary_action_label == "COMBAT:attack"
  and .final_objective_status == "open_world_after_action_ready"
  and (.final_next_action_ids | index("secure_expansion") != null)
  and (.final_next_action_ids | index("open_world_after_action") != null)
  and (.final_next_action_ids | index("resume_world_route") != null)
  and (.final_route_director_path | index("mirror-city-square") != null)
  and (.final_route_director_path | index("league-coliseum") != null)
  and (.final_route_director_history | index("route_director:task-fixture-first-route:mirror-city-square->league-coliseum") != null)
  and (.final_route_director_history | index("rts_open_world_after_action:league-coliseum:arrived") != null)
  and .campaign_entry_gate == true
  and .campaign_arrival_gate == true
  and .breadcrumb_gate == true
  and .breadcrumb_route_gate == true
  and .preview_gate == true
  and .native_boundary_gate == true
  and .first_minute_pixel_counts.player_first_campaign_view_non_background > 600000
  and .first_minute_pixel_counts.player_first_campaign_view_frame > 10000
  and .first_minute_pixel_counts.player_first_campaign_status_strip > 8000
  and .first_minute_pixel_counts.player_first_campaign_route_rail > 100000
  and .player_first_first_minute_screen_gate == true
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_MINUTE_READINESS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
