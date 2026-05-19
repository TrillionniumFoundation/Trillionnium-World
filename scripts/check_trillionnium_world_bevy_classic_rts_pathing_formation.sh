#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-pathing-formation.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-pathing-formation.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-pathing-formation "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_pathing_formation_v1"
  and .green == true
  and .preview_width == 640
  and .preview_height == 360
  and .write_gate == true
  and .input_path == "apply_live_native_action_with_source(classic_rts_pathing_input)"
  and .input_action_count == 2
  and .accepted_input_count == 2
  and (.action_labels | index("RTS:SELECT:1") != null)
  and (.action_labels | index("RTS:MOVE:8,4:wedge") != null)
  and (.path_tile_ids | index("6,5") != null)
  and (.path_tile_ids | index("7,5") != null)
  and (.path_tile_ids | index("8,4") != null)
  and (.blocked_tile_ids | index("7,4") != null)
  and (.formation_slot_tile_ids | length >= 4)
  and (.command_queue | index("path:6,5>7,5>8,4") != null)
  and (.command_queue | index("blocked:7,4") != null)
  and (.command_queue | index("formation:wedge") != null)
  and .pathing_status == "detour:7,4"
  and .non_background_pixels > 120000
  and .path_tile_pixel_count > 80
  and .blocked_tile_pixel_count > 40
  and .formation_slot_pixel_count > 80
  and .selection_marker_pixel_count > 800
  and .command_marker_pixel_count > 500
  and .live_pathing_input_gate == true
  and .path_tile_gate == true
  and .blocked_tile_gate == true
  and .formation_slot_gate == true
  and .command_visual_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PATHING_FORMATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
