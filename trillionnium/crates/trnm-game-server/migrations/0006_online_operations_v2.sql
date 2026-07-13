-- Online Operations v2: fenced fleet leases, replay frames, season control and appeals.

alter table trnm_online_fleet_instances
    add column if not exists instance_epoch bigint not null default 0;
alter table trnm_online_fleet_instances
    add column if not exists lease_expires_at timestamptz not null default now();
alter table trnm_online_fleet_instances
    add column if not exists drain_reason text;

alter table trnm_online_matches
    add column if not exists assigned_instance_epoch bigint not null default 0;
alter table trnm_online_matches
    add column if not exists initial_simulation_json jsonb;
alter table trnm_online_matches
    add column if not exists season_id text references trnm_online_seasons(season_id);

alter table trnm_online_fleet_failovers
    add column if not exists previous_instance_epoch bigint;
alter table trnm_online_fleet_failovers
    add column if not exists new_instance_epoch bigint;
do $$
declare existing_constraint record;
begin
    for existing_constraint in
        select conname from pg_constraint
        where conrelid = 'trnm_online_fleet_failovers'::regclass
          and contype = 'u'
          and pg_get_constraintdef(oid) like '%previous_instance_id, new_instance_id%'
    loop
        execute format('alter table trnm_online_fleet_failovers drop constraint %I',
                       existing_constraint.conname);
    end loop;
end $$;
create unique index if not exists idx_trnm_online_fleet_failover_epoch
    on trnm_online_fleet_failovers (
        match_id, previous_instance_id, previous_instance_epoch,
        new_instance_id, new_instance_epoch
    );

alter table trnm_online_replay_index
    add column if not exists final_snapshot_hash text;
alter table trnm_online_replay_index drop constraint if exists trnm_online_replay_final_hash_check;
alter table trnm_online_replay_index add constraint trnm_online_replay_final_hash_check
    check (final_snapshot_hash is null or length(final_snapshot_hash) = 64);

create table if not exists trnm_online_replay_frames (
    match_id uuid not null references trnm_online_matches(match_id) on delete cascade,
    tick bigint not null check (tick >= 0),
    snapshot_hash text not null check (length(snapshot_hash) = 64),
    simulation_json jsonb not null,
    frame_kind text not null check (frame_kind in ('initial', 'checkpoint', 'terminal')),
    created_at timestamptz not null default now(),
    primary key (match_id, tick)
);

create table if not exists trnm_online_season_admin_audit (
    audit_id uuid primary key,
    action text not null check (action in ('create', 'activate', 'close')),
    season_id text not null,
    previous_active_season_id text,
    detail jsonb not null,
    created_at timestamptz not null default now()
);

create table if not exists trnm_online_season_snapshots (
    season_id text not null references trnm_online_seasons(season_id),
    player_id text not null,
    final_rank integer not null check (final_rank > 0),
    rating integer not null,
    wins integer not null,
    losses integer not null,
    matches integer not null,
    captured_at timestamptz not null default now(),
    primary key (season_id, player_id)
);

create table if not exists trnm_online_enforcement_appeals (
    appeal_id uuid primary key,
    enforcement_id uuid not null references trnm_online_enforcements(enforcement_id),
    player_id text not null,
    account_id uuid not null,
    detail text not null check (length(detail) between 20 and 2000),
    status text not null default 'pending'
        check (status in ('pending', 'approved', 'rejected')),
    resolution text,
    created_at timestamptz not null default now(),
    due_at timestamptz not null default (now() + interval '72 hours'),
    resolved_at timestamptz,
    unique (enforcement_id, player_id)
);
create index if not exists idx_trnm_online_enforcement_appeal_queue
    on trnm_online_enforcement_appeals(status, due_at);

create table if not exists trnm_online_fleet_admin_audit (
    audit_id uuid primary key,
    instance_id text not null,
    action text not null check (action in ('activate', 'drain', 'offline')),
    reason text not null,
    created_at timestamptz not null default now()
);
