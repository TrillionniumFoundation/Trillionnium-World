#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-build-lifecycle "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_build_lifecycle_v1"
  and .green == true
  and .preview_width == 640
  and .preview_height == 360
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_build_lifecycle_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:build:watch_tower@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:complete:watch_tower@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:repair:watch_tower@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:build:scout_tower@8,4") != null)
  and (.action_labels | index("RTS:QUEUE:cancel:build:1") != null)
  and .final_structure_state == "cancelled:scout_tower@8,4"
  and (.final_build_site_tile_ids | index("7,4") != null)
  and (.final_build_site_tile_ids | index("8,4") != null)
  and .final_building_blueprint_id == "watch_tower"
  and .final_building_progress_percent == 100
  and (.final_completed_structure_ids | index("watch_tower") != null)
  and .final_repair_target_id == "watch_tower"
  and .final_repair_progress_percent >= 76
  and (.final_cancelled_structure_ids | index("scout_tower") != null)
  and (.final_refund_delta_log | index("gold:+180") != null)
  and (.final_structure_health_percents | length >= 2)
  and (.final_resource_spend_log | index("repair:-45g:-20l") != null)
  and (.final_command_queue | index("blueprint:watch_tower@7,4") != null)
  and (.final_command_queue | index("complete:watch_tower@7,4") != null)
  and (.final_command_queue | index("repair:watch_tower@7,4") != null)
  and (.final_command_queue | index("cancel:build:scout_tower@8,4") != null)
  and (.final_command_queue | index("refund:scout_tower@8,4:gold:+180") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and .rts_production_lifecycle_core_frame_order_gate == true
  and .rts_production_lifecycle_core_headless_replay_gate == true
  and (.rts_production_lifecycle_core_frame_orders | length == 6)
  and (.rts_production_lifecycle_core_frame_order_errors | length == 0)
  and .rts_production_lifecycle_core_frame_order_stream_error == null
  and (.rts_production_lifecycle_core_frame_order_stream.orders | length == 6)
  and (.rts_production_lifecycle_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.rts_production_lifecycle_core_frame_order_kind_labels | tostring == "[\"build\",\"complete\",\"repair\",\"build\",\"cancel\",\"refund\"]")
  and (.rts_production_lifecycle_core_frame_orders | any(
    .kind == "build"
    and .raw_command_label == "RTS:QUEUE:build:watch_tower@7,4"
    and .target_rule_id == "watch_tower"
    and .target_tile.x == 7
    and .target_tile.y == 4
    and .queued == true
  ))
  and (.rts_production_lifecycle_core_frame_orders | any(
    .kind == "complete"
    and .raw_command_label == "RTS:QUEUE:complete:watch_tower@7,4"
    and .target_rule_id == "watch_tower"
    and .target_tile.x == 7
    and .target_tile.y == 4
  ))
  and (.rts_production_lifecycle_core_frame_orders | any(
    .kind == "repair"
    and .raw_command_label == "RTS:QUEUE:repair:watch_tower@7,4"
    and .target_actor_id == "watch_tower"
    and .target_tile.x == 7
    and .target_tile.y == 4
  ))
  and (.rts_production_lifecycle_core_frame_orders | any(
    .kind == "cancel"
    and .raw_command_label == "RTS:QUEUE:cancel:build:1"
    and .target_rule_id == "scout_tower"
    and .target_tile.x == 8
    and .target_tile.y == 4
    and .queue_id == "cancel:build:scout_tower@8,4"
  ))
  and (.rts_production_lifecycle_core_frame_orders | any(
    .kind == "refund"
    and .raw_command_label == "RTS:QUEUE:refund:scout_tower@8,4:gold:+180"
    and .target_rule_id == "scout_tower"
    and .target_tile.x == 8
    and .target_tile.y == 4
    and .queue_id == "gold:+180"
  ))
  and .rts_production_lifecycle_core_headless_replay_error == null
  and .rts_production_lifecycle_core_headless_applied_order_count == 6
  and .rts_production_lifecycle_core_headless_actor_count == 1
  and .rts_production_lifecycle_core_headless_final_frame == 525
  and (.rts_production_lifecycle_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_production_lifecycle_core_lifecycle_order_count == 6
  and .rts_production_lifecycle_core_build_order_count == 2
  and .rts_production_lifecycle_core_complete_order_count == 1
  and .rts_production_lifecycle_core_repair_order_count == 1
  and .rts_production_lifecycle_core_cancel_order_count == 1
  and .rts_production_lifecycle_core_refund_order_count == 1
  and (.rts_production_lifecycle_core_refund_delta_labels | index("gold:+180") != null)
  and .rts_production_lifecycle_core_checkpoint.refund_order_count == 1
  and .non_background_pixels > 120000
  and .build_blueprint_pixel_count > 40
  and .build_progress_pixel_count > 20
  and .structure_complete_pixel_count > 80
  and .structure_health_pixel_count > 20
  and .repair_pixel_count > 60
  and .cancel_refund_pixel_count > 40
  and .live_build_lifecycle_input_gate == true
  and .build_placement_gate == true
  and .completion_gate == true
  and .repair_gate == true
  and .cancel_refund_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BUILD_LIFECYCLE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
