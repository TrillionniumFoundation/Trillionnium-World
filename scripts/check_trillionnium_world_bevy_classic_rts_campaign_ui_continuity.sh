#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-campaign-ui-continuity "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1"
  and .green == true
  and .campaign_handoff_contract == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1"
  and .campaign_handoff_green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .runtime_screen_mode == "player_runtime_campaign_ui_continuity_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .runtime_screen_layout.primary_tactical_viewport == "large restored route tactical state with open-world resume status"
  and .runtime_screen_layout.campaign_route_rail == "sixteen accepted campaign handoff stages shown as a player-side route rail"
  and .runtime_screen_layout.resume_timeline == "bottom player timeline binds objective, expansion, siege, keep, restoration, and open-world resume"
  and .rts_evidence_campaign_ui_continuity_review_contract == "trnm_rts_evidence_campaign_ui_continuity_review_v1"
  and .rts_evidence_campaign_ui_continuity_review.green == true
  and .rts_evidence_campaign_ui_continuity_review.campaign_handoff_contract == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1"
  and .rts_evidence_campaign_ui_continuity_review.preview_width == 1920
  and .rts_evidence_campaign_ui_continuity_review.preview_height == 1080
  and .rts_evidence_campaign_ui_continuity_review.capture_frame_count == 16
  and .rts_evidence_campaign_ui_continuity_review.final_current_room_id == "league-coliseum"
  and .rts_evidence_campaign_ui_continuity_review.restored_current_room_id == "league-coliseum"
  and .rts_evidence_campaign_ui_continuity_review.handoff_green_gate == true
  and .rts_evidence_campaign_ui_continuity_review.preview_resolution_gate == true
  and .rts_evidence_campaign_ui_continuity_review.live_input_gate == true
  and .rts_evidence_campaign_ui_continuity_review.milestone_gate == true
  and .rts_evidence_campaign_ui_continuity_review.map_ui_state_gate == true
  and .rts_evidence_campaign_ui_continuity_review.restored_ui_state_gate == true
  and .rts_evidence_campaign_ui_continuity_review.persistence_gate == true
  and .rts_evidence_campaign_ui_continuity_review.render_readability_gate == true
  and .rts_evidence_campaign_ui_continuity_review.native_client_boundary_gate == true
  and .rts_evidence_campaign_ui_continuity_review.player_first_campaign_continuity_screen_gate == true
  and (.rts_evidence_campaign_ui_continuity_review.input_path | contains("campaign handoff evidence JSON"))
  and (.rts_evidence_campaign_ui_continuity_review.evidence_path | contains("campaign_ui_continuity_review"))
  and (.rts_evidence_campaign_ui_continuity_review.source_of_truth | contains("RTS evidence crate reviews campaign handoff"))
  and .rts_evidence_campaign_ui_continuity_review_gate == true
  and .capture_frame_count == 16
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_outdoor"
  and .final_route_director_task_id == "task-fixture-first-route"
  and .final_route_director_next_room_id == null
  and .final_open_world_handoff_state == "resumed:league-coliseum"
  and .final_contextual_primary_action_label == "COMBAT:attack"
  and (.final_contextual_action_labels | index("COMBAT:attack") != null)
  and (.final_active_task_ids | index("task-fixture-first-route") != null)
  and .final_objective_status == "open_world_after_action_ready"
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
  and .campaign_continuity_pixel_counts.player_first_campaign_view_non_background > 600000
  and .campaign_continuity_pixel_counts.player_first_campaign_view_frame > 10000
  and .campaign_continuity_pixel_counts.player_first_campaign_status_strip > 8000
  and .campaign_continuity_pixel_counts.player_first_campaign_route_rail > 100000
  and .handoff_green_gate == true
  and .preview_resolution_gate == true
  and .live_input_gate == true
  and .milestone_gate == true
  and .map_ui_state_gate == true
  and .restored_ui_state_gate == true
  and .persistence_gate == true
  and .render_readability_gate == true
  and .native_client_boundary_gate == true
  and .player_first_campaign_continuity_screen_gate == true
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_UI_CONTINUITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
