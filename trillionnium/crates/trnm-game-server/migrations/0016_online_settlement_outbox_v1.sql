-- Transaction-free external settlement for terminal World campaigns.
--
-- Claim/apply transactions remain PostgreSQL-only. Signer and CEX calls are
-- executed by trnm-settlement-worker after a lease has committed and before a
-- second lease-fenced apply transaction begins.

create table if not exists public.trnm_online_settlement_jobs (
    job_id text primary key
        check (job_id ~ '^trnm-settlement-outbox-v1:[0-9a-f]+$'),
    contract_version text not null
        check (contract_version = 'trnm_settlement_outbox_v1'),
    match_id uuid not null
        references public.trnm_online_matches(match_id) on delete cascade,
    campaign_id text not null
        references public.trnm_online_campaigns(campaign_id) on delete cascade,
    intent_id text not null check (btrim(intent_id) <> ''),
    intent_hash text not null check (intent_hash ~ '^[0-9a-f]{64}$'),
    expected_campaign_revision bigint not null
        check (expected_campaign_revision >= 0),
    queue_lane text not null
        check (queue_lane in ('ordinary', 'compensation')),
    intent_json jsonb not null check (jsonb_typeof(intent_json) = 'object'),
    authorized_intent_json jsonb
        check (authorized_intent_json is null
            or jsonb_typeof(authorized_intent_json) = 'object'),
    entitlement_issued_at_epoch bigint,
    entitlement_expires_at_epoch bigint,
    entitlement_nonce text,
    signer_receipt_hash text
        check (signer_receipt_hash is null
            or signer_receipt_hash ~ '^[0-9a-f]{64}$'),
    state text not null default 'pending'
        check (state in ('pending', 'leased', 'retryable', 'succeeded', 'dead_letter')),
    attempts integer not null default 0
        check (attempts between 0 and 16),
    lease_owner text,
    lease_generation bigint not null default 0
        check (lease_generation >= 0),
    lease_expires_at timestamptz,
    next_attempt_at timestamptz,
    last_error text check (last_error is null or length(last_error) <= 1024),
    receipt_id text,
    receipt_hash text
        check (receipt_hash is null or receipt_hash ~ '^[0-9a-f]{64}$'),
    receipt_json jsonb
        check (receipt_json is null or jsonb_typeof(receipt_json) = 'object'),
    wallet_snapshot_json jsonb
        check (wallet_snapshot_json is null
            or jsonb_typeof(wallet_snapshot_json) = 'object'),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    completed_at timestamptz,
    unique (match_id, campaign_id, intent_id),
    check (
        (state = 'leased'
            and lease_owner is not null
            and btrim(lease_owner) <> ''
            and lease_generation > 0
            and lease_expires_at is not null
            and completed_at is null)
        or
        (state in ('pending', 'retryable')
            and lease_owner is null
            and lease_expires_at is null
            and completed_at is null)
        or
        (state in ('succeeded', 'dead_letter')
            and lease_owner is null
            and lease_expires_at is null
            and completed_at is not null)
    ),
    check (
        entitlement_issued_at_epoch is null
        or (
            entitlement_expires_at_epoch is not null
            and entitlement_expires_at_epoch > entitlement_issued_at_epoch
            and entitlement_nonce is not null
            and btrim(entitlement_nonce) <> ''
        )
    ),
    check (
        state <> 'succeeded'
        or (
            receipt_id is not null
            and btrim(receipt_id) <> ''
            and receipt_hash is not null
            and receipt_json is not null
        )
    )
);

create index if not exists idx_trnm_online_settlement_jobs_eligible
    on public.trnm_online_settlement_jobs (
        queue_lane desc,
        coalesce(next_attempt_at, created_at),
        created_at,
        job_id
    )
    where state in ('pending', 'retryable', 'leased') and attempts < 16;

create index if not exists idx_trnm_online_settlement_jobs_match
    on public.trnm_online_settlement_jobs(match_id, campaign_id, created_at);

create index if not exists idx_trnm_online_settlement_jobs_dead_letter
    on public.trnm_online_settlement_jobs(updated_at, job_id)
    where state = 'dead_letter';

create or replace function public.trnm_online_settlement_match_ready_v1(
    p_match_id uuid
)
returns boolean
language sql
stable
security invoker
set search_path = pg_catalog, public
as $function$
    select exists (
        select 1
          from public.trnm_online_matches m
          join public.trnm_online_terminal_publication_acks a
            on a.match_id = m.match_id
         where m.match_id = p_match_id
           and m.phase = 'complete'
           and m.settlement_state = 'pending'
           and m.terminal_publication_state = 'acknowledged'
           and a.local_tombstone_state = 'sealed'
           and m.checkpoint_sequence = m.next_sequence
           and m.result_hash is not null
           and m.terminal_publication_actor_generation is not null
           and a.actor_generation = m.terminal_publication_actor_generation
           and a.instance_id = m.assigned_instance_id
           and a.actor_epoch = m.assigned_instance_epoch
           and a.physical_host_id = m.assigned_physical_host_id
           and a.authoritative_tick = m.authoritative_tick
           and a.next_sequence = m.next_sequence
           and a.match_revision = m.match_revision
           and a.next_input_sequences = coalesce(
               (select jsonb_object_agg(
                   member.player_id,
                   to_jsonb(member.next_input_sequence)
                   order by member.player_id
                )
                  from public.trnm_online_match_members member
                 where member.match_id = m.match_id),
               '{}'::jsonb
           )
           and a.snapshot_hash = m.snapshot_hash
           and a.phase = 'complete'
           and a.result_hash = m.result_hash
           and a.published_settlement_state = m.settlement_state
    )
$function$;

create or replace function public.trnm_online_claim_settlement_job_v1(
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
         where job.attempts < 16
           and (
               job.state = 'pending'
               or (job.state = 'retryable' and job.next_attempt_at <= pg_catalog.now())
               or (job.state = 'leased' and job.lease_expires_at <= pg_catalog.now())
           )
         order by
           case job.queue_lane when 'compensation' then 0 else 1 end,
           coalesce(job.next_attempt_at, job.created_at),
           job.created_at,
           job.job_id
         for update skip locked
         limit 1
    )
    update public.trnm_online_settlement_jobs job
       set state = 'leased',
           attempts = job.attempts + 1,
           lease_owner = p_owner,
           lease_generation = job.lease_generation + 1,
           lease_expires_at = pg_catalog.now()
               + pg_catalog.make_interval(secs => p_lease_milliseconds::double precision / 1000.0),
           next_attempt_at = null,
           last_error = null,
           entitlement_issued_at_epoch = coalesce(
               job.entitlement_issued_at_epoch,
               floor(extract(epoch from pg_catalog.now()))::bigint
           ),
           entitlement_expires_at_epoch = coalesce(
               job.entitlement_expires_at_epoch,
               floor(extract(epoch from pg_catalog.now()))::bigint + 600
           ),
           entitlement_nonce = coalesce(job.entitlement_nonce, job.job_id),
           updated_at = pg_catalog.now()
      from candidate
     where job.job_id = candidate.job_id
     returning job.* into claimed;

    if found then
        return next claimed;
    end if;
    return;
end
$function$;
