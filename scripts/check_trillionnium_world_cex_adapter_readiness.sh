#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S3_repository_adapter/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_SUMMARY:-$ACCEPTANCE_DIR/cex-production-adapter-readiness.json}"
RAW_EVIDENCE="$ACCEPTANCE_DIR/cex-production-adapter-readiness.raw.json"
REQUIRE_READY=0

for arg in "$@"; do
  case "$arg" in
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

INPUT_EVIDENCE="${TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_EVIDENCE:-}"
INPUT_URL="${TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_URL:-}"
FETCH_STATUS="not_attempted"
FETCH_DETAIL=""
EVIDENCE_AVAILABLE=false

if [[ -n "$INPUT_EVIDENCE" ]]; then
  if [[ -f "$INPUT_EVIDENCE" ]]; then
    cp "$INPUT_EVIDENCE" "$RAW_EVIDENCE"
    FETCH_STATUS="file_copied"
    FETCH_DETAIL="$INPUT_EVIDENCE"
    EVIDENCE_AVAILABLE=true
  else
    FETCH_STATUS="missing_file"
    FETCH_DETAIL="$INPUT_EVIDENCE"
  fi
elif [[ -n "$INPUT_URL" ]]; then
  if curl -fsS "$INPUT_URL" >"$RAW_EVIDENCE"; then
    FETCH_STATUS="url_fetched"
    FETCH_DETAIL="$INPUT_URL"
    EVIDENCE_AVAILABLE=true
  else
    FETCH_STATUS="url_fetch_failed"
    FETCH_DETAIL="$INPUT_URL"
  fi
elif [[ -f "$RAW_EVIDENCE" ]]; then
  FETCH_STATUS="cached_raw_evidence"
  FETCH_DETAIL="$RAW_EVIDENCE"
  EVIDENCE_AVAILABLE=true
fi

json_field() {
  local expr="$1"
  if [[ "$EVIDENCE_AVAILABLE" == "true" && -f "$RAW_EVIDENCE" ]]; then
    jq -r "$expr // empty" "$RAW_EVIDENCE" 2>/dev/null || true
  fi
}

json_check() {
  local expr="$1"
  if [[ "$EVIDENCE_AVAILABLE" == "true" && -f "$RAW_EVIDENCE" ]] && jq -e "$expr" "$RAW_EVIDENCE" >/dev/null 2>&1; then
    printf 'true'
  else
    printf 'false'
  fi
}

CONTRACT_VERSION="$(json_field '.contract_version')"
PROTOCOL_CONTRACT="$(json_field '.protocol_contract')"
DOMAIN_CONTRACT="$(json_field '.domain_contract')"
SOURCE_STATUS="$(json_field '.status')"
CUTOVER_STATUS="$(json_field '.standalone_runtime_adapter_readiness.cutover_status')"
CEX_DEPENDENCY_STATUS="$(json_field '.standalone_runtime_adapter_readiness.cex_dependency_status')"
STATUS_COUNT="$(json_field '(.standalone_runtime_adapter_readiness.statuses // []) | length')"
ROUTE_RECORD_TOTAL="$(json_field '.route_records.total')"
WORLD_NODE_COUNT="$(json_field '.standalone_world_counts.nodes')"
REPOSITORY_SOURCE="$(json_field '.repository.source_of_truth')"
LEDGER_RESERVE_SOURCE="$(json_field '.ledger.reserve.source_of_truth')"
METRIC_SOURCE="$(json_field '.metric.source_of_truth')"

CONTRACT_OK="$(json_check '.contract_version == "cex_trillionnium_world_production_adapter_v1"')"
PROTOCOL_OK="$(json_check '.protocol_contract == "trillionnium_world_runtime_adapter_v1"')"
DOMAIN_OK="$(json_check '.domain_contract == "trillionnium_world_domain_v1"')"
STATUS_OK="$(json_check '.status == "cex_production_adapter_bridge_ready"')"
CUTOVER_OK="$(json_check '.standalone_runtime_adapter_readiness.cutover_status == "cex_production_impls_connected_to_standalone_traits"')"
DEPENDENCY_OK="$(json_check '.standalone_runtime_adapter_readiness.cex_dependency_status == "consumer_entry_api_depends_on_trnm_world_api_without_trnm_world_importing_cex"')"
ADAPTERS_OK="$(json_check '(.standalone_runtime_adapter_readiness.statuses // []) | length == 6 and all(.[]; .status == "cex_production_impl_connected" and .production_adapter_trait_ready == true and .source_of_truth == "cex_consumer_entry_trillionnium_world_adapters")')"
ROLES_OK="$(json_check '["identity","session_guard","ledger","repository","evidence_sink","metrics_sink"] as $required | ((.standalone_runtime_adapter_readiness.statuses // []) | map(.role)) as $roles | all($required[]; $roles | index(.))')"
REPOSITORY_OK="$(json_check '.repository.source_of_truth == "cex_league_repository_normalized_world_tables" and .repository.status == "cex_repository_bridge_ready_persistence_stays_in_existing_async_command_path"')"
LEDGER_OK="$(json_check '.ledger.reserve.source_of_truth == "cex_world_contract_and_tactics_ledger_settlement" and .ledger.release.source_of_truth == "cex_world_contract_and_tactics_ledger_settlement"')"
EVIDENCE_OK="$(json_check '.evidence.source_of_truth == "cex_world_events_contracts_commerce_tactics_records"')"
METRIC_OK="$(json_check '.metric.source_of_truth == "cex_consumer_entry_metrics_projection"')"
COUNTS_OK="$(json_check '(.standalone_world_counts.nodes // 0) > 0 and (.route_records.total // 0) > 0')"

GREEN=false
STATUS="blocked_missing_cex_adapter_readiness_evidence"
if [[ "$EVIDENCE_AVAILABLE" == "true" && -f "$RAW_EVIDENCE" ]]; then
  STATUS="cex_adapter_readiness_failed_contract_validation"
  if [[ "$CONTRACT_OK" == "true" \
    && "$PROTOCOL_OK" == "true" \
    && "$DOMAIN_OK" == "true" \
    && "$STATUS_OK" == "true" \
    && "$CUTOVER_OK" == "true" \
    && "$DEPENDENCY_OK" == "true" \
    && "$ADAPTERS_OK" == "true" \
    && "$ROLES_OK" == "true" \
    && "$REPOSITORY_OK" == "true" \
    && "$LEDGER_OK" == "true" \
    && "$EVIDENCE_OK" == "true" \
    && "$METRIC_OK" == "true" \
    && "$COUNTS_OK" == "true" ]]; then
    GREEN=true
    STATUS="cex_adapter_readiness_green"
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_cex_adapter_readiness_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg raw_evidence "$RAW_EVIDENCE" \
  --arg input_evidence "$INPUT_EVIDENCE" \
  --arg input_url "$INPUT_URL" \
  --arg fetch_status "$FETCH_STATUS" \
  --arg fetch_detail "$FETCH_DETAIL" \
  --arg source_contract_version "$CONTRACT_VERSION" \
  --arg protocol_contract "$PROTOCOL_CONTRACT" \
  --arg domain_contract "$DOMAIN_CONTRACT" \
  --arg source_status "$SOURCE_STATUS" \
  --arg cutover_status "$CUTOVER_STATUS" \
  --arg cex_dependency_status "$CEX_DEPENDENCY_STATUS" \
  --arg status_count "${STATUS_COUNT:-0}" \
  --arg route_record_total "${ROUTE_RECORD_TOTAL:-0}" \
  --arg world_node_count "${WORLD_NODE_COUNT:-0}" \
  --arg repository_source "$REPOSITORY_SOURCE" \
  --arg ledger_reserve_source "$LEDGER_RESERVE_SOURCE" \
  --arg metric_source "$METRIC_SOURCE" \
  --argjson green "$GREEN" \
  --argjson contract_ok "$CONTRACT_OK" \
  --argjson protocol_ok "$PROTOCOL_OK" \
  --argjson domain_ok "$DOMAIN_OK" \
  --argjson status_ok "$STATUS_OK" \
  --argjson cutover_ok "$CUTOVER_OK" \
  --argjson dependency_ok "$DEPENDENCY_OK" \
  --argjson adapters_ok "$ADAPTERS_OK" \
  --argjson roles_ok "$ROLES_OK" \
  --argjson repository_ok "$REPOSITORY_OK" \
  --argjson ledger_ok "$LEDGER_OK" \
  --argjson evidence_ok "$EVIDENCE_OK" \
  --argjson metric_ok "$METRIC_OK" \
  --argjson counts_ok "$COUNTS_OK" \
  '{
    contract_version: $contract_version,
    status: $status,
    green: $green,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_cex_adapter_readiness_gate",
    cex_import_rule: "trillionnium_world_crates_do_not_import_cex_service_internals; cex_runtime_exports_json_evidence_for_trillionnium_release_review",
    raw_evidence_path: $raw_evidence,
    input: {
      evidence_path: $input_evidence,
      url: $input_url,
      fetch_status: $fetch_status,
      fetch_detail: $fetch_detail
    },
    observed: {
      contract_version: $source_contract_version,
      protocol_contract: $protocol_contract,
      domain_contract: $domain_contract,
      status: $source_status,
      cutover_status: $cutover_status,
      cex_dependency_status: $cex_dependency_status,
      status_count: ($status_count | tonumber),
      route_record_total: ($route_record_total | tonumber),
      world_node_count: ($world_node_count | tonumber),
      repository_source: $repository_source,
      ledger_reserve_source: $ledger_reserve_source,
      metric_source: $metric_source
    },
    checks: {
      contract_ok: $contract_ok,
      protocol_ok: $protocol_ok,
      domain_ok: $domain_ok,
      status_ok: $status_ok,
      cutover_ok: $cutover_ok,
      dependency_ok: $dependency_ok,
      adapters_ok: $adapters_ok,
      roles_ok: $roles_ok,
      repository_ok: $repository_ok,
      ledger_ok: $ledger_ok,
      evidence_ok: $evidence_ok,
      metric_ok: $metric_ok,
      counts_ok: $counts_ok
    }
  }' >"$SUMMARY_FILE"

if [[ "$GREEN" == "true" ]]; then
  printf 'TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_GREEN %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
if [[ "$REQUIRE_READY" -eq 1 ]]; then
  exit 1
fi
