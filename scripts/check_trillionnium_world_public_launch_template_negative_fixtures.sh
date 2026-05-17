#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MAP_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/public-launch-template-negative-fixtures.json"
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_TEMPLATE_NEGATIVE_FIXTURES_SUMMARY && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_TEMPLATE_NEGATIVE_FIXTURES_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_TEMPLATE_NEGATIVE_FIXTURES_SUMMARY"
fi
RESULTS_FILE="$(mktemp)"
TMP_DIR="$(mktemp -d)"
trap 'rm -f "$RESULTS_FILE"; rm -rf "$TMP_DIR"' EXIT

mkdir -p "$ACCEPTANCE_DIR" "$S5_DIR" "$MAP_DIR"

KIT_LOG="$ACCEPTANCE_DIR/public-launch-template-negative-fixtures-kit.log"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" >"$KIT_LOG"

add_result() {
  local name="$1"
  local validator="$2"
  local summary_path="$3"
  local expected_status="$4"
  local template_paths_json="$5"
  local exit_status="$6"
  local actual_status
  actual_status="$(jq -r '.status // empty' "$summary_path" 2>/dev/null || true)"
  local rejected=false
  if [[ "$exit_status" != "0" && "$actual_status" == "$expected_status" ]]; then
    rejected=true
  fi
  jq -nc \
    --arg name "$name" \
    --arg validator "$validator" \
    --arg summary_path "$summary_path" \
    --arg expected_status "$expected_status" \
    --arg actual_status "$actual_status" \
    --argjson template_paths "$template_paths_json" \
    --argjson exit_status "$exit_status" \
    --argjson rejected "$rejected" \
    '{name: $name, validator: $validator, summary_path: $summary_path, expected_status: $expected_status, actual_status: (if $actual_status == "" then null else $actual_status end), template_paths: $template_paths, exit_status: $exit_status, rejected: $rejected}' >>"$RESULTS_FILE"
}

json_paths() {
  printf '%s\n' "$@" | jq -Rsc 'split("\n") | map(select(length > 0))'
}

run_case() {
  local name="$1"
  local validator="$2"
  local summary_path="$3"
  local expected_status="$4"
  local template_paths_json="$5"
  shift 5
  set +e
  "$@" >/dev/null 2>"$TMP_DIR/$name.stderr.log"
  local exit_status=$?
  set -e
  add_result "$name" "$validator" "$summary_path" "$expected_status" "$template_paths_json" "$exit_status"
}

S5_TEMPLATE="$S5_DIR/s5-device-evidence.template.json"
MAP_TEMPLATE="$MAP_DIR/production-map-pack-public-evidence.template.json"
COHORT_TEMPLATE="$ACCEPTANCE_DIR/first-beta-cohort-evidence.template.json"
COMMERCIAL_TEMPLATE="$ACCEPTANCE_DIR/commercial-launch-drill-evidence.template.json"
LATENCY_TEMPLATE="$ACCEPTANCE_DIR/multi-node-latency-evidence.template.json"
DEPLOY_TEMPLATE="$ACCEPTANCE_DIR/public-network-deploy-evidence.template.json"

run_case \
  s5_real_device_template \
  check_trillionnium_world_s5_real_device_evidence \
  "$TMP_DIR/s5-real-device-template-summary.json" \
  blocked_missing_s5_real_device_evidence \
  "$(json_paths "$S5_TEMPLATE")" \
  env TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH="$S5_TEMPLATE" TRILLIONNIUM_WORLD_S5_REAL_DEVICE_VALIDATION_SUMMARY="$TMP_DIR/s5-real-device-template-summary.json" "$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" --require-ready

run_case \
  production_map_pack_template \
  check_trillionnium_world_production_map_pack_public_evidence \
  "$TMP_DIR/production-map-pack-template-summary.json" \
  blocked_missing_production_map_pack_public_evidence \
  "$(json_paths "$MAP_TEMPLATE")" \
  env TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH="$MAP_TEMPLATE" TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY="$TMP_DIR/production-map-pack-template-summary.json" "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" --require-ready

run_case \
  cohort_commercial_templates \
  check_trillionnium_world_cohort_commercial_evidence \
  "$TMP_DIR/cohort-commercial-template-summary.json" \
  blocked_missing_cohort_commercial_real_evidence \
  "$(json_paths "$COHORT_TEMPLATE" "$COMMERCIAL_TEMPLATE")" \
  env TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH="$COHORT_TEMPLATE" TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH="$COMMERCIAL_TEMPLATE" TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_SUMMARY="$TMP_DIR/cohort-commercial-template-summary.json" "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh" --require-ready

run_case \
  external_ops_templates \
  check_trillionnium_world_external_ops_evidence \
  "$TMP_DIR/external-ops-template-summary.json" \
  blocked_missing_external_ops_real_evidence \
  "$(json_paths "$LATENCY_TEMPLATE" "$DEPLOY_TEMPLATE")" \
  env TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH="$LATENCY_TEMPLATE" TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH="$DEPLOY_TEMPLATE" TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_SUMMARY="$TMP_DIR/external-ops-template-summary.json" "$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" --require-ready

RESULTS_JSON="$(jq -s '.' "$RESULTS_FILE")"
FAILURES_JSON="$(jq -c '[.[] | select(.rejected != true)]' <<<"$RESULTS_JSON")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
RESULT_COUNT="$(jq 'length' <<<"$RESULTS_JSON")"
TEMPLATE_COUNT="$(jq '[.[] | .template_paths[]] | length' <<<"$RESULTS_JSON")"
STATUS=public_launch_template_negative_fixtures_green
GREEN=true
if [[ "$FAILURE_COUNT" != "0" ]]; then
  STATUS=public_launch_template_negative_fixtures_blocked
  GREEN=false
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_template_negative_fixtures_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg kit_log "$KIT_LOG" \
  --argjson green "$GREEN" \
  --argjson result_count "$RESULT_COUNT" \
  --argjson template_count "$TEMPLATE_COUNT" \
  --argjson results "$RESULTS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{contract_version: $contract_version, status: $status, generated_at: $generated_at, source_of_truth: "trillionnium_world_public_launch_template_negative_fixtures", green: $green, public_launch_claimed: false, android_s5_real_device_claimed: false, live_map_ingestion_performed: false, live_public_exposure_performed: false, template_negative_rule: "no_credit_templates_must_fail_strict_field_validators_before_public_launch_handoff", evidence_kit_log: $kit_log, result_count: $result_count, template_count: $template_count, results: $results, failures: $failures}' >"$SUMMARY_FILE"

if [[ "$STATUS" == "public_launch_template_negative_fixtures_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_TEMPLATE_NEGATIVE_FIXTURES_GREEN %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_TEMPLATE_NEGATIVE_FIXTURES_BLOCKED %s\n' "$SUMMARY_FILE" >&2
exit 1
