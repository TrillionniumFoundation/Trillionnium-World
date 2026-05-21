#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-local-obstruction-recovery.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-local-obstruction-recovery.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-local-obstruction-recovery "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_local_obstruction_recovery_v1"
  and .green == true
  and .preview_width == 3200
  and .preview_height == 360
  and .renderer_path == "classic_draw_scene+classic_draw_rts_local_obstruction_recovery_overlay"
  and .input_path == "apply_live_native_action_with_source(classic_rts_local_obstruction_recovery_input)"
  and .input_action_count == 5
  and .accepted_input_count == 5
  and (.stage_summaries | length) == 5
  and (.stage_summaries | map(.stage) | index("detect_block") != null)
  and (.stage_summaries | map(.stage) | index("hold_queue") != null)
  and (.stage_summaries | map(.stage) | index("side_step") != null)
  and (.stage_summaries | map(.stage) | index("gap_claim") != null)
  and (.stage_summaries | map(.stage) | index("flow_resume") != null)
  and (.stage_summaries | map(select(.stage == "detect_block"))[0].blocked_tile_ids | length) >= 2
  and (.stage_summaries | map(select(.stage == "hold_queue"))[0].queued_unit_ids | length) >= 3
  and (.stage_summaries | map(select(.stage == "side_step"))[0].side_step_unit_ids | length) >= 2
  and (.stage_summaries | map(select(.stage == "gap_claim"))[0].gap_claims | length) >= 4
  and (.stage_summaries | map(select(.stage == "flow_resume"))[0].group_route_tile_ids | length) >= 5
  and .block_pixel_count > 180
  and .queue_pixel_count > 220
  and .side_step_pixel_count > 160
  and .gap_pixel_count > 180
  and .resume_pixel_count > 160
  and .live_input_gate == true
  and .detect_block_gate == true
  and .hold_queue_gate == true
  and .side_step_gate == true
  and .gap_claim_gate == true
  and .flow_resume_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LOCAL_OBSTRUCTION_RECOVERY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
