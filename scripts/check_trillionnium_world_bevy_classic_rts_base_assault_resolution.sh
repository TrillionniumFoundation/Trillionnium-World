#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-base-assault-resolution.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-base-assault-resolution.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-base-assault-resolution "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_base_assault_resolution_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_base_assault_resolution_input)"
  and .input_action_count == 9
  and .accepted_input_count == 9
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:army:supply:field_lodge@6,4") != null)
  and (.action_labels | index("RTS:QUEUE:army:train:guard_pair@training_hall") != null)
  and (.action_labels | index("RTS:QUEUE:army:train:wayfinder_pair@signal_spire") != null)
  and (.action_labels | index("RTS:QUEUE:army:rally:forward_watch@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:army:assign:control_group_3@forward_watch") != null)
  and (.action_labels | index("RTS:MOVE:10,3:siege") != null)
  and (.action_labels | index("RTS:ATTACK:enemy_barracks") != null)
  and (.action_labels | index("RTS:QUEUE:assault:breach:enemy_barracks@10,3") != null)
  and (.final_army_spawned_unit_ids | length) >= 4
  and (.final_selected_unit_ids == .final_army_spawned_unit_ids)
  and (.final_active_control_group_ids | index("3") != null)
  and (.final_base_assault_target_ids | length) >= 3
  and (.final_base_assault_path_tile_ids | length) >= 6
  and (.final_base_assault_path_tile_ids | index("10,3") != null)
  and (.final_enemy_structure_health_percents | length) >= 3
  and (.final_enemy_structure_health_percents | min) <= 18
  and .final_base_breach_percent == 100
  and .final_base_assault_result_state == "breached:enemy_barracks"
  and (.final_base_assault_reward_log | length) >= 2
  and (.final_command_queue | index("base_assault:breach:enemy_barracks@10,3") != null)
  and .non_background_pixels > 350000
  and .assault_path_pixel_count > 120
  and .breach_pixel_count > 80
  and .enemy_base_health_pixel_count > 40
  and .assault_reward_pixel_count > 8
  and .live_base_assault_input_gate == true
  and .army_dependency_gate == true
  and .assault_path_gate == true
  and .enemy_base_health_gate == true
  and .breach_resolution_gate == true
  and .reward_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BASE_ASSAULT_RESOLUTION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
