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
MODERATOR_TOKEN="${TRNM_MODERATOR_TOKEN:-trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN}"
RUN_ID="online-product-v2-$(date +%s)-${RANDOM}"
SLOT_KEY="pvp-${RANDOM}"
PRODUCT_PROTOCOL="trnm_online_product_v2"
PRODUCT_BUILD="trnm-online-product-2026.07-v2"
AUTHORITY_PROTOCOL="trnm_online_authority_v2"
AUTHORITY_BUILD="trnm-online-authority-2026.07-v2"

# Failed local acceptance runs must not remain eligible as real queue candidates.
cex_psql_stdin -Atc "update trnm_online_solo_queue set status = 'cancelled', updated_at = now()
  where status = 'queued' and player_id like 'online-product-v2-%'" >/dev/null

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

expect_player_status() {
  local expected="$1" session="$2" path="$3" body="$4" status
  status="$(curl -sS -o /dev/null -w '%{http_code}' "$ONLINE_URL$path" \
    -H "x-trnm-player-session: $session" -H 'content-type: application/json' \
    --data-binary "$body")"
  [[ "$status" == "$expected" ]] || {
    echo "expected HTTP $expected from $path, received $status" >&2
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

access_body() {
  local player="$1" account="$2"
  jq -cn --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
    --arg player "$player" --arg account "$account" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account}'
}

connect_campaign() {
  local session="$1" player="$2" account="$3"
  player_post "$session" /v1/online/campaigns/connect "$(jq -cn \
    --arg protocol "$AUTHORITY_PROTOCOL" --arg build "$AUTHORITY_BUILD" \
    --arg player "$player" --arg account "$account" --arg slot "$SLOT_KEY" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')"
}

queue_join() {
  local session="$1" player="$2" account="$3" campaign="$4"
  player_post "$session" /v1/product/solo-queue/join "$(jq -cn \
    --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" \
    --arg player "$player" --arg account "$account" --arg campaign "$campaign" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,map_id:"first_contact"}')"
}

snapshot() {
  local session="$1" player="$2" account="$3" match_id="$4"
  player_post "$session" "/v1/online/matches/$match_id/snapshot" "$(jq -cn \
    --arg protocol "$AUTHORITY_PROTOCOL" --arg build "$AUTHORITY_BUILD" \
    --arg player "$player" --arg account "$account" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account}')"
}

submit_attack() {
  local session="$1" player="$2" account="$3" match_id="$4" subject="$5" target="$6"
  local current sequence revision target_tick target_x target_y body
  current="$(snapshot "$session" "$player" "$account" "$match_id")"
  sequence="$(jq -er .view.next_sequence <<<"$current")"
  revision="$(jq -er .view.match_revision <<<"$current")"
  target_tick="$(( $(jq -er .view.authoritative_tick <<<"$current") + 40 ))"
  target_x="$(jq -er --arg target "$target" '[.snapshot.party[],.snapshot.enemies[]] | .[] | select(.unit_id == $target) | .position.x' <<<"$current")"
  target_y="$(jq -er --arg target "$target" '[.snapshot.party[],.snapshot.enemies[]] | .[] | select(.unit_id == $target) | .position.y' <<<"$current")"
  body="$(jq -cn --arg protocol "$AUTHORITY_PROTOCOL" --arg build "$AUTHORITY_BUILD" \
    --arg player "$player" --arg account "$account" --arg match "$match_id" \
    --arg subject "$subject" --argjson target_x "$target_x" --argjson target_y "$target_y" \
    --argjson sequence "$sequence" --argjson revision "$revision" --argjson tick "$target_tick" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
      command_id:("pvp:"+$match+":"+$player+":"+($sequence|tostring)),sequence:$sequence,
      expected_match_revision:$revision,target_tick:$tick,
      order:{contract:"trnm_rts_order_protocol_v1",frame:$tick,player_id:$player,
        subject_actor_ids:[$subject],kind:"attack_move",queued:false,
        target_tile:{x:$target_x,y:$target_y},target_actor_id:null,target_rule_id:null,
        queue_id:null,formation_id:null,source:"local_input",raw_command_label:"ranked_pvp_attack_move"}}')"
  player_post "$session" "/v1/online/matches/$match_id/commands" "$body"
}

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_CREDENTIAL < <(register_player host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_CREDENTIAL < <(register_player guest)
IFS=$'\t' read -r BLOCKED_PLAYER BLOCKED_ACCOUNT BLOCKED_CREDENTIAL < <(register_player blocked)
HOST_SESSION="$(login_player "$HOST_PLAYER" "$HOST_CREDENTIAL" "$RUN_ID-host-device")"
GUEST_SESSION="$(login_player "$GUEST_PLAYER" "$GUEST_CREDENTIAL" "$RUN_ID-guest-device")"
BLOCKED_SESSION="$(login_player "$BLOCKED_PLAYER" "$BLOCKED_CREDENTIAL" "$RUN_ID-blocked-device")"

systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=40
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" | jq -e \
  '.status == "ok" and .online_product_protocol == "trnm_online_product_v2" and
   .ranked_solo_queue == true and .authoritative_pvp == true and .persistent_mmr == true' >/dev/null

HOST_CAMPAIGN="$(connect_campaign "$HOST_SESSION" "$HOST_PLAYER" "$HOST_ACCOUNT")"
GUEST_CAMPAIGN="$(connect_campaign "$GUEST_SESSION" "$GUEST_PLAYER" "$GUEST_ACCOUNT")"
BLOCKED_CAMPAIGN="$(connect_campaign "$BLOCKED_SESSION" "$BLOCKED_PLAYER" "$BLOCKED_ACCOUNT")"

friend_request="$(player_post "$HOST_SESSION" /v1/product/social/friends/request "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg target "$GUEST_PLAYER" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,target_player_id:$target}')")"
jq -e --arg guest "$GUEST_PLAYER" '.outgoing_requests == [$guest]' <<<"$friend_request" >/dev/null
friend_accept="$(player_post "$GUEST_SESSION" /v1/product/social/friends/resolve "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" --arg host "$HOST_PLAYER" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,requester_player_id:$host,accept:true}')")"
jq -e --arg host "$HOST_PLAYER" '.friends == [$host]' <<<"$friend_accept" >/dev/null

for tuple in "$HOST_SESSION|$HOST_PLAYER|$HOST_ACCOUNT" "$GUEST_SESSION|$GUEST_PLAYER|$GUEST_ACCOUNT"; do
  IFS='|' read -r session player account <<<"$tuple"
  player_post "$session" /v1/product/social/block "$(jq -cn \
    --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" --arg player "$player" \
    --arg account "$account" --arg target "$BLOCKED_PLAYER" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,target_player_id:$target,blocked:true}')" >/dev/null
done

blocked_ticket="$(queue_join "$BLOCKED_SESSION" "$BLOCKED_PLAYER" "$BLOCKED_ACCOUNT" "$(jq -er .campaign_id <<<"$BLOCKED_CAMPAIGN")")"
jq -e '.status == "queued"' <<<"$blocked_ticket" >/dev/null
host_ticket="$(queue_join "$HOST_SESSION" "$HOST_PLAYER" "$HOST_ACCOUNT" "$(jq -er .campaign_id <<<"$HOST_CAMPAIGN")")"
jq -e '.status == "queued"' <<<"$host_ticket" >/dev/null
expect_player_status 409 "$HOST_SESSION" /v1/product/solo-queue/join "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg campaign "$(jq -er .campaign_id <<<"$HOST_CAMPAIGN")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,map_id:"first_contact"}')"
guest_ticket="$(queue_join "$GUEST_SESSION" "$GUEST_PLAYER" "$GUEST_ACCOUNT" "$(jq -er .campaign_id <<<"$GUEST_CAMPAIGN")")"
jq -e --arg opponent "$HOST_PLAYER" '.status == "matched" and .opponent_player_id == $opponent' <<<"$guest_ticket" >/dev/null
MATCH_ID="$(jq -er .match_id <<<"$guest_ticket")"
LOBBY_ID="$(jq -er .matched_lobby_id <<<"$guest_ticket")"
host_ticket="$(player_post "$HOST_SESSION" /v1/product/solo-queue/status "$(access_body "$HOST_PLAYER" "$HOST_ACCOUNT")")"
jq -e --arg match "$MATCH_ID" --arg opponent "$GUEST_PLAYER" \
  '.status == "matched" and .match_id == $match and .opponent_player_id == $opponent' <<<"$host_ticket" >/dev/null

initial="$(snapshot "$HOST_SESSION" "$HOST_PLAYER" "$HOST_ACCOUNT" "$MATCH_ID")"
jq -e '.view.match_mode == "ranked_pvp" and .snapshot.human_enemy_authority == true and
  (.view.members | length) == 2' <<<"$initial" >/dev/null
HOST_UNIT="$(jq -er --arg player "$HOST_PLAYER" '.view.members[] | select(.player_id == $player) | .controlled_unit_ids[0]' <<<"$initial")"
GUEST_UNIT="$(jq -er --arg player "$GUEST_PLAYER" '.view.members[] | select(.player_id == $player) | .controlled_unit_ids[0]' <<<"$initial")"
expect_player_status 403 "$HOST_SESSION" "/v1/online/matches/$MATCH_ID/commands" "$(jq -cn \
  --arg protocol "$AUTHORITY_PROTOCOL" --arg build "$AUTHORITY_BUILD" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg subject "$GUEST_UNIT" --arg target "$HOST_UNIT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
    command_id:"pvp-cross-control",sequence:0,expected_match_revision:1,target_tick:40,
    order:{contract:"trnm_rts_order_protocol_v1",frame:40,player_id:$player,
      subject_actor_ids:[$subject],kind:"attack",queued:false,target_tile:null,target_actor_id:$target,
      target_rule_id:null,queue_id:null,formation_id:null,source:"local_input",raw_command_label:"attack"}}')"

submit_attack "$HOST_SESSION" "$HOST_PLAYER" "$HOST_ACCOUNT" "$MATCH_ID" "$HOST_UNIT" "$GUEST_UNIT" >/dev/null
submit_attack "$GUEST_SESSION" "$GUEST_PLAYER" "$GUEST_ACCOUNT" "$MATCH_ID" "$GUEST_UNIT" "$HOST_UNIT" >/dev/null
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done

final=""
for _ in $(seq 1 180); do
  final="$(snapshot "$HOST_SESSION" "$HOST_PLAYER" "$HOST_ACCOUNT" "$MATCH_ID")"
  [[ "$(jq -r .view.phase <<<"$final")" == "complete" ]] && break
  sleep 1
done
jq -e '.view.phase == "complete" and .view.settlement_state == "settled"' <<<"$final" >/dev/null

host_rating="$(player_post "$HOST_SESSION" /v1/product/rating "$(access_body "$HOST_PLAYER" "$HOST_ACCOUNT")")"
guest_rating="$(player_post "$GUEST_SESSION" /v1/product/rating "$(access_body "$GUEST_PLAYER" "$GUEST_ACCOUNT")")"
jq -e '.provisional_matches == 1 and ((.wins == 1 and .losses == 0) or (.wins == 0 and .losses == 1))' <<<"$host_rating" >/dev/null
jq -e '.provisional_matches == 1 and ((.wins == 1 and .losses == 0) or (.wins == 0 and .losses == 1))' <<<"$guest_rating" >/dev/null
[[ "$(( $(jq -er .rating <<<"$host_rating") + $(jq -er .rating <<<"$guest_rating") ))" == "2000" ]]

report="$(player_post "$HOST_SESSION" /v1/product/reports "$(jq -cn \
  --arg protocol "$PRODUCT_PROTOCOL" --arg build "$PRODUCT_BUILD" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg target "$GUEST_PLAYER" --arg match "$MATCH_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
    target_player_id:$target,match_id:$match,category:"other",
    detail:"Automated Product v2 moderation intake drill with authenticated match provenance."}')")"
REPORT_ID="$(jq -er .report_id <<<"$report")"
status="$(curl -sS -o /dev/null -w '%{http_code}' "$ONLINE_URL/v1/product/moderation/reports/resolve" \
  -H 'x-trnm-moderator: wrong' -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg report "$REPORT_ID" \
    '{report_id:$report,decision:"reviewed",resolution:"Unauthorized resolution attempt must fail closed."}')")"
[[ "$status" == "401" ]]
resolved="$(curl -fsS "$ONLINE_URL/v1/product/moderation/reports/resolve" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg report "$REPORT_ID" \
    '{report_id:$report,decision:"reviewed",resolution:"Authenticated moderator reviewed the automated evidence and preserved the audit record."}')")"
jq -e '.status == "reviewed"' <<<"$resolved" >/dev/null

player_post "$BLOCKED_SESSION" /v1/product/solo-queue/cancel "$(access_body "$BLOCKED_PLAYER" "$BLOCKED_ACCOUNT")" >/dev/null
database="$(cex_psql_stdin -Atc "select json_build_object(
  'match_mode',(select match_mode from trnm_online_matches where match_id = '$MATCH_ID'::uuid),
  'human_enemy',(select simulation_json->>'human_enemy_authority' from trnm_online_matches where match_id = '$MATCH_ID'::uuid),
  'rating_events',(select count(*) from trnm_online_rating_events where match_id = '$MATCH_ID'::uuid),
  'rating_delta_sum',(select coalesce(sum(rating_delta),0) from trnm_online_rating_events where match_id = '$MATCH_ID'::uuid),
  'progression_events',(select count(*) from trnm_online_progression_events where match_id = '$MATCH_ID'::uuid),
  'value_entitlements',(select count(*) from trnm_value_entitlements where entitlement_json->>'match_id' = '$MATCH_ID'),
  'friends',(select count(*) from trnm_online_friendships where status = 'accepted' and requester_player_id = '$HOST_PLAYER' and target_player_id = '$GUEST_PLAYER'),
  'blocks',(select count(*) from trnm_online_blocks where blocked_player_id = '$BLOCKED_PLAYER' and blocker_player_id in ('$HOST_PLAYER','$GUEST_PLAYER')),
  'report_status',(select status from trnm_online_reports where report_id = '$REPORT_ID'::uuid),
  'allocations',(select count(*) from trnm_online_matchmaking_allocations where lobby_id = '$LOBBY_ID'::uuid and queue_mode = 'ranked_pvp')
)" | jq -c .)"
jq -e '.match_mode == "ranked_pvp" and .human_enemy == "true" and .rating_events == 2 and
  .rating_delta_sum == 0 and .progression_events == 2 and .value_entitlements == 0 and
  .friends == 1 and .blocks == 2 and .report_status == "reviewed" and .allocations == 1' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg lobby_id "$LOBBY_ID" --arg match_id "$MATCH_ID" \
  --argjson host_rating "$host_rating" --argjson guest_rating "$guest_rating" \
  --argjson database "$database" \
  '{status:"passed",run_id:$run_id,lobby_id:$lobby_id,match_id:$match_id,
    native_product_protocol:"trnm_online_product_v2",ranked_solo_queue:true,
    block_aware_pairing:true,authoritative_human_vs_human:true,restart_recovery:true,
    persistent_mmr:true,ranked_cex_value_rewards:false,friends:true,blocks:true,
    authenticated_report_moderation:true,host_rating:$host_rating,guest_rating:$guest_rating,
    database:$database}'
