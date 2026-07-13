-- Online Product v2: solo ranked PvP, rating provenance and minimum social/moderation state.

alter table trnm_online_lobbies drop constraint if exists trnm_online_lobbies_queue_mode_check;
alter table trnm_online_lobbies add constraint trnm_online_lobbies_queue_mode_check
    check (queue_mode in ('coop_vs_ai', 'ranked_pvp'));

alter table trnm_online_matchmaking_allocations
    drop constraint if exists trnm_online_matchmaking_allocations_queue_mode_check;
alter table trnm_online_matchmaking_allocations
    add constraint trnm_online_matchmaking_allocations_queue_mode_check
    check (queue_mode in ('coop_vs_ai', 'ranked_pvp'));

alter table trnm_online_matches
    add column if not exists match_mode text not null default 'coop_vs_ai';
alter table trnm_online_matches drop constraint if exists trnm_online_matches_match_mode_check;
alter table trnm_online_matches add constraint trnm_online_matches_match_mode_check
    check (match_mode in ('coop_vs_ai', 'ranked_pvp'));

create table if not exists trnm_online_ratings (
    player_id text primary key,
    account_id uuid not null,
    rating integer not null default 1000 check (rating between 0 and 5000),
    wins integer not null default 0 check (wins >= 0),
    losses integer not null default 0 check (losses >= 0),
    provisional_matches integer not null default 0 check (provisional_matches >= 0),
    updated_at timestamptz not null default now()
);

create table if not exists trnm_online_solo_queue (
    ticket_id uuid primary key,
    player_id text not null,
    account_id uuid not null,
    campaign_id text not null references trnm_online_campaigns(campaign_id),
    map_id text not null,
    queue_mode text not null default 'ranked_pvp' check (queue_mode = 'ranked_pvp'),
    status text not null default 'queued' check (status in ('queued', 'matched', 'cancelled')),
    rating_at_join integer not null check (rating_at_join between 0 and 5000),
    matched_lobby_id uuid references trnm_online_lobbies(lobby_id),
    match_id uuid references trnm_online_matches(match_id),
    opponent_player_id text,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create unique index if not exists idx_trnm_online_one_active_solo_ticket
    on trnm_online_solo_queue(player_id) where status = 'queued';
create index if not exists idx_trnm_online_solo_pairing
    on trnm_online_solo_queue(queue_mode, map_id, rating_at_join, created_at)
    where status = 'queued';

create table if not exists trnm_online_rating_events (
    event_id uuid primary key,
    match_id uuid not null references trnm_online_matches(match_id),
    player_id text not null,
    opponent_player_id text not null,
    result text not null check (result in ('win', 'loss')),
    rating_before integer not null check (rating_before between 0 and 5000),
    rating_after integer not null check (rating_after between 0 and 5000),
    rating_delta integer not null,
    result_hash text not null check (length(result_hash) = 64),
    created_at timestamptz not null default now(),
    unique (match_id, player_id)
);

create table if not exists trnm_online_friendships (
    requester_player_id text not null,
    target_player_id text not null,
    status text not null default 'pending' check (status in ('pending', 'accepted', 'rejected')),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    primary key (requester_player_id, target_player_id),
    check (requester_player_id <> target_player_id)
);

create table if not exists trnm_online_blocks (
    blocker_player_id text not null,
    blocked_player_id text not null,
    created_at timestamptz not null default now(),
    primary key (blocker_player_id, blocked_player_id),
    check (blocker_player_id <> blocked_player_id)
);

create table if not exists trnm_online_reports (
    report_id uuid primary key,
    reporter_player_id text not null,
    target_player_id text not null,
    match_id uuid not null references trnm_online_matches(match_id),
    category text not null check (category in ('cheating', 'harassment', 'griefing', 'name', 'other')),
    detail text not null check (length(detail) between 10 and 2000),
    status text not null default 'open' check (status in ('open', 'reviewed', 'actioned', 'dismissed')),
    resolution text,
    created_at timestamptz not null default now(),
    resolved_at timestamptz,
    unique (reporter_player_id, target_player_id, match_id, category),
    check (reporter_player_id <> target_player_id),
    check ((status = 'open' and resolution is null and resolved_at is null)
        or (status <> 'open' and resolution is not null and resolved_at is not null))
);

create index if not exists idx_trnm_online_reports_triage
    on trnm_online_reports(status, created_at);
