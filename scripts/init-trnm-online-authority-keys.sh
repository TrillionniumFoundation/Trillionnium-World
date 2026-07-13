#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
KEY_DIR="${TRNM_ENTITLEMENT_KEY_DIR:-$ROOT_DIR/run/online-authority}"
SEED_FILE="${TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE:-$KEY_DIR/ed25519-private-seed.base64}"
REGISTRY_FILE="${TRNM_ENTITLEMENT_ISSUER_REGISTRY_PATH:-$KEY_DIR/issuer-registry.json}"
KEY_ID_FILE="${TRNM_ENTITLEMENT_ED25519_KEY_ID_FILE:-$KEY_DIR/active-key-id}"
if [[ -z "${TRNM_ENTITLEMENT_ED25519_KEY_ID:-}" && -s "$KEY_ID_FILE" ]]; then
  TRNM_ENTITLEMENT_ED25519_KEY_ID="$(tr -d '\r\n' <"$KEY_ID_FILE")"
fi
export TRNM_ENTITLEMENT_ED25519_KEY_ID="${TRNM_ENTITLEMENT_ED25519_KEY_ID:-trnm-online-ed25519-v1}"

umask 077
mkdir -p "$KEY_DIR"
if [[ ! -s "$SEED_FILE" ]]; then
  openssl rand -base64 32 | tr -d '\n' >"$SEED_FILE"
fi
export TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64="$(tr -d '\r\n' <"$SEED_FILE")"

tmp_registry="$REGISTRY_FILE.tmp.$$"
"$ROOT_DIR/target/release/trnm-entitlement-keygen" >"$tmp_registry"
chmod 644 "$tmp_registry"
mv -f "$tmp_registry" "$REGISTRY_FILE"
chmod 600 "$SEED_FILE"
printf '%s\n' "$TRNM_ENTITLEMENT_ED25519_KEY_ID" >"$KEY_ID_FILE"
chmod 644 "$KEY_ID_FILE"

echo "$REGISTRY_FILE"
