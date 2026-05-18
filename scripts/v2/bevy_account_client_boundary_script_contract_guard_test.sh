#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_account_client_boundary.sh"

while IFS= read -r line; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] Bevy account client boundary script missing contract line: $line" >&2
    exit 1
  fi
done <<'REQUIRED_LINES'
trillionnium_world_bevy_account_client_boundary_v1
trillionnium_world_account_api_v1
trillionnium_world_account_client_boundary_v1
bevy-account-client-boundary.json
TRILLIONNIUM_WORLD_BEVY_ACCOUNT_CLIENT_BOUNDARY_SUMMARY
cargo run -p trnm-world-bevy -- account-client-boundary
player_client_owner == "trnm-world-bevy"
account_api_owner == "trillionnium_world_account_api"
passwords_tokens_or_cookie_values_logged == false
cex_runtime_player_client_allowed == false
bevy_projection_contains_account_client == true
TRILLIONNIUM_WORLD_BEVY_ACCOUNT_CLIENT_BOUNDARY_GREEN
REQUIRED_LINES

echo "[PASS] Bevy account client boundary script gates Trillionnium-owned account API consumption without CEX runtime"
