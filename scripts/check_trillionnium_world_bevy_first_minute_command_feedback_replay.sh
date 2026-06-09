#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-first-minute-command-feedback-replay.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-first-minute-command-feedback-replay-slots"
FIRST_RECORDING="$EVIDENCE_DIR/bevy-first-minute-command-feedback-source-recording.json"
COMMAND_RECORDING="$EVIDENCE_DIR/bevy-first-minute-command-feedback-recording.json"
PREVIEW="$EVIDENCE_DIR/bevy-first-minute-command-feedback-replay.ppm"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" first-minute-command-feedback-replay \
    "$SLOT_DIR" \
    "$FIRST_RECORDING" \
    "$COMMAND_RECORDING" \
    "$PREVIEW" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_command_feedback_replay_v1"
  and .input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1"
  and .input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .command_feedback_recording_contract == "trillionnium_world_bevy_first_minute_command_feedback_recording_v1"
  and .command_feedback_strip_contract == "trillionnium_world_bevy_classic_rts_control_group_command_feedback_strip_v1"
  and .command_feedback_lifecycle_contract == "trillionnium_world_bevy_classic_rts_control_group_command_feedback_lifecycle_v1"
  and .command_history_contract == "trillionnium_world_bevy_classic_rts_control_group_command_history_v1"
  and .command_history_prune_contract == "trillionnium_world_bevy_classic_rts_control_group_command_history_prune_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .preview_path == "'"$PREVIEW"'"
  and .renderer_path == "first_minute_input_replay+classic_draw_scene"
  and .input_path == "apply_live_native_action_with_source(first_minute_command_feedback_replay_input)"
  and .expected_slot_dir == .action_slot_dir
  and .first_minute_recording_path == "'"$FIRST_RECORDING"'"
  and .command_recording_path == "'"$COMMAND_RECORDING"'"
  and .first_minute_recording_bytes > 512
  and .command_recording_bytes > 512
  and .command_input_action_count == 7
  and .accepted_command_input_count == 7
  and (.command_recording.steps | length) == 7
  and (.replay_steps | length) == 7
  and (.stage_summaries | length) == 4
  and [.command_recording.steps[].action_label] == [
    "RTS:SELECT:26",
    "RTS:MOVE:18,31:line",
    "RTS:SELECT:27",
    "RTS:MOVE:21,25:line",
    "RTS:SELECT:28",
    "RTS:MOVE:1,31:line",
    "RTS:SELECT:26"
  ]
  and (.stage_summaries | all(.renderer_path == "classic_draw_scene"))
  and (.stage_summaries | all(.input_path == "apply_live_native_action_with_source(first_minute_command_feedback_replay_input)"))
  and (.stage_summaries | map(.stage) | index("group_26_queued") != null)
  and (.stage_summaries | map(.stage) | index("group_27_override") != null)
  and (.stage_summaries | map(.stage) | index("group_28_formation") != null)
  and (.stage_summaries | map(.stage) | index("cleared_history_bounded") != null)
  and (.replay_steps | all(.parsed_action == true))
  and (.replay_steps | all(.accepted == true))
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
  and .group_26_queued_target_tile == "18,31"
  and .group_27_canceled_target_tile == "21,25"
  and .group_27_override_final_tile_ids == ["20,30", "22,30"]
  and .group_28_formation_anchor_tile == "1,31"
  and .group_28_formation_slot_tile_ids == ["1,31", "2,31"]
  and .group_25_pruned_target_tile == "17,30"
  and .group_24_pruned_target_tile == "16,29"
  and (.history_entries | map(.group_id) | index("26") != null)
  and (.history_entries | map(.group_id) | index("27") != null)
  and (.history_entries | map(.group_id) | index("28") != null)
  and (.history_entries | map(.group_id) | index("25") == null)
  and (.pruned_history_entries | map(.group_id) | index("25") != null)
  and (.pruned_history_entries | map(.group_id) | index("24") != null)
  and .command_strip_hud_pixel_count > 900
  and .queue_pixel_count > 900
  and .cancel_pixel_count > 300
  and .final_pixel_count > 300
  and .anchor_pixel_count > 300
  and .history_frame_pixel_count > 800
  and .history_row_pixel_count > 8000
  and .history_row_pixel_count < 90000
  and .history_badge_pixel_count > 500
  and .history_age_pixel_count > 400
  and .history_retained_pixel_count > 200
  and .history_pruned_pixel_count > 150
  and .history_limit_pixel_count > 100
  and .cleared_ready_pixel_count > 300
  and .cleared_active_stale_pixel_count == 0
  and .first_minute_replay_gate == true
  and .command_recording_parse_gate == true
  and .command_action_parse_gate == true
  and .live_command_input_gate == true
  and .stage_gate == true
  and .strip_visual_gate == true
  and .lifecycle_visual_gate == true
  and .history_visual_gate == true
  and .history_prune_visual_gate == true
  and .retained_entry_gate == true
  and .pruned_entry_gate == true
  and .cleared_history_gate == true
  and .no_overflow_gate == true
  and .scene_renderer_gate == true
  and .original_art_policy_gate == true
  and .android_s5_real_device_claimed == false
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_command_feedback_recording_v1"
  and .source_input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1"
  and .source_input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .source_input_replay_green == true
  and .command_history_capacity == 3
  and .retained_history_group_ids == ["26", "27", "28"]
  and .pruned_history_group_ids == ["25", "24"]
  and (.steps | length) == 7
  and [.steps[].action_label] == [
    "RTS:SELECT:26",
    "RTS:MOVE:18,31:line",
    "RTS:SELECT:27",
    "RTS:MOVE:21,25:line",
    "RTS:SELECT:28",
    "RTS:MOVE:1,31:line",
    "RTS:SELECT:26"
  ]
  and (.steps | map(.preview_stage) | index("group_26_queued") != null)
  and (.steps | map(.preview_stage) | index("group_27_override") != null)
  and (.steps | map(.preview_stage) | index("group_28_formation") != null)
  and (.steps | map(.preview_stage) | index("cleared_history_bounded") != null)
  and .android_s5_real_device_claimed == false
' "$COMMAND_RECORDING" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_COMMAND_FEEDBACK_REPLAY_GREEN %s recording=%s preview=%s slot_dir=%s\n' "$SUMMARY" "$COMMAND_RECORDING" "$PREVIEW" "$SLOT_DIR"
