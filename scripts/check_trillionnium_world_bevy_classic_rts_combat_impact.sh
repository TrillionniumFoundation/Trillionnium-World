#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-impact.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-impact.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-combat-impact "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_combat_impact_loop_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.impact_event == "impact:hit_flash")] | length) == 1
  and ([.stage_summaries[] | select(.impact_event == "impact:stagger")] | length) == 1
  and ([.stage_summaries[] | select(.impact_event == "impact:damage_tick")] | length) == 1
  and ([.stage_summaries[] | select(.impact_event == "impact:death_fall")] | length) == 1
  and ([.stage_summaries[] | select(.impact_event == "impact:corpse_dissolve")] | length) == 1
  and ([.stage_summaries[] | select(.impact_event == "impact:victory_settle")] | length) == 1
  and .hit_pixel_count > 120
  and .stagger_pixel_count > 100
  and .damage_pixel_count > 90
  and .death_pixel_count > 100
  and .corpse_pixel_count > 80
  and .dissolve_pixel_count > 80
  and .victory_pixel_count > 100
  and .hit_gate == true
  and .stagger_gate == true
  and .damage_gate == true
  and .death_gate == true
  and .corpse_gate == true
  and .dissolve_gate == true
  and .victory_gate == true
  and .impact_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMBAT_IMPACT_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
