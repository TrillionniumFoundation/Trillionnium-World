#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-transition.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-transition.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-npc-transition "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_npc_transition_blend_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.transition_event == "transition:alert_turn")] | length) == 1
  and ([.stage_summaries[] | select(.transition_event == "transition:patrol_engage")] | length) == 1
  and ([.stage_summaries[] | select(.transition_event == "transition:work_carry")] | length) == 1
  and ([.stage_summaries[] | select(.transition_event == "transition:stalk_pounce")] | length) == 1
  and ([.stage_summaries[] | select(.transition_event == "transition:hit_recover")] | length) == 1
  and ([.stage_summaries[] | select(.transition_event == "transition:retreat_resume")] | length) == 1
  and .alert_pixel_count > 100
  and .engage_pixel_count > 120
  and .pickup_pixel_count > 100
  and .pounce_pixel_count > 110
  and .recover_pixel_count > 100
  and .resume_pixel_count > 90
  and .alert_gate == true
  and .engage_gate == true
  and .pickup_gate == true
  and .pounce_gate == true
  and .recover_gate == true
  and .resume_gate == true
  and .transition_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NPC_TRANSITION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
