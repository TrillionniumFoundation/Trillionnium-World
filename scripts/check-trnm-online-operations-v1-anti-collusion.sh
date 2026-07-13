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
RUN_ID="online-operations-collusion-$(date +%s)-${RANDOM}"
MAP_ID="cinder_crown"

cleanup() {
  local status=$?
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS TRNM_ALLOW_ACCELERATED_TEST_CLOCK >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
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
  identity="$(curl -fsS "$LEDGER_URL/v1/trnm/product/register" -H 'content-type: application/json' \
    --data-binary "$(jq -cn --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
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
  local player="$1" account="$2" session="$3" slot="$4"
  curl -fsS "$ONLINE_URL/v1/online/campaigns/connect" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn --arg player "$player" \
      --arg account "$account" --arg slot "$slot" \
      '{protocol_version:"trnm_online_authority_v2",build_id:"trnm-online-authority-2026.07-v2",
        player_id:$player,account_id:$account,slot_key:$slot}')" | jq -er .campaign_id
}

queue() {
  local player="$1" account="$2" session="$3" campaign="$4"
  curl -fsS "$ONLINE_URL/v1/product/solo-queue/join" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn --arg player "$player" \
      --arg account "$account" --arg campaign "$campaign" --arg map "$MAP_ID" \
      '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
        player_id:$player,account_id:$account,campaign_id:$campaign,map_id:$map}')"
}

cancel() {
  local player="$1" account="$2" session="$3"
  curl -fsS "$ONLINE_URL/v1/product/solo-queue/cancel" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn --arg player "$player" --arg account "$account" \
      '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
        player_id:$player,account_id:$account}')" >/dev/null
}

systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=5 \
  TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break; sleep 1; done

IFS=$'\t' read -r A A_ACCOUNT A_CREDENTIAL < <(create_identity a)
IFS=$'\t' read -r B B_ACCOUNT B_CREDENTIAL < <(create_identity b)
A_SHARED="$(login "$A" "$A_CREDENTIAL" "$RUN_ID-shared-device")"
B_SHARED="$(login "$B" "$B_CREDENTIAL" "$RUN_ID-shared-device")"
A_SESSION="$(jq -er .session_token <<<"$A_SHARED")"
B_SESSION="$(jq -er .session_token <<<"$B_SHARED")"
A_CAMPAIGN="$(connect_campaign "$A" "$A_ACCOUNT" "$A_SESSION" "collude-${RANDOM}-a")"
B_CAMPAIGN="$(connect_campaign "$B" "$B_ACCOUNT" "$B_SESSION" "collude-${RANDOM}-b")"
same_a="$(queue "$A" "$A_ACCOUNT" "$A_SESSION" "$A_CAMPAIGN")"
same_b="$(queue "$B" "$B_ACCOUNT" "$B_SESSION" "$B_CAMPAIGN")"
jq -e '.status == "queued" and .match_id == null' <<<"$same_a" >/dev/null
jq -e '.status == "queued" and .match_id == null' <<<"$same_b" >/dev/null
cancel "$A" "$A_ACCOUNT" "$A_SESSION"
cancel "$B" "$B_ACCOUNT" "$B_SESSION"

A_SESSION="$(login "$A" "$A_CREDENTIAL" "$RUN_ID-a-device" | jq -er .session_token)"
B_SESSION="$(login "$B" "$B_CREDENTIAL" "$RUN_ID-b-device" | jq -er .session_token)"
matches=()
for round in 1 2 3; do
  queue "$A" "$A_ACCOUNT" "$A_SESSION" "$A_CAMPAIGN" >/dev/null
  paired="$(queue "$B" "$B_ACCOUNT" "$B_SESSION" "$B_CAMPAIGN")"
  match_id="$(jq -er '.match_id | select(length == 36)' <<<"$paired")"
  matches+=("$match_id")
  phase="running"
  for _ in $(seq 1 120); do
    phase="$(cex_psql_stdin -Atc "select phase from trnm_online_matches where match_id='$match_id'::uuid")"
    [[ "$phase" == "complete" ]] && break
    sleep 0.25
  done
  [[ "$phase" == "complete" ]]
  cex_psql_stdin -Atc "update trnm_online_rating_events set created_at=now()-interval '11 minutes'
    where match_id='$match_id'::uuid" >/dev/null
done

fourth_a="$(queue "$A" "$A_ACCOUNT" "$A_SESSION" "$A_CAMPAIGN")"
fourth_b="$(queue "$B" "$B_ACCOUNT" "$B_SESSION" "$B_CAMPAIGN")"
jq -e '.status == "queued" and .match_id == null' <<<"$fourth_a" >/dev/null
jq -e '.status == "queued" and .match_id == null' <<<"$fourth_b" >/dev/null
cancel "$A" "$A_ACCOUNT" "$A_SESSION"
cancel "$B" "$B_ACCOUNT" "$B_SESSION"

match_list="$(printf "'%s'," "${matches[@]}")"
match_list="${match_list%,}"
database="$(cex_psql_stdin -Atc "select json_build_object(
  'completed',(select count(*) from trnm_online_matches where match_id in ($match_list) and phase='complete'),
  'repeat_signals',(select count(*) from trnm_online_integrity_signals where match_id in ($match_list)
    and signal_kind='repeat_opponent' and severity='medium'),
  'rating_events',(select count(*) from trnm_online_rating_events where match_id in ($match_list)),
  'replays',(select count(*) from trnm_online_replay_index where match_id in ($match_list)),
  'value_entitlements',(select count(*) from trnm_value_entitlements where entitlement_json->>'match_id'
    in ($match_list))
)" | jq -c .)"
jq -e '.completed == 3 and .repeat_signals >= 1 and .rating_events == 6 and
  .replays == 3 and .value_entitlements == 0' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --argjson matches "$(printf '%s\n' "${matches[@]}" | jq -R . | jq -s .)" \
  --argjson database "$database" \
  '{status:"passed",run_id:$run_id,shared_device_pairing_rejected:true,
    three_real_ranked_matches:true,repeat_opponent_signal:true,
    fourth_daily_pairing_rejected:true,matches:$matches,database:$database,
    note:"test backdates only synthetic rating-event timestamps by 11 minutes after each real match to cross the 10-minute cooldown while remaining inside the 24-hour cap"}'
