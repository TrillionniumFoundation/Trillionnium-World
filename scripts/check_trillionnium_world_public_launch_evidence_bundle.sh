#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/public-launch-evidence-bundle.json"
MARKDOWN_FILE="$ACCEPTANCE_DIR/public-launch-evidence-bundle.md"
BUNDLE_PATH=""
REQUIRE_READY=0
REFRESH_KIT="${TRNM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_REFRESH_KIT:-1}"
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_SUMMARY && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_SUMMARY"
fi
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_MD && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_MD" ]]; then
  MARKDOWN_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_MD"
fi
if [[ -v TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH && -n "$TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH" ]]; then
  BUNDLE_PATH="$TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH"
fi

for arg in "$@"; do
  case "$arg" in
    --require-ready)
      REQUIRE_READY=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

# shellcheck source=scripts/release_review_acceptance_lock.sh
source "$ROOT/scripts/release_review_acceptance_lock.sh"
trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"

ITEMS_FILE="$(mktemp)"
VALIDATORS_FILE="$(mktemp)"
TMP_DIR="$(mktemp -d)"
trap 'rm -f "$ITEMS_FILE" "$VALIDATORS_FILE"; rm -rf "$TMP_DIR"' EXIT

TEMPLATE_FILE="$ACCEPTANCE_DIR/public-launch-evidence-bundle.template.json"
KIT_LOG="$ACCEPTANCE_DIR/public-launch-evidence-bundle-kit.log"

jq -n '{
  contract_version: "trillionnium_world_public_launch_evidence_bundle_v1",
  status: "template_requires_real_external_public_launch_evidence",
  acceptance_status: "public_launch_evidence_bundle_green",
  evidence_paths: {
    s5_real_device: null,
    production_map_pack_public: null,
    first_beta_cohort: null,
    commercial_launch_drill: null,
    multi_node_or_live_traffic_latency: null,
    public_network_deploy: null
  },
  operator_signoff: {
    signed_by: null,
    signed_at: null,
    real_external_evidence_confirmed: false,
    synthetic_or_template_data_rejected: true
  }
}' >"$TEMPLATE_FILE"
if [[ "$REFRESH_KIT" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" >"$KIT_LOG"
fi

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

file_status() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    printf 'present'
  else
    printf 'missing'
  fi
}

resolve_input() {
  local path="$1"
  local fallback_name="$2"
  if [[ -n "$path" ]]; then
    printf '%s' "$path"
  else
    printf '%s/%s' "$TMP_DIR" "$fallback_name"
  fi
}

run_validator() {
  local name="$1"
  shift
  local log_path="$ACCEPTANCE_DIR/public-launch-evidence-bundle-$name.log"
  set +e
  "$@" >"$log_path" 2>&1
  local exit_status=$?
  set -e
  jq -nc --arg name "$name" --arg log_path "$log_path" --argjson exit_status "$exit_status" '{name: $name, log_path: $log_path, exit_status: $exit_status}' >>"$VALIDATORS_FILE"
}

add_item() {
  local id="$1"
  local label="$2"
  local evidence_path="$3"
  local env_var="$4"
  local summary_path="$5"
  local status_expr="$6"
  local accepted_status="$7"
  local validator_name="$8"
  local actual_status path_status green
  actual_status="$(read_json_field "$summary_path" "$status_expr")"
  path_status="$(file_status "$evidence_path")"
  green=false
  if [[ "$path_status" == "present" && "$actual_status" == "$accepted_status" ]]; then
    green=true
  fi
  jq -nc \
    --arg id "$id" \
    --arg label "$label" \
    --arg evidence_path "$evidence_path" \
    --arg path_status "$path_status" \
    --arg env_var "$env_var" \
    --arg summary_path "$summary_path" \
    --arg validator_name "$validator_name" \
    --arg actual_status "$actual_status" \
    --arg accepted_status "$accepted_status" \
    --argjson green "$green" \
    '{id: $id, label: $label, green: $green, evidence_path: (if $evidence_path == "" then null else $evidence_path end), file_status: $path_status, evidence_env_var: $env_var, validator_name: $validator_name, validator_summary: $summary_path, actual_status: (if $actual_status == "" then "missing" else $actual_status end), accepted_status: $accepted_status}' >>"$ITEMS_FILE"
}

BUNDLE_FILE_STATUS="$(file_status "$BUNDLE_PATH")"
BUNDLE_CONTRACT="$(read_json_field "$BUNDLE_PATH" '.contract_version')"
BUNDLE_STATUS_RAW="$(read_json_field "$BUNDLE_PATH" '.status')"
SIGNOFF_REAL="$(read_json_field "$BUNDLE_PATH" '.operator_signoff.real_external_evidence_confirmed == true')"
SIGNOFF_REJECTS_SYNTHETIC="$(read_json_field "$BUNDLE_PATH" '.operator_signoff.synthetic_or_template_data_rejected == true')"
SIGNOFF_BY="$(read_json_field "$BUNDLE_PATH" '.operator_signoff.signed_by')"
SIGNOFF_AT="$(read_json_field "$BUNDLE_PATH" '.operator_signoff.signed_at')"

S5_PATH="$(read_json_field "$BUNDLE_PATH" '.evidence_paths.s5_real_device')"
MAP_PATH="$(read_json_field "$BUNDLE_PATH" '.evidence_paths.production_map_pack_public')"
COHORT_PATH="$(read_json_field "$BUNDLE_PATH" '.evidence_paths.first_beta_cohort')"
COMMERCIAL_PATH="$(read_json_field "$BUNDLE_PATH" '.evidence_paths.commercial_launch_drill')"
LATENCY_PATH="$(read_json_field "$BUNDLE_PATH" '.evidence_paths.multi_node_or_live_traffic_latency')"
DEPLOY_PATH="$(read_json_field "$BUNDLE_PATH" '.evidence_paths.public_network_deploy')"

S5_INPUT="$(resolve_input "$S5_PATH" missing-s5-real-device.json)"
MAP_INPUT="$(resolve_input "$MAP_PATH" missing-map-pack-public.json)"
COHORT_INPUT="$(resolve_input "$COHORT_PATH" missing-first-beta-cohort.json)"
COMMERCIAL_INPUT="$(resolve_input "$COMMERCIAL_PATH" missing-commercial-drill.json)"
LATENCY_INPUT="$(resolve_input "$LATENCY_PATH" missing-multi-node-latency.json)"
DEPLOY_INPUT="$(resolve_input "$DEPLOY_PATH" missing-public-deploy.json)"

S5_SUMMARY="$ACCEPTANCE_DIR/public-launch-evidence-bundle-s5-real-device.json"
MAP_SUMMARY="$ACCEPTANCE_DIR/public-launch-evidence-bundle-production-map-pack.json"
COHORT_COMMERCIAL_SUMMARY="$ACCEPTANCE_DIR/public-launch-evidence-bundle-cohort-commercial.json"
EXTERNAL_OPS_SUMMARY="$ACCEPTANCE_DIR/public-launch-evidence-bundle-external-ops.json"

run_validator s5_real_device env TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH="$S5_INPUT" TRILLIONNIUM_WORLD_S5_REAL_DEVICE_VALIDATION_SUMMARY="$S5_SUMMARY" "$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" --require-ready
run_validator production_map_pack env TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH="$MAP_INPUT" TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY="$MAP_SUMMARY" "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" --require-ready
run_validator cohort_commercial env TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH="$COHORT_INPUT" TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH="$COMMERCIAL_INPUT" TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_SUMMARY="$COHORT_COMMERCIAL_SUMMARY" "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh" --require-ready
run_validator external_ops env TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH="$LATENCY_INPUT" TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH="$DEPLOY_INPUT" TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_SUMMARY="$EXTERNAL_OPS_SUMMARY" "$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" --require-ready

add_item s5_real_device "S5 Android real-device evidence" "$S5_PATH" TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH "$S5_SUMMARY" '.status' s5_real_device_evidence_green s5_real_device
add_item production_map_pack_public "Production map-pack public evidence" "$MAP_PATH" TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH "$MAP_SUMMARY" '.status' production_map_pack_public_ready_green production_map_pack
add_item first_beta_cohort "First beta cohort evidence" "$COHORT_PATH" TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH "$COHORT_COMMERCIAL_SUMMARY" '.first_beta.status' first_beta_cohort_evidence_green cohort_commercial
add_item commercial_launch_drill "Commercial launch drill evidence" "$COMMERCIAL_PATH" TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH "$COHORT_COMMERCIAL_SUMMARY" '.commercial_launch_drill.status' commercial_launch_drill_evidence_green cohort_commercial
add_item multi_node_or_live_traffic_latency "Multi-node or live-traffic latency evidence" "$LATENCY_PATH" TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH "$EXTERNAL_OPS_SUMMARY" '.multi_node_or_live_traffic_latency.status' multi_node_or_live_traffic_latency_green external_ops
add_item public_network_deploy "Public network deploy evidence" "$DEPLOY_PATH" TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH "$EXTERNAL_OPS_SUMMARY" '.public_network_deploy.status' public_network_deploy_green external_ops

ITEMS_JSON="$(jq -s '.' "$ITEMS_FILE")"
VALIDATORS_JSON="$(jq -s '.' "$VALIDATORS_FILE")"
ITEM_FAILURES_JSON="$(jq -c '[.[] | select(.green != true)]' <<<"$ITEMS_JSON")"
ITEM_FAILURE_COUNT="$(jq 'length' <<<"$ITEM_FAILURES_JSON")"
BUNDLE_SIGNOFF_OK=false
if [[ "$SIGNOFF_REAL" == "true" && "$SIGNOFF_REJECTS_SYNTHETIC" == "true" && -n "$SIGNOFF_BY" && -n "$SIGNOFF_AT" ]]; then
  BUNDLE_SIGNOFF_OK=true
fi
BUNDLE_METADATA_OK=false
if [[ "$BUNDLE_FILE_STATUS" == "present" && "$BUNDLE_CONTRACT" == "trillionnium_world_public_launch_evidence_bundle_v1" && "$BUNDLE_STATUS_RAW" == "public_launch_evidence_bundle_green" && "$BUNDLE_SIGNOFF_OK" == "true" ]]; then
  BUNDLE_METADATA_OK=true
fi
BUNDLE_GREEN=false
if [[ "$BUNDLE_METADATA_OK" == "true" && "$ITEM_FAILURE_COUNT" == "0" ]]; then
  BUNDLE_GREEN=true
fi

STATUS=public_launch_evidence_bundle_ready_for_real_evidence
if [[ "$BUNDLE_GREEN" == "true" ]]; then
  STATUS=public_launch_evidence_bundle_green
elif [[ "$BUNDLE_FILE_STATUS" == "present" ]]; then
  STATUS=public_launch_evidence_bundle_blocked_invalid_real_evidence
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_evidence_bundle_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg bundle_path "$BUNDLE_PATH" \
  --arg bundle_file_status "$BUNDLE_FILE_STATUS" \
  --arg bundle_contract "$BUNDLE_CONTRACT" \
  --arg bundle_status_raw "$BUNDLE_STATUS_RAW" \
  --arg template_path "$TEMPLATE_FILE" \
  --arg template_sha256 "$(sha256sum "$TEMPLATE_FILE" | awk '{print $1}')" \
  --arg markdown_path "$MARKDOWN_FILE" \
  --arg kit_log "$KIT_LOG" \
  --argjson bundle_green "$BUNDLE_GREEN" \
  --argjson bundle_metadata_ok "$BUNDLE_METADATA_OK" \
  --argjson bundle_signoff_ok "$BUNDLE_SIGNOFF_OK" \
  --argjson evidence_items "$ITEMS_JSON" \
  --argjson item_failures "$ITEM_FAILURES_JSON" \
  --argjson validators "$VALIDATORS_JSON" \
  '{contract_version: $contract_version, status: $status, generated_at: $generated_at, source_of_truth: "trillionnium_world_public_launch_evidence_bundle_gate", green: $bundle_green, public_launch_ready: $bundle_green, public_launch_claimed: false, android_s5_real_device_claimed: false, live_map_ingestion_performed_by_this_script: false, live_public_exposure_performed_by_this_script: false, bundle_rule: "single_manifest_must_point_to_real_external_evidence_that_passes_all_field_validators_before_public_launch_credit", evidence_bundle: {path: (if $bundle_path == "" then null else $bundle_path end), file_status: $bundle_file_status, contract_version: $bundle_contract, status: $bundle_status_raw, metadata_ok: $bundle_metadata_ok, signoff_ok: $bundle_signoff_ok}, template: {path: $template_path, sha256: $template_sha256, public_launch_credit: false}, markdown_path: $markdown_path, evidence_kit_log: $kit_log, evidence_items: $evidence_items, item_failures: $item_failures, validators: $validators}' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World Public Launch Evidence Bundle\n\n'
  printf -- '- status: %s\n' "$STATUS"
  printf -- '- public_launch_ready: %s\n' "$BUNDLE_GREEN"
  printf -- '- bundle_path: %s\n' "${BUNDLE_PATH:-missing}"
  printf -- '- template: %s\n\n' "$TEMPLATE_FILE"
  printf '## Evidence Items\n\n'
  jq -r '.evidence_items[] | "- " + .id + ": " + .actual_status + " (accepted: " + .accepted_status + ")"' "$SUMMARY_FILE"
  printf '\n## Boundary\n\n'
  printf -- '- This script validates a manifest only; it does not collect real external evidence.\n'
  printf -- '- Public launch credit requires the bundle status and all six field validators to be green.\n'
} >"$MARKDOWN_FILE"

case "$STATUS" in
  public_launch_evidence_bundle_green)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_GREEN %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
    if [[ "$REQUIRE_READY" -eq 1 ]]; then
      exit 1
    fi
    ;;
esac
