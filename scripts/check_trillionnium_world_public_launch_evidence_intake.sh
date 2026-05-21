#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_SUMMARY:-$ACCEPTANCE_DIR/public-launch-evidence-intake.json}"
MARKDOWN_FILE="${TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_MD:-$ACCEPTANCE_DIR/public-launch-evidence-intake.md}"
PUBLIC_LAUNCH_SUMMARY="$ACCEPTANCE_DIR/public-launch-readiness.json"
PUBLIC_LAUNCH_LOG="$ACCEPTANCE_DIR/public-launch-evidence-intake-readiness.log"
ITEMS_FILE="$(mktemp)"
REQUIRE_COMPLETE=0
trap 'rm -f "$ITEMS_FILE"' EXIT

for arg in "$@"; do
  case "$arg" in
    --require-complete)
      REQUIRE_COMPLETE=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$ACCEPTANCE_DIR"

"$ROOT/scripts/check_trillionnium_world_public_launch_readiness.sh" >"$PUBLIC_LAUNCH_LOG"

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

blocker_present() {
  local blocker="$1"
  jq -e --arg blocker "$blocker" '(.blockers // []) | index($blocker) != null' "$PUBLIC_LAUNCH_SUMMARY" >/dev/null
}

add_item() {
  local id="$1"
  local label="$2"
  local current_status="$3"
  local evidence_path="$4"
  local file_status_value="$5"
  local accepted_status="$6"
  local env_var="$7"
  local template_path="$8"
  local collection_requirement="$9"
  local green="${10}"
  local blocker_id="${11}"
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
  local blocked_by_gate=false
  if blocker_present "$blocker_id"; then
    blocked_by_gate=true
  fi

  jq -nc \
    --arg id "$id" \
    --arg label "$label" \
    --arg current_status "$current_status" \
    --arg evidence_path "$evidence_path" \
    --arg file_status "$file_status_value" \
    --arg accepted_status "$accepted_status" \
    --arg env_var "$env_var" \
    --arg template_path "$template_path" \
    --arg collection_requirement "$collection_requirement" \
    --arg collection_command "$collection_command" \
    --arg blocker_id "$blocker_id" \
    --argjson green "$green" \
    --argjson blocked_by_gate "$blocked_by_gate" \
    '{
      id: $id,
      label: $label,
      green: $green,
      blocked_by_public_launch_gate: $blocked_by_gate,
      blocker_id: $blocker_id,
      current_status: (if $current_status == "" then "missing" else $current_status end),
      accepted_status: $accepted_status,
      evidence_path: (if $evidence_path == "" then null else $evidence_path end),
      file_status: $file_status,
      evidence_env_var: (if $env_var == "" then null else $env_var end),
      template_path: (if $template_path == "" then null else $template_path end),
      collection_requirement: $collection_requirement,
      collection_command: $collection_command
    }' >>"$ITEMS_FILE"
}

PUBLIC_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.overall_status')"
PUBLIC_LAUNCH_READY="$(jq -r '(.overall_status == "ready_for_public_launch_review")' "$PUBLIC_LAUNCH_SUMMARY" 2>/dev/null || printf 'false')"
if [[ -z "$PUBLIC_LAUNCH_READY" ]]; then
  PUBLIC_LAUNCH_READY=false
fi
BLOCKERS_JSON="$(jq -c '.blockers // []' "$PUBLIC_LAUNCH_SUMMARY")"

S5_PATH="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.s5_real_device_matrix.evidence_path')"
S5_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.s5_real_device_matrix.status')"
S5_GREEN=false
if [[ "$S5_STATUS" == "real_device_evidence_green" || "$S5_STATUS" == "ready" ]]; then
  S5_GREEN=true
fi
add_item \
  s5_android_real_device_matrix \
  "S5 Android real-device matrix" \
  "$S5_STATUS" \
  "$S5_PATH" \
  "$(file_status "$S5_PATH")" \
  real_device_evidence_green \
  ANDROID_SERIAL \
  "$ROOT/acceptance/S5_native_bevy_device/latest/s5-device-evidence.template.json" \
	  "Run scripts/check_trillionnium_world_s5_device_evidence.sh --require-device on a real Android device and attach launch, screenshot, gfxinfo/frame, CJK/input, lifecycle, weak-network, APK resource/signature, and crash-free logcat evidence." \
  "$S5_GREEN" \
  s5_real_device_matrix

MAP_PATH="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.production_map_pack.evidence_path')"
MAP_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.production_map_pack.status')"
MAP_FILE_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.production_map_pack.file_status')"
MAP_GREEN=false
if [[ "$MAP_STATUS" == "production_map_pack_public_ready_green" ]]; then
  MAP_GREEN=true
fi
add_item \
  production_map_pack_public_evidence \
  "Production map-pack public evidence" \
  "$MAP_STATUS" \
  "$MAP_PATH" \
  "${MAP_FILE_STATUS:-$(file_status "$MAP_PATH")}" \
  production_map_pack_public_ready_green \
  TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH \
  "$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.template.json" \
  "Run scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh, then attach approved source, license/ODbL, offline/cache policy, attribution screenshots, sensitive POI, geofence, key custody, distribution revocation, rollback, and operator signoff evidence; live ingestion stays disabled." \
  "$MAP_GREEN" \
  production_map_pack_public_evidence

COHORT_PATH="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.first_beta_cohort.evidence_path')"
COHORT_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.first_beta_cohort.status')"
COHORT_FILE_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.first_beta_cohort.file_status')"
COHORT_GREEN=false
if [[ "$COHORT_STATUS" == "first_beta_cohort_evidence_green" ]]; then
  COHORT_GREEN=true
fi
add_item \
  first_beta_cohort_evidence \
  "First beta cohort evidence" \
  "$COHORT_STATUS" \
  "$COHORT_PATH" \
  "${COHORT_FILE_STATUS:-$(file_status "$COHORT_PATH")}" \
  first_beta_cohort_evidence_green \
  TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH \
  "$ROOT/acceptance/S6_public_launch/latest/first-beta-cohort-evidence.template.json" \
  "Run scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh, then attach real 5-10 participant cohort sessions and feedback; template or synthetic participant data must not claim green." \
  "$COHORT_GREEN" \
  first_beta_cohort_evidence

COMMERCIAL_PATH="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.commercial_launch_drill.evidence_path')"
COMMERCIAL_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.commercial_launch_drill.status')"
COMMERCIAL_FILE_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.commercial_launch_drill.file_status')"
COMMERCIAL_GREEN=false
if [[ "$COMMERCIAL_STATUS" == "commercial_launch_drill_evidence_green" ]]; then
  COMMERCIAL_GREEN=true
fi
add_item \
  commercial_launch_drill_evidence \
  "Commercial launch drill evidence" \
  "$COMMERCIAL_STATUS" \
  "$COMMERCIAL_PATH" \
  "${COMMERCIAL_FILE_STATUS:-$(file_status "$COMMERCIAL_PATH")}" \
  commercial_launch_drill_evidence_green \
  TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH \
  "$ROOT/acceptance/S6_public_launch/latest/commercial-launch-drill-evidence.template.json" \
  "Run scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh, then attach real or sanitized payment, refund, support, legal, operator, and traffic drill evidence." \
  "$COMMERCIAL_GREEN" \
  commercial_launch_drill_evidence

LATENCY_PATH="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.multi_node_or_live_traffic_latency.evidence_path')"
LATENCY_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.multi_node_or_live_traffic_latency.status')"
LATENCY_FILE_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.multi_node_or_live_traffic_latency.file_status')"
LATENCY_GREEN=false
if [[ "$LATENCY_STATUS" == "multi_node_or_live_traffic_latency_green" ]]; then
  LATENCY_GREEN=true
fi
add_item \
  multi_node_or_live_traffic_latency_evidence \
  "Multi-node or live-traffic latency evidence" \
  "$LATENCY_STATUS" \
  "$LATENCY_PATH" \
  "${LATENCY_FILE_STATUS:-$(file_status "$LATENCY_PATH")}" \
  multi_node_or_live_traffic_latency_green \
  TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH \
  "$ROOT/acceptance/S6_public_launch/latest/multi-node-latency-evidence.template.json" \
  "Run scripts/check_trillionnium_world_external_ops_evidence_collection.sh, then attach multi-node release latency or live public traffic latency evidence with public URL probe samples, monitoring timeseries, and rollback-under-load proof; local latency drill alone is not enough." \
  "$LATENCY_GREEN" \
  multi_node_or_live_traffic_latency_evidence

DEPLOY_PATH="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.public_network_deploy.evidence_path')"
DEPLOY_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.public_network_deploy.status')"
DEPLOY_FILE_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.gates.public_network_deploy.file_status')"
DEPLOY_GREEN=false
if [[ "$DEPLOY_STATUS" == "public_network_deploy_green" ]]; then
  DEPLOY_GREEN=true
fi
add_item \
  public_network_live_exposure_evidence \
  "Public network live exposure evidence" \
  "$DEPLOY_STATUS" \
  "$DEPLOY_PATH" \
  "${DEPLOY_FILE_STATUS:-$(file_status "$DEPLOY_PATH")}" \
  public_network_deploy_green \
  TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH \
  "$ROOT/acceptance/S6_public_launch/latest/public-deploy-runbook.md" \
  "Run scripts/check_trillionnium_world_external_ops_evidence_collection.sh, then attach approved host, domain/TLS, monitoring, backup, rollback, and public URL probe evidence; local deploy drill alone is not public exposure proof." \
  "$DEPLOY_GREEN" \
  public_network_live_exposure_evidence

ITEMS_JSON="$(jq -s '.' "$ITEMS_FILE")"
NEEDS_COLLECTION_JSON="$(jq -c '[.[] | select(.green != true)]' <<<"$ITEMS_JSON")"
NEEDS_COLLECTION_COUNT="$(jq 'length' <<<"$NEEDS_COLLECTION_JSON")"
KNOWN_BLOCKERS_JSON='["s5_real_device_matrix","production_map_pack_public_evidence","first_beta_cohort_evidence","commercial_launch_drill_evidence","multi_node_or_live_traffic_latency_evidence","public_network_live_exposure_evidence"]'
UNKNOWN_BLOCKERS_JSON="$(jq -c --argjson known "$KNOWN_BLOCKERS_JSON" '(.blockers // []) | map(select(($known | index(.)) == null))' "$PUBLIC_LAUNCH_SUMMARY")"
UNKNOWN_BLOCKER_COUNT="$(jq 'length' <<<"$UNKNOWN_BLOCKERS_JSON")"

STATUS=public_launch_evidence_intake_ready_for_operator_collection
COMPLETE=false
if [[ "$NEEDS_COLLECTION_COUNT" == "0" && "$PUBLIC_LAUNCH_READY" == "true" ]]; then
  STATUS=public_launch_evidence_intake_complete_green
  COMPLETE=true
elif [[ "$UNKNOWN_BLOCKER_COUNT" != "0" ]]; then
  STATUS=public_launch_evidence_intake_blocked_unknown_requirements
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_evidence_intake_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg public_launch_readiness "$PUBLIC_LAUNCH_SUMMARY" \
  --arg public_launch_log "$PUBLIC_LAUNCH_LOG" \
  --arg public_launch_status "$PUBLIC_STATUS" \
  --arg markdown_path "$MARKDOWN_FILE" \
  --argjson complete "$COMPLETE" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson blockers "$BLOCKERS_JSON" \
  --argjson unknown_blockers "$UNKNOWN_BLOCKERS_JSON" \
  --argjson evidence_items "$ITEMS_JSON" \
  --argjson needs_collection "$NEEDS_COLLECTION_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_public_launch_evidence_intake",
    public_launch_readiness_summary: $public_launch_readiness,
    public_launch_readiness_log: $public_launch_log,
    public_launch_readiness_status: $public_launch_status,
    markdown_path: $markdown_path,
    complete: $complete,
    public_launch_ready: $public_launch_ready,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    intake_rule: "collect_real_external_public_launch_evidence_without_claiming_public_launch_ready_or_android_s5_real_device_ready",
    blockers: $blockers,
    unknown_blockers: $unknown_blockers,
    evidence_items: $evidence_items,
    needs_collection: $needs_collection,
    reviewer_next_action: (if $complete then "review_public_launch_ready_evidence" else "collect_evidence_items_in_needs_collection" end)
  }' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World Public Launch Evidence Intake\n\n'
  printf -- '- status: `%s`\n' "$STATUS"
  printf -- '- public_launch_ready: `%s`\n' "$PUBLIC_LAUNCH_READY"
  printf -- '- public_launch_claimed: `false`\n'
  printf -- '- android_s5_real_device_claimed: `false`\n'
  printf -- '- live_map_ingestion_performed: `false`\n'
  printf -- '- live_public_exposure_performed: `false`\n\n'
  printf '## Evidence To Collect\n\n'
  jq -r '.needs_collection[] | "- [ ] \(.label) (`\(.accepted_status)`): \(.collection_requirement)\n  - env: `\(.evidence_env_var // "n/a")`\n  - current_status: `\(.current_status)`\n  - evidence_path: `\(.evidence_path // "n/a")`\n  - collect: `\(.collection_command)`\n  - template_path: `\(.template_path // "n/a")`"' "$SUMMARY_FILE"
  printf '\n## Evidence Already Green\n\n'
  jq -r 'if ([.evidence_items[] | select(.green == true)] | length) == 0 then "- [ ] No external public-launch evidence item is green yet." else .evidence_items[] | select(.green == true) | "- [x] \(.label): \(.current_status)" end' "$SUMMARY_FILE"
  printf '\n## Boundary\n\n'
  printf -- '- This is an intake/checklist artifact, not a public-launch approval.\n'
  printf -- '- Live map ingestion and live public exposure are not performed by this script.\n'
} >"$MARKDOWN_FILE"

case "$STATUS" in
  public_launch_evidence_intake_complete_green)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_COMPLETE %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
    ;;
  public_launch_evidence_intake_ready_for_operator_collection)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_READY_FOR_OPERATOR_COLLECTION %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    if [[ "$REQUIRE_COMPLETE" -eq 1 ]]; then
      exit 1
    fi
    exit 0
    ;;
esac

if [[ "$REQUIRE_COMPLETE" -eq 1 && "$STATUS" != "public_launch_evidence_intake_complete_green" ]]; then
  exit 1
fi
