#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_client_boundary.sh"

while IFS= read -r line; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] client boundary script missing contract line: $line" >&2
    exit 1
  fi
done <<'REQUIRED_LINES'
trillionnium_world_client_boundary_v1
client-boundary-cleanliness.json
TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_SUMMARY
trillionnium/crates/trnm-world-bevy
scripts/run_trillionnium_world_bevy_client.sh
legacy_adapter_evidence_and_migration_reference_only_not_player_client
account_logic_may_migrate_from_cex_but_product_api_must_be_trillionnium_owned_and_consumed_by_trnm_world_bevy
cex_runtime_player_client_allowed: false
bevy_crate_has_no_cex_internals
bevy_runner_has_no_cex_runtime_refs
TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_GREEN
REQUIRED_LINES

echo "[PASS] client boundary script pins Bevy native client ownership and blocks CEX runtime drift"
