#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh"

required_lines=(
  'trillionnium_world_public_launch_blocker_consistency_v1'
  'public-launch-blocker-consistency.json'
  'release_review_acceptance_lock.sh'
  'trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"'
  'public_launch_readiness_blockers_must_match_evidence_intake_items_and_field_level_validator_statuses'
  'check_trillionnium_world_public_launch_readiness.sh'
  'check_trillionnium_world_public_launch_evidence_intake.sh'
  's5_real_device_matrix'
  'production_map_pack_public_evidence'
  'first_beta_cohort_evidence'
  'commercial_launch_drill_evidence'
  'multi_node_or_live_traffic_latency_evidence'
  'public_network_live_exposure_evidence'
  's5_real_device_evidence_green'
  'production_map_pack_public_ready_green'
  'first_beta_cohort_evidence_green'
  'commercial_launch_drill_evidence_green'
  'multi_node_or_live_traffic_latency_green'
  'public_network_deploy_green'
  'unknown_readiness_blockers'
  'unknown_intake_blockers'
  'known_blocker_count: $known_blocker_count'
  'readiness_blocker_count: $readiness_blocker_count'
  'intake_needs_collection_count: $intake_needs_collection_count'
  'unknown_readiness_blocker_count: $unknown_readiness_blocker_count'
  'unknown_intake_blocker_count: $unknown_intake_blocker_count'
  'check_count: ($checks | length)'
  'failed_check_count: ($failures | length)'
  'green: ($failures | length == 0)'
  'blockers: $known_blockers'
  'public_launch_claimed: false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch blocker consistency script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch blocker consistency script links readiness blockers, intake items, and validator statuses"
