#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-battle-aftermath.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-battle-aftermath.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-battle-aftermath "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_battle_aftermath_v1"
  and .green == true
  and .preview_width == 2560
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_battle_aftermath_input)"
  and .input_action_count == 12
  and .accepted_input_count == 12
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:assault:breach:enemy_barracks@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:aftermath:destroy:enemy_barracks@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:aftermath:promote:control_group_3@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:aftermath:next:secure_expansion@9,2") != null)
  and (.final_army_spawned_unit_ids | length) >= 4
  and (.final_destroyed_structure_ids | index("enemy_barracks") != null)
  and (.final_debris_tile_ids | length) >= 4
  and (.final_smoke_tile_ids | length) >= 3
  and (.final_veteran_unit_ids | length) >= 3
  and (.final_veteran_level_log | length) >= 3
  and .final_growth_level >= 2
  and .final_base_breach_percent == 100
  and .final_base_assault_result_state == "destroyed:enemy_barracks"
  and .final_match_result_state == "victory_ready:secure_expansion"
  and (.final_next_action_ids | index("secure_expansion") != null)
  and .final_objective_extraction_tile_id == "9,2"
  and (.final_base_assault_reward_log | index("aftermath:+420xp:+240g") != null)
  and (.final_command_queue | index("aftermath_destroy:enemy_barracks@10,3") != null)
  and (.final_command_queue | index("aftermath_promote:control_group_3@10,3") != null)
  and (.final_command_queue | index("aftermath_next:secure_expansion@9,2") != null)
  and .non_background_pixels > 450000
  and .debris_pixel_count > 100
  and .smoke_pixel_count > 60
  and .veteran_pixel_count > 40
  and .match_result_pixel_count > 20
  and .next_action_pixel_count > 20
  and .assault_reward_pixel_count > 8
  and .live_aftermath_input_gate == true
  and .assault_dependency_gate == true
  and .destruction_gate == true
  and .veteran_gate == true
  and .match_result_gate == true
  and .next_action_gate == true
  and .reward_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BATTLE_AFTERMATH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
