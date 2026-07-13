#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
RUN_ID="online-product-$(date +%s)-${RANDOM}"
PRODUCT_PROTOCOL="trnm_online_product_v1"
PRODUCT_BUILD="trnm-online-product-2026.07-v1"
AUTHORITY_PROTOCOL="trnm_online_authority_v2"
AUTHORITY_BUILD="trnm-online-authority-2026.07-v2"

cleanup() {
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
}
trap cleanup EXIT

json_post() {
  local url="$1" body="$2"
  curl -fsS "$url" -H 'content-type: application/json' --data-binary "$body"
}

admin_post() {
  local path="$1" body="$2"
  curl -fsS "$LEDGER_URL$path" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$body"
}

player_post() {
  local session="$1" path="$2" body="$3"
  curl -fsS "$ONLINE_URL$path" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$body"
}

expect_status() {
  local expected="$1" url="$2" body="$3" header_name="${4:-}" header_value="${5:-}" status
  args=(-sS -o /dev/null -w '%{http_code}' -H 'content-type: application/json' --data-binary "$body")
  if [[ -n "$header_name" ]]; then
    args+=(-H "$header_name: $header_value")
  fi
  status="$(curl "${args[@]}" "$url")"
  [[ "$status" == "$expected" ]] || {
    echo "expected HTTP $expected from $url, received $status" >&2
    return 1
  }
}

register_player() {
  local role="$1" player credential identity invite_code
  player="$RUN_ID-$role"
  credential="credential-$RUN_ID-$role-012345678901234567890123"
  invite_code="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  identity="$(json_post "$LEDGER_URL/v1/trnm/product/register" "$(jq -cn \
    --arg player "$player" --arg credential "$credential" --arg invite "$invite_code" \
    '{player_id:$player,credential:$credential,invite_code:$invite}')")"
  printf '%s\t%s\t%s\n' "$player" "$(jq -er .account_id <<<"$identity")" "$credential"
}

login_player() {
  local player="$1" credential="$2" device="$3"
  json_post "$LEDGER_URL/v1/trnm/product/login" "$(jq -cn \
    --arg player "$player" --arg credential "$credential" --arg device "$device" \
    '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')" \
    | jq -er .session_token
}

expect_status 401 "$LEDGER_URL/v1/trnm/product/register" "$(jq -cn \
  --arg player "$RUN_ID-invalid" --arg credential "invalid-invite-credential-012345678901234567890123" \
  '{player_id:$player,credential:$credential,invite_code:"invalid"}')"
ONE_TIME_INVITE="$(admin_post /v1/trnm/product/registration-invites \
  '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
json_post "$LEDGER_URL/v1/trnm/product/register" "$(jq -cn \
  --arg player "$RUN_ID-invite-probe-a" \
  --arg credential "invite-probe-a-credential-012345678901234567890123" \
  --arg invite "$ONE_TIME_INVITE" \
  '{player_id:$player,credential:$credential,invite_code:$invite}')" >/dev/null
expect_status 401 "$LEDGER_URL/v1/trnm/product/register" "$(jq -cn \
  --arg player "$RUN_ID-invite-probe-b" \
  --arg credential "invite-probe-b-credential-012345678901234567890123" \
  --arg invite "$ONE_TIME_INVITE" \
  '{player_id:$player,credential:$credential,invite_code:$invite}')"

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_CREDENTIAL < <(register_player host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_CREDENTIAL < <(register_player guest)
IFS=$'\t' read -r INTRUDER_PLAYER INTRUDER_ACCOUNT INTRUDER_CREDENTIAL < <(register_player intruder)
IFS=$'\t' read -r LOCK_PLAYER _LOCK_ACCOUNT LOCK_CREDENTIAL < <(register_player lock-probe)
for attempt in $(seq 1 5); do
  expect_status 401 "$LEDGER_URL/v1/trnm/product/login" "$(jq -cn \
    --arg player "$LOCK_PLAYER" --arg credential "wrong-login-credential-012345678901234567890123-$attempt" \
    '{player_id:$player,credential:$credential,device_id:"rate-limit-probe",lifetime_seconds:3600}')"
done
expect_status 401 "$LEDGER_URL/v1/trnm/product/login" "$(jq -cn \
  --arg player "$LOCK_PLAYER" --arg credential "$LOCK_CREDENTIAL" \
  '{player_id:$player,credential:$credential,device_id:"rate-limit-probe",lifetime_seconds:3600}')"

HOST_SESSION="$(login_player "$HOST_PLAYER" "$HOST_CREDENTIAL" "$RUN_ID-host-device")"
GUEST_OLD_SESSION="$(login_player "$GUEST_PLAYER" "$GUEST_CREDENTIAL" "$RUN_ID-guest-device-a")"
INTRUDER_SESSION="$(login_player "$INTRUDER_PLAYER" "$INTRUDER_CREDENTIAL" "$RUN_ID-intruder-device")"

GUEST_NEW_CREDENTIAL="credential-$RUN_ID-guest-rotated-012345678901234567890123"
json_post "$LEDGER_URL/v1/trnm/product/credentials/rotate" "$(jq -cn \
  --arg player "$GUEST_PLAYER" --arg credential "$GUEST_CREDENTIAL" --arg next "$GUEST_NEW_CREDENTIAL" \
  '{player_id:$player,credential:$credential,new_credential:$next}')" >/dev/null
expect_status 401 "$LEDGER_URL/v1/trnm/identity/session/verify" "$(jq -cn \
  --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
  '{player_id:$player,account_id:$account}')" 'x-trnm-player-session' "$GUEST_OLD_SESSION"
GUEST_SESSION="$(login_player "$GUEST_PLAYER" "$GUEST_NEW_CREDENTIAL" "$RUN_ID-guest-device-b")"

admin_post /v1/trnm/identity/status "$(jq -cn --arg player "$GUEST_PLAYER" \
  '{player_id:$player,status:"suspended"}')" >/dev/null
expect_status 401 "$LEDGER_URL/v1/trnm/product/login" "$(jq -cn \
  --arg player "$GUEST_PLAYER" --arg credential "$GUEST_NEW_CREDENTIAL" \
  '{player_id:$player,credential:$credential,device_id:"blocked-device",lifetime_seconds:3600}')"
appeal="$(json_post "$LEDGER_URL/v1/trnm/product/appeals" "$(jq -cn \
  --arg player "$GUEST_PLAYER" --arg credential "$GUEST_NEW_CREDENTIAL" \
  '{player_id:$player,credential:$credential,message:"Please review this closed-alpha suspension with the attached local test evidence."}')")"
admin_post /v1/trnm/product/appeals/resolve "$(jq -cn \
  --arg appeal "$(jq -er .appeal_id <<<"$appeal")" \
  '{appeal_id:$appeal,decision:"approved",resolution:"Automated closed-alpha appeal drill approved after identity ownership verification."}')" >/dev/null
GUEST_SESSION="$(login_player "$GUEST_PLAYER" "$GUEST_NEW_CREDENTIAL" "$RUN_ID-guest-device-c")"

systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=40
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" | jq -e \
  '.status == "ok" and .online_product_protocol == "trnm_online_product_v2"' >/dev/null

connect_campaign() {
  local session="$1" player="$2" account="$3"
  player_post "$session" /v1/online/campaigns/connect "$(jq -cn \
    --arg protocol "$AUTHORITY_PROTOCOL" --arg build "$AUTHORITY_BUILD" \
    --arg player "$player" --arg account "$account" --arg slot "$RUN_ID" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')"
}

HOST_CAMPAIGN="$(connect_campaign "$HOST_SESSION" "$HOST_PLAYER" "$HOST_ACCOUNT")"
GUEST_CAMPAIGN="$(connect_campaign "$GUEST_SESSION" "$GUEST_PLAYER" "$GUEST_ACCOUNT")"
INTRUDER_CAMPAIGN="$(connect_campaign "$INTRUDER_SESSION" "$INTRUDER_PLAYER" "$INTRUDER_ACCOUNT")"

lobby="$(player_post "$HOST_SESSION" /v1/product/lobbies "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
  --arg campaign "$(jq -er .campaign_id <<<"$HOST_CAMPAIGN")" --arg name "$RUN_ID party" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,display_name:$name,map_id:"first_contact"}')")"
LOBBY_ID="$(jq -er .lobby_id <<<"$lobby")"
expect_status 409 "$ONLINE_URL/v1/product/lobbies" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
  --arg campaign "$(jq -er .campaign_id <<<"$HOST_CAMPAIGN")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,display_name:"duplicate active lobby",map_id:"first_contact"}')" \
  'x-trnm-player-session' "$HOST_SESSION"
invite="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/invites" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" --arg target "$GUEST_PLAYER" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,target_player_id:$target,expected_lobby_revision:0}')")"

expect_status 403 "$ONLINE_URL/v1/product/lobbies/invites/accept" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$INTRUDER_PLAYER" --arg account "$INTRUDER_ACCOUNT" \
  --arg campaign "$(jq -er .campaign_id <<<"$INTRUDER_CAMPAIGN")" \
  --arg token "$(jq -er .invite_token <<<"$invite")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,invite_token:$token}')" \
  'x-trnm-player-session' "$INTRUDER_SESSION"

lobby="$(player_post "$GUEST_SESSION" /v1/product/lobbies/invites/accept "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
  --arg campaign "$(jq -er .campaign_id <<<"$GUEST_CAMPAIGN")" \
  --arg token "$(jq -er .invite_token <<<"$invite")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,invite_token:$token}')")"
guest_lobby_view="$(player_post "$GUEST_SESSION" "/v1/product/lobbies/$LOBBY_ID/view" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account}')")"
jq -e '(.members | length) == 2 and .status == "open"' <<<"$guest_lobby_view" >/dev/null
expect_status 403 "$ONLINE_URL/v1/product/lobbies/$LOBBY_ID/view" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$INTRUDER_PLAYER" --arg account "$INTRUDER_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account}')" \
  'x-trnm-player-session' "$INTRUDER_SESSION"

lobby="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:1}')")"
lobby="$(player_post "$GUEST_SESSION" "/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:2}')")"
expect_status 409 "$ONLINE_URL/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:1}')" \
  'x-trnm-player-session' "$HOST_SESSION"
expect_status 403 "$ONLINE_URL/v1/product/lobbies/$LOBBY_ID/queue" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,expected_lobby_revision:3}')" \
  'x-trnm-player-session' "$GUEST_SESSION"

allocation="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/queue" "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
  --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,expected_lobby_revision:3}')")"
MATCH_ID="$(jq -er '.match_view.match_id' <<<"$allocation")"
jq -e '.lobby.status == "matched" and .match_view.phase == "running" and
  (.match_view.members | length) == 2' \
  <<<"$allocation" >/dev/null

authority="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_ONLINE_SLOT_KEY="$RUN_ID" \
  TRNM_ONLINE_EXISTING_MATCH_ID="$MATCH_ID" \
  TRNM_ONLINE_HOST_PLAYER_ID="$HOST_PLAYER" TRNM_ONLINE_HOST_ACCOUNT_ID="$HOST_ACCOUNT" \
  TRNM_ONLINE_HOST_SESSION="$HOST_SESSION" \
  TRNM_ONLINE_GUEST_PLAYER_ID="$GUEST_PLAYER" TRNM_ONLINE_GUEST_ACCOUNT_ID="$GUEST_ACCOUNT" \
  TRNM_ONLINE_GUEST_SESSION="$GUEST_SESSION" \
  "$ROOT_DIR/target/release/trnm-online-e2e")"

host_wallet="$(curl -fsS "$LEDGER_URL/v1/trnm/economy/wallet" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
    '{actor_id:$player,account_id:$account,reconciliation_cursor:0}')")"
guest_wallet="$(curl -fsS "$LEDGER_URL/v1/trnm/economy/wallet" \
  -H "x-trnm-player-session: $GUEST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
    '{actor_id:$player,account_id:$account,reconciliation_cursor:0}')")"
jq -e '.available_credits == 25 and .reserved_credits == 0' <<<"$host_wallet" >/dev/null
jq -e '.available_credits == 25 and .reserved_credits == 0' <<<"$guest_wallet" >/dev/null

database="$(cex_psql_stdin -Atc "select json_build_object(
  'lobby_status',(select status from trnm_online_lobbies where lobby_id = '$LOBBY_ID'::uuid),
  'lobby_members',(select count(*) from trnm_online_lobby_members where lobby_id = '$LOBBY_ID'::uuid),
  'allocations',(select count(*) from trnm_online_matchmaking_allocations where lobby_id = '$LOBBY_ID'::uuid and match_id = '$MATCH_ID'::uuid),
  'appeals_approved',(select count(*) from trnm_identity_appeals where player_id = '$GUEST_PLAYER' and status = 'approved'),
  'guest_generation',(select recovery_generation from trnm_player_identities where player_id = '$GUEST_PLAYER'),
  'guest_credential_argon2id',(select recovery_key_hash like '\$argon2id\$%' from trnm_player_identities where player_id = '$GUEST_PLAYER'),
  'guest_status',(select status from trnm_player_identities where player_id = '$GUEST_PLAYER'),
  'progression_events',(select count(*) from trnm_online_progression_events where match_id = '$MATCH_ID'::uuid),
  'ed25519_entitlements',(select count(*) from trnm_value_entitlements where entitlement_json->>'match_id' = '$MATCH_ID')
)" | jq -c .)"
jq -e '.lobby_status == "matched" and .lobby_members == 2 and .allocations == 1 and
  .appeals_approved == 1 and .guest_generation == 2 and .guest_status == "active" and
  .guest_credential_argon2id == true and
  .progression_events == 2 and .ed25519_entitlements == 2' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg lobby_id "$LOBBY_ID" --arg match_id "$MATCH_ID" \
  --argjson authority "$authority" --argjson host_wallet "$host_wallet" \
  --argjson guest_wallet "$guest_wallet" --argjson database "$database" \
  '{status:"passed",run_id:$run_id,lobby_id:$lobby_id,match_id:$match_id,
    closed_alpha_registration:true,consumed_registration_invite_rejected:true,
    durable_login_rate_limit:true,
    credential_rotation:true,suspension_appeal_reactivation:true,
    stolen_invite_rejected:true,stale_lobby_revision_rejected:true,non_owner_queue_rejected:true,
    duplicate_active_lobby_rejected:true,
    private_party_matchmaking:true,authority:$authority,host_wallet:$host_wallet,
    guest_wallet:$guest_wallet,database:$database}'
