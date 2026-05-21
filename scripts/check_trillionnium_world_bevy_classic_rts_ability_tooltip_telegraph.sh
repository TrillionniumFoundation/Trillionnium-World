#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ability-tooltip-telegraph.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ability-tooltip-telegraph.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-ability-tooltip-telegraph "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(classic_rts_ability_tooltip_telegraph_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.telegraph_event) | index("ability_tooltip_telegraph:hover_tooltip") != null)
  and (.stage_summaries | map(.telegraph_event) | index("ability_tooltip_telegraph:range_preview") != null)
  and (.stage_summaries | map(.telegraph_event) | index("ability_tooltip_telegraph:cast_windup") != null)
  and (.stage_summaries | map(.telegraph_event) | index("ability_tooltip_telegraph:cooldown_sweep") != null)
  and (.stage_summaries | map(.telegraph_event) | index("ability_tooltip_telegraph:queue_explain") != null)
  and (.stage_summaries | map(.telegraph_event) | index("ability_tooltip_telegraph:resource_warning") != null)
  and (.final_ability_command_ids | length) >= 6
  and (.final_ability_cooldown_percents | length) >= 6
  and (.final_production_queue | length) >= 4
  and (.final_resource_spend_log | length) >= 2
  and .tooltip_pixel_count > 900
  and .range_pixel_count > 500
  and .windup_pixel_count > 600
  and .cooldown_pixel_count > 450
  and .queue_pixel_count > 700
  and .warning_pixel_count > 900
  and .tooltip_gate == true
  and .range_gate == true
  and .windup_gate == true
  and .cooldown_gate == true
  and .queue_gate == true
  and .warning_gate == true
  and .telegraph_stage_gate == true
  and .ability_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ABILITY_TOOLTIP_TELEGRAPH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
