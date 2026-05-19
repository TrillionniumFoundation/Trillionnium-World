#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-commander-progression.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-commander-progression.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-commander-progression "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_commander_progression_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_commander_progression_input)"
  and .input_action_count == 15
  and .accepted_input_count == 15
  and (.action_labels | index("RTS:QUEUE:commander:loot:enemy_barracks@10,3") != null)
  and (.action_labels | index("RTS:QUEUE:commander:level:mirror_captain@battlefield") != null)
  and (.action_labels | index("RTS:QUEUE:commander:ability:rally_aura@mirror_captain") != null)
  and .final_commander_unit_id == "mirror_captain"
  and .final_commander_level >= 3
  and .final_commander_ability_point_count == 0
  and (.final_commander_aura_tile_ids | length) >= 5
  and (.final_commander_ability_log | length) >= 2
  and (.final_commander_ability_log | index("ability:rally_aura:aura_tiles=5") != null)
  and (.final_loot_item_ids | length) >= 3
  and (.final_loot_pickup_log | length) >= 3
  and (.final_inventory_items | index("field_banner_relic") != null)
  and .final_growth_level >= 3
  and .final_active_ability_id == "rally_aura"
  and .final_match_result_state == "victory_ready:secure_expansion"
  and (.final_next_action_ids | index("secure_expansion") != null)
  and (.final_command_queue | index("commander_loot:enemy_barracks@10,3") != null)
  and (.final_command_queue | index("commander_level:mirror_captain@battlefield") != null)
  and (.final_command_queue | index("commander_ability:rally_aura@mirror_captain") != null)
  and .non_background_pixels > 500000
  and .commander_pixel_count > 40
  and .aura_pixel_count > 80
  and .loot_pixel_count > 40
  and .ability_point_pixel_count > 20
  and .live_commander_input_gate == true
  and .aftermath_dependency_gate == true
  and .loot_gate == true
  and .commander_level_gate == true
  and .ability_point_gate == true
  and .aura_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMANDER_PROGRESSION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
