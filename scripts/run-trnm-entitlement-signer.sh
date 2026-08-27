#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${TRNM_WORLD_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"
STATE_ROOT="${TRNM_STATE_HOME:-$ROOT_DIR/run}"

fail() {
  printf 'trnm-entitlement-signer: %s\n' "$*" >&2
  exit 1
}

require_env() {
  local name="$1"
  [[ -n "${!name:-}" ]] || fail "$name is required"
}

select_release_binary() {
  local default_release_dir="$ROOT_DIR/run/releases/trnm-game-server/current"
  local release_dir=""

  if [[ "${TRNM_GAME_SERVER_RELEASE_DIR+x}" == "x" ]]; then
    [[ -n "$TRNM_GAME_SERVER_RELEASE_DIR" ]] \
      || fail "TRNM_GAME_SERVER_RELEASE_DIR was explicitly set to an empty path"
    release_dir="$TRNM_GAME_SERVER_RELEASE_DIR"
  elif [[ -e "$default_release_dir" || -L "$default_release_dir" ]]; then
    release_dir="$default_release_dir"
  elif [[ "${TRNM_ALLOW_DEV_BINARY:-0}" == "1" ]]; then
    local development_binary="$ROOT_DIR/target/release/trnm-entitlement-signer"
    [[ -f "$development_binary" && -x "$development_binary" ]] \
      || fail "explicit development signer binary is missing: $development_binary"
    printf '%s\n' "$development_binary"
    return
  else
    fail "no verified signer release is selected; set TRNM_GAME_SERVER_RELEASE_DIR or explicitly opt into local development with TRNM_ALLOW_DEV_BINARY=1"
  fi

  local verification
  verification="$("$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir")" \
    || fail "selected signer release failed verification: $release_dir"
  release_dir="$(
    jq -er \
      'select(.verified == true and .contract_version == "trnm_game_server_release_verification_v1") | .release_dir' \
      <<<"$verification"
  )" || fail "release verification did not return an accepted release directory"
  local selected="$release_dir/trnm-entitlement-signer"
  [[ -f "$selected" && -x "$selected" ]] \
    || fail "verified release does not contain trnm-entitlement-signer"
  printf '%s\n' "$selected"
}

command -v jq >/dev/null 2>&1 || fail "jq is required"
require_env DATABASE_URL
require_env TRNM_ENTITLEMENT_SIGNER_TOKEN
(( ${#TRNM_ENTITLEMENT_SIGNER_TOKEN} >= 32 )) \
  || fail "TRNM_ENTITLEMENT_SIGNER_TOKEN must contain at least 32 characters"

export DATABASE_URL
export TRNM_ENTITLEMENT_SIGNER_BIND_ADDR="${TRNM_ENTITLEMENT_SIGNER_BIND_ADDR:-127.0.0.1:7010}"
KEY_DIR="${TRNM_ENTITLEMENT_KEY_DIR:-$STATE_ROOT/entitlement-signer}"
export TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE="${TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE:-$KEY_DIR/ed25519-private-seed.base64}"
KEY_ID_FILE="${TRNM_ENTITLEMENT_ED25519_KEY_ID_FILE:-$KEY_DIR/active-key-id}"

[[ -s "$TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE" ]] \
  || fail "missing isolated signer private seed: $TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE"
[[ -s "$KEY_ID_FILE" ]] || fail "missing isolated signer key ID: $KEY_ID_FILE"

private_mode="$(stat -c '%a' "$TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE")" \
  || fail "cannot inspect signer private-key permissions"
if (( (8#$private_mode & 077) != 0 )); then
  fail "signer private seed must not be group/world accessible"
fi

export TRNM_ENTITLEMENT_ED25519_KEY_ID="$(tr -d '\r\n' <"$KEY_ID_FILE")"
[[ -n "$TRNM_ENTITLEMENT_ED25519_KEY_ID" ]] || fail "isolated signer key ID is empty"

signer_binary="$(select_release_binary)"
exec "$signer_binary"
