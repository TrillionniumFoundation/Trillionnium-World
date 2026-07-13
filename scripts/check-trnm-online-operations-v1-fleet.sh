#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
PRIMARY_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
SECONDARY_URL="http://127.0.0.1:7006"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
RUN_ID="online-operations-fleet-$(date +%s)-${RANDOM}"
SLOT_PREFIX="fleet-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-operations-v1-fleet/$RUN_ID"
SECONDARY_PID=""
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  [[ -z "$SECONDARY_PID" ]] || kill "$SECONDARY_PID" >/dev/null 2>&1 || true
  cex_psql_stdin -Atc "update trnm_online_fleet_instances set status='offline'
    where instance_id='$RUN_ID-secondary'" >/dev/null 2>&1 || true
  systemctl --user start trnm-game-server.service >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local role="$1" player credential invite identity
  player="$RUN_ID-$role"
  credential="credential-$RUN_ID-$role-012345678901234567890123"
  invite="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  identity="$(curl -fsS "$LEDGER_URL/v1/trnm/product/register" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
      '{player_id:$player,credential:$credential,invite_code:$invite}')")"
  printf '%s\t%s\t%s\n' "$player" "$(jq -er .account_id <<<"$identity")" "$credential"
}

login() {
  local player="$1" credential="$2" device="$3"
  curl -fsS "$LEDGER_URL/v1/trnm/product/login" -H 'content-type: application/json' \
    --data-binary "$(jq -cn --arg player "$player" --arg credential "$credential" --arg device "$device" \
      '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')"
}

connect_campaign() {
  local url="$1" player="$2" account="$3" session="$4" slot="$5"
  curl -fsS "$url/v1/online/campaigns/connect" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg account "$account" --arg slot "$slot" \
      '{protocol_version:"trnm_online_authority_v2",build_id:"trnm-online-authority-2026.07-v2",
        player_id:$player,account_id:$account,slot_key:$slot}')"
}

join_queue() {
  local url="$1" player="$2" account="$3" session="$4" campaign="$5"
  curl -fsS "$url/v1/product/solo-queue/join" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg account "$account" --arg campaign "$campaign" \
      '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
        player_id:$player,account_id:$account,campaign_id:$campaign,map_id:"first_contact"}')"
}

TRNM_GAME_SERVER_BIND_ADDR="127.0.0.1:7006" \
TRNM_FLEET_INSTANCE_ID="$RUN_ID-secondary" \
TRNM_FLEET_REGION="local-backup" \
TRNM_FLEET_PUBLIC_ENDPOINT="$SECONDARY_URL" \
TRNM_FLEET_CAPACITY=1 TRNM_GAME_SERVER_TICK_MS=20 \
TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1 \
  "$ROOT_DIR/scripts/run-trnm-game-server.sh" >"$EVIDENCE/secondary.log" 2>&1 &
SECONDARY_PID=$!
for _ in $(seq 1 60); do
  curl -fsS "$SECONDARY_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$SECONDARY_URL/v1/online/readiness" >/dev/null

IFS=$'\t' read -r HOST HOST_ACCOUNT HOST_CREDENTIAL < <(create_identity host)
IFS=$'\t' read -r GUEST GUEST_ACCOUNT GUEST_CREDENTIAL < <(create_identity guest)
HOST_LOGIN="$(login "$HOST" "$HOST_CREDENTIAL" "$RUN_ID-host-device")"
GUEST_LOGIN="$(login "$GUEST" "$GUEST_CREDENTIAL" "$RUN_ID-guest-device")"
HOST_SESSION="$(jq -er .session_token <<<"$HOST_LOGIN")"
GUEST_SESSION="$(jq -er .session_token <<<"$GUEST_LOGIN")"
HOST_CAMPAIGN="$(connect_campaign "$PRIMARY_URL" "$HOST" "$HOST_ACCOUNT" "$HOST_SESSION" "$SLOT_PREFIX-host" | jq -er .campaign_id)"
GUEST_CAMPAIGN="$(connect_campaign "$PRIMARY_URL" "$GUEST" "$GUEST_ACCOUNT" "$GUEST_SESSION" "$SLOT_PREFIX-guest" | jq -er .campaign_id)"
join_queue "$PRIMARY_URL" "$HOST" "$HOST_ACCOUNT" "$HOST_SESSION" "$HOST_CAMPAIGN" >/dev/null
matched="$(join_queue "$PRIMARY_URL" "$GUEST" "$GUEST_ACCOUNT" "$GUEST_SESSION" "$GUEST_CAMPAIGN")"
MATCH_ID="$(jq -er '.match_id | select(length == 36)' <<<"$matched")"

assigned_before="$(cex_psql_stdin -Atc "select assigned_instance_id from trnm_online_matches
  where match_id='$MATCH_ID'::uuid")"
[[ "$assigned_before" == "trnm-local-primary" ]]
systemctl --user stop trnm-game-server.service

assigned_after=""
for _ in $(seq 1 80); do
  assigned_after="$(cex_psql_stdin -Atc "select assigned_instance_id from trnm_online_matches
    where match_id='$MATCH_ID'::uuid")"
  [[ "$assigned_after" == "$RUN_ID-secondary" ]] && break
  sleep 0.25
done
[[ "$assigned_after" == "$RUN_ID-secondary" ]]
sleep 3

capacity_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$SECONDARY_URL/v1/operations/fleet/route" -H 'content-type: application/json' \
  --data-binary '{"protocol_version":"trnm_online_operations_v1","build_id":"trnm-online-operations-2026.07-v1","preferred_region":"local-x230"}')"
[[ "$capacity_status" == "503" ]]

phase="running"
for _ in $(seq 1 120); do
  phase="$(cex_psql_stdin -Atc "select phase from trnm_online_matches where match_id='$MATCH_ID'::uuid")"
  [[ "$phase" == "complete" ]] && break
  sleep 0.25
done
[[ "$phase" == "complete" ]]

fallback="$(curl -fsS "$SECONDARY_URL/v1/operations/fleet/route" \
  -H 'content-type: application/json' \
  --data-binary '{"protocol_version":"trnm_online_operations_v1","build_id":"trnm-online-operations-2026.07-v1","preferred_region":"local-x230"}')"
jq -e --arg instance "$RUN_ID-secondary" '.selected.instance_id == $instance and
  .selected.region == "local-backup" and .cross_region_fallback == true' <<<"$fallback" >/dev/null

database="$(cex_psql_stdin -Atc "select json_build_object(
  'phase',(select phase from trnm_online_matches where match_id='$MATCH_ID'::uuid),
  'owner',(select assigned_instance_id from trnm_online_matches where match_id='$MATCH_ID'::uuid),
  'region',(select assigned_region from trnm_online_matches where match_id='$MATCH_ID'::uuid),
  'failovers',(select count(*) from trnm_online_fleet_failovers where match_id='$MATCH_ID'::uuid
    and previous_instance_id='trnm-local-primary' and new_instance_id='$RUN_ID-secondary'),
  'replays',(select count(*) from trnm_online_replay_index where match_id='$MATCH_ID'::uuid),
  'season_events',(select count(*) from trnm_online_rating_events where match_id='$MATCH_ID'::uuid
    and season_id is not null),
  'value_entitlements',(select count(*) from trnm_value_entitlements
    where entitlement_json->>'match_id'='$MATCH_ID')
)" | jq -c .)"
jq -e --arg owner "$RUN_ID-secondary" '.phase == "complete" and .owner == $owner and
  .region == "local-backup" and .failovers == 1 and .replays == 1 and
  .season_events == 2 and .value_entitlements == 0' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg match_id "$MATCH_ID" --arg assigned_before "$assigned_before" \
  --arg assigned_after "$assigned_after" --argjson fallback "$fallback" --argjson database "$database" \
  '{status:"passed",run_id:$run_id,match_id:$match_id,two_live_instances:true,
    assigned_before:$assigned_before,assigned_after:$assigned_after,heartbeat_expiry_takeover:true,
    capacity_fail_closed:true,cross_region_fallback:$fallback,database:$database,
    boundary:"same-host two-process fleet/failover evidence; not cross-host or regional HA"}' \
  | tee "$EVIDENCE/report.json"
