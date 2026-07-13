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
V1_REPORT="$($ROOT_DIR/scripts/check-trnm-online-operations-v1-e2e.sh)"
PRODUCT_RUN="$(jq -er .product_run <<<"$V1_REPORT")"
MATCH_ID="$(jq -er .match_id <<<"$V1_REPORT")"
REPORT_ID="$(jq -er .report_id <<<"$V1_REPORT")"
HOST="$PRODUCT_RUN-host"
GUEST="$PRODUCT_RUN-guest"
HOST_CREDENTIAL="credential-$PRODUCT_RUN-host-012345678901234567890123"
GUEST_CREDENTIAL="credential-$PRODUCT_RUN-guest-012345678901234567890123"
RUN_ID="online-operations-v2-${PRODUCT_RUN#online-product-v2-}"

login() {
  local player="$1" credential="$2" device="$3"
  curl -fsS "$LEDGER_URL/v1/trnm/product/login" -H 'content-type: application/json' \
    --data-binary "$(jq -cn --arg player "$player" --arg credential "$credential" --arg device "$device" \
      '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')"
}

HOST_LOGIN="$(login "$HOST" "$HOST_CREDENTIAL" "$RUN_ID-host-playback")"
GUEST_LOGIN="$(login "$GUEST" "$GUEST_CREDENTIAL" "$RUN_ID-guest-appeal")"
HOST_SESSION="$(jq -er .session_token <<<"$HOST_LOGIN")"
HOST_ACCOUNT="$(jq -er .account_id <<<"$HOST_LOGIN")"
GUEST_SESSION="$(jq -er .session_token <<<"$GUEST_LOGIN")"
GUEST_ACCOUNT="$(jq -er .account_id <<<"$GUEST_LOGIN")"

playback_request="$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" --arg match "$MATCH_ID" \
  '{protocol_version:"trnm_online_operations_v2",build_id:"trnm-online-operations-2026.07-v2",
    player_id:$player,account_id:$account,match_id:$match}')"
playback="$(curl -fsS "$ONLINE_URL/v1/operations/replays/playback" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$playback_request")"
jq -e --arg match "$MATCH_ID" \
  '.integrity_verified == true and .replay.match_id == $match and
   (.commands | length) == .replay.command_count and (.frames | length) >= 2 and
   .frames[0].frame_kind == "initial" and .frames[-1].frame_kind == "terminal" and
   .frames[-1].snapshot_hash == .replay.final_snapshot_hash and .result != null' \
  <<<"$playback" >/dev/null

ENFORCEMENT_ID="$(cex_psql_stdin -Atc "select enforcement_id from trnm_online_enforcements
  where source_report_id='$REPORT_ID'::uuid and player_id='$GUEST' order by created_at desc limit 1")"
[[ "$ENFORCEMENT_ID" =~ ^[0-9a-f-]{36}$ ]]
appeal_body="$(jq -cn --arg player "$GUEST" --arg account "$GUEST_ACCOUNT" \
  --arg enforcement "$ENFORCEMENT_ID" \
  '{protocol_version:"trnm_online_operations_v2",build_id:"trnm-online-operations-2026.07-v2",
    player_id:$player,account_id:$account,enforcement_id:$enforcement,
    detail:"I request review of the replay evidence and ranked enforcement for this acceptance case."}')"
appeal="$(curl -fsS "$ONLINE_URL/v1/operations/enforcements/appeals" \
  -H "x-trnm-player-session: $GUEST_SESSION" -H 'content-type: application/json' \
  --data-binary "$appeal_body")"
APPEAL_ID="$(jq -er '.appeal_id | select(length == 36)' <<<"$appeal")"
jq -e '.status == "pending" and .overdue == false and .due_at_epoch > .created_at_epoch' \
  <<<"$appeal" >/dev/null
duplicate_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/operations/enforcements/appeals" \
  -H "x-trnm-player-session: $GUEST_SESSION" -H 'content-type: application/json' \
  --data-binary "$appeal_body")"
[[ "$duplicate_status" == "409" ]]
wrong_queue_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/operations/moderation/appeals" \
  -H 'x-trnm-moderator: wrong' -H 'content-type: application/json' \
  --data-binary '{"status":"pending","limit":100}')"
[[ "$wrong_queue_status" == "401" ]]
appeal_queue="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_MODERATOR_TOKEN="$MODERATOR_TOKEN" \
  "$ROOT_DIR/target/release/trnm-moderation-console" appeals pending)"
jq -e --arg appeal "$APPEAL_ID" \
  '.pending_count >= 1 and .overdue_count == 0 and any(.appeals[]; .appeal_id == $appeal)' \
  <<<"$appeal_queue" >/dev/null
appeal_resolution="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_MODERATOR_TOKEN="$MODERATOR_TOKEN" \
  "$ROOT_DIR/target/release/trnm-moderation-console" appeal "$APPEAL_ID" approved \
  "Replay evidence was re-reviewed; automated acceptance enforcement is revoked.")"
jq -e '.status == "approved" and .overdue == false' <<<"$appeal_resolution" >/dev/null

NOW="$(date +%s)"
END="$((NOW + 7776000))"
SEASON_ID="season-ops-v2-$NOW-${RANDOM}"
season_created="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_MODERATOR_TOKEN="$MODERATOR_TOKEN" \
  "$ROOT_DIR/target/release/trnm-moderation-console" season create "$SEASON_ID" \
  "Operations-v2-$NOW" "trnm_ranked_rules_2026_07_v2" "$NOW" "$END")"
jq -e --arg season "$SEASON_ID" '.season.season_id == $season and .season.status == "scheduled"' \
  <<<"$season_created" >/dev/null
HOST_CAMPAIGN="$(cex_psql_stdin -Atc "select campaign_id from trnm_online_campaigns
  where player_id='$HOST' and account_id='$HOST_ACCOUNT'::uuid order by updated_at desc limit 1")"
queue_body="$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" --arg campaign "$HOST_CAMPAIGN" \
  '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
    player_id:$player,account_id:$account,campaign_id:$campaign,map_id:"first_contact"}')"
queued="$(curl -fsS "$ONLINE_URL/v1/product/solo-queue/join" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$queue_body")"
jq -e '.status == "queued"' <<<"$queued" >/dev/null
rotation_while_queued_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/operations/seasons/admin" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg season "$SEASON_ID" \
    '{action:"activate",season_id:$season,display_name:null,rules_version:null,starts_at_epoch:null,ends_at_epoch:null}')")"
[[ "$rotation_while_queued_status" == "409" ]]
curl -fsS "$ONLINE_URL/v1/product/solo-queue/cancel" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" \
    '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
      player_id:$player,account_id:$account}')" >/dev/null
season_activated="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" TRNM_MODERATOR_TOKEN="$MODERATOR_TOKEN" \
  "$ROOT_DIR/target/release/trnm-moderation-console" season activate "$SEASON_ID")"
jq -e --arg season "$SEASON_ID" \
  '.season.season_id == $season and .season.status == "active" and
   .previous_active_season_id != null and .archived_entries >= 0' \
  <<<"$season_activated" >/dev/null

leaderboard_request="$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" \
  '{protocol_version:"trnm_online_operations_v2",build_id:"trnm-online-operations-2026.07-v2",
    player_id:$player,account_id:$account}')"
leaderboard="$(curl -fsS "$ONLINE_URL/v1/operations/leaderboard" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$leaderboard_request")"
jq -e --arg season "$SEASON_ID" \
  '.protocol_version == "trnm_online_operations_v2" and .season.season_id == $season and
   .season.status == "active" and .entries == [] and .requester == null' \
  <<<"$leaderboard" >/dev/null

database="$(cex_psql_stdin -Atc "select json_build_object(
  'frames',(select count(*) from trnm_online_replay_frames where match_id='$MATCH_ID'::uuid),
  'terminal_frames',(select count(*) from trnm_online_replay_frames where match_id='$MATCH_ID'::uuid and frame_kind='terminal'),
  'appeals',(select count(*) from trnm_online_enforcement_appeals where appeal_id='$APPEAL_ID'::uuid and status='approved'),
  'enforcement_revoked',(select count(*) from trnm_online_enforcements where enforcement_id='$ENFORCEMENT_ID'::uuid and revoked_at is not null),
  'season_audit',(select count(*) from trnm_online_season_admin_audit where season_id='$SEASON_ID'),
  'season_snapshots',(select count(*) from trnm_online_season_snapshots where season_id=(select previous_active_season_id from trnm_online_season_admin_audit where season_id='$SEASON_ID' and action='activate' order by created_at desc limit 1)),
  'eligible_previous',(select count(*) from trnm_online_season_ratings rating where season_id=(select previous_active_season_id from trnm_online_season_admin_audit where season_id='$SEASON_ID' and action='activate' order by created_at desc limit 1) and not exists(select 1 from trnm_online_rating_events event where event.season_id=rating.season_id and event.player_id=rating.player_id and event.integrity_state<>'clear')),
  'previous_closed',(select count(*) from trnm_online_seasons where season_id=(select previous_active_season_id from trnm_online_season_admin_audit where season_id='$SEASON_ID' and action='activate' order by created_at desc limit 1) and status='closed'),
  'active_season',(select season_id from trnm_online_seasons where status='active')
)" | jq -c .)"
jq -e --arg season "$SEASON_ID" \
  '.frames >= 2 and .terminal_frames == 1 and .appeals == 1 and
   .enforcement_revoked == 1 and .season_audit == 2 and
   .season_snapshots == .eligible_previous and .previous_closed == 1 and
   .active_season == $season' <<<"$database" >/dev/null

REPLAY_HASH="$(jq -er .replay.replay_hash <<<"$playback")"
REPLAY_FRAME_COUNT="$(jq -er '.frames | length' <<<"$playback")"
REPLAY_COMMAND_COUNT="$(jq -er '.commands | length' <<<"$playback")"
jq -n --arg run_id "$RUN_ID" --arg product_run "$PRODUCT_RUN" --arg match_id "$MATCH_ID" \
  --arg report_id "$REPORT_ID" --arg appeal_id "$APPEAL_ID" --arg season_id "$SEASON_ID" \
  --arg replay_hash "$REPLAY_HASH" --argjson replay_frames "$REPLAY_FRAME_COUNT" \
  --argjson replay_commands "$REPLAY_COMMAND_COUNT" --argjson database "$database" \
  '{status:"passed",run_id:$run_id,product_run:$product_run,match_id:$match_id,
    report_id:$report_id,authoritative_replay_playback:true,replay_hash:$replay_hash,
    replay_frame_count:$replay_frames,replay_command_count:$replay_commands,
    appeal_id:$appeal_id,appeal_sla_and_revocation:true,season_id:$season_id,
    active_ranked_rotation_rejected:true,season_rotation_and_archival:true,database:$database}'
