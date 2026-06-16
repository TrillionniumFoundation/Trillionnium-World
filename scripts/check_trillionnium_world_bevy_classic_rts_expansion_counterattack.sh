#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-expansion-counterattack "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_expansion_counterattack_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 1440
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_expansion_counterattack_input)"
  and .input_action_count == 19
  and .accepted_input_count == 19
  and (.action_labels | index("RTS:QUEUE:expansion:claim:forest_relay@9,2") != null)
  and (.action_labels | index("RTS:QUEUE:expansion:build:relay_outpost@9,2") != null)
  and (.action_labels | index("RTS:QUEUE:expansion:workers:gold_line@9,2") != null)
  and (.action_labels | index("RTS:QUEUE:expansion:defend:counter_wave@8,3") != null)
  and (.final_expansion_tile_ids | length) >= 5
  and (.final_expansion_tile_ids | index("9,2") != null)
  and (.final_expansion_structure_ids | index("relay_outpost") != null)
  and (.final_expansion_structure_ids | index("watch_lantern") != null)
  and (.final_expansion_worker_unit_ids | length) >= 3
  and .final_expansion_income_per_minute >= 220
  and (.final_expansion_resource_log | index("workers:gold_line:+220_income_per_minute") != null)
  and (.final_enemy_counterattack_unit_ids | length) >= 3
  and (.final_enemy_counterattack_route_tile_ids | length) >= 6
  and (.final_enemy_counterattack_route_tile_ids | index("8,3") != null)
  and .final_expansion_defense_state == "defended:counter_wave"
  and (.final_player_defense_structure_ids | index("watch_lantern") != null)
  and .final_objective_owner_state == "player:forest_relay"
  and .final_objective_capture_percent == 100
  and .final_structure_state == "completed:relay_outpost@9,2"
  and .final_build_progress_percent == 100
  and .final_active_ability_id == "rally_aura"
  and (.final_commander_ability_log | index("defense:rally_aura:counter_wave_held") != null)
  and (.final_next_action_ids | index("secure_expansion") != null)
  and (.final_command_queue | index("expansion_claim:forest_relay@9,2") != null)
  and (.final_command_queue | index("expansion_build:relay_outpost@9,2") != null)
  and (.final_command_queue | index("expansion_workers:gold_line@9,2:expansion_worker_alpha|expansion_worker_beta|expansion_worker_gamma") != null)
  and (.final_command_queue | index("expansion_defend:counter_wave@8,3:counter_raider_alpha|counter_raider_beta|counter_sapper") != null)
  and .non_background_pixels > 650000
  and .expansion_tile_pixel_count > 120
  and .expansion_base_pixel_count > 80
  and .expansion_worker_pixel_count > 60
  and .expansion_income_pixel_count > 40
  and .counterattack_pixel_count > 120
  and .expansion_defense_pixel_count > 40
  and .live_expansion_input_gate == true
  and .commander_dependency_gate == true
  and .expansion_claim_gate == true
  and .expansion_build_gate == true
  and .expansion_worker_income_gate == true
  and .counterattack_gate == true
  and .defense_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_EXPANSION_COUNTERATTACK_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
