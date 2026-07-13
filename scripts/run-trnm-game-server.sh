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
export TRNM_ENTITLEMENT_ED25519_KEY_ID="${TRNM_ENTITLEMENT_ED25519_KEY_ID:-trnm-online-ed25519-v1}"
TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE="${TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE:-$ROOT_DIR/run/online-authority/ed25519-private-seed.base64}"
if [[ ! -s "$TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE" ]]; then
  echo "missing Online Authority Ed25519 private key; run scripts/init-trnm-online-authority-keys.sh" >&2
  exit 1
fi
export TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64="$(tr -d '\r\n' <"$TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE")"
export TRNM_ASSET_ROOT="${TRNM_ASSET_ROOT:-$ROOT_DIR/assets}"
export TRNM_GAME_SERVER_BIND_ADDR="${TRNM_GAME_SERVER_BIND_ADDR:-127.0.0.1:7005}"
export TRNM_GAME_SERVER_TICK_MS="${TRNM_GAME_SERVER_TICK_MS:-50}"

for _ in $(seq 1 60); do
  if curl -fsS "$TRNM_CEX_LEDGER_URL/v1/trnm/economy/readiness" >/dev/null 2>&1; then
    exec "$ROOT_DIR/target/release/trnm-game-server"
  fi
  sleep 1
done
echo "CEX did not become ready for TRNM Online Authority" >&2
exit 1
