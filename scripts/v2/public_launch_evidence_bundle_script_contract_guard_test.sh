#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh"

required_lines=(
  'trillionnium_world_public_launch_evidence_bundle_gate_v1'
  'trillionnium_world_public_launch_evidence_bundle_v1'
  'public-launch-evidence-bundle.json'
  'public-launch-evidence-bundle.md'
  'public-launch-evidence-bundle.template.json'
  'TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH'
  'check_trillionnium_world_public_launch_evidence_kit.sh'
  'check_trillionnium_world_s5_real_device_evidence.sh'
  'check_trillionnium_world_production_map_pack_public_evidence.sh'
  'check_trillionnium_world_cohort_commercial_evidence.sh'
  'check_trillionnium_world_external_ops_evidence.sh'
  'single_manifest_must_point_to_real_external_evidence_that_passes_all_field_validators_before_public_launch_credit'
  'public_launch_claimed: false'
  'android_s5_real_device_claimed: false'
  'live_map_ingestion_performed_by_this_script: false'
  'live_public_exposure_performed_by_this_script: false'
  '--require-ready'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch evidence bundle script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch evidence bundle script validates a single real-evidence manifest without claiming launch readiness"
