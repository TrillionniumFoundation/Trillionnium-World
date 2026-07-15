create table if not exists trnm_online_physical_host_authorities (
    physical_host_id text primary key check (btrim(physical_host_id) <> ''),
    owner_nonce uuid not null,
    application_name text not null check (btrim(application_name) <> ''),
    backend_pid integer not null check (backend_pid > 0),
    backend_started_at timestamptz not null,
    database_system_identifier text not null check (
        database_system_identifier ~ '^[1-9][0-9]*$'
    ),
    database_timeline_id bigint not null check (database_timeline_id > 0),
    database_postmaster_started_at timestamptz not null,
    leader_lock_key bigint not null,
    barrier_lock_key bigint not null,
    claimed_at timestamptz not null default now()
);

alter table trnm_online_matches
    add column if not exists terminal_stage_simulation_json jsonb;
alter table trnm_online_matches
    add column if not exists terminal_stage_result_json jsonb;
alter table trnm_online_matches
    add column if not exists terminal_stage_result_hash text;
alter table trnm_online_matches
    add column if not exists terminal_stage_snapshot_hash text;
alter table trnm_online_matches
    add column if not exists terminal_stage_authoritative_tick bigint;
alter table trnm_online_matches
    add column if not exists terminal_stage_next_sequence bigint;
alter table trnm_online_matches
    add column if not exists terminal_stage_match_revision bigint;
alter table trnm_online_matches
    add column if not exists terminal_staged_at timestamptz;
alter table trnm_online_matches
    add column if not exists terminal_publication_actor_generation uuid;
alter table trnm_online_matches
    add column if not exists terminal_publication_state text;

alter table trnm_online_terminal_publication_acks
    add column if not exists instance_id text;
alter table trnm_online_terminal_publication_acks
    add column if not exists physical_host_id text;
alter table trnm_online_terminal_publication_acks
    add column if not exists local_tombstone_state text;

-- Existing V11 markers predate the cold-seal contract. They are eligible for
-- the one-time, exact-DB bootstrap path; new markers default to hot_pending
-- and may never be reconstructed from PostgreSQL alone.
update trnm_online_terminal_publication_acks
set local_tombstone_state = 'legacy_bootstrap_pending'
where local_tombstone_state is null;

alter table trnm_online_terminal_publication_acks
    alter column local_tombstone_state set default 'hot_pending';
alter table trnm_online_terminal_publication_acks
    alter column local_tombstone_state set not null;
alter table trnm_online_terminal_publication_acks
    drop constraint if exists trnm_online_terminal_publication_acks_local_tombstone_state_check;
alter table trnm_online_terminal_publication_acks
    add constraint trnm_online_terminal_publication_acks_local_tombstone_state_check check (
        local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending', 'sealed')
    );

update trnm_online_terminal_publication_acks a
set instance_id = m.assigned_instance_id,
    physical_host_id = m.assigned_physical_host_id
from trnm_online_matches m
where m.match_id = a.match_id
  and (a.instance_id is null or a.physical_host_id is null)
  and m.assigned_instance_id is not null
  and m.assigned_physical_host_id is not null
  and a.actor_epoch = m.assigned_instance_epoch;

do $$
begin
    if exists (
        select 1 from trnm_online_terminal_publication_acks
        where instance_id is null or physical_host_id is null
    ) then
        raise exception 'terminal publication ACK ownership cannot be verified for V12 backfill';
    end if;
end
$$;

alter table trnm_online_terminal_publication_acks
    alter column instance_id set not null;
alter table trnm_online_terminal_publication_acks
    alter column physical_host_id set not null;

update trnm_online_matches m
set terminal_publication_actor_generation = a.actor_generation
from trnm_online_terminal_publication_acks a
where a.match_id = m.match_id
  and m.phase = 'complete'
  and m.terminal_publication_actor_generation is null
  and a.instance_id = m.assigned_instance_id
  and a.actor_epoch = m.assigned_instance_epoch
  and a.physical_host_id = m.assigned_physical_host_id;

-- V11 allowed settlement to advance after publication without advancing the
-- marker. Upgrade only the already-proven, full-tuple ACK; never synthesize a
-- marker for legacy terminal rows that do not have one.
update trnm_online_terminal_publication_acks a
set published_settlement_state = 'settled'
from trnm_online_matches m
where m.match_id = a.match_id
  and m.phase = 'complete'
  and m.settlement_state = 'settled'
  and a.published_settlement_state = 'pending'
  and m.terminal_publication_actor_generation is not null
  and a.actor_generation = m.terminal_publication_actor_generation
  and a.instance_id = m.assigned_instance_id
  and a.actor_epoch = m.assigned_instance_epoch
  and a.physical_host_id = m.assigned_physical_host_id
  and a.authoritative_tick = m.authoritative_tick
  and a.next_sequence = m.next_sequence
  and a.match_revision = m.match_revision
  and a.next_input_sequences = coalesce(
      (
          select jsonb_object_agg(
              mm.player_id,
              to_jsonb(mm.next_input_sequence)
              order by mm.player_id
          )
          from trnm_online_match_members mm
          where mm.match_id = m.match_id
      ),
      '{}'::jsonb
  )
  and a.snapshot_hash = m.snapshot_hash
  and a.phase = 'complete'
  and a.result_hash = m.result_hash;

update trnm_online_matches m
set terminal_publication_state = case
    when m.phase <> 'complete' then 'pending'
    when exists (
        select 1
        from trnm_online_terminal_publication_acks a
        where a.match_id = m.match_id
          and m.terminal_publication_actor_generation is not null
          and a.actor_generation = m.terminal_publication_actor_generation
          and a.instance_id = m.assigned_instance_id
          and a.actor_epoch = m.assigned_instance_epoch
          and a.physical_host_id = m.assigned_physical_host_id
          and a.authoritative_tick = m.authoritative_tick
          and a.next_sequence = m.next_sequence
          and a.match_revision = m.match_revision
          and a.next_input_sequences = coalesce(
              (
                  select jsonb_object_agg(
                      mm.player_id,
                      to_jsonb(mm.next_input_sequence)
                      order by mm.player_id
                  )
                  from trnm_online_match_members mm
                  where mm.match_id = m.match_id
              ),
              '{}'::jsonb
          )
          and a.snapshot_hash = m.snapshot_hash
          and a.phase = 'complete'
          and a.result_hash = m.result_hash
          and a.published_settlement_state = m.settlement_state
    ) then 'acknowledged'
    else 'legacy_quarantined'
end
where m.terminal_publication_state is null;

alter table trnm_online_matches
    alter column terminal_publication_state set default 'pending';
alter table trnm_online_matches
    alter column terminal_publication_state set not null;
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_publication_state_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_publication_state_check check (
        terminal_publication_state in ('pending', 'acknowledged', 'legacy_quarantined')
        and (
            (
                phase = 'complete'
                and terminal_publication_state in ('acknowledged', 'legacy_quarantined')
            )
            or (
                phase <> 'complete'
                and terminal_publication_state = 'pending'
            )
        )
    );

alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_stage_hash_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_stage_hash_check check (
        terminal_stage_result_hash is null
        or length(terminal_stage_result_hash) = 64
    );
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_stage_snapshot_hash_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_stage_snapshot_hash_check check (
        terminal_stage_snapshot_hash is null
        or length(terminal_stage_snapshot_hash) = 64
    );
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_stage_tick_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_stage_tick_check check (
        terminal_stage_authoritative_tick is null
        or terminal_stage_authoritative_tick >= 0
    );
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_stage_sequence_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_stage_sequence_check check (
        terminal_stage_next_sequence is null
        or terminal_stage_next_sequence >= 0
    );
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_stage_revision_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_stage_revision_check check (
        terminal_stage_match_revision is null
        or terminal_stage_match_revision >= 0
    );
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_terminal_stage_all_or_none_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_terminal_stage_all_or_none_check check (
        (
            terminal_stage_simulation_json is null
            and terminal_stage_result_json is null
            and terminal_stage_result_hash is null
            and terminal_stage_snapshot_hash is null
            and terminal_stage_authoritative_tick is null
            and terminal_stage_next_sequence is null
            and terminal_stage_match_revision is null
            and terminal_staged_at is null
        )
        or (
            phase = 'running'
            and terminal_stage_simulation_json is not null
            and terminal_stage_result_json is not null
            and terminal_stage_result_hash is not null
            and terminal_stage_snapshot_hash is not null
            and terminal_stage_authoritative_tick is not null
            and terminal_stage_next_sequence is not null
            and terminal_stage_match_revision is not null
            and terminal_staged_at is not null
        )
    );

create index if not exists trnm_online_terminal_stage_pending_idx
    on trnm_online_matches (assigned_physical_host_id, match_id)
    where phase = 'running' and terminal_stage_snapshot_hash is not null;

create index if not exists trnm_online_terminal_publication_host_state_idx
    on trnm_online_matches (
        assigned_physical_host_id,
        terminal_publication_state,
        match_id
    ) include (
        assigned_instance_id,
        assigned_instance_epoch,
        terminal_publication_actor_generation,
        authoritative_tick,
        next_sequence,
        match_revision,
        checkpoint_sequence,
        snapshot_hash,
        result_hash,
        settlement_state
    ) where phase = 'complete';

create index if not exists trnm_online_terminal_publication_state_idx
    on trnm_online_matches (terminal_publication_state, match_id)
    where phase = 'complete';

create index if not exists trnm_online_complete_publication_lookup_idx
    on trnm_online_matches (
        assigned_instance_id,
        assigned_instance_epoch,
        assigned_physical_host_id,
        match_id
    ) include (
        authoritative_tick,
        next_sequence,
        match_revision,
        checkpoint_sequence,
        snapshot_hash,
        result_hash,
        settlement_state,
        terminal_publication_actor_generation
    ) where phase = 'complete';

create index if not exists trnm_online_terminal_ack_ownership_idx
    on trnm_online_terminal_publication_acks (
        instance_id,
        actor_epoch,
        physical_host_id,
        match_id,
        actor_generation
    );

create index if not exists trnm_online_terminal_ack_local_seal_idx
    on trnm_online_terminal_publication_acks (
        physical_host_id,
        local_tombstone_state,
        match_id
    ) include (acknowledged_at);

create index if not exists trnm_online_rating_events_publication_gate_idx
    on trnm_online_rating_events (season_id, player_id, match_id)
    include (integrity_state, result_hash);

create index if not exists trnm_online_progression_events_publication_gate_idx
    on trnm_online_progression_events (campaign_id, match_id)
    include (result_hash);
