-- Online Operations v1: seasons, replay provenance, integrity/moderation and fleet ownership.

alter table trnm_online_solo_queue add column if not exists device_hash text;
alter table trnm_online_solo_queue drop constraint if exists trnm_online_solo_queue_device_hash_check;
alter table trnm_online_solo_queue add constraint trnm_online_solo_queue_device_hash_check
    check (device_hash is null or length(device_hash) = 64);

alter table trnm_online_matches add column if not exists assigned_instance_id text;
alter table trnm_online_matches add column if not exists assigned_region text;

create table if not exists trnm_online_seasons (
    season_id text primary key,
    display_name text not null check (length(display_name) between 3 and 80),
    status text not null check (status in ('scheduled', 'active', 'closed')),
    rules_version text not null,
    starts_at timestamptz not null,
    ends_at timestamptz not null,
    created_at timestamptz not null default now(),
    check (ends_at > starts_at)
);
create unique index if not exists idx_trnm_online_one_active_season
    on trnm_online_seasons(status) where status = 'active';

insert into trnm_online_seasons (
    season_id, display_name, status, rules_version, starts_at, ends_at
) values (
    'season-2026-prealpha-1', '2026 Pre-Alpha Season 1', 'active',
    'trnm_ranked_rules_2026_07_v1', now() - interval '1 day', now() + interval '89 days'
) on conflict (season_id) do nothing;

create table if not exists trnm_online_season_ratings (
    season_id text not null references trnm_online_seasons(season_id),
    player_id text not null,
    account_id uuid not null,
    rating integer not null default 1000 check (rating between 0 and 5000),
    wins integer not null default 0 check (wins >= 0),
    losses integer not null default 0 check (losses >= 0),
    matches integer not null default 0 check (matches >= 0),
    updated_at timestamptz not null default now(),
    primary key (season_id, player_id)
);
create index if not exists idx_trnm_online_season_leaderboard
    on trnm_online_season_ratings(season_id, rating desc, wins desc, player_id);

alter table trnm_online_rating_events add column if not exists season_id text
    references trnm_online_seasons(season_id);
alter table trnm_online_rating_events add column if not exists integrity_state text
    not null default 'clear';
alter table trnm_online_rating_events drop constraint if exists trnm_online_rating_events_integrity_check;
alter table trnm_online_rating_events add constraint trnm_online_rating_events_integrity_check
    check (integrity_state in ('clear', 'under_review', 'voided'));

create table if not exists trnm_online_replay_index (
    match_id uuid primary key references trnm_online_matches(match_id),
    season_id text references trnm_online_seasons(season_id),
    result_hash text not null check (length(result_hash) = 64),
    replay_hash text not null check (length(replay_hash) = 64),
    command_count integer not null check (command_count >= 0),
    map_id text not null,
    build_id text not null,
    participant_ids jsonb not null,
    created_at timestamptz not null default now()
);

create table if not exists trnm_online_integrity_signals (
    signal_id uuid primary key,
    match_id uuid references trnm_online_matches(match_id),
    player_ids jsonb not null,
    signal_kind text not null check (signal_kind in (
        'repeat_opponent', 'shared_device', 'queue_coordination', 'manual_review'
    )),
    severity text not null check (severity in ('low', 'medium', 'high')),
    evidence jsonb not null,
    status text not null default 'open' check (status in ('open', 'reviewed', 'confirmed', 'dismissed')),
    created_at timestamptz not null default now(),
    resolved_at timestamptz
);
create index if not exists idx_trnm_online_integrity_triage
    on trnm_online_integrity_signals(status, severity, created_at);

alter table trnm_online_reports add column if not exists replay_hash text;
alter table trnm_online_reports add column if not exists season_id text
    references trnm_online_seasons(season_id);
alter table trnm_online_reports add column if not exists integrity_signal_id uuid
    references trnm_online_integrity_signals(signal_id);
alter table trnm_online_reports drop constraint if exists trnm_online_reports_replay_hash_check;
alter table trnm_online_reports add constraint trnm_online_reports_replay_hash_check
    check (replay_hash is null or length(replay_hash) = 64);

create table if not exists trnm_online_enforcements (
    enforcement_id uuid primary key,
    player_id text not null,
    scope text not null check (scope in ('ranked', 'online')),
    reason text not null check (length(reason) between 10 and 2000),
    source_report_id uuid references trnm_online_reports(report_id),
    starts_at timestamptz not null default now(),
    expires_at timestamptz not null,
    revoked_at timestamptz,
    created_at timestamptz not null default now(),
    check (expires_at > starts_at)
);
create index if not exists idx_trnm_online_active_enforcement
    on trnm_online_enforcements(player_id, scope, expires_at) where revoked_at is null;

create table if not exists trnm_online_moderation_audit (
    audit_id uuid primary key,
    report_id uuid references trnm_online_reports(report_id),
    action text not null,
    target_player_id text,
    resolution text not null,
    created_at timestamptz not null default now()
);

create table if not exists trnm_online_fleet_instances (
    instance_id text primary key,
    region text not null,
    public_endpoint text not null,
    build_id text not null,
    capacity integer not null check (capacity between 1 and 10000),
    status text not null check (status in ('active', 'draining', 'offline')),
    active_matches integer not null default 0 check (active_matches >= 0),
    started_at timestamptz not null default now(),
    heartbeat_at timestamptz not null default now()
);
create index if not exists idx_trnm_online_fleet_route
    on trnm_online_fleet_instances(region, status, heartbeat_at);

create table if not exists trnm_online_fleet_failovers (
    failover_id uuid primary key,
    match_id uuid not null references trnm_online_matches(match_id),
    previous_instance_id text,
    new_instance_id text not null,
    previous_region text,
    new_region text not null,
    reason text not null,
    created_at timestamptz not null default now(),
    unique (match_id, previous_instance_id, new_instance_id)
);
