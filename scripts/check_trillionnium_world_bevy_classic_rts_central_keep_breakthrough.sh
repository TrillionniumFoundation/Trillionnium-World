#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-central-keep-breakthrough "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_central_keep_breakthrough_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 3240
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_central_keep_breakthrough_input)"
  and .input_action_count == 45
  and .accepted_input_count == 45
  and (.action_labels | index("RTS:QUEUE:tier2:keep_pressure:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_breach:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:guardian_counter:high_warden@13,4") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_hold:final_line@12,4") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_break:central_keep@13,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:keep_claim:central_keep@13,3") != null)
  and .final_central_keep_state == "broken:central_keep"
  and (.final_keep_breach_tile_ids | length) >= 4
  and (.final_keep_breach_tile_ids | index("13,3") != null)
  and .final_keep_breach_percent == 100
  and (.final_guardian_counter_unit_ids | length) >= 3
  and (.final_player_hold_tile_ids | length) >= 4
  and (.final_keep_claim_tile_ids | length) >= 4
  and .final_victory_banner_state == "victory:central_keep"
  and .final_central_keep_breakthrough_state == "claimed:central_keep"
  and .final_target_health_percent == 0
  and .final_target_shield_percent == 0
  and .final_enemy_structure_health_percents == [0]
  and .final_objective_owner_state == "player:central_keep"
  and .final_objective_capture_percent == 100
  and .final_match_result_state == "classic_rts_victory:central_keep"
  and (.final_next_action_ids | index("break_central_keep") != null)
  and (.final_next_action_ids | index("restore_mirror_city") != null)
  and .final_defeat_risk_percent >= 58
  and (.final_siege_damage_log | index("stonebreak_cart:central_keep:-26:breach_open") != null)
  and (.final_siege_damage_log | index("stonebreak_cart:central_keep:-32:keep_broken") != null)
  and (.final_combat_event_log | index("guardian_counter:high_warden:high_warden|ward_lancer|last_mirror_guard") != null)
  and (.final_resource_delta_log | index("field_engineer:+12armor") != null)
  and (.final_base_assault_reward_log | index("central_keep_break:+360xp:+240gold") != null)
  and (.final_base_assault_reward_log | index("mirror_city_restored:+1banner") != null)
  and (.final_command_queue | index("tier2_keep_pressure:central_keep@13,3:shield=24") != null)
  and (.final_command_queue | index("tier2_keep_breach:central_keep@13,3:13,3|13,4|14,3|14,4") != null)
  and (.final_command_queue | index("tier2_guardian_counter:high_warden@13,4") != null)
  and (.final_command_queue | index("tier2_keep_hold:final_line@12,4:11,4|12,4|13,4|12,3") != null)
  and (.final_command_queue | index("tier2_keep_break:central_keep@13,3:breach=100") != null)
  and (.final_command_queue | index("tier2_keep_claim:central_keep@13,3:capture=100") != null)
  and .non_background_pixels > 1100000
  and .keep_breach_pixel_count > 60
  and .keep_counter_pixel_count > 45
  and .keep_claim_pixel_count > 45
  and .keep_victory_pixel_count > 20
  and .live_keep_breakthrough_input_gate == true
  and .central_keep_pressure_dependency_gate == true
  and .keep_breach_gate == true
  and .guardian_counter_gate == true
  and .keep_hold_gate == true
  and .keep_break_gate == true
  and .keep_claim_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CENTRAL_KEEP_BREAKTHROUGH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
