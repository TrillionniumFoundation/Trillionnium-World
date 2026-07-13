-- Online Production v2: distributed admission/capacity evidence and moderated
-- shift ownership. External KMS, edge and second-host attestations remain
-- deliberately absent until supplied by real infrastructure.

create table if not exists trnm_online_admission_windows (
    bucket_key text not null check (length(bucket_key) = 64),
    window_started_at timestamptz not null,
    request_class text not null check (request_class in ('control', 'data')),
    request_count bigint not null default 0 check (request_count >= 0),
    rejection_count bigint not null default 0 check (rejection_count >= 0),
    last_instance_id text not null,
    updated_at timestamptz not null default now(),
    primary key (bucket_key, window_started_at)
);
create index if not exists idx_trnm_online_admission_recent
    on trnm_online_admission_windows(window_started_at desc);

create table if not exists trnm_online_capacity_samples (
    sample_id uuid primary key,
    instance_id text not null,
    instance_epoch bigint not null,
    physical_host_id text not null,
    region text not null,
    active_matches integer not null check (active_matches >= 0),
    fleet_capacity integer not null check (fleet_capacity > 0),
    admission_requests bigint not null check (admission_requests >= 0),
    admission_rejections bigint not null check (admission_rejections >= 0),
    sampled_at timestamptz not null default now()
);
create index if not exists idx_trnm_online_capacity_recent
    on trnm_online_capacity_samples(sampled_at desc, instance_id);

create table if not exists trnm_online_moderation_shifts (
    shift_id uuid primary key,
    moderator_id text not null,
    status text not null default 'active' check (status in ('active', 'closed', 'expired')),
    starts_at timestamptz not null default now(),
    ends_at timestamptz not null,
    last_heartbeat_at timestamptz not null default now(),
    note text not null,
    closed_at timestamptz,
    close_note text
);
create unique index if not exists idx_trnm_online_moderation_one_active_shift
    on trnm_online_moderation_shifts(moderator_id) where status = 'active';

create table if not exists trnm_online_moderation_case_claims (
    claim_id uuid primary key,
    shift_id uuid not null references trnm_online_moderation_shifts(shift_id),
    case_kind text not null check (case_kind in ('report', 'appeal')),
    case_id uuid not null,
    status text not null default 'claimed' check (status in ('claimed', 'resolved', 'released')),
    claimed_at timestamptz not null default now(),
    resolved_at timestamptz,
    unique (case_kind, case_id)
);
create index if not exists idx_trnm_online_moderation_shift_claims
    on trnm_online_moderation_case_claims(shift_id, status);

create table if not exists trnm_online_host_attestation_audit (
    attestation_id uuid primary key,
    instance_id text not null,
    instance_epoch bigint not null,
    physical_host_id text not null,
    region text not null,
    challenge_hash text not null check (length(challenge_hash) = 64),
    evidence_hash text not null check (length(evidence_hash) = 64),
    observed_at timestamptz not null default now()
);
create index if not exists idx_trnm_online_host_attestation_recent
    on trnm_online_host_attestation_audit(physical_host_id, observed_at desc);
