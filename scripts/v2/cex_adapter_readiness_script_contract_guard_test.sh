#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh"

required_lines=(
  'trillionnium_world_cex_adapter_readiness_gate_v1'
  'cex_trillionnium_world_production_adapter_v1'
  'trillionnium_world_runtime_adapter_v1'
  'trillionnium_world_domain_v1'
  'cex_production_adapter_bridge_ready'
  'cex_production_impls_connected_to_standalone_traits'
  'consumer_entry_api_depends_on_trnm_world_api_without_trnm_world_importing_cex'
  'cex_consumer_entry_trillionnium_world_adapters'
  'cex_league_repository_normalized_world_tables'
  'TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_EVIDENCE'
  'TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_URL'
  'TRILLIONNIUM_WORLD_CEX_ADAPTER_READINESS_GREEN'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] CEX adapter readiness script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] CEX adapter readiness script validates CEX production adapter evidence without importing CEX internals"
