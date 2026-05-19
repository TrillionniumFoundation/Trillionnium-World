#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-central-keep-pressure "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_central_keep_pressure_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 2880
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_central_keep_pressure_input)"
  and .input_action_count == 40
  and .accepted_input_count == 40
  and (.action_labels | index("RTS:QUEUE:tier2:inner_secure:signal_core@12,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_route:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_shield:mirror_ward@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_guard:warden_line@12,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_siege:final_line@12,4") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_pressure:central_keep@13,3") != null)
  and .final_inner_objective_state == "inner_core_secured:signal_core"
  and (.final_central_keep_target_ids | index("mirror_ward") != null)
  and (.final_central_keep_target_ids | index("central_keep") != null)
  and (.final_central_keep_route_tile_ids | length) >= 5
  and (.final_central_keep_route_tile_ids | index("13,3") != null)
  and .final_keep_shield_percent == 24
  and .final_target_health_percent == 58
  and .final_target_shield_percent == 24
  and (.final_boss_guard_unit_ids | length) >= 3
  and (.final_player_siege_line_tile_ids | length) >= 4
  and .final_central_keep_state == "pressure_locked:central_keep"
  and .final_match_result_state == "central_keep_pressure:central_keep"
  and (.final_base_assault_reward_log | index("central_keep_pressure:+180xp:+90gold") != null)
  and (.final_next_action_ids | index("press_central_keep") != null)
  and (.final_next_action_ids | index("break_central_keep") != null)
  and .final_active_ability_id == "siege_push"
  and .final_defeat_risk_percent >= 42
  and (.final_siege_damage_log | index("stonebreak_cart:mirror_ward:-34:shield_crack") != null)
  and (.final_siege_damage_log | index("stonebreak_cart:central_keep:-58:pressure_lock") != null)
  and (.final_combat_event_log | index("keep_guard:warden_line:keep_warden_alpha|keep_warden_beta|ward_sentinel") != null)
  and (.final_intel_log | index("keep_shield:mirror_ward@13,3:shield=82") != null)
  and (.final_command_queue | index("tier2_keep_route:central_keep@13,3:12,3>12,4>13,4>13,3>14,3") != null)
  and (.final_command_queue | index("tier2_keep_shield:mirror_ward@13,3:shield=82") != null)
  and (.final_command_queue | index("tier2_keep_guard:warden_line@12,3") != null)
  and (.final_command_queue | index("tier2_keep_siege:final_line@12,4:11,4|12,4|13,4|12,3") != null)
  and (.final_command_queue | index("tier2_keep_pressure:central_keep@13,3:shield=24") != null)
  and .non_background_pixels > 1050000
  and .keep_route_pixel_count > 80
  and .keep_shield_pixel_count > 50
  and .keep_guard_pixel_count > 45
  and .keep_siege_line_pixel_count > 45
  and .keep_pressure_pixel_count > 35
  and .live_central_keep_input_gate == true
  and .inner_lane_dependency_gate == true
  and .keep_route_gate == true
  and .keep_shield_gate == true
  and .keep_guard_gate == true
  and .keep_siege_line_gate == true
  and .keep_pressure_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CENTRAL_KEEP_PRESSURE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
