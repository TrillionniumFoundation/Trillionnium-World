-- Collapse the running-match command commit into one PostgreSQL statement.
-- The caller's pool session already owns the shared K2 handoff barrier for
-- its entire physical lifetime. This function revalidates the exact K1 owner,
-- locks fleet -> match -> member in the canonical order, and advances the
-- command event plus both cursors atomically inside this single statement.

create or replace function public.trnm_online_commit_actor_command_v1(
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
    p_barrier_lock_key bigint
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
    fleet_record record;
    match_record record;
    member_record record;
    command_record record;
    affected_rows bigint;
begin
    if not exists (
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
    ) then
        result_outcome := 'host_authority_fenced';
        return next;
        return;
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
        result_outcome := 'fleet_fenced';
        return next;
        return;
    end if;

    select match.phase, match.next_sequence, match.match_revision,
           match.checkpoint_sequence, match.assigned_instance_id,
           match.assigned_instance_epoch, match.assigned_physical_host_id,
           match.terminal_stage_snapshot_hash
      into match_record
      from public.trnm_online_matches match
     where match.match_id = p_match_id
     for update;
    if not found then
        result_outcome := 'match_not_found';
        return next;
        return;
    end if;

    result_durable_next_sequence := match_record.next_sequence;
    result_durable_match_revision := match_record.match_revision;
    result_checkpoint_sequence := match_record.checkpoint_sequence;
    result_phase := match_record.phase;

    if match_record.assigned_physical_host_id is distinct from p_physical_host_id
       or match_record.assigned_instance_id is distinct from p_instance_id
       or match_record.assigned_instance_epoch is distinct from p_instance_epoch then
        result_outcome := 'match_fenced';
        return next;
        return;
    end if;

    select command.sequence, command.input_sequence, command.player_id,
           command.request_hash, command.accepted_match_revision,
           command.accepted_snapshot_hash, command.target_tick,
           command.client_observed_tick
      into command_record
      from public.trnm_online_commands command
     where command.match_id = p_match_id
       and command.command_id = p_command_id;
    if found then
        select member.next_input_sequence
          into result_durable_member_input_sequence
          from public.trnm_online_match_members member
         where member.match_id = p_match_id
           and member.player_id = command_record.player_id;
        if command_record.player_id is distinct from p_player_id
           or command_record.request_hash is distinct from p_request_hash then
            result_outcome := 'command_conflict';
            return next;
            return;
        end if;
        if match_record.phase <> 'running'
           or result_durable_member_input_sequence is null
           or match_record.next_sequence < command_record.sequence + 1
           or result_durable_member_input_sequence <
               command_record.input_sequence + 1 then
            result_outcome := 'duplicate_publication_pending';
            return next;
            return;
        end if;
        result_outcome := 'duplicate';
        result_sequence := command_record.sequence;
        result_input_sequence := command_record.input_sequence;
        result_player_id := command_record.player_id;
        result_request_hash := command_record.request_hash;
        result_accepted_match_revision :=
            command_record.accepted_match_revision;
        result_accepted_snapshot_hash :=
            command_record.accepted_snapshot_hash;
        result_target_tick := command_record.target_tick;
        result_client_observed_tick := command_record.client_observed_tick;
        return next;
        return;
    end if;

    if match_record.phase <> 'running' then
        result_outcome := 'match_not_running';
        return next;
        return;
    end if;
    if match_record.terminal_stage_snapshot_hash is not null then
        result_outcome := 'terminal_stage_fenced';
        return next;
        return;
    end if;
    if match_record.next_sequence <> p_base_next_sequence
       or match_record.match_revision <> p_base_match_revision
       or p_accepted_match_revision <> p_base_match_revision + 1 then
        result_outcome := 'match_cursor_fenced';
        return next;
        return;
    end if;

    select member.next_input_sequence
      into member_record
      from public.trnm_online_match_members member
     where member.match_id = p_match_id
       and member.player_id = p_player_id
     for update;
    if not found then
        result_outcome := 'member_not_found';
        return next;
        return;
    end if;
    result_durable_member_input_sequence := member_record.next_input_sequence;
    if member_record.next_input_sequence <> p_input_sequence then
        result_outcome := 'member_cursor_fenced';
        return next;
        return;
    end if;

    insert into public.trnm_online_commands (
        match_id, sequence, command_id, player_id, input_sequence,
        request_hash, target_tick, client_observed_tick, order_json,
        accepted_snapshot_hash, accepted_match_revision,
        post_simulation_json
    ) values (
        p_match_id, p_base_next_sequence, p_command_id, p_player_id,
        p_input_sequence, p_request_hash, p_target_tick,
        p_client_observed_tick, p_order_json, p_accepted_snapshot_hash,
        p_accepted_match_revision, p_post_simulation_json
    );

    update public.trnm_online_match_members member
       set next_input_sequence = p_input_sequence + 1,
           last_seen_at = pg_catalog.now()
     where member.match_id = p_match_id
       and member.player_id = p_player_id
       and member.next_input_sequence = p_input_sequence;
    get diagnostics affected_rows = row_count;
    if affected_rows <> 1 then
        raise exception using
            errcode = 'P0001',
            message = 'TRNM command member cursor changed inside locked commit';
    end if;

    update public.trnm_online_matches match
       set next_sequence = p_base_next_sequence + 1,
           match_revision = p_accepted_match_revision,
           updated_at = pg_catalog.now()
     where match.match_id = p_match_id
       and match.phase = 'running'
       and match.assigned_instance_id = p_instance_id
       and match.assigned_instance_epoch = p_instance_epoch
       and match.assigned_physical_host_id = p_physical_host_id
       and match.terminal_stage_snapshot_hash is null
       and match.next_sequence = p_base_next_sequence
       and match.match_revision = p_base_match_revision;
    get diagnostics affected_rows = row_count;
    if affected_rows <> 1 then
        raise exception using
            errcode = 'P0001',
            message = 'TRNM command match cursor changed inside locked commit';
    end if;

    result_outcome := 'inserted';
    result_sequence := p_base_next_sequence;
    result_input_sequence := p_input_sequence;
    result_player_id := p_player_id;
    result_request_hash := p_request_hash;
    result_accepted_match_revision := p_accepted_match_revision;
    result_accepted_snapshot_hash := p_accepted_snapshot_hash;
    result_target_tick := p_target_tick;
    result_client_observed_tick := p_client_observed_tick;
    result_durable_next_sequence := p_base_next_sequence + 1;
    result_durable_match_revision := p_accepted_match_revision;
    result_durable_member_input_sequence := p_input_sequence + 1;
    return next;
end
$function$;

comment on function public.trnm_online_commit_actor_command_v1(
    uuid, text, text, bigint, text, bigint, bigint, jsonb, text, bigint,
    jsonb, bigint, bigint, text, bigint, text, uuid, text, integer,
    timestamptz, text, bigint, timestamptz, bigint, bigint
) is
'Atomic one-round-trip Authority v3 running-command commit with exact host/fleet fencing.';
