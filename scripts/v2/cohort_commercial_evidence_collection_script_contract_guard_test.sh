#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh"

required_lines=(
  'trillionnium_world_cohort_commercial_evidence_collection_v1'
  'cohort-commercial-evidence-collection.json'
  'cohort-commercial-evidence-collection.md'
  'check_trillionnium_world_cohort_commercial_schema.sh'
  'check_trillionnium_world_cohort_commercial_evidence.sh'
  'TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json>'
  'TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json>'
  'first_beta_participants'
  'first_beta_sessions'
  'first_beta_feedback_summary'
  'first_beta_operator_signoff'
  'commercial_payment_drill'
  'commercial_refund_drill'
  'commercial_support_drill'
  'commercial_legal_drill'
  'commercial_operator_drill'
  'commercial_traffic_drill'
  'commercial_operator_signoff'
  'public_launch_credit: false'
  'Use sanitized participant ids'
  'Do not store private personal data in templates'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] cohort/commercial evidence collection script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] cohort/commercial evidence collection script keeps participant/session/drill checklist, validator command, and privacy boundary"
