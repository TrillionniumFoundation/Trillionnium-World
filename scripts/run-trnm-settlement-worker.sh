#!/usr/bin/env bash
set -euo pipefail
builtin umask 077

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
ROOT_DIR="${TRNM_WORLD_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd -P)}"

fail() {
  printf 'trnm-settlement-worker: %s\n' "$*" >&2
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

for command in realpath sha256sum stat; do
  command -v "$command" >/dev/null 2>&1 \
    || fail "required command is unavailable: $command"
done

require_env DATABASE_URL
require_env TRNM_GAME_AUTHORITY_TOKEN
require_env TRNM_ENTITLEMENT_SIGNER_TOKEN
require_min_length TRNM_GAME_AUTHORITY_TOKEN 24
require_min_length TRNM_ENTITLEMENT_SIGNER_TOKEN 32
[[ "$TRNM_GAME_AUTHORITY_TOKEN" != "$TRNM_ENTITLEMENT_SIGNER_TOKEN" ]] \
  || fail "game-authority and signer-service credentials must be distinct"

export TRNM_CEX_LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
export TRNM_ENTITLEMENT_SIGNER_URL="${TRNM_ENTITLEMENT_SIGNER_URL:-http://127.0.0.1:7010}"
export TRNM_SETTLEMENT_WORKER_ID="${TRNM_SETTLEMENT_WORKER_ID:-trnm-settlement-primary}"
export TRNM_SETTLEMENT_BATCH_SIZE="${TRNM_SETTLEMENT_BATCH_SIZE:-8}"
export TRNM_SETTLEMENT_LEASE_MILLISECONDS="${TRNM_SETTLEMENT_LEASE_MILLISECONDS:-120000}"
export TRNM_SETTLEMENT_POLL_MILLISECONDS="${TRNM_SETTLEMENT_POLL_MILLISECONDS:-250}"
export TRNM_SETTLEMENT_DATABASE_MAX_CONNECTIONS="${TRNM_SETTLEMENT_DATABASE_MAX_CONNECTIONS:-4}"

select_binary() {
  local candidate="${TRNM_SETTLEMENT_WORKER_BINARY:-}"
  if [[ -z "$candidate" ]]; then
    if [[ "${TRNM_ALLOW_DEV_BINARY:-0}" != "1" ]]; then
      fail "TRNM_SETTLEMENT_WORKER_BINARY is required unless TRNM_ALLOW_DEV_BINARY=1"
    fi
    candidate="$ROOT_DIR/target/release/trnm-settlement-worker"
  fi

  candidate="$(realpath -e -- "$candidate" 2>/dev/null)" \
    || fail "settlement worker binary does not exist"
  [[ -f "$candidate" && ! -L "$candidate" && -x "$candidate" ]] \
    || fail "settlement worker binary must be a regular executable: $candidate"
  [[ "$(stat -c '%h' -- "$candidate")" == "1" ]] \
    || fail "settlement worker binary must not be an externally mutable hard link"

  if [[ "${TRNM_ALLOW_DEV_BINARY:-0}" != "1" ]]; then
    require_env TRNM_SETTLEMENT_WORKER_SHA256
    [[ "$TRNM_SETTLEMENT_WORKER_SHA256" =~ ^[0-9a-f]{64}$ ]] \
      || fail "TRNM_SETTLEMENT_WORKER_SHA256 must contain 64 lowercase hex characters"
    local actual_sha
    actual_sha="$(sha256sum "$candidate" | awk '{print $1}')"
    [[ "$actual_sha" == "$TRNM_SETTLEMENT_WORKER_SHA256" ]] \
      || fail "settlement worker binary digest mismatch"
  fi
  printf '%s\n' "$candidate"
}

worker_binary="$(select_binary)"
exec "$worker_binary"
