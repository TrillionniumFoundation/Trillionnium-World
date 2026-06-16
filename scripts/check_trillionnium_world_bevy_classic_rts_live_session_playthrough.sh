#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.ppm"
TRACE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.trace.json"
SLOT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough-slots"
mkdir -p "$(dirname "$SUMMARY")" "$SLOT_DIR"

TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
  TRNM_WORLD_BEVY_CAMPAIGN_ENTRY_SLOT_PATH="$SLOT_DIR/bevy-classic-rts-campaign-entry.snapshot.json" \
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-live-session-playthrough "$PREVIEW" >"$SUMMARY"

jq -e \
  --arg trace "$TRACE" '
  .contract_version == "trillionnium_world_bevy_classic_rts_live_session_playthrough_v1"
  and .status == "classic_rts_live_session_playthrough_green"
  and .green == true
  and .preview_width == 1600
  and .preview_height == 900
  and .preview_format == "ppm_p3_rgb"
  and .trace_path == $trace
  and .trace_write_gate == true
  and .trace_seed == "classic_rts_live_session_seed_v1"
  and .same_process_session_playthrough == true
  and .runtime_screen_mode == "player_runtime_live_session_playthrough_screen"
  and .runtime_screen_gate == true
  and .evidence_board_only == false
  and .input_path == "apply_native_first_playable_action + apply_live_native_action_with_source(classic_rts_live_session_playthrough_input)"
  and .stage_count == 6
  and .stage_ids == ["title_account", "match_setup", "in_match_hud", "command_feedback", "save_load_resume", "outcome_open_world"]
  and (.stage_summaries | length) == 6
  and [.stage_summaries[].step_id] == ["title_account", "match_setup", "in_match_hud", "command_feedback", "save_load_resume", "outcome_open_world"]
  and .top_level_action_count >= 12
  and .top_level_accepted_action_count == .top_level_action_count
  and .accepted_input_count >= 78
  and .campaign_handoff_input_count >= 70
  and .live_command_input_count == 5
  and .slot_a_bytes > 10000
  and .pixel_counts.non_background > 300000
  and .pixel_counts.title_account > 1000
  and .pixel_counts.match_setup > 1000
  and .pixel_counts.in_match_hud > 1000
  and .pixel_counts.command_feedback > 1000
  and .pixel_counts.save_load_resume > 1000
  and .pixel_counts.outcome_open_world > 1000
  and .pixel_counts.player_first_live_view_non_background > 250000
  and .pixel_counts.player_first_live_view_frame > 8000
  and .pixel_counts.player_first_live_status_strip > 10000
  and .pixel_counts.player_first_live_stage_rail > 25000
  and .final_state.current_room_id == "league-coliseum"
  and .final_state.map_scene == "arena_league_coliseum"
  and .final_state.objective_status == "open_world_after_action_ready"
  and .final_state.open_world_handoff_state == "resumed:league-coliseum"
  and .final_state.open_world_resume_room_id == "league-coliseum"
  and .final_state.contextual_primary_action_label == "COMBAT:attack"
  and .title_account_gate == true
  and .match_setup_gate == true
  and .in_match_hud_gate == true
  and .command_feedback_gate == true
  and .save_resume_gate == true
  and .outcome_open_world_gate == true
  and .same_process_trace_gate == true
  and .player_first_live_session_screen_gate == true
  and .preview_gate == true
  and .native_client_boundary_gate == true
  and .live_session_playthrough_gate == true
  and .internal_live_session_playthrough_claimed == true
  and .external_evidence_ignored_for_current_playtest_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_live_session_playthrough_v1"
  and .trace_seed == "classic_rts_live_session_seed_v1"
  and .same_process_session_playthrough == true
  and .runtime_screen_gate == true
  and .top_level_action_count >= 12
  and .top_level_accepted_action_count == .top_level_action_count
  and .accepted_input_count >= 78
  and .campaign_handoff_input_count >= 70
  and .live_command_input_count == 5
  and (.trace_events | length) >= 12
  and ([.trace_events[].action_label] | index("ACCOUNT:LOGIN") != null)
  and ([.trace_events[].action_label] | index("CAMPAIGN:START") != null)
  and ([.trace_events[].action_label] | index("RTS:SELECT:1") != null)
  and ([.trace_events[].action_label] | index("RTS:QUEUE:train:guard") != null)
  and ([.trace_events[].action_label] | index("RTS:MOVE:7,4:diamond") != null)
  and ([.trace_events[].action_label] | index("RTS:ATTACK:arena_creep_attack") != null)
  and ([.trace_events[].action_label] | index("RTS:ABILITY:focus_fire") != null)
  and ([.trace_events[].action_label] | index("SAVE:SELECTED") != null)
  and ([.trace_events[].action_label] | index("LOAD:SELECTED") != null)
  and ([.trace_events[].action_label] | index("CONTINUE:SESSION") != null)
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_league_coliseum"
  and .final_objective_status == "open_world_after_action_ready"
  and .final_open_world_handoff_state == "resumed:league-coliseum"
  and .final_open_world_resume_room_id == "league-coliseum"
  and (.final_command_queue | index("select_group_1") != null)
  and (.final_command_queue | index("move:7,4") != null)
  and (.final_command_queue | index("attack:arena_creep_attack") != null)
  and (.final_command_queue | index("ability:focus_fire") != null)
  and (.final_production_queue | index("train:guard") != null)
  and (.final_session_resume_history | index("session_resume_from:A") != null)
  and (.final_session_resume_history | index("session_resume_continued:A") != null)
' "$TRACE" >/dev/null

test -s "$PREVIEW"
test -s "$TRACE"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_SESSION_PLAYTHROUGH_GREEN %s %s %s\n' "$SUMMARY" "$PREVIEW" "$TRACE"
