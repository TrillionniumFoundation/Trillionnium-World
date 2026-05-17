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

add_artifact() {
  local id="$1"
  local artifact_path="$TMP_DIR/${id}.json"
  printf '{"contract_version":"fixture_contract_v1","status":"fixture_green","payload":"%s"}\n' "$id" >"$artifact_path"
  local artifact_sha
  artifact_sha="$(sha256sum "$artifact_path" | awk '{print $1}')"
  local artifact_bytes
  artifact_bytes="$(wc -c <"$artifact_path" | tr -d ' ')"
  jq -nc \
    --arg id "$id" \
    --arg path "$artifact_path" \
    --arg sha "$artifact_sha" \
    --arg bytes "$artifact_bytes" \
    '{id: $id, label: $id, path: $path, role: "fixture", file_status: "present", sha256: $sha, bytes: ($bytes | tonumber), contract_version: "fixture_contract_v1", status: "fixture_green"}' >>"$artifacts_jsonl"
}

for id in artifact_1 artifact_2 artifact_3 artifact_4 artifact_5 artifact_6 artifact_7 artifact_8 artifact_9 artifact_10 artifact_11 artifact_12 artifact_13 artifact_14 artifact_15 artifact_16 artifact_17 artifact_18 artifact_19 artifact_20 artifact_21 artifact_22; do
  add_artifact "$id"
done

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

printf '{"contract_version":"fixture_contract_v2","status":"fixture_red","payload":"artifact_1 drifted"}\n' >"$TMP_DIR/artifact_1.json"

set +e
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$packet_json" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$packet_md" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG="$packet_log" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY="$summary_json" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh" --no-refresh >"$TMP_DIR/stdout.log" 2>"$TMP_DIR/stderr.log"
status=$?
set -e

if [[ "$status" -eq 0 ]]; then
  echo "[FAIL] packet integrity drift fixture unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi

if [[ ! -f "$summary_json" ]]; then
  echo "[FAIL] packet integrity drift fixture did not write summary" >&2
  exit 1
fi

jq -e '
  .status == "release_review_packet_integrity_blocked"
  and .green == false
  and ([.failures[].detail] | index("sha256_mismatch"))
  and ([.failures[].detail] | index("bytes_mismatch"))
  and ([.failures[].detail] | index("contract_mismatch"))
  and ([.failures[].detail] | index("status_mismatch"))
' "$summary_json" >/dev/null

echo "[PASS] release review packet integrity rejects artifact sha/byte/contract/status drift fixtures"
