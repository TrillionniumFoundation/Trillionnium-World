#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-session-state-continuity.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-session-state-continuity.ppm"
mkdir -p "$(dirname "$SUMMARY")"
SUMMARY_RAW="$(mktemp "${SUMMARY}.raw.XXXXXX")"
SUMMARY_TMP="$(mktemp "${SUMMARY}.tmp.XXXXXX")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-session-state-continuity "$PREVIEW" >"$SUMMARY_RAW"

jq '
  .source_contract_count = (.source_contracts | keys | length)
  | .source_path_count = (.source_paths | keys | length)
  | .source_headline_field_count = (.source_headline | keys | length)
  | .runtime_screen_layout_count = (.runtime_screen_layout | keys | length)
  | .state_continuity_surface_name_count = (.state_continuity_surface_names | length)
  | .state_continuity_slot_id_count = (.state_continuity_slot_ids | length)
  | .state_continuity_source_surface_count = (.state_continuity_source_surfaces | length)
  | .state_continuity_pixel_count_field_count = (.state_continuity_pixel_counts | keys | length)
  | .resume_chain_count = (.resume_chain | length)
  | .gate_count = ([
      .shell_meta_gate,
      .session_slot_confirm_gate,
      .session_load_resume_gate,
      .session_recovery_gate,
      .match_setup_gate,
      .hud_restore_gate,
      .campaign_outcome_gate,
      .campaign_continuity_gate,
      .state_continuity_chain_gate,
      .native_client_boundary_gate,
      .preview_gate,
      .player_first_session_resume_screen_gate,
      .source_preview_gate,
      .runtime_screen_gate,
      .rts_evidence_session_state_continuity_review_gate,
      .session_state_continuity_gate
    ] | length)
  | .passed_gate_count = ([
      .shell_meta_gate,
      .session_slot_confirm_gate,
      .session_load_resume_gate,
      .session_recovery_gate,
      .match_setup_gate,
      .hud_restore_gate,
      .campaign_outcome_gate,
      .campaign_continuity_gate,
      .state_continuity_chain_gate,
      .native_client_boundary_gate,
      .preview_gate,
      .player_first_session_resume_screen_gate,
      .source_preview_gate,
      .runtime_screen_gate,
      .rts_evidence_session_state_continuity_review_gate,
      .session_state_continuity_gate
    ] | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_session_state_continuity_v1"
  and .status == "classic_rts_session_state_continuity_green"
  and .green == true
  and .preview_width == 1600
  and .preview_height == 900
  and .preview_format == "ppm_p3_rgb"
  and .source_contract_count == (.source_contracts | keys | length)
  and .source_path_count == (.source_paths | keys | length)
  and .source_headline_field_count == (.source_headline | keys | length)
  and .runtime_screen_layout_count == (.runtime_screen_layout | keys | length)
  and .source_contracts.shell_meta_ui_replication == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"
  and .source_contracts.session_slot_confirm == "trillionnium_world_bevy_session_slot_confirm_v1"
  and .source_contracts.session_load_resume == "trillionnium_world_bevy_session_load_resume_v1"
  and .source_contracts.session_recovery_ui == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .source_contracts.match_setup_ui_replication == "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1"
  and .source_contracts.in_match_hud_state_replication == "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1"
  and .source_contracts.campaign_outcome_ui_readiness == "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  and .source_contracts.campaign_ui_continuity == "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1"
  and .runtime_screen_mode == "player_runtime_session_resume_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .runtime_screen_layout.resume_chain_lane == "single visible save/load/continue chain from match setup into restored play"
  and .runtime_screen_layout.primary_tactical_viewport == "large restored tactical state with save-resume rail"
  and .runtime_screen_layout.slot_resume_controls == "selected Slot A write, load lock, and continue unlock controls"
  and .runtime_screen_layout.hud_restore_panel == "restored in-match resources, selection, command, minimap, and objective state"
  and .runtime_screen_layout.outcome_resume_panel == "campaign outcome rewards and open-world league-coliseum route resume"
  and .rts_evidence_session_state_continuity_review_contract == "trnm_rts_evidence_session_state_continuity_review_v1"
  and .rts_evidence_session_state_continuity_review.green == true
  and .rts_evidence_session_state_continuity_review.shell_meta_contract == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"
  and .rts_evidence_session_state_continuity_review.session_slot_confirm_contract == "trillionnium_world_bevy_session_slot_confirm_v1"
  and .rts_evidence_session_state_continuity_review.session_load_resume_contract == "trillionnium_world_bevy_session_load_resume_v1"
  and .rts_evidence_session_state_continuity_review.session_recovery_contract == "trillionnium_world_bevy_session_recovery_ui_v1"
  and .rts_evidence_session_state_continuity_review.match_setup_contract == "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1"
  and .rts_evidence_session_state_continuity_review.hud_contract == "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1"
  and .rts_evidence_session_state_continuity_review.campaign_outcome_contract == "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  and .rts_evidence_session_state_continuity_review.campaign_continuity_contract == "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1"
  and .rts_evidence_session_state_continuity_review.surface_chain_gate == true
  and .rts_evidence_session_state_continuity_review.state_continuity_chain_gate == true
  and .rts_evidence_session_state_continuity_review.native_client_boundary_gate == true
  and .rts_evidence_session_state_continuity_review.preview_gate == true
  and .rts_evidence_session_state_continuity_review.player_first_session_resume_screen_gate == true
  and .rts_evidence_session_state_continuity_review.source_preview_gate == true
  and .rts_evidence_session_state_continuity_review.runtime_screen_gate == true
  and .rts_evidence_session_state_continuity_review.session_state_continuity_gate == true
  and .rts_evidence_session_state_continuity_review.load_resume_final_objective_status == "first_playable_loop_complete"
  and .rts_evidence_session_state_continuity_review.campaign_outcome_open_world_state == "resumed:league-coliseum"
  and .rts_evidence_session_state_continuity_review.campaign_continuity_restored_room_id == "league-coliseum"
  and (.rts_evidence_session_state_continuity_review.input_path | contains("session-state continuity source JSON"))
  and (.rts_evidence_session_state_continuity_review.evidence_path | contains("session_state_continuity_review"))
  and (.rts_evidence_session_state_continuity_review.source_of_truth | contains("RTS evidence crate reviews save-slot confirmation"))
  and .rts_evidence_session_state_continuity_review_gate == true
  and .state_continuity_surface_count == 8
  and .state_continuity_surface_name_count == (.state_continuity_surface_names | length)
  and .state_continuity_slot_id_count == (.state_continuity_slot_ids | length)
  and .state_continuity_source_surface_count == (.state_continuity_source_surfaces | length)
  and .state_continuity_pixel_count_field_count == (.state_continuity_pixel_counts | keys | length)
  and .resume_chain_count == (.resume_chain | length)
  and .gate_count == 16
  and .passed_gate_count == 16
  and .failed_gate_count == 0
  and (.state_continuity_surface_names | index("MATCH SETUP SNAPSHOT") != null)
  and (.state_continuity_surface_names | index("SESSION SLOT WRITE") != null)
  and (.state_continuity_surface_names | index("LOAD RESUME LOCK") != null)
  and (.state_continuity_surface_names | index("CONTINUE UNLOCK") != null)
  and (.state_continuity_surface_names | index("IN-MATCH HUD RESTORE") != null)
  and (.state_continuity_surface_names | index("OUTCOME REWARD STATE") != null)
  and (.state_continuity_surface_names | index("OPEN-WORLD RESUME") != null)
  and (.state_continuity_surface_names | index("RECOVERY UI GUARD") != null)
  and (.resume_chain | index("match_setup_saved") != null)
  and (.resume_chain | index("slot_a_written") != null)
  and (.resume_chain | index("load_resume_locked") != null)
  and (.resume_chain | index("continue_unlocked") != null)
  and (.resume_chain | index("in_match_hud_restored") != null)
  and (.resume_chain | index("campaign_outcome_saved") != null)
  and (.resume_chain | index("open_world_resumed") != null)
  and .state_continuity_pixel_counts.non_background > 300000
  and .state_continuity_pixel_counts.board > 100000
  and .state_continuity_pixel_counts.match_setup_snapshot > 2000
  and .state_continuity_pixel_counts.session_slot_write > 2000
  and .state_continuity_pixel_counts.load_resume_lock > 2000
  and .state_continuity_pixel_counts.continue_unlock > 2000
  and .state_continuity_pixel_counts.in_match_hud_restore > 2000
  and .state_continuity_pixel_counts.outcome_reward_state > 2000
  and .state_continuity_pixel_counts.open_world_resume > 2000
  and .state_continuity_pixel_counts.recovery_ui_guard > 2000
  and .state_continuity_pixel_counts.highlight > 1000
  and .state_continuity_pixel_counts.player_first_resume_view_non_background > 250000
  and .state_continuity_pixel_counts.player_first_resume_view_frame > 8000
  and .state_continuity_pixel_counts.player_first_resume_status_strip > 10000
  and .state_continuity_pixel_counts.player_first_resume_stage_rail > 70000
  and .source_headline.shell_meta_surface_count == 12
  and .source_headline.shell_meta_runtime_screen_mode == "player_runtime_shell_meta_screen"
  and .source_headline.match_setup_runtime_screen_mode == "player_runtime_match_setup_screen"
  and .source_headline.hud_runtime_screen_mode == "player_runtime_in_match_hud_screen"
  and .source_headline.confirmed_slot_a_bytes > 512
  and .source_headline.load_resume_slot_a_bytes > 512
  and .source_headline.load_resume_final_objective_status == "first_playable_loop_complete"
  and .source_headline.match_setup_map_id == "first_contact_basin"
  and .source_headline.hud_surface_count == 8
  and .source_headline.hud_army_supply_used == 9
  and .source_headline.campaign_outcome_open_world_state == "resumed:league-coliseum"
  and .source_headline.campaign_continuity_restored_room_id == "league-coliseum"
  and .shell_meta_gate == true
  and .session_slot_confirm_gate == true
  and .session_load_resume_gate == true
  and .session_recovery_gate == true
  and .match_setup_gate == true
  and .hud_restore_gate == true
  and .campaign_outcome_gate == true
  and .campaign_continuity_gate == true
  and .state_continuity_chain_gate == true
  and .native_client_boundary_gate == true
  and .preview_gate == true
  and .player_first_session_resume_screen_gate == true
  and .source_preview_gate == true
  and .runtime_screen_gate == true
  and .session_state_continuity_gate == true
  and .internal_session_state_continuity_claimed == true
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

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SESSION_STATE_CONTINUITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
