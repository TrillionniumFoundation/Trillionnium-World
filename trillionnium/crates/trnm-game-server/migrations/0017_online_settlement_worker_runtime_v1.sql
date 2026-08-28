-- Runtime integration for the transaction-free terminal settlement outbox.
--
-- This migration adds immutable capture fences, stable remote request identity,
-- lease-fenced remote completion, and a separate campaign-apply marker. External
-- signer/CEX calls remain outside PostgreSQL transactions in
-- trnm-settlement-worker.

alter table public.trnm_online_settlement_jobs
    add column if not exists expected_campaign_state_hash text;

update public.trnm_online_settlement_jobs job
   set expected_campaign_state_hash = campaign.state_hash
  from public.trnm_online_campaigns campaign
 where campaign.campaign_id = job.campaign_id
   and job.expected_campaign_state_hash is null;

alter table public.trnm_online_settlement_jobs
    alter column expected_campaign_state_hash set not null;

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_expected_state_hash_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_expected_state_hash_check
    check (expected_campaign_state_hash ~ '^[0-9a-f]{64}$');

alter table public.trnm_online_settlement_jobs
    add column if not exists capture_id text;
alter table public.trnm_online_settlement_jobs
    add column if not exists capture_generation bigint not null default 1
        check (capture_generation > 0);

-- job_id remains the capture-scoped worker row identity. remote_request_id is
-- the stable signer/CEX idempotency identity and deliberately excludes
-- capture_id/capture_generation and intent_hash. It is derived for every
-- existing and future row from a SHA-256 over unambiguous u32-big-endian
-- length-prefixed UTF-8 components: contract domain, match ID, campaign ID and
-- intent ID. The immutable intent_hash remains a separate payload-integrity
-- fence, so reusing one intent ID with different bytes produces an exact remote
-- conflict instead of silently minting a second request identity.
alter table public.trnm_online_settlement_jobs
    add column if not exists remote_request_id text;

-- IF NOT EXISTS must never silently preserve a stored-generated or otherwise
-- incompatible column left by a locally applied pre-review migration. The
-- migration checksum blocks changed deployed revisions; this catalogue check
-- also fails closed on an unexpected DDL shape.
do $function$
begin
    if not exists (
        select 1
          from pg_catalog.pg_attribute attribute
         where attribute.attrelid =
               'public.trnm_online_settlement_jobs'::pg_catalog.regclass
           and attribute.attname = 'remote_request_id'
           and attribute.attgenerated = ''
           and not attribute.attisdropped
    ) then
        raise exception 'remote_request_id must be an ordinary stored column';
    end if;
end;
$function$;

create or replace function public.trnm_online_remote_request_id_v1(
    p_match_id uuid,
    p_campaign_id text,
    p_intent_id text
)
returns text
language sql
stable
strict
security invoker
set search_path = pg_catalog, public
as $function$
    select 'trnm-settlement-remote-v1:' || pg_catalog.encode(
        pg_catalog.sha256(
            pg_catalog.decode(
                pg_catalog.lpad(
                    pg_catalog.to_hex(
                        pg_catalog.octet_length(
                            pg_catalog.convert_to('trnm_settlement_remote_v1', 'UTF8')
                        )
                    ),
                    8,
                    '0'
                ),
                'hex'
            )
            || pg_catalog.convert_to('trnm_settlement_remote_v1', 'UTF8')
            || pg_catalog.decode(
                pg_catalog.lpad(
                    pg_catalog.to_hex(
                        pg_catalog.octet_length(
                            pg_catalog.convert_to(p_match_id::text, 'UTF8')
                        )
                    ),
                    8,
                    '0'
                ),
                'hex'
            )
            || pg_catalog.convert_to(p_match_id::text, 'UTF8')
            || pg_catalog.decode(
                pg_catalog.lpad(
                    pg_catalog.to_hex(
                        pg_catalog.octet_length(
                            pg_catalog.convert_to(p_campaign_id, 'UTF8')
                        )
                    ),
                    8,
                    '0'
                ),
                'hex'
            )
            || pg_catalog.convert_to(p_campaign_id, 'UTF8')
            || pg_catalog.decode(
                pg_catalog.lpad(
                    pg_catalog.to_hex(
                        pg_catalog.octet_length(
                            pg_catalog.convert_to(p_intent_id, 'UTF8')
                        )
                    ),
                    8,
                    '0'
                ),
                'hex'
            )
            || pg_catalog.convert_to(p_intent_id, 'UTF8')
        ),
        'hex'
    )
$function$;

update public.trnm_online_settlement_jobs job
   set remote_request_id = public.trnm_online_remote_request_id_v1(
       job.match_id,
       job.campaign_id,
       job.intent_id
   )
 where job.remote_request_id is null;

alter table public.trnm_online_settlement_jobs
    alter column remote_request_id set not null;

create or replace function public.trnm_online_set_remote_request_id_v1()
returns trigger
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    expected text;
begin
    if tg_op = 'UPDATE'
       and (
           new.match_id is distinct from old.match_id
           or new.campaign_id is distinct from old.campaign_id
           or new.intent_id is distinct from old.intent_id
       ) then
        raise exception using
            errcode = '23514',
            message = 'settlement match, campaign and intent identity fields are immutable';
    end if;

    expected := public.trnm_online_remote_request_id_v1(
        new.match_id,
        new.campaign_id,
        new.intent_id
    );
    if new.remote_request_id is not null
       and new.remote_request_id <> expected then
        raise exception using
            errcode = '23514',
            message = 'remote_request_id does not match durable settlement identity';
    end if;
    new.remote_request_id := expected;
    return new;
end;
$function$;

drop trigger if exists trnm_online_settlement_remote_id_insert_v1
    on public.trnm_online_settlement_jobs;
create trigger trnm_online_settlement_remote_id_insert_v1
before insert on public.trnm_online_settlement_jobs
for each row
execute function public.trnm_online_set_remote_request_id_v1();

drop trigger if exists trnm_online_settlement_remote_id_update_v1
    on public.trnm_online_settlement_jobs;
create trigger trnm_online_settlement_remote_id_update_v1
before update of match_id, campaign_id, intent_id, remote_request_id
on public.trnm_online_settlement_jobs
for each row
execute function public.trnm_online_set_remote_request_id_v1();

alter table public.trnm_online_settlement_jobs
    add column if not exists authorization_request_id text;
alter table public.trnm_online_settlement_jobs
    add column if not exists remote_attempts integer not null default 0
        check (remote_attempts between 0 and 16);
alter table public.trnm_online_settlement_jobs
    add column if not exists remote_completed_at timestamptz;
alter table public.trnm_online_settlement_jobs
    add column if not exists campaign_applied_at timestamptz;
alter table public.trnm_online_settlement_jobs
    add column if not exists failure_class text
        check (failure_class is null or failure_class in ('retryable', 'permanent', 'stale'));

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_capture_id_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_capture_id_check
    check (capture_id is null or capture_id ~ '^trnm-settlement-capture-v1:[0-9a-f]{64}$');

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_remote_request_id_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_remote_request_id_check
    check (
        remote_request_id ~ '^trnm-settlement-remote-v1:[0-9a-f]{64}$'
        and length(remote_request_id) <= 256
    );

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_authorization_identity_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_authorization_identity_check
    check (
        authorization_request_id is null
        or authorization_request_id = remote_request_id
    );

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_entitlement_nonce_identity_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_entitlement_nonce_identity_check
    check (
        entitlement_nonce is null
        or entitlement_nonce = remote_request_id
    );

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_authorization_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_authorization_check check (
        authorized_intent_json is null
        or (
            authorization_request_id is not null
            and btrim(authorization_request_id) <> ''
            and length(authorization_request_id) <= 256
        )
    );

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_apply_check;
alter table public.trnm_online_settlement_jobs
    add constraint trnm_online_settlement_jobs_apply_check check (
        campaign_applied_at is null
        or (state = 'succeeded' and receipt_json is not null)
    );

create table if not exists public.trnm_online_settlement_captures (
    capture_id text primary key
        check (capture_id ~ '^trnm-settlement-capture-v1:[0-9a-f]{64}$'),
    contract_version text not null
        check (contract_version = 'trnm_settlement_capture_v1'),
    match_id uuid not null
        references public.trnm_online_matches(match_id) on delete restrict,
    capture_generation bigint not null check (capture_generation > 0),
    terminal_identity_hash text not null
        check (terminal_identity_hash ~ '^[0-9a-f]{64}$'),
    terminal_identity_json jsonb not null
        check (jsonb_typeof(terminal_identity_json) = 'object'),
    campaign_fences_json jsonb not null
        check (jsonb_typeof(campaign_fences_json) = 'object'),
    head_intent_ids_json jsonb not null
        check (jsonb_typeof(head_intent_ids_json) = 'object'),
    state text not null default 'active'
        check (state in ('active', 'applied', 'finalized', 'stale', 'dead_letter')),
    last_error text check (last_error is null or length(last_error) <= 1024),
    created_at timestamptz not null default clock_timestamp(),
    updated_at timestamptz not null default clock_timestamp(),
    applied_at timestamptz,
    unique (match_id, capture_generation),
    check (
        (state = 'active' and applied_at is null)
        or (state in ('applied', 'finalized') and applied_at is not null)
        or state in ('stale', 'dead_letter')
    )
);

create unique index if not exists idx_trnm_online_settlement_capture_active
    on public.trnm_online_settlement_captures(match_id)
    where state = 'active';

create index if not exists idx_trnm_online_settlement_capture_apply
    on public.trnm_online_settlement_captures(created_at, capture_id)
    where state = 'active';

alter table public.trnm_online_settlement_jobs
    drop constraint if exists trnm_online_settlement_jobs_match_id_campaign_id_intent_id_key;

create unique index if not exists idx_trnm_online_settlement_job_capture_intent
    on public.trnm_online_settlement_jobs(capture_id, campaign_id, intent_id)
    where capture_id is not null;

create index if not exists idx_trnm_online_settlement_job_remote_request
    on public.trnm_online_settlement_jobs(remote_request_id, created_at, job_id);

create index if not exists idx_trnm_online_settlement_job_unapplied_success
    on public.trnm_online_settlement_jobs(capture_id, campaign_id, created_at)
    where state = 'succeeded' and campaign_applied_at is null;

-- The v1 claim function predates active-capture fencing and stable remote
-- identity. Leaving it callable would create a second, weaker claim path.
create or replace function public.trnm_online_claim_settlement_job_v1(
    p_owner text,
    p_lease_milliseconds bigint
)
returns setof public.trnm_online_settlement_jobs
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
begin
    raise exception using
        errcode = '0A000',
        message = 'trnm_online_claim_settlement_job_v1 is retired; use v2';
    return;
end;
$function$;

create or replace function public.trnm_online_claim_settlement_job_v2(
    p_owner text,
    p_lease_milliseconds bigint
)
returns setof public.trnm_online_settlement_jobs
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    claimed public.trnm_online_settlement_jobs%rowtype;
begin
    if p_owner is null or btrim(p_owner) = '' or length(p_owner) > 256 then
        raise exception 'invalid settlement lease owner';
    end if;
    if p_lease_milliseconds <= 0 or p_lease_milliseconds > 300000 then
        raise exception 'invalid settlement lease duration';
    end if;

    with candidate as materialized (
        select job.job_id
          from public.trnm_online_settlement_jobs job
          join public.trnm_online_settlement_captures capture
            on capture.capture_id = job.capture_id
         where capture.state = 'active'
           and job.campaign_applied_at is null
           and job.remote_attempts < 16
           and (
               job.state = 'pending'
               or (job.state = 'retryable' and job.next_attempt_at <= pg_catalog.clock_timestamp())
               or (job.state = 'leased' and job.lease_expires_at <= pg_catalog.clock_timestamp())
           )
         order by
           case job.queue_lane when 'compensation' then 0 else 1 end,
           coalesce(job.next_attempt_at, job.created_at),
           job.created_at,
           job.job_id
         for update of job skip locked
         limit 1
    )
    update public.trnm_online_settlement_jobs job
       set state = 'leased',
           attempts = least(job.attempts + 1, 16),
           lease_owner = p_owner,
           lease_generation = job.lease_generation + 1,
           lease_expires_at = pg_catalog.clock_timestamp()
               + pg_catalog.make_interval(secs => p_lease_milliseconds::double precision / 1000.0),
           next_attempt_at = null,
           last_error = null,
           failure_class = null,
           entitlement_issued_at_epoch = coalesce(
               job.entitlement_issued_at_epoch,
               floor(extract(epoch from pg_catalog.clock_timestamp()))::bigint
           ),
           entitlement_expires_at_epoch = coalesce(
               job.entitlement_expires_at_epoch,
               floor(extract(epoch from pg_catalog.clock_timestamp()))::bigint + 600
           ),
           entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id),
           authorization_request_id = coalesce(
               job.authorization_request_id,
               job.remote_request_id
           ),
           updated_at = pg_catalog.clock_timestamp()
      from candidate
     where job.job_id = candidate.job_id
     returning job.* into claimed;

    if found then
        return next claimed;
    end if;
    return;
end;
$function$;

create or replace function public.trnm_online_store_settlement_authorization_v1(
    p_job_id text,
    p_owner text,
    p_lease_generation bigint,
    p_authorization_request_id text,
    p_authorized_intent_json jsonb,
    p_signer_receipt_hash text
)
returns boolean
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    changed bigint;
begin
    if p_authorization_request_id is null
       or btrim(p_authorization_request_id) = ''
       or length(p_authorization_request_id) > 256
       or p_authorized_intent_json is null
       or jsonb_typeof(p_authorized_intent_json) <> 'object'
       or (p_signer_receipt_hash is not null
           and p_signer_receipt_hash !~ '^[0-9a-f]{64}$') then
        raise exception 'invalid settlement authorization payload';
    end if;

    update public.trnm_online_settlement_jobs
       set authorization_request_id = p_authorization_request_id,
           authorized_intent_json = p_authorized_intent_json,
           signer_receipt_hash = p_signer_receipt_hash,
           updated_at = pg_catalog.clock_timestamp()
     where job_id = p_job_id
       and state = 'leased'
       and lease_owner = p_owner
       and lease_generation = p_lease_generation
       and lease_expires_at > pg_catalog.clock_timestamp()
       and p_authorization_request_id = remote_request_id
       and (
           authorized_intent_json is null
           or (
               authorization_request_id = p_authorization_request_id
               and authorized_intent_json = p_authorized_intent_json
               and signer_receipt_hash is not distinct from p_signer_receipt_hash
           )
       );
    get diagnostics changed = row_count;
    return changed = 1;
end;
$function$;

create or replace function public.trnm_online_begin_settlement_remote_attempt_v1(
    p_job_id text,
    p_owner text,
    p_lease_generation bigint
)
returns integer
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    next_attempt integer;
begin
    update public.trnm_online_settlement_jobs
       set remote_attempts = remote_attempts + 1,
           updated_at = pg_catalog.clock_timestamp()
     where job_id = p_job_id
       and state = 'leased'
       and lease_owner = p_owner
       and lease_generation = p_lease_generation
       and lease_expires_at > pg_catalog.clock_timestamp()
       and remote_attempts < 16
     returning remote_attempts into next_attempt;
    return next_attempt;
end;
$function$;

create or replace function public.trnm_online_complete_settlement_job_v1(
    p_job_id text,
    p_owner text,
    p_lease_generation bigint,
    p_receipt_id text,
    p_receipt_hash text,
    p_receipt_json jsonb,
    p_wallet_snapshot_json jsonb
)
returns boolean
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    changed bigint;
begin
    if p_receipt_id is null
       or btrim(p_receipt_id) = ''
       or p_receipt_hash !~ '^[0-9a-f]{64}$'
       or p_receipt_json is null
       or jsonb_typeof(p_receipt_json) <> 'object'
       or (p_wallet_snapshot_json is not null
           and jsonb_typeof(p_wallet_snapshot_json) <> 'object') then
        raise exception 'invalid terminal settlement receipt';
    end if;

    update public.trnm_online_settlement_jobs
       set state = 'succeeded',
           receipt_id = p_receipt_id,
           receipt_hash = p_receipt_hash,
           receipt_json = p_receipt_json,
           wallet_snapshot_json = p_wallet_snapshot_json,
           remote_completed_at = pg_catalog.clock_timestamp(),
           completed_at = pg_catalog.clock_timestamp(),
           lease_owner = null,
           lease_expires_at = null,
           next_attempt_at = null,
           last_error = null,
           failure_class = null,
           updated_at = pg_catalog.clock_timestamp()
     where job_id = p_job_id
       and state = 'leased'
       and lease_owner = p_owner
       and lease_generation = p_lease_generation
       and lease_expires_at > pg_catalog.clock_timestamp();
    get diagnostics changed = row_count;
    return changed = 1;
end;
$function$;

create or replace function public.trnm_online_retry_settlement_job_v1(
    p_job_id text,
    p_owner text,
    p_lease_generation bigint,
    p_error text,
    p_delay_milliseconds bigint
)
returns text
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    next_state text;
begin
    if p_delay_milliseconds < 0 or p_delay_milliseconds > 3600000 then
        raise exception 'invalid settlement retry delay';
    end if;

    update public.trnm_online_settlement_jobs
       set state = case when remote_attempts >= 16 then 'dead_letter' else 'retryable' end,
           next_attempt_at = case
               when remote_attempts >= 16 then null
               else pg_catalog.clock_timestamp()
                   + pg_catalog.make_interval(secs => p_delay_milliseconds::double precision / 1000.0)
           end,
           last_error = left(coalesce(p_error, 'remote settlement retry'), 1024),
           failure_class = case when remote_attempts >= 16 then 'permanent' else 'retryable' end,
           completed_at = case when remote_attempts >= 16
               then pg_catalog.clock_timestamp() else null end,
           lease_owner = null,
           lease_expires_at = null,
           updated_at = pg_catalog.clock_timestamp()
     where job_id = p_job_id
       and state = 'leased'
       and lease_owner = p_owner
       and lease_generation = p_lease_generation
       and lease_expires_at > pg_catalog.clock_timestamp()
     returning state into next_state;
    return next_state;
end;
$function$;

create or replace function public.trnm_online_dead_letter_settlement_job_v1(
    p_job_id text,
    p_owner text,
    p_lease_generation bigint,
    p_error text
)
returns boolean
language plpgsql
security invoker
set search_path = pg_catalog, public
as $function$
declare
    changed bigint;
begin
    update public.trnm_online_settlement_jobs
       set state = 'dead_letter',
           next_attempt_at = null,
           last_error = left(coalesce(p_error, 'permanent settlement failure'), 1024),
           failure_class = 'permanent',
           completed_at = pg_catalog.clock_timestamp(),
           lease_owner = null,
           lease_expires_at = null,
           updated_at = pg_catalog.clock_timestamp()
     where job_id = p_job_id
       and state = 'leased'
       and lease_owner = p_owner
       and lease_generation = p_lease_generation
       and lease_expires_at > pg_catalog.clock_timestamp();
    get diagnostics changed = row_count;
    return changed = 1;
end;
$function$;

-- Raw state='succeeded' means the remote receipt is durably stored. It does not
-- mean campaign progression has been applied. This projection forces operators
-- and player-facing adapters to carry both dimensions explicitly.
create or replace view public.trnm_online_settlement_job_status_v1 as
select
    job.job_id,
    job.remote_request_id,
    job.capture_id,
    job.match_id,
    job.campaign_id,
    job.intent_id,
    job.queue_lane,
    case job.state
        when 'succeeded' then 'remote_succeeded'
        when 'dead_letter' then 'remote_dead_letter'
        else 'remote_' || job.state
    end as remote_state,
    case
        when job.campaign_applied_at is not null then 'applied'
        when job.state = 'succeeded' then 'pending_apply'
        when job.state = 'dead_letter' then 'blocked'
        else 'waiting_remote'
    end as application_state,
    job.attempts,
    job.remote_attempts,
    job.lease_generation,
    job.next_attempt_at,
    job.remote_completed_at,
    job.campaign_applied_at,
    job.updated_at
from public.trnm_online_settlement_jobs job;
