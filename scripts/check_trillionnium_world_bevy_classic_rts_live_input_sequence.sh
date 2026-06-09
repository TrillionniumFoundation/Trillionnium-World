#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-live-input-sequence "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_live_input_sequence_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 2160
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_live_input)"
  and .input_action_count == 10
  and .accepted_input_count == 10
  and (.input_sources | index("classic_rts_live_input") != null)
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:train:guard") != null)
  and (.action_labels | index("RTS:MOVE:7,4:diamond") != null)
  and (.action_labels | index("RTS:MOVE:9,4:shift_waypoint") != null)
  and (.action_labels | index("RTS:MOVE:6,5:hold") != null)
  and (.action_labels | index("RTS:MOVE:9,4:patrol") != null)
  and (.action_labels | index("RTS:MOVE:10,3:attack_move") != null)
  and (.action_labels | index("RTS:MOVE:10,3:stop") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:focus_fire") != null)
  and .non_background_pixels > 300000
  and .selection_marker_pixel_count > 1000
  and .command_marker_pixel_count > 600
  and .production_queue_pixel_count > 1000
  and .ability_command_pixel_count > 800
  and .target_health_pixel_count > 60
  and (.final_command_queue | index("select_group_1") != null)
  and (.final_command_queue | index("move:7,4") != null)
  and (.final_command_queue | index("formation:diamond") != null)
  and (.final_command_queue | index("move:9,4") != null)
  and (.final_command_queue | index("formation:shift_waypoint") != null)
  and (.final_command_queue | index("formation:hold") != null)
  and (.final_command_queue | index("formation:patrol") != null)
  and (.final_command_queue | index("formation:attack_move") != null)
  and (.final_command_queue | index("formation:stop") != null)
  and (.final_command_queue | index("stop:10,3") != null)
  and (.final_command_queue | index("command_queue_path_preview:shift_waypoints") != null)
  and (.final_command_queue | index("command_queue_path_preview:queue_stack") != null)
  and (.final_command_queue | index("command_queue_path_preview:rally_chain") != null)
  and (.final_command_queue | index("command_queue_path_preview:attack_focus") != null)
  and (.final_command_queue | index("command_queue_path_preview:cancel_repath") != null)
  and (.final_command_queue | index("feedback:diamond@7,4") != null)
  and (.final_command_queue | index("feedback:waypoint_queued@9,4") != null)
  and (.final_command_queue | index("feedback:hold_position@6,5") != null)
  and (.final_command_queue | index("feedback:patrol_route@9,4") != null)
  and (.final_command_queue | any(startswith("feedback:attack_move@10,3:")))
  and (.final_command_queue | index("feedback:stop_hold@10,3") != null)
  and (.final_command_queue | index("attack:arena_creep_attack") != null)
  and (.final_command_queue | index("ability:focus_fire") != null)
  and .command_feedback_chip_count >= 6
  and (.final_production_queue | index("train:guard") != null)
  and .final_attack_target_id == "arena_creep_attack"
  and .final_active_ability_id == "focus_fire"
  and .final_target_health_percent < 60
  and (.final_combat_event_log | index("damage:28") != null)
  and .live_input_gate == true
  and .selection_live_gate == true
  and .production_live_gate == true
  and .move_live_gate == true
  and .waypoint_live_gate == true
  and .hold_live_gate == true
  and .patrol_live_gate == true
  and .attack_move_live_gate == true
  and .stop_live_gate == true
  and .attack_live_gate == true
  and .ability_live_gate == true
  and .command_feedback_chip_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_INPUT_SEQUENCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
