#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

packet_json="$TMP_DIR/release-review-packet.json"
packet_md="$TMP_DIR/release-review-packet.md"
packet_log="$TMP_DIR/release-review-packet.log"
summary_json="$TMP_DIR/release-review-packet-integrity.json"
artifacts_jsonl="$TMP_DIR/artifacts.jsonl"

add_artifact_from_path() {
  local id="$1"
  local label="$2"
  local artifact_path="$3"
  local role="$4"
  local artifact_sha
  local artifact_bytes
  local contract_version=""
  local status=""

  artifact_sha="$(sha256sum "$artifact_path" | awk '{print $1}')"
  artifact_bytes="$(wc -c <"$artifact_path" | tr -d ' ')"
  if [[ "$artifact_path" == *.json ]]; then
    contract_version="$(jq -r '.contract_version // empty' "$artifact_path" 2>/dev/null || true)"
    status="$(jq -r '.status // .overall_status // empty' "$artifact_path" 2>/dev/null || true)"
  fi

  jq -nc \
    --arg id "$id" \
    --arg label "$label" \
    --arg path "$artifact_path" \
    --arg role "$role" \
    --arg sha "$artifact_sha" \
    --arg bytes "$artifact_bytes" \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    '{
      id: $id,
      label: $label,
      path: $path,
      role: $role,
      file_status: "present",
      sha256: $sha,
      bytes: ($bytes | tonumber),
      contract_version: (if $contract_version == "" then null else $contract_version end),
      status: (if $status == "" then null else $status end)
    }' >>"$artifacts_jsonl"
}

for index in $(seq 1 54); do
  artifact_path="$TMP_DIR/fixture_${index}.json"
  jq -nc \
    --arg id "fixture_${index}" \
    '{contract_version: "fixture_contract_v1", status: "fixture_green", payload: $id}' >"$artifact_path"
  add_artifact_from_path "fixture_${index}" "fixture_${index}" "$artifact_path" fixture
done

semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1",
  status: "release_review_packet_integrity_semantic_fixture_green",
  green: true,
  fixture_kind: "first_minute_command_feedback_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_first_minute_command_feedback_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 77,
  expected_semantic_failure_count: 4,
  expected_semantic_failure_names: [
    "first_minute_command_feedback_replay_semantics",
    "first_minute_command_feedback_source_recording_semantics",
    "first_minute_command_feedback_recording_semantics",
    "first_minute_command_feedback_replay_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_semantic_fixture "Release review packet integrity semantic fixture" "$semantic_fixture_json" release_review_gate

bot_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture_v1",
  status: "release_review_packet_integrity_bot_executor_semantic_fixture_green",
  green: true,
  fixture_kind: "bot_executor_source_chain_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_bot_executor_source_chain_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 77,
  expected_semantic_failure_count: 9,
  expected_semantic_failure_names: [
    "bot_planner_action_executor_semantics",
    "bot_planner_action_executor_log_semantics",
    "bot_planner_action_executor_ppm_semantics",
    "bot_planner_executor_replay_determinism_semantics",
    "bot_planner_executor_replay_determinism_log_semantics",
    "bot_planner_executor_replay_determinism_ppm_semantics",
    "multi_match_bot_executor_evaluation_semantics",
    "multi_match_bot_executor_evaluation_log_semantics",
    "multi_match_bot_executor_evaluation_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$bot_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_bot_executor_semantic_fixture "Release review packet integrity bot executor semantic fixture" "$bot_semantic_fixture_json" release_review_gate

bot_matrix_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture_v1",
  status: "release_review_packet_integrity_bot_executor_matrix_semantic_fixture_green",
  green: true,
  fixture_kind: "bot_executor_failure_recovery_matrix_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_bot_executor_failure_recovery_matrix_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 77,
  expected_semantic_failure_count: 3,
  expected_semantic_failure_names: [
    "bot_executor_failure_recovery_matrix_semantics",
    "bot_executor_failure_recovery_matrix_log_semantics",
    "bot_executor_failure_recovery_matrix_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$bot_matrix_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_bot_executor_matrix_semantic_fixture "Release review packet integrity bot executor failure/recovery matrix semantic fixture" "$bot_matrix_semantic_fixture_json" release_review_gate

replay_json="$TMP_DIR/bevy-first-minute-command-feedback-replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_replay_v1",
  green: true,
  command_input_action_count: 7,
  accepted_command_input_count: 6,
  first_minute_replay_gate: true,
  command_recording_parse_gate: true,
  live_command_input_gate: true,
  scene_renderer_gate: true,
  history_entry_count: 3,
  history_capacity: 3,
  retained_history_group_ids: ["26", "27", "25"],
  pruned_history_group_ids: ["24"],
  cleared_active_stale_pixel_count: 12,
  preview_width: 1280,
  preview_height: 720,
  android_s5_real_device_claimed: false
}' >"$replay_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$replay_json" release_review_input

source_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-source-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_timeline_contract: "trillionnium_world_bevy_first_minute_interaction_timeline_v1",
  source_timeline_green: false,
  steps: [range(0; 9) | {action_label: ("STEP:" + (tostring))}],
  android_s5_real_device_claimed: false
}' >"$source_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$source_recording_json" release_review_recording

command_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_recording_v1",
  source_input_replay_contract: "trillionnium_world_bevy_first_minute_input_replay_v1",
  source_input_recording_contract: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_input_replay_green: true,
  command_history_capacity: 3,
  retained_history_group_ids: ["26", "27", "25"],
  pruned_history_group_ids: ["24"],
  steps: [
    {action_label: "RTS:SELECT:26"},
    {action_label: "RTS:MOVE:18,31:line"},
    {action_label: "RTS:SELECT:27"},
    {action_label: "RTS:MOVE:99,99:line"},
    {action_label: "RTS:SELECT:28"},
    {action_label: "RTS:MOVE:1,31:line"},
    {action_label: "RTS:SELECT:26"}
  ],
  android_s5_real_device_claimed: false
}' >"$command_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$command_recording_json" release_review_recording

ppm_path="$TMP_DIR/bevy-first-minute-command-feedback-replay.ppm"
printf 'P3\n1279 720\n255\n' >"$ppm_path"
truncate -s 8000001 "$ppm_path"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$ppm_path" release_review_visual_evidence

rejection_replay_json="$TMP_DIR/bevy-first-minute-command-feedback-rejection-replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_rejection_replay_v1",
  green: true,
  command_input_action_count: 7,
  accepted_command_input_count: 1,
  blocked_command_input_count: 6,
  blocked_reasons: [
    "rts_group_selection_required",
    "rts_invalid_tile:bad-tile",
    "rts_attack_target_required",
    "rts_attack_required_before_ability",
    "rts_queue_id_required",
    "rts_group_id_required"
  ],
  command_queue_rejection_pollution_count: 0,
  first_minute_replay_gate: true,
  rejection_recording_parse_gate: true,
  command_action_parse_gate: true,
  replay_expectation_gate: true,
  blocked_feedback_gate: true,
  blocked_action_history_gate: true,
  blocked_history_non_pollution_gate: true,
  history_entry_count: 3,
  history_capacity: 3,
  retained_history_group_ids: ["26", "27", "28"],
  pruned_history_group_ids: ["25", "24"],
  cleared_active_stale_pixel_count: 0,
  preview_width: 1280,
  preview_height: 720,
  android_s5_real_device_claimed: false
}' >"$rejection_replay_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$rejection_replay_json" release_review_input

rejection_source_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-rejection-source-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_timeline_contract: "trillionnium_world_bevy_first_minute_interaction_timeline_v1",
  source_timeline_green: true,
  steps: [range(0; 10) | {action_label: ("STEP:" + (tostring))}],
  android_s5_real_device_claimed: false
}' >"$rejection_source_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$rejection_source_recording_json" release_review_recording

rejection_recording_json="$TMP_DIR/bevy-first-minute-command-feedback-rejection-recording.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_first_minute_command_feedback_rejection_recording_v1",
  source_input_replay_contract: "trillionnium_world_bevy_first_minute_input_replay_v1",
  source_input_recording_contract: "trillionnium_world_bevy_first_minute_input_recording_v1",
  source_input_replay_green: true,
  command_history_capacity: 3,
  retained_history_group_ids: ["26", "27", "28"],
  pruned_history_group_ids: ["25", "24"],
  steps: [
    {action_label: "RTS:MOVE:18,31:line", expected_accepted: false, expected_reason: "rts_group_selection_required"},
    {action_label: "RTS:SELECT:26", expected_accepted: true},
    {action_label: "RTS:MOVE:bad-tile:line", expected_accepted: false, expected_reason: "rts_invalid_tile:bad-tile"},
    {action_label: "RTS:ATTACK:", expected_accepted: false, expected_reason: "rts_attack_target_required"},
    {action_label: "RTS:ABILITY:guard_break", expected_accepted: false, expected_reason: "rts_attack_required_before_ability"},
    {action_label: "RTS:QUEUE:", expected_accepted: false, expected_reason: "rts_queue_id_required"},
    {action_label: "RTS:SELECT:", expected_accepted: false, expected_reason: "rts_group_id_required"}
  ],
  android_s5_real_device_claimed: false
}' >"$rejection_recording_json"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$rejection_recording_json" release_review_recording

rejection_ppm_path="$TMP_DIR/bevy-first-minute-command-feedback-rejection-replay.ppm"
printf 'P3\n1280 720\n255\n' >"$rejection_ppm_path"
truncate -s 8000001 "$rejection_ppm_path"
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$rejection_ppm_path" release_review_visual_evidence

action_log_json="$TMP_DIR/bot-planner-action-executor.actions.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1",
  executor_action_count: 6,
  accepted_action_count: 6,
  command_marker_hit_count: 6,
  input_source: "classic_rts_bot_planner_action_executor_input",
  planner_live_decision_log_sha256: "1111111111111111111111111111111111111111111111111111111111111111",
  planner_strategy_checksum_sha256: "2222222222222222222222222222222222222222222222222222222222222222",
  execution_log: [
    {accepted: true, action_label: "RTS:QUEUE:faction:mirror_guard", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:recon:sweep:watchtower_scan@7,4", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:objective:claim:relay_beacon@6,5", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:tier2:push:gate_bulwark@10,3", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"},
    {accepted: true, action_label: "RTS:QUEUE:tier2:finish:gate_bulwark@10,3", command_marker_hit: true, feedback_event_delta: 1, input_source: "classic_rts_bot_planner_action_executor_input"}
  ]
}' >"$action_log_json"
add_artifact_from_path native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$action_log_json" release_review_recording

action_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-action-executor.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_action_executor_v1",
  green: true,
  bot_planner_action_executor_gate: true,
  executor_action_count: 6,
  accepted_action_count: 6,
  command_marker_hit_count: 6,
  action_labels: [
    "RTS:QUEUE:faction:mirror_guard",
    "RTS:QUEUE:recon:sweep:watchtower_scan@7,4",
    "RTS:QUEUE:objective:claim:relay_beacon@6,5",
    "RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost",
    "RTS:QUEUE:tier2:push:gate_bulwark@10,3",
    "RTS:QUEUE:tier2:finish:gate_bulwark@10,3"
  ],
  input_sources: ["classic_rts_bot_planner_action_executor_input"],
  final_runtime_summary: {
    faction_id: "mirror_guard",
    objective_capture_percent: 100,
    tier_two_tech_ids: ["relay_foundry"],
    siege_breach_state: "counterplay_won:gate_bulwark",
    match_result_state: "siege_breakthrough:inner_lane"
  },
  bevy_bot_planner_action_executor_claimed: true,
  bevy_openra_runtime_bot_executor_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false
}' >"$action_summary_json"
add_artifact_from_path native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$action_summary_json" release_review_input

action_ppm_path="$TMP_DIR/bot-planner-action-executor.ppm"
printf 'P3\n1280 1080\n255\n' >"$action_ppm_path"
truncate -s 8000001 "$action_ppm_path"
add_artifact_from_path native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$action_ppm_path" release_review_visual_evidence

replay_log_json="$TMP_DIR/bot-planner-executor-replay-determinism.replay.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1",
  replay_action_count: 6,
  accepted_replay_action_count: 6,
  replay_command_marker_hit_count: 6,
  command_delta_match_count: 6,
  source_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  replay_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  source_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_input_source: "classic_rts_bot_planner_executor_replay_input",
  source_action_log_path: "/tmp/bot-planner-action-executor.actions.json",
  source_action_log_sha256: "5555555555555555555555555555555555555555555555555555555555555555",
  source_executor_summary_sha256: "6666666666666666666666666666666666666666666666666666666666666666",
  execution_log: [
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"},
    {accepted: true, action_label_parse_gate: true, command_marker_hit: true, command_delta_match: true, input_source: "classic_rts_bot_planner_executor_replay_input"}
  ]
}' >"$replay_log_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$replay_log_json" release_review_recording

replay_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-executor-replay-determinism.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism_v1",
  green: true,
  bot_planner_executor_replay_determinism_gate: true,
  source_executor_action_count: 6,
  replay_action_count: 6,
  accepted_replay_action_count: 6,
  replay_command_marker_hit_count: 6,
  command_delta_match_count: 6,
  runtime_determinism_gate: true,
  source_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  replay_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  source_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  replay_input_sources: ["classic_rts_bot_planner_executor_replay_input"],
  bevy_bot_planner_executor_replay_determinism_claimed: true,
  bevy_openra_runtime_bot_executor_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false
}' >"$replay_summary_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$replay_summary_json" release_review_input

replay_ppm_path="$TMP_DIR/bot-planner-executor-replay-determinism.ppm"
printf 'P3\n1280 1080\n255\n' >"$replay_ppm_path"
truncate -s 8000001 "$replay_ppm_path"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$replay_ppm_path" release_review_visual_evidence

multi_log_json="$TMP_DIR/multi-match-bot-executor-evaluation.matches.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1",
  variant_count: 4,
  accepted_variant_count: 4,
  total_replay_action_count: 24,
  total_accepted_action_count: 24,
  total_command_marker_hit_count: 24,
  total_command_delta_match_count: 24,
  runtime_sha_match_count: 4,
  command_queue_sha_match_count: 4,
  evaluation_input_source: "classic_rts_multi_match_bot_executor_evaluation_input",
  source_action_log_path: "/tmp/bot-planner-action-executor.actions.json",
  source_action_log_sha256: "5555555555555555555555555555555555555555555555555555555555555555",
  source_final_runtime_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
  source_command_queue_sha256: "4444444444444444444444444444444444444444444444444444444444444444",
  source_replay_dir: "/tmp/source-replay",
  variant_summaries: [
    {variant_id: "seed_2026052901_forest_relay", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "forest_relay", economy_variant: "balanced"},
    {variant_id: "seed_2026052902_ridge_watch", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "ridge_watch", economy_variant: "low_gold"},
    {variant_id: "seed_2026052903_marsh_gate", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "marsh_gate", economy_variant: "high_pressure"},
    {variant_id: "seed_2026052904_market_ruins", replay_action_count: 6, accepted_action_count: 6, command_marker_hit_count: 6, command_delta_match_count: 6, runtime_sha_match: true, command_queue_sha_match: true, map_variant: "market_ruins", economy_variant: "delayed_tech"}
  ]
}' >"$multi_log_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$multi_log_json" release_review_recording

multi_summary_json="$TMP_DIR/bevy-classic-rts-multi-match-bot-executor-evaluation.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation_v1",
  green: true,
  multi_match_bot_executor_evaluation_gate: true,
  variant_count: 4,
  accepted_variant_count: 4,
  total_replay_action_count: 24,
  total_accepted_action_count: 24,
  total_command_marker_hit_count: 24,
  total_command_delta_match_count: 24,
  runtime_sha_match_count: 4,
  command_queue_sha_match_count: 4,
  variant_map_values: ["forest_relay", "ridge_watch", "marsh_gate", "market_ruins"],
  bevy_multi_match_bot_executor_evaluation_claimed: true,
  bevy_bot_planner_executor_replay_determinism_claimed: true,
  bevy_openra_runtime_bot_executor_claimed: false,
  android_s5_real_device_claimed: false,
  public_launch_ready: false
}' >"$multi_summary_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$multi_summary_json" release_review_input

multi_ppm_path="$TMP_DIR/multi-match-bot-executor-evaluation.ppm"
printf 'P3\n1280 720\n255\n' >"$multi_ppm_path"
truncate -s 8000001 "$multi_ppm_path"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$multi_ppm_path" release_review_visual_evidence

failure_matrix_log_json="$TMP_DIR/bot-executor-failure-recovery-matrix.matrix.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1",
  source_replay_action_count: 6,
  blocked_injection_count: 6,
  blocked_rejection_count: 6,
  blocked_expected_reason_count: 6,
  blocked_feedback_event_count: 6,
  blocked_command_queue_unchanged_count: 6,
  blocked_command_queue_sha_match_count: 6,
  blocked_input_source: "classic_rts_bot_executor_failure_recovery_matrix_blocked_input",
  recovery_input_source: "classic_rts_bot_executor_failure_recovery_matrix_recovery_input",
  recovery_action_count: 6,
  recovery_accepted_action_count: 6,
  recovery_command_marker_hit_count: 6,
  recovery_command_delta_match_count: 6,
  recovery_safe_runtime_sha_match: true,
  command_queue_sha_match: true,
  source_action_log_path: "/tmp/bot-planner-action-executor.actions.json",
  source_action_log_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  source_final_runtime_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
  source_recovery_safe_runtime_sha256: "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
  source_command_queue_sha256: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
  source_multi_match_dir: "/tmp/source-multi-match",
  matrix_log: [
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_queue_id_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_group_id_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_attack_required_before_ability", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_invalid_tile:bad-tile", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_queue_id_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}},
    {blocked: {accepted: false, rejected: true, expected_reason: "rts_attack_target_required", expected_reason_match: true, command_queue_unchanged: true, command_queue_sha_match: true, feedback_event_delta: 1, blocked_history_delta: 1}, recovery: {action_label_parse_gate: true, accepted: true, command_marker_hit: true, command_delta_match: true}}
  ]
}' >"$failure_matrix_log_json"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$failure_matrix_log_json" release_review_recording

failure_matrix_summary_json="$TMP_DIR/bevy-classic-rts-bot-executor-failure-recovery-matrix.json"
jq -n \
  --argjson matrix_log "$(jq -c '.matrix_log' "$failure_matrix_log_json")" \
  '{
    contract_version: "trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix_v1",
    green: true,
    bot_executor_failure_recovery_matrix_state: "bevy_executor_rejects_blocked_actions_and_recovers_without_command_queue_pollution_not_openra_runtime_bot",
    bot_executor_failure_recovery_matrix_gate: true,
    source_replay_action_count: 6,
    blocked_injection_count: 6,
    blocked_rejection_count: 6,
    blocked_expected_reason_count: 6,
    blocked_feedback_event_count: 6,
    blocked_command_queue_unchanged_count: 6,
    blocked_command_queue_sha_match_count: 6,
    blocked_reason_values: ["rts_queue_id_required", "rts_group_id_required", "rts_attack_required_before_ability", "rts_invalid_tile:bad-tile", "rts_attack_target_required"],
    blocked_input_sources: ["classic_rts_bot_executor_failure_recovery_matrix_blocked_input"],
    recovery_input_sources: ["classic_rts_bot_executor_failure_recovery_matrix_recovery_input"],
    recovery_action_count: 6,
    recovery_accepted_action_count: 6,
    recovery_command_marker_hit_count: 6,
    recovery_command_delta_match_count: 6,
    feedback_blocked_count: 6,
    feedback_recovery_count: 6,
    final_input_feedback_event_count: 12,
    recovery_safe_runtime_sha_match: true,
    command_queue_sha_match: true,
    matrix_log: $matrix_log,
    source_multi_match_summary: {
      variant_count: 4,
      total_replay_action_count: 24,
      total_accepted_action_count: 24,
      evaluation_log_sha256: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
    },
    final_recovery_safe_runtime_summary: {
      faction_id: "mirror_guard",
      objective_capture_percent: 100,
      tier_two_tech_ids: ["relay_foundry"],
      siege_breach_state: "counterplay_won:gate_bulwark",
      match_result_state: "siege_breakthrough:inner_lane"
    },
    bevy_bot_executor_failure_recovery_matrix_claimed: true,
    bevy_multi_match_bot_executor_evaluation_claimed: true,
    bevy_openra_runtime_bot_executor_claimed: false,
    android_s5_real_device_claimed: false,
    public_launch_ready: false
  }' >"$failure_matrix_summary_json"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$failure_matrix_summary_json" release_review_input

failure_matrix_ppm_path="$TMP_DIR/bot-executor-failure-recovery-matrix.ppm"
printf 'P3\n1280 1080\n255\n' >"$failure_matrix_ppm_path"
truncate -s 8000001 "$failure_matrix_ppm_path"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$failure_matrix_ppm_path" release_review_visual_evidence

jq -n \
  --argjson artifacts "$(jq -s '.' "$artifacts_jsonl")" \
  '{
    contract_version: "trillionnium_world_release_review_packet_v1",
    status: "release_review_packet_ready_with_public_launch_blockers",
    ready_for_release_review: true,
    public_launch_ready: false,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    missing_artifacts: [],
    artifacts: $artifacts
  }' >"$packet_json"

{
  printf '# Fixture Packet\n\n'
  printf '## Still Requires Real External Evidence\n\n'
  printf -- '- [ ] fixture blocker\n\n'
  printf '## Boundary\n\n'
  printf -- '- Native/Bevy replay, action coach, HUD/debug layer, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.\n'
} >"$packet_md"

set +e
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$packet_json" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$packet_md" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG="$packet_log" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY="$summary_json" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh" --no-refresh >"$TMP_DIR/stdout.log" 2>"$TMP_DIR/stderr.log"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
  echo "[FAIL] packet integrity semantic fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi

if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity semantic fixture did not write summary" >&2
  exit 1
fi

jq -e '
  .status == "release_review_packet_integrity_blocked"
  and .green == false
  and (.failures | length) == 4
  and ([.failures[].name] | index("first_minute_command_feedback_replay_semantics"))
  and ([.failures[].name] | index("first_minute_command_feedback_source_recording_semantics"))
  and ([.failures[].name] | index("first_minute_command_feedback_recording_semantics"))
  and ([.failures[].name] | index("first_minute_command_feedback_replay_ppm_semantics"))
  and (([.failures[].detail] | index("sha256_mismatch")) == null)
  and (([.failures[].detail] | index("bytes_mismatch")) == null)
  and (([.failures[].detail] | index("contract_mismatch")) == null)
  and (([.failures[].detail] | index("status_mismatch")) == null)
' "$summary_json" >/dev/null

echo "[PASS] release review packet integrity rejects semantically invalid first-minute command feedback artifacts even when checksums match"
