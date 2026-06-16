#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-tier-two-siege-push "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_tier_two_siege_push_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 1800
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_tier_two_siege_push_input)"
  and .input_action_count == 24
  and .accepted_input_count == 24
  and (.action_labels | index("RTS:QUEUE:expansion:defend:counter_wave@8,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:upgrade:siege_harness@relay_foundry") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:train:stonebreak_cart@relay_foundry") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:enemy_fortify:gate_bulwark@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:tier2:push:gate_bulwark@10,3") != null)
  and .final_expansion_defense_state == "defended:counter_wave"
  and .final_expansion_income_per_minute >= 220
  and (.final_expansion_structure_ids | index("relay_outpost") != null)
  and (.final_tier_two_tech_ids | index("relay_foundry") != null)
  and (.final_tier_two_upgrade_ids | index("siege_harness") != null)
  and (.final_siege_unit_ids | index("stonebreak_cart") != null)
  and (.final_siege_push_route_tile_ids | length) >= 6
  and (.final_siege_push_route_tile_ids | index("10,3") != null)
  and (.final_enemy_fortification_ids | index("gate_bulwark") != null)
  and (.final_siege_damage_log | index("stonebreak_cart:gate_bulwark:-60") != null)
  and .final_tier_two_push_state == "siege_push_ready:gate_bulwark"
  and .final_base_assault_result_state == "siege_softened:gate_bulwark"
  and .final_base_breach_percent == 62
  and (.final_enemy_structure_health_percents | index(40) != null)
  and .final_tech_state == "tier_two_online:relay_foundry@relay_outpost"
  and .final_army_production_state == "tier_two_siege_ready:stonebreak_cart"
  and (.final_ability_command_ids | index("siege_push") != null)
  and .final_active_ability_id == "siege_push"
  and (.final_command_queue | index("tier2_tech:relay_foundry@relay_outpost") != null)
  and (.final_command_queue | index("tier2_upgrade:siege_harness@relay_foundry") != null)
  and (.final_command_queue | index("tier2_train:stonebreak_cart@relay_foundry:stonebreak_cart") != null)
  and (.final_command_queue | index("tier2_enemy_fortify:gate_bulwark@10,3") != null)
  and (.final_command_queue | index("tier2_push:gate_bulwark@10,3:route=9,2>9,3>10,3>10,2>11,2>10,3") != null)
  and .non_background_pixels > 700000
  and .tier_two_tech_pixel_count > 60
  and .siege_unit_pixel_count > 60
  and .siege_route_pixel_count > 80
  and .enemy_fortification_pixel_count > 80
  and .siege_damage_pixel_count > 40
  and .live_tier_two_input_gate == true
  and .expansion_dependency_gate == true
  and .tier_two_tech_gate == true
  and .tier_two_upgrade_gate == true
  and .siege_unit_gate == true
  and .enemy_fortification_gate == true
  and .siege_push_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TIER_TWO_SIEGE_PUSH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
