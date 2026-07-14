-- Stage-1 realtime input: clients sequence their own inputs while the server
-- assigns the authoritative total order kept in the legacy `sequence` column.

alter table trnm_online_match_members
    add column if not exists next_input_sequence bigint;

alter table trnm_online_commands
    add column if not exists input_sequence bigint;
alter table trnm_online_commands
    add column if not exists client_observed_tick bigint;

with ranked as (
    select match_id, sequence,
           row_number() over (
               partition by match_id, player_id order by sequence
           ) - 1 as input_sequence
    from trnm_online_commands
)
update trnm_online_commands command
set input_sequence = ranked.input_sequence
from ranked
where command.match_id = ranked.match_id
  and command.sequence = ranked.sequence
  and command.input_sequence is null;

update trnm_online_match_members member
set next_input_sequence = (
    select count(*)
    from trnm_online_commands command
    where command.match_id = member.match_id
      and command.player_id = member.player_id
)
where member.next_input_sequence is null;

alter table trnm_online_match_members
    alter column next_input_sequence set default 0;
alter table trnm_online_match_members
    alter column next_input_sequence set not null;
alter table trnm_online_match_members
    drop constraint if exists trnm_online_match_members_next_input_sequence_check;
alter table trnm_online_match_members
    add constraint trnm_online_match_members_next_input_sequence_check
    check (next_input_sequence >= 0);

-- Keep the event column nullable during the V2/V3 rolling-compatibility
-- window. A legacy writer omits it; the compatibility trigger below assigns
-- and advances the same per-player cursor atomically. Contracting to NOT NULL
-- is a separate fleet-wide migration after V2 rollback support is retired.
alter table trnm_online_commands
    alter column input_sequence drop not null;
alter table trnm_online_commands
    drop constraint if exists trnm_online_commands_input_sequence_check;
alter table trnm_online_commands
    add constraint trnm_online_commands_input_sequence_check
    check (input_sequence >= 0);
alter table trnm_online_commands
    drop constraint if exists trnm_online_commands_client_observed_tick_check;
alter table trnm_online_commands
    add constraint trnm_online_commands_client_observed_tick_check
    check (client_observed_tick is null or client_observed_tick >= 0);

do $$
begin
    if exists (
        select 1 from pg_indexes
        where schemaname = current_schema()
          and indexname = 'idx_trnm_online_player_input_sequence'
          and indexdef not ilike '% where %'
    ) then
        execute 'drop index idx_trnm_online_player_input_sequence';
    end if;
end
$$;

create unique index if not exists idx_trnm_online_player_input_sequence
    on trnm_online_commands(match_id, player_id, input_sequence)
    where input_sequence is not null;

create or replace function trnm_online_assign_legacy_input_sequence()
returns trigger
language plpgsql
as $$
declare
    assigned_sequence bigint;
begin
    if new.input_sequence is not null then
        return new;
    end if;
    select next_input_sequence into assigned_sequence
    from trnm_online_match_members
    where match_id = new.match_id and player_id = new.player_id
    for update;
    if assigned_sequence is null then
        raise exception 'legacy command member cursor is unavailable';
    end if;
    new.input_sequence := assigned_sequence;
    update trnm_online_match_members
    set next_input_sequence = assigned_sequence + 1, last_seen_at = now()
    where match_id = new.match_id and player_id = new.player_id
      and next_input_sequence = assigned_sequence;
    if not found then
        raise exception 'legacy command member cursor was fenced';
    end if;
    return new;
end
$$;

do $$
begin
    if not exists (
        select 1 from pg_trigger
        where tgname = 'trg_trnm_online_assign_legacy_input_sequence'
          and tgrelid = 'trnm_online_commands'::regclass
          and not tgisinternal
    ) then
        create trigger trg_trnm_online_assign_legacy_input_sequence
        before insert on trnm_online_commands
        for each row execute function trnm_online_assign_legacy_input_sequence();
    end if;
end
$$;
