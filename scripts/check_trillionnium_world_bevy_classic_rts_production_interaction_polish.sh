#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-production-interaction-polish "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .source_contracts.production_ui_skin == "trillionnium_world_bevy_classic_rts_production_ui_skin_v1"
  and .source_contracts.command_affordance == "trillionnium_world_bevy_classic_rts_command_affordance_v1"
  and .source_contracts.selection_command_feedback == "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1"
  and .source_contracts.build_lifecycle == "trillionnium_world_bevy_classic_rts_build_lifecycle_v1"
  and .source_contracts.scrollable_map == "trillionnium_world_bevy_classic_rts_scrollable_map_v1"
  and .source_contracts.command_queue_path_preview == "trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1"
  and .interaction_surface_count == 6
  and (.interaction_surface_names | index("DRAG SELECT") != null)
  and (.interaction_surface_names | index("RIGHT CLICK MOVE") != null)
  and (.interaction_surface_names | index("ATTACK LOCK") != null)
  and (.interaction_surface_names | index("BUILD GHOST") != null)
  and (.interaction_surface_names | index("QUEUE PATH") != null)
  and (.interaction_surface_names | index("SCROLL MINIMAP") != null)
  and (.interaction_replacement_slots | index("drag_marquee_skin_slot") != null)
  and (.interaction_replacement_slots | index("right_click_marker_skin_slot") != null)
  and (.interaction_replacement_slots | index("attack_cursor_skin_slot") != null)
  and (.interaction_replacement_slots | index("build_ghost_skin_slot") != null)
  and (.interaction_replacement_slots | index("queued_path_skin_slot") != null)
  and (.interaction_replacement_slots | index("scroll_minimap_skin_slot") != null)
  and .runtime_screen_mode == "player_runtime_command_interaction_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .runtime_screen_layout.drag_select == "visible marquee skin and selection feedback strip"
  and .runtime_screen_layout.right_click_move == "right-click move marker with command confirmation"
  and .runtime_screen_layout.attack_lock == "attack cursor lock, target warning, and error/ack feedback"
  and .runtime_screen_layout.build_ghost == "build placement ghost, completion, repair, cancel/refund feedback"
  and .runtime_screen_layout.queue_path == "queued waypoint path, rally chain, reservation, and cancel/repath strip"
  and .runtime_screen_layout.scroll_minimap == "edge-scroll, drag-pan, wheel zoom, minimap jump, and clamp feedback"
  and .ui_skin_runtime_screen_mode == "player_runtime_production_hud_skin_screen"
  and .ui_skin_runtime_screen_gate == true
  and .ui_skin_evidence_board_only == false
  and .ui_skin_surface_count == 8
  and .ui_skin_feedback_marker_pixel_count > 1000
  and .ui_skin_hotkey_strip_pixel_count > 1000
  and .command_affordance_drag_marquee_pixel_count > 80
  and .command_affordance_right_click_marker_pixel_count > 120
  and .command_affordance_attack_cursor_pixel_count > 120
  and .selection_feedback_ack_pixel_count > 240
  and .selection_feedback_error_pixel_count > 420
  and .build_lifecycle_blueprint_pixel_count > 40
  and .build_lifecycle_cancel_refund_pixel_count > 40
  and .scrollable_map_minimap_pixel_count > 600
  and .scrollable_map_drag_pixel_count > 250
  and .command_queue_slot_pixel_count > 1200
  and .command_queue_path_pixel_count > 400
  and .interaction_board_pixel_count > 80000
  and .drag_select_skin_pixel_count > 1000
  and .right_click_move_skin_pixel_count > 1000
  and .attack_lock_skin_pixel_count > 1000
  and .build_ghost_skin_pixel_count > 1000
  and .queue_path_skin_pixel_count > 1000
  and .scroll_minimap_skin_pixel_count > 1000
  and .hud_binding_pixel_count > 8000
  and .polish_highlight_pixel_count > 3000
  and .blocked_state_pixel_count > 600
  and .interaction_pixel_counts.player_first_command_interaction_view_non_background > 120000
  and .interaction_pixel_counts.player_first_command_interaction_view_frame > 8000
  and .interaction_pixel_counts.player_first_command_interaction_status_strip > 10000
  and .interaction_pixel_counts.player_first_command_interaction_right_rail > 50000
  and .interaction_pixel_counts.player_first_command_interaction_command_lane > 60000
  and .ui_skin_gate == true
  and .command_affordance_gate == true
  and .selection_feedback_gate == true
  and .build_lifecycle_gate == true
  and .scrollable_map_gate == true
  and .command_queue_path_gate == true
  and .production_interaction_polish_preview_gate == true
  and .player_first_command_interaction_screen_gate == true
  and .runtime_screen_gate == true
  and .source_preview_gate == true
  and .no_copy_boundary_gate == true
  and .production_interaction_polish_gate == true
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .final_external_bitmap_art_shipped == false
  and .production_ready_interaction_ui_shipped == false
  and .screen_for_screen_openra_ui_claimed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .gpu_upload_claimed == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_INTERACTION_POLISH_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
