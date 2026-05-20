#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-handoff.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-handoff.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-campaign-handoff "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_campaign_handoff_input)"
  and .input_action_count == 73
  and .accepted_input_count == 73
  and .capture_frame_count == 16
  and (.input_sources | index("classic_rts_campaign_handoff_input") != null)
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:objective:extract:relay_beacon@9,2") != null)
  and (.action_labels | index("RTS:QUEUE:camp:clear:forest_creep_camp@8,3") != null)
  and (.action_labels | index("RTS:QUEUE:recon:mark:enemy_base@10,2") != null)
  and (.action_labels | index("RTS:QUEUE:counter:fortify:watch_tower@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:aftermath:next:secure_expansion@9,2") != null)
  and (.action_labels | index("RTS:QUEUE:commander:ability:rally_aura@mirror_captain") != null)
  and (.action_labels | index("RTS:QUEUE:expansion:defend:counter_wave@8,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:push:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:finish:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_secure:signal_core@12,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_pressure:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_claim:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:victory_handoff:mirror_city@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:open_world_resume:league-coliseum@12,3") != null)
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_outdoor"
  and .final_route_director_task_id == "task-fixture-first-route"
  and .final_route_director_target_room_id == "league-coliseum"
  and .final_route_director_next_room_id == null
  and .final_open_world_handoff_state == "resumed:league-coliseum"
  and .final_contextual_primary_action_label == "COMBAT:attack"
  and (.final_contextual_action_labels | index("COMBAT:attack") != null)
  and (.final_active_task_ids | index("task-fixture-first-route") != null)
  and .final_objective_status == "open_world_after_action_ready"
  and .final_match_result_state == "classic_rts_restored:mirror_city"
  and .snapshot_json_byte_count > 20000
  and .restored_node_id == "mirror-city-square"
  and .restored_current_room_id == "league-coliseum"
  and .restored_map_scene == "arena_outdoor"
  and .restored_open_world_handoff_state == "resumed:league-coliseum"
  and .restored_route_director_task_id == "task-fixture-first-route"
  and .restored_route_director_next_room_id == null
  and (.restored_contextual_action_labels | index("COMBAT:attack") != null)
  and (.restored_active_task_ids | index("task-fixture-first-route") != null)
  and (.milestones | to_entries | all(.value == true))
  and .non_background_pixels > 500000
  and .victory_pixel_count > 20
  and .expansion_pixel_count > 60
  and .breach_pixel_count > 40
  and .keep_pixel_count > 40
  and .restoration_pixel_count > 20
  and .open_world_pixel_count > 60
  and .live_campaign_input_gate == true
  and .early_campaign_gate == true
  and .mid_campaign_gate == true
  and .end_campaign_gate == true
  and .open_world_resume_gate == true
  and .snapshot_round_trip_gate == true
  and .render_milestone_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_HANDOFF_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
