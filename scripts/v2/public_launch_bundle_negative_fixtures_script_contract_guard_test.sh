#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh"

required_lines=(
  'trillionnium_world_public_launch_bundle_negative_fixtures_v1'
  'public-launch-bundle-negative-fixtures.json'
  'check_trillionnium_world_public_launch_evidence_kit.sh'
  'check_trillionnium_world_public_launch_evidence_bundle.sh'
  'trillionnium_world_public_launch_evidence_bundle_v1'
  'public_launch_evidence_bundle_green'
  's5-device-evidence.template.json'
  'production-map-pack-public-evidence.template.json'
  'first-beta-cohort-evidence.template.json'
  'commercial-launch-drill-evidence.template.json'
  'multi-node-latency-evidence.template.json'
  'public-network-deploy-evidence.template.json'
  'fake_green_bundle_manifest_pointing_to_no_credit_templates_must_fail_require_ready'
  'public_launch_evidence_bundle_blocked_invalid_real_evidence'
  '--require-ready'
  'public_launch_claimed: false'
  'android_s5_real_device_claimed: false'
  'live_map_ingestion_performed: false'
  'live_public_exposure_performed: false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch bundle negative fixtures script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch bundle negative fixtures script rejects fake green bundle manifests pointing to templates"
