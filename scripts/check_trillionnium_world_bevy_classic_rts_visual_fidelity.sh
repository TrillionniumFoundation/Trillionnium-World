#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-visual-fidelity.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-visual-fidelity.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-visual-fidelity "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_visual_fidelity_v1"
  and .green == true
  and .preview_width == 960
  and .preview_height == 540
  and (.selected_unit_ids | length) >= 4
  and (.selected_unit_ids | index("trnm.worker") != null)
  and (.selected_unit_ids | index("trnm.horizon.scout") != null)
  and (.selected_unit_ids | index("trnm.forge.warden") != null)
  and (.selected_unit_ids | index("trnm.flux.relay") != null)
  and (.ability_command_ids | length) >= 6
  and (.command_queue | index("move:16,9") != null)
  and (.command_queue | index("attack:trnm.flux.beacon") != null)
  and .fidelity_panel_pixel_count > 16000
  and .portrait_pixel_count > 2000
  and .model_edge_pixel_count > 1200
  and .model_highlight_pixel_count > 200
  and .command_grid_pixel_count > 1200
  and .animation_ghost_pixel_count > 200
  and .action_trail_pixel_count > 120
  and .npc_action_pixel_count > 100
  and .desktop_product_visual_alignment_gate == true
  and .basin_terrain_height_pixel_count > 450
  and (.basin_opening_action_pixel_count + .basin_tactical_viewport_pixel_count) > 300
  and .basin_unit_state_pixel_count > 350
  and .basin_combat_phase_pixel_count > 320
  and .basin_command_feedback_pixel_count > 420
  and .basin_model_identity_pixel_count > 700
  and .basin_tactical_viewport_pixel_count > 2800
  and .selected_units_gate == true
  and .command_surface_gate == true
  and .model_fidelity_gate == true
  and .npc_animation_gate == true
  and .mature_rts_hud_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_VISUAL_FIDELITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
