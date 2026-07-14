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
RUN_ID="online-authority-$(date +%s)-${RANDOM}"
E2E_TICK_MS="${TRNM_E2E_TICK_MS:-20}"
MANAGE_SERVER="${TRNM_ONLINE_E2E_MANAGE_SERVER:-1}"

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local role="$1" account player recovery session
  account="$(admin_post /v1/accounts "$(jq -cn \
    --arg org '00000000-0000-0000-0000-00000000ce01' --arg role "$role" \
    '{org_id:$org,account_type:("online-authority-"+$role),currency_unit:"credit",initial_balance:0}')" \
    | jq -er .account_id)"
  player="$RUN_ID-$role"
  recovery="recovery-$RUN_ID-$role-012345678901234567890123"
  admin_post /v1/trnm/identity/register "$(jq -cn \
    --arg player "$player" --arg account "$account" --arg recovery "$recovery" \
    '{player_id:$player,account_id:$account,recovery_key:$recovery}')" >/dev/null
  session="$(curl -fsS "$LEDGER_URL/v1/trnm/identity/session" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg recovery "$recovery" --arg device "$RUN_ID-$role-device" \
      '{player_id:$player,recovery_key:$recovery,device_id:$device,lifetime_seconds:3600}')" \
    | jq -er .session_token)"
  printf '%s\t%s\t%s\n' "$player" "$account" "$session"
}

restore_runtime() {
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS TRNM_ALLOW_ACCELERATED_TEST_CLOCK || true
  systemctl --user restart trnm-game-server.service || true
}
if [[ "$MANAGE_SERVER" == "1" ]]; then
  trap restore_runtime EXIT
fi

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_SESSION < <(create_identity host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_SESSION < <(create_identity guest)

if [[ "$MANAGE_SERVER" == "1" ]]; then
  systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS="$E2E_TICK_MS" \
    TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1
  systemctl --user restart trnm-game-server.service
fi
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" | jq -e '.status == "ok"' >/dev/null

report="$(TRNM_GAME_SERVER_URL="$ONLINE_URL" \
  TRNM_ONLINE_HOST_PLAYER_ID="$HOST_PLAYER" \
  TRNM_ONLINE_HOST_ACCOUNT_ID="$HOST_ACCOUNT" \
  TRNM_ONLINE_HOST_SESSION="$HOST_SESSION" \
  TRNM_ONLINE_GUEST_PLAYER_ID="$GUEST_PLAYER" \
  TRNM_ONLINE_GUEST_ACCOUNT_ID="$GUEST_ACCOUNT" \
  TRNM_ONLINE_GUEST_SESSION="$GUEST_SESSION" \
  "$ROOT_DIR/target/release/trnm-online-e2e")"

wallet="$(curl -fsS "$LEDGER_URL/v1/trnm/economy/wallet" \
  -H "x-trnm-player-session: $HOST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$HOST_PLAYER" --arg account "$HOST_ACCOUNT" \
    '{actor_id:$player,account_id:$account,reconciliation_cursor:0}')")"
jq -e '.available_credits > 0 and .reserved_credits == 0' <<<"$wallet" >/dev/null
guest_wallet="$(curl -fsS "$LEDGER_URL/v1/trnm/economy/wallet" \
  -H "x-trnm-player-session: $GUEST_SESSION" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg player "$GUEST_PLAYER" --arg account "$GUEST_ACCOUNT" \
    '{actor_id:$player,account_id:$account,reconciliation_cursor:0}')")"
jq -e '.available_credits > 0 and .reserved_credits == 0' <<<"$guest_wallet" >/dev/null

match_id="$(jq -er .match_id <<<"$report")"
database_evidence="$(cex_psql_stdin -Atc "select json_build_object(
  'campaigns',(select count(*) from trnm_online_campaigns where player_id = '$HOST_PLAYER'),
  'matches',(select count(*) from trnm_online_matches where match_id = '$match_id'::uuid),
  'members',(select count(*) from trnm_online_match_members where match_id = '$match_id'::uuid),
  'member_campaigns',(select count(distinct campaign_id) from trnm_online_match_members where match_id = '$match_id'::uuid),
  'progression_events',(select count(*) from trnm_online_progression_events where match_id = '$match_id'::uuid),
  'ed25519_entitlements',(select count(*) from trnm_value_entitlements where entitlement_json->>'contract_version' = 'trnm_server_signed_value_entitlement_v2' and entitlement_json->>'signature_algorithm' = 'ed25519' and entitlement_json->>'match_id' = '$match_id'),
  'commands',(select count(*) from trnm_online_commands where match_id = '$match_id'::uuid),
  'fingerprinted_commands',(select count(*) from trnm_online_commands where match_id = '$match_id'::uuid and request_hash is not null and length(request_hash) = 64),
  'nonnull_input_sequences',(select count(*) from trnm_online_commands where match_id = '$match_id'::uuid and input_sequence is not null),
  'client_observed_ticks',(select count(*) from trnm_online_commands where match_id = '$match_id'::uuid and client_observed_tick is not null),
  'duplicate_sequences',(select count(*) from (
    select sequence from trnm_online_commands where match_id = '$match_id'::uuid
    group by sequence having count(*) > 1) duplicates),
  'duplicate_player_inputs',(select count(*) from (
    select player_id,input_sequence from trnm_online_commands
    where match_id = '$match_id'::uuid and input_sequence is not null
    group by player_id,input_sequence having count(*) > 1) duplicates),
  'member_cursor_mismatches',(select count(*) from (
    select member.player_id
    from trnm_online_match_members member
    left join trnm_online_commands command
      on command.match_id = member.match_id and command.player_id = member.player_id
    where member.match_id = '$match_id'::uuid
    group by member.player_id,member.next_input_sequence
    having member.next_input_sequence <> count(command.sequence)) mismatches),
  'compatibility_trigger_count',(select count(*) from pg_trigger
    where tgname = 'trg_trnm_online_assign_legacy_input_sequence'
      and tgrelid = 'trnm_online_commands'::regclass and not tgisinternal),
  'partial_input_index_count',(select count(*) from pg_indexes
    where schemaname = current_schema()
      and indexname = 'idx_trnm_online_player_input_sequence'
      and indexdef ilike '% where %'),
  'phase',(select phase from trnm_online_matches where match_id = '$match_id'::uuid),
  'settlement',(select settlement_state from trnm_online_matches where match_id = '$match_id'::uuid)
)" | jq -c .)"
jq -e '.campaigns == 1 and .matches == 1 and .members == 2 and
  .member_campaigns == 2 and .progression_events == 2 and
  .ed25519_entitlements == 2 and
  .commands >= 8 and .fingerprinted_commands == .commands and
  .nonnull_input_sequences == .commands and .client_observed_ticks == .commands and
  .duplicate_sequences == 0 and .duplicate_player_inputs == 0 and
  .member_cursor_mismatches == 0 and .compatibility_trigger_count == 1 and
  .partial_input_index_count == 1 and .phase == "complete" and
  .settlement == "settled"' <<<"$database_evidence" >/dev/null

server_managed_json=false
if [[ "$MANAGE_SERVER" == "1" ]]; then
  server_managed_json=true
fi
jq -n --argjson report "$report" --argjson wallet "$wallet" \
  --argjson guest_wallet "$guest_wallet" \
  --argjson database "$database_evidence" \
  --argjson server_managed "$server_managed_json" \
  '$report + {wallet:$wallet,guest_wallet:$guest_wallet,database:$database,server_restart_via_systemd:$server_managed}'
