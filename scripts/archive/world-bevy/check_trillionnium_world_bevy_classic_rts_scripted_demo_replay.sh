#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scripted-demo-replay.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scripted-demo-replay.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-scripted-demo-replay "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_scripted_demo_replay_v1"
  and .status == "classic_rts_scripted_demo_replay_green"
  and .green == true
  and .demo_id == "queue_cancel_refund_sequence"
  and .preview_width == 1800
  and .preview_height == 1620
  and .panel_width == 900
  and .panel_height == 540
  and .write_gate == true
  and .rendered_frame_count == 5
  and .stage_ids == ["drag_select_frontline", "rally_path_minimap", "watch_tower_footprint", "cancel_refund", "queued_worker_ready"]
  and .stage_status_labels == ["DEMO 1 SELECT FRONTLINE", "DEMO 2 RALLY PATH 8 4", "DEMO 3 QUEUE WATCH TOWER", "DEMO 4 CANCEL REFUND", "DEMO 5 WORKER QUEUE READY"]
  and (.stage_summaries | length) == 5
  and (.stage_summaries | all(.renderer_path == "classic_draw_scene"))
  and (.stage_summaries | all(.input_path == "apply_classic_rts_scripted_demo_stage_runtime(queue_cancel_refund_sequence)"))
  and (.stage_summaries | all(.state_gate == true))
  and (.stage_summaries[0].selected_unit_ids | length) >= 3
  and (.stage_summaries[0].selection_box_tile_ids | index("5,4") != null)
  and (.stage_summaries[1].command_destination_tile == "8,4")
  and (.stage_summaries[1].minimap_command_tile_id == "8,4")
  and (.stage_summaries[1].minimap_command_kind == "rally")
  and (.stage_summaries[1].army_rally_tile_ids | index("8,4") != null)
  and (.stage_summaries[2].build_queue | index("build:watch_tower@7,4") != null)
  and .stage_summaries[2].building_blueprint_id == "watch_tower"
  and (.stage_summaries[2].build_site_tile_ids | index("7,4") != null)
  and .stage_summaries[2].building_progress_percent == 24
  and (.stage_summaries[3].cancelled_structure_ids | index("watch_tower") != null)
  and (.stage_summaries[3].refund_delta_log | index("gold:+210") != null)
  and .stage_summaries[3].minimap_command_kind == "cancel_refund"
  and (.stage_summaries[4].production_queue | index("train:worker") != null)
  and (.stage_summaries[4].command_queue | any(contains("cancel:build:watch_tower@7,4")))
  and .stage_summaries[4].training_progress_percent == 0
  and .pixel_counts.non_background > 500000
  and .pixel_counts.selection_marker > 200
  and .pixel_counts.command_marker > 80
  and .pixel_counts.minimap_command > 20
  and .pixel_counts.build_blueprint > 20
  and .pixel_counts.cancel_refund > 20
  and .pixel_counts.production_queue > 1000
  and .sequence_frame_gate == true
  and .scripted_runtime_gate == true
  and .tactical_status_gate == true
  and .visual_feedback_gate == true
  and .preview_gate == true
  and .original_art_policy_gate == true
  and .queue_tick_paused_for_screenshot_stability == true
  and .internal_scripted_demo_replay_claimed == true
  and .external_evidence_ignored_for_current_demo_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SCRIPTED_DEMO_REPLAY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
