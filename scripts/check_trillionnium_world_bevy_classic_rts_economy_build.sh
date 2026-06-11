#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-economy-build.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-economy-build.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-economy-build "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_economy_build_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_economy_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:harvest:gold_vein") != null)
  and (.action_labels | index("RTS:QUEUE:build:watch_tower@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:train:worker") != null)
  and .final_economy_state == "building:watch_tower@7,4"
  and (.final_harvest_node_ids | index("gold_vein") != null)
  and (.final_worker_assignment_ids | length >= 2)
  and .final_dropoff_structure_id == "town_hall"
  and (.final_resource_delta_log | index("gold:+80") != null)
  and (.final_resource_delta_log | index("lumber:+20") != null)
  and (.final_build_site_tile_ids | index("7,4") != null)
  and (.final_build_site_tile_ids | index("7,5") != null)
  and (.final_build_site_tile_ids | index("8,4") != null)
  and .final_building_blueprint_id == "watch_tower"
  and .final_building_progress_percent >= 42
  and (.final_production_queue | index("train:worker") != null)
  and (.final_build_queue | index("build:watch_tower@7,4") != null)
  and (.final_command_queue | index("harvest:gold_vein->town_hall") != null)
  and (.final_command_queue | index("blueprint:watch_tower@7,4") != null)
  and .rts_core_contract == "trnm_rts_core_frame_order_v1"
  and (.rts_economy_core_frame_orders | length == 3)
  and .rts_economy_core_frame_order_kind_labels == ["harvest","build","train"]
  and (.rts_economy_core_frame_order_errors | length == 0)
  and .rts_economy_core_frame_order_stream_error == null
  and (.rts_economy_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_economy_core_headless_replay_error == null
  and (.rts_economy_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .rts_economy_core_headless_applied_order_count == 3
  and .rts_economy_core_headless_actor_count == 4
  and .rts_economy_core_headless_final_frame == 693
  and .rts_economy_core_lifecycle_order_count == 2
  and .rts_economy_core_build_order_count == 1
  and .rts_economy_core_train_order_count == 1
  and .rts_economy_core_harvest_order_count == 4
  and (.rts_economy_core_build_rule_ids | index("watch_tower") != null)
  and (.rts_economy_core_train_rule_ids | index("worker") != null)
  and .non_background_pixels > 220000
  and .harvest_node_pixel_count > 80
  and .worker_route_pixel_count > 80
  and .dropoff_pixel_count > 80
  and .build_blueprint_pixel_count > 80
  and .build_progress_pixel_count > 20
  and .production_queue_pixel_count > 1000
  and .live_economy_input_gate == true
  and .harvest_loop_gate == true
  and .build_loop_gate == true
  and .production_loop_gate == true
  and .rts_economy_core_frame_order_gate == true
  and .rts_economy_core_headless_replay_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ECONOMY_BUILD_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
