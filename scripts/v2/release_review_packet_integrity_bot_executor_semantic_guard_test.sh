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

for index in $(seq 1 52); do
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
  fake_packet_artifact_count: 87,
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
  fake_packet_artifact_count: 87,
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
  fake_packet_artifact_count: 87,
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

bot_gap_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture_v1",
  status: "release_review_packet_integrity_bot_gap_semantic_fixture_green",
  green: true,
  fixture_kind: "bot_gap_foundation_micro_intel_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_bot_gap_foundation_micro_intel_artifacts_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 87,
  expected_semantic_failure_count: 8,
  expected_semantic_failure_names: [
    "bot_decision_state_gap_semantics",
    "bot_decision_state_gap_ppm_semantics",
    "bot_adaptive_build_order_gap_semantics",
    "bot_adaptive_build_order_gap_ppm_semantics",
    "bot_tactical_micro_gap_semantics",
    "bot_tactical_micro_gap_ppm_semantics",
    "bot_map_intel_gap_semantics",
    "bot_map_intel_gap_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$bot_gap_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_bot_gap_semantic_fixture "Release review packet integrity bot gap semantic fixture" "$bot_gap_semantic_fixture_json" release_review_gate

control_loop_semantic_fixture_json="$TMP_DIR/release-review-packet-integrity-control-loop-semantic-fixture.json"
jq -n '{
  contract_version: "trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture_v1",
  status: "release_review_packet_integrity_control_loop_semantic_fixture_green",
  green: true,
  fixture_kind: "classic_rts_control_loop_semantic_negative_fixture",
  fixture_rule: "packet_integrity_must_reject_semantically_invalid_classic_rts_control_loop_summary_and_ppm_even_when_sha_bytes_contract_and_status_match",
  fake_packet_artifact_count: 87,
  expected_semantic_failure_count: 2,
  expected_semantic_failure_names: [
    "classic_rts_control_loop_semantics",
    "classic_rts_control_loop_ppm_semantics"
  ],
  checksum_mismatch_failure_count: 0,
  bytes_mismatch_failure_count: 0,
  contract_mismatch_failure_count: 0,
  status_mismatch_failure_count: 0,
  ready_for_release_review: true,
  public_launch_ready: false,
  android_s5_real_device_claimed: false,
  proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
}' >"$control_loop_semantic_fixture_json"
add_artifact_from_path release_review_packet_integrity_control_loop_semantic_fixture "Release review packet integrity control loop semantic fixture" "$control_loop_semantic_fixture_json" release_review_gate

control_loop_json="$TMP_DIR/bevy-classic-rts-control-loop.json"
jq -n '{
  contract_version: "trillionnium_world_bevy_classic_rts_control_loop_v1",
  green: true,
  preview_width: 1280,
  preview_height: 360,
  preview_format: "ppm_p3_rgb",
  write_gate: true,
  mirror_scene_gate: true,
  coliseum_scene_gate: true,
  non_background_pixels: 460800,
  control_group_id: "1",
  move_selected_unit_count: 4,
  attack_selected_unit_count: 4,
  move_command_queue: ["select_group_1", "move:7,4", "formation:diamond"],
  attack_command_queue: ["select_group_1", "attack:arena_creep_attack"],
  attack_target_id: "arena_creep_attack",
  selection_marker_pixel_count: 1576,
  formation_line_pixel_count: 485,
  command_marker_pixel_count: 808,
  attack_feedback_pixel_count: 601,
  strategy_panel_pixel_count: 140066,
  minimap_pixel_count: 3833,
  fog_pixel_count: 2124,
  vision_pixel_count: 240,
  resource_hud_pixel_count: 462,
  production_queue_pixel_count: 9045,
  move_production_queue: ["train:worker", "train:guard"],
  move_build_queue: ["build:scout_tower"],
  attack_build_queue: ["upgrade:training_hall"],
  move_training_progress_percent: 64,
  attack_build_progress_percent: 56,
  unit_health_card_pixel_count: 804,
  ability_command_pixel_count: 13511,
  target_health_pixel_count: 350,
  attack_target_health_percent: 46,
  attack_active_ability_id: "focus_fire",
  attack_ability_command_ids: ["attack", "focus_fire", "guard", "retreat"],
  attack_combat_event_log: ["focus_fire:arena_creep_attack", "damage:28"],
  selection_gate: true,
  command_queue_gate: true,
  strategy_hud_gate: true,
  macro_loop_gate: true,
  tactical_combat_gate: true,
  gameplay_surface_gate: true,
  cex_runtime_player_client_allowed: false,
  wgpu_required: false
}' >"$control_loop_json"
add_artifact_from_path native_bevy_classic_rts_control_loop "Native/Bevy classic RTS control loop" "$control_loop_json" release_review_input

control_loop_ppm="$TMP_DIR/bevy-classic-rts-control-loop.ppm"
printf 'P3\n1280 360\n255\n' >"$control_loop_ppm"
truncate -s 4000001 "$control_loop_ppm"
add_artifact_from_path native_bevy_classic_rts_control_loop_ppm "Native/Bevy classic RTS control loop PPM" "$control_loop_ppm" release_review_visual_evidence

"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_replay.sh" >"$TMP_DIR/first-minute-command-feedback-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_first_minute_command_feedback_rejection_replay.sh" >"$TMP_DIR/first-minute-command-feedback-rejection-replay.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_action_executor.sh" >"$TMP_DIR/bot-planner-action-executor.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_planner_executor_replay_determinism.sh" >"$TMP_DIR/bot-planner-executor-replay-determinism.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_multi_match_bot_executor_evaluation.sh" >"$TMP_DIR/multi-match-bot-executor-evaluation.log"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_executor_failure_recovery_matrix.sh" >"$TMP_DIR/bot-executor-failure-recovery-matrix.log"

first_minute_dir="$ROOT/acceptance/S5_native_bevy_device/latest"
add_artifact_from_path native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$first_minute_dir/bevy-first-minute-command-feedback-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$first_minute_dir/bevy-first-minute-command-feedback-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$first_minute_dir/bevy-first-minute-command-feedback-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$first_minute_dir/bevy-first-minute-command-feedback-replay.ppm" release_review_visual_evidence
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-replay.json" release_review_input
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-source-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-recording.json" release_review_recording
add_artifact_from_path native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$first_minute_dir/bevy-first-minute-command-feedback-rejection-replay.ppm" release_review_visual_evidence

action_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-action-executor.bad.json"
jq '
  .accepted_action_count = 5
  | .command_marker_hit_count = 5
  | .action_labels[1] = "RTS:QUEUE:recon:sweep:wrong_tile@0,0"
  | .final_runtime_summary.objective_capture_percent = 50
' "$first_minute_dir/bevy-classic-rts-bot-planner-action-executor.json" >"$action_summary_json"
add_artifact_from_path native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$action_summary_json" release_review_input

action_log_json="$TMP_DIR/bot-planner-action-executor.actions.bad.json"
jq '
  .accepted_action_count = 5
  | .command_marker_hit_count = 5
  | .execution_log[0].accepted = false
  | .execution_log[0].command_marker_hit = false
  | .execution_log[0].action_label = "RTS:QUEUE:faction:wrong_guard"
' "$first_minute_dir/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.actions.json" >"$action_log_json"
add_artifact_from_path native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$action_log_json" release_review_recording

action_ppm_path="$TMP_DIR/bot-planner-action-executor.bad.ppm"
printf 'P3\n1279 1080\n255\n' >"$action_ppm_path"
truncate -s 8000001 "$action_ppm_path"
add_artifact_from_path native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$action_ppm_path" release_review_visual_evidence

replay_summary_json="$TMP_DIR/bevy-classic-rts-bot-planner-executor-replay-determinism.bad.json"
jq '
  .accepted_replay_action_count = 5
  | .command_delta_match_count = 5
  | .runtime_determinism_gate = false
  | .replay_final_runtime_sha256 = "bad-runtime-sha"
' "$first_minute_dir/bevy-classic-rts-bot-planner-executor-replay-determinism.json" >"$replay_summary_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$replay_summary_json" release_review_input

replay_log_json="$TMP_DIR/bot-planner-executor-replay-determinism.replay.bad.json"
jq '
  .accepted_replay_action_count = 5
  | .command_delta_match_count = 5
  | .replay_final_runtime_sha256 = "bad-runtime-sha"
  | .execution_log[0].accepted = false
  | .execution_log[0].command_delta_match = false
' "$first_minute_dir/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.replay.json" >"$replay_log_json"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$replay_log_json" release_review_recording

replay_ppm_path="$TMP_DIR/bot-planner-executor-replay-determinism.bad.ppm"
printf 'P3\n1280 1079\n255\n' >"$replay_ppm_path"
truncate -s 8000001 "$replay_ppm_path"
add_artifact_from_path native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$replay_ppm_path" release_review_visual_evidence

multi_summary_json="$TMP_DIR/bevy-classic-rts-multi-match-bot-executor-evaluation.bad.json"
jq '
  .variant_count = 3
  | .accepted_variant_count = 3
  | .total_accepted_action_count = 23
  | .total_command_delta_match_count = 23
  | .runtime_sha_match_count = 3
  | .variant_map_values = ["forest_relay", "ridge_watch", "marsh_gate"]
' "$first_minute_dir/bevy-classic-rts-multi-match-bot-executor-evaluation.json" >"$multi_summary_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$multi_summary_json" release_review_input

multi_log_json="$TMP_DIR/multi-match-bot-executor-evaluation.matches.bad.json"
jq '
  .variant_count = 3
  | .accepted_variant_count = 3
  | .total_accepted_action_count = 23
  | .total_command_delta_match_count = 23
  | .runtime_sha_match_count = 3
  | .variant_summaries[0].accepted_action_count = 5
  | .variant_summaries[0].command_delta_match_count = 5
  | .variant_summaries[0].runtime_sha_match = false
' "$first_minute_dir/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.matches.json" >"$multi_log_json"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$multi_log_json" release_review_recording

multi_ppm_path="$TMP_DIR/multi-match-bot-executor-evaluation.bad.ppm"
printf 'P3\n1279 720\n255\n' >"$multi_ppm_path"
truncate -s 8000001 "$multi_ppm_path"
add_artifact_from_path native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$multi_ppm_path" release_review_visual_evidence

add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$first_minute_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix.json" release_review_input
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$first_minute_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.matrix.json" release_review_recording
add_artifact_from_path native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$first_minute_dir/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.ppm" release_review_visual_evidence

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
  echo "[FAIL] packet integrity bot executor semantic fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi

if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity bot executor semantic fixture did not write summary" >&2
  exit 1
fi

jq -e '
  .status == "release_review_packet_integrity_blocked"
  and .green == false
  and (.failures | length) == 9
  and ([.failures[].name] | index("bot_planner_action_executor_semantics"))
  and ([.failures[].name] | index("bot_planner_action_executor_log_semantics"))
  and ([.failures[].name] | index("bot_planner_action_executor_ppm_semantics"))
  and ([.failures[].name] | index("bot_planner_executor_replay_determinism_semantics"))
  and ([.failures[].name] | index("bot_planner_executor_replay_determinism_log_semantics"))
  and ([.failures[].name] | index("bot_planner_executor_replay_determinism_ppm_semantics"))
  and ([.failures[].name] | index("multi_match_bot_executor_evaluation_semantics"))
  and ([.failures[].name] | index("multi_match_bot_executor_evaluation_log_semantics"))
  and ([.failures[].name] | index("multi_match_bot_executor_evaluation_ppm_semantics"))
  and (([.failures[].detail] | index("sha256_mismatch")) == null)
  and (([.failures[].detail] | index("bytes_mismatch")) == null)
  and (([.failures[].detail] | index("contract_mismatch")) == null)
  and (([.failures[].detail] | index("status_mismatch")) == null)
' "$summary_json" >/dev/null

echo "[PASS] release review packet integrity rejects semantically invalid bot executor source-chain artifacts even when checksums match"
