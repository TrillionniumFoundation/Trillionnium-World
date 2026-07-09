#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.ppm"
mkdir -p "$(dirname "$SUMMARY")"
SUMMARY_RAW="$(mktemp "${SUMMARY}.raw.XXXXXX")"
SUMMARY_TMP="$(mktemp "${SUMMARY}.tmp.XXXXXX")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-full-game-visual-ui-replication "$PREVIEW" >"$SUMMARY_RAW"

jq '
  .source_contract_count = (.source_contracts | keys | length)
  | .source_path_count = (.source_paths | keys | length)
  | .source_review_contract_count = (.source_review_contracts | keys | length)
  | .source_review_gate_count = (.source_review_gates | keys | length)
  | .source_review_source_count = (.source_review_sources | keys | length)
  | .source_headline_field_count = (.source_headline | keys | length)
  | .single_screen_runtime_layout_count = (.single_screen_runtime_layout | keys | length)
  | .pixel_count_field_count = (.pixel_counts | keys | length)
  | .coverage_surface_name_count = (.coverage_surface_names | length)
  | .command_grid_role_id_count = (.full_game_command_grid_role_ids | length)
  | .command_grid_icon_signature_count = (.full_game_command_grid_icon_signatures | length)
  | .command_grid_state_sample_count = (.full_game_command_grid_state_samples | length)
  | .gate_count = ([
      .source_contract_gate,
      .source_green_gate,
      .runtime_screen_chain_gate,
      .runtime_screen_gate,
      .player_flow_gate,
      .coverage_surface_gate,
      .preview_gate,
      .player_first_first_contact_screen_readability_gate,
      .player_first_tactical_composition_gate,
      .full_game_command_grid_readability_gate,
      .player_first_full_game_visual_ui_screen_gate,
      .no_copy_boundary_gate,
      .rts_evidence_full_game_visual_ui_replication_review_gate,
      .full_game_visual_ui_replication_gate
    ] | length)
  | .passed_gate_count = ([
      .source_contract_gate,
      .source_green_gate,
      .runtime_screen_chain_gate,
      .runtime_screen_gate,
      .player_flow_gate,
      .coverage_surface_gate,
      .preview_gate,
      .player_first_first_contact_screen_readability_gate,
      .player_first_tactical_composition_gate,
      .full_game_command_grid_readability_gate,
      .player_first_full_game_visual_ui_screen_gate,
      .no_copy_boundary_gate,
      .rts_evidence_full_game_visual_ui_replication_review_gate,
      .full_game_visual_ui_replication_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1"
  and .status == "classic_rts_full_game_visual_ui_replication_green"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .preview_format == "ppm_p3_rgb"
  and .runtime_screen_mode == "player_runtime_full_game_visual_ui_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .source_contract_count == (.source_contracts | keys | length)
  and .source_path_count == (.source_paths | keys | length)
  and .source_review_contract_count == (.source_review_contracts | keys | length)
  and .source_review_gate_count == (.source_review_gates | keys | length)
  and .source_review_source_count == (.source_review_sources | keys | length)
  and .source_headline_field_count == (.source_headline | keys | length)
  and .single_screen_runtime_layout_count == (.single_screen_runtime_layout | keys | length)
  and .pixel_count_field_count == (.pixel_counts | keys | length)
  and .coverage_surface_count == 18
  and .coverage_surface_name_count == (.coverage_surface_names | length)
  and .command_grid_role_id_count == (.full_game_command_grid_role_ids | length)
  and .command_grid_icon_signature_count == (.full_game_command_grid_icon_signatures | length)
  and .command_grid_state_sample_count == (.full_game_command_grid_state_samples | length)
  and .gate_count == 14
  and .passed_gate_count == 14
  and .failed_gate_count == 0
  and ([.coverage_surface_names[]] | index("title_account_shell") != null)
  and ([.coverage_surface_names[]] | index("match_setup_start") != null)
  and ([.coverage_surface_names[]] | index("tactical_viewport") != null)
  and ([.coverage_surface_names[]] | index("map_minimap_camera") != null)
  and ([.coverage_surface_names[]] | index("command_grid") != null)
  and ([.coverage_surface_names[]] | index("session_slot_save_load") != null)
  and ([.coverage_surface_names[]] | index("open_world_handoff") != null)
  and .source_contracts.full_screen_ui_replication == "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1"
  and .source_contracts.shell_meta_ui_replication == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"
  and .source_contracts.match_setup_ui_replication == "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1"
  and .source_contracts.in_match_hud_state_replication == "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1"
  and .source_contracts.session_state_continuity == "trillionnium_world_bevy_classic_rts_session_state_continuity_v1"
  and .source_contracts.continuous_player_flow == "trillionnium_world_bevy_classic_rts_continuous_player_flow_v1"
  and .source_contracts.live_session_playthrough == "trillionnium_world_bevy_classic_rts_live_session_playthrough_v1"
  and .source_review_contracts.session_state_continuity == "trnm_rts_evidence_session_state_continuity_review_v1"
  and .source_review_contracts.continuous_player_flow == "trnm_rts_evidence_continuous_player_flow_review_v1"
  and .source_review_contracts.live_session_playthrough == "trnm_rts_evidence_live_session_playthrough_review_v1"
  and .source_review_gates.session_state_continuity == true
  and .source_review_gates.continuous_player_flow == true
  and .source_review_gates.live_session_playthrough == true
  and (.source_review_sources.session_state_continuity | contains("save-slot confirmation"))
  and (.source_review_sources.continuous_player_flow | contains("six-step continuous player flow"))
  and (.source_review_sources.live_session_playthrough | contains("same-process local live session playthrough"))
  and .source_contract_gate == true
  and .source_green_gate == true
  and .runtime_screen_chain_gate == true
  and .player_flow_gate == true
  and .coverage_surface_gate == true
  and .preview_gate == true
  and .player_first_first_contact_screen_readability_gate == true
  and .first_contact_art_direction == "top_down_pixel_rts"
  and .player_first_map_composition_gate == true
  and .player_first_hud_player_copy_gate == true
  and .player_first_five_second_readability_gate == true
  and .player_first_tactical_composition_gate == true
  and .first_contact_five_second_readability_checks.selected_group == true
  and .first_contact_five_second_readability_checks.beacon == true
  and .first_contact_five_second_readability_checks.route == true
  and .first_contact_five_second_readability_checks.blocked_route == true
  and .first_contact_five_second_readability_checks.next_command == true
  and .full_game_command_grid_readability_gate == true
  and .full_game_command_grid_role_ids == ["worker","scout","warden","relay","core","signal","worker","scout","warden","relay","core","signal"]
  and (.full_game_command_grid_icon_signatures | index("unit_pickaxe_ore") != null)
  and (.full_game_command_grid_icon_signatures | index("diamond_eye_crosshair") != null)
  and (.full_game_command_grid_icon_signatures | index("shield_barrier") != null)
  and (.full_game_command_grid_icon_signatures | index("mast_broadcast") != null)
  and (.full_game_command_grid_icon_signatures | index("stepped_base") != null)
  and (.full_game_command_grid_icon_signatures | index("pulse_spire") != null)
  and .full_game_command_grid_unique_icon_signature_count >= 6
  and .full_game_command_grid_active_role == "signal"
  and .full_game_command_grid_active_slot_count >= 1
  and .full_game_command_grid_sent_slot_count >= 3
  and .full_game_command_grid_available_slot_count >= 1
  and (.full_game_command_grid_state_samples | length) == 12
  and (.full_game_command_grid_state_samples[] | select(.role == "signal" and .active == true and .signature == "pulse_spire")) != null
  and .player_first_full_game_visual_ui_screen_gate == true
  and .no_copy_boundary_gate == true
  and .full_game_visual_ui_replication_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review_contract == "trnm_rts_evidence_full_game_visual_ui_replication_review_v1"
  and .rts_evidence_full_game_visual_ui_replication_review.green == true
  and .rts_evidence_full_game_visual_ui_replication_review.source_contract_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.source_green_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.source_review_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.coverage_surface_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.source_headline_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.player_flow_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.preview_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.runtime_screen_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.player_first_full_game_visual_ui_screen_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.no_copy_boundary_gate == true
  and .rts_evidence_full_game_visual_ui_replication_review.full_game_visual_ui_replication_gate == true
  and (.rts_evidence_full_game_visual_ui_replication_review.source_of_truth | contains("full-game visual/UI replication aggregate"))
  and .rts_evidence_full_game_visual_ui_replication_review_gate == true
  and .internal_rust_full_game_visual_ui_replication_claimed == true
  and .pixel_counts.non_background > 900000
  and .pixel_counts.hud_chrome > 120000
  and .pixel_counts.command > 20000
  and .pixel_counts.session > 10000
  and .pixel_counts.outcome > 10000
  and .pixel_counts.player_first_tactical_preview_non_background > 700000
  and .pixel_counts.player_first_tactical_viewport_frame > 8000
  and .pixel_counts.player_first_tactical_status_strip > 10000
  and .pixel_counts.player_first_first_contact_lane_ground > 40000
  and .pixel_counts.player_first_first_contact_hot_lane_anchor < 1500
  and .pixel_counts.player_first_first_contact_combat_flow >= 100
  and .pixel_counts.player_first_first_contact_target_rim >= 80
  and .pixel_counts.player_first_tactical_occluding_panel < 2000
  and .first_contact_art_direction_pixel_counts.terrain_regions > 80000
  and .first_contact_art_direction_pixel_counts.terrain_edge_transitions > 1500
  and .first_contact_art_direction_pixel_counts.unit_sprite_family_marks > 80
  and .first_contact_art_direction_pixel_counts.structure_sprite_family_marks > 80
  and .first_contact_art_direction_pixel_counts.blocked_route_geometry > 40
  and .first_contact_art_direction_pixel_counts.next_command_chip > 900
  and .source_headline.full_screen_surface_count == 10
  and .source_headline.shell_meta_surface_count == 12
  and .source_headline.match_setup_surface_count == 10
  and .source_headline.hud_surface_count == 8
  and .source_headline.continuous_step_count == 6
  and .source_headline.live_session_stage_count == 6
  and .source_headline.live_session_accepted_input_count >= 78
  and .source_headline.live_session_final_objective_status == "open_world_after_action_ready"
  and .source_headline.live_session_open_world_state == "resumed:league-coliseum"
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

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_GAME_VISUAL_UI_REPLICATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
