#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-objective-victory-loop "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_objective_victory_loop_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 1080
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_objective_victory_loop_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:QUEUE:ai:skirmish_wave") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:guard_break") != null)
  and (.action_labels | index("RTS:QUEUE:objective:claim:relay_beacon@6,5") != null)
  and (.action_labels | index("RTS:QUEUE:objective:extract:relay_beacon@9,2") != null)
  and (.final_objective_tile_ids | length) >= 3
  and .final_objective_capture_percent == 100
  and .final_objective_owner_state == "player:relay_beacon"
  and .final_objective_result_state == "victory:relay_beacon_extracted"
  and .final_objective_extraction_tile_id == "9,2"
  and .final_defeat_risk_percent <= 8
  and .final_ai_pressure_percent <= 34
  and (.final_objective_score_delta_log | index("victory:+250xp:+120g") != null)
  and (.final_command_queue | index("objective_claim:relay_beacon@6,5") != null)
  and (.final_command_queue | index("extract:relay_beacon@9,2") != null)
  and (.final_command_queue | index("victory:relay_beacon") != null)
  and .non_background_pixels > 250000
  and .objective_pixel_count > 80
  and .capture_bar_pixel_count > 20
  and .victory_pixel_count > 20
  and .defeat_risk_pixel_count > 5
  and .extraction_pixel_count > 40
  and .live_objective_input_gate == true
  and .objective_marker_gate == true
  and .capture_progress_gate == true
  and .victory_resolution_gate == true
  and .defeat_pressure_gate == true
  and .extraction_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OBJECTIVE_VICTORY_LOOP_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
