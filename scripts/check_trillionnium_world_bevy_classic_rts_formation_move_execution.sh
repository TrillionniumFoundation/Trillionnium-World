#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-execution.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-execution.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-formation-move-execution "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_formation_move_execution_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene+classic_draw_rts_formation_move_execution_overlay"
  and .input_path == "apply_live_native_action_with_source(classic_rts_formation_move_execution_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.stage) | index("slot_claim") != null)
  and (.stage_summaries | map(.stage) | index("path_reservation") != null)
  and (.stage_summaries | map(.stage) | index("stagger_step") != null)
  and (.stage_summaries | map(.stage) | index("crowd_avoidance") != null)
  and (.stage_summaries | map(.stage) | index("blocked_reroute") != null)
  and (.stage_summaries | map(.stage) | index("arrival_lock") != null)
  and (.stage_summaries | map(select(.stage == "slot_claim"))[0].slot_claims | length) >= 4
  and (.stage_summaries | map(select(.stage == "path_reservation"))[0].path_reservations | length) >= 3
  and (.stage_summaries | map(select(.stage == "stagger_step"))[0].unit_response_state == "stagger_step:line_reflow")
  and (.stage_summaries | map(select(.stage == "crowd_avoidance"))[0].group_command_state == "split_route:group_2")
  and (.stage_summaries | map(select(.stage == "blocked_reroute"))[0].blocked_tile_ids | index("7,4") != null)
  and (.stage_summaries | map(select(.stage == "arrival_lock"))[0].arrival_locked_unit_ids | length) >= 4
  and .slot_pixel_count > 600
  and .reservation_pixel_count > 500
  and .step_pixel_count > 160
  and .avoidance_pixel_count > 220
  and .reroute_pixel_count > 280
  and .arrival_pixel_count > 250
  and .live_input_gate == true
  and .slot_visual_gate == true
  and .reservation_visual_gate == true
  and .step_visual_gate == true
  and .avoidance_visual_gate == true
  and .reroute_visual_gate == true
  and .arrival_visual_gate == true
  and .execution_stage_gate == true
  and .slot_claim_gate == true
  and .path_reservation_gate == true
  and .stagger_step_gate == true
  and .crowd_avoidance_gate == true
  and .blocked_reroute_gate == true
  and .arrival_lock_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FORMATION_MOVE_EXECUTION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
