#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_external_ops_evidence_collection.sh"

required_lines=(
  'trillionnium_world_external_ops_evidence_collection_v1'
  'external-ops-evidence-collection.json'
  'external-ops-evidence-collection.md'
  'external-ops-evidence-collection-validator.log'
  'check_trillionnium_world_external_ops_evidence.sh'
  'TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json>'
  'TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json>'
  'public_launch_credit: false'
  'multi_node_or_live_traffic_latency_ready: false'
  'public_network_deploy_ready: false'
  'live_public_exposure_performed: false'
  'multi_node_or_live_traffic_scope'
  'latency_endpoints'
  'latency_public_url_probes'
  'latency_p95_budget'
  'monitoring_timeseries'
  'rollback_under_load'
  'latency_operator_signoff'
  'public_exposure_approval'
  'public_host_domain_tls'
  'public_url_health_probes'
  'public_monitoring_backup_rollback'
  'public_exposure_operator_signoff'
  'It does not open a public network route.'
  'It does not create live public traffic.'
  'Local latency/deploy drills are useful but have no public-launch credit.'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] external ops evidence collection script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] external ops evidence collection script emits no-credit multi-node/live-traffic and public exposure checklist"
