#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_SUMMARY:-$ACCEPTANCE_DIR/external-ops-evidence.json}"
MULTI_NODE_EVIDENCE_PATH="${TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH:-}"
PUBLIC_DEPLOY_EVIDENCE_PATH="${TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH:-}"
LOCAL_LATENCY_EVIDENCE="$ACCEPTANCE_DIR/release-latency-drill.json"
LOCAL_DEPLOY_EVIDENCE="$ACCEPTANCE_DIR/public-network-deploy-evidence.json"
MULTI_NODE_TEMPLATE="$ACCEPTANCE_DIR/multi-node-latency-evidence.template.json"
PUBLIC_DEPLOY_TEMPLATE="$ACCEPTANCE_DIR/public-network-deploy-evidence.template.json"
REQUIRE_READY=0

usage() {
  cat <<'USAGE'
Usage: scripts/check_trillionnium_world_external_ops_evidence.sh [--require-ready]

Validates real external operations evidence for public-launch credit.

Environment:
  TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json>
  TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json>
  TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_SUMMARY=<summary.json>

Collection helper:
  scripts/check_trillionnium_world_external_ops_evidence_collection.sh

This validator records local latency/deploy drills as no-credit context only.
It does not open public network exposure or create live public traffic.
USAGE
}

for arg in "$@"; do
  case "$arg" in
    --help|-h)
      usage
      exit 0
      ;;
    --require-ready)
      REQUIRE_READY=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$ACCEPTANCE_DIR"

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

to_int() {
  local value="$1"
  if [[ "$value" =~ ^[0-9]+$ ]]; then
    printf '%s' "$value"
  else
    printf '0'
  fi
}

json_array_from_lines() {
  jq -Rsc 'split("\n") | map(select(length > 0))'
}

jq -n '{
  contract_version: "trillionnium_world_multi_node_or_live_traffic_latency_evidence_v1",
  status: "template_requires_multi_node_or_live_traffic_latency",
  acceptance_status: "multi_node_or_live_traffic_latency_green",
  scope: {
    multi_node_or_live_traffic_confirmed: false,
    node_count: 0,
    live_public_traffic_confirmed: false
  },
  latency: {
    p95_seconds: null,
    p95_budget_seconds: 0.5,
    endpoints: []
  },
  probes: {
    public_url_probe_samples: [],
    monitoring_timeseries_evidence: null
  },
  rollback_under_load: {
    status: "not_run",
    evidence: null
  },
  operator_signoff: {
    signed_by: null,
    signed_at: null,
    real_multi_node_or_live_traffic_confirmed: false,
    synthetic_or_template_data_rejected: true
  },
  collection: {
    command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready",
    output_path: "acceptance/S6_public_launch/latest/multi-node-latency-evidence.json",
    requires_multi_node_or_live_traffic: true,
    requires_public_url_probe_samples_min: 3,
    requires_latency_endpoints_min: 3,
    requires_monitoring_timeseries_evidence: true,
    requires_rollback_under_load: true,
    public_launch_credit: false
  }
}' >"$MULTI_NODE_TEMPLATE"

jq -n '{
  contract_version: "trillionnium_world_public_network_deploy_evidence_v1",
  status: "template_requires_public_network_deploy",
  acceptance_status: "public_network_deploy_green",
  approval: {
    public_network_exposure_approved: false,
    approved_by: null,
    approved_at: null
  },
  deployment: {
    host: null,
    domain: null,
    public_base_url: null,
    tls_certificate: { status: "not_verified", evidence: null }
  },
  probes: {
    public_url_probe_samples: [],
    health_status: "not_run"
  },
  monitoring: { status: "not_configured", evidence: null },
  backup: { status: "not_verified", evidence: null },
  rollback: { status: "not_verified", evidence: null },
  operator_signoff: {
    signed_by: null,
    signed_at: null,
    public_exposure_confirmed: false,
    synthetic_or_template_data_rejected: true
  },
  collection: {
    command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready",
    output_path: "acceptance/S6_public_launch/latest/public-network-deploy-evidence.json",
    requires_public_exposure_approval: true,
    requires_host_domain_public_url: true,
    requires_tls_certificate: true,
    requires_public_url_probe_samples_min: 3,
    requires_monitoring_backup_rollback: true,
    opens_public_network_route: false,
    public_launch_credit: false
  }
}' >"$PUBLIC_DEPLOY_TEMPLATE"

LATENCY_BLOCKERS=()
LATENCY_FILE_STATUS="$(file_status "$MULTI_NODE_EVIDENCE_PATH")"
LATENCY_CONTRACT="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '.contract_version')"
LATENCY_STATUS_RAW="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '.status')"
LATENCY_SCOPE_CONFIRMED="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '(.scope.multi_node_or_live_traffic_confirmed == true) or (.scope.live_public_traffic_confirmed == true) or (.real_multi_node_or_live_traffic_confirmed == true)')"
LATENCY_NODE_COUNT="$(to_int "$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '(.scope.node_count // .node_count // 0)')")"
LATENCY_ENDPOINT_COUNT="$(to_int "$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '((.latency.endpoints // .endpoints // []) | length)')")"
LATENCY_PROBE_COUNT="$(to_int "$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '((.probes.public_url_probe_samples // .public_url_probe_samples // []) | length)')")"
LATENCY_P95_OK="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '((.latency.p95_seconds // .p95_seconds // 999999) <= (.latency.p95_budget_seconds // .p95_budget_seconds // 0.5))')"
LATENCY_MONITORING_EVIDENCE="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '(.probes.monitoring_timeseries_evidence // .monitoring.timeseries_evidence // .monitoring.evidence)')"
LATENCY_ROLLBACK_STATUS="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '(.rollback_under_load.status // .rollback.status)')"
LATENCY_SIGNOFF_REAL="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '.operator_signoff.real_multi_node_or_live_traffic_confirmed == true')"
LATENCY_REJECTS_SYNTHETIC="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '.operator_signoff.synthetic_or_template_data_rejected == true')"
LATENCY_SIGNED_BY="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '.operator_signoff.signed_by')"
LATENCY_SIGNED_AT="$(read_json_field "$MULTI_NODE_EVIDENCE_PATH" '.operator_signoff.signed_at')"
LOCAL_LATENCY_STATUS="$(read_json_field "$LOCAL_LATENCY_EVIDENCE" '.status')"
LOCAL_LATENCY_FILE_STATUS="$(file_status "$LOCAL_LATENCY_EVIDENCE")"

[[ "$LATENCY_FILE_STATUS" == "present" ]] || LATENCY_BLOCKERS+=("multi_node_latency_evidence_file")
[[ "$LATENCY_CONTRACT" == "trillionnium_world_multi_node_or_live_traffic_latency_evidence_v1" ]] || LATENCY_BLOCKERS+=("multi_node_latency_contract")
[[ "$LATENCY_STATUS_RAW" == "multi_node_or_live_traffic_latency_green" ]] || LATENCY_BLOCKERS+=("multi_node_latency_status")
[[ "$LATENCY_SCOPE_CONFIRMED" == "true" ]] || LATENCY_BLOCKERS+=("multi_node_or_live_traffic_scope_confirmed")
if [[ "$LATENCY_NODE_COUNT" -lt 2 && "$LATENCY_SCOPE_CONFIRMED" != "true" ]]; then
  LATENCY_BLOCKERS+=("multi_node_count_or_live_traffic")
fi
[[ "$LATENCY_ENDPOINT_COUNT" -ge 3 ]] || LATENCY_BLOCKERS+=("latency_endpoint_count")
[[ "$LATENCY_PROBE_COUNT" -ge 3 ]] || LATENCY_BLOCKERS+=("public_url_probe_samples")
[[ "$LATENCY_P95_OK" == "true" ]] || LATENCY_BLOCKERS+=("p95_latency_budget")
[[ -n "$LATENCY_MONITORING_EVIDENCE" && "$LATENCY_MONITORING_EVIDENCE" != "null" ]] || LATENCY_BLOCKERS+=("monitoring_timeseries_evidence")
[[ "$LATENCY_ROLLBACK_STATUS" == "green" || "$LATENCY_ROLLBACK_STATUS" == "passed" || "$LATENCY_ROLLBACK_STATUS" == "rollback_under_load_green" ]] || LATENCY_BLOCKERS+=("rollback_under_load")
[[ "$LATENCY_SIGNOFF_REAL" == "true" ]] || LATENCY_BLOCKERS+=("real_multi_node_or_live_traffic_signoff")
[[ "$LATENCY_REJECTS_SYNTHETIC" == "true" ]] || LATENCY_BLOCKERS+=("synthetic_latency_rejected")
[[ -n "$LATENCY_SIGNED_BY" && -n "$LATENCY_SIGNED_AT" ]] || LATENCY_BLOCKERS+=("latency_operator_signature")

LATENCY_BLOCKERS_JSON="$(printf '%s\n' "${LATENCY_BLOCKERS[@]}" | json_array_from_lines)"
LATENCY_STATUS="multi_node_or_live_traffic_latency_green"
if [[ "$(jq 'length' <<<"$LATENCY_BLOCKERS_JSON")" != "0" ]]; then
  LATENCY_STATUS="blocked_missing_multi_node_or_live_traffic_latency_evidence"
fi

DEPLOY_BLOCKERS=()
DEPLOY_FILE_STATUS="$(file_status "$PUBLIC_DEPLOY_EVIDENCE_PATH")"
DEPLOY_CONTRACT="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.contract_version')"
DEPLOY_STATUS_RAW="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.status')"
DEPLOY_APPROVED="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.approval.public_network_exposure_approved == true) or (.public_network_exposure_approved == true)')"
DEPLOY_HOST="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.deployment.host // .host // .target_host)')"
DEPLOY_DOMAIN="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.deployment.domain // .domain)')"
DEPLOY_PUBLIC_URL="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.deployment.public_base_url // .public_base_url // .public_url)')"
DEPLOY_TLS_STATUS="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.deployment.tls_certificate.status // .tls.status // .tls_certificate.status)')"
DEPLOY_PROBE_COUNT="$(to_int "$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '((.probes.public_url_probe_samples // .public_url_probe_samples // []) | length)')")"
DEPLOY_HEALTH_STATUS="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.probes.health_status // .health.status // .public_url_health_status)')"
DEPLOY_MONITORING_STATUS="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.monitoring.status // .monitoring_alerts.status)')"
DEPLOY_BACKUP_STATUS="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '(.backup.status // .backup_restore.status)')"
DEPLOY_ROLLBACK_STATUS="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.rollback.status')"
DEPLOY_SIGNOFF_REAL="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.operator_signoff.public_exposure_confirmed == true')"
DEPLOY_REJECTS_SYNTHETIC="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.operator_signoff.synthetic_or_template_data_rejected == true')"
DEPLOY_SIGNED_BY="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.operator_signoff.signed_by')"
DEPLOY_SIGNED_AT="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE_PATH" '.operator_signoff.signed_at')"
LOCAL_DEPLOY_STATUS="$(read_json_field "$LOCAL_DEPLOY_EVIDENCE" '.status')"
LOCAL_DEPLOY_FILE_STATUS="$(file_status "$LOCAL_DEPLOY_EVIDENCE")"

[[ "$DEPLOY_FILE_STATUS" == "present" ]] || DEPLOY_BLOCKERS+=("public_network_deploy_evidence_file")
[[ "$DEPLOY_CONTRACT" == "trillionnium_world_public_network_deploy_evidence_v1" ]] || DEPLOY_BLOCKERS+=("public_network_deploy_contract")
[[ "$DEPLOY_STATUS_RAW" == "public_network_deploy_green" ]] || DEPLOY_BLOCKERS+=("public_network_deploy_status")
[[ "$DEPLOY_APPROVED" == "true" ]] || DEPLOY_BLOCKERS+=("public_network_exposure_approved")
[[ -n "$DEPLOY_HOST" && -n "$DEPLOY_DOMAIN" && -n "$DEPLOY_PUBLIC_URL" ]] || DEPLOY_BLOCKERS+=("host_domain_public_url")
[[ "$DEPLOY_TLS_STATUS" == "green" || "$DEPLOY_TLS_STATUS" == "verified" || "$DEPLOY_TLS_STATUS" == "tls_certificate_green" ]] || DEPLOY_BLOCKERS+=("tls_certificate")
[[ "$DEPLOY_PROBE_COUNT" -ge 3 ]] || DEPLOY_BLOCKERS+=("public_url_probe_samples")
[[ "$DEPLOY_HEALTH_STATUS" == "green" || "$DEPLOY_HEALTH_STATUS" == "ok" || "$DEPLOY_HEALTH_STATUS" == "passed" ]] || DEPLOY_BLOCKERS+=("public_url_health_probe")
[[ "$DEPLOY_MONITORING_STATUS" == "green" || "$DEPLOY_MONITORING_STATUS" == "configured" || "$DEPLOY_MONITORING_STATUS" == "passed" ]] || DEPLOY_BLOCKERS+=("monitoring_alerts")
[[ "$DEPLOY_BACKUP_STATUS" == "green" || "$DEPLOY_BACKUP_STATUS" == "verified" || "$DEPLOY_BACKUP_STATUS" == "passed" ]] || DEPLOY_BLOCKERS+=("backup_restore")
[[ "$DEPLOY_ROLLBACK_STATUS" == "green" || "$DEPLOY_ROLLBACK_STATUS" == "verified" || "$DEPLOY_ROLLBACK_STATUS" == "passed" ]] || DEPLOY_BLOCKERS+=("public_deploy_rollback")
[[ "$DEPLOY_SIGNOFF_REAL" == "true" ]] || DEPLOY_BLOCKERS+=("public_exposure_signoff")
[[ "$DEPLOY_REJECTS_SYNTHETIC" == "true" ]] || DEPLOY_BLOCKERS+=("synthetic_deploy_rejected")
[[ -n "$DEPLOY_SIGNED_BY" && -n "$DEPLOY_SIGNED_AT" ]] || DEPLOY_BLOCKERS+=("public_deploy_operator_signature")

DEPLOY_BLOCKERS_JSON="$(printf '%s\n' "${DEPLOY_BLOCKERS[@]}" | json_array_from_lines)"
DEPLOY_BLOCKER_COUNT="$(jq 'length' <<<"$DEPLOY_BLOCKERS_JSON")"
DEPLOY_STATUS="public_network_deploy_green"
if [[ "$DEPLOY_BLOCKER_COUNT" != "0" ]]; then
  DEPLOY_STATUS="blocked_missing_public_network_live_exposure_evidence"
fi

LATENCY_BLOCKER_COUNT="$(jq 'length' <<<"$LATENCY_BLOCKERS_JSON")"
STATUS="external_ops_evidence_green"
if [[ "$LATENCY_STATUS" != "multi_node_or_live_traffic_latency_green" || "$DEPLOY_STATUS" != "public_network_deploy_green" ]]; then
  STATUS="blocked_missing_external_ops_real_evidence"
fi
EXTERNAL_OPS_GREEN=false
MULTI_NODE_READY=false
PUBLIC_NETWORK_READY=false
if [[ "$STATUS" == "external_ops_evidence_green" ]]; then
  EXTERNAL_OPS_GREEN=true
fi
if [[ "$LATENCY_STATUS" == "multi_node_or_live_traffic_latency_green" ]]; then
  MULTI_NODE_READY=true
fi
if [[ "$DEPLOY_STATUS" == "public_network_deploy_green" ]]; then
  PUBLIC_NETWORK_READY=true
fi
BLOCKER_COUNT=$((LATENCY_BLOCKER_COUNT + DEPLOY_BLOCKER_COUNT))

jq -n \
  --arg contract_version "trillionnium_world_external_ops_evidence_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson external_ops_green "$EXTERNAL_OPS_GREEN" \
  --argjson multi_node_ready "$MULTI_NODE_READY" \
  --argjson public_network_ready "$PUBLIC_NETWORK_READY" \
  --argjson blocker_count "$BLOCKER_COUNT" \
  --argjson multi_node_blocker_count "$LATENCY_BLOCKER_COUNT" \
  --argjson public_network_blocker_count "$DEPLOY_BLOCKER_COUNT" \
  --arg multi_node_path "$MULTI_NODE_EVIDENCE_PATH" \
  --arg multi_node_file_status "$LATENCY_FILE_STATUS" \
  --arg multi_node_contract "$LATENCY_CONTRACT" \
  --arg multi_node_raw_status "$LATENCY_STATUS_RAW" \
  --arg multi_node_status "$LATENCY_STATUS" \
  --arg local_latency_path "$LOCAL_LATENCY_EVIDENCE" \
  --arg local_latency_status "$LOCAL_LATENCY_STATUS" \
  --arg local_latency_file_status "$LOCAL_LATENCY_FILE_STATUS" \
  --argjson latency_node_count "$LATENCY_NODE_COUNT" \
  --argjson latency_endpoint_count "$LATENCY_ENDPOINT_COUNT" \
  --argjson latency_probe_count "$LATENCY_PROBE_COUNT" \
  --argjson latency_blockers "$LATENCY_BLOCKERS_JSON" \
  --arg public_deploy_path "$PUBLIC_DEPLOY_EVIDENCE_PATH" \
  --arg public_deploy_file_status "$DEPLOY_FILE_STATUS" \
  --arg public_deploy_contract "$DEPLOY_CONTRACT" \
  --arg public_deploy_raw_status "$DEPLOY_STATUS_RAW" \
  --arg public_deploy_status "$DEPLOY_STATUS" \
  --arg local_deploy_path "$LOCAL_DEPLOY_EVIDENCE" \
  --arg local_deploy_status "$LOCAL_DEPLOY_STATUS" \
  --arg local_deploy_file_status "$LOCAL_DEPLOY_FILE_STATUS" \
  --argjson public_deploy_probe_count "$DEPLOY_PROBE_COUNT" \
  --argjson deploy_blockers "$DEPLOY_BLOCKERS_JSON" \
  --arg multi_node_template "$MULTI_NODE_TEMPLATE" \
  --arg public_deploy_template "$PUBLIC_DEPLOY_TEMPLATE" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_external_ops_evidence_gate",
    green: $external_ops_green,
    external_ops_ready: $external_ops_green,
    multi_node_or_live_traffic_latency_ready: $multi_node_ready,
    public_network_deploy_ready: $public_network_ready,
    public_launch_credit: "only_when_multi_node_or_live_traffic_and_public_network_deploy_statuses_are_green_after_field_validation",
    local_drill_rule: "local_release_load_drill_only_not_multi_node_or_live_traffic",
    live_public_exposure_performed_by_this_script: false,
    blocker_count: $blocker_count,
    multi_node_or_live_traffic_latency_blocker_count: $multi_node_blocker_count,
    public_network_deploy_blocker_count: $public_network_blocker_count,
    template_count: 2,
    local_drill_count: 2,
    multi_node_or_live_traffic_latency: {
      status: $multi_node_status,
      accepted_status: "multi_node_or_live_traffic_latency_green",
      operator_evidence: {
        path: (if $multi_node_path == "" then null else $multi_node_path end),
        file_status: $multi_node_file_status,
        contract_version: $multi_node_contract,
        status: $multi_node_raw_status
      },
      local_drill: {
        path: $local_latency_path,
        file_status: $local_latency_file_status,
        status: $local_latency_status,
        public_launch_credit: false
      },
      node_count: $latency_node_count,
      endpoint_count: $latency_endpoint_count,
      public_url_probe_sample_count: $latency_probe_count,
      blockers: $latency_blockers
    },
    public_network_deploy: {
      status: $public_deploy_status,
      accepted_status: "public_network_deploy_green",
      operator_evidence: {
        path: (if $public_deploy_path == "" then null else $public_deploy_path end),
        file_status: $public_deploy_file_status,
        contract_version: $public_deploy_contract,
        status: $public_deploy_raw_status
      },
      local_drill: {
        path: $local_deploy_path,
        file_status: $local_deploy_file_status,
        status: $local_deploy_status,
        public_launch_credit: false
      },
      public_url_probe_sample_count: $public_deploy_probe_count,
      blockers: $deploy_blockers
    },
    templates: {
      multi_node_or_live_traffic_latency: $multi_node_template,
      public_network_deploy: $public_deploy_template
    }
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "external_ops_evidence_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_EXTERNAL_OPS_EVIDENCE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
if [[ "$REQUIRE_READY" -eq 1 ]]; then
  exit 1
fi
