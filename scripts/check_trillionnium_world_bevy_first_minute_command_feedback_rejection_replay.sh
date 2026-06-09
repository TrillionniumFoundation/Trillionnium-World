#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-first-minute-command-feedback-rejection-replay.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-first-minute-command-feedback-rejection-replay-slots"
FIRST_RECORDING="$EVIDENCE_DIR/bevy-first-minute-command-feedback-rejection-source-recording.json"
REJECTION_RECORDING="$EVIDENCE_DIR/bevy-first-minute-command-feedback-rejection-recording.json"
PREVIEW="$EVIDENCE_DIR/bevy-first-minute-command-feedback-rejection-replay.ppm"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" first-minute-command-feedback-rejection-replay \
    "$SLOT_DIR" \
    "$FIRST_RECORDING" \
    "$REJECTION_RECORDING" \
    "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1"
  and .input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1"
  and .input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .command_feedback_replay_contract == "trillionnium_world_bevy_first_minute_command_feedback_replay_v1"
  and .rejection_recording_contract == "trillionnium_world_bevy_first_minute_command_feedback_rejection_recording_v1"
  and .command_history_contract == "trillionnium_world_bevy_classic_rts_control_group_command_history_v1"
  and .command_history_prune_contract == "trillionnium_world_bevy_classic_rts_control_group_command_history_prune_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .preview_path == "'"$PREVIEW"'"
  and .renderer_path == "first_minute_input_replay+classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(first_minute_command_feedback_rejection_replay_input)"
  and .expected_slot_dir == .action_slot_dir
  and .first_minute_recording_path == "'"$FIRST_RECORDING"'"
  and .rejection_recording_path == "'"$REJECTION_RECORDING"'"
  and .first_minute_recording_bytes > 512
  and .rejection_recording_bytes > 512
  and .command_input_action_count == 7
  and .accepted_command_input_count == 1
  and .blocked_command_input_count == 6
  and (.rejection_recording.steps | length) == 7
  and (.replay_steps | length) == 7
  and (.stage_summaries | length) == 4
  and [.rejection_recording.steps[].action_label] == [
    "RTS:MOVE:18,31:line",
    "RTS:SELECT:26",
    "RTS:MOVE:bad-tile:line",
    "RTS:ATTACK:",
    "RTS:ABILITY:guard_break",
    "RTS:QUEUE:",
    "RTS:SELECT:"
  ]
  and .expected_blocked_reasons == [
    "rts_group_selection_required",
    "rts_invalid_tile:bad-tile",
    "rts_attack_target_required",
    "rts_attack_required_before_ability",
    "rts_queue_id_required",
    "rts_group_id_required"
  ]
  and .blocked_reasons == .expected_blocked_reasons
  and .input_telemetry_summary.blocked_reasons == .expected_blocked_reasons
  and .input_telemetry_summary.blocked_events == 6
  and (.replay_steps | all(.parsed_action == true))
  and (.replay_steps | all(.accepted_match == true and .reason_match == true))
  and (.replay_steps | map(select(.accepted == false and .command_queue_changed == true)) | length) == 6
  and (.replay_steps | map(select(.accepted == false and .executable_command_queue_changed == true)) | length) == 0
  and (.replay_steps | map(select(.accepted == true)) | length) == 1
  and .command_queue_blocked_feedback_chip_count == 6
  and (.command_queue_blocked_feedback_chips | index("feedback:blocked:move:rts_group_selection_required") != null)
  and (.command_queue_blocked_feedback_chips | index("feedback:blocked:move:rts_invalid_tile:bad-tile") != null)
  and (.command_queue_blocked_feedback_chips | index("feedback:blocked:attack:rts_attack_target_required") != null)
  and (.command_queue_blocked_feedback_chips | index("feedback:blocked:ability:rts_attack_required_before_ability") != null)
  and (.command_queue_blocked_feedback_chips | index("feedback:blocked:queue:rts_queue_id_required") != null)
  and (.command_queue_blocked_feedback_chips | index("feedback:blocked:select:rts_group_id_required") != null)
  and .blocked_feedback_chip_pixel_count > 240
  and (.stage_summaries | all(.frame_blocked_feedback_chip_pixel_count > 40))
  and (.stage_summaries | all(.renderer_path == "classic_draw_scene"))
  and (.stage_summaries | all(.input_path == "apply_live_native_action_with_source(first_minute_command_feedback_rejection_replay_input)"))
  and (.stage_summaries | map(.stage) | index("group_selection_required") != null)
  and (.stage_summaries | map(.stage) | index("invalid_tile") != null)
  and (.stage_summaries | map(.stage) | index("attack_target_required") != null)
  and (.stage_summaries | map(.stage) | index("history_preserved_after_rejections") != null)
  and .first_minute_summary.green == true
  and .first_minute_summary.signature_match_gate == true
  and .first_minute_summary.final_completion_gate == true
  and .first_minute_summary.final_objective_status == "combat_resolved"
  and .history_entry_count == 3
  and .history_capacity == 3
  and .history_overflow_row_count == 0
  and .retained_history_group_ids == ["26", "27", "28"]
  and .pruned_history_group_ids == ["25", "24"]
  and .pruned_entry_count == 2
  and .stale_group_25_visible == false
  and .active_strip_lifecycle_states == ["cleared"]
  and (.history_entries | map(.group_id) | index("26") != null)
  and (.history_entries | map(.group_id) | index("27") != null)
  and (.history_entries | map(.group_id) | index("28") != null)
  and (.history_entries | map(.group_id) | index("25") == null)
  and (.pruned_history_entries | map(.group_id) | index("25") != null)
  and (.pruned_history_entries | map(.group_id) | index("24") != null)
  and .command_queue_rejection_pollution_count == 0
  and .executable_command_queue_after_rejections == .executable_command_queue_after_setup_input
  and .blocked_tile_pixel_count > 40
  and .history_frame_pixel_count > 800
  and .history_row_pixel_count > 8000
  and .history_badge_pixel_count > 500
  and .history_age_pixel_count > 400
  and .history_retained_pixel_count > 200
  and .history_pruned_pixel_count > 150
  and .history_limit_pixel_count > 100
  and .cleared_ready_pixel_count > 300
  and .cleared_active_stale_pixel_count == 0
  and .first_minute_replay_gate == true
  and .rejection_recording_parse_gate == true
  and .command_action_parse_gate == true
  and .replay_expectation_gate == true
  and .blocked_feedback_gate == true
  and .blocked_feedback_chip_gate == true
  and .accepted_setup_input_gate == true
  and .blocked_step_non_pollution_gate == true
  and .blocked_history_non_pollution_gate == true
  and .blocked_action_history_gate == true
  and .retained_entry_gate == true
  and .pruned_entry_gate == true
  and .stage_gate == true
  and .history_visual_gate == true
  and .history_prune_visual_gate == true
  and .rejection_visual_gate == true
  and .blocked_feedback_chip_visual_gate == true
  and .original_art_policy_gate == true
  and .android_s5_real_device_claimed == false
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_command_feedback_rejection_recording_v1"
  and .source_input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1"
  and .source_input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .source_input_replay_green == true
  and .command_history_capacity == 3
  and .retained_history_group_ids == ["26", "27", "28"]
  and .pruned_history_group_ids == ["25", "24"]
  and (.steps | length) == 7
  and [.steps[].action_label] == [
    "RTS:MOVE:18,31:line",
    "RTS:SELECT:26",
    "RTS:MOVE:bad-tile:line",
    "RTS:ATTACK:",
    "RTS:ABILITY:guard_break",
    "RTS:QUEUE:",
    "RTS:SELECT:"
  ]
  and [.steps[] | select(.expected_accepted == false) | .expected_reason] == [
    "rts_group_selection_required",
    "rts_invalid_tile:bad-tile",
    "rts_attack_target_required",
    "rts_attack_required_before_ability",
    "rts_queue_id_required",
    "rts_group_id_required"
  ]
  and .android_s5_real_device_claimed == false
' "$REJECTION_RECORDING" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REJECTION_REPLAY_GREEN %s recording=%s preview=%s slot_dir=%s\n' "$SUMMARY" "$REJECTION_RECORDING" "$PREVIEW" "$SLOT_DIR"
