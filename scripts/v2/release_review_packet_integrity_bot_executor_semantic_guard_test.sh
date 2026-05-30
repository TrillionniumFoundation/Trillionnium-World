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
  fake_packet_artifact_count: 76,
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
  fake_packet_artifact_count: 76,
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
