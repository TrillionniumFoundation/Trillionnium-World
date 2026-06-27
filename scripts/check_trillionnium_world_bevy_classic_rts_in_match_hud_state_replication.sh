#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.json"
SUMMARY_RAW="$SUMMARY.raw"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-in-match-hud-state-replication "$PREVIEW" >"$SUMMARY_RAW"

jq '
  .source_contract_count = (.source_contracts | keys | length)
  | .selected_unit_count = (.selected_unit_ids | length)
  | .active_control_group_count = (.active_control_group_ids | length)
  | .command_queue_count = (.command_queue | length)
  | .production_queue_count = (.production_queue | length)
  | .build_queue_count = (.build_queue | length)
  | .resource_spend_log_count = (.resource_spend_log | length)
  | .ability_command_count = (.ability_command_ids | length)
  | .combat_event_log_count = (.combat_event_log | length)
  | .visible_tile_count = (.visible_tile_ids | length)
  | .fogged_tile_count = (.fogged_tile_ids | length)
' "$SUMMARY_RAW" >"$SUMMARY"
rm -f "$SUMMARY_RAW"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1"
  and .status == "classic_rts_in_match_hud_state_replication_green"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .preview_format == "ppm_p3_rgb"
  and .source_contracts.production_ui_skin == "trillionnium_world_bevy_classic_rts_production_ui_skin_v1"
  and .source_contracts.production_interaction_polish == "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1"
  and .source_contracts.selection_minimap == "trillionnium_world_bevy_classic_rts_selection_minimap_v1"
  and .source_contracts.unit_status_portrait == "trillionnium_world_bevy_classic_rts_unit_status_portrait_v1"
  and .source_contracts.selection_command_feedback == "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1"
  and .source_contracts.ability_tooltip_telegraph == "trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1"
  and .source_contracts.camera_minimap_sync == "trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1"
  and .source_contracts.command_queue_path_preview == "trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1"
  and .source_contracts.full_screen_ui_replication == "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1"
  and .source_contracts.match_setup_ui_replication == "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1"
  and .source_contracts.campaign_outcome_ui_readiness == "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  and .runtime_screen_mode == "player_runtime_in_match_hud_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .runtime_screen_layout.tactical_viewport == "single in-match First Contact Basin tactical viewport"
  and .runtime_screen_layout.top_resource_strip == "top resources, supply, and pressure readout"
  and .runtime_screen_layout.right_production_ability_rail == "production, build, ability, cooldown, and alert panels"
  and .runtime_screen_layout.bottom_command_grid == "move/train/build/attack command grid and queue"
  and .hud_surface_count == 8
  and .source_contract_count == (.source_contracts | keys | length)
  and .selected_unit_count == (.selected_unit_ids | length)
  and .active_control_group_count == (.active_control_group_ids | length)
  and .command_queue_count == (.command_queue | length)
  and .production_queue_count == (.production_queue | length)
  and .build_queue_count == (.build_queue | length)
  and .resource_spend_log_count == (.resource_spend_log | length)
  and .ability_command_count == (.ability_command_ids | length)
  and .combat_event_log_count == (.combat_event_log | length)
  and .visible_tile_count == (.visible_tile_ids | length)
  and .fogged_tile_count == (.fogged_tile_ids | length)
  and (.hud_surface_names | index("RESOURCES") != null)
  and (.hud_surface_names | index("SELECTION") != null)
  and (.hud_surface_names | index("COMMAND GRID") != null)
  and (.hud_surface_names | index("MINIMAP") != null)
  and (.hud_surface_names | index("PRODUCTION") != null)
  and (.hud_surface_names | index("ABILITIES") != null)
  and (.hud_surface_names | index("COMBAT ALERTS") != null)
  and (.hud_surface_names | index("OBJECTIVE") != null)
  and (.selected_unit_ids | length == 4)
  and (.selected_unit_ids | index("trnm.worker") != null)
  and (.selected_unit_ids | index("trnm.forge.warden") != null)
  and (.active_control_group_ids | index("1") != null)
  and (.command_queue | index("move:16,9") != null)
  and (.command_queue | index("train:trnm.worker") != null)
  and (.command_queue | index("build:trnm.flux.relay") != null)
  and (.command_queue | index("attack:trnm.flux.beacon") != null)
  and (.production_queue | length >= 3)
  and (.build_queue | length >= 2)
  and (.resource_spend_log | length >= 2)
  and (.ability_command_ids | length >= 6)
  and (.combat_event_log | length >= 4)
  and (.visible_tile_ids | length >= 7)
  and (.fogged_tile_ids | length >= 6)
  and .training_progress_percent >= 70
  and .build_progress_percent >= 50
  and .target_health_percent < 50
  and .target_armor_percent > 0
  and .visibility_percent >= 70
  and .enemy_pressure_warning_percent >= 40
  and .army_supply_used == 9
  and .army_supply_cap == 18
  and .hud_pixel_counts.non_background > 100000
  and .hud_pixel_counts.resources > 40
  and .hud_pixel_counts.selection > 40
  and .hud_pixel_counts.command_grid > 40
  and .hud_pixel_counts.minimap > 40
  and .hud_pixel_counts.production > 40
  and .hud_pixel_counts.abilities > 40
  and .hud_pixel_counts.combat_alerts > 40
  and .hud_pixel_counts.objective > 40
  and .hud_pixel_counts.highlight > 20
  and .player_first_in_match_hud_screen_gate == true
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_view_non_background > 350000
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_view_frame > 8000
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_top_status_strip > 45000
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_surface_cards > 40000
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_right_rail_non_background > 90000
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_bottom_command_lane > 60000
  and .in_match_hud_player_first_pixel_counts.player_first_in_match_hud_control_colors > 8000
  and .selection_gate == true
  and .command_gate == true
  and .resource_gate == true
  and .production_gate == true
  and .ability_gate == true
  and .combat_alert_gate == true
  and .minimap_objective_gate == true
  and .native_client_boundary_gate == true
  and .preview_gate == true
  and .runtime_screen_gate == true
  and .in_match_hud_state_replication_gate == true
  and .internal_in_match_hud_state_replication_claimed == true
  and .external_evidence_ignored_for_current_replication_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_IN_MATCH_HUD_STATE_REPLICATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
