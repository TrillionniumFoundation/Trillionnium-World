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
  fake_packet_artifact_count: 85,
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
  fake_packet_artifact_count: 85,
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
  fake_packet_artifact_count: 85,
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

"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh" >"$TMP_DIR/first-minute-command-feedback-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh" >"$TMP_DIR/first-minute-command-feedback-rejection-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh" >"$TMP_DIR/bot-planner-action-executor.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh" >"$TMP_DIR/bot-planner-executor-replay-determinism.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh" >"$TMP_DIR/multi-match-bot-executor-evaluation.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh" >"$TMP_DIR/bot-executor-failure-recovery-matrix.log"

native_dir="$ROOT/acceptance/S5_native_bevy_device/latest"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$native_dir/bevy-first-minute-command-feedback-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$native_dir/bevy-first-minute-command-feedback-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$native_dir/bevy-first-minute-command-feedback-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$native_dir/bevy-first-minute-command-feedback-replay.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$native_dir/bevy-first-minute-command-feedback-rejection-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$native_dir/bevy-first-minute-command-feedback-rejection-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$native_dir/bevy-first-minute-command-feedback-rejection-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$native_dir/bevy-first-minute-command-feedback-rejection-replay.ppm" release_review_visual_evidence

add_artifact_from_path native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$native_dir/bevy-classic-rts-bot-planner-action-executor.json" release_review_input
add_artifact_from_path native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$native_dir/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.actions.json" release_review_recording
add_artifact_from_path native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$native_dir/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$native_dir/bevy-classic-rts-bot-planner-executor-replay-determinism.json" release_review_input
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$native_dir/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.replay.json" release_review_recording
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$native_dir/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$native_dir/bevy-classic-rts-multi-match-bot-executor-evaluation.json" release_review_input
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$native_dir/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.matches.json" release_review_recording
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$native_dir/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.ppm" release_review_visual_evidence

matrix_summary_json="$TMP_DIR/bevy-classic-rts-bot-executor-failure-recovery-matrix.bad.json"
jq '
  .blocked_rejection_count = 5
  | .blocked_expected_reason_count = 5
  | .blocked_command_queue_unchanged_count = 5
  | .recovery_accepted_action_count = 5
  | .recovery_command_delta_match_count = 5
  | .recovery_safe_runtime_sha_match = false
  | .command_queue_sha_match = false
  | .matrix_log[0].blocked.expected_reason_match = false
  | .matrix_log[0].blocked.command_queue_unchanged = false
  | .matrix_log[0].recovery.command_delta_match = false
  | .source_multi_match_summary.variant_count = 3
  | .source_multi_match_summary.total_accepted_action_count = 23
  | .final_recovery_safe_runtime_summary.objective_capture_percent = 50
' "$native_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix.json" >"$matrix_summary_json"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$matrix_summary_json" release_review_input

matrix_log_json="$TMP_DIR/bot-executor-failure-recovery-matrix.matrix.bad.json"
jq '
  .blocked_rejection_count = 5
  | .blocked_command_queue_unchanged_count = 5
  | .recovery_accepted_action_count = 5
  | .recovery_command_delta_match_count = 5
  | .recovery_safe_runtime_sha_match = false
  | .command_queue_sha_match = false
  | .matrix_log[0].blocked.expected_reason = "bad_reason"
  | .matrix_log[0].blocked.command_queue_unchanged = false
  | .matrix_log[0].blocked.command_queue_sha_match = false
  | .matrix_log[0].recovery.accepted = false
  | .matrix_log[0].recovery.command_delta_match = false
' "$native_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.matrix.json" >"$matrix_log_json"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$matrix_log_json" release_review_recording

matrix_ppm_path="$TMP_DIR/bot-executor-failure-recovery-matrix.bad.ppm"
printf 'P3\n1279 1080\n255\n' >"$matrix_ppm_path"
truncate -s 8000001 "$matrix_ppm_path"
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$matrix_ppm_path" release_review_visual_evidence

bot_decision_gap_json="$TMP_DIR/bevy-classic-rts-bot-decision-state-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_decision_state_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_decision_gap_state: "bevy_bot_decision_vocabulary_not_openra_native_bot_ai",
  bevy_native_bot_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  bot_decision_stage_count: 6,
  stage_summaries: [
    {stage: "economy_seed"},
    {stage: "scout_objectives"},
    {stage: "capture_beacon"},
    {stage: "tech_switch"},
    {stage: "defend_counter"},
    {stage: "attack_commit_with_counter_repath"}
  ],
  decision_signal_count: 18,
  economy_decision_count: 3,
  objective_decision_count: 4,
  combat_decision_count: 4,
  tech_decision_count: 2,
  final_bot_decision_state: "attack_commit_with_counter_repath",
  final_rts_ai_pressure_percent: 70,
  final_rts_defeat_risk_percent: 35,
  final_objective_capture_percent: 90,
  final_match_result_state: "bot_decision_gap:attack_commit_with_counter_repath",
  final_command_queue: ["decision:combat:attack_commit_with_counter_repath", "parity_claim:false"],
  final_army_production_batch_ids: ["batch:tech:signal+skimmer+bastion"],
  bot_decision_state_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_decision_gap_json"
add_artifact_from_path native_bevy_bot_decision_state_gap "Native/Bevy bot decision-state gap" "$bot_decision_gap_json" release_review_input

bot_decision_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-decision-state-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_decision_gap_ppm"
truncate -s 8000001 "$bot_decision_gap_ppm"
add_artifact_from_path native_bevy_bot_decision_state_gap_ppm "Native/Bevy bot decision-state gap PPM" "$bot_decision_gap_ppm" release_review_visual_evidence

bot_adaptive_gap_json="$TMP_DIR/bevy-classic-rts-bot-adaptive-build-order-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_adaptive_build_gap_state: "bevy_adaptive_build_order_vocabulary_not_openra_native_ai_planner",
  bevy_native_adaptive_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  adaptive_stage_count: 6,
  stage_summaries: [
    {stage: "opening_worker_split"},
    {stage: "scout_trigger_response"},
    {stage: "expand_or_defend_branch"},
    {stage: "tech_counter_switch"},
    {stage: "pressure_window_commit"},
    {stage: "retreat_rebuild_reattack"}
  ],
  adaptive_signal_count: 24,
  opening_build_order_count: 3,
  scout_trigger_count: 2,
  branch_switch_count: 3,
  counter_tech_switch_count: 2,
  pressure_window_count: 2,
  retreat_rebuild_count: 2,
  final_adaptive_state: "pressure_window_rebuild_reattack",
  final_rts_ai_pressure_percent: 70,
  final_rts_defeat_risk_percent: 20,
  final_objective_capture_percent: 90,
  final_match_result_state: "adaptive_build_gap:pressure_window_rebuild_reattack",
  final_command_queue: ["adaptive_stage:retreat_rebuild_reattack", "native_openra_ai_planner:false"],
  final_army_production_batch_ids: ["build_order:signal_array_into_skimmer", "build_order:pullback_rebuild_then_reattack"],
  adaptive_build_order_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_adaptive_gap_json"
add_artifact_from_path native_bevy_bot_adaptive_build_order_gap "Native/Bevy bot adaptive build-order gap" "$bot_adaptive_gap_json" release_review_input

bot_adaptive_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-adaptive-build-order-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_adaptive_gap_ppm"
truncate -s 8000001 "$bot_adaptive_gap_ppm"
add_artifact_from_path native_bevy_bot_adaptive_build_order_gap_ppm "Native/Bevy bot adaptive build-order gap PPM" "$bot_adaptive_gap_ppm" release_review_visual_evidence

bot_tactical_micro_gap_json="$TMP_DIR/bevy-classic-rts-bot-tactical-micro-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_tactical_micro_gap_state: "bevy_tactical_micro_vocabulary_not_openra_native_combat_ai",
  bevy_native_combat_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  micro_stage_count: 6,
  stage_summaries: [
    {stage: "target_priority_probe"},
    {stage: "focus_fire_commit"},
    {stage: "kite_and_stutter_step"},
    {stage: "flank_angle_split"},
    {stage: "ability_timing_window"},
    {stage: "low_health_pullback_regroup"}
  ],
  micro_signal_count: 24,
  target_swap_count: 3,
  focus_fire_order_count: 3,
  kite_step_count: 3,
  flank_angle_count: 2,
  ability_timing_count: 2,
  low_health_pullback_count: 2,
  final_micro_state: "pullback_regroup_reattack",
  final_rts_ai_pressure_percent: 70,
  final_rts_defeat_risk_percent: 20,
  final_objective_capture_percent: 90,
  final_match_result_state: "tactical_micro_gap:pullback_regroup_reattack",
  final_command_queue: ["micro_stage:low_health_pullback_regroup", "native_openra_combat_ai:false"],
  final_army_production_batch_ids: ["micro_control:focus_fire_low_armor_striker", "micro_control:pull_redline_units_regroup_reattack"],
  tactical_micro_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_tactical_micro_gap_json"
add_artifact_from_path native_bevy_bot_tactical_micro_gap "Native/Bevy bot tactical micro gap" "$bot_tactical_micro_gap_json" release_review_input

bot_tactical_micro_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-tactical-micro-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_tactical_micro_gap_ppm"
truncate -s 8000001 "$bot_tactical_micro_gap_ppm"
add_artifact_from_path native_bevy_bot_tactical_micro_gap_ppm "Native/Bevy bot tactical micro gap PPM" "$bot_tactical_micro_gap_ppm" release_review_visual_evidence

bot_map_intel_gap_json="$TMP_DIR/bevy-classic-rts-bot-map-intel-gap.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_bot_map_intel_gap_v1",
  green: true,
  preview_width: 1280,
  preview_height: 1080,
  write_gate: true,
  input_action_count: 0,
  bevy_bot_map_intel_gap_state: "bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai",
  bevy_native_shroud_memory_ai_claimed: false,
  bevy_openra_parity_claimed: false,
  openra_gap_not_closed_gate: true,
  openra_bot_economy_tech_target_commit: "f6c47d9",
  openra_bot_beacon_pressure_target_commit: "2b6f25b",
  openra_organic_bot_terminal_target_commit: "5f1bf76",
  intel_stage_count: 6,
  stage_summaries: [
    {stage: "initial_scout_sweep"},
    {stage: "fog_memory_stamp"},
    {stage: "expansion_threat_inference"},
    {stage: "enemy_tech_read"},
    {stage: "hidden_army_prediction"},
    {stage: "rotate_pressure_reveal"}
  ],
  intel_signal_count: 24,
  scout_sweep_count: 3,
  fog_memory_stamp_count: 4,
  expansion_threat_count: 3,
  enemy_tech_read_count: 2,
  hidden_army_prediction_count: 2,
  pressure_rotation_count: 2,
  final_intel_state: "rotate_pressure_confirmed_beacon",
  final_rts_ai_pressure_percent: 80,
  final_rts_defeat_risk_percent: 20,
  final_objective_capture_percent: 90,
  final_match_result_state: "map_intel_gap:rotate_pressure_confirmed_beacon",
  final_command_queue: ["intel_stage:rotate_pressure_reveal", "native_openra_shroud_memory_ai:false"],
  final_army_production_batch_ids: ["map_intel:fog_memory_last_seen_grid", "map_intel:rotate_pressure_to_confirmed_beacon"],
  map_intel_gap_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$bot_map_intel_gap_json"
add_artifact_from_path native_bevy_bot_map_intel_gap "Native/Bevy bot map intel gap" "$bot_map_intel_gap_json" release_review_input

bot_map_intel_gap_ppm="$TMP_DIR/bevy-classic-rts-bot-map-intel-gap.ppm"
printf 'P3\n1280 1080\n255\n' >"$bot_map_intel_gap_ppm"
truncate -s 8000001 "$bot_map_intel_gap_ppm"
add_artifact_from_path native_bevy_bot_map_intel_gap_ppm "Native/Bevy bot map intel gap PPM" "$bot_map_intel_gap_ppm" release_review_visual_evidence

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
  echo "[FAIL] packet integrity bot executor matrix semantic fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi

if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity bot executor matrix semantic fixture did not write summary" >&2
  exit 1
fi

jq -e '
  .status == "release_review_packet_integrity_blocked"
  and .green == false
  and (.failures | length) == 3
  and ([.failures[].name] | index("bot_executor_failure_recovery_matrix_semantics"))
  and ([.failures[].name] | index("bot_executor_failure_recovery_matrix_log_semantics"))
  and ([.failures[].name] | index("bot_executor_failure_recovery_matrix_ppm_semantics"))
  and (([.failures[].detail] | index("sha256_mismatch")) == null)
  and (([.failures[].detail] | index("bytes_mismatch")) == null)
  and (([.failures[].detail] | index("contract_mismatch")) == null)
  and (([.failures[].detail] | index("status_mismatch")) == null)
' "$summary_json" >/dev/null

echo "[PASS] release review packet integrity rejects semantically invalid bot executor failure/recovery matrix artifacts even when checksums match"
