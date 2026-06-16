#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-inner-lane-breakthrough "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_inner_lane_breakthrough_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 2520
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_inner_lane_breakthrough_input)"
  and .input_action_count == 35
  and .accepted_input_count == 35
  and (.action_labels | index("RTS:QUEUE:tier2:finish:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_route:inner_lane@11,2") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_gate:inner_latch@11,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_supply:relay_convoy@9,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_split:flank_team@10,4") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_clear:second_line@11,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:inner_secure:signal_core@12,3") != null)
  and .final_siege_breach_state == "counterplay_won:gate_bulwark"
  and .final_base_assault_result_state == "breached:gate_bulwark"
  and (.final_inner_lane_tile_ids | length) >= 5
  and (.final_inner_lane_tile_ids | index("12,3") != null)
  and (.final_inner_gate_ids | index("inner_latch") != null)
  and (.final_inner_gate_ids | index("signal_lock") != null)
  and (.final_inner_defender_unit_ids | length) >= 3
  and (.final_supply_convoy_ids | length) >= 3
  and (.final_split_squad_tile_ids | length) >= 4
  and .final_inner_objective_state == "inner_core_secured:signal_core"
  and .final_enemy_structure_health_percents == [0]
  and .final_target_health_percent == 0
  and .final_match_result_state == "inner_lane_control:signal_core"
  and .final_objective_owner_state == "player:signal_core"
  and .final_objective_capture_percent == 100
  and (.final_base_assault_reward_log | index("inner_lane_control:+240xp:+160gold") != null)
  and (.final_next_action_ids | index("enter_inner_lane") != null)
  and (.final_next_action_ids | index("press_central_keep") != null)
  and (.final_active_control_group_ids | index("4") != null)
  and .final_group_command_state == "split:flank_team"
  and (.final_resource_delta_log | index("field_medic:+18hp") != null)
  and (.final_combat_event_log | index("inner_clear:second_line:inner_guard_alpha|inner_guard_beta|signal_lancer") != null)
  and (.final_intel_log | index("inner_gate:inner_latch@11,3:lock=76") != null)
  and (.final_command_queue | index("tier2_inner_route:inner_lane@11,2:10,3>11,2>11,3>12,3>12,4") != null)
  and (.final_command_queue | index("tier2_inner_gate:inner_latch@11,3") != null)
  and (.final_command_queue | index("tier2_inner_supply:relay_convoy@9,3:convoy_cart|field_medic|ammo_runner") != null)
  and (.final_command_queue | index("tier2_inner_split:flank_team@10,4:10,4|11,4|12,4|12,3") != null)
  and (.final_command_queue | index("tier2_inner_clear:second_line@11,3") != null)
  and (.final_command_queue | index("tier2_inner_secure:signal_core@12,3:capture=100") != null)
  and .non_background_pixels > 950000
  and .inner_route_pixel_count > 90
  and .inner_gate_pixel_count > 40
  and .inner_defender_pixel_count > 45
  and .inner_supply_pixel_count > 40
  and .inner_split_pixel_count > 45
  and .inner_core_pixel_count > 35
  and .live_inner_lane_input_gate == true
  and .siege_breach_dependency_gate == true
  and .inner_route_gate == true
  and .inner_gate_gate == true
  and .supply_convoy_gate == true
  and .split_squad_gate == true
  and .second_line_clear_gate == true
  and .signal_core_secure_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_INNER_LANE_BREAKTHROUGH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
