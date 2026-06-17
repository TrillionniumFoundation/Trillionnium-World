#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_operator_handoff.sh"

while IFS= read -r line; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch operator handoff script missing contract line: $line" >&2
    exit 1
  fi
done <<'REQUIRED_LINES'
trillionnium_world_public_launch_operator_handoff_v1
public-launch-operator-handoff.json
public-launch-operator-handoff.md
release_review_acceptance_lock.sh
trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"
TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_SUMMARY
TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_MD
TRNM_PUBLIC_LAUNCH_OPERATOR_HANDOFF_REFRESH_INPUTS
if [[ "$REFRESH_INPUTS" != "0" ]]
check_trillionnium_world_release_review_status.sh
check_trillionnium_world_public_launch_evidence_intake.sh
check_trillionnium_world_public_launch_evidence_kit.sh
check_trillionnium_world_production_map_pack_public_evidence_collection.sh
check_trillionnium_world_cohort_commercial_evidence_collection.sh
check_trillionnium_world_external_ops_evidence_collection.sh
check_trillionnium_world_public_launch_blocker_consistency.sh
.green == true and (.failures // [] | length) == 0
check_trillionnium_world_public_launch_template_negative_fixtures.sh
check_trillionnium_world_public_launch_evidence_bundle.sh
check_trillionnium_world_public_launch_bundle_negative_fixtures.sh
public_launch_operator_handoff_ready_with_external_blockers
public_launch_operator_handoff_complete_green
operator_handoff_collects_real_external_public_launch_evidence_without_claiming_public_launch_ready_or_android_s5_real_device_ready
operator_actions
handoff_artifacts
sha256sum
s5-device-evidence.template.json
production-map-pack-public-evidence.template.json
first-beta-cohort-evidence.template.json
commercial-launch-drill-evidence.template.json
multi-node-latency-evidence.template.json
public-network-deploy-evidence.template.json
public-launch-evidence-bundle.template.json
public_launch_claimed: false
android_s5_real_device_claimed: false
live_map_ingestion_performed: false
live_public_exposure_performed: false
REQUIRED_LINES

echo "[PASS] public launch operator handoff script emits checksum-bound operator actions without claiming public launch or Android S5 readiness"
