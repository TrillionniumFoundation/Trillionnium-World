#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY"
fi

PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet.json"
PACKET_MD="$ACCEPTANCE_DIR/release-review-packet.md"
PACKET_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-packet.log"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON" ]]; then
  PACKET_JSON="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD" ]]; then
  PACKET_MD="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG" ]]; then
  PACKET_LOG="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG"
fi
CHECK_RESULTS="$(mktemp)"
REFRESH_PACKET=1
trap 'rm -f "$CHECK_RESULTS"' EXIT

for arg in "$@"; do
  case "$arg" in
    --no-refresh)
      REFRESH_PACKET=0
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$ACCEPTANCE_DIR"

add_check() {
  local name="$1"
  local status="$2"
  local path="$3"
  local detail="$4"
  local expected="${5:-}"
  local actual="${6:-}"
  jq -nc \
    --arg name "$name" \
    --arg status "$status" \
    --arg path "$path" \
    --arg detail "$detail" \
    --arg expected "$expected" \
    --arg actual "$actual" \
    '{name: $name, status: $status, path: $path, detail: $detail, expected: (if $expected == "" then null else $expected end), actual: (if $actual == "" then null else $actual end)}' >>"$CHECK_RESULTS"
}

require_json_expr() {
  local name="$1"
  local path="$2"
  local expr="$3"
  local detail="$4"
  if [[ ! -f "$path" ]]; then
    add_check "$name" fail "$path" missing
  elif jq -e "$expr" "$path" >/dev/null; then
    add_check "$name" ok "$path" "$detail"
  else
    add_check "$name" fail "$path" "$detail"
  fi
}

packet_artifact_path() {
  local artifact_id="$1"
  if [[ ! -f "$PACKET_JSON" ]]; then
    return 0
  fi
  jq -r --arg artifact_id "$artifact_id" '(.artifacts // [] | map(select(.id == $artifact_id)) | .[0].path) // empty' "$PACKET_JSON" 2>/dev/null || true
}

require_artifact_json_expr() {
  local name="$1"
  local artifact_id="$2"
  local expr="$3"
  local detail="$4"
  local path
  path="$(packet_artifact_path "$artifact_id")"
  if [[ -z "$path" ]]; then
    add_check "$name" fail "$artifact_id" missing_packet_artifact
  else
    require_json_expr "$name" "$path" "$expr" "$detail"
  fi
}

require_artifact_ppm_header() {
  local name="$1"
  local artifact_id="$2"
  local min_bytes="$3"
  local path
  path="$(packet_artifact_path "$artifact_id")"
  if [[ -z "$path" ]]; then
    add_check "$name" fail "$artifact_id" missing_packet_artifact
    return
  fi
  if [[ ! -f "$path" ]]; then
    add_check "$name" fail "$path" missing
    return
  fi

  local header
  local bytes
  header="$(head -c 15 "$path" || true)"
  bytes="$(wc -c <"$path" | tr -d ' ')"
  if [[ "$header" == $'P3\n1280 720\n255' && "$bytes" -gt "$min_bytes" ]]; then
    add_check "$name" ok "$path" ppm_header_and_size_match "P3 1280x720 > ${min_bytes} bytes" "$bytes"
  else
    add_check "$name" fail "$path" ppm_header_or_size_mismatch "P3 1280x720 > ${min_bytes} bytes" "bytes=${bytes}"
  fi
}

if [[ "$REFRESH_PACKET" -eq 1 ]]; then
  if TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$PACKET_JSON" \
    TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$PACKET_MD" \
    "$ROOT/scripts/check_trillionnium_world_release_review_packet.sh" >"$PACKET_LOG" 2>&1; then
    add_check packet_refresh ok "$PACKET_LOG" refreshed
  else
    add_check packet_refresh fail "$PACKET_LOG" failed
  fi
else
  add_check packet_refresh skipped "$PACKET_LOG" no_refresh_requested
fi

require_json_expr packet_contract "$PACKET_JSON" '.contract_version == "trillionnium_world_release_review_packet_v1"' "packet contract matches"
require_json_expr packet_boundary "$PACKET_JSON" '.android_s5_real_device_claimed == false and .proof_scope == "host_side_bevy_runtime_replay_not_android_real_device"' "packet keeps Android S5 no-claim boundary"
require_json_expr packet_review_status "$PACKET_JSON" '.ready_for_release_review == true and (.status == "release_review_packet_ready_with_public_launch_blockers" or .status == "release_review_packet_green")' "packet is ready for review or public launch review"
require_json_expr packet_missing_artifacts "$PACKET_JSON" '(.missing_artifacts // []) | length == 0' "packet reports no missing artifacts"
require_json_expr packet_artifact_count "$PACKET_JSON" '(.artifacts // []) | length == 58' "packet has expected fifty-eight artifacts including operator handoff, checkpoint manifest, CEX adapter readiness, Bevy action coach, player HUD/debug layer, player UI rescue, classic RTS control loop, first-minute command feedback replay, first-minute command feedback recordings, first-minute command feedback contact sheet, live-window screenshots, sprite texture sampling, sampled texture live-window correlation, render asset eligibility, map modeling gate, bundle negative fixtures, evidence bundle, template negative fixtures, evidence kit, blocker consistency, status-only fixture guard, S5 real-device validation, public launch evidence intake, production map-pack collection, cohort/commercial collection, external ops collection, production map-pack public evidence, cohort/commercial validation, and external ops validation"
require_artifact_json_expr first_minute_command_feedback_replay_semantics native_bevy_first_minute_command_feedback_replay '.contract_version == "trillionnium_world_bevy_first_minute_command_feedback_replay_v1" and .green == true and .command_input_action_count == 7 and .accepted_command_input_count == 7 and .first_minute_replay_gate == true and .command_recording_parse_gate == true and .live_command_input_gate == true and .scene_renderer_gate == true and .history_entry_count == 3 and .history_capacity == 3 and .retained_history_group_ids == ["26", "27", "28"] and .pruned_history_group_ids == ["25", "24"] and .cleared_active_stale_pixel_count == 0 and .preview_width == 1280 and .preview_height == 720 and .android_s5_real_device_claimed == false' "first-minute command feedback replay keeps 7/7 live RTS inputs, recent-3 prune evidence, stale-chip absence, 1280x720 contact sheet boundary"
require_artifact_json_expr first_minute_command_feedback_source_recording_semantics native_bevy_first_minute_command_feedback_source_recording '.contract_version == "trillionnium_world_bevy_first_minute_input_recording_v1" and .source_timeline_contract == "trillionnium_world_bevy_first_minute_interaction_timeline_v1" and .source_timeline_green == true and (.steps | length) == 10 and .android_s5_real_device_claimed == false' "first-minute source recording keeps original first-minute replay timeline and Android no-claim boundary"
require_artifact_json_expr first_minute_command_feedback_recording_semantics native_bevy_first_minute_command_feedback_recording '.contract_version == "trillionnium_world_bevy_first_minute_command_feedback_recording_v1" and .source_input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1" and .source_input_recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1" and .source_input_replay_green == true and .command_history_capacity == 3 and .retained_history_group_ids == ["26", "27", "28"] and .pruned_history_group_ids == ["25", "24"] and (.steps | length) == 7 and [.steps[].action_label] == ["RTS:SELECT:26", "RTS:MOVE:18,31:line", "RTS:SELECT:27", "RTS:MOVE:21,25:line", "RTS:SELECT:28", "RTS:MOVE:1,31:line", "RTS:SELECT:26"] and .android_s5_real_device_claimed == false' "command feedback recording keeps exact 7 action labels, recent-3 history, prune list, and Android no-claim boundary"
require_artifact_ppm_header first_minute_command_feedback_replay_ppm_semantics native_bevy_first_minute_command_feedback_replay_ppm 8000000

if [[ -f "$PACKET_MD" ]]; then
  if grep -Fq -- 'Native/Bevy replay, action coach, HUD/debug layer, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.' "$PACKET_MD" && grep -Fq -- 'Still Requires Real External Evidence' "$PACKET_MD"; then
    add_check packet_markdown_boundary ok "$PACKET_MD" "Markdown includes blocker and boundary sections"
  else
    add_check packet_markdown_boundary fail "$PACKET_MD" "Markdown missing blocker or boundary sections"
  fi
else
  add_check packet_markdown_boundary fail "$PACKET_MD" missing
fi

if [[ -f "$PACKET_JSON" ]]; then
  while IFS= read -r encoded; do
    artifact_json="$(printf '%s' "$encoded" | base64 -d)"
    id="$(jq -r '.id' <<<"$artifact_json")"
    path="$(jq -r '.path' <<<"$artifact_json")"
    expected_sha="$(jq -r '.sha256 // empty' <<<"$artifact_json")"
    expected_bytes="$(jq -r '.bytes // empty' <<<"$artifact_json")"
    expected_contract="$(jq -r '.contract_version // empty' <<<"$artifact_json")"
    expected_status="$(jq -r '.status // empty' <<<"$artifact_json")"

    if [[ ! -f "$path" ]]; then
      add_check "artifact_${id}_present" fail "$path" missing
      continue
    fi

    actual_sha="$(sha256sum "$path" | awk '{print $1}')"
    actual_bytes="$(wc -c <"$path" | tr -d ' ')"
    if [[ "$actual_sha" == "$expected_sha" ]]; then
      add_check "artifact_${id}_sha256" ok "$path" sha256_match "$expected_sha" "$actual_sha"
    else
      add_check "artifact_${id}_sha256" fail "$path" sha256_mismatch "$expected_sha" "$actual_sha"
    fi
    if [[ "$actual_bytes" == "$expected_bytes" ]]; then
      add_check "artifact_${id}_bytes" ok "$path" bytes_match "$expected_bytes" "$actual_bytes"
    else
      add_check "artifact_${id}_bytes" fail "$path" bytes_mismatch "$expected_bytes" "$actual_bytes"
    fi

    if [[ "$path" == *.json ]]; then
      actual_contract="$(jq -r '.contract_version // empty' "$path" 2>/dev/null || true)"
      actual_status="$(jq -r '.status // .overall_status // empty' "$path" 2>/dev/null || true)"
      if [[ -n "$expected_contract" ]]; then
        if [[ "$actual_contract" == "$expected_contract" ]]; then
          add_check "artifact_${id}_contract" ok "$path" contract_match "$expected_contract" "$actual_contract"
        else
          add_check "artifact_${id}_contract" fail "$path" contract_mismatch "$expected_contract" "$actual_contract"
        fi
      fi
      if [[ -n "$expected_status" ]]; then
        if [[ "$actual_status" == "$expected_status" ]]; then
          add_check "artifact_${id}_status" ok "$path" status_match "$expected_status" "$actual_status"
        else
          add_check "artifact_${id}_status" fail "$path" status_mismatch "$expected_status" "$actual_status"
        fi
      fi
    fi
  done < <(jq -r '.artifacts[] | @base64' "$PACKET_JSON")
fi

CHECKS_JSON="$(jq -s '.' "$CHECK_RESULTS")"
FAILURES_JSON="$(jq -s '[.[] | select(.status == "fail")]' "$CHECK_RESULTS")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$PACKET_JSON" 2>/dev/null || printf 'false')"
READY_FOR_RELEASE_REVIEW="$(jq -r '.ready_for_release_review // false' "$PACKET_JSON" 2>/dev/null || printf 'false')"
ARTIFACT_COUNT="$(jq -r '(.artifacts // []) | length' "$PACKET_JSON" 2>/dev/null || printf '0')"

GREEN=false
STATUS=release_review_packet_integrity_blocked
if [[ "$FAILURE_COUNT" == "0" && "$READY_FOR_RELEASE_REVIEW" == "true" ]]; then
  GREEN=true
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    STATUS=release_review_packet_integrity_green
  else
    STATUS=release_review_packet_integrity_green_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_packet_integrity_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg packet_json "$PACKET_JSON" \
  --arg packet_markdown "$PACKET_MD" \
  --arg packet_log "$PACKET_LOG" \
  --argjson green "$GREEN" \
  --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson artifact_count "$ARTIFACT_COUNT" \
  --argjson checks "$CHECKS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_packet_integrity",
    green: $green,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    packet: {json_path: $packet_json, markdown_path: $packet_markdown, refresh_log_path: $packet_log},
    integrity_rule: "packet_artifact_paths_must_exist_and_recorded_sha256_bytes_contract_status_must_match_current_files_including_checkpoint_manifest_cex_adapter_local_bevy_playability_evidence_and_first_minute_command_feedback_replay_semantics",
    artifact_count: $artifact_count,
    checks: $checks,
    failures: $failures,
    reviewer_next_action: (if $green and $public_launch_ready then "review_public_launch_ready_evidence" elif $green then "collect_real_external_public_launch_evidence" else "regenerate_or_repair_release_review_packet" end)
  }' >"$SUMMARY_FILE"

case "$STATUS" in
  release_review_packet_integrity_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  release_review_packet_integrity_green_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac
