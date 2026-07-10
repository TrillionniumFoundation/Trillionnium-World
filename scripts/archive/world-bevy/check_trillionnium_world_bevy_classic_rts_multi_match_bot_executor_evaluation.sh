#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-multi-match-bot-executor-evaluation.json"
SUMMARY_RAW="$SUMMARY.raw.$$"
SUMMARY_TMP="$SUMMARY.tmp.$$"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-multi-match-bot-executor-evaluation"
EVALUATION_LOG="$PREVIEW_DIR/multi-match-bot-executor-evaluation.matches.json"
mkdir -p "$(dirname "$SUMMARY")"
trap 'rm -f "$SUMMARY_RAW" "$SUMMARY_TMP"' EXIT

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-multi-match-bot-executor-evaluation "$PREVIEW_DIR" >"$SUMMARY_RAW"

jq '
  ([.write_preview_gate, .source_replay_contract_gate, .source_replay_gate, .source_action_log_readback_gate, .variant_count_gate, .variant_diversity_gate, .multi_match_acceptance_gate, .multi_match_runtime_gate, .preview_gate, .evaluation_log_write_gate, .evaluation_log_readback_gate, .evaluation_log_gate, .boundary_gate, .multi_match_bot_executor_evaluation_gate]) as $gates
  | .variant_summary_count = ((.variant_summaries // []) | length)
  | .variant_seed_count = ((.variant_seed_values // []) | length)
  | .variant_map_count = ((.variant_map_values // []) | length)
  | .variant_economy_count = ((.variant_economy_values // []) | length)
  | .preview_path_count = ((.preview_paths // {}) | keys | length)
  | .source_replay_summary_field_count = ((.source_replay_summary // {}) | keys | length)
  | .gate_count = ($gates | length)
  | .passed_gate_count = ($gates | map(select(. == true)) | length)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$SUMMARY_RAW" >"$SUMMARY_TMP"
mv "$SUMMARY_TMP" "$SUMMARY"

jq -e '
  . as $root |
  .contract_version == "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1"
  and .green == true
  and .multi_match_bot_executor_evaluation_state == "bevy_executor_action_log_runs_across_multiple_deterministic_match_variants_not_openra_ladder"
  and .variant_count == 4
  and .accepted_variant_count == 4
  and .variant_summary_count == (.variant_summaries | length)
  and .variant_summary_count == 4
  and .variant_seed_count == (.variant_seed_values | length)
  and .variant_seed_count == 4
  and .variant_map_count == (.variant_map_values | length)
  and .variant_map_count == 4
  and .variant_economy_count == (.variant_economy_values | length)
  and .variant_economy_count == 4
  and .preview_path_count == (.preview_paths | keys | length)
  and .preview_path_count >= 2
  and .source_replay_summary_field_count == (.source_replay_summary | keys | length)
  and .gate_count == 14
  and .passed_gate_count == 14
  and .failed_gate_count == 0
  and (.variant_seed_values | length) == 4
  and (.variant_map_values | length) == 4
  and (.variant_map_values | index("forest_relay") != null)
  and (.variant_map_values | index("ridge_watch") != null)
  and (.variant_map_values | index("marsh_gate") != null)
  and (.variant_map_values | index("market_ruins") != null)
  and (.variant_economy_values | length) == 4
  and .total_replay_action_count == 24
  and .total_accepted_action_count == 24
  and .total_command_marker_hit_count == 24
  and .total_command_delta_match_count == 24
  and .runtime_sha_match_count == 4
  and .command_queue_sha_match_count == 4
  and (.variant_summaries | length) == 4
  and (.variant_summaries | all(
    .variant_gate == true
    and .replay_action_count == 6
    and .accepted_action_count == 6
    and .command_marker_hit_count == 6
    and .command_delta_match_count == 6
    and .runtime_sha_match == true
    and .command_queue_sha_match == true
    and (.input_sources == ["classic_rts_multi_match_bot_executor_evaluation_input"])
    and (.execution_log | length) == 6
    and (.execution_log | all(.action_label_parse_gate == true and .accepted == true and .command_marker_hit == true and .command_delta_match == true))
    and .final_runtime_sha256 == $root.source_final_runtime_sha256
    and .final_command_queue_sha256 == $root.source_command_queue_sha256
    and .final_runtime_summary.faction_id == "mirror_guard"
    and .final_runtime_summary.objective_capture_percent == 100
    and (.final_runtime_summary.tier_two_tech_ids | index("relay_foundry") != null)
    and .final_runtime_summary.tier_two_push_state == "siege_push_ready:gate_bulwark"
    and .final_runtime_summary.siege_breach_state == "counterplay_won:gate_bulwark"
    and .final_runtime_summary.base_breach_percent == 100
    and .final_runtime_summary.match_result_state == "siege_breakthrough:inner_lane"
    and (.final_runtime_summary.next_action_ids | index("enter_inner_lane") != null)
  ))
  and (.source_action_log_sha256 | type == "string" and length == 64)
  and (.evaluation_log_sha256 | type == "string" and length == 64)
  and (.planner_live_decision_log_sha256 | type == "string" and length == 64)
  and (.planner_strategy_checksum_sha256 | type == "string" and length == 64)
  and (.source_final_runtime_sha256 | type == "string" and length == 64)
  and (.source_command_queue_sha256 | type == "string" and length == 64)
  and .source_replay_summary.source_executor_action_count == 6
  and .source_replay_summary.replay_action_count == 6
  and .source_replay_summary.accepted_replay_action_count == 6
  and .source_replay_summary.command_delta_match_count == 6
  and (.source_replay_summary.replay_log_sha256 | type == "string" and length == 64)
  and .non_background_pixels > 250000
  and .write_preview_gate == true
  and .source_replay_contract_gate == true
  and .source_replay_gate == true
  and .source_action_log_readback_gate == true
  and .variant_count_gate == true
  and .variant_diversity_gate == true
  and .multi_match_acceptance_gate == true
  and .multi_match_runtime_gate == true
  and .preview_gate == true
  and .evaluation_log_write_gate == true
  and .evaluation_log_readback_gate == true
  and .evaluation_log_gate == true
  and .boundary_gate == true
  and .multi_match_bot_executor_evaluation_gate == true
  and .bevy_multi_match_bot_executor_evaluation_claimed == true
  and .bevy_bot_planner_executor_replay_determinism_claimed == true
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
  .contract_version == "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1"
  and .variant_count == 4
  and .accepted_variant_count == 4
  and .total_replay_action_count == 24
  and .total_accepted_action_count == 24
  and .total_command_marker_hit_count == 24
  and .total_command_delta_match_count == 24
  and .runtime_sha_match_count == 4
  and .command_queue_sha_match_count == 4
  and (.variant_summaries | length) == 4
  and (.variant_summaries | all(.variant_gate == true and (.execution_log | length) == 6))
' "$EVALUATION_LOG" >/dev/null

test -s "$PREVIEW_DIR/multi-match-bot-executor-evaluation.ppm"
test -s "$EVALUATION_LOG"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MULTI_MATCH_BOT_EXECUTOR_EVALUATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
