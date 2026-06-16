#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-screen-for-screen-ui-replication "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication_v1"
  and .status == "classic_rts_openra_screen_for_screen_ui_replication_green"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .preview_format == "ppm_p3_rgb"
  and .screen_for_screen_mode == "openra_style_widget_root_screen_set_and_interaction_surface_replication_original_trillionnium_art"
  and .runtime_screen_mode == "player_runtime_openra_style_ingame_screen_set"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .openra_widget_root_count == 4
  and ([.openra_widget_roots[]] | index("ShellmapRoot=MAINMENU") != null)
  and ([.openra_widget_roots[]] | index("IngameRoot=INGAME_ROOT") != null)
  and ([.openra_widget_roots[]] | index("GameSaveLoadingRoot=GAMESAVE_LOADING_SCREEN") != null)
  and ([.openra_widget_roots[]] | index("EditorRoot=EDITOR_ROOT") != null)
  and .openra_reference_screen_count == 8
  and ([.openra_reference_screens[]] | index("MAINMENU_shellmap_root") != null)
  and ([.openra_reference_screens[]] | index("SKIRMISH_mission_browser") != null)
  and ([.openra_reference_screens[]] | index("MULTIPLAYER_server_browser") != null)
  and ([.openra_reference_screens[]] | index("LOBBY_setup_room") != null)
  and ([.openra_reference_screens[]] | index("LOADING_briefing_progress") != null)
  and ([.openra_reference_screens[]] | index("INGAME_ROOT_sidebar_hud") != null)
  and ([.openra_reference_screens[]] | index("PAUSE_options_overlay") != null)
  and ([.openra_reference_screens[]] | index("POSTGAME_statistics") != null)
  and .replicated_interaction_surface_count == 8
  and .source_contracts.full_game_visual_ui_replication == "trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1"
  and .source_contracts.openra_like_core == "trillionnium_world_bevy_classic_rts_openra_like_core_v1"
  and .source_contracts.openra_parity_lane == "trillionnium_world_bevy_classic_rts_openra_parity_lane_v1"
  and .source_headline.full_game_surface_count == 18
  and .source_headline.full_game_internal_claimed == true
  and .source_headline.openra_like_runtime_model == "rust_bevy_owned_openra_like_rts_core"
  and .source_headline.openra_parity_lane_axis_count == 6
  and .source_contract_gate == true
  and .source_green_gate == true
  and .openra_runtime_vocabulary_gate == true
  and .widget_root_reference_gate == true
  and .screen_set_gate == true
  and .source_screen_chain_gate == true
  and .preview_gate == true
  and .no_asset_copy_boundary_gate == true
  and .player_first_openra_style_ingame_screen_gate == true
  and .openra_style_ui_screen_set_replication_gate == true
  and .openra_screen_for_screen_ui_replication_gate == true
  and .openra_style_widget_root_screen_set_claimed == true
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_screen_for_screen_ui_replication_claimed == false
  and .pixel_counts.non_background > 1200000
  and .pixel_counts.mainmenu > 8000
  and .pixel_counts.skirmish > 8000
  and .pixel_counts.server_browser > 8000
  and .pixel_counts.lobby > 8000
  and .pixel_counts.loading > 8000
  and .pixel_counts.ingame > 8000
  and .pixel_counts.pause > 8000
  and .pixel_counts.postgame_stats > 8000
  and .pixel_counts.active_highlight > 6000
  and .openra_style_ingame_pixel_counts.player_first_openra_style_ingame_view_non_background > 70000
  and .openra_style_ingame_pixel_counts.player_first_openra_style_ingame_sidebar_non_background > 30000
  and .openra_style_ingame_pixel_counts.player_first_openra_style_ingame_command_lane_non_background > 5000
  and .openra_style_ingame_pixel_counts.player_first_openra_style_ingame_control_color > 30000
  and .openra_style_ingame_pixel_counts.player_first_openra_style_active_highlight > 6000
  and .openra_pixel_perfect_asset_parity_claimed == false
  and .openra_engine_port_claimed == false
  and .openra_asset_copied == false
  and .warcraft_iii_asset_copied == false
  and .third_party_asset_copied == false
  and .bevy_openra_runtime_parity_claimed == false
  and .bevy_openra_replay_file_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_SCREEN_FOR_SCREEN_UI_REPLICATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
