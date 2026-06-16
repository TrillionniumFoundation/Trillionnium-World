#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-behavior.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-behavior.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-npc-behavior "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_npc_behavior_loop_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.behavior_event == "behavior:guard_patrol")] | length) == 1
  and ([.stage_summaries[] | select(.behavior_event == "behavior:guard_engage")] | length) == 1
  and ([.stage_summaries[] | select(.behavior_event == "behavior:worker_work")] | length) == 1
  and ([.stage_summaries[] | select(.behavior_event == "behavior:worker_carry")] | length) == 1
  and ([.stage_summaries[] | select(.behavior_event == "behavior:creep_stalk")] | length) == 1
  and ([.stage_summaries[] | select(.behavior_event == "behavior:creep_retreat")] | length) == 1
  and .patrol_pixel_count > 70
  and .engage_pixel_count > 100
  and .work_pixel_count > 70
  and .carry_pixel_count > 70
  and .stalk_pixel_count > 70
  and .retreat_pixel_count > 70
  and .route_pixel_count > 120
  and .patrol_gate == true
  and .engage_gate == true
  and .work_gate == true
  and .carry_gate == true
  and .stalk_gate == true
  and .retreat_gate == true
  and .route_gate == true
  and .behavior_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NPC_BEHAVIOR_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
