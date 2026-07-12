#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_PROJECT_ROOT="${CEX_PROJECT_ROOT:-$(cd -- "$PROJECT_ROOT/../CEX" && pwd)}"
# shellcheck source=/dev/null
source "$CEX_PROJECT_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${LEDGER_BASE_URL:-http://127.0.0.1:7002}"
ADMIN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?ledger admin token required}}"
ACCOUNT="${TRNM_CEX_ACCOUNT_ID:?TRNM_CEX_ACCOUNT_ID is required}"
ACTOR="${TRNM_CEX_ACTOR_ID:-trnm-local-player}"
DEVICE="${TRNM_CEX_DEVICE_ID:-$(hostname)-native-client}"
CREDENTIAL_DIR="$PROJECT_ROOT/run/credentials"
RECOVERY_FILE="$CREDENTIAL_DIR/${ACTOR//[^a-zA-Z0-9_.-]/_}.recovery"
mkdir -p "$CREDENTIAL_DIR"
chmod 700 "$CREDENTIAL_DIR"

identity_count="$(cex_psql_stdin -Atc "select count(*) from trnm_player_identities
  where player_id = '$ACTOR' and account_id = '$ACCOUNT'")"
if [[ "$identity_count" == 0 ]]; then
  if [[ ! -s "$RECOVERY_FILE" ]]; then
    umask 077
    openssl rand -hex 32 >"$RECOVERY_FILE"
  fi
  recovery="$(<"$RECOVERY_FILE")"
  curl -fsS "$LEDGER_URL/v1/trnm/identity/register" -H "x-admin-token: $ADMIN" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg p "$ACTOR" --arg a "$ACCOUNT" --arg r "$recovery" \
      '{player_id:$p,account_id:$a,recovery_key:$r}')" >/dev/null
elif [[ ! -s "$RECOVERY_FILE" ]]; then
  echo "identity exists but the machine recovery credential is unavailable" >&2
  exit 1
fi

recovery="$(<"$RECOVERY_FILE")"
session="$(curl -fsS "$LEDGER_URL/v1/trnm/identity/session" \
  -H 'content-type: application/json' --data-binary "$(jq -cn \
    --arg p "$ACTOR" --arg r "$recovery" --arg d "$DEVICE" \
    '{player_id:$p,recovery_key:$r,device_id:$d,lifetime_seconds:86400}')" | jq -er .session_token)"
systemctl --user set-environment TRNM_CEX_PLAYER_SESSION="$session"
systemctl --user unset-environment TRNM_CEX_ENTRY_TOKEN
echo "provisioned player-scoped TRNM CEX session for $ACTOR / $ACCOUNT (24h)"
