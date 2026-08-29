-- Settlement runtime lifecycle, poison isolation and exact campaign-job fences.
--
-- This migration is intentionally limited to World-owned outbox runtime
-- safety. It does not enable trusted settlement or public online operation.

do $function$
begin
    if exists (
        select 1
          from public.trnm_online_settlement_jobs
         where capture_id is not null
         group by capture_id, campaign_id
        having count(*) > 1
    ) then
        raise exception using
            errcode = '23505',
            message = 'duplicate settlement jobs exist for one capture/campaign';
    end if;
end;
$function$;

create unique index if not exists idx_trnm_online_settlement_job_capture_campaign_v1
    on public.trnm_online_settlement_jobs(capture_id, campaign_id)
    where capture_id is not null;

create table if not exists public.trnm_online_settlement_runtime_failures (
    subject_kind text not null
        check (subject_kind in ('capture', 'apply')),
    subject_id text not null,
    consecutive_failures integer not null
        check (consecutive_failures between 1 and 1000000),
    last_error text not null
        check (length(btrim(last_error)) between 1 and 1024),
    next_attempt_at timestamptz not null,
    quarantined_at timestamptz,
    first_failed_at timestamptz not null default pg_catalog.clock_timestamp(),
    updated_at timestamptz not null default pg_catalog.clock_timestamp(),
    primary key (subject_kind, subject_id),
    check (
        (subject_kind = 'capture'
         and subject_id ~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$')
        or
        (subject_kind = 'apply'
         and subject_id ~ '^trnm-settlement-capture-v1:[0-9a-f]{64}$')
    ),
    check (quarantined_at is null or quarantined_at >= first_failed_at)
);

create index if not exists idx_trnm_online_settlement_runtime_retry_v1
    on public.trnm_online_settlement_runtime_failures(
        subject_kind,
        next_attempt_at,
        subject_id
    )
    where quarantined_at is null;

create index if not exists idx_trnm_online_settlement_runtime_quarantine_v1
    on public.trnm_online_settlement_runtime_failures(
        quarantined_at,
        subject_kind,
        subject_id
    )
    where quarantined_at is not null;

create table if not exists public.trnm_online_settlement_quarantine_releases (
    release_id text primary key
        check (release_id ~ '^trnm-settlement-quarantine-release-v1:[0-9a-f]{64}$'),
    contract_version text not null
        check (contract_version = 'trnm_settlement_quarantine_release_v1'),
    subject_kind text not null
        check (subject_kind in ('capture', 'apply')),
    subject_id text not null,
    prior_consecutive_failures integer not null
        check (prior_consecutive_failures between 8 and 1000000),
    prior_last_error text not null
        check (length(btrim(prior_last_error)) between 1 and 1024),
    prior_quarantined_at timestamptz not null,
    operator_id text not null
        check (length(btrim(operator_id)) between 1 and 256),
    change_ticket text not null
        check (change_ticket ~ '^[A-Za-z0-9][A-Za-z0-9._:/-]{2,127}$'),
    reason text not null
        check (length(btrim(reason)) between 8 and 1024),
    policy_revision bigint not null
        references public.trnm_online_settlement_operator_policy_revisions(policy_revision)
        on delete restrict,
    released_at timestamptz not null default pg_catalog.clock_timestamp(),
    retain_until timestamptz not null,
    check (
        (subject_kind = 'capture'
         and subject_id ~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$')
        or
        (subject_kind = 'apply'
         and subject_id ~ '^trnm-settlement-capture-v1:[0-9a-f]{64}$')
    ),
    check (retain_until >= released_at + interval '365 days')
);

create index if not exists idx_trnm_online_settlement_quarantine_release_subject_v1
    on public.trnm_online_settlement_quarantine_releases(
        subject_kind,
        subject_id,
        released_at
    );

drop trigger if exists trnm_online_settlement_quarantine_release_no_update_delete_v1
    on public.trnm_online_settlement_quarantine_releases;
create trigger trnm_online_settlement_quarantine_release_no_update_delete_v1
before update or delete on public.trnm_online_settlement_quarantine_releases
for each statement
execute function public.trnm_online_reject_settlement_operator_evidence_mutation_v1();

drop trigger if exists trnm_online_settlement_quarantine_release_no_truncate_v1
    on public.trnm_online_settlement_quarantine_releases;
create trigger trnm_online_settlement_quarantine_release_no_truncate_v1
before truncate on public.trnm_online_settlement_quarantine_releases
for each statement
execute function public.trnm_online_reject_settlement_operator_evidence_mutation_v1();

create or replace function public.trnm_online_release_settlement_quarantine_v1(
    p_release_id text,
    p_subject_kind text,
    p_subject_id text,
    p_operator_id text,
    p_change_ticket text,
    p_reason text
)
returns boolean
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    failure public.trnm_online_settlement_runtime_failures%rowtype;
    existing public.trnm_online_settlement_quarantine_releases%rowtype;
    policy public.trnm_online_settlement_operator_policy_revisions%rowtype;
    now_at timestamptz := pg_catalog.clock_timestamp();
begin
    if p_release_id is null
       or p_release_id !~ '^trnm-settlement-quarantine-release-v1:[0-9a-f]{64}$'
       or p_subject_kind is null
       or p_subject_kind not in ('capture', 'apply')
       or p_subject_id is null
       or btrim(p_subject_id) = ''
       or (
           p_subject_kind = 'capture'
           and p_subject_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
       )
       or (
           p_subject_kind = 'apply'
           and p_subject_id !~ '^trnm-settlement-capture-v1:[0-9a-f]{64}$'
       )
       or p_operator_id is null
       or length(btrim(p_operator_id)) not between 1 and 256
       or p_change_ticket is null
       or p_change_ticket !~ '^[A-Za-z0-9][A-Za-z0-9._:/-]{2,127}$'
       or p_reason is null
       or length(btrim(p_reason)) not between 8 and 1024 then
        raise exception using
            errcode = '22023',
            message = 'invalid settlement quarantine release request';
    end if;

    select * into existing
      from public.trnm_online_settlement_quarantine_releases
     where release_id = p_release_id;
    if found then
        if existing.subject_kind = p_subject_kind
           and existing.subject_id = p_subject_id
           and existing.operator_id = p_operator_id
           and existing.change_ticket = p_change_ticket
           and existing.reason = p_reason then
            return true;
        end if;
        raise exception using
            errcode = '23514',
            message = 'quarantine release identity was reused with different material';
    end if;

    select * into failure
      from public.trnm_online_settlement_runtime_failures
     where subject_kind = p_subject_kind
       and subject_id = p_subject_id
     for update;
    if not found or failure.quarantined_at is null then
        raise exception using
            errcode = '55000',
            message = 'settlement runtime subject is not quarantined';
    end if;

    select * into policy
      from public.trnm_online_settlement_operator_policy_revisions
     order by policy_revision desc
     limit 1;
    if not found then
        raise exception using
            errcode = '55000',
            message = 'settlement operator policy is missing';
    end if;

    insert into public.trnm_online_settlement_quarantine_releases (
        release_id,
        contract_version,
        subject_kind,
        subject_id,
        prior_consecutive_failures,
        prior_last_error,
        prior_quarantined_at,
        operator_id,
        change_ticket,
        reason,
        policy_revision,
        released_at,
        retain_until
    ) values (
        p_release_id,
        'trnm_settlement_quarantine_release_v1',
        failure.subject_kind,
        failure.subject_id,
        failure.consecutive_failures,
        failure.last_error,
        failure.quarantined_at,
        p_operator_id,
        p_change_ticket,
        p_reason,
        policy.policy_revision,
        now_at,
        now_at + pg_catalog.make_interval(days => policy.retention_days)
    );

    delete from public.trnm_online_settlement_runtime_failures
     where subject_kind = failure.subject_kind
       and subject_id = failure.subject_id
       and quarantined_at = failure.quarantined_at;
    if not found then
        raise exception using
            errcode = '55000',
            message = 'settlement quarantine release lost its exact fence';
    end if;
    return true;
end;
$function$;

revoke all on function public.trnm_online_release_settlement_quarantine_v1(
    text, text, text, text, text, text
) from public;

create or replace view public.trnm_online_settlement_runtime_failure_status_v1 as
select
    subject_kind,
    subject_id,
    consecutive_failures,
    last_error,
    next_attempt_at,
    quarantined_at,
    first_failed_at,
    updated_at,
    case
        when quarantined_at is not null then 'quarantined'
        when next_attempt_at > pg_catalog.clock_timestamp() then 'backoff'
        else 'eligible'
    end as runtime_state
from public.trnm_online_settlement_runtime_failures;

create or replace view public.trnm_online_settlement_runtime_failure_metrics_v1 as
select
    count(*) filter (where subject_kind = 'capture' and quarantined_at is null)
        as capture_backoff,
    count(*) filter (where subject_kind = 'apply' and quarantined_at is null)
        as apply_backoff,
    count(*) filter (where subject_kind = 'capture' and quarantined_at is not null)
        as capture_quarantined,
    count(*) filter (where subject_kind = 'apply' and quarantined_at is not null)
        as apply_quarantined,
    max(consecutive_failures) as maximum_consecutive_failures,
    min(first_failed_at) filter (where quarantined_at is not null)
        as oldest_quarantine_at
from public.trnm_online_settlement_runtime_failures;
