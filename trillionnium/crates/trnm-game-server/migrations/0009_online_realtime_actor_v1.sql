-- Stage-0 realtime authority: command events are durable immediately while
-- full simulation state is checkpointed periodically by an in-memory actor.

alter table trnm_online_matches
    add column if not exists checkpoint_sequence bigint;

-- Existing rows already persisted their complete simulation on every command,
-- so their current next_sequence is the correct recovery boundary.
update trnm_online_matches
set checkpoint_sequence = next_sequence
where checkpoint_sequence is null;

alter table trnm_online_matches
    alter column checkpoint_sequence set default 0;
alter table trnm_online_matches
    alter column checkpoint_sequence set not null;
alter table trnm_online_matches
    drop constraint if exists trnm_online_matches_checkpoint_sequence_check;
alter table trnm_online_matches
    add constraint trnm_online_matches_checkpoint_sequence_check
    check (checkpoint_sequence >= 0 and checkpoint_sequence <= next_sequence);

alter table trnm_online_commands
    add column if not exists post_simulation_json jsonb;

create index if not exists idx_trnm_online_command_recovery
    on trnm_online_commands(match_id, sequence desc)
    where post_simulation_json is not null;
