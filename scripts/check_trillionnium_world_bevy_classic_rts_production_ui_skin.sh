#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-production-ui-skin "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_ui_skin_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .source_contracts.production_asset_atlas == "trillionnium_world_bevy_classic_rts_production_asset_atlas_v1"
  and .source_contracts.command_surface == "trillionnium_world_bevy_classic_rts_command_surface_v1"
  and .source_contracts.selection_minimap == "trillionnium_world_bevy_classic_rts_selection_minimap_v1"
  and .source_contracts.unit_status_portrait == "trillionnium_world_bevy_classic_rts_unit_status_portrait_v1"
  and .source_contracts.selection_command_feedback == "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1"
  and .source_contracts.ability_tooltip_telegraph == "trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1"
  and .source_contracts.control_group_hotkey_feedback == "trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback_v1"
  and .ui_skin_surface_count == 8
  and (.ui_skin_surface_names | index("HUD CHROME") != null)
  and (.ui_skin_surface_names | index("COMMAND GRID") != null)
  and (.ui_skin_surface_names | index("MINIMAP BEZEL") != null)
  and (.ui_skin_surface_names | index("UNIT CARD") != null)
  and (.ui_skin_surface_names | index("TOOLTIP PANEL") != null)
  and (.ui_skin_surface_names | index("FEEDBACK MARKERS") != null)
  and (.ui_skin_surface_names | index("HOTKEY STRIP") != null)
  and (.ui_skin_surface_names | index("STATUS BARS") != null)
  and (.ui_skin_replacement_slots | index("hud_panel_chrome_slot") != null)
  and (.ui_skin_replacement_slots | index("command_button_skin_slot") != null)
  and (.ui_skin_replacement_slots | index("minimap_frame_slot") != null)
  and (.ui_skin_replacement_slots | index("portrait_card_slot") != null)
  and (.ui_skin_replacement_slots | index("tooltip_panel_slot") != null)
  and (.ui_skin_replacement_slots | index("feedback_marker_slot") != null)
  and (.ui_skin_replacement_slots | index("hotkey_strip_slot") != null)
  and (.ui_skin_replacement_slots | index("status_bar_slot") != null)
  and .asset_atlas_family_count == 10
  and .asset_atlas_frame_count >= 32
  and .asset_atlas_sprite_binding_count >= 32
  and .asset_atlas_hud_icon_pixel_count > 2000
  and .command_surface_selection_frame_pixel_count > 800
  and .command_surface_ready_pixel_count > 500
  and .command_surface_queue_confirm_pixel_count > 250
  and .selection_minimap_selection_box_pixel_count > 160
  and .selection_minimap_minimap_command_pixel_count > 80
  and .unit_status_portrait_frame_pixel_count > 1200
  and .unit_status_health_bar_pixel_count > 300
  and .selection_command_feedback_ack_pixel_count > 240
  and .selection_command_feedback_error_pixel_count > 420
  and .ability_tooltip_tooltip_pixel_count > 900
  and .ability_tooltip_warning_pixel_count > 900
  and .hotkey_feedback_assign_pixel_count > 1000
  and .hotkey_feedback_ability_pixel_count > 700
  and .ui_skin_board_pixel_count > 80000
  and .hud_chrome_pixel_count > 1000
  and .command_grid_skin_pixel_count > 1000
  and .minimap_bezel_pixel_count > 1000
  and .unit_card_skin_pixel_count > 1000
  and .tooltip_skin_pixel_count > 1000
  and .feedback_marker_pixel_count > 1000
  and .hotkey_strip_pixel_count > 1000
  and .status_bar_skin_pixel_count > 1000
  and .skin_highlight_pixel_count > 3000
  and .asset_atlas_gate == true
  and .command_surface_skin_gate == true
  and .selection_minimap_skin_gate == true
  and .unit_status_skin_gate == true
  and .command_feedback_skin_gate == true
  and .tooltip_skin_gate == true
  and .hotkey_skin_gate == true
  and .production_ui_skin_preview_gate == true
  and .source_preview_gate == true
  and .no_copy_boundary_gate == true
  and .production_ui_skin_gate == true
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .final_external_bitmap_art_shipped == false
  and .production_ready_ui_shipped == false
  and .screen_for_screen_openra_ui_claimed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .gpu_upload_claimed == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_UI_SKIN_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
