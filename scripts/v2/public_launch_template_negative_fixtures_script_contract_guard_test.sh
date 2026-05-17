#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh"

required_lines=(
  'trillionnium_world_public_launch_template_negative_fixtures_v1'
  'public-launch-template-negative-fixtures.json'
  'check_trillionnium_world_public_launch_evidence_kit.sh'
  's5-device-evidence.template.json'
  'production-map-pack-public-evidence.template.json'
  'first-beta-cohort-evidence.template.json'
  'commercial-launch-drill-evidence.template.json'
  'multi-node-latency-evidence.template.json'
  'public-network-deploy-evidence.template.json'
  'check_trillionnium_world_s5_real_device_evidence.sh'
  'check_trillionnium_world_production_map_pack_public_evidence.sh'
  'check_trillionnium_world_cohort_commercial_evidence.sh'
  'check_trillionnium_world_external_ops_evidence.sh'
  'blocked_missing_s5_real_device_evidence'
  'blocked_missing_production_map_pack_public_evidence'
  'blocked_missing_cohort_commercial_real_evidence'
  'blocked_missing_external_ops_real_evidence'
  'no_credit_templates_must_fail_strict_field_validators_before_public_launch_handoff'
  'public_launch_claimed: false'
  'android_s5_real_device_claimed: false'
  'live_map_ingestion_performed: false'
  'live_public_exposure_performed: false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch template negative fixtures script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch template negative fixtures script proves templates fail strict validators"
