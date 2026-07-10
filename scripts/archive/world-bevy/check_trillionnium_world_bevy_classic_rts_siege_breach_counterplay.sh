#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-siege-breach-counterplay "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_siege_breach_counterplay_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 2160
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_siege_breach_counterplay_input)"
  and .input_action_count == 29
  and .accepted_input_count == 29
  and (.action_labels | index("RTS:QUEUE:tier2:push:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:breach:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:enemy_repair:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:enemy_flank:ridge_sentries@9,4") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:hold:shield_line@9,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:finish:gate_bulwark@10,3") != null)
  and .final_tier_two_push_state == "siege_push_ready:gate_bulwark"
  and (.final_siege_unit_ids | index("stonebreak_cart") != null)
  and (.final_enemy_fortification_ids | index("gate_bulwark") != null)
  and .final_siege_breach_target_id == "gate_bulwark"
  and (.final_siege_breach_tile_ids | length) >= 5
  and (.final_siege_breach_tile_ids | index("10,3") != null)
  and (.final_enemy_repair_unit_ids | length) >= 2
  and (.final_enemy_flank_unit_ids | length) >= 3
  and (.final_player_hold_tile_ids | length) >= 4
  and .final_siege_breach_state == "counterplay_won:gate_bulwark"
  and (.final_siege_damage_log | index("stonebreak_cart:gate_bulwark:-18:breach_window") != null)
  and (.final_siege_damage_log | index("stonebreak_cart:gate_bulwark:-34:final_break") != null)
  and (.final_ai_response_log | index("enemy_repair:gate_bulwark:repair_adept_alpha|repair_adept_beta") != null)
  and (.final_combat_event_log | index("enemy_flank:ridge_sentries:ridge_sentry_left|ridge_sentry_right|ridge_sapper") != null)
  and (.final_commander_ability_log | index("hold:shield_line:rally_aura_screen") != null)
  and .final_enemy_structure_health_percents == [0]
  and .final_base_breach_percent == 100
  and .final_base_assault_result_state == "breached:gate_bulwark"
  and .final_match_result_state == "siege_breakthrough:inner_lane"
  and (.final_base_assault_reward_log | index("siege_breakthrough:+180xp:+120gold") != null)
  and (.final_next_action_ids | index("enter_inner_lane") != null)
  and .final_active_ability_id == "rally_aura"
  and .final_defeat_risk_percent >= 36
  and (.final_command_queue | index("tier2_breach:gate_bulwark@10,3:tiles=9,3>10,3>10,2>11,2>10,3") != null)
  and (.final_command_queue | index("tier2_enemy_repair:gate_bulwark@10,3") != null)
  and (.final_command_queue | index("tier2_enemy_flank:ridge_sentries@9,4") != null)
  and (.final_command_queue | index("tier2_hold:shield_line@9,3:8,3|9,3|9,4|10,3") != null)
  and (.final_command_queue | index("tier2_finish:gate_bulwark@10,3:breach=100") != null)
  and .non_background_pixels > 800000
  and .breach_pixel_count > 80
  and .repair_pixel_count > 35
  and .flank_pixel_count > 35
  and .hold_pixel_count > 50
  and .resolution_pixel_count > 30
  and .live_siege_breach_input_gate == true
  and .tier_two_dependency_gate == true
  and .breach_window_gate == true
  and .repair_reaction_gate == true
  and .flank_pressure_gate == true
  and .hold_line_gate == true
  and .resolution_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SIEGE_BREACH_COUNTERPLAY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
