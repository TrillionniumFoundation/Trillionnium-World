#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MAP_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/public-launch-bundle-negative-fixtures.json"
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BUNDLE_NEGATIVE_FIXTURES_SUMMARY && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BUNDLE_NEGATIVE_FIXTURES_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BUNDLE_NEGATIVE_FIXTURES_SUMMARY"
fi
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$ACCEPTANCE_DIR" "$S5_DIR" "$MAP_DIR"

KIT_LOG="$ACCEPTANCE_DIR/public-launch-bundle-negative-fixtures-kit.log"
BUNDLE_LOG="$ACCEPTANCE_DIR/public-launch-bundle-negative-fixtures-bundle.log"
FAKE_BUNDLE="$TMP_DIR/fake-green-template-bundle.json"
BUNDLE_SUMMARY="$TMP_DIR/fake-green-template-bundle-summary.json"
BUNDLE_MARKDOWN="$TMP_DIR/fake-green-template-bundle-summary.md"

"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" >"$KIT_LOG"

S5_TEMPLATE="$S5_DIR/s5-device-evidence.template.json"
MAP_TEMPLATE="$MAP_DIR/production-map-pack-public-evidence.template.json"
COHORT_TEMPLATE="$ACCEPTANCE_DIR/first-beta-cohort-evidence.template.json"
COMMERCIAL_TEMPLATE="$ACCEPTANCE_DIR/commercial-launch-drill-evidence.template.json"
LATENCY_TEMPLATE="$ACCEPTANCE_DIR/multi-node-latency-evidence.template.json"
DEPLOY_TEMPLATE="$ACCEPTANCE_DIR/public-network-deploy-evidence.template.json"

jq -n \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg s5_template "$S5_TEMPLATE" \
  --arg map_template "$MAP_TEMPLATE" \
  --arg cohort_template "$COHORT_TEMPLATE" \
  --arg commercial_template "$COMMERCIAL_TEMPLATE" \
  --arg latency_template "$LATENCY_TEMPLATE" \
  --arg deploy_template "$DEPLOY_TEMPLATE" \
  '{contract_version: "trillionnium_world_public_launch_evidence_bundle_v1", status: "public_launch_evidence_bundle_green", fixture_kind: "negative_template_bundle_must_not_pass", generated_at: $generated_at, evidence_paths: {s5_real_device: $s5_template, production_map_pack_public: $map_template, first_beta_cohort: $cohort_template, commercial_launch_drill: $commercial_template, multi_node_or_live_traffic_latency: $latency_template, public_network_deploy: $deploy_template}, operator_signoff: {signed_by: "openclaw-negative-fixture", signed_at: $generated_at, real_external_evidence_confirmed: true, synthetic_or_template_data_rejected: true}}' >"$FAKE_BUNDLE"

set +e
TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH="$FAKE_BUNDLE" \
TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_SUMMARY="$BUNDLE_SUMMARY" \
TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_MD="$BUNDLE_MARKDOWN" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh" --require-ready >"$BUNDLE_LOG" 2>&1
EXIT_STATUS=$?
set -e

ACTUAL_STATUS="$(jq -r '.status // empty' "$BUNDLE_SUMMARY" 2>/dev/null || true)"
ITEM_FAILURE_COUNT="$(jq -r '(.item_failures // []) | length' "$BUNDLE_SUMMARY" 2>/dev/null || printf '0')"
EXPECTED_STATUS=public_launch_evidence_bundle_blocked_invalid_real_evidence
GREEN=false
if [[ "$EXIT_STATUS" != "0" && "$ACTUAL_STATUS" == "$EXPECTED_STATUS" && "$ITEM_FAILURE_COUNT" == "6" ]]; then
  GREEN=true
fi
STATUS=public_launch_bundle_negative_fixtures_green
if [[ "$GREEN" != "true" ]]; then
  STATUS=public_launch_bundle_negative_fixtures_blocked
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_bundle_negative_fixtures_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg fake_bundle_path "$FAKE_BUNDLE" \
  --arg bundle_summary "$BUNDLE_SUMMARY" \
  --arg bundle_log "$BUNDLE_LOG" \
  --arg kit_log "$KIT_LOG" \
  --arg expected_status "$EXPECTED_STATUS" \
  --arg actual_status "$ACTUAL_STATUS" \
  --argjson green "$GREEN" \
  --argjson exit_status "$EXIT_STATUS" \
  --argjson item_failure_count "$ITEM_FAILURE_COUNT" \
  '{contract_version: $contract_version, status: $status, generated_at: $generated_at, source_of_truth: "trillionnium_world_public_launch_bundle_negative_fixtures", green: $green, public_launch_claimed: false, android_s5_real_device_claimed: false, live_map_ingestion_performed: false, live_public_exposure_performed: false, bundle_negative_rule: "fake_green_bundle_manifest_pointing_to_no_credit_templates_must_fail_require_ready", fake_bundle_path: $fake_bundle_path, evidence_kit_log: $kit_log, bundle_validation_summary: $bundle_summary, bundle_validation_log: $bundle_log, expected_status: $expected_status, actual_status: $actual_status, validator_exit_status: $exit_status, expected_item_failure_count: 6, actual_item_failure_count: $item_failure_count}' >"$SUMMARY_FILE"

if [[ "$STATUS" == "public_launch_bundle_negative_fixtures_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BUNDLE_NEGATIVE_FIXTURES_GREEN %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BUNDLE_NEGATIVE_FIXTURES_BLOCKED %s\n' "$SUMMARY_FILE" >&2
exit 1
