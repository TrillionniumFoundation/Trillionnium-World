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
export TRNM_GAME_SERVER_TICK_MS="${TRNM_GAME_SERVER_TICK_MS:-100}"
export TRNM_FLEET_INSTANCE_ID="${TRNM_FLEET_INSTANCE_ID:-trnm-local-primary}"
export TRNM_FLEET_REGION="${TRNM_FLEET_REGION:-local-x230}"
export TRNM_FLEET_PUBLIC_ENDPOINT="${TRNM_FLEET_PUBLIC_ENDPOINT:-http://127.0.0.1:7005}"
if [[ -z "${TRNM_FLEET_PHYSICAL_HOST_ID:-}" ]]; then
  TRNM_FLEET_PHYSICAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)"
fi
export TRNM_FLEET_PHYSICAL_HOST_ID
export TRNM_FLEET_CAPACITY="${TRNM_FLEET_CAPACITY:-4}"
export TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE="${TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE:-600}"
export TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES="${TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES:-262144}"

delay=1
for _ in $(seq 1 8); do
  if cex_readiness="$(curl -fsS "$TRNM_CEX_LEDGER_URL/v1/trnm/economy/readiness" 2>/dev/null)" \
    && signer_readiness="$(curl -fsS "$TRNM_ENTITLEMENT_SIGNER_URL/v1/signer/readiness" 2>/dev/null)" \
    && jq -e '.status == "ok" and .postgres_healthy == true and .fail_fast == true' \
      >/dev/null <<<"$cex_readiness" \
    && jq -e '.status == "ok" and .contract_version == "trnm_entitlement_signer_v1" and
      .private_key_exported_to_game_server == false and .postgres_receipts == true' \
      >/dev/null <<<"$signer_readiness"; then
    exec "$ROOT_DIR/target/release/trnm-game-server"
  fi
  sleep "$delay"
  delay=$((delay * 2))
  (( delay > 15 )) && delay=15
done
echo "CEX/signer compatibility did not become ready for TRNM Online Authority" >&2
exit 1
