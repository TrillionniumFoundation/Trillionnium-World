#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/cohort-commercial-evidence-collection.json"
MARKDOWN_FILE="$ACCEPTANCE_DIR/cohort-commercial-evidence-collection.md"
SCHEMA_LOG="$ACCEPTANCE_DIR/cohort-commercial-evidence-collection-schema.log"
VALIDATOR_LOG="$ACCEPTANCE_DIR/cohort-commercial-evidence-collection-validator.log"
SCHEMA_SUMMARY="$ACCEPTANCE_DIR/cohort-commercial-evidence-schema.json"
VALIDATOR_SUMMARY="$ACCEPTANCE_DIR/cohort-commercial-evidence.json"
COHORT_TEMPLATE="$ACCEPTANCE_DIR/first-beta-cohort-evidence.template.json"
COMMERCIAL_TEMPLATE="$ACCEPTANCE_DIR/commercial-launch-drill-evidence.template.json"
COHORT_SCHEMA="$ACCEPTANCE_DIR/first-beta-cohort-evidence.schema.json"
COMMERCIAL_SCHEMA="$ACCEPTANCE_DIR/commercial-launch-drill-evidence.schema.json"

mkdir -p "$ACCEPTANCE_DIR"

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_COLLECTION_FAILED missing command: %s\n' "$name" >&2
    exit 1
  fi
}

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

file_sha256() {
  local path="$1"
  if [[ -f "$path" ]]; then
    sha256sum "$path" | awk '{print $1}'
  fi
}

require_cmd jq
require_cmd sha256sum

bash "$ROOT/scripts/check_trillionnium_world_cohort_commercial_schema.sh" >"$SCHEMA_LOG" 2>&1
bash "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh" >"$VALIDATOR_LOG" 2>&1 || true

SCHEMA_STATUS="$(read_json_field "$SCHEMA_SUMMARY" '.status')"
COHORT_STATUS="$(read_json_field "$VALIDATOR_SUMMARY" '.first_beta.status')"
COMMERCIAL_STATUS="$(read_json_field "$VALIDATOR_SUMMARY" '.commercial_launch_drill.status')"
STATUS="cohort_commercial_evidence_collection_ready"
if [[ "$SCHEMA_STATUS" != "cohort_commercial_evidence_schema_green" || ! -f "$COHORT_TEMPLATE" || ! -f "$COMMERCIAL_TEMPLATE" ]]; then
  STATUS="cohort_commercial_evidence_collection_blocked"
fi
COLLECTION_GREEN=false
if [[ "$STATUS" == "cohort_commercial_evidence_collection_ready" ]]; then
  COLLECTION_GREEN=true
fi
BLOCKED_VALIDATOR_STATUS_COUNT=0
if [[ "$COHORT_STATUS" != "first_beta_cohort_evidence_green" ]]; then
  BLOCKED_VALIDATOR_STATUS_COUNT=$((BLOCKED_VALIDATOR_STATUS_COUNT + 1))
fi
if [[ "$COMMERCIAL_STATUS" != "commercial_launch_drill_evidence_green" ]]; then
  BLOCKED_VALIDATOR_STATUS_COUNT=$((BLOCKED_VALIDATOR_STATUS_COUNT + 1))
fi

jq -n \
  --arg contract_version "trillionnium_world_cohort_commercial_evidence_collection_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson collection_green "$COLLECTION_GREEN" \
  --argjson blocked_validator_status_count "$BLOCKED_VALIDATOR_STATUS_COUNT" \
  --arg schema_summary "$SCHEMA_SUMMARY" \
  --arg schema_status "$SCHEMA_STATUS" \
  --arg schema_log "$SCHEMA_LOG" \
  --arg validator_summary "$VALIDATOR_SUMMARY" \
  --arg validator_log "$VALIDATOR_LOG" \
  --arg cohort_status "$COHORT_STATUS" \
  --arg commercial_status "$COMMERCIAL_STATUS" \
  --arg cohort_template "$COHORT_TEMPLATE" \
  --arg commercial_template "$COMMERCIAL_TEMPLATE" \
  --arg cohort_schema "$COHORT_SCHEMA" \
  --arg commercial_schema "$COMMERCIAL_SCHEMA" \
  --arg cohort_template_sha256 "$(file_sha256 "$COHORT_TEMPLATE")" \
  --arg commercial_template_sha256 "$(file_sha256 "$COMMERCIAL_TEMPLATE")" \
  --arg cohort_schema_sha256 "$(file_sha256 "$COHORT_SCHEMA")" \
  --arg commercial_schema_sha256 "$(file_sha256 "$COMMERCIAL_SCHEMA")" \
  --arg markdown_path "$MARKDOWN_FILE" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_cohort_commercial_evidence_collection",
    green: $collection_green,
    public_launch_credit: false,
    first_beta_ready: false,
    commercial_launch_drill_ready: false,
    template_count: 2,
    template_schema_count: 2,
    required_evidence_count: 11,
    validator_status_count: 2,
    blocked_validator_status_count: $blocked_validator_status_count,
    collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready",
    schema: {
      summary: $schema_summary,
      status: $schema_status,
      log: $schema_log
    },
    validator: {
      summary: $validator_summary,
      log: $validator_log,
      first_beta_status: $cohort_status,
      commercial_launch_drill_status: $commercial_status
    },
    templates: {
      first_beta: {
        path: $cohort_template,
        sha256: $cohort_template_sha256,
        schema_path: $cohort_schema,
        schema_sha256: $cohort_schema_sha256,
        accepted_status: "first_beta_cohort_evidence_green",
        public_launch_credit: false
      },
      commercial_launch_drill: {
        path: $commercial_template,
        sha256: $commercial_template_sha256,
        schema_path: $commercial_schema,
        schema_sha256: $commercial_schema_sha256,
        accepted_status: "commercial_launch_drill_evidence_green",
        public_launch_credit: false
      }
    },
    required_evidence: [
      { id: "first_beta_participants", field: "participant_count, participants", evidence: "5-10 real beta participants with sanitized participant ids, consent/recruiting notes, and no template-only records" },
      { id: "first_beta_sessions", field: "sessions", evidence: "session evidence covering every participant, with timestamps, build/version, platform, duration, and completed gameplay path notes" },
      { id: "first_beta_feedback_summary", field: "feedback_summary", evidence: "aggregated playability score, retention intent, blocking issues, and reviewer disposition" },
      { id: "first_beta_operator_signoff", field: "operator_signoff", evidence: "real_participants_confirmed=true, synthetic_or_template_data_rejected=true, signed_by, signed_at" },
      { id: "commercial_payment_drill", field: "drills.payment", evidence: "real or sanitized payment drill evidence with status green/passed and artifact reference" },
      { id: "commercial_refund_drill", field: "drills.refund", evidence: "real or sanitized refund drill evidence with status green/passed and artifact reference" },
      { id: "commercial_support_drill", field: "drills.support", evidence: "support escalation/response drill evidence with status green/passed and artifact reference" },
      { id: "commercial_legal_drill", field: "drills.legal", evidence: "legal/privacy/terms/compliance drill evidence with status green/passed and artifact reference" },
      { id: "commercial_operator_drill", field: "drills.operator", evidence: "operator runbook/on-call/rollback drill evidence with status green/passed and artifact reference" },
      { id: "commercial_traffic_drill", field: "drills.traffic", evidence: "traffic/load/monitoring drill evidence with status green/passed and artifact reference" },
      { id: "commercial_operator_signoff", field: "operator_signoff", evidence: "real_or_sanitized_drill_confirmed=true, synthetic_or_template_data_rejected=true, signed_by, signed_at" }
    ],
    privacy_boundary: [
      "Use sanitized participant ids and evidence references.",
      "Do not store private personal data in templates.",
      "Templates and collection checklists carry no public-launch credit."
    ],
    reviewer_next_action: "collect_real_first_beta_and_commercial_drill_evidence_then_run_validator"
  }' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World Cohort / Commercial Evidence Collection\n\n'
  printf -- '- status: %s\n' "$STATUS"
  printf -- '- green: %s\n' "$COLLECTION_GREEN"
  printf -- '- first_beta_ready: false\n'
  printf -- '- commercial_launch_drill_ready: false\n'
  printf -- '- required_evidence_count: 11\n'
  printf -- '- template_count: 2\n'
  printf -- '- template_schema_count: 2\n'
  printf -- '- blocked_validator_status_count: %s\n' "$BLOCKED_VALIDATOR_STATUS_COUNT"
  printf -- '- schema_status: %s\n' "$SCHEMA_STATUS"
  printf -- '- first_beta_validator_status: %s\n' "$COHORT_STATUS"
  printf -- '- commercial_validator_status: %s\n\n' "$COMMERCIAL_STATUS"
  printf '## Commands\n\n'
  printf -- '- collect: scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh\n'
  printf -- '- validate: TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready\n\n'
  printf '## Required Evidence\n\n'
  jq -r '.required_evidence[] | "- [ ] " + .id + ": " + .evidence + "\n  - field: " + .field' "$SUMMARY_FILE"
  printf '\n## Privacy Boundary\n\n'
  printf -- '- Use sanitized participant ids and evidence references.\n'
  printf -- '- Do not store private personal data in templates.\n'
  printf -- '- Collection artifacts have no public-launch credit.\n'
} >"$MARKDOWN_FILE"

if [[ "$STATUS" == "cohort_commercial_evidence_collection_ready" ]]; then
  printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_COLLECTION_READY %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_COLLECTION_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
exit 1
