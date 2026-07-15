-- Keep the running Authority v3 data plane at one PostgreSQL protocol round
-- trip under WAN-like database latency.  Admission is committed in the same
-- transaction as a command, while fleet heartbeat and periodic checkpoint
-- work execute as server-side statements so they do not hold rows across
-- client/server round trips.

create or replace function public.trnm_online_host_authority_exact_v1(
    p_physical_host_id text,
    p_host_owner_nonce uuid,
    p_host_application_name text,
    p_host_backend_pid integer,
    p_host_backend_started_at timestamptz,
    p_database_system_identifier text,
    p_database_timeline_id bigint,
    p_database_postmaster_started_at timestamptz,
    p_leader_lock_key bigint,
    p_barrier_lock_key bigint
)
returns boolean
language sql
stable
security invoker
set search_path = pg_catalog, public
as $function$
    select exists (
        select 1
          from public.trnm_online_physical_host_authorities authority
          join pg_catalog.pg_stat_activity activity
            on activity.pid = authority.backend_pid
          join pg_catalog.pg_locks authority_lock
            on authority_lock.pid = authority.backend_pid
         where authority.physical_host_id = p_physical_host_id
           and authority.owner_nonce = p_host_owner_nonce
           and authority.application_name = p_host_application_name
           and authority.backend_pid = p_host_backend_pid
           and authority.backend_started_at = p_host_backend_started_at
           and authority.database_system_identifier = p_database_system_identifier
           and authority.database_timeline_id = p_database_timeline_id
           and authority.database_postmaster_started_at =
               p_database_postmaster_started_at
           and authority.leader_lock_key = p_leader_lock_key
           and authority.barrier_lock_key = p_barrier_lock_key
           and activity.datname = pg_catalog.current_database()
           and activity.backend_start = p_host_backend_started_at
           and activity.application_name = p_host_application_name
           and authority_lock.locktype = 'advisory'
           and authority_lock.database = (
               select database.oid
                 from pg_catalog.pg_database database
                where database.datname = pg_catalog.current_database()
           )
           and authority_lock.classid::bigint =
               ((p_leader_lock_key >> 32) & 4294967295::bigint)
           and authority_lock.objid::bigint =
               (p_leader_lock_key & 4294967295::bigint)
           and authority_lock.objsubid = 1
           and authority_lock.mode = 'ExclusiveLock'
           and authority_lock.granted
           and not pg_catalog.pg_is_in_recovery()
           and (select system_identifier::text
                  from pg_catalog.pg_control_system()) =
               p_database_system_identifier
           and (select timeline_id::bigint
                  from pg_catalog.pg_control_checkpoint()) =
               p_database_timeline_id
           and pg_catalog.pg_postmaster_start_time() =
               p_database_postmaster_started_at
    ) and exists (
        select 1
          from pg_catalog.pg_locks barrier_lock
         where barrier_lock.pid = pg_catalog.pg_backend_pid()
           and barrier_lock.locktype = 'advisory'
           and barrier_lock.database = (
               select database.oid
                 from pg_catalog.pg_database database
                where database.datname = pg_catalog.current_database()
           )
           and barrier_lock.classid::bigint =
               ((p_barrier_lock_key >> 32) & 4294967295::bigint)
           and barrier_lock.objid::bigint =
               (p_barrier_lock_key & 4294967295::bigint)
           and barrier_lock.objsubid = 1
           and barrier_lock.mode = 'ShareLock'
           and barrier_lock.granted
    )
$function$;

create or replace function public.trnm_online_commit_actor_command_v2(
    p_match_id uuid,
    p_command_id text,
    p_player_id text,
    p_input_sequence bigint,
    p_request_hash text,
    p_target_tick bigint,
    p_client_observed_tick bigint,
    p_order_json jsonb,
    p_accepted_snapshot_hash text,
    p_accepted_match_revision bigint,
    p_post_simulation_json jsonb,
    p_base_next_sequence bigint,
    p_base_match_revision bigint,
    p_instance_id text,
    p_instance_epoch bigint,
    p_physical_host_id text,
    p_host_owner_nonce uuid,
    p_host_application_name text,
    p_host_backend_pid integer,
    p_host_backend_started_at timestamptz,
    p_database_system_identifier text,
    p_database_timeline_id bigint,
    p_database_postmaster_started_at timestamptz,
    p_leader_lock_key bigint,
    p_barrier_lock_key bigint,
    p_admission_bucket_key text,
    p_admission_limit bigint
)
returns table (
    result_outcome text,
    result_sequence bigint,
    result_input_sequence bigint,
    result_player_id text,
    result_request_hash text,
    result_accepted_match_revision bigint,
    result_accepted_snapshot_hash text,
    result_target_tick bigint,
    result_client_observed_tick bigint,
    result_durable_next_sequence bigint,
    result_durable_match_revision bigint,
    result_checkpoint_sequence bigint,
    result_durable_member_input_sequence bigint,
    result_phase text
)
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    admission_count bigint;
begin
    if not public.trnm_online_host_authority_exact_v1(
        p_physical_host_id, p_host_owner_nonce, p_host_application_name,
        p_host_backend_pid, p_host_backend_started_at,
        p_database_system_identifier, p_database_timeline_id,
        p_database_postmaster_started_at, p_leader_lock_key,
        p_barrier_lock_key
    ) then
        result_outcome := 'host_authority_fenced';
        return next;
        return;
    end if;
    if p_admission_bucket_key !~ '^[0-9a-f]{64}$'
       or p_admission_limit <= 0 then
        result_outcome := 'admission_invalid';
        return next;
        return;
    end if;

    insert into public.trnm_online_admission_windows (
        bucket_key, window_started_at, request_class, request_count,
        rejection_count, last_instance_id
    ) values (
        p_admission_bucket_key, pg_catalog.date_trunc('minute', pg_catalog.now()),
        'data', 1, 0, p_instance_id
    )
    on conflict (bucket_key, window_started_at) do update set
        request_count = public.trnm_online_admission_windows.request_count + 1,
        last_instance_id = excluded.last_instance_id,
        updated_at = pg_catalog.now()
    returning request_count into admission_count;

    if admission_count > p_admission_limit then
        update public.trnm_online_admission_windows
           set rejection_count = rejection_count + 1,
               updated_at = pg_catalog.now()
         where bucket_key = p_admission_bucket_key
           and window_started_at =
               pg_catalog.date_trunc('minute', pg_catalog.now());
        result_outcome := 'rate_limited';
        return next;
        return;
    end if;

    return query
    select *
      from public.trnm_online_commit_actor_command_v1(
        p_match_id, p_command_id, p_player_id, p_input_sequence,
        p_request_hash, p_target_tick, p_client_observed_tick, p_order_json,
        p_accepted_snapshot_hash, p_accepted_match_revision,
        p_post_simulation_json, p_base_next_sequence, p_base_match_revision,
        p_instance_id, p_instance_epoch, p_physical_host_id,
        p_host_owner_nonce, p_host_application_name, p_host_backend_pid,
        p_host_backend_started_at, p_database_system_identifier,
        p_database_timeline_id, p_database_postmaster_started_at,
        p_leader_lock_key, p_barrier_lock_key
      );
end
$function$;

create or replace function public.trnm_online_heartbeat_fleet_v1(
    p_instance_id text,
    p_instance_epoch bigint,
    p_physical_host_id text,
    p_region text,
    p_public_endpoint text,
    p_build_id text,
    p_capacity integer,
    p_host_owner_nonce uuid,
    p_host_application_name text,
    p_host_backend_pid integer,
    p_host_backend_started_at timestamptz,
    p_database_system_identifier text,
    p_database_timeline_id bigint,
    p_database_postmaster_started_at timestamptz,
    p_leader_lock_key bigint,
    p_barrier_lock_key bigint
)
returns boolean
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    active_match_count bigint;
    affected_rows bigint;
begin
    if not public.trnm_online_host_authority_exact_v1(
        p_physical_host_id, p_host_owner_nonce, p_host_application_name,
        p_host_backend_pid, p_host_backend_started_at,
        p_database_system_identifier, p_database_timeline_id,
        p_database_postmaster_started_at, p_leader_lock_key,
        p_barrier_lock_key
    ) then
        return false;
    end if;

    select count(*)
      into active_match_count
      from public.trnm_online_matches match
     where match.phase = 'running'
       and match.assigned_instance_id = p_instance_id
       and match.assigned_instance_epoch = p_instance_epoch
       and match.assigned_physical_host_id = p_physical_host_id;

    update public.trnm_online_fleet_instances fleet
       set region = p_region,
           public_endpoint = p_public_endpoint,
           build_id = p_build_id,
           capacity = p_capacity,
           active_matches = case
               when active_match_count > 2147483647 then 2147483647
               else active_match_count::integer
           end,
           heartbeat_at = pg_catalog.now(),
           lease_expires_at = pg_catalog.now() + interval '5 seconds',
           physical_host_id = p_physical_host_id
     where fleet.instance_id = p_instance_id
       and fleet.instance_epoch = p_instance_epoch
       and fleet.physical_host_id = p_physical_host_id
       and fleet.status <> 'offline';
    get diagnostics affected_rows = row_count;
    return affected_rows = 1;
end
$function$;

create or replace function public.trnm_online_checkpoint_actor_v1(
    p_match_id uuid,
    p_tick bigint,
    p_snapshot_hash text,
    p_simulation_json jsonb,
    p_next_sequence bigint,
    p_match_revision bigint,
    p_instance_id text,
    p_instance_epoch bigint,
    p_physical_host_id text,
    p_host_owner_nonce uuid,
    p_host_application_name text,
    p_host_backend_pid integer,
    p_host_backend_started_at timestamptz,
    p_database_system_identifier text,
    p_database_timeline_id bigint,
    p_database_postmaster_started_at timestamptz,
    p_leader_lock_key bigint,
    p_barrier_lock_key bigint
)
returns boolean
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    fleet_record record;
    match_record record;
    affected_rows bigint;
begin
    if not public.trnm_online_host_authority_exact_v1(
        p_physical_host_id, p_host_owner_nonce, p_host_application_name,
        p_host_backend_pid, p_host_backend_started_at,
        p_database_system_identifier, p_database_timeline_id,
        p_database_postmaster_started_at, p_leader_lock_key,
        p_barrier_lock_key
    ) then
        return false;
    end if;

    select fleet.instance_epoch, fleet.status, fleet.physical_host_id,
           fleet.lease_expires_at > pg_catalog.now() as lease_current
      into fleet_record
      from public.trnm_online_fleet_instances fleet
     where fleet.instance_id = p_instance_id
     for share;
    if not found
       or fleet_record.instance_epoch <> p_instance_epoch
       or fleet_record.physical_host_id <> p_physical_host_id
       or fleet_record.status not in ('active', 'draining')
       or not fleet_record.lease_current then
        return false;
    end if;

    select match.phase, match.assigned_instance_id,
           match.assigned_instance_epoch, match.assigned_physical_host_id,
           match.terminal_stage_snapshot_hash, match.checkpoint_sequence,
           match.next_sequence, match.match_revision
      into match_record
      from public.trnm_online_matches match
     where match.match_id = p_match_id
     for update;
    if not found
       or match_record.phase <> 'running'
       or match_record.assigned_instance_id is distinct from p_instance_id
       or match_record.assigned_instance_epoch is distinct from p_instance_epoch
       or match_record.assigned_physical_host_id is distinct from p_physical_host_id
       or match_record.terminal_stage_snapshot_hash is not null
       or match_record.checkpoint_sequence > p_next_sequence
       or match_record.next_sequence < p_next_sequence
       or match_record.match_revision < p_match_revision then
        return false;
    end if;

    insert into public.trnm_online_replay_frames (
        match_id, tick, snapshot_hash, simulation_json, frame_kind
    ) values (
        p_match_id, p_tick, p_snapshot_hash, p_simulation_json, 'checkpoint'
    )
    on conflict (match_id, tick) do update set
        snapshot_hash = excluded.snapshot_hash,
        simulation_json = excluded.simulation_json,
        frame_kind = excluded.frame_kind;

    update public.trnm_online_matches match
       set simulation_json = p_simulation_json,
           snapshot_hash = p_snapshot_hash,
           authoritative_tick = p_tick,
           checkpoint_sequence = p_next_sequence,
           updated_at = pg_catalog.now()
     where match.match_id = p_match_id
       and match.phase = 'running'
       and match.assigned_instance_id = p_instance_id
       and match.assigned_instance_epoch = p_instance_epoch
       and match.assigned_physical_host_id = p_physical_host_id
       and match.terminal_stage_snapshot_hash is null
       and match.checkpoint_sequence <= p_next_sequence
       and match.next_sequence >= p_next_sequence;
    get diagnostics affected_rows = row_count;
    if affected_rows <> 1 then
        raise exception using
            errcode = 'P0001',
            message = 'TRNM checkpoint cursor changed inside locked commit';
    end if;
    return true;
end
$function$;

comment on function public.trnm_online_commit_actor_command_v2(
    uuid, text, text, bigint, text, bigint, bigint, jsonb, text, bigint,
    jsonb, bigint, bigint, text, bigint, text, uuid, text, integer,
    timestamptz, text, bigint, timestamptz, bigint, bigint, text, bigint
) is
'One-round-trip Authority v3 admission plus durable running-command commit.';

comment on function public.trnm_online_heartbeat_fleet_v1(
    text, bigint, text, text, text, text, integer, uuid, text, integer,
    timestamptz, text, bigint, timestamptz, bigint, bigint
) is
'One-statement fenced fleet heartbeat without WAN-held row locks.';

comment on function public.trnm_online_checkpoint_actor_v1(
    uuid, bigint, text, jsonb, bigint, bigint, text, bigint, text, uuid,
    text, integer, timestamptz, text, bigint, timestamptz, bigint, bigint
) is
'One-statement fenced periodic actor checkpoint without WAN-held row locks.';
