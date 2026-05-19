#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-mirror-city-restoration.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-mirror-city-restoration.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-mirror-city-restoration "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_mirror_city_restoration_v1"
  and .green == true
  and .preview_width == 2560
  and .preview_height == 360
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_mirror_city_restoration_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.action_labels | index("RTS:QUEUE:tier2:restore_city:mirror_city@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:rebuild_core:signal_core@12,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:assign_garrison:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:victory_handoff:mirror_city@13,3") != null)
  and (.final_restored_zone_ids | length) >= 4
  and (.final_restored_zone_ids | index("central_keep") != null)
  and (.final_rebuild_structure_ids | length) >= 3
  and (.final_rebuild_structure_ids | index("signal_core") != null)
  and (.final_garrison_unit_ids | length) >= 3
  and .final_victory_handoff_state == "handoff_ready:mirror_city"
  and .final_match_result_state == "classic_rts_restored:mirror_city"
  and (.final_next_action_ids | index("restore_mirror_city") != null)
  and (.final_next_action_ids | index("open_world_after_action") != null)
  and (.final_resource_delta_log | index("mirror_city_restore:+4zones") != null)
  and (.final_resource_delta_log | index("signal_core_rebuild:+3structures") != null)
  and (.final_base_assault_reward_log | index("mirror_city_restored:+1banner") != null)
  and (.final_base_assault_reward_log | index("mirror_city_handoff:+restoration_complete") != null)
  and (.final_command_queue | index("tier2_restore_city:mirror_city@13,3:central_keep|signal_core|inner_lane|forest_relay") != null)
  and (.final_command_queue | index("tier2_rebuild_core:signal_core@12,3:signal_core|inner_latch|mirror_ward") != null)
  and (.final_command_queue | index("tier2_assign_garrison:central_keep@13,3:mirror_guard_alpha|signal_lancer|field_engineer") != null)
  and (.final_command_queue | index("tier2_victory_handoff:mirror_city@13,3:ready") != null)
  and .non_background_pixels > 200000
  and .restore_zone_pixel_count > 40
  and .rebuild_core_pixel_count > 40
  and .garrison_pixel_count > 25
  and .handoff_pixel_count > 20
  and .live_restoration_input_gate == true
  and .victory_dependency_gate == true
  and .restore_city_gate == true
  and .rebuild_core_gate == true
  and .garrison_gate == true
  and .handoff_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MIRROR_CITY_RESTORATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
