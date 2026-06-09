#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-build-lifecycle "$PREVIEW" >"$SUMMARY"
)

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
