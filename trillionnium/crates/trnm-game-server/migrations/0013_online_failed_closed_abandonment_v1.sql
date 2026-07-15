create table if not exists trnm_online_local_cold_witness_summaries (
    physical_host_id text primary key check (btrim(physical_host_id) <> ''),
    terminal_total_count bigint not null default 0 check (terminal_total_count >= 0),
    terminal_sealed_count bigint not null default 0 check (terminal_sealed_count >= 0),
    abandonment_total_count bigint not null default 0 check (abandonment_total_count >= 0),
    abandonment_sealed_count bigint not null default 0 check (abandonment_sealed_count >= 0),
    updated_at timestamptz not null default now(),
    check (terminal_sealed_count <= terminal_total_count),
    check (abandonment_sealed_count <= abandonment_total_count)
);

create table if not exists trnm_online_failed_closed_abandonment_markers (
    match_id uuid primary key references trnm_online_matches(match_id) on delete restrict,
    journal_owner_id uuid not null check (
        journal_owner_id <> '00000000-0000-0000-0000-000000000000'::uuid
    ),
    actor_generation uuid not null check (
        actor_generation <> '00000000-0000-0000-0000-000000000000'::uuid
    ),
    instance_id text not null check (btrim(instance_id) <> ''),
    actor_epoch bigint not null check (actor_epoch > 0),
    physical_host_id text not null check (btrim(physical_host_id) <> ''),
    authoritative_tick bigint not null check (authoritative_tick >= 0),
    next_sequence bigint not null check (next_sequence >= 0),
    match_revision bigint not null check (match_revision >= 0),
    next_input_sequences jsonb not null check (
        jsonb_typeof(next_input_sequences) = 'object'
    ),
    snapshot_hash text not null check (snapshot_hash ~ '^[0-9a-f]{64}$'),
    failure_reason text not null check (
        btrim(failure_reason) <> ''
        and octet_length(failure_reason) <= 1024
        and failure_reason !~ '[[:cntrl:]]'
    ),
    abandoned_at timestamptz not null default clock_timestamp(),
    local_tombstone_state text not null default 'hot_pending' check (
        local_tombstone_state in ('hot_pending', 'sealed')
    )
);

alter table trnm_online_failed_closed_abandonment_markers
    drop constraint if exists trnm_online_failed_closed_abandonment_reason_check;
alter table trnm_online_failed_closed_abandonment_markers
    add constraint trnm_online_failed_closed_abandonment_reason_check check (
        btrim(failure_reason) <> ''
        and octet_length(failure_reason) <= 1024
        and failure_reason !~ '[[:cntrl:]]'
    );

-- V11 originally used ON DELETE CASCADE. Cold evidence is authority history,
-- so deleting a match must never erase an older non-latest witness while the
-- O(1) summary and latest sentinel remain green.
alter table trnm_online_terminal_publication_acks
    drop constraint if exists trnm_online_terminal_publication_acks_match_id_fkey;
alter table trnm_online_terminal_publication_acks
    add constraint trnm_online_terminal_publication_acks_match_id_fkey
    foreign key (match_id) references trnm_online_matches(match_id) on delete restrict;

create index if not exists trnm_online_failed_closed_abandonment_host_state_idx
    on trnm_online_failed_closed_abandonment_markers (
        physical_host_id,
        local_tombstone_state,
        match_id
    ) include (abandoned_at);

create or replace function trnm_online_guard_abandonment_marker_update()
returns trigger
language plpgsql
as $$
begin
    if new.match_id <> old.match_id
       or new.journal_owner_id <> old.journal_owner_id
       or new.actor_generation <> old.actor_generation
       or new.instance_id <> old.instance_id
       or new.actor_epoch <> old.actor_epoch
       or new.physical_host_id <> old.physical_host_id
       or new.authoritative_tick <> old.authoritative_tick
       or new.next_sequence <> old.next_sequence
       or new.match_revision <> old.match_revision
       or new.next_input_sequences <> old.next_input_sequences
       or new.snapshot_hash <> old.snapshot_hash
       or new.failure_reason <> old.failure_reason
       or new.abandoned_at <> old.abandoned_at then
        raise exception 'failed-closed abandonment marker authority is immutable';
    end if;
    if old.local_tombstone_state = 'sealed'
       and new.local_tombstone_state <> 'sealed' then
        raise exception 'sealed abandonment marker cannot regress';
    end if;
    return new;
end
$$;

drop trigger if exists trnm_online_guard_abandonment_marker_update
    on trnm_online_failed_closed_abandonment_markers;
create trigger trnm_online_guard_abandonment_marker_update
before update on trnm_online_failed_closed_abandonment_markers
for each row execute function trnm_online_guard_abandonment_marker_update();

create or replace function trnm_online_guard_terminal_ack_update()
returns trigger
language plpgsql
as $$
begin
    if new.match_id is distinct from old.match_id
       or new.actor_generation is distinct from old.actor_generation
       or new.actor_epoch is distinct from old.actor_epoch
       or new.authoritative_tick is distinct from old.authoritative_tick
       or new.next_sequence is distinct from old.next_sequence
       or new.match_revision is distinct from old.match_revision
       or new.next_input_sequences is distinct from old.next_input_sequences
       or new.snapshot_hash is distinct from old.snapshot_hash
       or new.phase is distinct from old.phase
       or new.result_hash is distinct from old.result_hash
       or new.acknowledged_at is distinct from old.acknowledged_at
       or new.instance_id is distinct from old.instance_id
       or new.physical_host_id is distinct from old.physical_host_id then
        raise exception 'terminal publication ACK authority is immutable';
    end if;
    if new.published_settlement_state is distinct from old.published_settlement_state
       and not (
           old.published_settlement_state = 'pending'
           and new.published_settlement_state = 'settled'
       ) then
        raise exception 'terminal publication ACK settlement state cannot regress or change';
    end if;
    if new.local_tombstone_state is distinct from old.local_tombstone_state
       and not (
           old.local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending')
           and new.local_tombstone_state = 'sealed'
       ) then
        raise exception 'terminal publication ACK cold-seal state cannot regress or change';
    end if;
    return new;
end
$$;

drop trigger if exists trnm_online_guard_terminal_ack_update
    on trnm_online_terminal_publication_acks;
create trigger trnm_online_guard_terminal_ack_update
before update on trnm_online_terminal_publication_acks
for each row execute function trnm_online_guard_terminal_ack_update();

create or replace function trnm_online_forbid_cold_witness_delete()
returns trigger
language plpgsql
as $$
begin
    raise exception 'durable cold witness history cannot be deleted';
end
$$;

drop trigger if exists trnm_online_forbid_terminal_ack_delete
    on trnm_online_terminal_publication_acks;
create trigger trnm_online_forbid_terminal_ack_delete
before delete on trnm_online_terminal_publication_acks
for each row execute function trnm_online_forbid_cold_witness_delete();

drop trigger if exists trnm_online_forbid_abandonment_delete
    on trnm_online_failed_closed_abandonment_markers;
create trigger trnm_online_forbid_abandonment_delete
before delete on trnm_online_failed_closed_abandonment_markers
for each row execute function trnm_online_forbid_cold_witness_delete();

-- A post-V13 running match may enter failed_closed only in the same
-- PostgreSQL transaction that creates its exact hot_pending marker. The
-- deferred check allows the transition UPDATE to run before the marker
-- INSERT, while still rejecting every legacy naked UPDATE at commit.
create or replace function trnm_online_guard_abandonment_marker_insert()
returns trigger
language plpgsql
as $$
begin
    if new.local_tombstone_state <> 'hot_pending' or not exists (
        select 1
          from trnm_online_matches match_row
         where match_row.match_id = new.match_id
           and match_row.phase = 'failed_closed'
           and match_row.settlement_state = 'failed_closed'
           and match_row.terminal_publication_state = 'pending'
           and match_row.checkpoint_sequence = match_row.next_sequence
           and match_row.simulation_json is not null
           and match_row.terminal_stage_simulation_json is null
           and match_row.terminal_stage_result_json is null
           and match_row.terminal_stage_result_hash is null
           and match_row.terminal_stage_snapshot_hash is null
           and match_row.terminal_stage_authoritative_tick is null
           and match_row.terminal_stage_next_sequence is null
           and match_row.terminal_stage_match_revision is null
           and match_row.terminal_staged_at is null
           and match_row.result_json is null
           and match_row.result_hash is null
           and match_row.terminal_publication_actor_generation is null
           and new.failure_reason = match_row.failure_reason
           and new.instance_id = match_row.assigned_instance_id
           and new.actor_epoch = match_row.assigned_instance_epoch
           and new.physical_host_id = match_row.assigned_physical_host_id
           and new.authoritative_tick = match_row.authoritative_tick
           and new.next_sequence = match_row.next_sequence
           and new.match_revision = match_row.match_revision
           and new.snapshot_hash = match_row.snapshot_hash
           and new.next_input_sequences = coalesce(
               (select jsonb_object_agg(
                   member.player_id,
                   to_jsonb(member.next_input_sequence)
                   order by member.player_id
                )
                  from trnm_online_match_members member
                 where member.match_id = match_row.match_id),
               '{}'::jsonb
           )
           and not exists (
               select 1 from trnm_online_terminal_publication_acks terminal
                where terminal.match_id = match_row.match_id
           )
    ) then
        raise exception
            'abandonment marker insert requires exact failed_closed durable authority';
    end if;
    return new;
end
$$;

drop trigger if exists trnm_online_guard_abandonment_marker_insert
    on trnm_online_failed_closed_abandonment_markers;
create trigger trnm_online_guard_abandonment_marker_insert
before insert on trnm_online_failed_closed_abandonment_markers
for each row execute function trnm_online_guard_abandonment_marker_insert();

-- Phase transitions are a small explicit state machine. In particular, a
-- running row cannot hop through waiting/complete and later enter
-- failed_closed without the deferred atomic-marker check below.
-- Once a marker exists, every match field attested by that marker (including
-- the exact simulation payload behind its hash) is immutable.
create or replace function trnm_online_guard_marked_abandonment_match_update()
returns trigger
language plpgsql
as $$
begin
    if new.phase <> old.phase and not (
        (old.phase = 'waiting' and new.phase in ('running', 'failed_closed'))
        or (old.phase = 'running' and new.phase in ('complete', 'failed_closed'))
    ) then
        raise exception 'online match phase transition is not monotonic';
    end if;
    if exists (
        select 1
          from trnm_online_failed_closed_abandonment_markers marker
         where marker.match_id = old.match_id
    ) and (
        new.match_id is distinct from old.match_id
        or new.phase is distinct from old.phase
        or new.settlement_state is distinct from old.settlement_state
        or new.terminal_publication_state is distinct from old.terminal_publication_state
        or new.simulation_json is distinct from old.simulation_json
        or new.snapshot_hash is distinct from old.snapshot_hash
        or new.authoritative_tick is distinct from old.authoritative_tick
        or new.next_sequence is distinct from old.next_sequence
        or new.checkpoint_sequence is distinct from old.checkpoint_sequence
        or new.match_revision is distinct from old.match_revision
        or new.assigned_instance_id is distinct from old.assigned_instance_id
        or new.assigned_instance_epoch is distinct from old.assigned_instance_epoch
        or new.assigned_physical_host_id is distinct from old.assigned_physical_host_id
        or new.result_json is distinct from old.result_json
        or new.result_hash is distinct from old.result_hash
        or new.failure_reason is distinct from old.failure_reason
        or new.terminal_publication_actor_generation is distinct from
           old.terminal_publication_actor_generation
        or new.terminal_stage_simulation_json is distinct from
           old.terminal_stage_simulation_json
        or new.terminal_stage_result_json is distinct from old.terminal_stage_result_json
        or new.terminal_stage_result_hash is distinct from old.terminal_stage_result_hash
        or new.terminal_stage_snapshot_hash is distinct from old.terminal_stage_snapshot_hash
        or new.terminal_stage_authoritative_tick is distinct from
           old.terminal_stage_authoritative_tick
        or new.terminal_stage_next_sequence is distinct from
           old.terminal_stage_next_sequence
        or new.terminal_stage_match_revision is distinct from
           old.terminal_stage_match_revision
        or new.terminal_staged_at is distinct from old.terminal_staged_at
    ) then
        raise exception 'marked failed-closed match authority is immutable';
    end if;
    return new;
end
$$;

drop trigger if exists trnm_online_guard_marked_abandonment_match_update
    on trnm_online_matches;
create trigger trnm_online_guard_marked_abandonment_match_update
before update on trnm_online_matches
for each row
when (old.phase = 'failed_closed' or new.phase is distinct from old.phase)
execute function trnm_online_guard_marked_abandonment_match_update();

create or replace function trnm_online_require_atomic_running_abandonment()
returns trigger
language plpgsql
as $$
begin
    if old.phase = 'running' and new.phase = 'failed_closed' and not exists (
        select 1
          from trnm_online_failed_closed_abandonment_markers marker
         where marker.match_id = new.match_id
           and marker.local_tombstone_state = 'hot_pending'
           and new.settlement_state = 'failed_closed'
           and new.terminal_publication_state = 'pending'
           and new.checkpoint_sequence = new.next_sequence
           and new.terminal_stage_simulation_json is null
           and new.terminal_stage_result_json is null
           and new.terminal_stage_result_hash is null
           and new.terminal_stage_snapshot_hash is null
           and new.terminal_stage_authoritative_tick is null
           and new.terminal_stage_next_sequence is null
           and new.terminal_stage_match_revision is null
           and new.terminal_staged_at is null
           and new.result_json is null
           and new.result_hash is null
           and new.terminal_publication_actor_generation is null
           and marker.failure_reason = new.failure_reason
           and marker.instance_id = new.assigned_instance_id
           and marker.actor_epoch = new.assigned_instance_epoch
           and marker.physical_host_id = new.assigned_physical_host_id
           and marker.authoritative_tick = new.authoritative_tick
           and marker.next_sequence = new.next_sequence
           and marker.match_revision = new.match_revision
           and marker.snapshot_hash = new.snapshot_hash
           and marker.next_input_sequences = coalesce(
               (select jsonb_object_agg(
                   member.player_id,
                   to_jsonb(member.next_input_sequence)
                   order by member.player_id
                )
                  from trnm_online_match_members member
                 where member.match_id = new.match_id),
               '{}'::jsonb
           )
           and not exists (
               select 1 from trnm_online_terminal_publication_acks terminal
                where terminal.match_id = new.match_id
           )
    ) then
        raise exception
            'running failed_closed transition requires an exact same-transaction abandonment marker';
    end if;
    return new;
end
$$;

drop trigger if exists trnm_online_require_atomic_running_abandonment
    on trnm_online_matches;
create constraint trigger trnm_online_require_atomic_running_abandonment
after update on trnm_online_matches
deferrable initially deferred
for each row
when (old.phase = 'running' and new.phase = 'failed_closed')
execute function trnm_online_require_atomic_running_abandonment();

create or replace function trnm_online_maintain_cold_witness_summary()
returns trigger
language plpgsql
as $$
declare
    witness_kind text := tg_argv[0];
begin
    if tg_op = 'INSERT' then
        if witness_kind = 'terminal' then
            insert into trnm_online_local_cold_witness_summaries (
                physical_host_id,
                terminal_total_count,
                terminal_sealed_count
            ) values (
                new.physical_host_id,
                1,
                case when new.local_tombstone_state = 'sealed' then 1 else 0 end
            )
            on conflict (physical_host_id) do update
            set terminal_total_count =
                    trnm_online_local_cold_witness_summaries.terminal_total_count + 1,
                terminal_sealed_count =
                    trnm_online_local_cold_witness_summaries.terminal_sealed_count
                    + case when new.local_tombstone_state = 'sealed' then 1 else 0 end,
                updated_at = now();
        elsif witness_kind = 'abandonment' then
            insert into trnm_online_local_cold_witness_summaries (
                physical_host_id,
                abandonment_total_count,
                abandonment_sealed_count
            ) values (
                new.physical_host_id,
                1,
                case when new.local_tombstone_state = 'sealed' then 1 else 0 end
            )
            on conflict (physical_host_id) do update
            set abandonment_total_count =
                    trnm_online_local_cold_witness_summaries.abandonment_total_count + 1,
                abandonment_sealed_count =
                    trnm_online_local_cold_witness_summaries.abandonment_sealed_count
                    + case when new.local_tombstone_state = 'sealed' then 1 else 0 end,
                updated_at = now();
        else
            raise exception 'unknown cold witness summary kind %', witness_kind;
        end if;
        return new;
    end if;

    if old.physical_host_id <> new.physical_host_id then
        raise exception 'cold witness physical host identity is immutable';
    end if;
    if old.local_tombstone_state = 'sealed'
       and new.local_tombstone_state <> 'sealed' then
        raise exception 'sealed cold witness state cannot regress';
    end if;
    if old.local_tombstone_state <> 'sealed'
       and new.local_tombstone_state = 'sealed' then
        if witness_kind = 'terminal' then
            update trnm_online_local_cold_witness_summaries
            set terminal_sealed_count = terminal_sealed_count + 1,
                updated_at = now()
            where physical_host_id = new.physical_host_id;
        elsif witness_kind = 'abandonment' then
            update trnm_online_local_cold_witness_summaries
            set abandonment_sealed_count = abandonment_sealed_count + 1,
                updated_at = now()
            where physical_host_id = new.physical_host_id;
        else
            raise exception 'unknown cold witness summary kind %', witness_kind;
        end if;
        if not found then
            raise exception 'cold witness summary row is missing for host %', new.physical_host_id;
        end if;
    end if;
    return new;
end
$$;

drop trigger if exists trnm_online_terminal_ack_cold_witness_summary
    on trnm_online_terminal_publication_acks;
create trigger trnm_online_terminal_ack_cold_witness_summary
after insert or update of physical_host_id, local_tombstone_state
on trnm_online_terminal_publication_acks
for each row execute function trnm_online_maintain_cold_witness_summary('terminal');

drop trigger if exists trnm_online_abandonment_cold_witness_summary
    on trnm_online_failed_closed_abandonment_markers;
create trigger trnm_online_abandonment_cold_witness_summary
after insert or update of physical_host_id, local_tombstone_state
on trnm_online_failed_closed_abandonment_markers
for each row execute function trnm_online_maintain_cold_witness_summary('abandonment');

-- The migration runs while the host-wide PostgreSQL barrier is exclusive, so
-- this one-time rebuild is an exact baseline for all subsequent trigger-owned
-- O(1) counters.
delete from trnm_online_local_cold_witness_summaries;
insert into trnm_online_local_cold_witness_summaries (
    physical_host_id,
    terminal_total_count,
    terminal_sealed_count,
    abandonment_total_count,
    abandonment_sealed_count,
    updated_at
)
select hosts.physical_host_id,
       coalesce(terminals.total_count, 0),
       coalesce(terminals.sealed_count, 0),
       coalesce(abandonments.total_count, 0),
       coalesce(abandonments.sealed_count, 0),
       now()
from (
    select physical_host_id from trnm_online_terminal_publication_acks
    union
    select physical_host_id from trnm_online_failed_closed_abandonment_markers
) hosts
left join (
    select physical_host_id,
           count(*)::bigint as total_count,
           count(*) filter (where local_tombstone_state = 'sealed')::bigint as sealed_count
    from trnm_online_terminal_publication_acks
    group by physical_host_id
) terminals using (physical_host_id)
left join (
    select physical_host_id,
           count(*)::bigint as total_count,
           count(*) filter (where local_tombstone_state = 'sealed')::bigint as sealed_count
    from trnm_online_failed_closed_abandonment_markers
    group by physical_host_id
) abandonments using (physical_host_id);
