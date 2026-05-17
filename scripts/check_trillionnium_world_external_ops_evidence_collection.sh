#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/external-ops-evidence-collection.json"
MARKDOWN_FILE="$ACCEPTANCE_DIR/external-ops-evidence-collection.md"
VALIDATOR_LOG="$ACCEPTANCE_DIR/external-ops-evidence-collection-validator.log"
VALIDATOR_SUMMARY="$ACCEPTANCE_DIR/external-ops-evidence.json"
MULTI_NODE_TEMPLATE="$ACCEPTANCE_DIR/multi-node-latency-evidence.template.json"
PUBLIC_DEPLOY_TEMPLATE="$ACCEPTANCE_DIR/public-network-deploy-evidence.template.json"
LOCAL_LATENCY_EVIDENCE="$ACCEPTANCE_DIR/release-latency-drill.json"
LOCAL_DEPLOY_EVIDENCE="$ACCEPTANCE_DIR/public-network-deploy-evidence.json"

mkdir -p "$ACCEPTANCE_DIR"

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    printf 'TRILLIONNIUM_WORLD_EXTERNAL_OPS_COLLECTION_FAILED missing command: %s\n' "$name" >&2
    exit 1
  fi
}

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

file_status() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    printf 'present'
  else
    printf 'missing'
  fi
}

file_sha256() {
  local path="$1"
  if [[ -f "$path" ]]; then
    sha256sum "$path" | awk '{print $1}'
  fi
}

require_cmd jq
require_cmd sha256sum

bash "$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" >"$VALIDATOR_LOG" 2>&1 || true

LATENCY_STATUS="$(read_json_field "$VALIDATOR_SUMMARY" '.multi_node_or_live_traffic_latency.status')"
DEPLOY_STATUS="$(read_json_field "$VALIDATOR_SUMMARY" '.public_network_deploy.status')"
LOCAL_LATENCY_STATUS="$(read_json_field "$LOCAL_LATENCY_EVIDENCE" '.status')"
LOCAL_DEPLOY_STATUS="$(read_json_field "$LOCAL_DEPLOY_EVIDENCE" '.status')"
STATUS="external_ops_evidence_collection_ready"
if [[ ! -f "$MULTI_NODE_TEMPLATE" || ! -f "$PUBLIC_DEPLOY_TEMPLATE" ]]; then
  STATUS="external_ops_evidence_collection_blocked"
fi

jq -n \
  --arg contract_version "trillionnium_world_external_ops_evidence_collection_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg validator_summary "$VALIDATOR_SUMMARY" \
  --arg validator_log "$VALIDATOR_LOG" \
  --arg latency_status "$LATENCY_STATUS" \
  --arg deploy_status "$DEPLOY_STATUS" \
  --arg multi_node_template "$MULTI_NODE_TEMPLATE" \
  --arg public_deploy_template "$PUBLIC_DEPLOY_TEMPLATE" \
  --arg multi_node_template_sha256 "$(file_sha256 "$MULTI_NODE_TEMPLATE")" \
  --arg public_deploy_template_sha256 "$(file_sha256 "$PUBLIC_DEPLOY_TEMPLATE")" \
  --arg local_latency_evidence "$LOCAL_LATENCY_EVIDENCE" \
  --arg local_latency_file_status "$(file_status "$LOCAL_LATENCY_EVIDENCE")" \
  --arg local_latency_status "$LOCAL_LATENCY_STATUS" \
  --arg local_deploy_evidence "$LOCAL_DEPLOY_EVIDENCE" \
  --arg local_deploy_file_status "$(file_status "$LOCAL_DEPLOY_EVIDENCE")" \
  --arg local_deploy_status "$LOCAL_DEPLOY_STATUS" \
  --arg markdown_path "$MARKDOWN_FILE" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_external_ops_evidence_collection",
    public_launch_credit: false,
    multi_node_or_live_traffic_latency_ready: false,
    public_network_deploy_ready: false,
    live_public_exposure_performed: false,
    collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready",
    validator: {
      summary: $validator_summary,
      log: $validator_log,
      multi_node_or_live_traffic_latency_status: $latency_status,
      public_network_deploy_status: $deploy_status
    },
    templates: {
      multi_node_or_live_traffic_latency: {
        path: $multi_node_template,
        sha256: $multi_node_template_sha256,
        accepted_status: "multi_node_or_live_traffic_latency_green",
        public_launch_credit: false
      },
      public_network_deploy: {
        path: $public_deploy_template,
        sha256: $public_deploy_template_sha256,
        accepted_status: "public_network_deploy_green",
        public_launch_credit: false
      }
    },
    local_drills: {
      release_latency: {
        path: $local_latency_evidence,
        file_status: $local_latency_file_status,
        status: $local_latency_status,
        public_launch_credit: false
      },
      public_deploy: {
        path: $local_deploy_evidence,
        file_status: $local_deploy_file_status,
        status: $local_deploy_status,
        public_launch_credit: false
      }
    },
    required_evidence: [
      { id: "multi_node_or_live_traffic_scope", field: "scope", evidence: "multi-node deployment with node_count >= 2 or verified live public traffic scope" },
      { id: "latency_endpoints", field: "latency.endpoints", evidence: "at least 3 endpoint latency samples, including health/home/adapter or equivalent public paths" },
      { id: "latency_public_url_probes", field: "probes.public_url_probe_samples", evidence: "at least 3 public URL probe samples from outside localhost" },
      { id: "latency_p95_budget", field: "latency.p95_seconds", evidence: "p95 within declared budget, default <= 0.5s" },
      { id: "monitoring_timeseries", field: "probes.monitoring_timeseries_evidence", evidence: "monitoring timeseries or dashboard artifact covering live/multi-node traffic window" },
      { id: "rollback_under_load", field: "rollback_under_load", evidence: "rollback-under-load drill artifact with green/passed status" },
      { id: "latency_operator_signoff", field: "operator_signoff", evidence: "real_multi_node_or_live_traffic_confirmed=true, synthetic_or_template_data_rejected=true, signed_by, signed_at" },
      { id: "public_exposure_approval", field: "approval", evidence: "explicit approval for public network exposure, approver, and timestamp" },
      { id: "public_host_domain_tls", field: "deployment", evidence: "real host, domain, public_base_url, and verified TLS certificate evidence" },
      { id: "public_url_health_probes", field: "probes", evidence: "at least 3 public URL probe samples and green public health status" },
      { id: "public_monitoring_backup_rollback", field: "monitoring, backup, rollback", evidence: "monitoring/alerts, backup/restore, and public deploy rollback evidence with green/verified status" },
      { id: "public_exposure_operator_signoff", field: "operator_signoff", evidence: "public_exposure_confirmed=true, synthetic_or_template_data_rejected=true, signed_by, signed_at" }
    ],
    boundary: [
      "This script creates a collection checklist only.",
      "It does not open a public network route.",
      "It does not create live public traffic.",
      "Local latency/deploy drills are useful but have no public-launch credit."
    ],
    reviewer_next_action: "collect_real_external_ops_evidence_then_run_validator"
  }' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World External Ops Evidence Collection\n\n'
  printf -- '- status: %s\n' "$STATUS"
  printf -- '- multi_node_or_live_traffic_latency_ready: false\n'
  printf -- '- public_network_deploy_ready: false\n'
  printf -- '- live_public_exposure_performed: false\n'
  printf -- '- latency_validator_status: %s\n' "$LATENCY_STATUS"
  printf -- '- public_deploy_validator_status: %s\n\n' "$DEPLOY_STATUS"
  printf '## Commands\n\n'
  printf -- '- collect: scripts/check_trillionnium_world_external_ops_evidence_collection.sh\n'
  printf -- '- validate: TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready\n\n'
  printf '## Required Evidence\n\n'
  jq -r '.required_evidence[] | "- [ ] " + .id + ": " + .evidence + "\n  - field: " + .field' "$SUMMARY_FILE"
  printf '\n## Boundary\n\n'
  printf -- '- No public network route is opened here.\n'
  printf -- '- No live public traffic is created here.\n'
  printf -- '- Local latency/deploy drills have no public-launch credit.\n'
} >"$MARKDOWN_FILE"

if [[ "$STATUS" == "external_ops_evidence_collection_ready" ]]; then
  printf 'TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_COLLECTION_READY %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_COLLECTION_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
exit 1
