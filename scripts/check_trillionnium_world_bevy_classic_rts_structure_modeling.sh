#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-structure-modeling.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-structure-modeling.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-structure-modeling "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_structure_modeling_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.structure_event) | index("structure:foundation_shadow") != null)
  and (.stage_summaries | map(.structure_event) | index("structure:scaffold") != null)
  and (.stage_summaries | map(.structure_event) | index("structure:construction_spark") != null)
  and (.stage_summaries | map(.structure_event) | index("structure:production_glow") != null)
  and (.stage_summaries | map(.structure_event) | index("structure:damage_crack") != null)
  and (.stage_summaries | map(.structure_event) | index("structure:repair_beam") != null)
  and .foundation_shadow_pixel_count > 220
  and .scaffold_pixel_count > 300
  and .construction_spark_pixel_count > 120
  and .production_glow_pixel_count > 120
  and .damage_crack_pixel_count > 120
  and .repair_beam_pixel_count > 160
  and .foundation_gate == true
  and .scaffold_gate == true
  and .construction_spark_gate == true
  and .production_glow_gate == true
  and .damage_crack_gate == true
  and .repair_beam_gate == true
  and .structure_stage_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_STRUCTURE_MODELING_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
