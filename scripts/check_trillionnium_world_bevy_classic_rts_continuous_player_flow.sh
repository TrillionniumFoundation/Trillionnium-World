#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-continuous-player-flow "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_continuous_player_flow_v1"
  and .status == "classic_rts_continuous_player_flow_green"
  and .green == true
  and .preview_width == 1600
  and .preview_height == 900
  and .preview_format == "ppm_p3_rgb"
  and .source_contracts.shell_meta_ui_replication == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"
  and .source_contracts.match_setup_ui_replication == "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1"
  and .source_contracts.in_match_hud_state_replication == "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1"
  and .source_contracts.production_interaction_polish == "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1"
  and .source_contracts.session_state_continuity == "trillionnium_world_bevy_classic_rts_session_state_continuity_v1"
  and .source_contracts.campaign_outcome_ui_readiness == "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  and .source_contracts.campaign_ui_continuity == "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1"
  and .runtime_screen_mode == "player_runtime_continuous_player_flow_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .runtime_screen_layout.flow_lane == "single continuous local Rust/Bevy player flow from title/account through open-world return"
  and .runtime_screen_layout.primary_tactical_viewport == "large current playable tactical state with continuous flow rail"
  and .runtime_screen_layout.title_account == "title actions, account panels, save slots, recovery surfaces"
  and .runtime_screen_layout.command_feedback == "drag select, right-click move, attack lock, build ghost, queued path, scroll/minimap feedback"
  and .runtime_screen_layout.save_load_resume == "selected save slot write, load lock, continue unlock, restored HUD and objective state"
  and .runtime_screen_layout.outcome_open_world == "victory aftermath, rewards, and league-coliseum open-world handoff"
  and .continuous_player_flow_step_count == 6
  and (.continuous_player_flow_steps | map(.step_id) | index("title_account") != null)
  and (.continuous_player_flow_steps | map(.step_id) | index("match_setup") != null)
  and (.continuous_player_flow_steps | map(.step_id) | index("in_match_hud") != null)
  and (.continuous_player_flow_steps | map(.step_id) | index("command_feedback") != null)
  and (.continuous_player_flow_steps | map(.step_id) | index("save_load_resume") != null)
  and (.continuous_player_flow_steps | map(.step_id) | index("outcome_open_world") != null)
  and (.transition_sequence | index("title_account") != null)
  and (.transition_sequence | index("match_setup") != null)
  and (.transition_sequence | index("in_match_hud") != null)
  and (.transition_sequence | index("command_feedback") != null)
  and (.transition_sequence | index("save_load_resume") != null)
  and (.transition_sequence | index("outcome_open_world") != null)
  and .flow_pixel_counts.non_background > 250000
  and .flow_pixel_counts.board > 100000
  and .flow_pixel_counts.title_account > 2000
  and .flow_pixel_counts.match_setup > 2000
  and .flow_pixel_counts.in_match_hud > 2000
  and .flow_pixel_counts.command_feedback > 2000
  and .flow_pixel_counts.save_load_resume > 2000
  and .flow_pixel_counts.outcome_open_world > 2000
  and .flow_pixel_counts.lane > 500
  and .flow_pixel_counts.highlight > 1000
  and .flow_pixel_counts.player_first_flow_view_non_background > 300000
  and .flow_pixel_counts.player_first_flow_view_frame > 8000
  and .flow_pixel_counts.player_first_flow_status_strip > 10000
  and .flow_pixel_counts.player_first_flow_stage_rail > 50000
  and .source_headline.shell_meta_surface_count == 12
  and .source_headline.shell_meta_runtime_screen_mode == "player_runtime_shell_meta_screen"
  and .source_headline.match_setup_runtime_screen_mode == "player_runtime_match_setup_screen"
  and .source_headline.match_setup_map_id == "first_contact_basin"
  and .source_headline.match_setup_faction_id == "mirror_guard"
  and .source_headline.hud_runtime_screen_mode == "player_runtime_in_match_hud_screen"
  and .source_headline.hud_surface_count == 8
  and .source_headline.hud_army_supply_used == 9
  and .source_headline.interaction_runtime_screen_mode == "player_runtime_command_interaction_screen"
  and .source_headline.interaction_surface_count == 6
  and .source_headline.session_runtime_screen_mode == "player_runtime_session_resume_screen"
  and .source_headline.session_final_objective_status == "first_playable_loop_complete"
  and .source_headline.session_open_world_state == "resumed:league-coliseum"
  and .source_headline.campaign_outcome_runtime_screen_mode == "player_runtime_campaign_outcome_screen"
  and .source_headline.campaign_outcome_open_world_state == "resumed:league-coliseum"
  and .source_headline.campaign_continuity_restored_room_id == "league-coliseum"
  and .title_account_gate == true
  and .match_setup_gate == true
  and .in_match_hud_gate == true
  and .command_feedback_gate == true
  and .save_resume_gate == true
  and .outcome_open_world_gate == true
  and .continuous_player_flow_chain_gate == true
  and .source_preview_gate == true
  and .preview_gate == true
  and .player_first_continuous_flow_screen_gate == true
  and .native_client_boundary_gate == true
  and .runtime_screen_gate == true
  and .continuous_player_flow_gate == true
  and .internal_continuous_player_flow_claimed == true
  and .external_evidence_ignored_for_current_replication_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTINUOUS_PLAYER_FLOW_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
