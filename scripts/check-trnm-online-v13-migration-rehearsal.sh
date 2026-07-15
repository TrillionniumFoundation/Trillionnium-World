#!/usr/bin/bash
set -euo pipefail
umask 077

readonly ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
readonly CEX_HELPER="$ROOT_DIR/../CEX/scripts/_dev-helpers.sh"
readonly MIGRATION="$ROOT_DIR/trillionnium/crates/trnm-game-server/migrations/0013_online_failed_closed_abandonment_v1.sql"
readonly REHEARSAL_CONTRACT="trnm_online_v13_migration_rehearsal_v1"

[[ -f "$CEX_HELPER" && ! -L "$CEX_HELPER" ]] || {
  echo "missing canonical CEX helper: $CEX_HELPER" >&2
  exit 1
}
[[ -f "$MIGRATION" && ! -L "$MIGRATION" ]] || {
  echo "missing V13 migration: $MIGRATION" >&2
  exit 1
}

# shellcheck source=/dev/null
source "$CEX_HELPER"
cex_load_env
cex_require_cmd awk sed sha256sum

MIGRATION_SHA256="$(sha256sum -- "$MIGRATION" | awk '{print $1}')"
readonly MIGRATION_SHA256
[[ "$MIGRATION_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
  echo "could not fingerprint the V13 migration" >&2
  exit 1
}

# _dev-helpers.sh intentionally abstracts local psql versus docker exec. Keep
# errexit inside this subshell so a failing psql is never hidden by the
# helper's trailing return statement.
run_psql() (
  set -euo pipefail
  cex_psql_stdin "$@"
)

v13_persistent_state() {
  run_psql -X -At -v ON_ERROR_STOP=1 -c "
    select concat_ws('|',
        (to_regclass('public.trnm_online_failed_closed_abandonment_markers') is not null)::integer,
        (to_regclass('public.trnm_online_local_cold_witness_summaries') is not null)::integer,
        coalesce((
            select migration_name || ':' || checksum_sha256 || ':'
                   || extract(epoch from applied_at)::text
              from public.trnm_online_schema_migrations
             where migration_version = 13
        ), 'absent')
    )"
}

BEFORE_STATE="$(v13_persistent_state)"
readonly BEFORE_STATE

set +e
REHEARSAL_OUTPUT="$({
  printf '%s\n' \
    '\set ON_ERROR_STOP on' \
    'begin;' \
    "set local application_name = '$REHEARSAL_CONTRACT';" \
    "set local lock_timeout = '10s';" \
    "set local statement_timeout = '120s';" \
    'select pg_advisory_xact_lock(6076004941226200137);'
  sed -n '1,$p' "$MIGRATION"
  printf '%s\n' "
create temporary table trnm_v13_rehearsal_config (
    migration_checksum text not null,
    failure_reason text not null
) on commit drop;

insert into trnm_v13_rehearsal_config (migration_checksum, failure_reason)
values (:'migration_checksum', 'TRNM V13 rollback-only migration rehearsal');

insert into public.trnm_online_schema_migrations (
    migration_version,
    migration_name,
    checksum_sha256
)
select 13,
       '0013_online_failed_closed_abandonment_v1',
       migration_checksum
  from trnm_v13_rehearsal_config
on conflict (migration_version) do nothing;

do \$rehearsal\$
begin
    if not exists (
        select 1
          from public.trnm_online_schema_migrations ledger
          join trnm_v13_rehearsal_config config
            on config.migration_checksum = ledger.checksum_sha256
         where ledger.migration_version = 13
           and ledger.migration_name = '0013_online_failed_closed_abandonment_v1'
    ) then
        raise exception 'V13_REHEARSAL_LEDGER_MISMATCH';
    end if;
end
\$rehearsal\$;

create temporary table trnm_v13_rehearsal_candidate (
    match_id uuid primary key,
    journal_owner_id uuid not null,
    actor_generation uuid not null,
    instance_id text not null,
    actor_epoch bigint not null,
    physical_host_id text not null,
    authoritative_tick bigint not null,
    next_sequence bigint not null,
    match_revision bigint not null,
    next_input_sequences jsonb not null,
    snapshot_hash text not null,
    failure_reason text not null,
    baseline_terminal_total bigint not null,
    baseline_terminal_sealed bigint not null,
    baseline_abandonment_total bigint not null,
    baseline_abandonment_sealed bigint not null
) on commit drop;

with picked as materialized (
    select match_row.match_id
      from trnm_online_matches match_row
     where match_row.phase = 'failed_closed'
       and match_row.settlement_state = 'failed_closed'
       and match_row.simulation_json is not null
       and match_row.result_json is null
       and match_row.result_hash is null
       and match_row.snapshot_hash ~ '^[0-9a-f]{64}\$'
       and match_row.assigned_instance_id is not null
       and btrim(match_row.assigned_instance_id) <> ''
       and match_row.assigned_instance_epoch > 0
       and match_row.assigned_physical_host_id is not null
       and btrim(match_row.assigned_physical_host_id) <> ''
       and match_row.checkpoint_sequence = match_row.next_sequence
       and match_row.terminal_publication_state = 'pending'
       and match_row.terminal_stage_simulation_json is null
       and match_row.terminal_stage_result_json is null
       and match_row.terminal_stage_result_hash is null
       and match_row.terminal_stage_snapshot_hash is null
       and match_row.terminal_stage_authoritative_tick is null
       and match_row.terminal_stage_next_sequence is null
       and match_row.terminal_stage_match_revision is null
       and match_row.terminal_staged_at is null
       and match_row.terminal_publication_actor_generation is null
       and not exists (
           select 1
             from trnm_online_terminal_publication_acks terminal
            where terminal.match_id = match_row.match_id
       )
       and exists (
           select 1
             from trnm_online_match_members member
            where member.match_id = match_row.match_id
              and member.next_input_sequence is not null
       )
       and not exists (
           select 1
             from trnm_online_failed_closed_abandonment_markers abandonment
            where abandonment.match_id = match_row.match_id
       )
     order by match_row.updated_at, match_row.match_id
     for update of match_row skip locked
     limit 1
)
insert into trnm_v13_rehearsal_candidate (
    match_id,
    journal_owner_id,
    actor_generation,
    instance_id,
    actor_epoch,
    physical_host_id,
    authoritative_tick,
    next_sequence,
    match_revision,
    next_input_sequences,
    snapshot_hash,
    failure_reason,
    baseline_terminal_total,
    baseline_terminal_sealed,
    baseline_abandonment_total,
    baseline_abandonment_sealed
)
select match_row.match_id,
       '00000000-0000-4000-8000-000000001301'::uuid,
       '00000000-0000-4000-8000-000000001302'::uuid,
       match_row.assigned_instance_id,
       match_row.assigned_instance_epoch,
       match_row.assigned_physical_host_id,
       match_row.authoritative_tick,
       match_row.next_sequence,
       match_row.match_revision,
       coalesce((
           select jsonb_object_agg(
               member.player_id,
               to_jsonb(member.next_input_sequence)
               order by member.player_id
           )
             from trnm_online_match_members member
            where member.match_id = match_row.match_id
       ), '{}'::jsonb),
       match_row.snapshot_hash,
       config.failure_reason,
       coalesce(summary.terminal_total_count, 0),
       coalesce(summary.terminal_sealed_count, 0),
       coalesce(summary.abandonment_total_count, 0),
       coalesce(summary.abandonment_sealed_count, 0)
  from picked
  join trnm_online_matches match_row using (match_id)
 cross join trnm_v13_rehearsal_config config
  left join trnm_online_local_cold_witness_summaries summary
    on summary.physical_host_id = match_row.assigned_physical_host_id;

do \$rehearsal\$
begin
    if not exists (select 1 from trnm_v13_rehearsal_candidate) then
        raise exception using
            errcode = 'P0001',
            message = 'V13_REHEARSAL_BLOCKED: no exact failed_closed simulation row without a cold witness is available';
    end if;
end
\$rehearsal\$;

select match_id::text as rehearsal_match_id
  from trnm_v13_rehearsal_candidate
\gset

-- V13 intentionally makes failed_closed terminal. Temporarily disable only
-- that new monotonic-state trigger while constructing the rollback-only
-- running fixture from a real durable row; the ALTER and UPDATE are invisible
-- outside this uncommitted transaction.
alter table trnm_online_matches
    disable trigger trnm_online_guard_marked_abandonment_match_update;

update trnm_online_matches match_row
   set phase = 'running',
       settlement_state = 'not_ready',
       failure_reason = null
  from trnm_v13_rehearsal_candidate candidate
 where match_row.match_id = candidate.match_id;

alter table trnm_online_matches
    enable trigger trnm_online_guard_marked_abandonment_match_update;

do \$rehearsal\$
begin
    if not exists (
        select 1
          from pg_trigger
         where tgrelid = 'trnm_online_matches'::regclass
           and tgname = 'trnm_online_guard_marked_abandonment_match_update'
           and tgenabled = 'O'
    ) then
        raise exception 'V13_REHEARSAL_MATCH_GUARD_WAS_NOT_REENABLED';
    end if;
end
\$rehearsal\$;

-- A marker created before the phase transition must be rejected by the
-- marker-side exact-authority trigger.
do \$rehearsal\$
declare
    rejected boolean := false;
    failure text;
begin
    begin
        insert into trnm_online_failed_closed_abandonment_markers (
            match_id,
            journal_owner_id,
            actor_generation,
            instance_id,
            actor_epoch,
            physical_host_id,
            authoritative_tick,
            next_sequence,
            match_revision,
            next_input_sequences,
            snapshot_hash,
            failure_reason
        )
        select match_id,
               journal_owner_id,
               actor_generation,
               instance_id,
               actor_epoch,
               physical_host_id,
               authoritative_tick,
               next_sequence,
               match_revision,
               next_input_sequences,
               snapshot_hash,
               failure_reason
          from trnm_v13_rehearsal_candidate;
    exception when sqlstate 'P0001' then
        get stacked diagnostics failure = message_text;
        if position('abandonment marker insert requires exact' in failure) = 0 then
            raise;
        end if;
        rejected := true;
    end;
    if not rejected or exists (
        select 1
          from trnm_online_failed_closed_abandonment_markers marker
          join trnm_v13_rehearsal_candidate candidate using (match_id)
    ) then
        raise exception 'V13_REHEARSAL_PRE_FORGERY_WAS_NOT_REJECTED';
    end if;
end
\$rehearsal\$;

-- The deferred constraint trigger must reject the naked phase transition at
-- SET CONSTRAINTS time, and the PL/pgSQL subtransaction must roll it back.
do \$rehearsal\$
declare
    rejected boolean := false;
    failure text;
begin
    begin
        update trnm_online_matches match_row
           set phase = 'failed_closed',
               settlement_state = 'failed_closed',
               failure_reason = candidate.failure_reason
          from trnm_v13_rehearsal_candidate candidate
         where match_row.match_id = candidate.match_id;
        set constraints trnm_online_require_atomic_running_abandonment immediate;
    exception when sqlstate 'P0001' then
        get stacked diagnostics failure = message_text;
        if position('running failed_closed transition requires an exact same-transaction abandonment marker' in failure) = 0 then
            raise;
        end if;
        rejected := true;
    end;
    if not rejected then
        raise exception 'V13_REHEARSAL_NAKED_TRANSITION_WAS_NOT_REJECTED';
    end if;
    if exists (
        select 1
          from trnm_online_matches match_row
          join trnm_v13_rehearsal_candidate candidate using (match_id)
         where match_row.phase <> 'running'
            or match_row.settlement_state <> 'not_ready'
            or match_row.failure_reason is not null
    ) then
        raise exception 'V13_REHEARSAL_NAKED_TRANSITION_WAS_NOT_ROLLED_BACK';
    end if;
end
\$rehearsal\$;

-- Exact phase transition and marker insertion are one PostgreSQL transaction.
update trnm_online_matches match_row
   set phase = 'failed_closed',
       settlement_state = 'failed_closed',
       failure_reason = candidate.failure_reason
  from trnm_v13_rehearsal_candidate candidate
 where match_row.match_id = candidate.match_id;

insert into trnm_online_failed_closed_abandonment_markers (
    match_id,
    journal_owner_id,
    actor_generation,
    instance_id,
    actor_epoch,
    physical_host_id,
    authoritative_tick,
    next_sequence,
    match_revision,
    next_input_sequences,
    snapshot_hash,
    failure_reason
)
select match_id,
       journal_owner_id,
       actor_generation,
       instance_id,
       actor_epoch,
       physical_host_id,
       authoritative_tick,
       next_sequence,
       match_revision,
       next_input_sequences,
       snapshot_hash,
       failure_reason
  from trnm_v13_rehearsal_candidate;

set constraints trnm_online_require_atomic_running_abandonment immediate;

create or replace function pg_temp.trnm_v13_assert_summary_exact()
returns void
language plpgsql
as \$rehearsal\$
begin
    if exists (
        with hosts as (
            select physical_host_id
              from trnm_online_local_cold_witness_summaries
            union
            select physical_host_id
              from trnm_online_terminal_publication_acks
            union
            select physical_host_id
              from trnm_online_failed_closed_abandonment_markers
        ), expected as (
            select hosts.physical_host_id,
                   (select count(*)::bigint
                      from trnm_online_terminal_publication_acks terminal
                     where terminal.physical_host_id = hosts.physical_host_id)
                       as terminal_total,
                   (select count(*)::bigint
                      from trnm_online_terminal_publication_acks terminal
                     where terminal.physical_host_id = hosts.physical_host_id
                       and terminal.local_tombstone_state = 'sealed')
                       as terminal_sealed,
                   (select count(*)::bigint
                      from trnm_online_failed_closed_abandonment_markers abandonment
                     where abandonment.physical_host_id = hosts.physical_host_id)
                       as abandonment_total,
                   (select count(*)::bigint
                      from trnm_online_failed_closed_abandonment_markers abandonment
                     where abandonment.physical_host_id = hosts.physical_host_id
                       and abandonment.local_tombstone_state = 'sealed')
                       as abandonment_sealed
              from hosts
        )
        select 1
          from expected
          full join trnm_online_local_cold_witness_summaries summary
            using (physical_host_id)
         where summary.physical_host_id is null
            or expected.physical_host_id is null
            or summary.terminal_total_count is distinct from expected.terminal_total
            or summary.terminal_sealed_count is distinct from expected.terminal_sealed
            or summary.abandonment_total_count is distinct from expected.abandonment_total
            or summary.abandonment_sealed_count is distinct from expected.abandonment_sealed
    ) then
        raise exception 'V13_REHEARSAL_SUMMARY_IS_NOT_EXACT';
    end if;
end
\$rehearsal\$;

select pg_temp.trnm_v13_assert_summary_exact();

do \$rehearsal\$
begin
    if not exists (
        select 1
          from trnm_online_local_cold_witness_summaries summary
          join trnm_v13_rehearsal_candidate candidate using (physical_host_id)
         where summary.terminal_total_count = candidate.baseline_terminal_total
           and summary.terminal_sealed_count = candidate.baseline_terminal_sealed
           and summary.abandonment_total_count = candidate.baseline_abandonment_total + 1
           and summary.abandonment_sealed_count = candidate.baseline_abandonment_sealed
    ) then
        raise exception 'V13_REHEARSAL_HOT_PENDING_SUMMARY_DELTA_IS_WRONG';
    end if;
end
\$rehearsal\$;

update trnm_online_failed_closed_abandonment_markers marker
   set local_tombstone_state = 'sealed'
  from trnm_v13_rehearsal_candidate candidate
 where marker.match_id = candidate.match_id;

select pg_temp.trnm_v13_assert_summary_exact();

do \$rehearsal\$
begin
    if not exists (
        select 1
          from trnm_online_local_cold_witness_summaries summary
          join trnm_v13_rehearsal_candidate candidate using (physical_host_id)
         where summary.terminal_total_count = candidate.baseline_terminal_total
           and summary.terminal_sealed_count = candidate.baseline_terminal_sealed
           and summary.abandonment_total_count = candidate.baseline_abandonment_total + 1
           and summary.abandonment_sealed_count = candidate.baseline_abandonment_sealed + 1
    ) then
        raise exception 'V13_REHEARSAL_SEALED_SUMMARY_DELTA_IS_WRONG';
    end if;
end
\$rehearsal\$;

-- Marker authority, cold history deletion, and the match authority tuple are
-- all immutable after the witness exists.
do \$rehearsal\$
declare
    rejected boolean := false;
    failure text;
begin
    begin
        update trnm_online_failed_closed_abandonment_markers marker
           set snapshot_hash = case
               when marker.snapshot_hash = repeat('f', 64) then repeat('e', 64)
               else repeat('f', 64)
           end
          from trnm_v13_rehearsal_candidate candidate
         where marker.match_id = candidate.match_id;
    exception when sqlstate 'P0001' then
        get stacked diagnostics failure = message_text;
        if position('abandonment marker authority is immutable' in failure) = 0 then
            raise;
        end if;
        rejected := true;
    end;
    if not rejected then
        raise exception 'V13_REHEARSAL_MARKER_UPDATE_WAS_NOT_REJECTED';
    end if;
end
\$rehearsal\$;

do \$rehearsal\$
declare
    rejected boolean := false;
    failure text;
begin
    begin
        delete from trnm_online_failed_closed_abandonment_markers marker
         using trnm_v13_rehearsal_candidate candidate
         where marker.match_id = candidate.match_id;
    exception when sqlstate 'P0001' then
        get stacked diagnostics failure = message_text;
        if position('durable cold witness history cannot be deleted' in failure) = 0 then
            raise;
        end if;
        rejected := true;
    end;
    if not rejected then
        raise exception 'V13_REHEARSAL_MARKER_DELETE_WAS_NOT_REJECTED';
    end if;
end
\$rehearsal\$;

do \$rehearsal\$
declare
    rejected boolean := false;
    failure text;
begin
    begin
        update trnm_online_matches match_row
           set authoritative_tick = match_row.authoritative_tick + 1
          from trnm_v13_rehearsal_candidate candidate
         where match_row.match_id = candidate.match_id;
    exception when sqlstate 'P0001' then
        get stacked diagnostics failure = message_text;
        if position('marked failed-closed match authority is immutable' in failure) = 0 then
            raise;
        end if;
        rejected := true;
    end;
    if not rejected then
        raise exception 'V13_REHEARSAL_MATCH_AUTHORITY_UPDATE_WAS_NOT_REJECTED';
    end if;
end
\$rehearsal\$;

do \$rehearsal\$
declare
    mutation_count bigint;
    failure text;
begin
    begin
        update trnm_online_match_members member
           set next_input_sequence = case
               when member.next_input_sequence = 9223372036854775807
                   then member.next_input_sequence - 1
               else member.next_input_sequence + 1
           end
          from trnm_v13_rehearsal_candidate candidate
         where member.match_id = candidate.match_id
           and member.player_id = (
               select min(selected.player_id)
                 from trnm_online_match_members selected
                where selected.match_id = candidate.match_id
           );
        get diagnostics mutation_count = row_count;
        if mutation_count <> 1 then
            raise exception 'V13_REHEARSAL_MEMBER_DRIFT_MUTATION_COUNT_%', mutation_count;
        end if;
        if exists (
            select 1
              from trnm_online_failed_closed_abandonment_markers marker
              join trnm_v13_rehearsal_candidate candidate using (match_id)
             where marker.next_input_sequences = coalesce((
                 select jsonb_object_agg(
                     member.player_id,
                     to_jsonb(member.next_input_sequence)
                     order by member.player_id
                 )
                   from trnm_online_match_members member
                  where member.match_id = marker.match_id
             ), '{}'::jsonb)
        ) then
            raise exception 'V13_REHEARSAL_MEMBER_DRIFT_DID_NOT_BREAK_EXACT_GATE';
        end if;
        -- Deliberately abort this subtransaction after proving that the same
        -- exact comparison used by seal/readiness turns false.
        raise exception 'V13_REHEARSAL_MEMBER_DRIFT_ROLLBACK';
    exception when sqlstate 'P0001' then
        get stacked diagnostics failure = message_text;
        if failure <> 'V13_REHEARSAL_MEMBER_DRIFT_ROLLBACK' then
            raise;
        end if;
    end;
    if not exists (
        select 1
          from trnm_online_failed_closed_abandonment_markers marker
          join trnm_v13_rehearsal_candidate candidate using (match_id)
         where marker.next_input_sequences = coalesce((
             select jsonb_object_agg(
                 member.player_id,
                 to_jsonb(member.next_input_sequence)
                 order by member.player_id
             )
               from trnm_online_match_members member
              where member.match_id = marker.match_id
         ), '{}'::jsonb)
    ) then
        raise exception 'V13_REHEARSAL_MEMBER_DRIFT_SUBTRANSACTION_DID_NOT_ROLL_BACK';
    end if;
end
\$rehearsal\$;

rollback;

select json_build_object(
    'status', 'passed',
    'contract', '$REHEARSAL_CONTRACT',
    'migration_sha256', :'migration_checksum',
    'match_id', :'rehearsal_match_id',
    'top_level_rollback', true,
    'pre_forgery_rejected', true,
    'naked_transition_rejected', true,
    'exact_atomic_transition_verified', true,
    'summary_exact_verified', true,
    'immutability_verified', true,
    'member_cursor_drift_exact_gate_verified', true
);
"
} | (
  set -euo pipefail
  cex_psql_stdin -X -qAt -v ON_ERROR_STOP=1 \
    -v "migration_checksum=$MIGRATION_SHA256" -f -
))"
REHEARSAL_STATUS=$?
set -e

AFTER_STATE="$(v13_persistent_state)"
readonly AFTER_STATE
if [[ "$AFTER_STATE" != "$BEFORE_STATE" ]]; then
  echo "V13 rehearsal failed closed: ledger or table state persisted after rollback" >&2
  echo "before_state=$BEFORE_STATE" >&2
  echo "after_state=$AFTER_STATE" >&2
  exit 1
fi

if (( REHEARSAL_STATUS != 0 )); then
  echo "V13 rehearsal failed; PostgreSQL disconnected with the top-level transaction rolled back" >&2
  exit "$REHEARSAL_STATUS"
fi

[[ "$REHEARSAL_OUTPUT" == *'"status" : "passed"'* \
   && "$REHEARSAL_OUTPUT" == *'"top_level_rollback" : true'* ]] || {
  echo "V13 rehearsal did not return the expected pass contract" >&2
  exit 1
}

printf '%s\n' "$REHEARSAL_OUTPUT"
