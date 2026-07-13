#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
MODERATOR_TOKEN="${TRNM_MODERATOR_TOKEN:-trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN}"
PRODUCT_REPORT="$($ROOT_DIR/scripts/check-trnm-online-product-v2-e2e.sh)"
RUN_ID="$(jq -er .run_id <<<"$PRODUCT_REPORT")"
MATCH_ID="$(jq -er .match_id <<<"$PRODUCT_REPORT")"
HOST="$RUN_ID-host"
GUEST="$RUN_ID-guest"
HOST_CREDENTIAL="credential-$RUN_ID-host-012345678901234567890123"
GUEST_CREDENTIAL="credential-$RUN_ID-guest-012345678901234567890123"

login() {
  local player="$1" credential="$2" device="$3"
  curl -fsS "$LEDGER_URL/v1/trnm/product/login" -H 'content-type: application/json' \
    --data-binary "$(jq -cn --arg player "$player" --arg credential "$credential" --arg device "$device" \
      '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')"
}

HOST_LOGIN="$(login "$HOST" "$HOST_CREDENTIAL" "$RUN_ID-operations-host")"
GUEST_LOGIN="$(login "$GUEST" "$GUEST_CREDENTIAL" "$RUN_ID-operations-guest")"
HOST_SESSION="$(jq -er .session_token <<<"$HOST_LOGIN")"
HOST_ACCOUNT="$(jq -er .account_id <<<"$HOST_LOGIN")"
GUEST_ACCOUNT="$(jq -er .account_id <<<"$GUEST_LOGIN")"

ops_access="$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" \
  '{protocol_version:"trnm_online_operations_v1",build_id:"trnm-online-operations-2026.07-v1",player_id:$player,account_id:$account}')"
leaderboard="$(curl -fsS "$ONLINE_URL/v1/operations/leaderboard" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$ops_access")"
jq -e --arg player "$HOST" '.season.status == "active" and .requester.player_id == $player and
  .requester.rating == 1016 and .requester.wins == 1 and .requester.matches == 1' \
  <<<"$leaderboard" >/dev/null

replay_request="$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" --arg match "$MATCH_ID" \
  '{protocol_version:"trnm_online_operations_v1",build_id:"trnm-online-operations-2026.07-v1",player_id:$player,account_id:$account,match_id:$match}')"
replay="$(curl -fsS "$ONLINE_URL/v1/operations/replays" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$replay_request")"
REPLAY_HASH="$(jq -er '.replay_hash | select(length == 64)' <<<"$replay")"
jq -e --arg match "$MATCH_ID" '.match_id == $match and .command_count >= 2 and
  .season_id == "season-2026-prealpha-1"' <<<"$replay" >/dev/null

report_body() {
  local replay_hash="$1"
  jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" --arg target "$GUEST" \
    --arg match "$MATCH_ID" --arg replay "$replay_hash" \
    '{protocol_version:"trnm_online_operations_v1",build_id:"trnm-online-operations-2026.07-v1",
      player_id:$player,account_id:$account,target_player_id:$target,match_id:$match,
      replay_hash:$replay,category:"harassment",detail:"Replay-bound Operations v1 moderation acceptance report."}'
}

tampered_hash="${REPLAY_HASH%?}0"
[[ "$tampered_hash" != "$REPLAY_HASH" ]] || tampered_hash="${REPLAY_HASH%?}1"
tampered_status="$(curl -sS -o /dev/null -w '%{http_code}' "$ONLINE_URL/v1/operations/reports/replay" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(report_body "$tampered_hash")")"
[[ "$tampered_status" == "409" ]]
report="$(curl -fsS "$ONLINE_URL/v1/operations/reports/replay" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(report_body "$REPLAY_HASH")")"
REPORT_ID="$(jq -er .report_id <<<"$report")"

held_leaderboard="$(curl -fsS "$ONLINE_URL/v1/operations/leaderboard" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$ops_access")"
jq -e '.requester == null' <<<"$held_leaderboard" >/dev/null

wrong_status="$(curl -sS -o /dev/null -w '%{http_code}' "$ONLINE_URL/v1/operations/moderation/queue" \
  -H 'x-trnm-moderator: wrong-moderator' -H 'content-type: application/json' \
  --data-binary '{"status":"open","limit":100}')"
[[ "$wrong_status" == "401" ]]

console_list="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_MODERATOR_TOKEN="$MODERATOR_TOKEN" \
  "$ROOT_DIR/target/release/trnm-moderation-console" list open)"
jq -e --arg report "$REPORT_ID" --arg replay "$REPLAY_HASH" \
  '.cases[] | select(.report.report_id == $report) |
   .replay.replay_hash == $replay and (.integrity_signals | length) >= 1' \
  <<<"$console_list" >/dev/null
action="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_MODERATOR_TOKEN="$MODERATOR_TOKEN" \
  "$ROOT_DIR/target/release/trnm-moderation-console" action "$REPORT_ID" actioned ranked 24 \
  "Replay evidence confirmed; ranked access suspended for Operations v1 acceptance.")"
jq -e --arg player "$GUEST" '.report.status == "actioned" and .target_player_id == $player and
  (.audit_id | length) == 36 and (.enforcement_id | length) == 36' <<<"$action" >/dev/null

GUEST_CAMPAIGN="$(cex_psql_stdin -Atc "select campaign_id from trnm_online_campaigns
  where player_id = '$GUEST' and account_id = '$GUEST_ACCOUNT'::uuid order by updated_at desc limit 1")"
GUEST_SESSION="$(jq -er .session_token <<<"$GUEST_LOGIN")"
suspended_status="$(curl -sS -o /dev/null -w '%{http_code}' "$ONLINE_URL/v1/product/solo-queue/join" \
  -H "x-trnm-player-session: $GUEST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$GUEST" --arg account "$GUEST_ACCOUNT" --arg campaign "$GUEST_CAMPAIGN" \
    '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
      player_id:$player,account_id:$account,campaign_id:$campaign,map_id:"first_contact"}')")"
[[ "$suspended_status" == "403" ]]

route="$(curl -fsS "$ONLINE_URL/v1/operations/fleet/route" -H 'content-type: application/json' \
  --data-binary '{"protocol_version":"trnm_online_operations_v1","build_id":"trnm-online-operations-2026.07-v1","preferred_region":"local-x230"}')"
jq -e '.selected.instance_id == "trnm-local-primary" and .selected.region == "local-x230" and
  .cross_region_fallback == false and .healthy_instances >= 1' <<<"$route" >/dev/null

database="$(cex_psql_stdin -Atc "select json_build_object(
  'season_events',(select count(*) from trnm_online_rating_events where match_id='$MATCH_ID'::uuid and season_id is not null),
  'season_ratings',(select count(*) from trnm_online_season_ratings where player_id in ('$HOST','$GUEST')),
  'replays',(select count(*) from trnm_online_replay_index where match_id='$MATCH_ID'::uuid),
  'integrity_state',(select min(integrity_state) from trnm_online_rating_events where match_id='$MATCH_ID'::uuid),
  'signals',(select count(*) from trnm_online_integrity_signals where match_id='$MATCH_ID'::uuid),
  'audit',(select count(*) from trnm_online_moderation_audit where report_id='$REPORT_ID'::uuid),
  'enforcement',(select count(*) from trnm_online_enforcements where source_report_id='$REPORT_ID'::uuid and expires_at > now()),
  'fleet',(select count(*) from trnm_online_fleet_instances where heartbeat_at > now()-interval '5 seconds')
)" | jq -c .)"
jq -e '.season_events == 2 and .season_ratings == 2 and .replays == 1 and
  .integrity_state == "voided" and .signals >= 1 and .audit == 1 and .enforcement == 1 and .fleet >= 1' \
  <<<"$database" >/dev/null

jq -n --arg run_id "online-operations-v1-${RUN_ID#online-product-v2-}" \
  --arg product_run "$RUN_ID" --arg match_id "$MATCH_ID" --arg report_id "$REPORT_ID" \
  --arg replay_hash "$REPLAY_HASH" --argjson route "$route" --argjson database "$database" \
  '{status:"passed",run_id:$run_id,product_run:$product_run,match_id:$match_id,
    active_season:true,leaderboard:true,replay_hash:$replay_hash,replay_bound_report:true,
    tampered_replay_rejected:true,moderation_console:true,report_id:$report_id,
    ranked_enforcement:true,route:$route,database:$database}'
