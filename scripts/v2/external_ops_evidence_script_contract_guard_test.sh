#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh"

required_lines=(
  'trillionnium_world_external_ops_evidence_gate_v1'
  'external-ops-evidence.json'
  'TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH'
  'TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH'
  'check_trillionnium_world_external_ops_evidence_collection.sh'
  'collection:'
  'trillionnium_world_multi_node_or_live_traffic_latency_evidence_v1'
  'trillionnium_world_public_network_deploy_evidence_v1'
  'multi_node_or_live_traffic_latency_green'
  'public_network_deploy_green'
  'local_release_load_drill_only_not_multi_node_or_live_traffic'
  'public_launch_credit: false'
  'public_url_probe_samples'
  'monitoring_timeseries_evidence'
  'rollback_under_load'
  'public_network_exposure_approved'
  'tls_certificate'
  'monitoring_alerts'
  'backup_restore'
  'live_public_exposure_performed_by_this_script: false'
  'blocked_missing_external_ops_real_evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] external ops evidence script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] external ops evidence script validates multi-node/live-traffic and public deploy fields without claiming local drills as launch credit"
