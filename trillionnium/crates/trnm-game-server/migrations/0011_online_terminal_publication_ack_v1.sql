create table if not exists trnm_online_terminal_publication_acks (
    match_id uuid primary key references trnm_online_matches(match_id) on delete cascade,
    actor_generation uuid not null,
    actor_epoch bigint not null check (actor_epoch > 0),
    authoritative_tick bigint not null check (authoritative_tick >= 0),
    next_sequence bigint not null check (next_sequence >= 0),
    match_revision bigint not null check (match_revision >= 0),
    next_input_sequences jsonb not null,
    snapshot_hash text not null check (length(snapshot_hash) = 64),
    phase text not null default 'complete' check (phase = 'complete'),
    result_hash text not null check (length(result_hash) = 64),
    published_settlement_state text not null
        check (published_settlement_state in ('pending', 'settled')),
    acknowledged_at timestamptz not null default now()
);

alter table trnm_online_terminal_publication_acks
    add column if not exists phase text;
alter table trnm_online_terminal_publication_acks
    add column if not exists result_hash text;
alter table trnm_online_terminal_publication_acks
    add column if not exists published_settlement_state text;

update trnm_online_terminal_publication_acks a
set phase = 'complete',
    result_hash = m.result_hash,
    published_settlement_state = m.settlement_state
from trnm_online_matches m
where m.match_id = a.match_id
  and a.phase is null
  and m.phase = 'complete'
  and m.result_hash is not null
  and m.settlement_state in ('pending', 'settled');

alter table trnm_online_terminal_publication_acks
    alter column phase set default 'complete';
alter table trnm_online_terminal_publication_acks
    alter column phase set not null;
alter table trnm_online_terminal_publication_acks
    alter column result_hash set not null;
alter table trnm_online_terminal_publication_acks
    alter column published_settlement_state set not null;
alter table trnm_online_terminal_publication_acks
    drop constraint if exists trnm_online_terminal_publication_acks_phase_check;
alter table trnm_online_terminal_publication_acks
    add constraint trnm_online_terminal_publication_acks_phase_check check (phase = 'complete');
alter table trnm_online_terminal_publication_acks
    drop constraint if exists trnm_online_terminal_publication_acks_result_hash_check;
alter table trnm_online_terminal_publication_acks
    add constraint trnm_online_terminal_publication_acks_result_hash_check
    check (length(result_hash) = 64);
alter table trnm_online_terminal_publication_acks
    drop constraint if exists trnm_online_terminal_publication_acks_settlement_check;
alter table trnm_online_terminal_publication_acks
    add constraint trnm_online_terminal_publication_acks_settlement_check
    check (published_settlement_state in ('pending', 'settled'));

create index if not exists trnm_online_terminal_publication_acks_acknowledged_at_idx
    on trnm_online_terminal_publication_acks (acknowledged_at);
