create table if not exists trnm_online_lobbies (
    lobby_id uuid primary key,
    display_name text not null check (length(display_name) between 1 and 80),
    owner_player_id text not null,
    owner_account_id uuid not null,
    status text not null default 'open' check (status in ('open', 'queued', 'matched', 'closed')),
    lobby_revision bigint not null default 0 check (lobby_revision >= 0),
    map_id text not null,
    queue_mode text not null default 'coop_vs_ai' check (queue_mode = 'coop_vs_ai'),
    match_id uuid references trnm_online_matches(match_id),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists trnm_online_lobby_members (
    lobby_id uuid not null references trnm_online_lobbies(lobby_id) on delete cascade,
    player_id text not null,
    account_id uuid not null,
    campaign_id text not null references trnm_online_campaigns(campaign_id),
    member_role text not null check (member_role in ('owner', 'member')),
    ready boolean not null default false,
    joined_at timestamptz not null default now(),
    primary key (lobby_id, player_id),
    unique (lobby_id, account_id),
    unique (lobby_id, member_role)
);

create table if not exists trnm_online_lobby_invites (
    invite_id uuid primary key,
    lobby_id uuid not null references trnm_online_lobbies(lobby_id) on delete cascade,
    inviter_player_id text not null,
    target_player_id text not null,
    invite_token_hash text not null unique check (length(invite_token_hash) = 64),
    status text not null default 'pending' check (status in ('pending', 'accepted', 'revoked', 'expired')),
    expires_at timestamptz not null,
    created_at timestamptz not null default now(),
    accepted_at timestamptz
);

create unique index if not exists idx_trnm_online_pending_invite_target
    on trnm_online_lobby_invites(lobby_id, target_player_id) where status = 'pending';

create table if not exists trnm_online_matchmaking_allocations (
    allocation_id uuid primary key,
    lobby_id uuid not null unique references trnm_online_lobbies(lobby_id),
    match_id uuid not null unique references trnm_online_matches(match_id),
    queue_mode text not null check (queue_mode = 'coop_vs_ai'),
    member_count integer not null check (member_count = 2),
    allocated_at timestamptz not null default now()
);
