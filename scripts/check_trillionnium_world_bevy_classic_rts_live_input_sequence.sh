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
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_live_input)"
  and .input_action_count == 5
  and .accepted_input_count == 5
  and (.input_sources | index("classic_rts_live_input") != null)
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:train:guard") != null)
  and (.action_labels | index("RTS:MOVE:7,4:diamond") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:focus_fire") != null)
  and .non_background_pixels > 300000
  and .selection_marker_pixel_count > 1000
  and .command_marker_pixel_count > 600
  and .attack_feedback_pixel_count > 180
  and .production_queue_pixel_count > 1000
  and .ability_command_pixel_count > 800
  and .target_health_pixel_count > 60
  and (.final_command_queue | index("select_group_1") != null)
  and (.final_command_queue | index("move:7,4") != null)
  and (.final_command_queue | index("formation:diamond") != null)
  and (.final_command_queue | index("attack:arena_creep_attack") != null)
  and (.final_command_queue | index("ability:focus_fire") != null)
  and (.final_production_queue | index("train:guard") != null)
  and .final_attack_target_id == "arena_creep_attack"
  and .final_active_ability_id == "focus_fire"
  and .final_target_health_percent < 60
  and (.final_combat_event_log | index("damage:28") != null)
  and .live_input_gate == true
  and .selection_live_gate == true
  and .production_live_gate == true
  and .move_live_gate == true
  and .attack_live_gate == true
  and .ability_live_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_INPUT_SEQUENCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
