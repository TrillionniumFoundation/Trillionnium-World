#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
KEY_DIR="${TRNM_ENTITLEMENT_KEY_DIR:-$ROOT_DIR/run/online-authority}"
SEED_FILE="${TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE:-$KEY_DIR/ed25519-private-seed.base64}"
KEY_ID_FILE="${TRNM_ENTITLEMENT_ED25519_KEY_ID_FILE:-$KEY_DIR/active-key-id}"
REGISTRY_FILE="${TRNM_ENTITLEMENT_ISSUER_REGISTRY_PATH:-$KEY_DIR/issuer-registry.json}"
NEW_KEY_ID="${1:-trnm-online-ed25519-$(date -u +%Y%m%d%H%M%S)}"
REVOKE_OLD="${2:-}"

[[ "$NEW_KEY_ID" =~ ^[A-Za-z0-9._-]{1,100}$ ]] || {
  echo "invalid new key id" >&2
  exit 64
}
[[ -s "$SEED_FILE" && -s "$KEY_ID_FILE" && -s "$REGISTRY_FILE" ]] || {
  echo "initialize signer keys before rotation" >&2
  exit 1
}
OLD_KEY_ID="$(tr -d '\r\n' <"$KEY_ID_FILE")"
[[ "$NEW_KEY_ID" != "$OLD_KEY_ID" ]] || {
  echo "new key id must differ from active key id" >&2
  exit 64
}

umask 077
mkdir -p "$KEY_DIR/retired"
tmp_seed="$KEY_DIR/.rotate-seed.$$"
tmp_entry="$KEY_DIR/.rotate-entry.$$"
tmp_registry="$KEY_DIR/.rotate-registry.$$"
trap 'rm -f "$tmp_seed" "$tmp_entry" "$tmp_registry"' EXIT
openssl rand -base64 32 | tr -d '\n' >"$tmp_seed"
TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64="$(tr -d '\r\n' <"$tmp_seed")" \
TRNM_ENTITLEMENT_ED25519_KEY_ID="$NEW_KEY_ID" \
  "$ROOT_DIR/target/release/trnm-entitlement-keygen" >"$tmp_entry"

jq --slurpfile added "$tmp_entry" --arg old "$OLD_KEY_ID" --arg revoke "$REVOKE_OLD" \
  '.keys[$old].status = (if $revoke == "--revoke-old" then "revoked" else "active" end)
   | .keys += $added[0].keys' "$REGISTRY_FILE" >"$tmp_registry"
jq -e --arg key "$NEW_KEY_ID" '.keys[$key].status == "active"' "$tmp_registry" >/dev/null

install -m 0600 "$SEED_FILE" "$KEY_DIR/retired/$OLD_KEY_ID.seed.base64"
install -m 0600 "$tmp_seed" "$SEED_FILE"
printf '%s\n' "$NEW_KEY_ID" >"$KEY_ID_FILE"
chmod 644 "$KEY_ID_FILE"
install -m 0644 "$tmp_registry" "$REGISTRY_FILE"

systemctl --user restart trnm-entitlement-signer.service
systemctl --user restart cex-trnm-ledger.service
for _ in $(seq 1 60); do
  signer="$(curl -fsS http://127.0.0.1:7010/v1/signer/readiness 2>/dev/null || true)"
  ledger="$(curl -fsS http://127.0.0.1:7002/v1/trnm/economy/readiness 2>/dev/null || true)"
  if jq -e --arg key "$NEW_KEY_ID" '.status == "ok" and .key_id == $key' <<<"$signer" >/dev/null 2>&1 \
    && jq -e '.online_entitlement_active_issuer_keys >= 1' <<<"$ledger" >/dev/null 2>&1; then
    jq -n --arg old "$OLD_KEY_ID" --arg new "$NEW_KEY_ID" \
      --argjson old_revoked "$([[ "$REVOKE_OLD" == "--revoke-old" ]] && echo true || echo false)" \
      '{status:"rotated",old_key_id:$old,new_key_id:$new,old_key_revoked:$old_revoked,
        game_server_restart_required:false,custody:"isolated_process_file_seed_not_kms_hsm"}'
    exit 0
  fi
  sleep 1
done
echo "rotated signer or ledger did not become ready" >&2
exit 1
