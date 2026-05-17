#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/cohort-commercial-evidence-schema.json"
COHORT_SCHEMA="$ACCEPTANCE_DIR/first-beta-cohort-evidence.schema.json"
COMMERCIAL_SCHEMA="$ACCEPTANCE_DIR/commercial-launch-drill-evidence.schema.json"
COHORT_TEMPLATE="$ACCEPTANCE_DIR/first-beta-cohort-evidence.template.json"
COMMERCIAL_TEMPLATE="$ACCEPTANCE_DIR/commercial-launch-drill-evidence.template.json"

mkdir -p "$ACCEPTANCE_DIR"

jq -n '{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Trillionnium World First Beta Cohort Evidence",
  "type": "object",
  "required": ["contract_version", "status", "participant_count", "participants", "sessions", "feedback_summary", "operator_signoff"],
  "properties": {
    "contract_version": { "const": "trillionnium_world_first_beta_cohort_evidence_v1" },
    "status": { "enum": ["first_beta_cohort_evidence_green", "template_requires_real_participants", "blocked"] },
    "participant_count": { "type": "integer", "minimum": 5, "maximum": 10 },
    "participants": { "type": "array", "minItems": 5, "maxItems": 10 },
    "sessions": { "type": "array", "minItems": 5 },
    "feedback_summary": { "type": "object" },
    "operator_signoff": { "type": "object" }
  }
}' >"$COHORT_SCHEMA"

jq -n '{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Trillionnium World Commercial Launch Drill Evidence",
  "type": "object",
  "required": ["contract_version", "status", "drills", "operator_signoff"],
  "properties": {
    "contract_version": { "const": "trillionnium_world_commercial_launch_drill_evidence_v1" },
    "status": { "enum": ["commercial_launch_drill_evidence_green", "template_requires_real_drill", "blocked"] },
    "drills": {
      "type": "object",
      "required": ["payment", "refund", "support", "legal", "operator", "traffic"]
    },
    "operator_signoff": { "type": "object" }
  }
}' >"$COMMERCIAL_SCHEMA"

jq -n '{
  "contract_version": "trillionnium_world_first_beta_cohort_evidence_v1",
  "status": "template_requires_real_participants",
  "acceptance_status": "first_beta_cohort_evidence_green",
  "participant_count": 0,
  "required_participant_range": { "min": 5, "max": 10 },
  "participants": [],
  "sessions": [],
  "feedback_summary": {
    "playability_score_avg": null,
    "retention_intent_avg": null,
    "blocking_issues": []
  },
  "operator_signoff": {
    "signed_by": null,
    "signed_at": null,
    "real_participants_confirmed": false,
    "synthetic_or_template_data_rejected": true
  },
  "collection": {
    "command": "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh",
    "validation_command": "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready",
    "output_path": "acceptance/S6_public_launch/latest/first-beta-cohort-evidence.json",
    "requires_real_participants": true,
    "requires_participant_count_min": 5,
    "requires_participant_count_max": 10,
    "privacy_note": "Use sanitized participant ids and evidence references; do not store raw private data in templates."
  }
}' >"$COHORT_TEMPLATE"

jq -n '{
  "contract_version": "trillionnium_world_commercial_launch_drill_evidence_v1",
  "status": "template_requires_real_drill",
  "acceptance_status": "commercial_launch_drill_evidence_green",
  "drills": {
    "payment": { "status": "not_run", "evidence": null },
    "refund": { "status": "not_run", "evidence": null },
    "support": { "status": "not_run", "evidence": null },
    "legal": { "status": "not_run", "evidence": null },
    "operator": { "status": "not_run", "evidence": null },
    "traffic": { "status": "not_run", "evidence": null }
  },
  "operator_signoff": {
    "signed_by": null,
    "signed_at": null,
    "real_or_sanitized_drill_confirmed": false,
    "synthetic_or_template_data_rejected": true
  },
  "collection": {
    "command": "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh",
    "validation_command": "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready",
    "output_path": "acceptance/S6_public_launch/latest/commercial-launch-drill-evidence.json",
    "required_drills": ["payment", "refund", "support", "legal", "operator", "traffic"],
    "requires_real_or_sanitized_drill": true,
    "privacy_note": "Use sanitized drill artifacts and evidence references; do not store private customer/payment data in templates."
  }
}' >"$COMMERCIAL_TEMPLATE"

COHORT_TEMPLATE_STATUS="$(jq -r '.status' "$COHORT_TEMPLATE")"
COMMERCIAL_TEMPLATE_STATUS="$(jq -r '.status' "$COMMERCIAL_TEMPLATE")"
COHORT_CONTRACT="$(jq -r '.contract_version' "$COHORT_TEMPLATE")"
COMMERCIAL_CONTRACT="$(jq -r '.contract_version' "$COMMERCIAL_TEMPLATE")"

STATUS="cohort_commercial_evidence_schema_green"
if [[ "$COHORT_TEMPLATE_STATUS" == "first_beta_cohort_evidence_green" ]]; then
  STATUS="cohort_template_must_not_claim_green"
fi
if [[ "$COMMERCIAL_TEMPLATE_STATUS" == "commercial_launch_drill_evidence_green" ]]; then
  STATUS="commercial_template_must_not_claim_green"
fi
if [[ "$COHORT_CONTRACT" != "trillionnium_world_first_beta_cohort_evidence_v1" ]]; then
  STATUS="cohort_schema_contract_failed"
fi
if [[ "$COMMERCIAL_CONTRACT" != "trillionnium_world_commercial_launch_drill_evidence_v1" ]]; then
  STATUS="commercial_schema_contract_failed"
fi

jq -n \
  --arg contract_version "trillionnium_world_cohort_commercial_evidence_schema_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg cohort_schema "$COHORT_SCHEMA" \
  --arg commercial_schema "$COMMERCIAL_SCHEMA" \
  --arg cohort_template "$COHORT_TEMPLATE" \
  --arg commercial_template "$COMMERCIAL_TEMPLATE" \
  --arg cohort_schema_sha256 "$(sha256sum "$COHORT_SCHEMA" | awk '{print $1}')" \
  --arg commercial_schema_sha256 "$(sha256sum "$COMMERCIAL_SCHEMA" | awk '{print $1}')" \
  --arg cohort_template_sha256 "$(sha256sum "$COHORT_TEMPLATE" | awk '{print $1}')" \
  --arg commercial_template_sha256 "$(sha256sum "$COMMERCIAL_TEMPLATE" | awk '{print $1}')" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_cohort_commercial_schema_gate",
    public_launch_credit: "schema_and_template_only_not_real_cohort_or_commercial_drill",
    production_ready: false,
    cohort: {
      contract_version: "trillionnium_world_first_beta_cohort_evidence_v1",
      accepted_status: "first_beta_cohort_evidence_green",
      schema_path: $cohort_schema,
      schema_sha256: $cohort_schema_sha256,
      template_path: $cohort_template,
      template_sha256: $cohort_template_sha256,
      min_participants: 5,
      max_participants: 10
    },
    commercial: {
      contract_version: "trillionnium_world_commercial_launch_drill_evidence_v1",
      accepted_status: "commercial_launch_drill_evidence_green",
      schema_path: $commercial_schema,
      schema_sha256: $commercial_schema_sha256,
      template_path: $commercial_template,
      template_sha256: $commercial_template_sha256,
      required_drills: ["payment", "refund", "support", "legal", "operator", "traffic"]
    },
    guardrails: [
      "templates_must_not_claim_green",
      "real_or_sanitized_evidence_required_for_green_status",
      "synthetic_data_rejected_for_public_launch"
    ]
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "cohort_commercial_evidence_schema_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_SCHEMA_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_SCHEMA_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
exit 1
