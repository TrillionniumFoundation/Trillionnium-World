#!/usr/bin/bash
set -euo pipefail
umask 077

readonly ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
readonly CEX_HELPER="$ROOT_DIR/../CEX/scripts/_dev-helpers.sh"
readonly MIGRATION="$ROOT_DIR/trillionnium/crates/trnm-game-server/migrations/0014_online_command_commit_rpc_v1.sql"
readonly REHEARSAL_CONTRACT="trnm_online_v14_command_rpc_rehearsal_v1"

[[ -f "$CEX_HELPER" && ! -L "$CEX_HELPER" ]] || {
  echo "missing canonical CEX helper: $CEX_HELPER" >&2
  exit 1
}
[[ -f "$MIGRATION" && ! -L "$MIGRATION" ]] || {
  echo "missing V14 migration: $MIGRATION" >&2
  exit 1
}

# shellcheck source=/dev/null
source "$CEX_HELPER"
cex_load_env
cex_require_cmd awk sed sha256sum jq

MIGRATION_SHA256="$(sha256sum -- "$MIGRATION" | awk '{print $1}')"
readonly MIGRATION_SHA256
[[ "$MIGRATION_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
  echo "could not fingerprint the V14 migration" >&2
  exit 1
}

run_psql() (
  set -euo pipefail
  cex_psql_stdin "$@"
)

v14_persistent_state() {
  run_psql -X -At -v ON_ERROR_STOP=1 -c "
    select concat_ws('|',
        (select count(*)
           from pg_proc procedure
           join pg_namespace namespace on namespace.oid = procedure.pronamespace
          where namespace.nspname = 'public'
            and procedure.proname = 'trnm_online_commit_actor_command_v1'
            and procedure.pronargs = 25),
        coalesce((
            select migration_name || ':' || checksum_sha256 || ':'
                   || extract(epoch from applied_at)::text
              from public.trnm_online_schema_migrations
             where migration_version = 14
        ), 'absent')
    )"
}

BEFORE_STATE="$(v14_persistent_state)"
readonly BEFORE_STATE

set +e
REHEARSAL_OUTPUT="$({
  printf '%s\n' \
    '\set ON_ERROR_STOP on' \
    'begin;' \
    "set local application_name = '$REHEARSAL_CONTRACT';" \
    "set local lock_timeout = '10s';" \
    "set local statement_timeout = '120s';"
  sed -n '1,$p' "$MIGRATION"
  printf '%s\n' "
do \$rehearsal\$
declare
    authority record;
    result record;
    rehearsal_match_id uuid := gen_random_uuid();
    host_account_id uuid := gen_random_uuid();
    guest_account_id uuid := gen_random_uuid();
    host_player_id text := 'v14-host-player-' || rehearsal_match_id::text;
    guest_player_id text := 'v14-guest-player-' || rehearsal_match_id::text;
    host_campaign_id text := 'v14-host-campaign-' || rehearsal_match_id::text;
    guest_campaign_id text := 'v14-guest-campaign-' || rehearsal_match_id::text;
    rehearsal_instance_id text := 'v14-instance-' || rehearsal_match_id::text;
begin
    select host_authority.*
      into strict authority
      from public.trnm_online_physical_host_authorities host_authority
     order by host_authority.claimed_at desc
     limit 1;

    perform pg_advisory_xact_lock_shared(authority.barrier_lock_key);

    insert into public.trnm_online_fleet_instances (
        instance_id, region, public_endpoint, build_id, capacity, status,
        instance_epoch, lease_expires_at, physical_host_id
    ) values (
        rehearsal_instance_id, 'v14-rehearsal', 'http://127.0.0.1:1',
        'trnm-online-authority-2026.07-v3', 1, 'active', 1,
        now() + interval '1 hour', authority.physical_host_id
    );

    insert into public.trnm_online_campaigns (
        campaign_id, player_id, account_id, slot_key, campaign_revision,
        schema_revision, state_hash, campaign_json
    ) values
      (host_campaign_id, host_player_id, host_account_id,
       'v14-host-' || rehearsal_match_id::text, 0, 12, repeat('0', 64), '{}'::jsonb),
      (guest_campaign_id, guest_player_id, guest_account_id,
       'v14-guest-' || rehearsal_match_id::text, 0, 12, repeat('0', 64), '{}'::jsonb);

    insert into public.trnm_online_matches (
        match_id, campaign_id, host_player_id, host_account_id, join_code,
        phase, build_id, map_id, rules_version, simulation_json,
        snapshot_hash, assigned_instance_id, assigned_region,
        assigned_instance_epoch, assigned_physical_host_id
    ) values (
        rehearsal_match_id, host_campaign_id, host_player_id, host_account_id,
        'v14-' || rehearsal_match_id::text, 'running',
        'trnm-online-authority-2026.07-v3', 'first_contact_river_watch',
        'trnm-rts-rules-v1', '{\"tick\":0}'::jsonb, repeat('a', 64),
        rehearsal_instance_id, 'v14-rehearsal', 1, authority.physical_host_id
    );

    insert into public.trnm_online_match_members (
        match_id, player_id, account_id, member_role, controlled_unit_ids,
        campaign_id
    ) values
      (rehearsal_match_id, host_player_id, host_account_id, 'host',
       '[\"host-unit\"]'::jsonb, host_campaign_id),
      (rehearsal_match_id, guest_player_id, guest_account_id, 'coop_guest',
       '[\"guest-unit\"]'::jsonb, guest_campaign_id);

    select * into strict result
      from public.trnm_online_commit_actor_command_v1(
        rehearsal_match_id, 'v14-command-0', host_player_id, 0,
        repeat('1', 64), 10, 9, '{\"frame\":10}'::jsonb,
        repeat('2', 64), 1, '{\"tick\":10}'::jsonb, 0, 0,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key
      );
    if result.result_outcome <> 'inserted'
       or result.result_sequence <> 0
       or result.result_input_sequence <> 0
       or result.result_durable_next_sequence <> 1
       or result.result_durable_member_input_sequence <> 1 then
        raise exception 'V14_REHEARSAL_FIRST_INSERT_MISMATCH';
    end if;

    select * into strict result
      from public.trnm_online_commit_actor_command_v1(
        rehearsal_match_id, 'v14-command-0', host_player_id, 0,
        repeat('1', 64), 10, 9, '{\"frame\":10}'::jsonb,
        repeat('2', 64), 1, '{\"tick\":10}'::jsonb, 0, 0,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key
      );
    if result.result_outcome <> 'duplicate' then
        raise exception 'V14_REHEARSAL_DUPLICATE_MISMATCH';
    end if;

    select * into strict result
      from public.trnm_online_commit_actor_command_v1(
        rehearsal_match_id, 'v14-command-0', host_player_id, 0,
        repeat('9', 64), 10, 9, '{\"frame\":10}'::jsonb,
        repeat('2', 64), 1, '{\"tick\":10}'::jsonb, 0, 0,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key
      );
    if result.result_outcome <> 'command_conflict' then
        raise exception 'V14_REHEARSAL_ALTERED_DUPLICATE_NOT_REJECTED';
    end if;

    select * into strict result
      from public.trnm_online_commit_actor_command_v1(
        rehearsal_match_id, 'v14-command-stale', host_player_id, 0,
        repeat('3', 64), 11, 10, '{\"frame\":11}'::jsonb,
        repeat('4', 64), 1, '{\"tick\":11}'::jsonb, 0, 0,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key
      );
    if result.result_outcome <> 'match_cursor_fenced' then
        raise exception 'V14_REHEARSAL_STALE_CURSOR_NOT_FENCED';
    end if;

    select * into strict result
      from public.trnm_online_commit_actor_command_v1(
        rehearsal_match_id, 'v14-command-1', host_player_id, 1,
        repeat('5', 64), 11, 10, '{\"frame\":11}'::jsonb,
        repeat('6', 64), 2, '{\"tick\":11}'::jsonb, 1, 1,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key
      );
    if result.result_outcome <> 'inserted'
       or result.result_sequence <> 1
       or result.result_input_sequence <> 1 then
        raise exception 'V14_REHEARSAL_SECOND_INSERT_MISMATCH';
    end if;

    if not exists (
        select 1 from public.trnm_online_matches match
         where match.match_id = rehearsal_match_id
           and match.next_sequence = 2 and match.match_revision = 2
    ) or not exists (
        select 1 from public.trnm_online_match_members member
         where member.match_id = rehearsal_match_id
           and member.player_id = host_player_id
           and member.next_input_sequence = 2
    ) or (select count(*) from public.trnm_online_commands command
          where command.match_id = rehearsal_match_id) <> 2 then
        raise exception 'V14_REHEARSAL_ATOMIC_CURSOR_STATE_MISMATCH';
    end if;
end
\$rehearsal\$;

rollback;"
} | run_psql -X -At -v ON_ERROR_STOP=1 2>&1)"
REHEARSAL_STATUS=$?
set -e

if (( REHEARSAL_STATUS != 0 )); then
  printf '%s\n' "$REHEARSAL_OUTPUT" >&2
  echo "V14 command RPC rehearsal failed" >&2
  exit "$REHEARSAL_STATUS"
fi

AFTER_STATE="$(v14_persistent_state)"
readonly AFTER_STATE
[[ "$AFTER_STATE" == "$BEFORE_STATE" ]] || {
  echo "V14 rehearsal did not restore the persistent migration/function state" >&2
  exit 1
}

jq -n \
  --arg contract_version "$REHEARSAL_CONTRACT" \
  --arg migration_sha256 "$MIGRATION_SHA256" \
  --arg before_state "$BEFORE_STATE" \
  --arg after_state "$AFTER_STATE" \
  '{contract_version:$contract_version,passed:true,rolled_back:true,
    migration_sha256:$migration_sha256,before_state:$before_state,
    after_state:$after_state,checks:{first_insert:true,exact_duplicate:true,
    altered_duplicate_rejected:true,stale_cursor_fenced:true,
    second_insert:true,atomic_cursors:true,persistent_state_restored:true}}'
