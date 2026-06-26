#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MAP_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/public-launch-evidence-kit.json"
MARKDOWN_FILE="$ACCEPTANCE_DIR/public-launch-evidence-kit.md"
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_SUMMARY && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_SUMMARY"
fi
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_MD && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_MD" ]]; then
  MARKDOWN_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_MD"
fi

# shellcheck source=scripts/release_review_acceptance_lock.sh
source "$ROOT/scripts/release_review_acceptance_lock.sh"
trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"

ITEMS_FILE="$(mktemp)"
trap 'rm -f "$ITEMS_FILE"' EXIT

mkdir -p "$ACCEPTANCE_DIR" "$S5_DIR" "$MAP_DIR"

S5_LOG="$ACCEPTANCE_DIR/public-launch-evidence-kit-s5.log"
MAP_LOG="$ACCEPTANCE_DIR/public-launch-evidence-kit-map-pack.log"
COHORT_SCHEMA_LOG="$ACCEPTANCE_DIR/public-launch-evidence-kit-cohort-schema.log"
EXTERNAL_OPS_LOG="$ACCEPTANCE_DIR/public-launch-evidence-kit-external-ops.log"
INTAKE_LOG="$ACCEPTANCE_DIR/public-launch-evidence-kit-intake.log"

"$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" >"$S5_LOG"
"$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" >"$MAP_LOG"
"$ROOT/scripts/check_trillionnium_world_cohort_commercial_schema.sh" >"$COHORT_SCHEMA_LOG"
"$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" >"$EXTERNAL_OPS_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" >"$INTAKE_LOG"

INTAKE_SUMMARY="$ACCEPTANCE_DIR/public-launch-evidence-intake.json"
S5_SUMMARY="$S5_DIR/s5-real-device-evidence-validation.json"
MAP_SUMMARY="$MAP_DIR/production-map-pack-public-evidence.json"
EXTERNAL_OPS_SUMMARY="$ACCEPTANCE_DIR/external-ops-evidence.json"
COHORT_COMMERCIAL_SUMMARY="$ACCEPTANCE_DIR/cohort-commercial-evidence.json"

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

template_sha() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    sha256sum "$path" | awk '{print $1}'
  fi
}

add_item() {
  local id="$1"
  local blocker_id="$2"
  local label="$3"
  local env_var="$4"
  local template_path="$5"
  local validator_command="$6"
  local validator_summary="$7"
  local accepted_status="$8"
  local current_status="$9"
  shift 9
  local collection_requirement="$*"
  local collection_command="fill_real_evidence_template_then_run_validator"
  if [[ "$id" == "s5_android_real_device_matrix" ]]; then
    collection_command="ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device"
  elif [[ "$id" == "production_map_pack_public_evidence" ]]; then
    collection_command="scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh"
  elif [[ "$id" == "first_beta_cohort_evidence" || "$id" == "commercial_launch_drill_evidence" ]]; then
    collection_command="scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh"
  elif [[ "$id" == "multi_node_or_live_traffic_latency_evidence" || "$id" == "public_network_live_exposure_evidence" ]]; then
    collection_command="scripts/check_trillionnium_world_external_ops_evidence_collection.sh"
  fi

  local template_file_status template_status template_hash template_ok
  template_file_status="$(file_status "$template_path")"
  template_status="$(read_json_field "$template_path" '.status')"
  template_hash="$(template_sha "$template_path")"
  template_ok=false
  if [[ "$template_file_status" == "present" && -n "$template_status" && "$template_status" != "$accepted_status" ]]; then
    template_ok=true
  fi

  jq -nc \
    --arg id "$id" \
    --arg blocker_id "$blocker_id" \
    --arg label "$label" \
    --arg env_var "$env_var" \
    --arg template_path "$template_path" \
    --arg template_file_status "$template_file_status" \
    --arg template_status "$template_status" \
    --arg template_sha256 "$template_hash" \
    --arg validator_command "$validator_command" \
    --arg validator_summary "$validator_summary" \
    --arg accepted_status "$accepted_status" \
    --arg current_status "$current_status" \
    --arg collection_requirement "$collection_requirement" \
    --arg collection_command "$collection_command" \
    --argjson template_ok "$template_ok" \
    '{id: $id, blocker_id: $blocker_id, label: $label, evidence_env_var: $env_var, template_path: $template_path, template_file_status: $template_file_status, template_status: (if $template_status == "" then null else $template_status end), template_sha256: (if $template_sha256 == "" then null else $template_sha256 end), template_ok: $template_ok, collection_command: $collection_command, validator_command: $validator_command, validator_summary: $validator_summary, accepted_status: $accepted_status, current_status: (if $current_status == "" then "missing" else $current_status end), collection_requirement: $collection_requirement, template_public_launch_credit: false}' >>"$ITEMS_FILE"
}

add_item s5_android_real_device_matrix s5_real_device_matrix "S5 Android real-device matrix" ANDROID_SERIAL "$S5_DIR/s5-device-evidence.template.json" "TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH=<real-s5-evidence.json> scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready" "$S5_SUMMARY" s5_real_device_evidence_green "$(read_json_field "$S5_SUMMARY" '.status')" "Attach a USB-debugging Android device, confirm adb shows it as device, run scripts/check_trillionnium_world_s5_device_evidence.sh --require-device, set TRILLIONNIUM_WORLD_S5_WEAK_NETWORK_EVIDENCE_PATH for the real weak-network run, then validate the generated real Android serial, adb devices output, screenshot, gfxinfo/frame stats, logcat, lifecycle, CJK/input, weak-network, APK resource/signature, crash-free window, native library, and symbol evidence."
add_item production_map_pack_public_evidence production_map_pack_public_evidence "Production map-pack public evidence" TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH "$MAP_DIR/production-map-pack-public-evidence.template.json" "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready" "$MAP_SUMMARY" production_map_pack_public_ready_green "$(read_json_field "$MAP_SUMMARY" '.status')" "Run scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh, then fill the map-pack template with approved source, ODbL/license, cache/offline policy, attribution screenshots, sensitive POI filter, geofence, key custody, distribution revocation, rollback, and operator signoff."
add_item first_beta_cohort_evidence first_beta_cohort_evidence "First beta cohort evidence" TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH "$ACCEPTANCE_DIR/first-beta-cohort-evidence.template.json" "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready" "$COHORT_COMMERCIAL_SUMMARY" first_beta_cohort_evidence_green "$(read_json_field "$COHORT_COMMERCIAL_SUMMARY" '.first_beta.status')" "Run scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh, then fill the first-beta template with real 5-10 participant records, sessions covering the participants, feedback summary, and operator signoff."
add_item commercial_launch_drill_evidence commercial_launch_drill_evidence "Commercial launch drill evidence" TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH "$ACCEPTANCE_DIR/commercial-launch-drill-evidence.template.json" "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready" "$COHORT_COMMERCIAL_SUMMARY" commercial_launch_drill_evidence_green "$(read_json_field "$COHORT_COMMERCIAL_SUMMARY" '.commercial_launch_drill.status')" "Run scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh, then fill the commercial drill template with payment, refund, support, legal, operator, traffic, real-or-sanitized drill signoff, and synthetic/template rejection evidence."
add_item multi_node_or_live_traffic_latency_evidence multi_node_or_live_traffic_latency_evidence "Multi-node or live-traffic latency evidence" TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH "$ACCEPTANCE_DIR/multi-node-latency-evidence.template.json" "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready" "$EXTERNAL_OPS_SUMMARY" multi_node_or_live_traffic_latency_green "$(read_json_field "$EXTERNAL_OPS_SUMMARY" '.multi_node_or_live_traffic_latency.status')" "Run scripts/check_trillionnium_world_external_ops_evidence_collection.sh, then fill the latency template with multi-node or live public traffic scope, at least 3 endpoints, at least 3 public URL probes, p95 within budget, monitoring timeseries, rollback-under-load, and operator signoff."
add_item public_network_live_exposure_evidence public_network_live_exposure_evidence "Public network live exposure evidence" TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH "$ACCEPTANCE_DIR/public-network-deploy-evidence.template.json" "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready" "$EXTERNAL_OPS_SUMMARY" public_network_deploy_green "$(read_json_field "$EXTERNAL_OPS_SUMMARY" '.public_network_deploy.status')" "Run scripts/check_trillionnium_world_external_ops_evidence_collection.sh, then fill the public deploy template with approved public exposure, host, domain, public URL, TLS, probes, monitoring, backup, rollback, and operator signoff."

ITEMS_JSON="$(jq -s '.' "$ITEMS_FILE")"
TEMPLATE_FAILURES_JSON="$(jq -c '[.[] | select(.template_ok != true)]' <<<"$ITEMS_JSON")"
EVIDENCE_ITEM_COUNT="$(jq 'length' <<<"$ITEMS_JSON")"
READY_TEMPLATE_COUNT="$(jq '[.[] | select(.template_ok == true)] | length' <<<"$ITEMS_JSON")"
TEMPLATE_FAILURE_COUNT="$(jq 'length' <<<"$TEMPLATE_FAILURES_JSON")"
REFRESH_LOG_COUNT=5
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$INTAKE_SUMMARY" 2>/dev/null || printf 'false')"
NEEDS_COLLECTION_COUNT="$(jq -r '(.needs_collection // []) | length' "$INTAKE_SUMMARY" 2>/dev/null || printf '6')"

STATUS=public_launch_evidence_kit_ready_for_operator_collection
GREEN=true
if [[ "$TEMPLATE_FAILURE_COUNT" != "0" ]]; then
  STATUS=public_launch_evidence_kit_blocked_template_drift
  GREEN=false
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_evidence_kit_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg markdown_path "$MARKDOWN_FILE" \
  --arg intake_summary "$INTAKE_SUMMARY" \
  --arg s5_log "$S5_LOG" \
  --arg map_log "$MAP_LOG" \
  --arg cohort_schema_log "$COHORT_SCHEMA_LOG" \
  --arg external_ops_log "$EXTERNAL_OPS_LOG" \
  --arg intake_log "$INTAKE_LOG" \
  --argjson green "$GREEN" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson needs_collection_count "$NEEDS_COLLECTION_COUNT" \
  --argjson evidence_item_count "$EVIDENCE_ITEM_COUNT" \
  --argjson ready_template_count "$READY_TEMPLATE_COUNT" \
  --argjson template_failure_count "$TEMPLATE_FAILURE_COUNT" \
  --argjson refresh_log_count "$REFRESH_LOG_COUNT" \
  --argjson evidence_items "$ITEMS_JSON" \
  --argjson template_failures "$TEMPLATE_FAILURES_JSON" \
  '{contract_version: $contract_version, status: $status, generated_at: $generated_at, source_of_truth: "trillionnium_world_public_launch_evidence_kit", green: $green, public_launch_ready: $public_launch_ready, public_launch_claimed: false, android_s5_real_device_claimed: false, live_map_ingestion_performed: false, live_public_exposure_performed: false, kit_rule: "operator_templates_must_exist_and_must_not_claim_green_until_real_external_evidence_passes_field_validators", markdown_path: $markdown_path, intake_summary: $intake_summary, refresh_logs: {s5_real_device: $s5_log, production_map_pack: $map_log, cohort_schema: $cohort_schema_log, external_ops: $external_ops_log, evidence_intake: $intake_log}, needs_collection_count: $needs_collection_count, evidence_item_count: $evidence_item_count, ready_template_count: $ready_template_count, template_failure_count: $template_failure_count, refresh_log_count: $refresh_log_count, evidence_items: $evidence_items, template_failures: $template_failures, reviewer_next_action: (if $public_launch_ready then "review_public_launch_ready_evidence" else "collect_real_external_public_launch_evidence_using_templates" end)}' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World Public Launch Evidence Kit\n\n'
  printf -- '- status: %s\n' "$STATUS"
  printf -- '- public_launch_ready: %s\n' "$PUBLIC_LAUNCH_READY"
  printf -- '- public_launch_claimed: false\n'
  printf -- '- android_s5_real_device_claimed: false\n\n'
  printf -- '- evidence_item_count: %s\n' "$EVIDENCE_ITEM_COUNT"
  printf -- '- ready_template_count: %s\n' "$READY_TEMPLATE_COUNT"
  printf -- '- template_failure_count: %s\n' "$TEMPLATE_FAILURE_COUNT"
  printf -- '- refresh_log_count: %s\n\n' "$REFRESH_LOG_COUNT"
  printf '## Evidence Templates\n\n'
  jq -r '.evidence_items[] | "- " + .id + ": " + .template_path + "\n  - env: " + .evidence_env_var + "\n  - accepted_status: " + .accepted_status + "\n  - current_status: " + .current_status + "\n  - template_status: " + (.template_status // "missing") + "\n  - collect: " + .collection_command + "\n  - validator: " + .validator_command + "\n  - requirement: " + .collection_requirement' "$SUMMARY_FILE"
  printf '\n## Boundary\n\n'
  printf -- '- Templates are collection scaffolding only and carry no public-launch credit.\n'
  printf -- '- Public launch stays blocked until each real evidence file passes its field-level validator.\n'
} >"$MARKDOWN_FILE"

if [[ "$STATUS" == "public_launch_evidence_kit_ready_for_operator_collection" ]]; then
  printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_READY %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_KIT_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
exit 1
