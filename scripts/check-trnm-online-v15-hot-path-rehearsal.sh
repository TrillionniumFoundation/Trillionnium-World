#!/usr/bin/bash
set -euo pipefail
umask 077

readonly ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
readonly CEX_HELPER="$ROOT_DIR/../CEX/scripts/_dev-helpers.sh"
readonly MIGRATION="$ROOT_DIR/trillionnium/crates/trnm-game-server/migrations/0015_online_realtime_hot_path_v1.sql"
readonly REHEARSAL_CONTRACT="trnm_online_v15_hot_path_rehearsal_v1"

[[ -f "$CEX_HELPER" && ! -L "$CEX_HELPER" ]] || {
  echo "missing canonical CEX helper: $CEX_HELPER" >&2
  exit 1
}
[[ -f "$MIGRATION" && ! -L "$MIGRATION" ]] || {
  echo "missing V15 migration: $MIGRATION" >&2
  exit 1
}

# shellcheck source=/dev/null
source "$CEX_HELPER"
cex_load_env
cex_require_cmd awk sed sha256sum jq

MIGRATION_SHA256="$(sha256sum -- "$MIGRATION" | awk '{print $1}')"
readonly MIGRATION_SHA256
[[ "$MIGRATION_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
  echo "could not fingerprint the V15 migration" >&2
  exit 1
}

run_psql() (
  set -euo pipefail
  cex_psql_stdin "$@"
)

v15_persistent_state() {
  run_psql -X -At -v ON_ERROR_STOP=1 -c "
    select concat_ws('|',
        (select count(*)
           from pg_proc procedure
           join pg_namespace namespace on namespace.oid = procedure.pronamespace
          where namespace.nspname = 'public'
            and procedure.proname in (
              'trnm_online_host_authority_exact_v1',
              'trnm_online_commit_actor_command_v2',
              'trnm_online_heartbeat_fleet_v1',
              'trnm_online_checkpoint_actor_v1'
            )),
        coalesce((
            select migration_name || ':' || checksum_sha256 || ':'
                   || extract(epoch from applied_at)::text
              from public.trnm_online_schema_migrations
             where migration_version = 15
        ), 'absent')
    )"
}

BEFORE_STATE="$(v15_persistent_state)"
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
    host_player_id text := 'v15-host-player-' || rehearsal_match_id::text;
    host_campaign_id text := 'v15-host-campaign-' || rehearsal_match_id::text;
    rehearsal_instance_id text := 'v15-instance-' || rehearsal_match_id::text;
    admission_key text := repeat('d', 64);
    heartbeat_result boolean;
    checkpoint_result boolean;
begin
    select host_authority.*
      into strict authority
      from public.trnm_online_physical_host_authorities host_authority
     order by host_authority.claimed_at desc
     limit 1;

    perform pg_advisory_xact_lock_shared(authority.barrier_lock_key);
    if not public.trnm_online_host_authority_exact_v1(
        authority.physical_host_id, authority.owner_nonce,
        authority.application_name, authority.backend_pid,
        authority.backend_started_at, authority.database_system_identifier,
        authority.database_timeline_id,
        authority.database_postmaster_started_at,
        authority.leader_lock_key, authority.barrier_lock_key
    ) then
        raise exception 'V15_REHEARSAL_EXACT_HOST_AUTHORITY_MISMATCH';
    end if;
    if public.trnm_online_host_authority_exact_v1(
        authority.physical_host_id, gen_random_uuid(),
        authority.application_name, authority.backend_pid,
        authority.backend_started_at, authority.database_system_identifier,
        authority.database_timeline_id,
        authority.database_postmaster_started_at,
        authority.leader_lock_key, authority.barrier_lock_key
    ) then
        raise exception 'V15_REHEARSAL_WRONG_HOST_NONCE_ACCEPTED';
    end if;

    insert into public.trnm_online_fleet_instances (
        instance_id, region, public_endpoint, build_id, capacity, status,
        instance_epoch, lease_expires_at, physical_host_id
    ) values (
        rehearsal_instance_id, 'v15-rehearsal', 'http://127.0.0.1:1',
        'trnm-online-authority-2026.07-v3', 1, 'active', 1,
        now() + interval '1 hour', authority.physical_host_id
    );

    insert into public.trnm_online_campaigns (
        campaign_id, player_id, account_id, slot_key, campaign_revision,
        schema_revision, state_hash, campaign_json
    ) values (
        host_campaign_id, host_player_id, host_account_id,
        'v15-host-' || rehearsal_match_id::text,
        0, 12, repeat('0', 64), '{}'::jsonb
    );

    insert into public.trnm_online_matches (
        match_id, campaign_id, host_player_id, host_account_id, join_code,
        phase, build_id, map_id, rules_version, simulation_json,
        snapshot_hash, assigned_instance_id, assigned_region,
        assigned_instance_epoch, assigned_physical_host_id
    ) values (
        rehearsal_match_id, host_campaign_id, host_player_id, host_account_id,
        'v15-' || rehearsal_match_id::text, 'running',
        'trnm-online-authority-2026.07-v3', 'first_contact_river_watch',
        'trnm-rts-rules-v1', '{\"tick\":0}'::jsonb, repeat('a', 64),
        rehearsal_instance_id, 'v15-rehearsal', 1,
        authority.physical_host_id
    );

    insert into public.trnm_online_match_members (
        match_id, player_id, account_id, member_role, controlled_unit_ids,
        campaign_id
    ) values (
        rehearsal_match_id, host_player_id, host_account_id, 'host',
        '[\"host-unit\"]'::jsonb, host_campaign_id
    );

    heartbeat_result := public.trnm_online_heartbeat_fleet_v1(
        rehearsal_instance_id, 1, authority.physical_host_id,
        'v15-rehearsal', 'http://127.0.0.1:1',
        'trnm-online-production-2026.07-v2', 1,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at,
        authority.leader_lock_key, authority.barrier_lock_key
    );
    if not heartbeat_result or not exists (
        select 1 from public.trnm_online_fleet_instances fleet
         where fleet.instance_id = rehearsal_instance_id
           and fleet.active_matches = 1
    ) then
        raise exception 'V15_REHEARSAL_HEARTBEAT_MISMATCH result=%, active=%',
            heartbeat_result,
            (select fleet.active_matches
               from public.trnm_online_fleet_instances fleet
              where fleet.instance_id = rehearsal_instance_id);
    end if;

    select * into strict result
      from public.trnm_online_commit_actor_command_v2(
        rehearsal_match_id, 'v15-command-0', host_player_id, 0,
        repeat('1', 64), 10, 9, '{\"frame\":10}'::jsonb,
        repeat('2', 64), 1, '{\"tick\":10}'::jsonb, 0, 0,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key, admission_key, 1
      );
    if result.result_outcome <> 'inserted'
       or result.result_sequence <> 0
       or result.result_durable_next_sequence <> 1 then
        raise exception 'V15_REHEARSAL_COMMAND_INSERT_MISMATCH';
    end if;

    select * into strict result
      from public.trnm_online_commit_actor_command_v2(
        rehearsal_match_id, 'v15-command-rate-limited', host_player_id, 1,
        repeat('3', 64), 11, 10, '{\"frame\":11}'::jsonb,
        repeat('4', 64), 2, '{\"tick\":11}'::jsonb, 1, 1,
        rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at, authority.leader_lock_key,
        authority.barrier_lock_key, admission_key, 1
      );
    if result.result_outcome <> 'rate_limited'
       or (select count(*) from public.trnm_online_commands command
            where command.match_id = rehearsal_match_id) <> 1
       or not exists (
            select 1 from public.trnm_online_matches match
             where match.match_id = rehearsal_match_id
               and match.next_sequence = 1 and match.match_revision = 1
       ) or not exists (
            select 1 from public.trnm_online_admission_windows admission
             where admission.bucket_key = admission_key
               and admission.request_count = 2
               and admission.rejection_count = 1
       ) then
        raise exception 'V15_REHEARSAL_ATOMIC_ADMISSION_MISMATCH';
    end if;

    checkpoint_result := public.trnm_online_checkpoint_actor_v1(
        rehearsal_match_id, 10, repeat('2', 64), '{\"tick\":10}'::jsonb,
        1, 1, rehearsal_instance_id, 1, authority.physical_host_id,
        authority.owner_nonce, authority.application_name,
        authority.backend_pid, authority.backend_started_at,
        authority.database_system_identifier, authority.database_timeline_id,
        authority.database_postmaster_started_at,
        authority.leader_lock_key, authority.barrier_lock_key
    );
    if not checkpoint_result or not exists (
        select 1 from public.trnm_online_matches match
         where match.match_id = rehearsal_match_id
           and match.authoritative_tick = 10
           and match.checkpoint_sequence = 1
           and match.snapshot_hash = repeat('2', 64)
    ) or not exists (
        select 1 from public.trnm_online_replay_frames frame
         where frame.match_id = rehearsal_match_id and frame.tick = 10
           and frame.snapshot_hash = repeat('2', 64)
           and frame.frame_kind = 'checkpoint'
    ) then
        raise exception 'V15_REHEARSAL_CHECKPOINT_MISMATCH result=%',
            checkpoint_result;
    end if;
end
\$rehearsal\$;

rollback;"
} | run_psql -X -At -v ON_ERROR_STOP=1 2>&1)"
REHEARSAL_STATUS=$?
set -e

if (( REHEARSAL_STATUS != 0 )); then
  printf '%s\n' "$REHEARSAL_OUTPUT" >&2
  echo "V15 realtime hot-path rehearsal failed" >&2
  exit "$REHEARSAL_STATUS"
fi

AFTER_STATE="$(v15_persistent_state)"
readonly AFTER_STATE
[[ "$AFTER_STATE" == "$BEFORE_STATE" ]] || {
  echo "V15 rehearsal did not restore persistent migration/function state" >&2
  exit 1
}

jq -n \
  --arg contract_version "$REHEARSAL_CONTRACT" \
  --arg migration_sha256 "$MIGRATION_SHA256" \
  --arg before_state "$BEFORE_STATE" \
  --arg after_state "$AFTER_STATE" \
  '{contract_version:$contract_version,passed:true,rolled_back:true,
    migration_sha256:$migration_sha256,before_state:$before_state,
    after_state:$after_state,checks:{host_k1_k2_exact:true,
    wrong_host_nonce_rejected:true,heartbeat_single_statement:true,
    command_admission_atomic:true,rate_limit_no_command_mutation:true,
    checkpoint_single_statement:true,persistent_state_restored:true}}'
