#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-enemy-base-tech-pressure.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-enemy-base-tech-pressure.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-enemy-base-tech-pressure "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_enemy_base_tech_pressure_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:2") != null)
  and (.action_labels | index("RTS:QUEUE:recon:mark:enemy_base@10,2") != null)
  and (.action_labels | index("RTS:QUEUE:enemy:tech:shadow_lattice@enemy_barracks") != null)
  and (.action_labels | index("RTS:QUEUE:enemy:train:raider_wave@enemy_barracks") != null)
  and (.action_labels | index("RTS:QUEUE:counter:research:sentinel_lantern@signal_spire") != null)
  and (.action_labels | index("RTS:QUEUE:counter:fortify:watch_tower@7,4") != null)
  and (.final_enemy_base_tech_ids | length) >= 2
  and (.final_enemy_production_queue | length) >= 2
  and (.final_enemy_pressure_wave_unit_ids | length) >= 3
  and (.final_player_counter_tech_ids | length) >= 2
  and (.final_player_defense_structure_ids | length) >= 2
  and .final_enemy_pressure_warning_percent <= 48
  and .final_enemy_base_pressure_state == "counter_ready:enemy_base"
  and (.final_intel_log | index("marked:enemy_base@10,2") != null)
  and (.final_command_queue | index("enemy_tech:shadow_lattice@enemy_barracks") != null)
  and (.final_command_queue | index("enemy_train:raider_wave@enemy_barracks") != null)
  and (.final_command_queue | index("counter_research:sentinel_lantern@signal_spire") != null)
  and (.final_command_queue | index("counter_fortify:watch_tower@7,4") != null)
  and .non_background_pixels > 250000
  and .enemy_tech_pixel_count > 80
  and .enemy_production_pixel_count > 80
  and .player_counter_tech_pixel_count > 50
  and .defense_ready_pixel_count > 80
  and .pressure_warning_pixel_count > 20
  and .live_enemy_base_tech_pressure_input_gate == true
  and .intel_dependency_gate == true
  and .enemy_tech_gate == true
  and .enemy_production_gate == true
  and .player_counter_gate == true
  and .defense_ready_gate == true
  and .pressure_warning_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENEMY_BASE_TECH_PRESSURE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
