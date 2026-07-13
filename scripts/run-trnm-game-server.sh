#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

export DATABASE_URL="$(cex_effective_database_url)"
export TRNM_CEX_LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
export TRNM_GAME_AUTHORITY_TOKEN="${TRNM_GAME_AUTHORITY_TOKEN:-trnm-game-authority-v1:$IDENTITY_ADMIN_TOKEN}"
export TRNM_MODERATOR_TOKEN="${TRNM_MODERATOR_TOKEN:-trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN}"
export TRNM_ENTITLEMENT_SIGNER_URL="${TRNM_ENTITLEMENT_SIGNER_URL:-http://127.0.0.1:7010}"
export TRNM_ENTITLEMENT_SIGNER_TOKEN="${TRNM_ENTITLEMENT_SIGNER_TOKEN:-trnm-isolated-signer-v1:$IDENTITY_ADMIN_TOKEN}"
export TRNM_ASSET_ROOT="${TRNM_ASSET_ROOT:-$ROOT_DIR/assets}"
export TRNM_GAME_SERVER_BIND_ADDR="${TRNM_GAME_SERVER_BIND_ADDR:-127.0.0.1:7005}"
export TRNM_GAME_SERVER_TICK_MS="${TRNM_GAME_SERVER_TICK_MS:-50}"
export TRNM_FLEET_INSTANCE_ID="${TRNM_FLEET_INSTANCE_ID:-trnm-local-primary}"
export TRNM_FLEET_REGION="${TRNM_FLEET_REGION:-local-x230}"
export TRNM_FLEET_PUBLIC_ENDPOINT="${TRNM_FLEET_PUBLIC_ENDPOINT:-http://127.0.0.1:7005}"
if [[ -z "${TRNM_FLEET_PHYSICAL_HOST_ID:-}" ]]; then
  TRNM_FLEET_PHYSICAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)"
fi
export TRNM_FLEET_PHYSICAL_HOST_ID
export TRNM_FLEET_CAPACITY="${TRNM_FLEET_CAPACITY:-32}"
export TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE="${TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE:-600}"
export TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES="${TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES:-262144}"

for _ in $(seq 1 60); do
  if curl -fsS "$TRNM_CEX_LEDGER_URL/v1/trnm/economy/readiness" >/dev/null 2>&1 \
    && curl -fsS "$TRNM_ENTITLEMENT_SIGNER_URL/v1/signer/readiness" >/dev/null 2>&1; then
    exec "$ROOT_DIR/target/release/trnm-game-server"
  fi
  sleep 1
done
echo "CEX did not become ready for TRNM Online Authority" >&2
exit 1
