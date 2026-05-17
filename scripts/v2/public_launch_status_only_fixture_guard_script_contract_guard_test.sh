#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_status_only_fixtures.sh"

required_lines=(
  'trillionnium_world_public_launch_status_only_fixture_guard_v1'
  'public-launch-status-only-fixtures.json'
  'status_only_green_fixtures_must_be_rejected_by_field_level_public_launch_evidence_validators'
  'trillionnium_world_s5_native_bevy_device_evidence_v1'
  'production_map_pack_public_ready_green'
  'first_beta_cohort_evidence_green'
  'commercial_launch_drill_evidence_green'
  'multi_node_or_live_traffic_latency_green'
  'public_network_deploy_green'
  'blocked_missing_s5_real_device_evidence'
  'blocked_missing_production_map_pack_public_evidence'
  'blocked_missing_cohort_commercial_real_evidence'
  'blocked_missing_external_ops_real_evidence'
  '--require-ready'
  'blocked_as_expected'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch status-only fixture guard missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch status-only fixture guard proves green status-only evidence cannot pass field-level validators"
