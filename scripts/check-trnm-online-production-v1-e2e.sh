#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
SIGNER_URL="${TRNM_ENTITLEMENT_SIGNER_URL:-http://127.0.0.1:7010}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
MODERATOR_TOKEN="${TRNM_MODERATOR_TOKEN:-trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN}"
SIGNER_TOKEN="${TRNM_ENTITLEMENT_SIGNER_TOKEN:-trnm-isolated-signer-v1:$IDENTITY_ADMIN_TOKEN}"
PROTOCOL="trnm_online_production_v1"
BUILD="trnm-online-production-2026.07-v1"
RUN_ID="online-production-v1-$(date +%s)-${RANDOM}"
RATE_PID=""
EVIDENCE="$ROOT_DIR/acceptance/online-production-v1/$RUN_ID"
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  [[ -z "$RATE_PID" ]] || kill "$RATE_PID" >/dev/null 2>&1 || true
  exit "$status"
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

login() {
  local player="$1" credential="$2" device="$3"
  json_post "$LEDGER_URL/v1/trnm/product/login" "$(jq -cn \
    --arg player "$player" --arg credential "$credential" --arg device "$device" \
    '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')"
}

register_player() {
  local player="$1" credential="$2" invite identity
  invite="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  identity="$(json_post "$LEDGER_URL/v1/trnm/product/register" "$(jq -cn \
    --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
    '{player_id:$player,credential:$credential,invite_code:$invite}')")"
  jq -c . <<<"$identity"
}

BEFORE_PRODUCT="$($ROOT_DIR/scripts/check-trnm-online-product-v1-e2e.sh)"
BEFORE_RUN="$(jq -er .run_id <<<"$BEFORE_PRODUCT")"
MATCH_ID="$(jq -er .match_id <<<"$BEFORE_PRODUCT")"
HOST="$BEFORE_RUN-host"
GUEST="$BEFORE_RUN-guest"
HOST_CREDENTIAL="credential-$BEFORE_RUN-host-012345678901234567890123"
GUEST_CREDENTIAL="credential-$BEFORE_RUN-guest-rotated-012345678901234567890123"
HOST_LOGIN="$(login "$HOST" "$HOST_CREDENTIAL" "$RUN_ID-host-device")"
GUEST_LOGIN="$(login "$GUEST" "$GUEST_CREDENTIAL" "$RUN_ID-guest-device")"
HOST_SESSION="$(jq -er .session_token <<<"$HOST_LOGIN")"
HOST_ACCOUNT="$(jq -er .account_id <<<"$HOST_LOGIN")"
GUEST_SESSION="$(jq -er .session_token <<<"$GUEST_LOGIN")"

signer_receipt="$(cex_psql_stdin -Atc "select json_build_object(
  'request_id',request_id,'entitlement',entitlement_json
) from trnm_entitlement_signing_receipts
where entitlement_json->>'match_id'='$MATCH_ID' order by created_at limit 1" | jq -c .)"
signer_request="$(jq -c --arg contract 'trnm_entitlement_signer_v1' \
  '{contract_version:$contract,request_id:.request_id,
    entitlement:(.entitlement | .signature="" | .key_id="")}' <<<"$signer_receipt")"
signer_duplicate="$(curl -fsS "$SIGNER_URL/v1/signer/sign" \
  -H "x-trnm-signer-auth: $SIGNER_TOKEN" -H 'content-type: application/json' \
  --data-binary "$signer_request")"
jq -e '.duplicate == true and (.request_hash | length) == 64 and
  (.signing_receipt_hash | length) == 64' <<<"$signer_duplicate" >/dev/null
tampered_signer_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$SIGNER_URL/v1/signer/sign" -H "x-trnm-signer-auth: $SIGNER_TOKEN" \
  -H 'content-type: application/json' \
  --data-binary "$(jq -c '.entitlement.amount_credits += 1' <<<"$signer_request")")"
[[ "$tampered_signer_status" == "409" ]]

SPECTATOR="$RUN_ID-spectator"
SPECTATOR_CREDENTIAL="credential-$RUN_ID-spectator-012345678901234567890123"
SPECTATOR_IDENTITY="$(register_player "$SPECTATOR" "$SPECTATOR_CREDENTIAL")"
SPECTATOR_ACCOUNT="$(jq -er .account_id <<<"$SPECTATOR_IDENTITY")"
SPECTATOR_LOGIN="$(login "$SPECTATOR" "$SPECTATOR_CREDENTIAL" "$RUN_ID-spectator-device")"
SPECTATOR_SESSION="$(jq -er .session_token <<<"$SPECTATOR_LOGIN")"
SPECTATOR_SLOT="prod-$(date +%s)-${RANDOM}"
player_post "$SPECTATOR_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg player "$SPECTATOR" --arg account "$SPECTATOR_ACCOUNT" --arg slot "$SPECTATOR_SLOT" \
  '{protocol_version:"trnm_online_authority_v2",build_id:"trnm-online-authority-2026.07-v2",
    player_id:$player,account_id:$account,slot_key:$slot}')" >/dev/null

invite="$(player_post "$HOST_SESSION" /v1/production/spectators/invites "$(jq -cn \
  --arg protocol "$PROTOCOL" --arg build "$BUILD" --arg player "$HOST" \
  --arg account "$HOST_ACCOUNT" --arg match "$MATCH_ID" --arg target "$SPECTATOR" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
    match_id:$match,target_player_id:$target,delay_seconds:30}')")"
INVITE_TOKEN="$(jq -er .invite_token <<<"$invite")"
wrong_spectator_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/production/spectators/invites/accept" \
  -H "x-trnm-player-session: $GUEST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg protocol "$PROTOCOL" --arg build "$BUILD" \
    --arg player "$GUEST" --arg account "$(jq -er .account_id <<<"$GUEST_LOGIN")" \
    --arg token "$INVITE_TOKEN" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,invite_token:$token}')")"
[[ "$wrong_spectator_status" == "403" ]]
grant="$(player_post "$SPECTATOR_SESSION" /v1/production/spectators/invites/accept "$(jq -cn \
  --arg protocol "$PROTOCOL" --arg build "$BUILD" --arg player "$SPECTATOR" \
  --arg account "$SPECTATOR_ACCOUNT" --arg token "$INVITE_TOKEN" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,invite_token:$token}')")"
GRANT_ID="$(jq -er .grant_id <<<"$grant")"
duplicate_invite_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/production/spectators/invites/accept" \
  -H "x-trnm-player-session: $SPECTATOR_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg protocol "$PROTOCOL" --arg build "$BUILD" \
    --arg player "$SPECTATOR" --arg account "$SPECTATOR_ACCOUNT" --arg token "$INVITE_TOKEN" \
    '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,invite_token:$token}')")"
[[ "$duplicate_invite_status" == "409" ]]
playback_body="$(jq -cn --arg protocol "$PROTOCOL" --arg build "$BUILD" \
  --arg player "$SPECTATOR" --arg account "$SPECTATOR_ACCOUNT" --arg grant "$GRANT_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,grant_id:$grant}')"
before_delay="$(player_post "$SPECTATOR_SESSION" /v1/production/spectators/playback "$playback_body")"
jq -e '.terminal_visible == false and .grant.delay_seconds == 30' <<<"$before_delay" >/dev/null
cex_psql_stdin -c "update trnm_online_replay_frames set created_at=now()-interval '31 seconds'
  where match_id='$MATCH_ID'::uuid" >/dev/null
after_delay="$(player_post "$SPECTATOR_SESSION" /v1/production/spectators/playback "$playback_body")"
jq -e '.terminal_visible == true and (.frames | length) >= 2 and
  .frames[-1].frame_kind == "terminal" and .visible_through_tick == .authoritative_tick' \
  <<<"$after_delay" >/dev/null

HOST_CAMPAIGN="$(cex_psql_stdin -Atc "select campaign_id from trnm_online_campaigns
  where player_id='$HOST' and account_id='$HOST_ACCOUNT'::uuid order by updated_at desc limit 1")"
queue_body="$(jq -cn --arg player "$HOST" --arg account "$HOST_ACCOUNT" \
  --arg campaign "$HOST_CAMPAIGN" \
  '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
    player_id:$player,account_id:$account,campaign_id:$campaign,map_id:"first_contact"}')"
queued="$(player_post "$HOST_SESSION" /v1/product/solo-queue/join "$queue_body")"
jq -e '.status == "queued"' <<<"$queued" >/dev/null
NOW="$(date +%s)"
SEASON_ID="season-production-$NOW-${RANDOM}"
curl -fsS "$ONLINE_URL/v1/operations/seasons/admin" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg season "$SEASON_ID" --argjson start "$((NOW - 1))" \
    --argjson end "$((NOW + 7776000))" \
    '{action:"create",season_id:$season,display_name:("Production "+$season),
      rules_version:"trnm_ranked_rules_2026_07_production_v1",
      starts_at_epoch:$start,ends_at_epoch:$end}')" >/dev/null
automation="$(curl -fsS "$ONLINE_URL/v1/production/seasons/automation" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg season "$SEASON_ID" \
    '{season_id:$season,automatic_activation:true}')")"
jq -e '.automatic_activation == true and .automation_state == "scheduled"' \
  <<<"$automation" >/dev/null
sleep 6
deferred_state="$(cex_psql_stdin -Atc "select json_build_object(
  'state',automation_state,'reason',automation_deferred_reason,
  'audits',(select count(*) from trnm_online_season_automation_audit
    where season_id='$SEASON_ID' and action='deferred'))
  from trnm_online_seasons where season_id='$SEASON_ID'" | jq -c .)"
jq -e '.state == "deferred" and .audits == 1 and (.reason | length) > 0' \
  <<<"$deferred_state" >/dev/null
player_post "$HOST_SESSION" /v1/product/solo-queue/cancel "$(jq -cn \
  --arg player "$HOST" --arg account "$HOST_ACCOUNT" \
  '{protocol_version:"trnm_online_product_v2",build_id:"trnm-online-product-2026.07-v2",
    player_id:$player,account_id:$account}')" >/dev/null
for _ in $(seq 1 20); do
  ACTIVE_SEASON="$(cex_psql_stdin -Atc "select season_id from trnm_online_seasons where status='active'")"
  [[ "$ACTIVE_SEASON" == "$SEASON_ID" ]] && break
  sleep 1
done
[[ "$ACTIVE_SEASON" == "$SEASON_ID" ]]

ENFORCEMENT_ID="$(tr -d '\n' </proc/sys/kernel/random/uuid)"
cex_psql_stdin -c "insert into trnm_online_enforcements (
  enforcement_id,player_id,scope,reason,expires_at
) values ('$ENFORCEMENT_ID'::uuid,'$SPECTATOR','ranked',
  'Production v1 controlled SLA escalation acceptance fixture.',now()+interval '2 hours')" >/dev/null
appeal="$(player_post "$SPECTATOR_SESSION" /v1/operations/enforcements/appeals "$(jq -cn \
  --arg protocol "$PROTOCOL" --arg build "$BUILD" --arg player "$SPECTATOR" \
  --arg account "$SPECTATOR_ACCOUNT" --arg enforcement "$ENFORCEMENT_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
    enforcement_id:$enforcement,detail:"Production v1 controlled appeal escalation and resolution acceptance fixture."}')")"
APPEAL_ID="$(jq -er .appeal_id <<<"$appeal")"
cex_psql_stdin -c "update trnm_online_enforcement_appeals set due_at=now()-interval '1 second'
  where appeal_id='$APPEAL_ID'::uuid" >/dev/null
for _ in $(seq 1 12); do
  ESCALATIONS="$(cex_psql_stdin -Atc "select count(*) from trnm_online_appeal_escalations
    where appeal_id='$APPEAL_ID'::uuid and status='open'")"
  [[ "$ESCALATIONS" == "1" ]] && break
  sleep 1
done
[[ "$ESCALATIONS" == "1" ]]
curl -fsS "$ONLINE_URL/v1/operations/moderation/appeals/resolve" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg appeal "$APPEAL_ID" \
    '{appeal_id:$appeal,decision:"approved",
      resolution:"Production v1 SLA drill acknowledged and the controlled enforcement is revoked."}')" >/dev/null
[[ "$(cex_psql_stdin -Atc "select count(*) from trnm_online_appeal_escalations
  where appeal_id='$APPEAL_ID'::uuid and status='closed'")" == "1" ]]

TRNM_GAME_SERVER_BIND_ADDR=127.0.0.1:7007 \
TRNM_FLEET_INSTANCE_ID="$RUN_ID-rate-probe" \
TRNM_FLEET_REGION=local-x230 \
TRNM_FLEET_PUBLIC_ENDPOINT=http://127.0.0.1:7007 \
TRNM_FLEET_PHYSICAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)" \
TRNM_FLEET_CAPACITY=1 TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE=30 \
  "$ROOT_DIR/scripts/run-trnm-game-server.sh" >"$EVIDENCE/rate-probe.log" 2>&1 &
RATE_PID=$!
for _ in $(seq 1 60); do
  curl -fsS http://127.0.0.1:7007/health >/dev/null 2>&1 && break
  sleep 0.25
done
for _ in $(seq 1 29); do
  [[ "$(curl -sS -o /dev/null -w '%{http_code}' http://127.0.0.1:7007/health)" == "200" ]]
done
rate_limited_status="$(curl -sS -o /dev/null -w '%{http_code}' http://127.0.0.1:7007/health)"
[[ "$rate_limited_status" == "429" ]]
body_file="$EVIDENCE/oversize.json"
head -c 300000 /dev/zero | tr '\0' x >"$body_file"
body_limited_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  http://127.0.0.1:7007/v1/online/campaigns/connect \
  -H 'content-type: application/json' --data-binary "@$body_file")"
[[ "$body_limited_status" == "413" ]]
kill "$RATE_PID"
wait "$RATE_PID" 2>/dev/null || true
RATE_PID=""

GAME_PID_BEFORE="$(systemctl --user show trnm-game-server.service -p MainPID --value)"
OLD_KEY_ID="$(curl -fsS "$SIGNER_URL/v1/signer/readiness" | jq -er .key_id)"
NEW_KEY_ID="trnm-online-ed25519-production-$NOW-${RANDOM}"
rotation="$($ROOT_DIR/scripts/rotate-trnm-entitlement-signer-key.sh "$NEW_KEY_ID" --revoke-old)"
GAME_PID_AFTER="$(systemctl --user show trnm-game-server.service -p MainPID --value)"
[[ "$GAME_PID_BEFORE" == "$GAME_PID_AFTER" ]]
jq -e --arg old "$OLD_KEY_ID" --arg new "$NEW_KEY_ID" \
  '.old_key_id == $old and .new_key_id == $new and .old_key_revoked == true and
   .game_server_restart_required == false' <<<"$rotation" >/dev/null

AFTER_PRODUCT="$($ROOT_DIR/scripts/check-trnm-online-product-v1-e2e.sh)"
AFTER_MATCH="$(jq -er .match_id <<<"$AFTER_PRODUCT")"
post_rotation_receipts="$(cex_psql_stdin -Atc "select json_build_object(
  'receipts',(select count(*) from trnm_entitlement_signing_receipts
    where entitlement_json->>'match_id'='$AFTER_MATCH'),
  'new_key_receipts',(select count(*) from trnm_entitlement_signing_receipts
    where entitlement_json->>'match_id'='$AFTER_MATCH' and key_id='$NEW_KEY_ID'),
  'cex_entitlements',(select count(*) from trnm_value_entitlements
    where entitlement_json->>'match_id'='$AFTER_MATCH')
)" | jq -c .)"
jq -e '.receipts == 2 and .new_key_receipts == 2 and .cex_entitlements == 2' \
  <<<"$post_rotation_receipts" >/dev/null

GAME_PID="$(systemctl --user show trnm-game-server.service -p MainPID --value)"
game_private_env_count="$(tr '\0' '\n' </proc/"$GAME_PID"/environ | \
  grep -Ec 'TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY|PRIVATE_KEY_BASE64' || true)"
[[ "$game_private_env_count" == "0" ]]
production_status="$(curl -fsS "$ONLINE_URL/v1/production/status" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN")"
jq -e --arg key "$NEW_KEY_ID" --arg season "$SEASON_ID" \
  '.protocol_version == "trnm_online_production_v1" and .signer_ready == true and
   .signer_key_id == $key and .automatic_season_id == null and
   .distinct_healthy_physical_hosts == 1 and .public_edge_attested == false' \
  <<<"$production_status" >/dev/null

database="$(cex_psql_stdin -Atc "select json_build_object(
  'spectator_invites',(select count(*) from trnm_online_spectator_invites
    where match_id='$MATCH_ID'::uuid and consumed_at is not null),
  'spectator_grants',(select count(*) from trnm_online_spectator_grants
    where match_id='$MATCH_ID'::uuid and viewer_player_id='$SPECTATOR'),
  'season_auto_audits',(select count(*) from trnm_online_season_automation_audit
    where season_id='$SEASON_ID'),
  'active_season',(select season_id from trnm_online_seasons where status='active'),
  'closed_escalation',(select count(*) from trnm_online_appeal_escalations
    where appeal_id='$APPEAL_ID'::uuid and status='closed'),
  'distinct_hosts',(select count(distinct physical_host_id) from trnm_online_fleet_instances
    where status in ('active','draining') and lease_expires_at>now())
)" | jq -c .)"
jq -e --arg season "$SEASON_ID" \
  '.spectator_invites == 1 and .spectator_grants == 1 and
   .season_auto_audits >= 3 and .active_season == $season and
   .closed_escalation == 1 and .distinct_hosts == 1' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --arg before_match "$MATCH_ID" --arg after_match "$AFTER_MATCH" \
  --arg old_key "$OLD_KEY_ID" --arg new_key "$NEW_KEY_ID" --arg season "$SEASON_ID" \
  --arg appeal "$APPEAL_ID" --arg grant "$GRANT_ID" \
  --argjson before_product "$BEFORE_PRODUCT" --argjson after_product "$AFTER_PRODUCT" \
  --argjson rotation "$rotation" --argjson status_view "$production_status" \
  --argjson database "$database" --argjson post_rotation "$post_rotation_receipts" \
  '{status:"passed",run_id:$run_id,before_rotation_match:$before_match,
    after_rotation_match:$after_match,isolated_signer_private_key_absent_from_game:true,
    signer_strict_idempotency:true,signer_tampered_replay_status:409,
    old_key_id:$old_key,new_key_id:$new_key,rotation:$rotation,
    game_server_survived_signer_rotation:true,post_rotation:$post_rotation,
    spectator_grant_id:$grant,targeted_invite_and_single_use:true,
    delayed_terminal_withheld_then_released:true,delay_fixture_backdated_seconds:31,
    season_id:$season,season_busy_deferred_then_auto_activated:true,
    appeal_id:$appeal,appeal_overdue_escalated_then_closed:true,
    control_plane_rate_limit_status:429,request_body_limit_status:413,
    physical_host_identity_enforced:true,distinct_healthy_physical_hosts:1,
    cross_host_failover_claimed:false,public_edge_attested:false,kms_hsm_attested:false,
    production_status:$status_view,database:$database,
    before_product:$before_product,after_product:$after_product,
    boundary:"local isolated signer and production controls; no human, second physical host, public edge or KMS/HSM attestation"}' \
  | tee "$EVIDENCE/report.json"
