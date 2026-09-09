#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${TRNM_WORLD_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"
STATE_ROOT="${TRNM_STATE_HOME:-$ROOT_DIR/run}"

fail() {
  printf 'trnm-game-server: %s\n' "$*" >&2
  exit 1
}

require_env() {
  local name="$1"
  [[ -n "${!name:-}" ]] || fail "$name is required"
}

require_min_length() {
  local name="$1"
  local minimum="$2"
  local value="${!name:-}"
  (( ${#value} >= minimum )) || fail "$name must contain at least $minimum characters"
}

require_distinct_secret() {
  local first_name="$1"
  local second_name="$2"
  [[ "${!first_name}" != "${!second_name}" ]] \
    || fail "$first_name and $second_name must be independently generated credentials"
}

select_release_binary() {
  local binary_name="$1"
  local default_release_dir="$ROOT_DIR/run/releases/trnm-game-server/current"
  local release_dir=""

  if [[ "${TRNM_GAME_SERVER_RELEASE_DIR+x}" == "x" ]]; then
    [[ -n "$TRNM_GAME_SERVER_RELEASE_DIR" ]] \
      || fail "TRNM_GAME_SERVER_RELEASE_DIR was explicitly set to an empty path"
    release_dir="$TRNM_GAME_SERVER_RELEASE_DIR"
  elif [[ -e "$default_release_dir" || -L "$default_release_dir" ]]; then
    release_dir="$default_release_dir"
  elif [[ "${TRNM_ALLOW_DEV_BINARY:-0}" == "1" ]]; then
    local development_binary="$ROOT_DIR/target/release/$binary_name"
    [[ -f "$development_binary" && -x "$development_binary" ]] \
      || fail "explicit development binary is missing or not executable: $development_binary"
    printf '%s\n' "$development_binary"
    return
  else
    fail "no verified release is selected; set TRNM_GAME_SERVER_RELEASE_DIR or explicitly opt into local development with TRNM_ALLOW_DEV_BINARY=1"
  fi

  [[ -e "$release_dir" || -L "$release_dir" ]] \
    || fail "selected release path does not exist: $release_dir"
  local verification
  verification="$("$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir")" \
    || fail "selected release failed verification: $release_dir"
  release_dir="$(
    jq -er \
      'select(.verified == true and .contract_version == "trnm_game_server_release_verification_v1") | .release_dir' \
      <<<"$verification"
  )" || fail "release verification did not return an accepted release directory"
  local selected="$release_dir/$binary_name"
  [[ -f "$selected" && -x "$selected" ]] \
    || fail "verified release does not contain executable $binary_name"
  printf '%s\n' "$selected"
}

command -v jq >/dev/null 2>&1 || fail "jq is required"
command -v curl >/dev/null 2>&1 || fail "curl is required"

require_env DATABASE_URL
export DATABASE_URL
export TRNM_PUBLISHED_TICK_JOURNAL_DIR="${TRNM_PUBLISHED_TICK_JOURNAL_DIR:-$STATE_ROOT/game-server/published-ticks}"
export TRNM_FLEET_INSTANCE_ID="${TRNM_FLEET_INSTANCE_ID:-trnm-local-primary}"
if [[ -z "${TRNM_FLEET_PHYSICAL_HOST_ID:-}" ]]; then
  [[ -r /etc/machine-id ]] \
    || fail "TRNM_FLEET_PHYSICAL_HOST_ID is required when /etc/machine-id is unavailable"
  command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required to derive the local host identity"
  TRNM_FLEET_PHYSICAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)"
fi
export TRNM_FLEET_PHYSICAL_HOST_ID

mkdir -p "$TRNM_PUBLISHED_TICK_JOURNAL_DIR"
chmod 700 "$TRNM_PUBLISHED_TICK_JOURNAL_DIR"

game_server_binary="$(select_release_binary trnm-game-server)"

# Maintenance owns the journal and PostgreSQL host fences itself and must remain
# available while CEX or the isolated signer is degraded. It still requires an
# explicitly verified release unless local development was opted into above.
if [[ "${1:-}" == "--maintenance-fail-close" ]]; then
  exec "$game_server_binary" "$@"
fi

require_env TRNM_GAME_AUTHORITY_TOKEN
require_env TRNM_MODERATOR_TOKEN
require_env TRNM_ENTITLEMENT_SIGNER_TOKEN
require_min_length TRNM_GAME_AUTHORITY_TOKEN 24
require_min_length TRNM_MODERATOR_TOKEN 24
require_min_length TRNM_ENTITLEMENT_SIGNER_TOKEN 32
require_distinct_secret TRNM_GAME_AUTHORITY_TOKEN TRNM_MODERATOR_TOKEN
require_distinct_secret TRNM_GAME_AUTHORITY_TOKEN TRNM_ENTITLEMENT_SIGNER_TOKEN
require_distinct_secret TRNM_MODERATOR_TOKEN TRNM_ENTITLEMENT_SIGNER_TOKEN

export TRNM_CEX_LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
export TRNM_ENTITLEMENT_SIGNER_URL="${TRNM_ENTITLEMENT_SIGNER_URL:-http://127.0.0.1:7010}"
export TRNM_ASSET_ROOT="${TRNM_ASSET_ROOT:-$ROOT_DIR/assets}"
export TRNM_GAME_SERVER_BIND_ADDR="${TRNM_GAME_SERVER_BIND_ADDR:-127.0.0.1:7005}"
export TRNM_GAME_SERVER_TICK_MS="${TRNM_GAME_SERVER_TICK_MS:-100}"
export TRNM_FLEET_REGION="${TRNM_FLEET_REGION:-local}"
export TRNM_FLEET_PUBLIC_ENDPOINT="${TRNM_FLEET_PUBLIC_ENDPOINT:-http://127.0.0.1:7005}"
export TRNM_FLEET_CAPACITY="${TRNM_FLEET_CAPACITY:-4}"
export TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE="${TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE:-600}"
export TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES="${TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES:-262144}"

[[ -d "$TRNM_ASSET_ROOT" ]] || fail "TRNM_ASSET_ROOT is not a directory: $TRNM_ASSET_ROOT"

# Bound dependency wait: service startup fails closed instead of running with a
# memory fallback or an unverified signer/issuer registry.
delay=1
for _ in $(seq 1 8); do
  if cex_readiness="$(curl -fsS --max-time 5 "$TRNM_CEX_LEDGER_URL/v1/trnm/economy/readiness" 2>/dev/null)" \
    && signer_readiness="$(curl -fsS --max-time 5 "$TRNM_ENTITLEMENT_SIGNER_URL/v1/signer/readiness" 2>/dev/null)" \
    && jq -e '.status == "ok" and .postgres_healthy == true and .fail_fast == true' \
      >/dev/null <<<"$cex_readiness" \
    && jq -e '.status == "ok" and .contract_version == "trnm_entitlement_signer_v1" and
      .private_key_exported_to_game_server == false and .postgres_receipts == true' \
      >/dev/null <<<"$signer_readiness"; then
    exec "$game_server_binary" "$@"
  fi
  sleep "$delay"
  delay=$((delay * 2))
  (( delay > 15 )) && delay=15
done
fail "CEX/signer compatibility did not become ready for the World compatibility authority enclave"
