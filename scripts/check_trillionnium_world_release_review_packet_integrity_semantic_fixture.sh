#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-review-packet-integrity-semantic-fixture.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SEMANTIC_FIXTURE_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SEMANTIC_FIXTURE_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SEMANTIC_FIXTURE_SUMMARY"
fi

mkdir -p "$(dirname "$SUMMARY_FILE")"

"$ROOT/scripts/v2/release_review_packet_integrity_semantic_guard_test.sh"

jq -n \
  --arg contract_version "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1" \
  --arg status "release_review_packet_integrity_semantic_fixture_green" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_packet_integrity_semantic_fixture",
    green: true,
    fixture_kind: "release_review_status_quickcheck_release_signoff_and_first_minute_command_feedback_semantic_negative_fixture",
    fixture_rule: "packet_integrity_must_reject_semantically_invalid_release_review_status_quickcheck_release_signoff_summary_and_first_minute_command_feedback_artifacts_even_when_sha_bytes_contract_and_status_match",
    fake_packet_artifact_count: 111,
    expected_semantic_failure_count: 7,
    expected_semantic_failure_names: [
      "release_review_status_semantics",
      "release_review_quickcheck_semantics",
      "release_signoff_summary_semantics",
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
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    reviewer_next_action: "inspect_release_review_packet_integrity_semantic_fixture_before_collecting_real_external_public_launch_evidence"
  }' >"$SUMMARY_FILE"

jq -e '
  .contract_version == "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1"
  and .status == "release_review_packet_integrity_semantic_fixture_green"
  and .green == true
  and .fake_packet_artifact_count == 111
  and .expected_semantic_failure_count == 7
  and .expected_semantic_failure_names == [
    "release_review_status_semantics",
    "release_review_quickcheck_semantics",
    "release_signoff_summary_semantics",
    "first_minute_command_feedback_replay_semantics",
    "first_minute_command_feedback_source_recording_semantics",
    "first_minute_command_feedback_recording_semantics",
    "first_minute_command_feedback_replay_ppm_semantics"
  ]
  and .checksum_mismatch_failure_count == 0
  and .bytes_mismatch_failure_count == 0
  and .contract_mismatch_failure_count == 0
  and .status_mismatch_failure_count == 0
  and .ready_for_release_review == true
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$SUMMARY_FILE" >/dev/null

printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SEMANTIC_FIXTURE_GREEN %s\n' "$SUMMARY_FILE"
