alter table trnm_online_match_members
    add column if not exists campaign_id text references trnm_online_campaigns(campaign_id);
alter table trnm_online_match_members
    add column if not exists settlement_seed_json jsonb;
alter table trnm_online_match_members
    add column if not exists unit_id_map jsonb not null default '{}'::jsonb;
alter table trnm_online_match_members
    add column if not exists reconnect_count bigint not null default 0 check (reconnect_count >= 0);
alter table trnm_online_match_members
    add column if not exists last_acknowledged_sequence bigint not null default 0 check (last_acknowledged_sequence >= 0);
alter table trnm_online_match_members
    add column if not exists last_snapshot_hash text not null default '';
alter table trnm_online_match_members
    add column if not exists last_seen_at timestamptz not null default now();

create table if not exists trnm_online_progression_events (
    event_id text primary key,
    match_id uuid not null references trnm_online_matches(match_id) on delete cascade,
    player_id text not null,
    account_id uuid not null,
    campaign_id text not null references trnm_online_campaigns(campaign_id),
    result_hash text not null check (length(result_hash) = 64),
    experience_delta bigint not null check (experience_delta >= 0),
    reputation_delta integer not null,
    inventory_delta jsonb not null,
    campaign_revision bigint not null check (campaign_revision >= 0),
    created_at timestamptz not null default now(),
    unique (match_id, player_id),
    unique (match_id, account_id)
);

create index if not exists idx_trnm_online_progression_campaign
    on trnm_online_progression_events(campaign_id, created_at);
