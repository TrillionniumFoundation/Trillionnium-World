#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-army-production-rally.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-army-production-rally.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-army-production-rally "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_army_production_rally_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_army_production_rally_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:army:supply:field_lodge@6,4") != null)
  and (.action_labels | index("RTS:QUEUE:army:train:guard_pair@training_hall") != null)
  and (.action_labels | index("RTS:QUEUE:army:train:wayfinder_pair@signal_spire") != null)
  and (.action_labels | index("RTS:QUEUE:army:rally:forward_watch@7,4") != null)
  and (.action_labels | index("RTS:QUEUE:army:assign:control_group_3@forward_watch") != null)
  and .final_army_supply_cap >= 18
  and .final_army_supply_used >= 10
  and .final_army_supply_used <= .final_army_supply_cap
  and (.final_army_production_batch_ids | length) >= 2
  and (.final_army_spawned_unit_ids | length) >= 4
  and (.final_army_rally_tile_ids | length) >= 5
  and (.final_army_composition_log | length) >= 5
  and .final_army_production_state == "assigned:control_group_3:group_3"
  and (.final_active_control_group_ids | index("3") != null)
  and .final_selected_unit_ids == .final_army_spawned_unit_ids
  and (.final_command_queue | index("army_supply:field_lodge@6,4") != null)
  and (.final_command_queue | index("army_train:guard_pair@training_hall") != null)
  and (.final_command_queue | index("army_train:wayfinder_pair@signal_spire") != null)
  and (.final_command_queue | index("army_rally:forward_watch@7,4") != null)
  and (.final_command_queue | index("army_assign:control_group_3@forward_watch:group_3") != null)
  and .final_training_progress_percent == 100
  and .non_background_pixels > 250000
  and .supply_pixel_count > 20
  and .spawned_unit_pixel_count > 160
  and .rally_line_pixel_count > 80
  and .composition_pixel_count > 80
  and .live_army_production_input_gate == true
  and .supply_gate == true
  and .production_batch_gate == true
  and .rally_gate == true
  and .control_group_gate == true
  and .composition_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ARMY_PRODUCTION_RALLY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
