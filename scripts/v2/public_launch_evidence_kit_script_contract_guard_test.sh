#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh"

required_lines=(
  'trillionnium_world_public_launch_evidence_kit_v1'
  'public-launch-evidence-kit.json'
  'public-launch-evidence-kit.md'
  'collection_command'
  'check_trillionnium_world_s5_device_evidence.sh --require-device'
  'check_trillionnium_world_production_map_pack_public_evidence_collection.sh'
  'check_trillionnium_world_cohort_commercial_evidence_collection.sh'
  'check_trillionnium_world_external_ops_evidence_collection.sh'
  'check_trillionnium_world_s5_real_device_evidence.sh'
  'check_trillionnium_world_production_map_pack_public_evidence.sh'
  'check_trillionnium_world_cohort_commercial_schema.sh'
  'check_trillionnium_world_external_ops_evidence.sh'
  'check_trillionnium_world_public_launch_evidence_intake.sh'
  's5-device-evidence.template.json'
  'production-map-pack-public-evidence.template.json'
  'first-beta-cohort-evidence.template.json'
  'commercial-launch-drill-evidence.template.json'
  'multi-node-latency-evidence.template.json'
  'public-network-deploy-evidence.template.json'
  'operator_templates_must_exist_and_must_not_claim_green_until_real_external_evidence_passes_field_validators'
  'template_public_launch_credit: false'
  'public_launch_claimed: false'
  'android_s5_real_device_claimed: false'
  'live_map_ingestion_performed: false'
  'live_public_exposure_performed: false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch evidence kit script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch evidence kit script emits no-credit templates plus collection and validator commands for all external blockers"
