-- Online Production v1: isolated signer receipts, host identity, season automation,
-- delayed spectating and moderation escalation observability.

create table if not exists trnm_entitlement_signing_receipts (
    request_id text primary key,
    request_hash text not null check (length(request_hash) = 64),
    signing_receipt_hash text not null check (length(signing_receipt_hash) = 64),
    key_id text not null,
    issuer text not null,
    signature text not null,
    entitlement_json jsonb not null,
    created_at timestamptz not null default now()
);

alter table trnm_online_fleet_instances
    add column if not exists physical_host_id text not null default 'legacy-local-host';
alter table trnm_online_matches
    add column if not exists assigned_physical_host_id text;
alter table trnm_online_fleet_failovers
    add column if not exists previous_physical_host_id text;
alter table trnm_online_fleet_failovers
    add column if not exists new_physical_host_id text;
create index if not exists idx_trnm_online_fleet_physical_host
    on trnm_online_fleet_instances(physical_host_id, status, lease_expires_at);

alter table trnm_online_seasons
    add column if not exists automatic_activation boolean not null default false;
alter table trnm_online_seasons
    add column if not exists automation_state text not null default 'manual';
alter table trnm_online_seasons
    add column if not exists automation_deferred_reason text;
alter table trnm_online_seasons
    add column if not exists last_automation_attempt_at timestamptz;
alter table trnm_online_seasons drop constraint if exists trnm_online_seasons_automation_state_check;
alter table trnm_online_seasons add constraint trnm_online_seasons_automation_state_check
    check (automation_state in ('manual', 'scheduled', 'deferred', 'activated'));

create table if not exists trnm_online_season_automation_audit (
    audit_id uuid primary key,
    season_id text not null references trnm_online_seasons(season_id),
    action text not null check (action in ('configure', 'deferred', 'activate')),
    previous_active_season_id text,
    detail jsonb not null,
    created_at timestamptz not null default now()
);

create table if not exists trnm_online_spectator_invites (
    invite_id uuid primary key,
    match_id uuid not null references trnm_online_matches(match_id) on delete cascade,
    creator_player_id text not null,
    target_player_id text not null,
    token_hash text not null unique check (length(token_hash) = 64),
    delay_seconds integer not null check (delay_seconds between 30 and 600),
    expires_at timestamptz not null,
    consumed_at timestamptz,
    created_at timestamptz not null default now()
);

create table if not exists trnm_online_spectator_grants (
    grant_id uuid primary key,
    invite_id uuid not null unique references trnm_online_spectator_invites(invite_id),
    match_id uuid not null references trnm_online_matches(match_id) on delete cascade,
    viewer_player_id text not null,
    viewer_account_id uuid not null,
    delay_seconds integer not null check (delay_seconds between 30 and 600),
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    unique (match_id, viewer_player_id)
);
create index if not exists idx_trnm_online_spectator_grant_viewer
    on trnm_online_spectator_grants(viewer_player_id, expires_at);

create table if not exists trnm_online_appeal_escalations (
    escalation_id uuid primary key,
    appeal_id uuid not null unique references trnm_online_enforcement_appeals(appeal_id),
    escalation_kind text not null check (escalation_kind in ('sla_overdue')),
    status text not null default 'open' check (status in ('open', 'acknowledged', 'closed')),
    detail jsonb not null,
    created_at timestamptz not null default now(),
    acknowledged_at timestamptz,
    closed_at timestamptz
);
