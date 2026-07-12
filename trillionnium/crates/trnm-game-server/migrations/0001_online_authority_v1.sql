create table if not exists trnm_online_campaigns (
    campaign_id text primary key,
    player_id text not null,
    account_id uuid not null,
    slot_key text not null,
    campaign_revision bigint not null check (campaign_revision >= 0),
    schema_revision integer not null check (schema_revision >= 12),
    state_hash text not null check (length(state_hash) = 64),
    campaign_json jsonb not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    unique (account_id, slot_key)
);

create table if not exists trnm_online_matches (
    match_id uuid primary key,
    campaign_id text not null references trnm_online_campaigns(campaign_id),
    host_player_id text not null,
    host_account_id uuid not null,
    join_code text not null unique,
    phase text not null check (phase in ('waiting', 'running', 'complete', 'failed_closed')),
    build_id text not null,
    map_id text not null,
    rules_version text not null,
    seed_hash text not null default '',
    seed_json jsonb,
    simulation_json jsonb,
    result_json jsonb,
    result_hash text,
    snapshot_hash text not null default '',
    authoritative_tick bigint not null default 0 check (authoritative_tick >= 0),
    next_sequence bigint not null default 0 check (next_sequence >= 0),
    match_revision bigint not null default 0 check (match_revision >= 0),
    settlement_state text not null default 'not_ready'
        check (settlement_state in ('not_ready', 'pending', 'settled', 'failed_closed')),
    failure_reason text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists trnm_online_match_members (
    match_id uuid not null references trnm_online_matches(match_id) on delete cascade,
    player_id text not null,
    account_id uuid not null,
    member_role text not null check (member_role in ('host', 'coop_guest')),
    controlled_unit_ids jsonb not null default '[]'::jsonb,
    joined_at timestamptz not null default now(),
    primary key (match_id, player_id),
    unique (match_id, account_id),
    unique (match_id, member_role)
);

create table if not exists trnm_online_commands (
    match_id uuid not null references trnm_online_matches(match_id) on delete cascade,
    sequence bigint not null check (sequence >= 0),
    command_id text not null,
    player_id text not null,
    request_hash text,
    target_tick bigint not null check (target_tick >= 0),
    order_json jsonb not null,
    accepted_snapshot_hash text not null check (length(accepted_snapshot_hash) = 64),
    accepted_match_revision bigint not null check (accepted_match_revision >= 0),
    created_at timestamptz not null default now(),
    primary key (match_id, sequence),
    unique (match_id, command_id)
);

alter table trnm_online_commands
    add column if not exists request_hash text;

create index if not exists idx_trnm_online_running_matches
    on trnm_online_matches(phase, updated_at) where phase = 'running';
create index if not exists idx_trnm_online_pending_settlement
    on trnm_online_matches(settlement_state, updated_at) where settlement_state = 'pending';
