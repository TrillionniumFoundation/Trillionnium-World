#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-executor-failure-recovery-matrix.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-executor-failure-recovery-matrix"
MATRIX_LOG="$PREVIEW_DIR/bot-executor-failure-recovery-matrix.matrix.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-bot-executor-failure-recovery-matrix "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  . as $root |
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1"
  and .green == true
  and .bot_executor_failure_recovery_matrix_state == "bevy_executor_rejects_blocked_actions_and_recovers_without_command_queue_pollution_not_openra_runtime_bot"
  and .source_replay_action_count == 6
  and .blocked_injection_count == 6
  and .blocked_rejection_count == 6
  and .blocked_expected_reason_count == 6
  and .blocked_feedback_event_count == 6
  and .blocked_command_queue_unchanged_count == 6
  and .blocked_command_queue_sha_match_count == 6
  and (.blocked_reason_values | index("rts_queue_id_required") != null)
  and (.blocked_reason_values | index("rts_group_id_required") != null)
  and (.blocked_reason_values | index("rts_attack_required_before_ability") != null)
  and (.blocked_reason_values | index("rts_invalid_tile:bad-tile") != null)
  and (.blocked_reason_values | index("rts_attack_target_required") != null)
  and (.blocked_input_sources == ["classic_rts_bot_executor_failure_recovery_matrix_blocked_input"])
  and (.recovery_input_sources == ["classic_rts_bot_executor_failure_recovery_matrix_recovery_input"])
  and .recovery_action_count == 6
  and .recovery_accepted_action_count == 6
  and .recovery_command_marker_hit_count == 6
  and .recovery_command_delta_match_count == 6
  and .feedback_blocked_count == 6
  and .feedback_recovery_count == 6
  and .final_input_feedback_event_count == 12
  and .final_blocked_action_history_count >= 6
  and .recovery_safe_runtime_sha_match == true
  and .command_queue_sha_match == true
  and (.matrix_log | length) == 6
  and (.matrix_log | all(
    .blocked.accepted == false
    and .blocked.rejected == true
    and .blocked.expected_reason_match == true
    and .blocked.command_queue_unchanged == true
    and .blocked.command_queue_sha_match == true
    and .blocked.feedback_event_delta == 1
    and .blocked.blocked_history_delta == 1
    and .recovery.action_label_parse_gate == true
    and .recovery.accepted == true
    and .recovery.command_marker_hit == true
    and .recovery.command_delta_match == true
  ))
  and (.matrix_log[0].blocked.expected_reason == "rts_queue_id_required")
  and (.matrix_log[1].blocked.expected_reason == "rts_group_id_required")
  and (.matrix_log[2].blocked.expected_reason == "rts_attack_required_before_ability")
  and (.matrix_log[3].blocked.expected_reason == "rts_invalid_tile:bad-tile")
  and (.matrix_log[4].blocked.expected_reason == "rts_queue_id_required")
  and (.matrix_log[5].blocked.expected_reason == "rts_attack_target_required")
  and (.source_action_log_sha256 | type == "string" and length == 64)
  and (.matrix_log_sha256 | type == "string" and length == 64)
  and (.planner_live_decision_log_sha256 | type == "string" and length == 64)
  and (.planner_strategy_checksum_sha256 | type == "string" and length == 64)
  and (.source_final_runtime_sha256 | type == "string" and length == 64)
  and (.source_recovery_safe_runtime_sha256 | type == "string" and length == 64)
  and (.recovery_safe_runtime_sha256 | type == "string" and length == 64)
  and (.source_command_queue_sha256 | type == "string" and length == 64)
  and (.recovery_command_queue_sha256 | type == "string" and length == 64)
  and .source_recovery_safe_runtime_sha256 == .recovery_safe_runtime_sha256
  and .source_command_queue_sha256 == .recovery_command_queue_sha256
  and .source_multi_match_summary.variant_count == 4
  and .source_multi_match_summary.total_replay_action_count == 24
  and .source_multi_match_summary.total_accepted_action_count == 24
  and (.source_multi_match_summary.evaluation_log_sha256 | type == "string" and length == 64)
  and .final_recovery_safe_runtime_summary.faction_id == "mirror_guard"
  and .final_recovery_safe_runtime_summary.objective_capture_percent == 100
  and (.final_recovery_safe_runtime_summary.tier_two_tech_ids | index("relay_foundry") != null)
  and .final_recovery_safe_runtime_summary.tier_two_push_state == "siege_push_ready:gate_bulwark"
  and .final_recovery_safe_runtime_summary.siege_breach_state == "counterplay_won:gate_bulwark"
  and .final_recovery_safe_runtime_summary.base_breach_percent == 100
  and .final_recovery_safe_runtime_summary.match_result_state == "siege_breakthrough:inner_lane"
  and (.final_recovery_safe_runtime_summary.next_action_ids | index("enter_inner_lane") != null)
  and .non_background_pixels > 250000
  and .write_preview_gate == true
  and .source_multi_match_contract_gate == true
  and .source_multi_match_gate == true
  and .source_action_log_readback_gate == true
  and .blocked_rejection_gate == true
  and .blocked_non_pollution_gate == true
  and .recovery_acceptance_gate == true
  and .recovery_runtime_gate == true
  and .input_source_gate == true
  and .preview_gate == true
  and .matrix_log_write_gate == true
  and .matrix_log_readback_gate == true
  and .matrix_log_gate == true
  and .boundary_gate == true
  and .bot_executor_failure_recovery_matrix_gate == true
  and .bevy_bot_executor_failure_recovery_matrix_claimed == true
  and .bevy_multi_match_bot_executor_evaluation_claimed == true
  and .bevy_openra_runtime_bot_executor_claimed == false
  and .bevy_openra_live_bot_match_claimed == false
  and .bevy_openra_bot_ai_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1"
  and .source_replay_action_count == 6
  and .blocked_injection_count == 6
  and .blocked_rejection_count == 6
  and .recovery_action_count == 6
  and .recovery_accepted_action_count == 6
  and .recovery_command_delta_match_count == 6
  and .recovery_safe_runtime_sha_match == true
  and .command_queue_sha_match == true
  and (.matrix_log | length) == 6
  and (.matrix_log | all(.blocked.rejected == true and .blocked.command_queue_unchanged == true and .recovery.accepted == true and .recovery.command_delta_match == true))
' "$MATRIX_LOG" >/dev/null

test -s "$PREVIEW_DIR/bot-executor-failure-recovery-matrix.ppm"
test -s "$MATRIX_LOG"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_EXECUTOR_FAILURE_RECOVERY_MATRIX_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
