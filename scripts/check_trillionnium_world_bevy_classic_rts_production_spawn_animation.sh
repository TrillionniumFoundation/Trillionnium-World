#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-spawn-animation.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-spawn-animation.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-production-spawn-animation "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_spawn_animation_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(classic_rts_production_spawn_animation_input)"
  and .input_action_count == 6
  and .accepted_input_count == 6
  and (.stage_summaries | length) == 6
  and (.stage_summaries | map(.production_event) | index("production_spawn_anim:queue_pulse") != null)
  and (.stage_summaries | map(.production_event) | index("production_spawn_anim:training_tick") != null)
  and (.stage_summaries | map(.production_event) | index("production_spawn_anim:spawn_door") != null)
  and (.stage_summaries | map(.production_event) | index("production_spawn_anim:rally_flag") != null)
  and (.stage_summaries | map(.production_event) | index("production_spawn_anim:formation_join") != null)
  and (.stage_summaries | map(.production_event) | index("production_spawn_anim:supply_flash") != null)
  and .final_army_supply_cap >= 18
  and .final_army_supply_used >= 10
  and .final_army_supply_used <= .final_army_supply_cap
  and (.final_army_production_batch_ids | length) >= 2
  and (.final_army_spawned_unit_ids | length) >= 4
  and (.final_army_rally_tile_ids | length) >= 5
  and .final_training_progress_percent == 100
  and (.final_active_control_group_ids | index("3") != null)
  and .final_selected_unit_ids == .final_army_spawned_unit_ids
  and .queue_pulse_pixel_count > 120
  and .training_tick_pixel_count > 120
  and .spawn_door_pixel_count > 120
  and .rally_flag_pixel_count > 120
  and .formation_join_pixel_count > 120
  and .supply_flash_pixel_count > 120
  and .queue_pulse_gate == true
  and .training_tick_gate == true
  and .spawn_door_gate == true
  and .rally_flag_gate == true
  and .formation_join_gate == true
  and .supply_flash_gate == true
  and .production_stage_gate == true
  and .production_runtime_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_SPAWN_ANIMATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
