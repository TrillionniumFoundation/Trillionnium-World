#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

export DATABASE_URL="$(cex_effective_database_url)"
export TRNM_ENTITLEMENT_SIGNER_BIND_ADDR="${TRNM_ENTITLEMENT_SIGNER_BIND_ADDR:-127.0.0.1:7010}"
export TRNM_ENTITLEMENT_SIGNER_TOKEN="${TRNM_ENTITLEMENT_SIGNER_TOKEN:-trnm-isolated-signer-v1:$IDENTITY_ADMIN_TOKEN}"
KEY_DIR="${TRNM_ENTITLEMENT_KEY_DIR:-$ROOT_DIR/run/online-authority}"
export TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE="${TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE:-$KEY_DIR/ed25519-private-seed.base64}"
KEY_ID_FILE="${TRNM_ENTITLEMENT_ED25519_KEY_ID_FILE:-$KEY_DIR/active-key-id}"

if [[ ! -s "$TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE" || ! -s "$KEY_ID_FILE" ]]; then
  echo "missing isolated signer key material; run scripts/init-trnm-online-authority-keys.sh" >&2
  exit 1
fi
export TRNM_ENTITLEMENT_ED25519_KEY_ID="$(tr -d '\r\n' <"$KEY_ID_FILE")"
exec "$ROOT_DIR/target/release/trnm-entitlement-signer"
