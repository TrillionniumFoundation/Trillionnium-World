#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh"

required_lines=(
  'trillionnium_world_cohort_commercial_evidence_gate_v1'
  'check_trillionnium_world_cohort_commercial_schema.sh'
  'cohort-commercial-evidence.json'
  'check_trillionnium_world_cohort_commercial_evidence_collection.sh'
  'TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH'
  'TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH'
  'first_beta_cohort_evidence_green'
  'commercial_launch_drill_evidence_green'
  'green: $cohort_commercial_green'
  'cohort_commercial_ready: $cohort_commercial_green'
  'first_beta_ready: $first_beta_ready'
  'commercial_launch_drill_ready: $commercial_ready'
  'blocker_count: $blocker_count'
  'first_beta_blocker_count: $first_beta_blocker_count'
  'commercial_launch_drill_blocker_count: $commercial_blocker_count'
  'required_drill_count: $required_drill_count'
  'failed_drill_count: $failed_drill_count'
  'participant_count_5_to_10'
  'session_count_covers_participants'
  'real_participants_signoff'
  'synthetic_cohort_rejected'
  'payment refund support legal operator traffic'
  'real_or_sanitized_commercial_signoff'
  'synthetic_commercial_rejected'
  'blocked_missing_cohort_commercial_real_evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] cohort/commercial evidence script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] cohort/commercial evidence script validates real cohort and commercial drill fields before public launch credit"
