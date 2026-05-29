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
  fake_packet_artifact_count: 59,
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
