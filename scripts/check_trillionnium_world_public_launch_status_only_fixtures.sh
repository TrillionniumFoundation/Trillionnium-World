#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_STATUS_ONLY_FIXTURE_SUMMARY:-$ACCEPTANCE_DIR/public-launch-status-only-fixtures.json}"
TMP_DIR="$(mktemp -d)"
RESULTS_FILE="$(mktemp)"
trap 'rm -rf "$TMP_DIR"; rm -f "$RESULTS_FILE"' EXIT

mkdir -p "$ACCEPTANCE_DIR"

record_result() {
  local name="$1"
  local exit_code="$2"
  local summary_path="$3"
  local expected_status="$4"
  local summary_status="$5"
  local blocker_present="$6"
  local stdout_path="$7"
  local stderr_path="$8"
  local blocked_as_expected=false
  if [[ "$exit_code" != "0" && "$summary_status" == "$expected_status" ]]; then
    blocked_as_expected=true
  fi

  jq -nc \
    --arg name "$name" \
    --argjson exit_code "$exit_code" \
    --arg summary_path "$summary_path" \
    --arg expected_status "$expected_status" \
    --arg summary_status "$summary_status" \
    --arg stdout_path "$stdout_path" \
    --arg stderr_path "$stderr_path" \
    --argjson blocked_as_expected "$blocked_as_expected" \
    --argjson blocker_present "$blocker_present" \
    '{
      name: $name,
      exit_code: $exit_code,
      summary_path: $summary_path,
      expected_status: $expected_status,
      summary_status: $summary_status,
      blocked_as_expected: $blocked_as_expected,
      blocker_present: $blocker_present,
      stdout_path: $stdout_path,
      stderr_path: $stderr_path
    }' >>"$RESULTS_FILE"
}

json_blockers_present() {
  local summary_path="$1"
  local expr="$2"
  if [[ -f "$summary_path" ]] && jq -e "$expr" "$summary_path" >/dev/null; then
    printf 'true'
  else
    printf 'false'
  fi
}

S5_FIXTURE="$TMP_DIR/s5-status-only.json"
S5_SUMMARY="$TMP_DIR/s5-summary.json"
jq -n '{
  contract_version: "trillionnium_world_s5_native_bevy_device_evidence_v1",
  overall_status: "ready",
  android_target: "aarch64-linux-android",
  native_lib: {
    status: "android_native_cdylib_ready",
    path: "/tmp/status-only-missing/libtrnm_world_bevy.so",
    evidence: "/tmp/status-only-missing/native-lib-symbols.txt"
  },
  apk: {
    status: "signed_debug_apk_ready",
    path: "/tmp/status-only-missing/trillionnium-world-bevy-debug.apk"
  },
  device_matrix: {
    status: "real_device_evidence_collected",
    device_serial: "fixture-s5-status-only",
    adb_devices_evidence: "/tmp/status-only-missing/adb-devices.txt",
    screenshot_evidence: "/tmp/status-only-missing/screenshot.png",
    gfxinfo_evidence: "/tmp/status-only-missing/gfxinfo.txt",
    logcat_evidence: "/tmp/status-only-missing/logcat.txt",
    lifecycle_evidence: "/tmp/status-only-missing/lifecycle.txt",
    crash_free_gate: "crash_free_logcat_window"
  }
}' >"$S5_FIXTURE"
set +e
TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH="$S5_FIXTURE" \
TRILLIONNIUM_WORLD_S5_REAL_DEVICE_VALIDATION_SUMMARY="$S5_SUMMARY" \
  "$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" --require-ready >"$TMP_DIR/s5.out" 2>"$TMP_DIR/s5.err"
S5_EXIT=$?
set -e
record_result \
  s5_status_only_fixture \
  "$S5_EXIT" \
  "$S5_SUMMARY" \
  blocked_missing_s5_real_device_evidence \
  "$(jq -r '.status // "missing"' "$S5_SUMMARY" 2>/dev/null || printf 'missing')" \
  "$(json_blockers_present "$S5_SUMMARY" '(.blockers // []) | length > 0')" \
  "$TMP_DIR/s5.out" \
  "$TMP_DIR/s5.err"

MAP_FIXTURE="$TMP_DIR/map-status-only.json"
MAP_SUMMARY="$TMP_DIR/map-summary.json"
jq -n '{
  contract_version: "trillionnium_world_production_map_pack_public_evidence_v1",
  status: "production_map_pack_public_ready_green"
}' >"$MAP_FIXTURE"
set +e
TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH="$MAP_FIXTURE" \
TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY="$MAP_SUMMARY" \
  "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" --require-ready >"$TMP_DIR/map.out" 2>"$TMP_DIR/map.err"
MAP_EXIT=$?
set -e
record_result \
  production_map_pack_status_only_fixture \
  "$MAP_EXIT" \
  "$MAP_SUMMARY" \
  blocked_missing_production_map_pack_public_evidence \
  "$(jq -r '.status // "missing"' "$MAP_SUMMARY" 2>/dev/null || printf 'missing')" \
  "$(json_blockers_present "$MAP_SUMMARY" '(.blockers // []) | length > 0')" \
  "$TMP_DIR/map.out" \
  "$TMP_DIR/map.err"

COHORT_FIXTURE="$TMP_DIR/cohort-status-only.json"
COMMERCIAL_FIXTURE="$TMP_DIR/commercial-status-only.json"
COHORT_COMMERCIAL_SUMMARY="$TMP_DIR/cohort-commercial-summary.json"
jq -n '{
  contract_version: "trillionnium_world_first_beta_cohort_evidence_v1",
  status: "first_beta_cohort_evidence_green"
}' >"$COHORT_FIXTURE"
jq -n '{
  contract_version: "trillionnium_world_commercial_launch_drill_evidence_v1",
  status: "commercial_launch_drill_evidence_green"
}' >"$COMMERCIAL_FIXTURE"
set +e
TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH="$COHORT_FIXTURE" \
TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH="$COMMERCIAL_FIXTURE" \
TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_SUMMARY="$COHORT_COMMERCIAL_SUMMARY" \
  "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh" --require-ready >"$TMP_DIR/cohort-commercial.out" 2>"$TMP_DIR/cohort-commercial.err"
COHORT_COMMERCIAL_EXIT=$?
set -e
record_result \
  cohort_commercial_status_only_fixture \
  "$COHORT_COMMERCIAL_EXIT" \
  "$COHORT_COMMERCIAL_SUMMARY" \
  blocked_missing_cohort_commercial_real_evidence \
  "$(jq -r '.status // "missing"' "$COHORT_COMMERCIAL_SUMMARY" 2>/dev/null || printf 'missing')" \
  "$(json_blockers_present "$COHORT_COMMERCIAL_SUMMARY" '((.first_beta.blockers // []) | length > 0) and ((.commercial_launch_drill.blockers // []) | length > 0)')" \
  "$TMP_DIR/cohort-commercial.out" \
  "$TMP_DIR/cohort-commercial.err"

LATENCY_FIXTURE="$TMP_DIR/latency-status-only.json"
DEPLOY_FIXTURE="$TMP_DIR/deploy-status-only.json"
EXTERNAL_SUMMARY="$TMP_DIR/external-ops-summary.json"
jq -n '{
  contract_version: "trillionnium_world_multi_node_or_live_traffic_latency_evidence_v1",
  status: "multi_node_or_live_traffic_latency_green"
}' >"$LATENCY_FIXTURE"
jq -n '{
  contract_version: "trillionnium_world_public_network_deploy_evidence_v1",
  status: "public_network_deploy_green"
}' >"$DEPLOY_FIXTURE"
set +e
TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH="$LATENCY_FIXTURE" \
TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH="$DEPLOY_FIXTURE" \
TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_SUMMARY="$EXTERNAL_SUMMARY" \
  "$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" --require-ready >"$TMP_DIR/external-ops.out" 2>"$TMP_DIR/external-ops.err"
EXTERNAL_EXIT=$?
set -e
record_result \
  external_ops_status_only_fixture \
  "$EXTERNAL_EXIT" \
  "$EXTERNAL_SUMMARY" \
  blocked_missing_external_ops_real_evidence \
  "$(jq -r '.status // "missing"' "$EXTERNAL_SUMMARY" 2>/dev/null || printf 'missing')" \
  "$(json_blockers_present "$EXTERNAL_SUMMARY" '((.multi_node_or_live_traffic_latency.blockers // []) | length > 0) and ((.public_network_deploy.blockers // []) | length > 0)')" \
  "$TMP_DIR/external-ops.out" \
  "$TMP_DIR/external-ops.err"

RESULTS_JSON="$(jq -s '.' "$RESULTS_FILE")"
FAILURES_JSON="$(jq -c '[.[] | select(.blocked_as_expected != true or .blocker_present != true)]' <<<"$RESULTS_JSON")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
STATUS=public_launch_status_only_fixture_guard_green
if [[ "$FAILURE_COUNT" != "0" ]]; then
  STATUS=public_launch_status_only_fixture_guard_blocked
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_status_only_fixture_guard_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg fixture_dir "$TMP_DIR" \
  --argjson results "$RESULTS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_public_launch_status_only_fixture_guard",
    guard_rule: "status_only_green_fixtures_must_be_rejected_by_field_level_public_launch_evidence_validators",
    fixture_dir: $fixture_dir,
    result_count: ($results | length),
    failure_count: ($failures | length),
    results: $results,
    failures: $failures
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "public_launch_status_only_fixture_guard_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_STATUS_ONLY_FIXTURE_GUARD_GREEN %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_STATUS_ONLY_FIXTURE_GUARD_BLOCKED %s\n' "$SUMMARY_FILE" >&2
exit 1
