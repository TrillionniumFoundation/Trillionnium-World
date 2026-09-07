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
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
