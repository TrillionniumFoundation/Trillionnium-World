#![recursion_limit = "256"]

mod cex;
mod map;
mod operations_v1;
mod product_v2;
mod production_v1;
pub mod signer_protocol;

use axum::extract::DefaultBodyLimit;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cex::CexClient;
use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::row::Row;
use sqlx_postgres::{PgPool, PgPoolOptions};
use std::{
    collections::{BTreeMap, BTreeSet},
    net::SocketAddr,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::sync::{mpsc, oneshot, watch, RwLock};
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
    ONLINE_AUTHORITY_PROTOCOL, ONLINE_PRODUCT_BUILD, ONLINE_PRODUCT_PROTOCOL,
};
use trnm_rts_protocol::RtsOrderSource;
use trnm_rts_sim::{MissionSimV1, TICKS_PER_SECOND};
use uuid::Uuid;

const PLAYER_SESSION_HEADER: &str = "x-trnm-player-session";
const MIGRATION_V1: &str = include_str!("../migrations/0001_online_authority_v1.sql");
const MIGRATION_V2: &str = include_str!("../migrations/0002_online_authority_v2.sql");
const MIGRATION_V3: &str = include_str!("../migrations/0003_online_product_v1.sql");
const MIGRATION_V4: &str = include_str!("../migrations/0004_online_product_v2.sql");
const MIGRATION_V5: &str = include_str!("../migrations/0005_online_operations_v1.sql");
const MIGRATION_V6: &str = include_str!("../migrations/0006_online_operations_v2.sql");
const MIGRATION_V7: &str = include_str!("../migrations/0007_online_production_v1.sql");
const MIGRATION_V8: &str = include_str!("../migrations/0008_online_production_v2.sql");
const MIGRATION_V9: &str = include_str!("../migrations/0009_online_realtime_actor_v1.sql");
const MIGRATION_ADVISORY_LOCK: i64 = 0x5452_4e4d_4f4e_4c49;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
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
    match_actors: Arc<RwLock<BTreeMap<Uuid, MatchActorHandle>>>,
    shutdown: watch::Sender<bool>,
}

#[derive(Default)]
struct AuthorityClockTelemetry {
    started_at: Mutex<Option<Instant>>,
    wake_count: AtomicU64,
}

const MATCH_ACTOR_COMMAND_QUEUE: usize = 64;
const MATCH_CHECKPOINT_QUEUE: usize = 2;
const MATCH_CHECKPOINT_INTERVAL_TICKS: u64 = 100;
const MATCH_ACTOR_FENCE_INTERVAL: Duration = Duration::from_secs(1);
const MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS: f64 = 2.0;

#[derive(Clone)]
struct PublishedMatchState {
    simulation: Arc<MissionSimV1>,
    snapshot_hash: Arc<String>,
    next_sequence: u64,
    match_revision: u64,
}

#[derive(Clone)]
struct MatchActorHandle {
    actor_id: Uuid,
    commands: mpsc::Sender<ActorCommandEnvelope>,
    published: watch::Receiver<PublishedMatchState>,
}

struct ActorCommandEnvelope {
    request: OnlineCommandSubmitRequest,
    request_hash: String,
    controlled_unit_ids: BTreeSet<String>,
    member_role: String,
    response: oneshot::Sender<Result<OnlineCommandReceipt, ApiError>>,
}

struct LoadedMatchActor {
    simulation: MissionSimV1,
    match_mode: String,
    next_sequence: u64,
    match_revision: u64,
}

struct MatchCheckpointJob {
    simulation: MissionSimV1,
    snapshot_hash: String,
    next_sequence: u64,
    match_revision: u64,
    terminal: bool,
    completion: Option<oneshot::Sender<Result<(), String>>>,
}

pub struct AppStateConfig {
    pub database_url: String,
    pub cex_base_url: String,
    pub game_authority_token: String,
    pub entitlement_signer_url: String,
    pub entitlement_signer_token: String,
    pub asset_root: PathBuf,
    pub moderator_token: String,
    pub instance_id: String,
    pub region: String,
    pub public_endpoint: String,
    pub physical_host_id: String,
    pub capacity: i32,
    pub rate_limit_per_minute: u32,
    pub request_body_limit_bytes: u32,
    pub tick_interval: Duration,
    pub accelerated_test_clock: bool,
}

impl AppState {
    pub async fn connect(config: AppStateConfig) -> Result<Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&config.database_url)
            .await
            .map_err(|error| format!("connect Online Authority PostgreSQL: {error}"))?;
        let mut migrations = pool
            .begin()
            .await
            .map_err(|error| format!("begin Online Production migrations: {error}"))?;
        sqlx::query::query("select pg_advisory_xact_lock($1)")
            .bind(MIGRATION_ADVISORY_LOCK)
            .execute(&mut *migrations)
            .await
            .map_err(|error| format!("lock Online Production migrations: {error}"))?;
        for (label, sql) in [
            ("Online Authority", MIGRATION_V1),
            ("Online Authority v2", MIGRATION_V2),
            ("Online Product v1", MIGRATION_V3),
            ("Online Product v2", MIGRATION_V4),
            ("Online Operations v1", MIGRATION_V5),
            ("Online Operations v2", MIGRATION_V6),
            ("Online Production v1", MIGRATION_V7),
            ("Online Production v2", MIGRATION_V8),
            ("Online Realtime Actor v1", MIGRATION_V9),
        ] {
            sqlx::raw_sql::raw_sql(sql)
                .execute(&mut *migrations)
                .await
                .map_err(|error| format!("migrate {label} PostgreSQL: {error}"))?;
        }
        migrations
            .commit()
            .await
            .map_err(|error| format!("commit Online Production migrations: {error}"))?;
        if config.instance_id.trim().is_empty()
            || config.region.trim().is_empty()
            || config.public_endpoint.trim().is_empty()
            || config.physical_host_id.trim().is_empty()
            || !(1..=10_000).contains(&config.capacity)
            || !(30..=100_000).contains(&config.rate_limit_per_minute)
            || !(16_384..=1_048_576).contains(&config.request_body_limit_bytes)
        {
            return Err(
                "fleet identity, capacity and production ingress limits must be valid".to_string(),
            );
        }
        let instance_epoch: i64 = sqlx::query_scalar::query_scalar(
            "insert into trnm_online_fleet_instances (
                instance_id, region, public_endpoint, build_id, capacity, status,
                instance_epoch, lease_expires_at, physical_host_id
             ) values ($1, $2, $3, $4, $5, 'active', 1, now() + interval '5 seconds', $6)
             on conflict (instance_id) do update set region = excluded.region,
                public_endpoint = excluded.public_endpoint, build_id = excluded.build_id,
                capacity = excluded.capacity, status = 'active', heartbeat_at = now(),
                lease_expires_at = now() + interval '5 seconds',
                instance_epoch = trnm_online_fleet_instances.instance_epoch + 1,
                physical_host_id = excluded.physical_host_id, drain_reason = null
             returning instance_epoch",
        )
        .bind(config.instance_id.trim())
        .bind(config.region.trim())
        .bind(config.public_endpoint.trim())
        .bind(trnm_online_protocol::ONLINE_OPERATIONS_BUILD)
        .bind(config.capacity)
        .bind(config.physical_host_id.trim())
        .fetch_one(&pool)
        .await
        .map_err(|error| format!("register Online Operations fleet instance: {error}"))?;
        let cex = CexClient::new(
            config.cex_base_url,
            config.game_authority_token,
            config.entitlement_signer_url,
            config.entitlement_signer_token,
        )?;
        cex.readiness().await?;
        let (shutdown, _) = watch::channel(false);
        Ok(Self {
            pool,
            cex,
            asset_root: Arc::new(config.asset_root),
            moderator_token: Arc::new(config.moderator_token),
            instance_id: Arc::new(config.instance_id),
            region: Arc::new(config.region),
            public_endpoint: Arc::new(config.public_endpoint),
            physical_host_id: Arc::new(config.physical_host_id),
            capacity: config.capacity,
            instance_epoch,
            rate_limit_per_minute: config.rate_limit_per_minute,
            request_body_limit_bytes: config.request_body_limit_bytes,
            tick_interval: config.tick_interval,
            accelerated_test_clock: config.accelerated_test_clock,
            authority_clock: Arc::new(AuthorityClockTelemetry::default()),
            match_actors: Arc::new(RwLock::new(BTreeMap::new())),
            shutdown,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn graceful_shutdown(&self) -> Result<(), String> {
        self.shutdown.send_replace(true);
        tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                if self.match_actors.read().await.is_empty() {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .map_err(|_| "timed out flushing online match actors during shutdown".to_string())
    }
}

type ApiError = (StatusCode, Json<OnlineAuthorityError>);

pub fn validate_operations_bind_addr(bind_addr: SocketAddr) -> Result<(), String> {
    if !bind_addr.ip().is_loopback() {
        return Err(
            "public/non-loopback game-server bind is blocked until KMS/HSM custody, edge rate limiting, DDoS protection and an approved deployment attestation are implemented"
                .to_string(),
        );
    }
    Ok(())
}

fn api_error(status: StatusCode, message: impl Into<String>, recoverable: bool) -> ApiError {
    (
        status,
        Json(OnlineAuthorityError {
            error: message.into(),
            recoverable,
            authoritative_revision: None,
        }),
    )
}

async fn lock_current_fleet_epoch(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    state: &AppState,
    allow_draining: bool,
) -> Result<(), String> {
    let row = sqlx::query::query(
        "select instance_epoch, status, lease_expires_at > now() as lease_current
         from trnm_online_fleet_instances where instance_id = $1 for share",
    )
    .bind(state.instance_id.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "fleet instance registration is missing".to_string())?;
    let instance_epoch: i64 = row
        .try_get("instance_epoch")
        .map_err(|error| error.to_string())?;
    let status: String = row.try_get("status").map_err(|error| error.to_string())?;
    let lease_current: bool = row
        .try_get("lease_current")
        .map_err(|error| error.to_string())?;
    let status_current = status == "active" || (allow_draining && status == "draining");
    if instance_epoch != state.instance_epoch || !status_current || !lease_current {
        return Err("fleet instance epoch is fenced, expired or not routable".to_string());
    }
    Ok(())
}

fn conflict(message: impl Into<String>, revision: u64) -> ApiError {
    (
        StatusCode::CONFLICT,
        Json(OnlineAuthorityError {
            error: message.into(),
            recoverable: true,
            authoritative_revision: Some(revision),
        }),
    )
}

fn session_header(headers: &HeaderMap) -> Result<&str, ApiError> {
    headers
        .get(PLAYER_SESSION_HEADER)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            api_error(
                StatusCode::UNAUTHORIZED,
                "player session is required",
                false,
            )
        })
}

async fn verify_identity(
    state: &AppState,
    headers: &HeaderMap,
    player_id: &str,
    account_id: &str,
) -> Result<cex::SessionVerifyResponse, ApiError> {
    let token = session_header(headers)?;
    let verified = state
        .cex
        .verify_session(token, player_id, account_id)
        .await
        .map_err(|error| api_error(StatusCode::UNAUTHORIZED, error, false))?;
    if verified.player_id != player_id || verified.account_id != account_id {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "verified session identity mismatch",
            false,
        ));
    }
    if verified.session_id.is_empty()
        || verified.device_id.is_empty()
        || verified.recovery_generation <= 0
        || verified.expires_at_epoch < chrono::Utc::now().timestamp()
    {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "verified session metadata is expired or incomplete",
            false,
        ));
    }
    Ok(verified)
}

fn hash_json<T: serde::Serialize>(value: &T) -> Result<String, ApiError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn validate_slot_key(value: &str) -> Result<(), ApiError> {
    if value.is_empty()
        || value.len() > 32
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "slot_key must be 1-32 ASCII letters, digits, '-' or '_'",
            false,
        ));
    }
    Ok(())
}

fn validate_command_id(value: &str) -> Result<(), ApiError> {
    if value.is_empty()
        || value.len() > 160
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "command_id must be 1-160 portable ASCII identifier characters",
            false,
        ));
    }
    Ok(())
}

fn mission_for_map(map_id: &str) -> Result<CampaignMission, ApiError> {
    match map_id {
        "first_contact" => Ok(CampaignMission::FirstContact),
        "iron_delta" => Ok(CampaignMission::IronDeltaSkirmish),
        "night_watch_crossing" => Ok(CampaignMission::NightWatchCrossingSkirmish),
        "glass_basin" => Ok(CampaignMission::GlassBasinSkirmish),
        "ember_orchard" => Ok(CampaignMission::EmberOrchardSkirmish),
        "salt_marsh" => Ok(CampaignMission::SaltMarshSkirmish),
        "cinder_crown" => Ok(CampaignMission::CinderCrownSkirmish),
        _ => Err(api_error(
            StatusCode::BAD_REQUEST,
            "map is not in the Online Authority v2 authored allowlist",
            false,
        )),
    }
}

fn prepare_campaign_seed(
    campaign: &mut CampaignSaveV1,
    map_id: &str,
    map: BattleMapSeedV1,
) -> Result<BattleSeedV1, ApiError> {
    if map_id == "first_contact" {
        campaign
            .move_to(trnm_campaign_core::CampaignRoom::MentorHall)
            .and_then(|_| campaign.talk_to_mentor())
            .and_then(|_| campaign.train_with_mentor())
            .and_then(|_| campaign.equip_starter_weapon())
            .and_then(|_| campaign.move_to(trnm_campaign_core::CampaignRoom::ExpeditionGate))
            .and_then(|_| campaign.accept_first_contact_quest())
            .map_err(|error| api_error(StatusCode::CONFLICT, error.to_string(), false))?;
    } else {
        campaign
            .prepare_standalone_skirmish()
            .map_err(|error| api_error(StatusCode::CONFLICT, error.to_string(), false))?;
        campaign.active_mission = mission_for_map(map_id)?;
    }
    campaign
        .start_first_contact_battle(map)
        .map_err(|error| api_error(StatusCode::CONFLICT, error.to_string(), false))
}

fn member_units(
    seed: &BattleSeedV1,
    member_tag: &str,
    slot_offset: usize,
) -> (
    Vec<trnm_campaign_core::BattleUnitSeedV1>,
    BTreeMap<String, String>,
) {
    let mut id_map = BTreeMap::new();
    let units = seed.party[..2]
        .iter()
        .enumerate()
        .map(|(index, source)| {
            let mut unit = source.clone();
            unit.unit_id = format!("{member_tag}:{}", source.unit_id);
            unit.spawn_slot = format!("party_{}", slot_offset + index);
            id_map.insert(unit.unit_id.clone(), source.unit_id.clone());
            unit
        })
        .collect();
    (units, id_map)
}

async fn production_rate_limit(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    if path == "/health" {
        return next.run(request).await;
    }
    let effective_limit = if path.ends_with("/snapshot")
        || path.ends_with("/commands")
        || path.ends_with("/reconnect")
    {
        state.rate_limit_per_minute.saturating_mul(20)
    } else {
        state.rate_limit_per_minute
    };
    let identity = request
        .headers()
        .get(PLAYER_SESSION_HEADER)
        .or_else(|| request.headers().get("x-trnm-moderator"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("anonymous");
    let endpoint_class = path.split('/').take(5).collect::<Vec<_>>().join("/");
    let request_class = if effective_limit == state.rate_limit_per_minute {
        "control"
    } else {
        "data"
    };
    let key = format!(
        "{:x}",
        Sha256::digest(format!("{}:{}:{}", identity, request.method(), endpoint_class).as_bytes())
    );
    let count = sqlx::query_scalar::query_scalar::<_, i64>(
        "insert into trnm_online_admission_windows (
            bucket_key, window_started_at, request_class, request_count,
            rejection_count, last_instance_id
         ) values ($1, date_trunc('minute', now()), $2, 1, 0, $3)
         on conflict (bucket_key, window_started_at) do update set
            request_count = trnm_online_admission_windows.request_count + 1,
            last_instance_id = excluded.last_instance_id, updated_at = now()
         returning request_count",
    )
    .bind(&key)
    .bind(request_class)
    .bind(state.instance_id.as_str())
    .fetch_one(&state.pool)
    .await;
    let count = match count {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(%error, "distributed admission failed closed");
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "distributed admission is unavailable"})),
            )
                .into_response();
        }
    };
    if count > i64::from(effective_limit) {
        if let Err(error) = sqlx::query::query(
            "update trnm_online_admission_windows set
                rejection_count = rejection_count + 1, updated_at = now()
             where bucket_key = $1 and window_started_at = date_trunc('minute', now())",
        )
        .bind(&key)
        .execute(&state.pool)
        .await
        {
            tracing::error!(%error, "distributed admission rejection audit failed");
        }
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "error": "production request rate limit exceeded",
                "retry_after_seconds": 60,
            })),
        )
            .into_response();
    }
    next.run(request).await
}

pub fn build_router(state: AppState) -> Router {
    let body_limit = state.request_body_limit_bytes as usize;
    Router::new()
        .route("/health", get(health))
        .route("/v1/online/readiness", get(readiness))
        .route("/v1/online/campaigns/connect", post(connect_campaign))
        .route("/v1/online/matches", post(create_match))
        .route("/v1/online/matches/join", post(join_match))
        .route("/v1/online/matches/:match_id/start", post(start_match))
        .route(
            "/v1/online/matches/:match_id/commands",
            post(submit_command),
        )
        .route("/v1/online/matches/:match_id/snapshot", post(get_snapshot))
        .route(
            "/v1/online/matches/:match_id/reconnect",
            post(reconnect_match),
        )
        .route("/v1/product/lobbies", post(create_lobby))
        .route("/v1/product/lobbies/:lobby_id/view", post(get_lobby))
        .route(
            "/v1/product/lobbies/:lobby_id/invites",
            post(invite_to_lobby),
        )
        .route(
            "/v1/product/lobbies/invites/accept",
            post(accept_lobby_invite),
        )
        .route("/v1/product/lobbies/:lobby_id/ready", post(set_lobby_ready))
        .route("/v1/product/lobbies/:lobby_id/queue", post(queue_lobby))
        .route(
            "/v1/product/solo-queue/join",
            post(product_v2::join_solo_queue),
        )
        .route(
            "/v1/product/solo-queue/status",
            post(product_v2::get_solo_queue),
        )
        .route(
            "/v1/product/solo-queue/cancel",
            post(product_v2::cancel_solo_queue),
        )
        .route("/v1/product/rating", post(product_v2::get_rating))
        .route(
            "/v1/product/social/friends/request",
            post(product_v2::request_friend),
        )
        .route(
            "/v1/product/social/friends/resolve",
            post(product_v2::resolve_friend),
        )
        .route("/v1/product/social/block", post(product_v2::set_block))
        .route("/v1/product/social/view", post(product_v2::get_social))
        .route("/v1/product/reports", post(product_v2::create_report))
        .route(
            "/v1/product/moderation/reports/resolve",
            post(product_v2::resolve_report),
        )
        .route(
            "/v1/operations/leaderboard",
            post(operations_v1::get_leaderboard),
        )
        .route("/v1/operations/replays", post(operations_v1::get_replay))
        .route(
            "/v1/operations/replays/playback",
            post(operations_v1::get_replay_playback),
        )
        .route(
            "/v1/operations/replays/latest/playback",
            post(operations_v1::get_latest_replay_playback),
        )
        .route(
            "/v1/operations/reports/replay",
            post(operations_v1::create_replay_report),
        )
        .route(
            "/v1/operations/moderation/queue",
            post(operations_v1::moderation_queue),
        )
        .route(
            "/v1/operations/moderation/action",
            post(operations_v1::moderate_case),
        )
        .route(
            "/v1/operations/enforcements/appeals",
            post(operations_v1::create_enforcement_appeal),
        )
        .route(
            "/v1/operations/moderation/appeals",
            post(operations_v1::enforcement_appeal_queue),
        )
        .route(
            "/v1/operations/moderation/appeals/resolve",
            post(operations_v1::resolve_enforcement_appeal),
        )
        .route(
            "/v1/operations/seasons/admin",
            post(operations_v1::admin_season),
        )
        .route(
            "/v1/operations/fleet/route",
            post(operations_v1::route_fleet),
        )
        .route(
            "/v1/operations/fleet/admin",
            post(operations_v1::admin_fleet),
        )
        .route(
            "/v1/production/seasons/automation",
            post(production_v1::configure_season_automation),
        )
        .route(
            "/v1/production/spectators/invites",
            post(production_v1::create_spectator_invite),
        )
        .route(
            "/v1/production/spectators/invites/accept",
            post(production_v1::accept_spectator_invite),
        )
        .route(
            "/v1/production/spectators/playback",
            post(production_v1::spectator_playback),
        )
        .route(
            "/v1/production/player/status",
            post(production_v1::player_production_status),
        )
        .route(
            "/v1/production/host-attestation",
            post(production_v1::host_attestation),
        )
        .route(
            "/v1/production/moderation/shifts/start",
            post(production_v1::start_moderation_shift),
        )
        .route(
            "/v1/production/moderation/shifts/heartbeat",
            post(production_v1::heartbeat_moderation_shift),
        )
        .route(
            "/v1/production/moderation/claims",
            post(production_v1::claim_moderation_case),
        )
        .route(
            "/v1/production/moderation/shifts/close",
            post(production_v1::close_moderation_shift),
        )
        .route(
            "/v1/production/status",
            get(production_v1::production_status),
        )
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            production_rate_limit,
        ))
        .with_state(state)
}

async fn health() -> &'static str {
    "trnm-game-server ok"
}

async fn readiness(State(state): State<AppState>) -> Response {
    let postgres = sqlx::query_scalar::query_scalar::<_, i32>("select 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    let cex = state.cex.readiness().await.is_ok();
    let signer = state.cex.signer_readiness().await.ok();
    let signer_registry_verified = state.cex.signer_attestation().await.is_ok();
    let fleet_epoch_current = sqlx::query_scalar::query_scalar::<_, bool>(
        "select exists(
            select 1 from trnm_online_fleet_instances
            where instance_id = $1 and instance_epoch = $2
              and status in ('active', 'draining') and lease_expires_at > now()
        )",
    )
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .fetch_one(&state.pool)
    .await
    .unwrap_or(false);
    let healthy_fleet_instances = sqlx::query_scalar::query_scalar::<_, i64>(
        "select count(*) from trnm_online_fleet_instances
         where status = 'active' and lease_expires_at > now()",
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or_default();
    let active_matches = sqlx::query_scalar::query_scalar::<_, i64>(
        "select count(*) from trnm_online_matches where phase = 'running'",
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or_default();
    let active_match_actors = state.match_actors.read().await.len();
    let authority_clock_elapsed_ms = state
        .authority_clock
        .started_at
        .lock()
        .ok()
        .and_then(|started_at| started_at.as_ref().map(Instant::elapsed))
        .map(|elapsed| elapsed.as_secs_f64() * 1_000.0);
    let authority_clock_wake_count = state.authority_clock.wake_count.load(Ordering::Relaxed);
    let authority_clock_drift_ticks = authority_clock_elapsed_ms.map(|elapsed_ms| {
        authority_clock_wake_count as f64 - elapsed_ms / state.tick_interval.as_secs_f64() / 1_000.0
    });
    let authority_clock_operational = authority_clock_is_operational(authority_clock_drift_ticks);
    let ready = postgres
        && cex
        && signer.is_some()
        && signer_registry_verified
        && fleet_epoch_current
        && authority_clock_operational;
    let operational_readiness = json!({
        "postgres": postgres,
        "cex": cex,
        "signer": signer.is_some(),
        "signer_registry": signer_registry_verified,
        "fleet_epoch": fleet_epoch_current,
        "authority_clock": authority_clock_operational,
    });
    let mut readiness_body = json!({
            "status": if ready { "ok" } else { "blocked" },
            "protocol": ONLINE_AUTHORITY_PROTOCOL,
            "build_id": ONLINE_AUTHORITY_BUILD,
            "postgres_persistent": postgres,
            "fleet_epoch_current": fleet_epoch_current,
            "cex_identity_and_settlement": cex,
            "server_authoritative_campaign": true,
            "server_authoritative_rts": true,
            "tick_rate_hz": 1.0 / state.tick_interval.as_secs_f64(),
            "simulation_ticks_per_wake": 1,
            "simulation_design_tick_rate_hz": TICKS_PER_SECOND,
            "tick_interval_ms": state.tick_interval.as_millis(),
            "clock_mode": if state.accelerated_test_clock {
                "accelerated_test_only_no_catch_up"
            } else {
                "real_time_no_catch_up"
            },
            "restart_grants_immediate_tick": false,
            "authority_clock_elapsed_ms": authority_clock_elapsed_ms,
            "authority_clock_wake_count": authority_clock_wake_count,
            "authority_clock_drift_ticks": authority_clock_drift_ticks,
            "active_matches": active_matches,
            "active_match_actors": active_match_actors,
            "command_event_log": "postgres_immediate_with_post_command_recovery_state",
            "simulation_persistence": "in_memory_actor_periodic_checkpoint",
            "checkpoint_interval_ticks": MATCH_CHECKPOINT_INTERVAL_TICKS,
            "database_write_per_simulation_tick": false,
            "command_sequence_and_idempotency": true,
            "restart_recovery": true,
            "authenticated_reconnect": true,
            "bounded_command_replay": 256,
            "mode": "private coop plus ranked head-to-head authoritative PvP",
            "independent_member_progression": true,
            "inventory_event_provenance": true,
            "public_matchmaking": true,
            "online_product_protocol": ONLINE_PRODUCT_PROTOCOL,
            "online_product_build": ONLINE_PRODUCT_BUILD,
            "private_lobby_invites": true,
            "coop_vs_ai_match_allocation": true,
            "ranked_solo_queue": true,
            "authoritative_pvp": true,
            "persistent_mmr": true,
            "friends_and_blocks": true,
            "report_and_moderation_workflow": true,
            "online_operations_protocol": trnm_online_protocol::ONLINE_OPERATIONS_PROTOCOL,
            "online_operations_build": trnm_online_protocol::ONLINE_OPERATIONS_BUILD,
            "native_text_login_and_kernel_keyring": true,
            "active_season_and_leaderboard": true,
            "authoritative_replay_index": true,
            "replay_bound_reports": true,
            "integrity_signal_triage": true,
            "moderation_console_and_enforcement": true,
            "fleet_instance_id": state.instance_id.as_str(),
            "fleet_instance_epoch": state.instance_epoch,
            "fleet_region": state.region.as_str(),
            "fleet_capacity": state.capacity,
            "healthy_fleet_instances": healthy_fleet_instances,
            "cross_instance_failover": true,
            "operations_v2_fenced_fleet_leases": true,
            "operations_v2_replay_playback_frames": true,
            "operations_v2_season_admin_and_archival": true,
            "operations_v2_enforcement_appeals_sla": true,
            "operations_v2_drain_and_capacity_control": true,
            "operations_v2_loopback_only_public_bind_gate": true,
            "online_production_protocol": trnm_online_protocol::ONLINE_OPERATIONS_PROTOCOL,
            "online_production_build": trnm_online_protocol::ONLINE_OPERATIONS_BUILD,
            "production_v1_isolated_entitlement_signer": signer.is_some(),
            "production_v1_signer_key_id": signer.as_ref().map(|value| value.key_id.as_str()),
            "production_v1_signer_private_key_exported_to_game_server": false,
            "production_v1_durable_signing_receipts": true,
            "production_v1_rate_limit_per_minute": state.rate_limit_per_minute,
            "production_v1_request_body_limit_bytes": state.request_body_limit_bytes,
            "production_v1_automatic_season_rotation": true,
            "production_v1_targeted_delayed_spectating": true,
            "production_v1_appeal_sla_escalation": true,
            "production_v2_distributed_admission": true,
            "production_v2_capacity_sampling": true,
            "production_v2_signer_key_possession": true,
            "production_v2_signer_registry_verified": signer_registry_verified,
            "production_v2_player_season_spectator_status": true,
            "production_v2_moderation_shift_ownership": true,
            "production_v2_host_challenge_evidence": true,
            "fleet_physical_host_id": state.physical_host_id.as_str(),
            "entitlement_key_custody": signer.as_ref().map(|value| value.custody.as_str())
                .unwrap_or("isolated_signer_unavailable"),
            "kms_hsm_attested": false,
            "public_edge_ddos_attested": false,
    });
    readiness_body["authority_clock_operational"] = Value::Bool(authority_clock_operational);
    readiness_body["authority_clock_max_abs_drift_ticks"] =
        json!(MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS);
    readiness_body["operational_readiness"] = operational_readiness;
    (
        if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(readiness_body),
    )
        .into_response()
}

fn authority_clock_is_operational(drift_ticks: Option<f64>) -> bool {
    drift_ticks
        .is_some_and(|drift| drift.is_finite() && drift.abs() < MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS)
}

async fn connect_campaign(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineCampaignConnectRequest>,
) -> Result<Json<OnlineCampaignView>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    validate_slot_key(&request.slot_key)?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;

    if let Some(row) = sqlx::query::query(
        "select campaign_id, player_id, account_id, slot_key, campaign_revision,
                schema_revision, state_hash, campaign_json
         from trnm_online_campaigns where account_id = $1 and slot_key = $2",
    )
    .bind(account_id)
    .bind(&request.slot_key)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    {
        let stored_player: String = row.try_get("player_id").map_err(internal_db)?;
        if stored_player != request.player_id {
            return Err(api_error(
                StatusCode::FORBIDDEN,
                "campaign slot belongs to another player identity",
                false,
            ));
        }
        return Ok(Json(campaign_view_from_row(&row)?));
    }

    let mut campaign = CampaignSaveV1 {
        campaign_id: format!("online-campaign:{}", Uuid::new_v4()),
        ..CampaignSaveV1::default()
    };
    campaign
        .bind_cex_economy_account(&request.player_id, &request.account_id)
        .map_err(|error| api_error(StatusCode::BAD_REQUEST, error.to_string(), false))?;
    campaign.ensure_gameplay_defaults();
    let state_hash = hash_json(&campaign)?;
    let campaign_json = serde_json::to_value(&campaign)
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    sqlx::query::query(
        "insert into trnm_online_campaigns (
            campaign_id, player_id, account_id, slot_key, campaign_revision,
            schema_revision, state_hash, campaign_json
         ) values ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(&campaign.campaign_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.slot_key)
    .bind(campaign.revision as i64)
    .bind(i32::from(campaign.schema_revision))
    .bind(&state_hash)
    .bind(campaign_json)
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    Ok(Json(OnlineCampaignView {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        campaign_id: campaign.campaign_id,
        player_id: request.player_id,
        account_id: request.account_id,
        slot_key: request.slot_key,
        campaign_revision: campaign.revision,
        schema_revision: campaign.schema_revision,
        state_hash,
        level: campaign.progression.level,
        experience: campaign.progression.experience,
        reputation: campaign.character.attributes.reputation,
        inventory: campaign
            .progression
            .inventory
            .iter()
            .map(|stack| OnlineInventoryStack {
                item_id: stack.item_id.clone(),
                quantity: stack.quantity,
            })
            .collect(),
        settled_match_count: campaign.settled_battle_ids.len(),
    }))
}

async fn create_lobby(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineLobbyCreateRequest>,
) -> Result<Json<OnlineLobbyView>, ApiError> {
    validate_product_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    mission_for_map(&request.map_id)?;
    if request.display_name.trim().is_empty() || request.display_name.chars().count() > 80 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "lobby display_name must contain 1..80 characters",
            false,
        ));
    }
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    lock_player_lobby_scope(&mut transaction, &request.player_id).await?;
    ensure_campaign_owner(
        &mut transaction,
        &request.campaign_id,
        &request.player_id,
        account_id,
    )
    .await?;
    ensure_player_has_no_active_lobby(&mut transaction, &request.player_id).await?;
    let lobby_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_lobbies (
            lobby_id, display_name, owner_player_id, owner_account_id, map_id
         ) values ($1, $2, $3, $4, $5)",
    )
    .bind(lobby_id)
    .bind(request.display_name.trim())
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.map_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_lobby_members (
            lobby_id, player_id, account_id, campaign_id, member_role
         ) values ($1, $2, $3, $4, 'owner')",
    )
    .bind(lobby_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.campaign_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_lobby_view(&state.pool, lobby_id).await?))
}

async fn get_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineLobbyAccessRequest>,
) -> Result<Json<OnlineLobbyView>, ApiError> {
    validate_product_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let member: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_lobby_members
         where lobby_id = $1 and player_id = $2 and account_id = $3)",
    )
    .bind(lobby_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if !member {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "identity is not a lobby member",
            false,
        ));
    }
    Ok(Json(fetch_lobby_view(&state.pool, lobby_id).await?))
}

async fn invite_to_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineLobbyInviteRequest>,
) -> Result<Json<OnlineLobbyInviteReceipt>, ApiError> {
    validate_product_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    if request.target_player_id == request.player_id {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "lobby owner cannot invite itself",
            false,
        ));
    }
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let lobby = lock_lobby(&mut transaction, lobby_id).await?;
    require_lobby_owner(&lobby, &request.player_id, account_id)?;
    require_open_lobby_revision(&lobby, request.expected_lobby_revision)?;
    let blocked: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_blocks
         where (blocker_player_id = $1 and blocked_player_id = $2)
            or (blocker_player_id = $2 and blocked_player_id = $1))",
    )
    .bind(&request.player_id)
    .bind(&request.target_player_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if blocked {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "lobby invitation is blocked by a player safety rule",
            false,
        ));
    }
    let member_count: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_lobby_members where lobby_id = $1",
    )
    .bind(lobby_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if member_count != 1 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "lobby already has the maximum two members",
            false,
        ));
    }
    ensure_player_has_no_active_lobby(&mut transaction, &request.target_player_id).await?;
    let invite_id = Uuid::new_v4();
    let invite_token = format!("trnm-invite-{}", Uuid::new_v4());
    let invite_token_hash = sha256_text(&invite_token);
    let expires_at_epoch = Utc::now().timestamp().saturating_add(900);
    sqlx::query::query(
        "insert into trnm_online_lobby_invites (
            invite_id, lobby_id, inviter_player_id, target_player_id,
            invite_token_hash, expires_at
         ) values ($1, $2, $3, $4, $5, to_timestamp($6))",
    )
    .bind(invite_id)
    .bind(lobby_id)
    .bind(&request.player_id)
    .bind(&request.target_player_id)
    .bind(invite_token_hash)
    .bind(expires_at_epoch)
    .execute(&mut *transaction)
    .await
    .map_err(|error| {
        if error
            .as_database_error()
            .is_some_and(|db| db.is_unique_violation())
        {
            return api_error(
                StatusCode::CONFLICT,
                "target player already has a pending invite to this lobby",
                true,
            );
        }
        internal_db(error)
    })?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineLobbyInviteReceipt {
        lobby: fetch_lobby_view(&state.pool, lobby_id).await?,
        invite_id: invite_id.to_string(),
        invite_token,
        target_player_id: request.target_player_id,
        expires_at_epoch,
    }))
}

async fn accept_lobby_invite(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineLobbyInviteAcceptRequest>,
) -> Result<Json<OnlineLobbyView>, ApiError> {
    validate_product_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    lock_player_lobby_scope(&mut transaction, &request.player_id).await?;
    ensure_campaign_owner(
        &mut transaction,
        &request.campaign_id,
        &request.player_id,
        account_id,
    )
    .await?;
    ensure_player_has_no_active_lobby(&mut transaction, &request.player_id).await?;
    let invite = sqlx::query::query(
        "select invite_id, lobby_id, target_player_id, status,
                extract(epoch from expires_at)::bigint as expires_at_epoch
         from trnm_online_lobby_invites where invite_token_hash = $1 for update",
    )
    .bind(sha256_text(&request.invite_token))
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "lobby invite not found", false))?;
    let lobby_id: Uuid = invite.try_get("lobby_id").map_err(internal_db)?;
    let target_player_id: String = invite.try_get("target_player_id").map_err(internal_db)?;
    let status: String = invite.try_get("status").map_err(internal_db)?;
    let expires_at_epoch: i64 = invite.try_get("expires_at_epoch").map_err(internal_db)?;
    if target_player_id != request.player_id
        || status != "pending"
        || expires_at_epoch <= Utc::now().timestamp()
    {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "lobby invite is expired, consumed, or belongs to another player",
            false,
        ));
    }
    let lobby = lock_lobby(&mut transaction, lobby_id).await?;
    if lobby.try_get::<String, _>("status").map_err(internal_db)? != "open" {
        return Err(api_error(
            StatusCode::CONFLICT,
            "lobby is no longer accepting members",
            false,
        ));
    }
    let member_count: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_lobby_members where lobby_id = $1",
    )
    .bind(lobby_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if member_count != 1 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "lobby already has the maximum two members",
            false,
        ));
    }
    sqlx::query::query(
        "insert into trnm_online_lobby_members (
            lobby_id, player_id, account_id, campaign_id, member_role
         ) values ($1, $2, $3, $4, 'member')",
    )
    .bind(lobby_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.campaign_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "update trnm_online_lobby_invites set status = 'accepted', accepted_at = now()
         where invite_id = $1",
    )
    .bind(
        invite
            .try_get::<Uuid, _>("invite_id")
            .map_err(internal_db)?,
    )
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "update trnm_online_lobbies set lobby_revision = lobby_revision + 1,
             updated_at = now() where lobby_id = $1",
    )
    .bind(lobby_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_lobby_view(&state.pool, lobby_id).await?))
}

async fn set_lobby_ready(
    State(state): State<AppState>,
    Path(lobby_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineLobbyReadyRequest>,
) -> Result<Json<OnlineLobbyView>, ApiError> {
    validate_product_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let lobby = lock_lobby(&mut transaction, lobby_id).await?;
    require_open_lobby_revision(&lobby, request.expected_lobby_revision)?;
    let updated = sqlx::query::query(
        "update trnm_online_lobby_members set ready = $4
         where lobby_id = $1 and player_id = $2 and account_id = $3",
    )
    .bind(lobby_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(request.ready)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "identity is not a lobby member",
            false,
        ));
    }
    sqlx::query::query(
        "update trnm_online_lobbies set lobby_revision = lobby_revision + 1,
             updated_at = now() where lobby_id = $1",
    )
    .bind(lobby_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_lobby_view(&state.pool, lobby_id).await?))
}

async fn queue_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineLobbyQueueRequest>,
) -> Result<Json<OnlineMatchmakingReceipt>, ApiError> {
    validate_product_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let lobby = lock_lobby(&mut transaction, lobby_id).await?;
    require_lobby_owner(&lobby, &request.player_id, account_id)?;
    require_open_lobby_revision(&lobby, request.expected_lobby_revision)?;
    let members = sqlx::query::query(
        "select player_id, account_id, campaign_id, member_role, ready
         from trnm_online_lobby_members where lobby_id = $1
         order by case member_role when 'owner' then 0 else 1 end for update",
    )
    .bind(lobby_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if members.len() != 2
        || members
            .iter()
            .any(|member| !member.try_get::<bool, _>("ready").unwrap_or(false))
    {
        return Err(api_error(
            StatusCode::CONFLICT,
            "coop matchmaking requires exactly two ready members",
            true,
        ));
    }
    let match_id = Uuid::new_v4();
    let join_code = match_id.simple().to_string()[..10].to_ascii_uppercase();
    let map_id: String = lobby.try_get("map_id").map_err(internal_db)?;
    let host_campaign_id: String = members[0].try_get("campaign_id").map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_matches (
            match_id, campaign_id, host_player_id, host_account_id, join_code,
            phase, build_id, map_id, rules_version
         ) values ($1, $2, $3, $4, $5, 'waiting', $6, $7, $8)",
    )
    .bind(match_id)
    .bind(&host_campaign_id)
    .bind(
        members[0]
            .try_get::<String, _>("player_id")
            .map_err(internal_db)?,
    )
    .bind(
        members[0]
            .try_get::<Uuid, _>("account_id")
            .map_err(internal_db)?,
    )
    .bind(&join_code)
    .bind(ONLINE_AUTHORITY_BUILD)
    .bind(&map_id)
    .bind(trnm_campaign_core::FIRST_CONTACT_RULES_VERSION)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    for (index, member) in members.iter().enumerate() {
        sqlx::query::query(
            "insert into trnm_online_match_members (
                match_id, player_id, account_id, campaign_id, member_role
             ) values ($1, $2, $3, $4, $5)",
        )
        .bind(match_id)
        .bind(
            member
                .try_get::<String, _>("player_id")
                .map_err(internal_db)?,
        )
        .bind(
            member
                .try_get::<Uuid, _>("account_id")
                .map_err(internal_db)?,
        )
        .bind(
            member
                .try_get::<String, _>("campaign_id")
                .map_err(internal_db)?,
        )
        .bind(if index == 0 { "host" } else { "coop_guest" })
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    }
    sqlx::query::query(
        "update trnm_online_lobbies set status = 'queued',
             lobby_revision = lobby_revision + 1, updated_at = now()
         where lobby_id = $1",
    )
    .bind(lobby_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;

    let started = match start_match(
        State(state.clone()),
        Path(match_id),
        headers,
        Json(OnlineMatchStartRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: request.player_id.clone(),
            account_id: request.account_id.clone(),
            expected_match_revision: 0,
        }),
    )
    .await
    {
        Ok(Json(view)) => view,
        Err(error) => {
            let mut cleanup = state.pool.begin().await.map_err(internal_db)?;
            sqlx::query::query("delete from trnm_online_matches where match_id = $1")
                .bind(match_id)
                .execute(&mut *cleanup)
                .await
                .map_err(internal_db)?;
            sqlx::query::query(
                "update trnm_online_lobbies set status = 'open',
                     lobby_revision = lobby_revision + 1, updated_at = now()
                 where lobby_id = $1",
            )
            .bind(lobby_id)
            .execute(&mut *cleanup)
            .await
            .map_err(internal_db)?;
            cleanup.commit().await.map_err(internal_db)?;
            return Err(error);
        }
    };
    let allocation_id = Uuid::new_v4();
    let mut allocation = state.pool.begin().await.map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_matchmaking_allocations (
            allocation_id, lobby_id, match_id, queue_mode, member_count
         ) values ($1, $2, $3, 'coop_vs_ai', 2)",
    )
    .bind(allocation_id)
    .bind(lobby_id)
    .bind(match_id)
    .execute(&mut *allocation)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "update trnm_online_lobbies set status = 'matched', match_id = $2,
             lobby_revision = lobby_revision + 1, updated_at = now()
         where lobby_id = $1 and status = 'queued'",
    )
    .bind(lobby_id)
    .bind(match_id)
    .execute(&mut *allocation)
    .await
    .map_err(internal_db)?;
    allocation.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineMatchmakingReceipt {
        lobby: fetch_lobby_view(&state.pool, lobby_id).await?,
        match_view: started,
        queue_mode: "coop_vs_ai".to_string(),
        allocation_id: allocation_id.to_string(),
    }))
}

async fn create_match(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineMatchCreateRequest>,
) -> Result<Json<OnlineMatchView>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    mission_for_map(&request.map_id)?;
    let campaign_row = sqlx::query::query(
        "select player_id, account_id, campaign_revision from trnm_online_campaigns
         where campaign_id = $1",
    )
    .bind(&request.campaign_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "campaign not found", false))?;
    let player_id: String = campaign_row.try_get("player_id").map_err(internal_db)?;
    let account_id: Uuid = campaign_row.try_get("account_id").map_err(internal_db)?;
    let revision: i64 = campaign_row
        .try_get("campaign_revision")
        .map_err(internal_db)?;
    if revision as u64 != request.expected_campaign_revision {
        return Err(conflict("campaign revision changed", revision as u64));
    }
    verify_identity(&state, &headers, &player_id, &account_id.to_string()).await?;

    let match_id = Uuid::new_v4();
    let join_code = match_id.simple().to_string()[..10].to_ascii_uppercase();
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_matches (
            match_id, campaign_id, host_player_id, host_account_id, join_code,
            phase, build_id, map_id, rules_version
         ) values ($1, $2, $3, $4, $5, 'waiting', $6, $7, $8)",
    )
    .bind(match_id)
    .bind(&request.campaign_id)
    .bind(&player_id)
    .bind(account_id)
    .bind(&join_code)
    .bind(ONLINE_AUTHORITY_BUILD)
    .bind(&request.map_id)
    .bind(trnm_campaign_core::FIRST_CONTACT_RULES_VERSION)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_match_members (
            match_id, player_id, account_id, campaign_id, member_role
         ) values ($1, $2, $3, $4, 'host')",
    )
    .bind(match_id)
    .bind(&player_id)
    .bind(account_id)
    .bind(&request.campaign_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_match_view(&state.pool, match_id).await?))
}

async fn join_match(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineMatchJoinRequest>,
) -> Result<Json<OnlineMatchView>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let campaign_owner = sqlx::query::query(
        "select player_id, account_id from trnm_online_campaigns where campaign_id = $1",
    )
    .bind(&request.campaign_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "guest campaign not found", false))?;
    let campaign_player_id: String = campaign_owner.try_get("player_id").map_err(internal_db)?;
    let campaign_account_id: Uuid = campaign_owner.try_get("account_id").map_err(internal_db)?;
    if campaign_player_id != request.player_id || campaign_account_id != account_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "guest campaign does not belong to the authenticated player/account",
            false,
        ));
    }
    let row = sqlx::query::query(
        "select match_id, phase from trnm_online_matches where join_code = $1 for update",
    )
    .bind(request.join_code.to_ascii_uppercase())
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "join code not found", false))?;
    let match_id: Uuid = row.try_get("match_id").map_err(internal_db)?;
    let phase: String = row.try_get("phase").map_err(internal_db)?;
    if phase != "waiting" {
        return Err(api_error(
            StatusCode::CONFLICT,
            "match is no longer accepting members",
            false,
        ));
    }
    sqlx::query::query(
        "insert into trnm_online_match_members (
            match_id, player_id, account_id, campaign_id, member_role
         ) values ($1, $2, $3, $4, 'coop_guest')",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.campaign_id)
    .execute(&mut *transaction)
    .await
    .map_err(|error| {
        if let sqlx::Error::Database(database) = &error {
            if database.is_unique_violation() {
                return api_error(
                    StatusCode::CONFLICT,
                    "match already has a co-op guest or this identity already joined",
                    false,
                );
            }
        }
        internal_db(error)
    })?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_match_view(&state.pool, match_id).await?))
}

async fn start_match(
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineMatchStartRequest>,
) -> Result<Json<OnlineMatchView>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    lock_current_fleet_epoch(&mut transaction, &state, false)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?;
    let match_row = sqlx::query::query(
        "select campaign_id, host_player_id, host_account_id, phase, map_id, match_revision,
                match_mode
         from trnm_online_matches where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let host_player_id: String = match_row.try_get("host_player_id").map_err(internal_db)?;
    let host_account_id: Uuid = match_row.try_get("host_account_id").map_err(internal_db)?;
    if host_player_id != request.player_id || host_account_id != account_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "only the authenticated host may start the match",
            false,
        ));
    }
    let phase: String = match_row.try_get("phase").map_err(internal_db)?;
    let revision: i64 = match_row.try_get("match_revision").map_err(internal_db)?;
    if phase != "waiting" {
        return Err(api_error(
            StatusCode::CONFLICT,
            "match is not waiting",
            false,
        ));
    }
    if revision as u64 != request.expected_match_revision {
        return Err(conflict("match revision changed", revision as u64));
    }
    let member_count: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_match_members where match_id = $1",
    )
    .bind(match_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if member_count != 2 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "Online Authority v2 requires exactly host + one co-op guest",
            true,
        ));
    }
    let map_id: String = match_row.try_get("map_id").map_err(internal_db)?;
    let match_mode: String = match_row.try_get("match_mode").map_err(internal_db)?;
    let map = map::load_authoritative_map(&state.asset_root, &map_id)
        .map_err(|error| api_error(StatusCode::BAD_REQUEST, error, false))?;
    let member_rows = sqlx::query::query(
        "select player_id, account_id, member_role, campaign_id
         from trnm_online_match_members where match_id = $1
         order by case member_role when 'host' then 0 else 1 end for update",
    )
    .bind(match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if member_rows.len() != 2 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "both online members must bind a cloud campaign",
            true,
        ));
    }
    let mut prepared = Vec::with_capacity(2);
    for member in &member_rows {
        let campaign_id: Option<String> = member.try_get("campaign_id").map_err(internal_db)?;
        let campaign_id = campaign_id.ok_or_else(|| {
            api_error(
                StatusCode::CONFLICT,
                "online member is missing a cloud campaign",
                false,
            )
        })?;
        let campaign_value: Value = sqlx::query_scalar::query_scalar(
            "select campaign_json from trnm_online_campaigns where campaign_id = $1 for update",
        )
        .bind(&campaign_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(internal_db)?;
        let mut campaign: CampaignSaveV1 =
            serde_json::from_value(campaign_value).map_err(|error| {
                api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false)
            })?;
        let ranked_campaign = (match_mode == "ranked_pvp").then(|| campaign.clone());
        let seed = prepare_campaign_seed(&mut campaign, &map_id, map.clone())?;
        if let Some(original) = ranked_campaign {
            campaign = original;
        }
        prepared.push((campaign_id, campaign, seed));
    }
    let (host_units, host_id_map) = member_units(&prepared[0].2, "host", 0);
    let (guest_units, guest_id_map) = member_units(&prepared[1].2, "guest", 2);
    let mut seed = prepared[0].2.clone();
    seed.battle_id = format!("online-v2-{match_id}");
    seed.party = host_units
        .iter()
        .chain(guest_units.iter())
        .cloned()
        .collect();
    seed.seed_hash = seed
        .computed_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    seed.validate()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    let mut sim = MissionSimV1::from_seed(seed.clone())
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    if match_mode == "ranked_pvp" {
        let guest_control = guest_units
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        sim.enable_human_enemy_authority(&guest_control)
            .map_err(|error| {
                api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false)
            })?;
    }
    let snapshot_hash = sim
        .snapshot_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    let match_season_id = if match_mode == "ranked_pvp" {
        Some(
            operations_v1::active_season(&mut transaction)
                .await
                .map_err(|error| api_error(StatusCode::CONFLICT, error, true))?
                .0,
        )
    } else {
        None
    };
    for (index, (campaign_id, campaign, member_seed)) in prepared.iter().enumerate() {
        let (units, id_map) = if index == 0 {
            (&host_units, &host_id_map)
        } else {
            (&guest_units, &guest_id_map)
        };
        sqlx::query::query(
            "update trnm_online_match_members set controlled_unit_ids = $2,
                settlement_seed_json = $3, unit_id_map = $4
             where match_id = $1 and campaign_id = $5",
        )
        .bind(match_id)
        .bind(json!(units
            .iter()
            .map(|unit| &unit.unit_id)
            .collect::<Vec<_>>()))
        .bind(serde_json::to_value(member_seed).map_err(internal_serialization)?)
        .bind(serde_json::to_value(id_map).map_err(internal_serialization)?)
        .bind(campaign_id)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
        persist_campaign(&mut transaction, campaign).await?;
    }
    let initial_simulation = serde_json::to_value(&sim).map_err(internal_serialization)?;
    sqlx::query::query(
        "update trnm_online_matches set
            phase = 'running', seed_hash = $2, seed_json = $3,
            simulation_json = $4, snapshot_hash = $5,
            authoritative_tick = 0, checkpoint_sequence = 0,
            match_revision = match_revision + 1,
            assigned_instance_id = $6, assigned_region = $7,
            assigned_instance_epoch = $8, initial_simulation_json = $4,
            season_id = $9, assigned_physical_host_id = $10, updated_at = now()
         where match_id = $1",
    )
    .bind(match_id)
    .bind(&seed.seed_hash)
    .bind(serde_json::to_value(&seed).map_err(internal_serialization)?)
    .bind(&initial_simulation)
    .bind(&snapshot_hash)
    .bind(state.instance_id.as_str())
    .bind(state.region.as_str())
    .bind(state.instance_epoch)
    .bind(&match_season_id)
    .bind(state.physical_host_id.as_str())
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_replay_frames (
            match_id, tick, snapshot_hash, simulation_json, frame_kind
         ) values ($1, 0, $2, $3, 'initial')
         on conflict (match_id, tick) do nothing",
    )
    .bind(match_id)
    .bind(&snapshot_hash)
    .bind(initial_simulation)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    ensure_match_actor(&state, match_id)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?
        .ok_or_else(|| {
            api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "running match actor did not start",
                true,
            )
        })?;
    Ok(Json(fetch_match_view(&state.pool, match_id).await?))
}

async fn submit_command(
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineCommandSubmitRequest>,
) -> Result<Json<OnlineCommandReceipt>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    validate_command_id(&request.command_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let request_hash = hash_json(&request)?;
    let member = sqlx::query::query(
        "select controlled_unit_ids, member_role from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| {
        api_error(
            StatusCode::FORBIDDEN,
            "identity is not a match member",
            false,
        )
    })?;
    let controlled_value: Value = member.try_get("controlled_unit_ids").map_err(internal_db)?;
    let controlled = serde_json::from_value::<Vec<String>>(controlled_value)
        .map_err(internal_serialization)?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let requested_subjects = request
        .order
        .subject_actor_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if requested_subjects.is_empty() || !requested_subjects.is_subset(&controlled) {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "command subjects are outside this member's authoritative control set",
            false,
        ));
    }
    let member_role: String = member.try_get("member_role").map_err(internal_db)?;
    if let Some(receipt) = fetch_duplicate_command_receipt(
        &state,
        match_id,
        &request.command_id,
        &request.player_id,
        &request_hash,
    )
    .await?
    {
        return Ok(Json(receipt));
    }
    let actor = ensure_match_actor(&state, match_id)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?
        .ok_or_else(|| api_error(StatusCode::CONFLICT, "match is not running", false))?;
    let (response_tx, response_rx) = oneshot::channel();
    tokio::time::timeout(
        Duration::from_secs(5),
        actor.commands.send(ActorCommandEnvelope {
            request,
            request_hash,
            controlled_unit_ids: controlled,
            member_role,
            response: response_tx,
        }),
    )
    .await
    .map_err(|_| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "match actor command queue timed out",
            true,
        )
    })?
    .map_err(|_| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "match actor stopped before accepting the command",
            true,
        )
    })?;
    let receipt = tokio::time::timeout(Duration::from_secs(5), response_rx)
        .await
        .map_err(|_| {
            api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "match actor command persistence timed out",
                true,
            )
        })?
        .map_err(|_| {
            api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "match actor stopped before command acknowledgement",
                true,
            )
        })??;
    Ok(Json(receipt))
}

async fn fetch_duplicate_command_receipt(
    state: &AppState,
    match_id: Uuid,
    command_id: &str,
    player_id: &str,
    request_hash: &str,
) -> Result<Option<OnlineCommandReceipt>, ApiError> {
    let Some(row) = sqlx::query::query(
        "select c.sequence, c.player_id, c.request_hash,
                c.accepted_match_revision, c.accepted_snapshot_hash, c.target_tick,
                m.match_revision as current_match_revision
         from trnm_online_commands c
         join trnm_online_matches m on m.match_id = c.match_id
         where c.match_id = $1 and c.command_id = $2",
    )
    .bind(match_id)
    .bind(command_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    else {
        return Ok(None);
    };
    let stored_player_id: String = row.try_get("player_id").map_err(internal_db)?;
    let stored_request_hash: Option<String> = row.try_get("request_hash").map_err(internal_db)?;
    if stored_player_id != player_id || stored_request_hash.as_deref() != Some(request_hash) {
        return Err(conflict(
            "command_id was already used with a different authenticated request",
            row.try_get::<i64, _>("current_match_revision")
                .map_err(internal_db)? as u64,
        ));
    }
    Ok(Some(OnlineCommandReceipt {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        match_id: match_id.to_string(),
        command_id: command_id.to_string(),
        sequence: row.try_get::<i64, _>("sequence").map_err(internal_db)? as u64,
        duplicate: true,
        accepted_tick: row.try_get::<i64, _>("target_tick").map_err(internal_db)? as u64,
        match_revision: row
            .try_get::<i64, _>("accepted_match_revision")
            .map_err(internal_db)? as u64,
        snapshot_hash: row.try_get("accepted_snapshot_hash").map_err(internal_db)?,
    }))
}

async fn accept_actor_command(
    state: &AppState,
    match_id: Uuid,
    loaded: &mut LoadedMatchActor,
    request: OnlineCommandSubmitRequest,
    request_hash: String,
    controlled_unit_ids: &BTreeSet<String>,
    member_role: &str,
) -> Result<OnlineCommandReceipt, ApiError> {
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    lock_current_fleet_epoch(&mut transaction, state, true)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?;
    let row = sqlx::query::query(
        "select phase, next_sequence, match_revision,
                assigned_instance_id, assigned_instance_epoch
         from trnm_online_matches where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let durable_revision = row
        .try_get::<i64, _>("match_revision")
        .map_err(internal_db)? as u64;
    if let Some(command) = sqlx::query::query(
        "select sequence, player_id, request_hash, accepted_match_revision,
                accepted_snapshot_hash, target_tick
         from trnm_online_commands where match_id = $1 and command_id = $2",
    )
    .bind(match_id)
    .bind(&request.command_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    {
        let stored_player_id: String = command.try_get("player_id").map_err(internal_db)?;
        let stored_request_hash: Option<String> =
            command.try_get("request_hash").map_err(internal_db)?;
        if stored_player_id != request.player_id
            || stored_request_hash.as_deref() != Some(request_hash.as_str())
        {
            return Err(conflict(
                "command_id was already used with a different authenticated request",
                durable_revision,
            ));
        }
        transaction.commit().await.map_err(internal_db)?;
        return Ok(OnlineCommandReceipt {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            match_id: match_id.to_string(),
            command_id: request.command_id,
            sequence: command.try_get::<i64, _>("sequence").map_err(internal_db)? as u64,
            duplicate: true,
            accepted_tick: command
                .try_get::<i64, _>("target_tick")
                .map_err(internal_db)? as u64,
            match_revision: command
                .try_get::<i64, _>("accepted_match_revision")
                .map_err(internal_db)? as u64,
            snapshot_hash: command
                .try_get("accepted_snapshot_hash")
                .map_err(internal_db)?,
        });
    }
    let phase: String = row.try_get("phase").map_err(internal_db)?;
    let durable_next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(internal_db)? as u64;
    let assigned_instance: Option<String> =
        row.try_get("assigned_instance_id").map_err(internal_db)?;
    let assigned_epoch: i64 = row
        .try_get("assigned_instance_epoch")
        .map_err(internal_db)?;
    if phase != "running" {
        return Err(api_error(
            StatusCode::CONFLICT,
            "match is not running",
            false,
        ));
    }
    if assigned_instance.as_deref() != Some(state.instance_id.as_str())
        || assigned_epoch != state.instance_epoch
    {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "match actor was fenced by newer fleet authority",
            true,
        ));
    }
    if durable_next_sequence != loaded.next_sequence || durable_revision != loaded.match_revision {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "match actor event cursor diverged from durable authority",
            true,
        ));
    }
    if request.expected_match_revision != loaded.match_revision {
        return Err(conflict("match revision changed", loaded.match_revision));
    }
    if request.sequence != loaded.next_sequence {
        return Err(conflict(
            format!("expected command sequence {}", loaded.next_sequence),
            loaded.match_revision,
        ));
    }
    if request.target_tick < loaded.simulation.tick
        || request.target_tick > loaded.simulation.tick.saturating_add(200)
    {
        return Err(conflict(
            "target_tick is outside the authoritative window",
            loaded.match_revision,
        ));
    }
    let requested_subjects = request
        .order
        .subject_actor_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if requested_subjects.is_empty() || !requested_subjects.is_subset(controlled_unit_ids) {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "command subjects are outside this member's authoritative control set",
            false,
        ));
    }
    let mut candidate = loaded.simulation.clone();
    let order = prepare_and_apply_actor_order(
        &mut candidate,
        &loaded.match_mode,
        member_role,
        request.target_tick,
        request.order,
    )?;
    let snapshot_hash = candidate
        .snapshot_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    let accepted_revision = loaded.match_revision.saturating_add(1);
    let post_simulation = serde_json::to_value(&candidate).map_err(internal_serialization)?;
    sqlx::query::query(
        "insert into trnm_online_commands (
            match_id, sequence, command_id, player_id, request_hash, target_tick,
            order_json, accepted_snapshot_hash, accepted_match_revision,
            post_simulation_json
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(match_id)
    .bind(request.sequence as i64)
    .bind(&request.command_id)
    .bind(&request.player_id)
    .bind(&request_hash)
    .bind(request.target_tick as i64)
    .bind(serde_json::to_value(order).map_err(internal_serialization)?)
    .bind(&snapshot_hash)
    .bind(accepted_revision as i64)
    .bind(post_simulation)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let updated = sqlx::query::query(
        "update trnm_online_matches set next_sequence = $2,
            match_revision = $3, updated_at = now()
         where match_id = $1 and phase = 'running'
           and assigned_instance_id = $4 and assigned_instance_epoch = $5
           and next_sequence = $6 and match_revision = $7",
    )
    .bind(match_id)
    .bind(loaded.next_sequence.saturating_add(1) as i64)
    .bind(accepted_revision as i64)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(loaded.next_sequence as i64)
    .bind(loaded.match_revision as i64)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "command event was fenced before commit",
            true,
        ));
    }
    transaction.commit().await.map_err(internal_db)?;
    loaded.simulation = candidate;
    loaded.next_sequence = loaded.next_sequence.saturating_add(1);
    loaded.match_revision = accepted_revision;
    Ok(OnlineCommandReceipt {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        match_id: match_id.to_string(),
        command_id: request.command_id,
        sequence: request.sequence,
        duplicate: false,
        accepted_tick: request.target_tick,
        match_revision: accepted_revision,
        snapshot_hash,
    })
}

fn prepare_and_apply_actor_order(
    simulation: &mut MissionSimV1,
    match_mode: &str,
    member_role: &str,
    target_tick: u64,
    mut order: trnm_rts_protocol::RtsFrameOrder,
) -> Result<trnm_rts_protocol::RtsFrameOrder, ApiError> {
    order.player_id = if match_mode == "ranked_pvp" && member_role == "coop_guest" {
        "enemy-player".to_string()
    } else {
        "player".to_string()
    };
    order.frame = u32::try_from(target_tick).map_err(|_| {
        api_error(
            StatusCode::BAD_REQUEST,
            "target_tick exceeds frame range",
            false,
        )
    })?;
    order.source = RtsOrderSource::LocalInput;
    if match_mode == "ranked_pvp" && order.kind == trnm_rts_protocol::RtsOrderKind::Extract {
        return Err(api_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "ranked PvP does not allow withdrawal",
            false,
        ));
    }
    let merged_with_active_coop_order = match_mode == "coop_vs_ai"
        && order.queued
        && simulation.active_order.as_ref().is_some_and(|active| {
            active.kind == order.kind
                && active.target_tile == order.target_tile
                && active.target_actor_id == order.target_actor_id
                && active.target_rule_id == order.target_rule_id
        });
    if merged_with_active_coop_order {
        let active = simulation
            .active_order
            .as_mut()
            .expect("compatible active order was checked above");
        active
            .subject_actor_ids
            .extend(order.subject_actor_ids.clone());
        active.subject_actor_ids.sort();
        active.subject_actor_ids.dedup();
    } else if match_mode == "ranked_pvp" && member_role == "coop_guest" {
        simulation
            .issue_human_enemy_order(order.clone())
            .map_err(|error| {
                api_error(StatusCode::UNPROCESSABLE_ENTITY, error.to_string(), false)
            })?;
    } else {
        simulation.issue_order(order.clone()).map_err(|error| {
            api_error(StatusCode::UNPROCESSABLE_ENTITY, error.to_string(), false)
        })?;
    }
    Ok(order)
}

async fn published_actor_state(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<PublishedMatchState>, ApiError> {
    let handle = ensure_match_actor(state, match_id)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?;
    Ok(handle.map(|actor| actor.published.borrow().clone()))
}

fn apply_published_actor_view(view: &mut OnlineMatchView, published: &PublishedMatchState) {
    view.authoritative_tick = published.simulation.tick;
    view.next_sequence = published.next_sequence;
    view.match_revision = published.match_revision;
    view.snapshot_hash = published.snapshot_hash.as_ref().clone();
}

async fn get_snapshot(
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineMatchAccessRequest>,
) -> Result<Json<OnlineSnapshotResponse>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let member: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if member != 1 {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "identity is not a match member",
            false,
        ));
    }
    let mut view = fetch_match_view(&state.pool, match_id).await?;
    let published = published_actor_state(&state, match_id).await?;
    let snapshot = if let Some(published) = published {
        apply_published_actor_view(&mut view, &published);
        serde_json::to_value(published.simulation.as_ref()).map_err(internal_serialization)?
    } else {
        let snapshot = sqlx::query_scalar::query_scalar(
            "select simulation_json from trnm_online_matches where match_id = $1",
        )
        .bind(match_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(internal_db)?
        .flatten()
        .unwrap_or(Value::Null);
        // The actor can publish terminal state and be removed between the
        // initial view read and this fallback. Its terminal checkpoint updates
        // the durable view and simulation atomically, so refresh the view after
        // reading that durable simulation instead of returning a stale running
        // view paired with the new terminal snapshot.
        view = fetch_match_view(&state.pool, match_id).await?;
        snapshot
    };
    Ok(Json(OnlineSnapshotResponse { view, snapshot }))
}

async fn reconnect_match(
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<OnlineReconnectRequest>,
) -> Result<Json<OnlineReconnectResponse>, ApiError> {
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let published = published_actor_state(&state, match_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let member = sqlx::query::query(
        "select reconnect_count from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3 for update",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| {
        api_error(
            StatusCode::FORBIDDEN,
            "identity is not a match member",
            false,
        )
    })?;
    let match_row = sqlx::query::query(
        "select simulation_json, snapshot_hash, next_sequence, match_revision
         from trnm_online_matches where match_id = $1 for share",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let next_sequence = if let Some(published) = &published {
        published.next_sequence
    } else {
        match_row
            .try_get::<i64, _>("next_sequence")
            .map_err(internal_db)? as u64
    };
    let match_revision = if let Some(published) = &published {
        published.match_revision
    } else {
        match_row
            .try_get::<i64, _>("match_revision")
            .map_err(internal_db)? as u64
    };
    if request.last_acknowledged_sequence > next_sequence {
        return Err(conflict(
            "client acknowledged a command sequence beyond server authority",
            match_revision,
        ));
    }
    let snapshot_hash: String = if let Some(published) = &published {
        published.snapshot_hash.as_ref().clone()
    } else {
        match_row.try_get("snapshot_hash").map_err(internal_db)?
    };
    let command_rows = sqlx::query::query(
        "select sequence, command_id, target_tick, accepted_match_revision,
                accepted_snapshot_hash
         from trnm_online_commands
         where match_id = $1 and sequence >= $2
         order by sequence asc limit 256",
    )
    .bind(match_id)
    .bind(request.last_acknowledged_sequence as i64)
    .fetch_all(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let replayed_commands = command_rows
        .into_iter()
        .map(|row| {
            Ok(OnlineCommandReceipt {
                protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                match_id: match_id.to_string(),
                command_id: row.try_get("command_id").map_err(internal_db)?,
                sequence: row.try_get::<i64, _>("sequence").map_err(internal_db)? as u64,
                duplicate: true,
                accepted_tick: row.try_get::<i64, _>("target_tick").map_err(internal_db)? as u64,
                match_revision: row
                    .try_get::<i64, _>("accepted_match_revision")
                    .map_err(internal_db)? as u64,
                snapshot_hash: row.try_get("accepted_snapshot_hash").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let reconnect_count = member
        .try_get::<i64, _>("reconnect_count")
        .map_err(internal_db)? as u64
        + 1;
    sqlx::query::query(
        "update trnm_online_match_members set reconnect_count = $4,
            last_acknowledged_sequence = $5, last_snapshot_hash = $6, last_seen_at = now()
         where match_id = $1 and player_id = $2 and account_id = $3",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(reconnect_count as i64)
    .bind(next_sequence as i64)
    .bind(&snapshot_hash)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let snapshot = if let Some(published) = &published {
        serde_json::to_value(published.simulation.as_ref()).map_err(internal_serialization)?
    } else {
        match_row
            .try_get::<Option<Value>, _>("simulation_json")
            .map_err(internal_db)?
            .unwrap_or(Value::Null)
    };
    transaction.commit().await.map_err(internal_db)?;
    let mut view = fetch_match_view(&state.pool, match_id).await?;
    if let Some(published) = &published {
        apply_published_actor_view(&mut view, published);
    }
    Ok(Json(OnlineReconnectResponse {
        view,
        snapshot,
        replayed_commands,
        reconnect_count,
        full_snapshot_required: request.last_snapshot_hash != snapshot_hash,
    }))
}

async fn apply_member_progression(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    match_id: Uuid,
    combined_result: &BattleResultV1,
    combined_result_hash: &str,
) -> Result<bool, String> {
    let match_mode: String = sqlx::query_scalar::query_scalar(
        "select match_mode from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let members = sqlx::query::query(
        "select player_id, account_id, campaign_id, member_role,
                settlement_seed_json, unit_id_map
         from trnm_online_match_members where match_id = $1 order by member_role for update",
    )
    .bind(match_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    if members.len() != 2 {
        return Err("terminal online match is missing one member campaign".to_string());
    }
    let mut participant_ids = members
        .iter()
        .map(|member| member.try_get::<String, _>("player_id"))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    participant_ids.sort();
    let participants_hash = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&participant_ids).map_err(|error| error.to_string())?)
    );
    let mut any_pending = false;
    for member in members {
        let player_id: String = member
            .try_get("player_id")
            .map_err(|error| error.to_string())?;
        let member_role: String = member
            .try_get("member_role")
            .map_err(|error| error.to_string())?;
        let account_id: Uuid = member
            .try_get("account_id")
            .map_err(|error| error.to_string())?;
        let campaign_id: Option<String> = member
            .try_get("campaign_id")
            .map_err(|error| error.to_string())?;
        let campaign_id = campaign_id
            .ok_or_else(|| "terminal online member has no cloud campaign".to_string())?;
        let seed_value: Option<Value> = member
            .try_get("settlement_seed_json")
            .map_err(|error| error.to_string())?;
        let settlement_seed: BattleSeedV1 = serde_json::from_value(
            seed_value
                .ok_or_else(|| "terminal online member has no settlement seed".to_string())?,
        )
        .map_err(|error| error.to_string())?;
        let id_map_value: Value = member
            .try_get("unit_id_map")
            .map_err(|error| error.to_string())?;
        let id_map: BTreeMap<String, String> =
            serde_json::from_value(id_map_value).map_err(|error| error.to_string())?;
        let campaign_value: Value = sqlx::query_scalar::query_scalar(
            "select campaign_json from trnm_online_campaigns where campaign_id = $1 for update",
        )
        .bind(&campaign_id)
        .fetch_one(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
        let mut campaign: CampaignSaveV1 =
            serde_json::from_value(campaign_value).map_err(|error| error.to_string())?;
        if match_mode == "ranked_pvp" {
            sqlx::query::query(
                "insert into trnm_online_progression_events (
                    event_id, match_id, player_id, account_id, campaign_id, result_hash,
                    experience_delta, reputation_delta, inventory_delta, campaign_revision
                 ) values ($1, $2, $3, $4, $5, $6, 0, 0, '[]'::jsonb, $7)",
            )
            .bind(format!("online-progression:{match_id}:{player_id}"))
            .bind(match_id)
            .bind(&player_id)
            .bind(account_id)
            .bind(&campaign_id)
            .bind(combined_result_hash)
            .bind(campaign.revision as i64)
            .execute(&mut **transaction)
            .await
            .map_err(|error| error.to_string())?;
            continue;
        }
        let experience_before = campaign.progression.experience;
        let reputation_before = campaign.character.attributes.reputation;
        let member_outcome = if match_mode == "ranked_pvp" && member_role == "coop_guest" {
            match combined_result.outcome {
                BattleOutcome::Victory => BattleOutcome::Defeat,
                BattleOutcome::Defeat => BattleOutcome::Victory,
                BattleOutcome::Withdrawal => BattleOutcome::Defeat,
            }
        } else {
            combined_result.outcome
        };
        let mut unit_reports = Vec::with_capacity(settlement_seed.party.len());
        for seeded_unit in &settlement_seed.party {
            let combined_id = id_map
                .iter()
                .find_map(|(combined, local)| (local == &seeded_unit.unit_id).then_some(combined));
            let report = combined_id
                .and_then(|combined| {
                    combined_result
                        .units
                        .iter()
                        .find(|report| &report.unit_id == combined)
                })
                .map(|report| {
                    let mut local = report.clone();
                    local.unit_id = seeded_unit.unit_id.clone();
                    local
                })
                .unwrap_or_else(|| UnitBattleReportV1 {
                    unit_id: seeded_unit.unit_id.clone(),
                    status: UnitBattleStatus::Healthy,
                    remaining_hp: seeded_unit.stats.max_hp,
                    experience_gained: 0,
                    veteran_rank: seeded_unit.veteran_rank,
                    confirmed_kills: 0,
                });
            let mut report = report;
            if match_mode == "ranked_pvp" {
                report.experience_gained = if member_outcome == BattleOutcome::Victory {
                    25
                } else {
                    5
                };
            }
            unit_reports.push(report);
        }
        let member_result = BattleResultV1 {
            contract_version: combined_result.contract_version.clone(),
            battle_id: settlement_seed.battle_id.clone(),
            seed_hash: settlement_seed.seed_hash.clone(),
            outcome: member_outcome,
            units: unit_reports,
            loot: if match_mode == "ranked_pvp" {
                Vec::new()
            } else {
                combined_result.loot.clone()
            },
            resource_delta: if match_mode == "ranked_pvp" {
                0
            } else {
                combined_result.resource_delta
            },
            reputation_delta: if match_mode == "ranked_pvp" {
                i32::from(member_outcome == BattleOutcome::Victory)
            } else {
                combined_result.reputation_delta
            },
            world_flags: if match_mode == "ranked_pvp" {
                vec![format!(
                    "ranked_pvp_{}",
                    if member_outcome == BattleOutcome::Victory {
                        "won"
                    } else {
                        "lost"
                    }
                )]
            } else {
                combined_result.world_flags.clone()
            },
            elapsed_ticks: combined_result.elapsed_ticks,
            final_snapshot_hash: combined_result.final_snapshot_hash.clone(),
        };
        campaign
            .submit_battle_result(member_result)
            .map_err(|error| error.to_string())?;
        for intent in campaign
            .pending_economic_intents
            .iter_mut()
            .chain(campaign.pending_economic_compensations.iter_mut())
        {
            if matches!(
                intent.kind,
                trnm_economy_protocol::EconomicIntentKind::ReleaseReward
            ) && intent.amount_credits.unwrap_or_default() > 0
            {
                intent.metadata["online_match_id"] = json!(match_id.to_string());
                intent.metadata["online_rules_version"] =
                    json!(trnm_campaign_core::FIRST_CONTACT_RULES_VERSION);
                intent.metadata["online_build_id"] = json!(ONLINE_AUTHORITY_BUILD);
                intent.metadata["online_result_hash"] = json!(combined_result_hash);
                intent.metadata["online_participants_hash"] = json!(participants_hash);
            }
        }
        let experience_delta = campaign
            .progression
            .experience
            .saturating_sub(experience_before);
        let reputation_delta = campaign
            .character
            .attributes
            .reputation
            .saturating_sub(reputation_before);
        persist_campaign_string(transaction, &campaign)
            .await
            .map_err(|error| error.1 .0.error.clone())?;
        sqlx::query::query(
            "insert into trnm_online_progression_events (
                event_id, match_id, player_id, account_id, campaign_id, result_hash,
                experience_delta, reputation_delta, inventory_delta, campaign_revision
             ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(format!("online-progression:{match_id}:{player_id}"))
        .bind(match_id)
        .bind(&player_id)
        .bind(account_id)
        .bind(&campaign_id)
        .bind(combined_result_hash)
        .bind(experience_delta as i64)
        .bind(reputation_delta)
        .bind(serde_json::to_value(&combined_result.loot).map_err(|error| error.to_string())?)
        .bind(campaign.revision as i64)
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
        any_pending |= !campaign.pending_economic_intents.is_empty()
            || !campaign.pending_economic_compensations.is_empty();
    }
    Ok(any_pending)
}

pub fn production_authority_tick_interval() -> Duration {
    assert_eq!(1_000 % TICKS_PER_SECOND, 0);
    Duration::from_millis(1_000 / TICKS_PER_SECOND)
}

pub fn resolve_authority_tick_interval(
    requested_ms: Option<u64>,
    allow_accelerated_test_clock: bool,
) -> Result<Duration, String> {
    let production = production_authority_tick_interval();
    let requested = Duration::from_millis(
        requested_ms.unwrap_or_else(|| u64::try_from(production.as_millis()).unwrap_or(100)),
    );
    if requested.is_zero() || requested > Duration::from_secs(1) {
        return Err("TRNM_GAME_SERVER_TICK_MS must be between 1 and 1000".to_string());
    }
    if requested != production && !allow_accelerated_test_clock {
        return Err(format!(
            "production authority is fixed at {}ms ({}Hz); non-real-time clocks require TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1",
            production.as_millis(),
            TICKS_PER_SECOND
        ));
    }
    Ok(requested)
}

pub async fn run_authority_loop(state: AppState, tick_interval: Duration) {
    if let Err(error) = advance_running_matches(&state, i64::from(state.capacity)).await {
        tracing::error!(%error, "initial online match actor recovery failed closed");
    }

    let heartbeat_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = operations_v1::heartbeat_fleet(&heartbeat_state).await {
                tracing::error!(%error, "online fleet heartbeat failed closed");
            }
        }
    });

    let maintenance_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = production_v1::run_production_maintenance(&maintenance_state).await
            {
                tracing::error!(%error, "online production maintenance failed closed");
            }
        }
    });

    let settlement_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = settle_pending_matches(&settlement_state, 2).await {
                tracing::error!(%error, "online authority settlement remains pending");
            }
        }
    });

    let reconciliation_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = advance_running_matches(
                &reconciliation_state,
                i64::from(reconciliation_state.capacity),
            )
            .await
            {
                tracing::error!(%error, "online match actor reconciliation failed closed");
            }
        }
    });

    // This clock does no database or simulation work. Match actors own their
    // independent 10 Hz loops, while heartbeat, maintenance, settlement and
    // discovery run in separate tasks and cannot stall the realtime cadence.
    let started_at = Instant::now();
    if let Ok(mut clock_start) = state.authority_clock.started_at.lock() {
        *clock_start = Some(started_at);
    }
    state.authority_clock.wake_count.store(0, Ordering::Relaxed);
    let mut interval = tokio::time::interval_at(started_at + tick_interval, tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        state
            .authority_clock
            .wake_count
            .fetch_add(1, Ordering::Relaxed);
    }
}

pub async fn advance_running_matches(state: &AppState, limit: i64) -> Result<u64, String> {
    if *state.shutdown.borrow() {
        return Ok(0);
    }
    let ids = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select m.match_id from trnm_online_matches m
         left join trnm_online_fleet_instances f
           on f.instance_id = m.assigned_instance_id
          and f.instance_epoch = m.assigned_instance_epoch
         where m.phase = 'running' and (
            (m.assigned_instance_id = $2 and m.assigned_instance_epoch = $3)
            or m.assigned_instance_id is null
            or f.status is null or f.status = 'offline'
            or f.lease_expires_at <= now()
         ) order by m.updated_at limit $1",
    )
    .bind(limit)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .fetch_all(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    let mut active = 0u64;
    for match_id in ids {
        if ensure_match_actor(state, match_id).await?.is_some() {
            active = active.saturating_add(1);
        }
    }
    Ok(active)
}

async fn ensure_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<MatchActorHandle>, String> {
    if *state.shutdown.borrow() {
        return Ok(None);
    }
    if let Some(handle) = state.match_actors.read().await.get(&match_id).cloned() {
        return Ok(Some(handle));
    }
    let mut actors = state.match_actors.write().await;
    if let Some(handle) = actors.get(&match_id).cloned() {
        return Ok(Some(handle));
    }
    if actors.len() >= usize::try_from(state.capacity).unwrap_or(usize::MAX) {
        return Err(format!(
            "local match actor capacity {} is exhausted",
            state.capacity
        ));
    }
    let Some(loaded) = load_match_actor(state, match_id).await? else {
        return Ok(None);
    };
    let actor_id = Uuid::new_v4();
    let initial_hash = loaded
        .simulation
        .snapshot_hash()
        .map_err(|error| error.to_string())?;
    let initial = PublishedMatchState {
        simulation: Arc::new(loaded.simulation.clone()),
        snapshot_hash: Arc::new(initial_hash),
        next_sequence: loaded.next_sequence,
        match_revision: loaded.match_revision,
    };
    let (published_tx, published_rx) = watch::channel(initial);
    let (command_tx, command_rx) = mpsc::channel(MATCH_ACTOR_COMMAND_QUEUE);
    let handle = MatchActorHandle {
        actor_id,
        commands: command_tx,
        published: published_rx,
    };
    actors.insert(match_id, handle.clone());
    drop(actors);
    let actor_state = state.clone();
    tokio::spawn(async move {
        run_match_actor(
            actor_state,
            match_id,
            actor_id,
            loaded,
            command_rx,
            published_tx,
        )
        .await;
    });
    Ok(Some(handle))
}

async fn load_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<LoadedMatchActor>, String> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    lock_current_fleet_epoch(&mut transaction, state, true).await?;
    let Some(row) = sqlx::query::query(
        "select campaign_id, phase, simulation_json, match_mode,
                    next_sequence, match_revision, checkpoint_sequence,
                    assigned_instance_id, assigned_region, assigned_instance_epoch,
                    assigned_physical_host_id
             from trnm_online_matches
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    if row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?
        != "running"
    {
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(None);
    }
    let previous_instance: Option<String> = row
        .try_get("assigned_instance_id")
        .map_err(|error| error.to_string())?;
    let previous_region: Option<String> = row
        .try_get("assigned_region")
        .map_err(|error| error.to_string())?;
    let previous_epoch: i64 = row
        .try_get("assigned_instance_epoch")
        .map_err(|error| error.to_string())?;
    let previous_physical_host: Option<String> = row
        .try_get("assigned_physical_host_id")
        .map_err(|error| error.to_string())?;
    if previous_instance.as_deref() != Some(state.instance_id.as_str())
        || previous_epoch != state.instance_epoch
    {
        let previous_healthy: bool = if let Some(previous) = previous_instance.as_deref() {
            sqlx::query_scalar::query_scalar(
                "select exists(select 1 from trnm_online_fleet_instances
                 where instance_id = $1 and instance_epoch = $2
                   and status in ('active', 'draining')
                   and lease_expires_at > now())",
            )
            .bind(previous)
            .bind(previous_epoch)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?
        } else {
            false
        };
        if previous_healthy {
            transaction
                .commit()
                .await
                .map_err(|error| error.to_string())?;
            return Ok(None);
        }
        sqlx::query::query(
            "update trnm_online_matches set assigned_instance_id = $2,
                assigned_region = $3, assigned_instance_epoch = $4,
                assigned_physical_host_id = $5, updated_at = now() where match_id = $1",
        )
        .bind(match_id)
        .bind(state.instance_id.as_str())
        .bind(state.region.as_str())
        .bind(state.instance_epoch)
        .bind(state.physical_host_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        sqlx::query::query(
            "insert into trnm_online_fleet_failovers (
                failover_id, match_id, previous_instance_id, new_instance_id,
                previous_region, new_region, reason,
                previous_instance_epoch, new_instance_epoch,
                previous_physical_host_id, new_physical_host_id
             ) values ($1, $2, $3, $4, $5, $6, 'owner lease expired or epoch fenced', $7, $8, $9, $10)
             on conflict do nothing",
        )
        .bind(Uuid::new_v4())
        .bind(match_id)
        .bind(&previous_instance)
        .bind(state.instance_id.as_str())
        .bind(&previous_region)
        .bind(state.region.as_str())
        .bind(previous_epoch)
        .bind(state.instance_epoch)
        .bind(&previous_physical_host)
        .bind(state.physical_host_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    let checkpoint_sequence = row
        .try_get::<i64, _>("checkpoint_sequence")
        .map_err(|error| error.to_string())? as u64;
    let next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(|error| error.to_string())? as u64;
    let match_revision = row
        .try_get::<i64, _>("match_revision")
        .map_err(|error| error.to_string())? as u64;
    let value: Value = row
        .try_get("simulation_json")
        .map_err(|error| error.to_string())?;
    let mut simulation: MissionSimV1 =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    if checkpoint_sequence < next_sequence {
        let recovered: Option<Value> = sqlx::query_scalar::query_scalar(
            "select post_simulation_json from trnm_online_commands
             where match_id = $1 and sequence >= $2
               and post_simulation_json is not null
             order by sequence desc limit 1",
        )
        .bind(match_id)
        .bind(checkpoint_sequence as i64)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        let recovered = recovered.ok_or_else(|| {
            "command recovery event is missing its post-command simulation".to_string()
        })?;
        simulation = serde_json::from_value(recovered).map_err(|error| error.to_string())?;
    }
    let match_mode = row
        .try_get("match_mode")
        .map_err(|error| error.to_string())?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    Ok(Some(LoadedMatchActor {
        simulation,
        match_mode,
        next_sequence,
        match_revision,
    }))
}

async fn run_match_actor(
    state: AppState,
    match_id: Uuid,
    actor_id: Uuid,
    mut loaded: LoadedMatchActor,
    mut commands: mpsc::Receiver<ActorCommandEnvelope>,
    published: watch::Sender<PublishedMatchState>,
) {
    let (checkpoint_tx, checkpoint_rx) = mpsc::channel(MATCH_CHECKPOINT_QUEUE);
    let (checkpoint_failed_tx, mut checkpoint_failed_rx) = watch::channel(None::<String>);
    let checkpoint_state = state.clone();
    tokio::spawn(async move {
        run_match_checkpoint_writer(
            checkpoint_state,
            match_id,
            checkpoint_rx,
            checkpoint_failed_tx,
        )
        .await;
    });
    let (fenced_tx, mut fenced_rx) = watch::channel(false);
    let fence_state = state.clone();
    tokio::spawn(async move {
        run_match_fence_monitor(fence_state, match_id, fenced_tx).await;
    });
    let started_at = Instant::now();
    let mut shutdown = state.shutdown.subscribe();
    let mut interval =
        tokio::time::interval_at(started_at + state.tick_interval, state.tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            biased;
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    let snapshot_hash = match loaded.simulation.snapshot_hash() {
                        Ok(value) => value,
                        Err(error) => {
                            tracing::error!(%match_id, %error, "shutdown actor snapshot failed closed");
                            break;
                        }
                    };
                    let (completion_tx, completion_rx) = oneshot::channel();
                    let job = MatchCheckpointJob {
                        simulation: loaded.simulation.clone(),
                        snapshot_hash,
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: false,
                        completion: Some(completion_tx),
                    };
                    let result = if checkpoint_tx.send(job).await.is_ok() {
                        completion_rx.await.unwrap_or_else(|_| {
                            Err("shutdown checkpoint worker stopped before acknowledgement".to_string())
                        })
                    } else {
                        Err("shutdown checkpoint worker is unavailable".to_string())
                    };
                    if let Err(error) = result {
                        tracing::error!(%match_id, %error, "shutdown actor checkpoint failed closed");
                    }
                    break;
                }
            }
            changed = fenced_rx.changed() => {
                if changed.is_err() || *fenced_rx.borrow() {
                    tracing::warn!(%match_id, "match actor stopped after fleet epoch fencing");
                    break;
                }
            }
            changed = checkpoint_failed_rx.changed() => {
                if changed.is_err() {
                    tracing::error!(%match_id, "match actor checkpoint writer stopped unexpectedly");
                    break;
                }
                if let Some(error) = checkpoint_failed_rx.borrow().as_deref() {
                    tracing::error!(%match_id, %error, "match actor stopped after checkpoint failure");
                    break;
                }
            }
            _ = interval.tick() => {
                if !loaded.simulation.terminal() {
                    if let Err(error) = loaded.simulation.step() {
                        tracing::error!(%match_id, %error, "match actor simulation failed closed");
                        break;
                    }
                }
                let snapshot_hash = match loaded.simulation.snapshot_hash() {
                    Ok(value) => value,
                    Err(error) => {
                        tracing::error!(%match_id, %error, "match actor snapshot failed closed");
                        break;
                    }
                };
                published.send_replace(PublishedMatchState {
                    simulation: Arc::new(loaded.simulation.clone()),
                    snapshot_hash: Arc::new(snapshot_hash.clone()),
                    next_sequence: loaded.next_sequence,
                    match_revision: loaded.match_revision,
                });
                if loaded.simulation.terminal() {
                    let (completion_tx, completion_rx) = oneshot::channel();
                    let job = MatchCheckpointJob {
                        simulation: loaded.simulation.clone(),
                        snapshot_hash,
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: true,
                        completion: Some(completion_tx),
                    };
                    let result = if checkpoint_tx.send(job).await.is_ok() {
                        completion_rx.await.unwrap_or_else(|_| {
                            Err("terminal checkpoint worker stopped before acknowledgement".to_string())
                        })
                    } else {
                        Err("terminal checkpoint worker is unavailable".to_string())
                    };
                    if let Err(error) = result {
                        tracing::error!(%match_id, %error, "terminal actor checkpoint failed closed");
                    }
                    break;
                }
                if loaded.simulation.tick.is_multiple_of(MATCH_CHECKPOINT_INTERVAL_TICKS) {
                    let job = MatchCheckpointJob {
                        simulation: loaded.simulation.clone(),
                        snapshot_hash,
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: false,
                        completion: None,
                    };
                    if checkpoint_tx.try_send(job).is_err() {
                        tracing::warn!(%match_id, tick = loaded.simulation.tick, "checkpoint writer busy; latest durable checkpoint retained");
                    }
                }
            }
            envelope = commands.recv() => {
                let Some(envelope) = envelope else {
                    break;
                };
                let ActorCommandEnvelope {
                    request,
                    request_hash,
                    controlled_unit_ids,
                    member_role,
                    response,
                } = envelope;
                let result = accept_actor_command(
                    &state,
                    match_id,
                    &mut loaded,
                    request,
                    request_hash,
                    &controlled_unit_ids,
                    &member_role,
                )
                .await;
                if result.as_ref().is_ok_and(|receipt| !receipt.duplicate) {
                    if let Ok(snapshot_hash) = loaded.simulation.snapshot_hash() {
                        published.send_replace(PublishedMatchState {
                            simulation: Arc::new(loaded.simulation.clone()),
                            snapshot_hash: Arc::new(snapshot_hash),
                            next_sequence: loaded.next_sequence,
                            match_revision: loaded.match_revision,
                        });
                    }
                }
                let _ = response.send(result);
            }
        }
    }
    drop(checkpoint_tx);
    let mut actors = state.match_actors.write().await;
    if actors
        .get(&match_id)
        .is_some_and(|handle| handle.actor_id == actor_id)
    {
        actors.remove(&match_id);
    }
}

async fn run_match_fence_monitor(state: AppState, match_id: Uuid, fenced: watch::Sender<bool>) {
    let mut interval = tokio::time::interval(MATCH_ACTOR_FENCE_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = fenced.closed() => return,
            _ = interval.tick() => {
                let owns_match = sqlx::query_scalar::query_scalar::<_, bool>(
                    "select exists(
                        select 1 from trnm_online_matches m
                        join trnm_online_fleet_instances f
                          on f.instance_id = m.assigned_instance_id
                         and f.instance_epoch = m.assigned_instance_epoch
                        where m.match_id = $1 and m.phase = 'running'
                          and m.assigned_instance_id = $2
                          and m.assigned_instance_epoch = $3
                          and f.status in ('active', 'draining')
                          and f.lease_expires_at > now()
                    )",
                )
                .bind(match_id)
                .bind(state.instance_id.as_str())
                .bind(state.instance_epoch)
                .fetch_one(&state.pool)
                .await;
                match owns_match {
                    Ok(true) => {}
                    Ok(false) => {
                        fenced.send_replace(true);
                        return;
                    }
                    Err(error) => {
                        tracing::error!(%match_id, %error, "match actor fence check failed closed");
                        fenced.send_replace(true);
                        return;
                    }
                }
            }
        }
    }
}

async fn run_match_checkpoint_writer(
    state: AppState,
    match_id: Uuid,
    mut checkpoints: mpsc::Receiver<MatchCheckpointJob>,
    failed: watch::Sender<Option<String>>,
) {
    while let Some(mut job) = checkpoints.recv().await {
        let result = if job.terminal {
            persist_terminal_actor_checkpoint(&state, match_id, &job).await
        } else {
            persist_actor_checkpoint(&state, match_id, &job).await
        };
        if let Some(completion) = job.completion.take() {
            let _ = completion.send(result.clone());
        }
        if let Err(error) = result {
            tracing::error!(%match_id, %error, terminal = job.terminal, "match actor checkpoint failed");
            failed.send_replace(Some(error));
            break;
        }
        if job.terminal {
            break;
        }
    }
}

async fn persist_actor_checkpoint(
    state: &AppState,
    match_id: Uuid,
    job: &MatchCheckpointJob,
) -> Result<(), String> {
    let simulation_json =
        serde_json::to_value(&job.simulation).map_err(|error| error.to_string())?;
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    lock_current_fleet_epoch(&mut transaction, state, true).await?;
    sqlx::query::query(
        "insert into trnm_online_replay_frames (
            match_id, tick, snapshot_hash, simulation_json, frame_kind
         ) values ($1, $2, $3, $4, 'checkpoint')
         on conflict (match_id, tick) do update set
            snapshot_hash = excluded.snapshot_hash,
            simulation_json = excluded.simulation_json,
            frame_kind = excluded.frame_kind",
    )
    .bind(match_id)
    .bind(job.simulation.tick as i64)
    .bind(&job.snapshot_hash)
    .bind(&simulation_json)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let updated = sqlx::query::query(
        "update trnm_online_matches set simulation_json = $2, snapshot_hash = $3,
            authoritative_tick = $4, checkpoint_sequence = $5, updated_at = now()
         where match_id = $1 and phase = 'running'
           and assigned_instance_id = $6 and assigned_instance_epoch = $7
           and checkpoint_sequence <= $5 and next_sequence >= $5
           and match_revision >= $8
           and exists (
             select 1 from trnm_online_fleet_instances f
             where f.instance_id = $6 and f.instance_epoch = $7
               and f.status in ('active', 'draining')
               and f.lease_expires_at > now()
           )",
    )
    .bind(match_id)
    .bind(simulation_json)
    .bind(&job.snapshot_hash)
    .bind(job.simulation.tick as i64)
    .bind(job.next_sequence as i64)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(job.match_revision as i64)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if updated.rows_affected() != 1 {
        return Err("checkpoint was fenced or superseded".to_string());
    }
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())
}

async fn persist_terminal_actor_checkpoint(
    state: &AppState,
    match_id: Uuid,
    job: &MatchCheckpointJob,
) -> Result<(), String> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    lock_current_fleet_epoch(&mut transaction, state, true).await?;
    let row = sqlx::query::query(
        "select phase, match_mode, next_sequence, match_revision,
                assigned_instance_id, assigned_instance_epoch
         from trnm_online_matches where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "terminal match disappeared".to_string())?;
    let phase: String = row.try_get("phase").map_err(|error| error.to_string())?;
    if phase == "complete" {
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(());
    }
    let assigned_instance: Option<String> = row
        .try_get("assigned_instance_id")
        .map_err(|error| error.to_string())?;
    let assigned_epoch: i64 = row
        .try_get("assigned_instance_epoch")
        .map_err(|error| error.to_string())?;
    let durable_next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(|error| error.to_string())? as u64;
    let durable_revision = row
        .try_get::<i64, _>("match_revision")
        .map_err(|error| error.to_string())? as u64;
    if phase != "running"
        || assigned_instance.as_deref() != Some(state.instance_id.as_str())
        || assigned_epoch != state.instance_epoch
        || durable_next_sequence != job.next_sequence
        || durable_revision != job.match_revision
    {
        return Err("terminal checkpoint was fenced by newer authority".to_string());
    }
    let match_mode: String = row
        .try_get("match_mode")
        .map_err(|error| error.to_string())?;
    let simulation_json =
        serde_json::to_value(&job.simulation).map_err(|error| error.to_string())?;
    sqlx::query::query(
        "insert into trnm_online_replay_frames (
            match_id, tick, snapshot_hash, simulation_json, frame_kind
         ) values ($1, $2, $3, $4, 'terminal')
         on conflict (match_id, tick) do update set
            snapshot_hash = excluded.snapshot_hash,
            simulation_json = excluded.simulation_json,
            frame_kind = excluded.frame_kind",
    )
    .bind(match_id)
    .bind(job.simulation.tick as i64)
    .bind(&job.snapshot_hash)
    .bind(&simulation_json)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut result = job
        .simulation
        .clone()
        .into_result()
        .map_err(|error| error.to_string())?;
    if result.outcome == BattleOutcome::Victory && match_mode != "ranked_pvp" {
        result.resource_delta = result.resource_delta.max(25);
    }
    let result_hash = result.computed_hash().map_err(|error| error.to_string())?;
    let any_pending =
        apply_member_progression(&mut transaction, match_id, &result, &result_hash).await?;
    product_v2::apply_ranked_result(&mut transaction, match_id, result.outcome, &result_hash)
        .await?;
    operations_v1::finalize_ranked_operations(
        &mut transaction,
        match_id,
        result.outcome,
        &result_hash,
        &job.snapshot_hash,
    )
    .await?;
    let settlement_state = if any_pending { "pending" } else { "settled" };
    let updated = sqlx::query::query(
        "update trnm_online_matches set phase = 'complete', simulation_json = $2,
            result_json = $3, result_hash = $4, snapshot_hash = $5,
            authoritative_tick = $6, checkpoint_sequence = $7,
            settlement_state = $8, updated_at = now()
         where match_id = $1 and assigned_instance_id = $9
           and assigned_instance_epoch = $10 and next_sequence = $7",
    )
    .bind(match_id)
    .bind(simulation_json)
    .bind(serde_json::to_value(result).map_err(|error| error.to_string())?)
    .bind(result_hash)
    .bind(&job.snapshot_hash)
    .bind(job.simulation.tick as i64)
    .bind(job.next_sequence as i64)
    .bind(settlement_state)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if updated.rows_affected() != 1 {
        return Err("match completion was fenced by a newer fleet epoch".to_string());
    }
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())
}

pub async fn settle_pending_matches(state: &AppState, limit: i64) -> Result<u64, String> {
    let ids = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_id from trnm_online_matches where settlement_state = 'pending'
         order by updated_at limit $1",
    )
    .bind(limit)
    .fetch_all(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    let mut settled = 0u64;
    for match_id in ids {
        let mut transaction = state
            .pool
            .begin()
            .await
            .map_err(|error| error.to_string())?;
        let Some(row) = sqlx::query::query(
            "select settlement_state from trnm_online_matches
             where match_id = $1 for update skip locked",
        )
        .bind(match_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?
        else {
            continue;
        };
        if row
            .try_get::<String, _>("settlement_state")
            .map_err(|error| error.to_string())?
            != "pending"
        {
            continue;
        }
        let campaign_ids = sqlx::query_scalar::query_scalar::<_, Option<String>>(
            "select campaign_id from trnm_online_match_members where match_id = $1
             order by member_role",
        )
        .bind(match_id)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        let mut all_settled = campaign_ids.len() == 2;
        for campaign_id in campaign_ids {
            let campaign_id = campaign_id
                .ok_or_else(|| "pending member settlement has no cloud campaign".to_string())?;
            let value: Value = sqlx::query_scalar::query_scalar(
                "select campaign_json from trnm_online_campaigns where campaign_id = $1 for update",
            )
            .bind(&campaign_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
            let mut campaign: CampaignSaveV1 =
                serde_json::from_value(value).map_err(|error| error.to_string())?;
            let report = campaign
                .reconcile_economy(&state.cex, 8)
                .map_err(|error| error.to_string())?;
            all_settled &= report.remaining == 0;
            persist_campaign_string(&mut transaction, &campaign)
                .await
                .map_err(|error| error.1 .0.error.clone())?;
        }
        let settlement_state = if all_settled {
            settled = settled.saturating_add(1);
            "settled"
        } else {
            "pending"
        };
        sqlx::query::query(
            "update trnm_online_matches set settlement_state = $2, updated_at = now()
             where match_id = $1",
        )
        .bind(match_id)
        .bind(settlement_state)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
    }
    Ok(settled)
}

async fn persist_campaign(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    campaign: &CampaignSaveV1,
) -> Result<(), ApiError> {
    persist_campaign_string(transaction, campaign).await
}

async fn persist_campaign_string(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    campaign: &CampaignSaveV1,
) -> Result<(), ApiError> {
    let state_hash = hash_json(campaign)?;
    sqlx::query::query(
        "update trnm_online_campaigns set campaign_revision = $2, schema_revision = $3,
            state_hash = $4, campaign_json = $5, updated_at = now()
         where campaign_id = $1",
    )
    .bind(&campaign.campaign_id)
    .bind(campaign.revision as i64)
    .bind(i32::from(campaign.schema_revision))
    .bind(state_hash)
    .bind(serde_json::to_value(campaign).map_err(internal_serialization)?)
    .execute(&mut **transaction)
    .await
    .map_err(internal_db)?;
    Ok(())
}

async fn ensure_campaign_owner(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    campaign_id: &str,
    player_id: &str,
    account_id: Uuid,
) -> Result<(), ApiError> {
    let owner = sqlx::query::query(
        "select player_id, account_id from trnm_online_campaigns where campaign_id = $1",
    )
    .bind(campaign_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "cloud campaign not found", false))?;
    if owner
        .try_get::<String, _>("player_id")
        .map_err(internal_db)?
        != player_id
        || owner
            .try_get::<Uuid, _>("account_id")
            .map_err(internal_db)?
            != account_id
    {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "cloud campaign does not belong to the authenticated player/account",
            false,
        ));
    }
    Ok(())
}

async fn lock_player_lobby_scope(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    sqlx::query::query("select pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(format!("trnm-online-lobby:{player_id}"))
        .execute(&mut **transaction)
        .await
        .map_err(internal_db)?;
    Ok(())
}

async fn ensure_player_has_no_active_lobby(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    let active: bool = sqlx::query_scalar::query_scalar(
        "select exists(
            select 1 from trnm_online_lobby_members m
            join trnm_online_lobbies l on l.lobby_id = m.lobby_id
            where m.player_id = $1 and l.status in ('open', 'queued')
         )",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(internal_db)?;
    if active {
        return Err(api_error(
            StatusCode::CONFLICT,
            "player already belongs to an active lobby",
            true,
        ));
    }
    Ok(())
}

async fn lock_lobby(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    lobby_id: Uuid,
) -> Result<sqlx_postgres::PgRow, ApiError> {
    sqlx::query::query(
        "select owner_player_id, owner_account_id, status, lobby_revision, map_id
         from trnm_online_lobbies where lobby_id = $1 for update",
    )
    .bind(lobby_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "lobby not found", false))
}

fn require_lobby_owner(
    lobby: &sqlx_postgres::PgRow,
    player_id: &str,
    account_id: Uuid,
) -> Result<(), ApiError> {
    if lobby
        .try_get::<String, _>("owner_player_id")
        .map_err(internal_db)?
        != player_id
        || lobby
            .try_get::<Uuid, _>("owner_account_id")
            .map_err(internal_db)?
            != account_id
    {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "only the authenticated lobby owner may perform this operation",
            false,
        ));
    }
    Ok(())
}

fn require_open_lobby_revision(
    lobby: &sqlx_postgres::PgRow,
    expected_revision: u64,
) -> Result<(), ApiError> {
    if lobby.try_get::<String, _>("status").map_err(internal_db)? != "open" {
        return Err(api_error(StatusCode::CONFLICT, "lobby is not open", false));
    }
    let revision = lobby
        .try_get::<i64, _>("lobby_revision")
        .map_err(internal_db)? as u64;
    if revision != expected_revision {
        return Err(conflict("lobby revision changed", revision));
    }
    Ok(())
}

async fn fetch_lobby_view(pool: &PgPool, lobby_id: Uuid) -> Result<OnlineLobbyView, ApiError> {
    let lobby = sqlx::query::query(
        "select display_name, owner_player_id, status, lobby_revision, map_id,
                queue_mode, match_id
         from trnm_online_lobbies where lobby_id = $1",
    )
    .bind(lobby_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "lobby not found", false))?;
    let rows = sqlx::query::query(
        "select player_id, account_id, campaign_id, member_role, ready
         from trnm_online_lobby_members where lobby_id = $1
         order by case member_role when 'owner' then 0 else 1 end",
    )
    .bind(lobby_id)
    .fetch_all(pool)
    .await
    .map_err(internal_db)?;
    let members = rows
        .into_iter()
        .map(|row| {
            Ok(OnlineLobbyMemberView {
                player_id: row.try_get("player_id").map_err(internal_db)?,
                account_id: row
                    .try_get::<Uuid, _>("account_id")
                    .map_err(internal_db)?
                    .to_string(),
                campaign_id: row.try_get("campaign_id").map_err(internal_db)?,
                role: row.try_get("member_role").map_err(internal_db)?,
                ready: row.try_get("ready").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let status: String = lobby.try_get("status").map_err(internal_db)?;
    Ok(OnlineLobbyView {
        protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
        build_id: ONLINE_PRODUCT_BUILD.to_string(),
        lobby_id: lobby_id.to_string(),
        display_name: lobby.try_get("display_name").map_err(internal_db)?,
        owner_player_id: lobby.try_get("owner_player_id").map_err(internal_db)?,
        status: match status.as_str() {
            "open" => OnlineLobbyStatus::Open,
            "queued" => OnlineLobbyStatus::Queued,
            "matched" => OnlineLobbyStatus::Matched,
            _ => OnlineLobbyStatus::Closed,
        },
        lobby_revision: lobby
            .try_get::<i64, _>("lobby_revision")
            .map_err(internal_db)? as u64,
        map_id: lobby.try_get("map_id").map_err(internal_db)?,
        queue_mode: lobby.try_get("queue_mode").map_err(internal_db)?,
        members,
        match_id: lobby
            .try_get::<Option<Uuid>, _>("match_id")
            .map_err(internal_db)?
            .map(|value| value.to_string()),
    })
}

fn sha256_text(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

async fn fetch_match_view(pool: &PgPool, match_id: Uuid) -> Result<OnlineMatchView, ApiError> {
    let row = sqlx::query::query(
        "select match_id, join_code, phase, build_id, map_id, match_mode, rules_version, seed_hash,
                snapshot_hash, authoritative_tick, next_sequence, match_revision,
                result_hash, settlement_state
         from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let member_rows = sqlx::query::query(
        "select m.player_id, m.account_id, m.campaign_id, m.member_role,
                m.controlled_unit_ids, c.campaign_revision, c.campaign_json
         from trnm_online_match_members m
         join trnm_online_campaigns c on c.campaign_id = m.campaign_id
         where m.match_id = $1 order by m.member_role desc",
    )
    .bind(match_id)
    .fetch_all(pool)
    .await
    .map_err(internal_db)?;
    let mut members = Vec::with_capacity(member_rows.len());
    for member in member_rows {
        let controlled: Value = member.try_get("controlled_unit_ids").map_err(internal_db)?;
        let campaign_value: Value = member.try_get("campaign_json").map_err(internal_db)?;
        let campaign: CampaignSaveV1 =
            serde_json::from_value(campaign_value).map_err(internal_serialization)?;
        members.push(OnlineMatchMemberView {
            player_id: member.try_get("player_id").map_err(internal_db)?,
            account_id: member
                .try_get::<Uuid, _>("account_id")
                .map_err(internal_db)?
                .to_string(),
            campaign_id: member.try_get("campaign_id").map_err(internal_db)?,
            role: member.try_get("member_role").map_err(internal_db)?,
            controlled_unit_ids: serde_json::from_value(controlled)
                .map_err(internal_serialization)?,
            campaign_revision: member
                .try_get::<i64, _>("campaign_revision")
                .map_err(internal_db)? as u64,
            level: campaign.progression.level,
            experience: campaign.progression.experience,
            inventory_count: campaign
                .progression
                .inventory
                .iter()
                .map(|stack| u64::from(stack.quantity))
                .sum(),
        });
    }
    let phase: String = row.try_get("phase").map_err(internal_db)?;
    Ok(OnlineMatchView {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        build_id: row.try_get("build_id").map_err(internal_db)?,
        match_id: match_id.to_string(),
        join_code: row.try_get("join_code").map_err(internal_db)?,
        phase: match phase.as_str() {
            "waiting" => OnlineMatchPhase::Waiting,
            "running" => OnlineMatchPhase::Running,
            "complete" => OnlineMatchPhase::Complete,
            _ => OnlineMatchPhase::FailedClosed,
        },
        match_revision: row
            .try_get::<i64, _>("match_revision")
            .map_err(internal_db)? as u64,
        authoritative_tick: row
            .try_get::<i64, _>("authoritative_tick")
            .map_err(internal_db)? as u64,
        next_sequence: row
            .try_get::<i64, _>("next_sequence")
            .map_err(internal_db)? as u64,
        map_id: row.try_get("map_id").map_err(internal_db)?,
        match_mode: row.try_get("match_mode").map_err(internal_db)?,
        rules_version: row.try_get("rules_version").map_err(internal_db)?,
        seed_hash: row.try_get("seed_hash").map_err(internal_db)?,
        snapshot_hash: row.try_get("snapshot_hash").map_err(internal_db)?,
        members,
        result_hash: row.try_get("result_hash").map_err(internal_db)?,
        settlement_state: row.try_get("settlement_state").map_err(internal_db)?,
    })
}

fn campaign_view_from_row(row: &sqlx_postgres::PgRow) -> Result<OnlineCampaignView, ApiError> {
    let campaign_value: Value = row.try_get("campaign_json").map_err(internal_db)?;
    let campaign: CampaignSaveV1 =
        serde_json::from_value(campaign_value).map_err(internal_serialization)?;
    Ok(OnlineCampaignView {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        campaign_id: row.try_get("campaign_id").map_err(internal_db)?,
        player_id: row.try_get("player_id").map_err(internal_db)?,
        account_id: row
            .try_get::<Uuid, _>("account_id")
            .map_err(internal_db)?
            .to_string(),
        slot_key: row.try_get("slot_key").map_err(internal_db)?,
        campaign_revision: row
            .try_get::<i64, _>("campaign_revision")
            .map_err(internal_db)? as u64,
        schema_revision: row
            .try_get::<i32, _>("schema_revision")
            .map_err(internal_db)? as u16,
        state_hash: row.try_get("state_hash").map_err(internal_db)?,
        level: campaign.progression.level,
        experience: campaign.progression.experience,
        reputation: campaign.character.attributes.reputation,
        inventory: campaign
            .progression
            .inventory
            .iter()
            .map(|stack| OnlineInventoryStack {
                item_id: stack.item_id.clone(),
                quantity: stack.quantity,
            })
            .collect(),
        settled_match_count: campaign.settled_battle_ids.len(),
    })
}

fn internal_serialization(error: serde_json::Error) -> ApiError {
    api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false)
}

fn internal_db(error: sqlx::Error) -> ApiError {
    api_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Online Authority persistence failed: {error}"),
        false,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_map_allowlist_is_explicit() {
        assert_eq!(
            mission_for_map("iron_delta").unwrap().map_id(),
            "iron_delta"
        );
        assert_eq!(
            mission_for_map("first_contact").unwrap().map_id(),
            "first_contact"
        );
        assert!(mission_for_map("../../secret").is_err());
    }

    #[test]
    fn authority_materializes_base_and_overlay_maps() {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        for map_id in [
            "first_contact",
            "iron_delta",
            "night_watch_crossing",
            "glass_basin",
            "ember_orchard",
            "salt_marsh",
            "cinder_crown",
        ] {
            let map = map::load_authoritative_map(&assets, map_id)
                .unwrap_or_else(|error| panic!("{map_id}: {error}"));
            assert_eq!((map.width, map.height), (40, 24));
            assert!(!map.enemy_spawns.is_empty());
        }
    }

    #[test]
    fn slot_keys_are_bounded_and_portable() {
        assert!(validate_slot_key("main_01").is_ok());
        assert!(validate_slot_key("").is_err());
        assert!(validate_slot_key("bad/slot").is_err());
    }

    #[test]
    fn command_ids_are_bounded_and_portable() {
        assert!(validate_command_id("native:match.player_01-command").is_ok());
        assert!(validate_command_id("").is_err());
        assert!(validate_command_id("bad/command").is_err());
        assert!(validate_command_id(&"x".repeat(161)).is_err());
    }

    #[test]
    fn campaign_hash_changes_with_authoritative_revision() {
        let first = CampaignSaveV1::default();
        let mut second = first.clone();
        second.revision += 1;
        assert_ne!(hash_json(&first).unwrap(), hash_json(&second).unwrap());
    }

    #[test]
    fn online_campaign_starts_from_cex_connected_authority() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("player-a", "00000000-0000-0000-0000-000000000001")
            .unwrap();
        assert_eq!(
            campaign.economy_mode,
            trnm_campaign_core::EconomyMode::CexConnected
        );
        campaign.prepare_standalone_skirmish().unwrap();
        assert_eq!(
            campaign.room,
            trnm_campaign_core::CampaignRoom::ExpeditionGate
        );
    }

    #[test]
    fn public_bind_fails_closed_without_production_security_boundary() {
        assert!(validate_operations_bind_addr("127.0.0.1:7005".parse().unwrap()).is_ok());
        assert!(validate_operations_bind_addr("[::1]:7005".parse().unwrap()).is_ok());
        assert!(validate_operations_bind_addr("0.0.0.0:7005".parse().unwrap()).is_err());
        assert!(validate_operations_bind_addr("192.0.2.10:7005".parse().unwrap()).is_err());
    }

    #[test]
    fn production_authority_clock_is_exactly_ten_hz() {
        let interval = production_authority_tick_interval();
        assert_eq!(TICKS_PER_SECOND, 10);
        assert_eq!(interval, Duration::from_millis(100));
        assert_eq!(interval.saturating_mul(1_800), Duration::from_secs(180));
    }

    #[test]
    fn accelerated_clock_is_explicitly_test_only() {
        assert_eq!(
            resolve_authority_tick_interval(None, false).unwrap(),
            Duration::from_millis(100)
        );
        assert!(resolve_authority_tick_interval(Some(50), false).is_err());
        assert_eq!(
            resolve_authority_tick_interval(Some(20), true).unwrap(),
            Duration::from_millis(20)
        );
        assert!(resolve_authority_tick_interval(Some(0), true).is_err());
    }

    #[test]
    fn readiness_fails_closed_on_authority_clock_degradation() {
        assert!(authority_clock_is_operational(Some(0.0)));
        assert!(authority_clock_is_operational(Some(1.99)));
        assert!(authority_clock_is_operational(Some(-1.99)));
        assert!(!authority_clock_is_operational(Some(2.0)));
        assert!(!authority_clock_is_operational(Some(-2.0)));
        assert!(!authority_clock_is_operational(Some(f64::NAN)));
        assert!(!authority_clock_is_operational(None));
    }
}
