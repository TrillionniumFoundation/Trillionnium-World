use axum::extract::DefaultBodyLimit;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cex::CexClient;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use published_tick_journal::{
    PublishedTickAbandonmentTombstone, PublishedTickAbandonmentTombstoneInput,
    PublishedTickAckTombstone, PublishedTickAckTombstoneInput, PublishedTickColdWitness,
    PublishedTickHighWater, PublishedTickJournal, PublishedTickRecordInput,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{connection::Connection, executor::Executor, row::Row};
use sqlx_postgres::{PgConnection, PgPool, PgPoolOptions, Postgres};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    net::SocketAddr,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::sync::{mpsc, oneshot, watch, Mutex as AsyncMutex, RwLock};
use tokio::time::{Instant, MissedTickBehavior};
use trnm_campaign_core::{
    BattleMapSeedV1, BattleOutcome, BattleResultV1, BattleSeedV1, CampaignMission, CampaignSaveV1,
    UnitBattleReportV1, UnitBattleStatus,
};
use trnm_online_protocol::{
    validate_client_contract, validate_product_contract, OnlineAuthorityError,
    OnlineCampaignConnectRequest, OnlineCampaignView, OnlineCommandReceipt,
    OnlineCommandSubmitRequest, OnlineInventoryStack, OnlineLobbyAccessRequest,
    OnlineLobbyCreateRequest, OnlineLobbyInviteAcceptRequest, OnlineLobbyInviteReceipt,
    OnlineLobbyInviteRequest, OnlineLobbyMemberView, OnlineLobbyQueueRequest,
    OnlineLobbyReadyRequest, OnlineLobbyStatus, OnlineLobbyView, OnlineMatchAccessRequest,
    OnlineMatchCreateRequest, OnlineMatchJoinRequest, OnlineMatchMemberView, OnlineMatchPhase,
    OnlineMatchStartRequest, OnlineMatchView, OnlineMatchmakingReceipt, OnlineReconnectRequest,
    OnlineReconnectResponse, OnlineSnapshotResponse, ONLINE_AUTHORITY_BUILD,
    ONLINE_AUTHORITY_PROTOCOL, ONLINE_AUTHORITY_V2_PROTOCOL, ONLINE_PRODUCT_BUILD,
    ONLINE_PRODUCT_PROTOCOL,
};
use trnm_rts_protocol::RtsOrderSource;
use trnm_rts_sim::{MissionSimV1, TICKS_PER_SECOND};
use uuid::Uuid;

const PLAYER_SESSION_HEADER: &str = "x-trnm-player-session";
const MIGRATION_V1: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0001_online_authority_v1.sql"
));
const MIGRATION_V2: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0002_online_authority_v2.sql"
));
const MIGRATION_V3: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0003_online_product_v1.sql"
));
const MIGRATION_V4: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0004_online_product_v2.sql"
));
const MIGRATION_V5: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0005_online_operations_v1.sql"
));
const MIGRATION_V6: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0006_online_operations_v2.sql"
));
const MIGRATION_V7: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0007_online_production_v1.sql"
));
const MIGRATION_V8: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0008_online_production_v2.sql"
));
const MIGRATION_V9: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0009_online_realtime_actor_v1.sql"
));
const MIGRATION_V10: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0010_online_realtime_input_v1.sql"
));
const MIGRATION_V11: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0011_online_terminal_publication_ack_v1.sql"
));
const MIGRATION_V12: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0012_online_terminal_staging_v1.sql"
));
const MIGRATION_V13: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0013_online_failed_closed_abandonment_v1.sql"
));
const MIGRATION_V14: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0014_online_command_commit_rpc_v1.sql"
));
const MIGRATION_V15: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0015_online_realtime_hot_path_v1.sql"
));
const MIGRATION_V16: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0016_online_settlement_outbox_v1.sql"
));
const MIGRATION_V17: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0017_online_settlement_worker_runtime_v1.sql"
));
const MIGRATION_V18: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0018_online_settlement_operator_controls_v1.sql"
));
const MIGRATION_V19: &str = include_str!(concat!(
    ::std::env!("CARGO_MANIFEST_DIR"),
    "/migrations/0019_online_settlement_quarantine_v1.sql"
));
const MIGRATION_ADVISORY_LOCK: i64 = 0x5452_4e4d_4f4e_4c49;
const MIGRATION_LEDGER_DDL: &str =
    "create table if not exists public.trnm_online_schema_migrations (
    migration_version integer primary key check (migration_version > 0),
    migration_name text not null unique check (btrim(migration_name) <> ''),
    checksum_sha256 text not null check (checksum_sha256 ~ '^[0-9a-f]{64}$'),
    applied_at timestamptz not null default now()
)";
const DATABASE_HOST_LEADER_LOCK_SALT: i64 = 0x5452_4e4d_4c45_4144;
const DATABASE_HOST_BARRIER_LOCK_SALT: i64 = 0x5452_4e4d_4241_5252;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    readiness_pool: PgPool,
    cex: CexClient,
    asset_root: Arc<PathBuf>,
    moderator_token: Arc<String>,
    instance_id: Arc<String>,
    region: Arc<String>,
    public_endpoint: Arc<String>,
    physical_host_id: Arc<String>,
    capacity: i32,
    instance_epoch: i64,
    rate_limit_per_minute: u32,
    request_body_limit_bytes: u32,
    tick_interval: Duration,
    accelerated_test_clock: bool,
    authority_clock: Arc<AuthorityClockTelemetry>,
    published_tick_journal: PublishedTickJournal,
    terminal_acknowledged_high_waters: Arc<RwLock<BTreeSet<Uuid>>>,
    failed_closed_high_waters: Arc<RwLock<BTreeSet<Uuid>>>,
    match_actors: Arc<RwLock<MatchActorRegistry>>,
    stream_connections: Arc<Mutex<StreamConnectionRegistry>>,
    database_host_authority: Arc<DatabaseHostAuthorityFence>,
    fatal_shutdown: watch::Sender<bool>,
    draining: watch::Sender<bool>,
    shutdown: watch::Sender<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DatabaseHostAuthorityIdentity {
    physical_host_id: String,
    owner_nonce: Uuid,
    application_name: String,
    backend_pid: i32,
    backend_started_at: DateTime<Utc>,
    database_system_identifier: String,
    database_timeline_id: i64,
    database_postmaster_started_at: DateTime<Utc>,
    leader_lock_key: i64,
    barrier_lock_key: i64,
}

struct DatabaseHostAuthorityFence {
    identity: DatabaseHostAuthorityIdentity,
    connection: AsyncMutex<PgConnection>,
    healthy: AtomicBool,
    last_success: Mutex<Instant>,
}

impl DatabaseHostAuthorityFence {
    fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Acquire)
    }

    fn record_success(&self, observed_at: Instant) {
        let Ok(mut last_success) = self.last_success.lock() else {
            self.healthy.store(false, Ordering::Release);
            return;
        };
        *last_success = monotonic_database_host_authority_success(*last_success, observed_at);
    }

    fn last_success_age(&self, now: Instant) -> Option<Duration> {
        self.last_success
            .lock()
            .ok()
            .and_then(|last_success| now.checked_duration_since(*last_success))
    }

    fn is_fresh(&self, now: Instant) -> bool {
        self.last_success.lock().is_ok_and(|last_success| {
            database_host_authority_success_is_fresh(
                self.is_healthy(),
                *last_success,
                now,
                DATABASE_HOST_FENCE_FRESHNESS,
            )
        })
    }
}

fn monotonic_database_host_authority_success(previous: Instant, candidate: Instant) -> Instant {
    if candidate > previous {
        candidate
    } else {
        previous
    }
}

fn database_host_authority_success_is_fresh(
    healthy: bool,
    last_success: Instant,
    now: Instant,
    maximum_age: Duration,
) -> bool {
    healthy
        && now
            .checked_duration_since(last_success)
            .is_some_and(|age| age <= maximum_age)
}

#[derive(Default)]
struct MatchActorRegistry {
    actors: BTreeMap<Uuid, MatchActorHandle>,
    initializing: BTreeMap<Uuid, MatchActorInitialization>,
}

#[derive(Clone)]
struct MatchActorInitialization {
    token: Uuid,
    ready: watch::Sender<bool>,
    kind: MatchActorInitializationKind,
    started_at: Instant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MatchActorInitializationKind {
    Actor,
    TerminalRecovery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MatchActorLifecycle {
    Warming { started_at: Instant },
    Running,
    Terminalizing { started_at: Instant },
}

/// Releases a per-match initialization reservation if the request future is
/// cancelled while it is outside the registry lock. Without this guard, a
/// disconnected HTTP request could strand `initializing` forever and consume
/// fleet capacity even though no actor can ever be installed.
struct MatchActorInitializationReservation {
    registry: Arc<RwLock<MatchActorRegistry>>,
    match_id: Uuid,
    token: Uuid,
    ready: watch::Sender<bool>,
    armed: bool,
}

impl MatchActorInitializationReservation {
    fn new(
        registry: Arc<RwLock<MatchActorRegistry>>,
        match_id: Uuid,
        initialization: &MatchActorInitialization,
    ) -> Self {
        Self {
            registry,
            match_id,
            token: initialization.token,
            ready: initialization.ready.clone(),
            armed: true,
        }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for MatchActorInitializationReservation {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let registry = self.registry.clone();
        let match_id = self.match_id;
        let token = self.token;
        let ready = self.ready.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            drop(runtime.spawn(async move {
                let mut registry = registry.write().await;
                if registry
                    .initializing
                    .get(&match_id)
                    .is_some_and(|current| current.token == token)
                {
                    registry.initializing.remove(&match_id);
                }
                ready.send_replace(true);
            }));
        }
    }
}

struct InitializedMatchActor {
    actor_id: Uuid,
    handle: MatchActorHandle,
    loaded: LoadedMatchActor,
    commands: mpsc::Receiver<ActorCommandEnvelope>,
    published: watch::Sender<PublishedMatchState>,
    publication_acked: watch::Sender<ActorPublicationCursor>,
    lifecycle: watch::Sender<MatchActorLifecycle>,
}

enum MatchActorEnsureDecision {
    Existing(MatchActorHandle),
    Wait(watch::Receiver<bool>),
    Initialize(MatchActorInitialization),
}

#[derive(Default)]
struct StreamConnectionRegistry {
    total: usize,
    by_member_match: BTreeMap<String, usize>,
}

#[derive(Default)]
struct AuthorityClockTelemetry {
    window: Mutex<AuthorityClockWindow>,
}

#[derive(Default)]
struct AuthorityClockWindow {
    started_at: Option<Instant>,
    wake_count: u64,
    samples: VecDeque<AuthorityClockWake>,
}

#[derive(Clone, Copy)]
struct AuthorityClockWake {
    observed_at: Instant,
    lateness_ticks: f64,
}

struct AuthorityClockSnapshot {
    elapsed_ms: f64,
    wake_count: u64,
    cumulative_drift_ticks: f64,
    window_sample_count: usize,
    window_drift_ticks: Option<f64>,
    latest_lateness_ticks: Option<f64>,
    max_recent_lateness_ticks: Option<f64>,
    last_wake_age_ms: Option<f64>,
}

const MATCH_ACTOR_COMMAND_QUEUE: usize = 64;
const MATCH_COMMAND_PERSISTENCE_QUEUE: usize = 1;
const MATCH_ACTOR_PUBLICATION_COMPLETION_QUEUE: usize = 64;
const MATCH_ACTOR_RECEIPT_CACHE: usize = 256;
const MATCH_CHECKPOINT_QUEUE: usize = 2;
const MATCH_CHECKPOINT_INTERVAL_TICKS: u64 = 100;
const MATCH_ACTOR_FENCE_INTERVAL: Duration = Duration::from_secs(1);
const MATCH_ACTOR_WORKER_OPERATION_TIMEOUT: Duration = Duration::from_secs(6);
const MATCH_ACTOR_WORKER_JOIN_TIMEOUT: Duration = Duration::from_secs(2);
const MATCH_ACTOR_WORKER_ABORT_TIMEOUT: Duration = Duration::from_secs(1);
const MATCH_ACTOR_INITIALIZATION_TIMEOUT: Duration = Duration::from_secs(10);
const MATCH_ACTOR_TERMINAL_TRANSITION_TIMEOUT: Duration = Duration::from_secs(20);
const MATCH_ACTOR_PUBLICATION_FRESHNESS: Duration = Duration::from_millis(750);
const MATCH_ACTOR_CLOCK_WARMUP_EXTRA_TICKS: u32 = 2;
const MATCH_ACTOR_CLOCK_FIRST_WAKE_GRACE_TICKS: u32 = 2;
const INITIAL_AUTHORITY_RECOVERY_TIMEOUT: Duration = Duration::from_secs(30);
const DATABASE_STATEMENT_TIMEOUT: Duration = Duration::from_secs(5);
const DATABASE_LOCK_TIMEOUT: Duration = Duration::from_secs(2);
const DATABASE_HOST_HANDOFF_TIMEOUT: Duration = Duration::from_secs(10);
const DATABASE_HOST_HANDOFF_POLL_INTERVAL: Duration = Duration::from_millis(25);
const DATABASE_HOST_FENCE_MONITOR_INTERVAL: Duration = Duration::from_millis(250);
const DATABASE_HOST_FENCE_MONITOR_TIMEOUT: Duration = Duration::from_secs(2);
const DATABASE_HOST_FENCE_FRESHNESS: Duration = Duration::from_secs(1);
const DATABASE_MIGRATION_STATEMENT_TIMEOUT: Duration = Duration::from_secs(60);
const DATABASE_MIGRATION_LOCK_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS: f64 = 2.0;
const AUTHORITY_CLOCK_WINDOW_TICKS: usize = 20;
const AUTHORITY_CLOCK_MIN_SAMPLES: usize = 3;
const GAME_SERVER_DATABASE_MIN_CONNECTIONS: u32 = 12;
const GAME_SERVER_DATABASE_MAX_CONNECTIONS: u32 = 12;
const READINESS_DATABASE_MIN_CONNECTIONS: u32 = 4;
const READINESS_DATABASE_MAX_CONNECTIONS: u32 = 12;
const TERMINAL_ORPHAN_RECONCILIATION_CONCURRENCY: usize = 4;
const TERMINAL_ACK_GAP_SCAN_LIMIT: i64 = 256;
const TERMINAL_ACK_STARTUP_PAGE_SIZE: i64 = 512;
const ONLINE_COMMAND_COMMIT_V2_SQL: &str =
    "select * from public.trnm_online_commit_actor_command_v2(
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21, $22, $23, $24, $25, $26, $27
         )";
const ONLINE_HEARTBEAT_FLEET_V1_SQL: &str = "select public.trnm_online_heartbeat_fleet_v1(
            $1, $2, $3, $4, $5, $6, $7, $8,
            $9, $10, $11, $12, $13, $14, $15, $16
         )";
const ONLINE_CHECKPOINT_ACTOR_V1_SQL: &str = "select public.trnm_online_checkpoint_actor_v1(
            $1, $2, $3, $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14, $15, $16, $17, $18
         )";
const MATCH_ACTOR_FENCE_OWNERSHIP_SQL: &str = "select exists(
        select 1 from trnm_online_matches m
        join trnm_online_fleet_instances f
          on f.instance_id = m.assigned_instance_id
         and f.instance_epoch = m.assigned_instance_epoch
        where m.match_id = $1
          and m.assigned_instance_id = $2
          and m.assigned_instance_epoch = $3
          and m.assigned_physical_host_id = $4
          and f.physical_host_id = $4
          and f.status in ('active', 'draining')
          and f.lease_expires_at > now()
    )";
const DUPLICATE_COMMAND_RECEIPT_SQL: &str = "select
        c.sequence, c.input_sequence, c.player_id, c.request_hash,
        c.accepted_match_revision, c.accepted_snapshot_hash, c.target_tick,
        c.client_observed_tick,
        m.match_revision as current_match_revision,
        m.next_sequence as current_next_sequence,
        m.checkpoint_sequence, m.phase, m.assigned_physical_host_id,
        exists(
            select 1 from trnm_online_terminal_publication_acks a
            where a.match_id = m.match_id
              and m.phase = 'complete'
              and m.terminal_publication_state = 'acknowledged'
              and a.local_tombstone_state = 'sealed'
              and m.assigned_physical_host_id = $3
              and m.terminal_publication_actor_generation is not null
              and a.actor_generation = m.terminal_publication_actor_generation
              and a.instance_id = m.assigned_instance_id
              and a.actor_epoch = m.assigned_instance_epoch
              and a.physical_host_id = m.assigned_physical_host_id
              and a.authoritative_tick = m.authoritative_tick
              and a.next_sequence = m.next_sequence
              and a.match_revision = m.match_revision
              and a.snapshot_hash = m.snapshot_hash
              and a.phase = 'complete'
              and a.result_hash = m.result_hash
              and a.published_settlement_state = m.settlement_state
              and a.next_input_sequences = coalesce(
                  (select jsonb_object_agg(
                      mm.player_id,
                      mm.next_input_sequence order by mm.player_id
                   )
                   from trnm_online_match_members mm
                   where mm.match_id = m.match_id),
                  '{}'::jsonb
              )
        ) as terminal_publication_acked,
        (select mm.next_input_sequence from trnm_online_match_members mm
         where mm.match_id = c.match_id and mm.player_id = c.player_id)
           as current_member_input_sequence
 from trnm_online_commands c
 join trnm_online_matches m on m.match_id = c.match_id
 where c.match_id = $1 and c.command_id = $2";
const DATABASE_LINEAGE_SQL: &str = "select
        system.system_identifier::text as system_identifier,
        checkpoint.timeline_id::integer as timeline_id,
        pg_current_wal_flush_lsn()::text as wal_flush_lsn
      from pg_control_system() system
      cross join pg_control_checkpoint() checkpoint";
const LOCAL_TERMINAL_ACK_GAPS_SQL: &str = "select m.match_id
       from trnm_online_matches m
      where m.phase = 'complete'
        and m.assigned_physical_host_id = $1
        and m.terminal_publication_state <> 'legacy_quarantined'
        and not exists (
            select 1
              from trnm_online_terminal_publication_acks a
             where a.match_id = m.match_id
               and m.checkpoint_sequence = m.next_sequence
               and m.result_hash is not null
               and m.settlement_state in ('pending', 'settled')
               and m.terminal_publication_state = 'acknowledged'
               and a.local_tombstone_state = 'sealed'
               and m.terminal_publication_actor_generation is not null
               and a.actor_generation = m.terminal_publication_actor_generation
               and a.instance_id = m.assigned_instance_id
               and a.actor_epoch = m.assigned_instance_epoch
               and a.physical_host_id = m.assigned_physical_host_id
               and a.authoritative_tick = m.authoritative_tick
               and a.next_sequence = m.next_sequence
               and a.match_revision = m.match_revision
               and a.next_input_sequences = (
                   select coalesce(
                       jsonb_object_agg(
                           mm.player_id,
                           to_jsonb(mm.next_input_sequence)
                           order by mm.player_id
                       ),
                       '{}'::jsonb
                   )
                     from trnm_online_match_members mm
                    where mm.match_id = m.match_id
               )
               and a.snapshot_hash = m.snapshot_hash
               and a.phase = 'complete'
               and a.result_hash = m.result_hash
               and a.published_settlement_state = m.settlement_state
        )
      order by m.match_id
      limit $2";
const EXACT_TERMINAL_PUBLICATION_MARKER_SQL: &str = "select exists(
        select 1
          from trnm_online_terminal_publication_acks a
          join trnm_online_matches m on m.match_id = a.match_id
         where a.match_id = $1
           and m.phase = 'complete'
           and m.checkpoint_sequence = m.next_sequence
           and m.result_hash is not null
           and m.settlement_state in ('pending', 'settled')
           and m.terminal_publication_state = 'acknowledged'
           and a.local_tombstone_state = 'sealed'
           and m.terminal_publication_actor_generation is not null
           and a.actor_generation = m.terminal_publication_actor_generation
           and a.instance_id = m.assigned_instance_id
           and a.actor_epoch = m.assigned_instance_epoch
           and a.physical_host_id = m.assigned_physical_host_id
           and a.authoritative_tick = m.authoritative_tick
           and a.next_sequence = m.next_sequence
           and a.match_revision = m.match_revision
           and a.snapshot_hash = m.snapshot_hash
           and a.phase = 'complete'
           and a.result_hash = m.result_hash
           and a.published_settlement_state = m.settlement_state
           and a.next_input_sequences = coalesce(
               (select jsonb_object_agg(
                   mm.player_id,
                   mm.next_input_sequence order by mm.player_id
                )
                  from trnm_online_match_members mm
                 where mm.match_id = m.match_id),
               '{}'::jsonb
           )
    )";
const CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL: &str = "select exists(
        select 1
          from trnm_online_progression_events event
         where event.campaign_id = $1
           and not exists (
               select 1
                 from trnm_online_matches m
                 join trnm_online_terminal_publication_acks a on a.match_id = m.match_id
                where m.match_id = event.match_id
                  and m.phase = 'complete'
                  and m.checkpoint_sequence = m.next_sequence
                  and m.result_hash = event.result_hash
                  and m.settlement_state in ('pending', 'settled')
                  and m.terminal_publication_state = 'acknowledged'
                  and a.local_tombstone_state = 'sealed'
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
                         from trnm_online_match_members member
                        where member.match_id = m.match_id),
                      '{}'::jsonb
                  )
                  and a.snapshot_hash = m.snapshot_hash
                  and a.phase = 'complete'
                  and a.result_hash = m.result_hash
                  and a.published_settlement_state = m.settlement_state
           )
    )";
const READINESS_DATABASE_SUMMARY_SQL: &str = "with terminal_ack_gaps as materialized (
        select m.match_id
          from trnm_online_matches m
         where m.phase = 'complete'
           and m.assigned_physical_host_id = $3
           and m.terminal_publication_state <> 'legacy_quarantined'
           and not exists (
               select 1
                 from trnm_online_terminal_publication_acks a
                where a.match_id = m.match_id
                  and m.checkpoint_sequence = m.next_sequence
                  and m.result_hash is not null
                  and m.settlement_state in ('pending', 'settled')
                  and m.terminal_publication_state = 'acknowledged'
                  and a.local_tombstone_state = 'sealed'
                  and m.terminal_publication_actor_generation is not null
                  and a.actor_generation = m.terminal_publication_actor_generation
                  and a.instance_id = m.assigned_instance_id
                  and a.actor_epoch = m.assigned_instance_epoch
                  and a.physical_host_id = m.assigned_physical_host_id
                  and a.authoritative_tick = m.authoritative_tick
                  and a.next_sequence = m.next_sequence
                  and a.match_revision = m.match_revision
                  and a.next_input_sequences = (
                      select coalesce(
                          jsonb_object_agg(
                              mm.player_id,
                              to_jsonb(mm.next_input_sequence)
                              order by mm.player_id
                          ),
                          '{}'::jsonb
                      )
                        from trnm_online_match_members mm
                       where mm.match_id = m.match_id
                  )
                  and a.snapshot_hash = m.snapshot_hash
                  and a.phase = 'complete'
                  and a.result_hash = m.result_hash
                  and a.published_settlement_state = m.settlement_state
           )
         order by m.match_id
         limit $4
    ), pending_terminal_seals as materialized (
        select a.match_id
          from trnm_online_terminal_publication_acks a
         where a.physical_host_id = $3
           and a.local_tombstone_state <> 'sealed'
         order by a.match_id
         limit $4
    ), pending_abandonment_seals as materialized (
        select marker.match_id
          from trnm_online_failed_closed_abandonment_markers marker
         where marker.physical_host_id = $3
           and marker.local_tombstone_state <> 'sealed'
         order by marker.match_id
         limit $4
    ), historical_projection as materialized (
        select
            (select count(*) from trnm_online_matches
              where terminal_publication_state = 'legacy_quarantined')
                as legacy_terminal_match_count,
            exists(
                select 1 from trnm_online_progression_events event
                join trnm_online_matches m on m.match_id = event.match_id
                where m.terminal_publication_state = 'legacy_quarantined'
            ) as campaign_projection_polluted,
            exists(
                select 1 from trnm_online_rating_events event
                join trnm_online_matches m on m.match_id = event.match_id
                where m.terminal_publication_state = 'legacy_quarantined'
            ) as rating_projection_polluted
    )
    select
        true as postgres_healthy,
        exists(
            select 1 from trnm_online_fleet_instances
             where instance_id = $1 and instance_epoch = $2
               and physical_host_id = $3
               and status in ('active', 'draining') and lease_expires_at > now()
        ) as fleet_epoch_current,
        (select count(*) from trnm_online_fleet_instances
          where status = 'active' and lease_expires_at > now())
            as healthy_fleet_instances,
        (select count(*) from trnm_online_matches
          where phase = 'running' and assigned_instance_id = $1
            and assigned_instance_epoch = $2 and assigned_physical_host_id = $3)
            as active_matches,
        (select count(*) from terminal_ack_gaps) as terminal_ack_gap_count,
        coalesce(
            (select array_agg(match_id order by match_id) from terminal_ack_gaps),
            array[]::uuid[]
        ) as terminal_ack_gap_match_ids,
        coalesce(
            (select array_agg(match_id order by match_id) from pending_terminal_seals),
            array[]::uuid[]
        ) as pending_terminal_seal_match_ids,
        coalesce(
            (select array_agg(match_id order by match_id) from pending_abandonment_seals),
            array[]::uuid[]
        ) as pending_abandonment_seal_match_ids,
        historical_projection.legacy_terminal_match_count,
        historical_projection.campaign_projection_polluted,
        historical_projection.rating_projection_polluted,
        coalesce(cold.terminal_total_count, 0)::bigint as terminal_total_count,
        coalesce(cold.terminal_sealed_count, 0)::bigint as terminal_sealed_count,
        coalesce(cold.abandonment_total_count, 0)::bigint as abandonment_total_count,
        coalesce(cold.abandonment_sealed_count, 0)::bigint as abandonment_sealed_count
      from historical_projection
      left join trnm_online_local_cold_witness_summaries cold
        on cold.physical_host_id = $3";
const LOCAL_TERMINAL_ACK_STARTUP_PAGE_SQL: &str = "select
        a.match_id, a.actor_generation, a.instance_id, a.actor_epoch,
        a.physical_host_id, a.authoritative_tick, a.next_sequence,
        a.match_revision, a.next_input_sequences, a.snapshot_hash,
        a.result_hash, a.published_settlement_state, a.local_tombstone_state,
        floor(extract(epoch from a.acknowledged_at) * 1000)::bigint
            as acknowledged_at_unix_ms,
        coalesce(
            m.phase = 'complete'
            and m.checkpoint_sequence = m.next_sequence
            and m.result_hash is not null
            and m.settlement_state in ('pending', 'settled')
            and m.terminal_publication_state = 'acknowledged'
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
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and a.snapshot_hash = m.snapshot_hash
            and a.phase = 'complete'
            and a.result_hash = m.result_hash
            and a.published_settlement_state = m.settlement_state,
            false
        ) as tuple_exact
      from trnm_online_terminal_publication_acks a
      left join trnm_online_matches m on m.match_id = a.match_id
     where a.physical_host_id = $1
       and a.local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending')
       and ($2::uuid is null or a.match_id > $2)
     order by a.match_id
     limit $3";
const TERMINAL_ACK_DATABASE_EVIDENCE_BY_MATCH_SQL: &str = "select
        a.match_id, a.actor_generation, a.instance_id, a.actor_epoch,
        a.physical_host_id, a.authoritative_tick, a.next_sequence,
        a.match_revision, a.next_input_sequences, a.snapshot_hash,
        a.result_hash, a.published_settlement_state, a.local_tombstone_state,
        floor(extract(epoch from a.acknowledged_at) * 1000)::bigint
            as acknowledged_at_unix_ms,
        coalesce(
            m.phase = 'complete'
            and m.checkpoint_sequence = m.next_sequence
            and m.result_hash is not null
            and m.settlement_state in ('pending', 'settled')
            and m.terminal_publication_state = 'acknowledged'
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
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and a.snapshot_hash = m.snapshot_hash
            and a.phase = 'complete'
            and a.result_hash = m.result_hash
            and a.published_settlement_state = m.settlement_state,
            false
        ) as tuple_exact
      from trnm_online_terminal_publication_acks a
      left join trnm_online_matches m on m.match_id = a.match_id
     where a.match_id = $1";
const LOCAL_ABANDONMENT_STARTUP_PAGE_SQL: &str = "select
        a.match_id, a.journal_owner_id, a.actor_generation, a.instance_id,
        a.actor_epoch, a.physical_host_id, a.authoritative_tick,
        a.next_sequence, a.match_revision, a.next_input_sequences,
        a.snapshot_hash, a.failure_reason, a.local_tombstone_state,
        floor(extract(epoch from a.abandoned_at) * 1000)::bigint
            as abandoned_at_unix_ms,
        m.simulation_json,
        coalesce(
            m.phase = 'failed_closed'
            and m.settlement_state = 'failed_closed'
            and m.terminal_publication_state = 'pending'
            and m.checkpoint_sequence = m.next_sequence
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.failure_reason = a.failure_reason
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
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and a.snapshot_hash = m.snapshot_hash
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                where terminal.match_id = m.match_id
            ),
            false
        ) as tuple_exact
      from trnm_online_failed_closed_abandonment_markers a
      left join trnm_online_matches m on m.match_id = a.match_id
     where a.physical_host_id = $1
       and a.local_tombstone_state = 'hot_pending'
       and ($2::uuid is null or a.match_id > $2)
     order by a.match_id
     limit $3";
const ABANDONMENT_DATABASE_EVIDENCE_BY_MATCH_SQL: &str = "select
        a.match_id, a.journal_owner_id, a.actor_generation, a.instance_id,
        a.actor_epoch, a.physical_host_id, a.authoritative_tick,
        a.next_sequence, a.match_revision, a.next_input_sequences,
        a.snapshot_hash, a.failure_reason, a.local_tombstone_state,
        floor(extract(epoch from a.abandoned_at) * 1000)::bigint
            as abandoned_at_unix_ms,
        m.simulation_json,
        coalesce(
            m.phase = 'failed_closed'
            and m.settlement_state = 'failed_closed'
            and m.terminal_publication_state = 'pending'
            and m.checkpoint_sequence = m.next_sequence
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.failure_reason = a.failure_reason
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
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and a.snapshot_hash = m.snapshot_hash
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                where terminal.match_id = m.match_id
            ),
            false
        ) as tuple_exact
      from trnm_online_failed_closed_abandonment_markers a
      left join trnm_online_matches m on m.match_id = a.match_id
     where a.match_id = $1";

#[derive(Clone, Copy)]
enum ActorCheckpointBoundary {
    Periodic,
    Shutdown,
    Terminal,
    Fenced,
    CheckpointFailure,
}

fn actor_checkpoint_allowed(boundary: ActorCheckpointBoundary, command_pending: bool) -> bool {
    !command_pending
        && !matches!(
            boundary,
            ActorCheckpointBoundary::Fenced | ActorCheckpointBoundary::CheckpointFailure
        )
}

fn actor_command_lane_publish_allowed(
    command_pending: bool,
    durable_commit_confirmed: bool,
) -> bool {
    !command_pending || durable_commit_confirmed
}

fn committed_publication_tick_is_monotonic(
    latest_visible_candidate_tick: u64,
    committed_tick: u64,
) -> bool {
    committed_tick >= latest_visible_candidate_tick
}

impl AuthorityClockTelemetry {
    fn reset(&self, started_at: Instant) {
        let Ok(mut window) = self.window.lock() else {
            return;
        };
        window.started_at = Some(started_at);
        window.wake_count = 0;
        window.samples.clear();
    }

    fn record_wake(&self, scheduled_at: Instant, observed_at: Instant, tick_interval: Duration) {
        let Ok(mut window) = self.window.lock() else {
            return;
        };
        let lateness_ticks = observed_at
            .saturating_duration_since(scheduled_at)
            .as_secs_f64()
            / tick_interval.as_secs_f64();
        window.wake_count = window.wake_count.saturating_add(1);
        window.samples.push_back(AuthorityClockWake {
            observed_at,
            lateness_ticks,
        });
        while window.samples.len() > AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            window.samples.pop_front();
        }
    }

    fn snapshot(
        &self,
        observed_at: Instant,
        tick_interval: Duration,
    ) -> Option<AuthorityClockSnapshot> {
        let window = self.window.lock().ok()?;
        let started_at = window.started_at?;
        let elapsed = observed_at.saturating_duration_since(started_at);
        let tick_seconds = tick_interval.as_secs_f64();
        let first_wake = window.samples.front();
        let last_wake = window.samples.back();
        let window_drift_ticks = first_wake.zip(last_wake).and_then(|(first, last)| {
            (window.samples.len() >= 2).then(|| {
                let expected_ticks = last
                    .observed_at
                    .saturating_duration_since(first.observed_at)
                    .as_secs_f64()
                    / tick_seconds;
                (window.samples.len() - 1) as f64 - expected_ticks
            })
        });
        let latest_lateness_ticks = last_wake.map(|sample| sample.lateness_ticks);
        let max_recent_lateness_ticks = window
            .samples
            .iter()
            .map(|sample| sample.lateness_ticks)
            .reduce(f64::max);
        let last_wake_age_ms = last_wake.map(|sample| {
            observed_at
                .saturating_duration_since(sample.observed_at)
                .as_secs_f64()
                * 1_000.0
        });
        Some(AuthorityClockSnapshot {
            elapsed_ms: elapsed.as_secs_f64() * 1_000.0,
            wake_count: window.wake_count,
            cumulative_drift_ticks: window.wake_count as f64 - elapsed.as_secs_f64() / tick_seconds,
            window_sample_count: window.samples.len(),
            window_drift_ticks,
            latest_lateness_ticks,
            max_recent_lateness_ticks,
            last_wake_age_ms,
        })
    }
}

#[derive(Clone)]
struct PublishedMatchState {
    simulation: Arc<MissionSimV1>,
    snapshot_hash: Arc<String>,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: Arc<BTreeMap<String, u64>>,
    phase: OnlineMatchPhase,
    result_hash: Option<String>,
    settlement_state: String,
    state_sequence: u64,
    published_at: Instant,
}

#[derive(Clone)]
struct MatchActorHandle {
    actor_id: Uuid,
    commands: mpsc::Sender<ActorCommandEnvelope>,
    members: Arc<BTreeMap<String, ActorMemberAuthority>>,
    published: watch::Receiver<PublishedMatchState>,
    publication_acked: watch::Receiver<ActorPublicationCursor>,
    clock: Arc<AuthorityClockTelemetry>,
    lifecycle: watch::Receiver<MatchActorLifecycle>,
}

#[derive(Clone)]
struct ActorMemberAuthority {
    account_id: Uuid,
    controlled_unit_ids: BTreeSet<String>,
    member_role: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActorPublicationCursor {
    tick: u64,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: BTreeMap<String, u64>,
    phase: OnlineMatchPhase,
    receipts_replayable: bool,
    snapshot_hash: String,
}

struct CommandReceiptPublicationBarrier<'a> {
    sequence: u64,
    accepted_revision: u64,
    durable_next_sequence: u64,
    checkpoint_sequence: u64,
    player_id: &'a str,
    input_sequence: u64,
    durable_member_input_sequence: u64,
    phase: &'a str,
    terminal_publication_acked: bool,
    actor_cursor: Option<&'a ActorPublicationCursor>,
}

struct ActorCommandEnvelope {
    request: OnlineCommandSubmitRequest,
    request_hash: String,
    admission: ActorCommandAdmission,
    controlled_unit_ids: BTreeSet<String>,
    member_role: String,
    response: oneshot::Sender<Result<OnlineCommandReceipt, ApiError>>,
}

struct ActorCommandAdmission {
    bucket_key: String,
    limit: i64,
}

#[derive(Clone)]
struct LoadedMatchActor {
    simulation: MissionSimV1,
    match_mode: String,
    members: Arc<BTreeMap<String, ActorMemberAuthority>>,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: BTreeMap<String, u64>,
    durable_recovery_tick: u64,
}

struct PreparedActorCommand {
    candidate_simulation: MissionSimV1,
    persistence: ActorCommandPersistence,
}

struct ActorCommandPersistence {
    request: OnlineCommandSubmitRequest,
    request_hash: String,
    admission: ActorCommandAdmission,
    order: trnm_rts_protocol::RtsFrameOrder,
    post_simulation: Value,
    snapshot_hash: String,
    input_sequence: u64,
    client_observed_tick: Option<u64>,
    effective_tick: u64,
    base_next_sequence: u64,
    base_match_revision: u64,
    accepted_revision: u64,
}

struct ActorPersistenceCompletion {
    command_id: String,
    result: Result<OnlineCommandReceipt, ApiError>,
}

struct PendingActorCommand {
    command_id: String,
    player_id: String,
    request_hash: String,
    input_sequence: u64,
    accepted_revision: u64,
    sequence: u64,
    accepted_snapshot_hash: String,
    durable_simulation_tick: u64,
    visible: LoadedMatchActor,
    response: oneshot::Sender<Result<OnlineCommandReceipt, ApiError>>,
}

#[derive(Clone)]
struct ActorReceiptCacheEntry {
    player_id: String,
    request_hash: String,
    receipt: OnlineCommandReceipt,
}

struct MatchCheckpointJob {
    simulation: MissionSimV1,
    snapshot_hash: String,
    next_sequence: u64,
    match_revision: u64,
    terminal: bool,
    completion: Option<oneshot::Sender<Result<(), String>>>,
}

#[derive(Clone)]
struct ActorPublicationCandidate {
    state: PublishedMatchState,
    durable_db_next_sequence: u64,
    durable_db_match_revision: u64,
    durable_db_next_input_sequences: BTreeMap<String, u64>,
    receipts_replayable: bool,
    publish_to_watch: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TerminalPublicationEvidence {
    authoritative_tick: u64,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: BTreeMap<String, u64>,
    snapshot_hash: String,
    phase: OnlineMatchPhase,
    result_hash: Option<String>,
    settlement_state: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TerminalPublicationCommit {
    result_hash: String,
    settlement_state: String,
    acknowledged_at_unix_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DatabaseLineage {
    system_identifier: String,
    timeline_id: u32,
    wal_flush_lsn: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ColdWitnessDatabaseSummary {
    terminal_total_count: u64,
    terminal_sealed_count: u64,
    abandonment_total_count: u64,
    abandonment_sealed_count: u64,
}

impl ColdWitnessDatabaseSummary {
    fn all_sealed(self) -> bool {
        self.terminal_total_count == self.terminal_sealed_count
            && self.abandonment_total_count == self.abandonment_sealed_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReadinessDatabaseSummary {
    postgres_healthy: bool,
    fleet_epoch_current: bool,
    healthy_fleet_instances: i64,
    active_matches: i64,
    terminal_ack_gap_count: usize,
    terminal_ack_gap_match_ids: Vec<Uuid>,
    pending_terminal_seal_match_ids: Vec<Uuid>,
    pending_abandonment_seal_match_ids: Vec<Uuid>,
    historical_projection: HistoricalTerminalProjectionQuarantine,
    cold_witness: ColdWitnessDatabaseSummary,
}

impl ReadinessDatabaseSummary {
    fn blocked() -> Self {
        Self {
            postgres_healthy: false,
            fleet_epoch_current: false,
            healthy_fleet_instances: 0,
            active_matches: -1,
            terminal_ack_gap_count: usize::MAX,
            terminal_ack_gap_match_ids: Vec::new(),
            pending_terminal_seal_match_ids: Vec::new(),
            pending_abandonment_seal_match_ids: Vec::new(),
            historical_projection: HistoricalTerminalProjectionQuarantine {
                legacy_terminal_match_count: -1,
                campaign_projection_polluted: true,
                rating_projection_polluted: true,
            },
            cold_witness: ColdWitnessDatabaseSummary::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AbandonmentDatabaseEvidence {
    match_id: Uuid,
    journal_owner_id: Uuid,
    actor_generation: Uuid,
    instance_id: String,
    actor_epoch: i64,
    physical_host_id: String,
    authoritative_tick: u64,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: BTreeMap<String, u64>,
    snapshot_hash: String,
    failure_reason: String,
    local_tombstone_state: String,
    abandoned_at_unix_ms: u64,
}

impl AbandonmentDatabaseEvidence {
    fn matches_high_water(&self, high_water: &PublishedTickHighWater) -> bool {
        self.match_id == high_water.match_id
            && self.journal_owner_id == high_water.journal_owner_id
            && self.actor_generation == high_water.actor_generation
            && self.instance_id == high_water.instance_id
            && self.actor_epoch == high_water.actor_epoch
            && self.physical_host_id == high_water.physical_host_id
            && self.authoritative_tick == high_water.tick
            && self.next_sequence == high_water.next_sequence
            && self.match_revision == high_water.match_revision
            && self.next_input_sequences == high_water.next_input_sequences
            && self.snapshot_hash == high_water.snapshot_hash
            && high_water.phase == "running"
            && high_water.receipts_replayable
    }
}

fn valid_abandonment_failure_reason(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 1_024 && !value.chars().any(char::is_control)
}

fn legacy_abandonment_adoption_allowed(
    requested: bool,
    match_updated_at: DateTime<Utc>,
    migration_applied_at: DateTime<Utc>,
) -> bool {
    requested && match_updated_at < migration_applied_at
}

fn parse_postgres_wal_lsn(value: &str) -> Option<u64> {
    fn parse_canonical_hex_part(part: &str) -> Option<u32> {
        if part.is_empty()
            || part.len() > 8
            || (part != "0" && part.starts_with('0'))
            || !part
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'A'..=b'F'))
        {
            return None;
        }
        u32::from_str_radix(part, 16).ok()
    }

    let (high, low) = value.split_once('/')?;
    if low.contains('/') {
        return None;
    }
    let high = parse_canonical_hex_part(high)?;
    let low = parse_canonical_hex_part(low)?;
    Some((u64::from(high) << 32) | u64::from(low))
}

fn terminal_tombstone_settlement_matches(tombstone_state: &str, database_state: &str) -> bool {
    tombstone_state == database_state
        || (tombstone_state == "pending" && database_state == "settled")
}

fn database_lineage_covers_tombstone(
    lineage: &DatabaseLineage,
    tombstone: &PublishedTickAckTombstone,
) -> bool {
    lineage.system_identifier == tombstone.database_system_identifier
        && lineage.timeline_id == tombstone.database_timeline_id
        && parse_postgres_wal_lsn(&lineage.wal_flush_lsn)
            .zip(parse_postgres_wal_lsn(&tombstone.database_wal_lsn))
            .is_some_and(|(current, sealed)| current >= sealed)
}

fn database_lineage_covers_abandonment(
    lineage: &DatabaseLineage,
    tombstone: &PublishedTickAbandonmentTombstone,
) -> bool {
    lineage.system_identifier == tombstone.database_system_identifier
        && lineage.timeline_id == tombstone.database_timeline_id
        && parse_postgres_wal_lsn(&lineage.wal_flush_lsn)
            .zip(parse_postgres_wal_lsn(&tombstone.database_wal_lsn))
            .is_some_and(|(current, sealed)| current >= sealed)
}

fn database_lineage_covers_cold_witness(
    lineage: &DatabaseLineage,
    witness: &PublishedTickColdWitness,
) -> bool {
    match witness {
        PublishedTickColdWitness::TerminalAck(tombstone) => {
            database_lineage_covers_tombstone(lineage, tombstone)
        }
        PublishedTickColdWitness::FailedClosedAbandonment(tombstone) => {
            database_lineage_covers_abandonment(lineage, tombstone)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StartupTerminalAckSealSource {
    Cold,
    Hot,
    LegacyDatabaseBootstrap,
}

fn startup_terminal_ack_seal_source(
    local_tombstone_state: &str,
    has_hot: bool,
    has_cold: bool,
) -> Option<StartupTerminalAckSealSource> {
    if has_cold {
        return Some(StartupTerminalAckSealSource::Cold);
    }
    match local_tombstone_state {
        "legacy_bootstrap_pending" if has_hot => Some(StartupTerminalAckSealSource::Hot),
        "legacy_bootstrap_pending" => Some(StartupTerminalAckSealSource::LegacyDatabaseBootstrap),
        "hot_pending" if has_hot => Some(StartupTerminalAckSealSource::Hot),
        "sealed" => None,
        _ => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TerminalAckDatabaseEvidence {
    match_id: Uuid,
    actor_generation: Uuid,
    instance_id: String,
    actor_epoch: i64,
    physical_host_id: String,
    authoritative_tick: u64,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: BTreeMap<String, u64>,
    snapshot_hash: String,
    result_hash: String,
    settlement_state: String,
    local_tombstone_state: String,
    acknowledged_at_unix_ms: u64,
}

impl TerminalAckDatabaseEvidence {
    fn record_input(&self) -> PublishedTickRecordInput {
        PublishedTickRecordInput {
            instance_id: self.instance_id.clone(),
            match_id: self.match_id,
            actor_generation: self.actor_generation,
            actor_epoch: self.actor_epoch,
            tick: self.authoritative_tick,
            next_sequence: self.next_sequence,
            match_revision: self.match_revision,
            next_input_sequences: self.next_input_sequences.clone(),
            phase: "complete".to_string(),
            receipts_replayable: true,
            snapshot_hash: self.snapshot_hash.clone(),
        }
    }

    fn matches_high_water(&self, high_water: &PublishedTickHighWater) -> bool {
        high_water.instance_id == self.instance_id
            && high_water.physical_host_id == self.physical_host_id
            && high_water.match_id == self.match_id
            && high_water.actor_generation == self.actor_generation
            && high_water.actor_epoch == self.actor_epoch
            && high_water.tick == self.authoritative_tick
            && high_water.next_sequence == self.next_sequence
            && high_water.match_revision == self.match_revision
            && high_water.next_input_sequences == self.next_input_sequences
            && high_water.phase == "complete"
            && high_water.receipts_replayable
            && high_water.snapshot_hash == self.snapshot_hash
    }

    fn matches_tombstone(
        &self,
        tombstone: &PublishedTickAckTombstone,
        lineage: &DatabaseLineage,
    ) -> bool {
        self.matches_high_water(&tombstone.high_water)
            && tombstone.result_hash == self.result_hash
            && terminal_tombstone_settlement_matches(
                &tombstone.settlement_state,
                &self.settlement_state,
            )
            && tombstone.acknowledged_at_unix_ms == self.acknowledged_at_unix_ms
            && database_lineage_covers_tombstone(lineage, tombstone)
    }
}

#[derive(Clone)]
struct StagedTerminalAuthority {
    simulation: MissionSimV1,
    result: BattleResultV1,
    result_hash: String,
    snapshot_hash: String,
    authoritative_tick: u64,
    next_sequence: u64,
    match_revision: u64,
}

struct TerminalFinalizationExpectation<'a> {
    match_id: Uuid,
    actor_generation: Uuid,
    actor_epoch: i64,
    instance_id: &'a str,
    physical_host_id: &'a str,
    authoritative_tick: u64,
    next_sequence: u64,
    match_revision: u64,
    next_input_sequences: &'a BTreeMap<String, u64>,
    snapshot_hash: &'a str,
    result_hash: Option<&'a str>,
}

impl TerminalPublicationEvidence {
    fn from_state(state: &PublishedMatchState) -> Option<Self> {
        (state.phase == OnlineMatchPhase::Complete).then(|| Self {
            authoritative_tick: state.simulation.tick,
            next_sequence: state.next_sequence,
            match_revision: state.match_revision,
            next_input_sequences: state.next_input_sequences.as_ref().clone(),
            snapshot_hash: state.snapshot_hash.as_ref().clone(),
            phase: state.phase,
            result_hash: state.result_hash.clone(),
            settlement_state: state.settlement_state.clone(),
        })
    }
}

struct ActorPublicationCompletion {
    state_sequence: u64,
    deferred_terminal: Option<DeferredTerminalPublication>,
    result: Result<(), String>,
}

struct TerminalCheckpointCompletion {
    snapshot_hash: String,
    result: Result<(), String>,
}

struct TerminalAckCompletion {
    deferred_terminal: DeferredTerminalPublication,
    result: Result<TerminalPublicationCommit, String>,
}

struct DeferredTerminalPublication {
    state: PublishedMatchState,
    cursor: ActorPublicationCursor,
    evidence: TerminalPublicationEvidence,
    publish_to_watch: bool,
}

struct ActorPublicationWorker {
    journal: PublishedTickJournal,
    instance_id: Arc<String>,
    match_id: Uuid,
    actor_id: Uuid,
    actor_epoch: i64,
    candidates: watch::Receiver<Option<ActorPublicationCandidate>>,
    permit: watch::Receiver<bool>,
    durable_recovery_tick: watch::Receiver<u64>,
    published: watch::Sender<PublishedMatchState>,
    publication_acked: watch::Sender<ActorPublicationCursor>,
    completions: mpsc::Sender<ActorPublicationCompletion>,
}

struct PendingPublishedCommandReceipt {
    state_sequence: u64,
    request_hash: String,
    receipt: OnlineCommandReceipt,
    response: oneshot::Sender<Result<OnlineCommandReceipt, ApiError>>,
}

