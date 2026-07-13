#![recursion_limit = "256"]

mod cex;
mod map;
mod operations_v1;
mod product_v2;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cex::CexClient;
use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::{
    collections::{BTreeMap, BTreeSet},
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use tokio::time::MissedTickBehavior;
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
use trnm_rts_sim::MissionSimV1;
use uuid::Uuid;

const PLAYER_SESSION_HEADER: &str = "x-trnm-player-session";
const MIGRATION_V1: &str = include_str!("../migrations/0001_online_authority_v1.sql");
const MIGRATION_V2: &str = include_str!("../migrations/0002_online_authority_v2.sql");
const MIGRATION_V3: &str = include_str!("../migrations/0003_online_product_v1.sql");
const MIGRATION_V4: &str = include_str!("../migrations/0004_online_product_v2.sql");
const MIGRATION_V5: &str = include_str!("../migrations/0005_online_operations_v1.sql");
const MIGRATION_V6: &str = include_str!("../migrations/0006_online_operations_v2.sql");

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    cex: CexClient,
    asset_root: Arc<PathBuf>,
    moderator_token: Arc<String>,
    instance_id: Arc<String>,
    region: Arc<String>,
    public_endpoint: Arc<String>,
    capacity: i32,
    instance_epoch: i64,
}

pub struct AppStateConfig {
    pub database_url: String,
    pub cex_base_url: String,
    pub game_authority_token: String,
    pub entitlement_signing_seed_base64: String,
    pub entitlement_key_id: String,
    pub asset_root: PathBuf,
    pub moderator_token: String,
    pub instance_id: String,
    pub region: String,
    pub public_endpoint: String,
    pub capacity: i32,
}

impl AppState {
    pub async fn connect(config: AppStateConfig) -> Result<Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&config.database_url)
            .await
            .map_err(|error| format!("connect Online Authority PostgreSQL: {error}"))?;
        sqlx::raw_sql(MIGRATION_V1)
            .execute(&pool)
            .await
            .map_err(|error| format!("migrate Online Authority PostgreSQL: {error}"))?;
        sqlx::raw_sql(MIGRATION_V2)
            .execute(&pool)
            .await
            .map_err(|error| format!("migrate Online Authority v2 PostgreSQL: {error}"))?;
        sqlx::raw_sql(MIGRATION_V3)
            .execute(&pool)
            .await
            .map_err(|error| format!("migrate Online Product v1 PostgreSQL: {error}"))?;
        sqlx::raw_sql(MIGRATION_V4)
            .execute(&pool)
            .await
            .map_err(|error| format!("migrate Online Product v2 PostgreSQL: {error}"))?;
        sqlx::raw_sql(MIGRATION_V5)
            .execute(&pool)
            .await
            .map_err(|error| format!("migrate Online Operations v1 PostgreSQL: {error}"))?;
        sqlx::raw_sql(MIGRATION_V6)
            .execute(&pool)
            .await
            .map_err(|error| format!("migrate Online Operations v2 PostgreSQL: {error}"))?;
        if config.instance_id.trim().is_empty()
            || config.region.trim().is_empty()
            || config.public_endpoint.trim().is_empty()
            || !(1..=10_000).contains(&config.capacity)
        {
            return Err("fleet instance, region, endpoint and capacity must be valid".to_string());
        }
        let instance_epoch: i64 = sqlx::query_scalar(
            "insert into trnm_online_fleet_instances (
                instance_id, region, public_endpoint, build_id, capacity, status,
                instance_epoch, lease_expires_at
             ) values ($1, $2, $3, $4, $5, 'active', 1, now() + interval '5 seconds')
             on conflict (instance_id) do update set region = excluded.region,
                public_endpoint = excluded.public_endpoint, build_id = excluded.build_id,
                capacity = excluded.capacity, status = 'active', heartbeat_at = now(),
                lease_expires_at = now() + interval '5 seconds',
                instance_epoch = trnm_online_fleet_instances.instance_epoch + 1,
                drain_reason = null
             returning instance_epoch",
        )
        .bind(config.instance_id.trim())
        .bind(config.region.trim())
        .bind(config.public_endpoint.trim())
        .bind(trnm_online_protocol::ONLINE_OPERATIONS_BUILD)
        .bind(config.capacity)
        .fetch_one(&pool)
        .await
        .map_err(|error| format!("register Online Operations fleet instance: {error}"))?;
        let cex = CexClient::new(
            config.cex_base_url,
            config.game_authority_token,
            config.entitlement_signing_seed_base64,
            config.entitlement_key_id,
        )?;
        cex.readiness().await?;
        Ok(Self {
            pool,
            cex,
            asset_root: Arc::new(config.asset_root),
            moderator_token: Arc::new(config.moderator_token),
            instance_id: Arc::new(config.instance_id),
            region: Arc::new(config.region),
            public_endpoint: Arc::new(config.public_endpoint),
            capacity: config.capacity,
            instance_epoch,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
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

pub fn build_router(state: AppState) -> Router {
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
        .with_state(state)
}

async fn health() -> &'static str {
    "trnm-game-server ok"
}

async fn readiness(State(state): State<AppState>) -> Response {
    let postgres = sqlx::query_scalar::<_, i32>("select 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    let cex = state.cex.readiness().await.is_ok();
    let ready = postgres && cex;
    let healthy_fleet_instances = sqlx::query_scalar::<_, i64>(
        "select count(*) from trnm_online_fleet_instances
         where status = 'active' and lease_expires_at > now()",
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or_default();
    (
        if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(json!({
            "status": if ready { "ok" } else { "blocked" },
            "protocol": ONLINE_AUTHORITY_PROTOCOL,
            "build_id": ONLINE_AUTHORITY_BUILD,
            "postgres_persistent": postgres,
            "cex_identity_and_settlement": cex,
            "server_authoritative_campaign": true,
            "server_authoritative_rts": true,
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
            "entitlement_key_custody": "local_mode_600_ed25519_seed_not_kms",
            "public_edge_ddos_attested": false,
        })),
    )
        .into_response()
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

    if let Some(row) = sqlx::query(
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
    sqlx::query(
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
    sqlx::query(
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
    sqlx::query(
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
    let member: bool = sqlx::query_scalar(
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
    let blocked: bool = sqlx::query_scalar(
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
    let member_count: i64 =
        sqlx::query_scalar("select count(*) from trnm_online_lobby_members where lobby_id = $1")
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
    sqlx::query(
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
    let invite = sqlx::query(
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
    let member_count: i64 =
        sqlx::query_scalar("select count(*) from trnm_online_lobby_members where lobby_id = $1")
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
    sqlx::query(
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
    sqlx::query(
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
    sqlx::query(
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
    let updated = sqlx::query(
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
    sqlx::query(
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
    let members = sqlx::query(
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
    sqlx::query(
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
        sqlx::query(
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
    sqlx::query(
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
            sqlx::query("delete from trnm_online_matches where match_id = $1")
                .bind(match_id)
                .execute(&mut *cleanup)
                .await
                .map_err(internal_db)?;
            sqlx::query(
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
    sqlx::query(
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
    sqlx::query(
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
    let campaign_row = sqlx::query(
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
    sqlx::query(
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
    sqlx::query(
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
    let campaign_owner = sqlx::query(
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
    let row = sqlx::query(
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
    sqlx::query(
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
    let match_row = sqlx::query(
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
    let member_count: i64 =
        sqlx::query_scalar("select count(*) from trnm_online_match_members where match_id = $1")
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
    let member_rows = sqlx::query(
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
        let campaign_value: Value = sqlx::query_scalar(
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
        sqlx::query(
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
    sqlx::query(
        "update trnm_online_matches set
            phase = 'running', seed_hash = $2, seed_json = $3,
            simulation_json = $4, snapshot_hash = $5,
            authoritative_tick = 0, match_revision = match_revision + 1,
            assigned_instance_id = $6, assigned_region = $7,
            assigned_instance_epoch = $8, initial_simulation_json = $4,
            season_id = $9, updated_at = now()
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
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query(
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
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let match_row = sqlx::query(
        "select phase, match_revision, authoritative_tick, next_sequence,
                simulation_json, snapshot_hash, match_mode
         from trnm_online_matches where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let member = sqlx::query(
        "select controlled_unit_ids, member_role from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3",
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
    if let Some(row) = sqlx::query(
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
        let stored_player_id: String = row.try_get("player_id").map_err(internal_db)?;
        let stored_request_hash: Option<String> =
            row.try_get("request_hash").map_err(internal_db)?;
        if stored_player_id != request.player_id
            || stored_request_hash.as_deref() != Some(request_hash.as_str())
        {
            return Err(conflict(
                "command_id was already used with a different authenticated request",
                match_row
                    .try_get::<i64, _>("match_revision")
                    .map_err(internal_db)? as u64,
            ));
        }
        transaction.commit().await.map_err(internal_db)?;
        return Ok(Json(OnlineCommandReceipt {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            match_id: match_id.to_string(),
            command_id: request.command_id,
            sequence: row.try_get::<i64, _>("sequence").map_err(internal_db)? as u64,
            duplicate: true,
            accepted_tick: row.try_get::<i64, _>("target_tick").map_err(internal_db)? as u64,
            match_revision: row
                .try_get::<i64, _>("accepted_match_revision")
                .map_err(internal_db)? as u64,
            snapshot_hash: row.try_get("accepted_snapshot_hash").map_err(internal_db)?,
        }));
    }
    let phase: String = match_row.try_get("phase").map_err(internal_db)?;
    if phase != "running" {
        return Err(api_error(
            StatusCode::CONFLICT,
            "match is not running",
            false,
        ));
    }
    let revision = match_row
        .try_get::<i64, _>("match_revision")
        .map_err(internal_db)? as u64;
    let next_sequence = match_row
        .try_get::<i64, _>("next_sequence")
        .map_err(internal_db)? as u64;
    let current_tick = match_row
        .try_get::<i64, _>("authoritative_tick")
        .map_err(internal_db)? as u64;
    if request.expected_match_revision != revision {
        return Err(conflict("match revision changed", revision));
    }
    if request.sequence != next_sequence {
        return Err(conflict(
            format!("expected command sequence {next_sequence}"),
            revision,
        ));
    }
    if request.target_tick < current_tick || request.target_tick > current_tick.saturating_add(200)
    {
        return Err(conflict(
            "target_tick is outside the authoritative window",
            revision,
        ));
    }
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
    let sim_value: Value = match_row.try_get("simulation_json").map_err(internal_db)?;
    let mut sim: MissionSimV1 =
        serde_json::from_value(sim_value).map_err(internal_serialization)?;
    let match_mode: String = match_row.try_get("match_mode").map_err(internal_db)?;
    let member_role: String = member.try_get("member_role").map_err(internal_db)?;
    let mut order = request.order;
    order.player_id = if match_mode == "ranked_pvp" && member_role == "coop_guest" {
        "enemy-player".to_string()
    } else {
        "player".to_string()
    };
    order.frame = u32::try_from(request.target_tick).map_err(|_| {
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
        && sim.active_order.as_ref().is_some_and(|active| {
            active.kind == order.kind
                && active.target_tile == order.target_tile
                && active.target_actor_id == order.target_actor_id
                && active.target_rule_id == order.target_rule_id
        });
    if merged_with_active_coop_order {
        let active = sim
            .active_order
            .as_mut()
            .expect("compatible active order was checked above");
        active
            .subject_actor_ids
            .extend(order.subject_actor_ids.clone());
        active.subject_actor_ids.sort();
        active.subject_actor_ids.dedup();
    } else if match_mode == "ranked_pvp" && member_role == "coop_guest" {
        sim.issue_human_enemy_order(order.clone())
            .map_err(|error| {
                api_error(StatusCode::UNPROCESSABLE_ENTITY, error.to_string(), false)
            })?;
    } else {
        sim.issue_order(order.clone()).map_err(|error| {
            api_error(StatusCode::UNPROCESSABLE_ENTITY, error.to_string(), false)
        })?;
    }
    let snapshot_hash = sim
        .snapshot_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    let accepted_revision = revision.saturating_add(1);
    sqlx::query(
        "insert into trnm_online_commands (
            match_id, sequence, command_id, player_id, request_hash, target_tick,
            order_json, accepted_snapshot_hash, accepted_match_revision
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
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
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query(
        "update trnm_online_matches set simulation_json = $2, snapshot_hash = $3,
            next_sequence = next_sequence + 1, match_revision = $4, updated_at = now()
         where match_id = $1",
    )
    .bind(match_id)
    .bind(serde_json::to_value(sim).map_err(internal_serialization)?)
    .bind(&snapshot_hash)
    .bind(accepted_revision as i64)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineCommandReceipt {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        match_id: match_id.to_string(),
        command_id: request.command_id,
        sequence: request.sequence,
        duplicate: false,
        accepted_tick: request.target_tick,
        match_revision: accepted_revision,
        snapshot_hash,
    }))
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
    let member: i64 = sqlx::query_scalar(
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
    let snapshot: Option<Value> =
        sqlx::query_scalar("select simulation_json from trnm_online_matches where match_id = $1")
            .bind(match_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(internal_db)?
            .flatten();
    Ok(Json(OnlineSnapshotResponse {
        view: fetch_match_view(&state.pool, match_id).await?,
        snapshot: snapshot.unwrap_or(Value::Null),
    }))
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
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let member = sqlx::query(
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
    let match_row = sqlx::query(
        "select simulation_json, snapshot_hash, next_sequence, match_revision
         from trnm_online_matches where match_id = $1 for share",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let next_sequence = match_row
        .try_get::<i64, _>("next_sequence")
        .map_err(internal_db)? as u64;
    let match_revision = match_row
        .try_get::<i64, _>("match_revision")
        .map_err(internal_db)? as u64;
    if request.last_acknowledged_sequence > next_sequence {
        return Err(conflict(
            "client acknowledged a command sequence beyond server authority",
            match_revision,
        ));
    }
    let snapshot_hash: String = match_row.try_get("snapshot_hash").map_err(internal_db)?;
    let command_rows = sqlx::query(
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
    sqlx::query(
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
    let snapshot: Option<Value> = match_row.try_get("simulation_json").map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineReconnectResponse {
        view: fetch_match_view(&state.pool, match_id).await?,
        snapshot: snapshot.unwrap_or(Value::Null),
        replayed_commands,
        reconnect_count,
        full_snapshot_required: request.last_snapshot_hash != snapshot_hash,
    }))
}

async fn apply_member_progression(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    match_id: Uuid,
    combined_result: &BattleResultV1,
    combined_result_hash: &str,
) -> Result<bool, String> {
    let match_mode: String =
        sqlx::query_scalar("select match_mode from trnm_online_matches where match_id = $1")
            .bind(match_id)
            .fetch_one(&mut **transaction)
            .await
            .map_err(|error| error.to_string())?;
    let members = sqlx::query(
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
        let campaign_value: Value = sqlx::query_scalar(
            "select campaign_json from trnm_online_campaigns where campaign_id = $1 for update",
        )
        .bind(&campaign_id)
        .fetch_one(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
        let mut campaign: CampaignSaveV1 =
            serde_json::from_value(campaign_value).map_err(|error| error.to_string())?;
        if match_mode == "ranked_pvp" {
            sqlx::query(
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
        sqlx::query(
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

pub async fn run_authority_loop(state: AppState, tick_interval: Duration) {
    let mut interval = tokio::time::interval(tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut heartbeat_elapsed = Duration::from_secs(1);
    loop {
        interval.tick().await;
        heartbeat_elapsed = heartbeat_elapsed.saturating_add(tick_interval);
        if heartbeat_elapsed >= Duration::from_secs(1) {
            if let Err(error) = operations_v1::heartbeat_fleet(&state).await {
                tracing::error!(%error, "online fleet heartbeat failed closed");
            }
            heartbeat_elapsed = Duration::ZERO;
        }
        if let Err(error) = advance_running_matches(&state, 4).await {
            tracing::error!(%error, "online authority tick failed closed");
        }
        if let Err(error) = settle_pending_matches(&state, 2).await {
            tracing::error!(%error, "online authority settlement remains pending");
        }
    }
}

pub async fn advance_running_matches(state: &AppState, limit: i64) -> Result<u64, String> {
    let ids = sqlx::query_scalar::<_, Uuid>(
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
    let mut advanced = 0u64;
    for match_id in ids {
        let mut transaction = state
            .pool
            .begin()
            .await
            .map_err(|error| error.to_string())?;
        let Some(row) = sqlx::query(
            "select campaign_id, phase, simulation_json, match_mode,
                    assigned_instance_id, assigned_region, assigned_instance_epoch
             from trnm_online_matches
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
            .try_get::<String, _>("phase")
            .map_err(|error| error.to_string())?
            != "running"
        {
            continue;
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
        if previous_instance.as_deref() != Some(state.instance_id.as_str())
            || previous_epoch != state.instance_epoch
        {
            let previous_healthy: bool = if let Some(previous) = previous_instance.as_deref() {
                sqlx::query_scalar(
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
                continue;
            }
            sqlx::query(
                "update trnm_online_matches set assigned_instance_id = $2,
                    assigned_region = $3, assigned_instance_epoch = $4,
                    updated_at = now() where match_id = $1",
            )
            .bind(match_id)
            .bind(state.instance_id.as_str())
            .bind(state.region.as_str())
            .bind(state.instance_epoch)
            .execute(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
            sqlx::query(
                "insert into trnm_online_fleet_failovers (
                    failover_id, match_id, previous_instance_id, new_instance_id,
                    previous_region, new_region, reason,
                    previous_instance_epoch, new_instance_epoch
                 ) values ($1, $2, $3, $4, $5, $6, 'owner lease expired or epoch fenced', $7, $8)
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
            .execute(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
        }
        let value: Value = row
            .try_get("simulation_json")
            .map_err(|error| error.to_string())?;
        let mut sim: MissionSimV1 =
            serde_json::from_value(value).map_err(|error| error.to_string())?;
        let match_mode: String = row
            .try_get("match_mode")
            .map_err(|error| error.to_string())?;
        for _ in 0..5 {
            if sim.terminal() {
                break;
            }
            sim.step().map_err(|error| error.to_string())?;
        }
        let snapshot_hash = sim.snapshot_hash().map_err(|error| error.to_string())?;
        let terminal = sim.terminal();
        if terminal || sim.tick.is_multiple_of(100) {
            sqlx::query(
                "insert into trnm_online_replay_frames (
                    match_id, tick, snapshot_hash, simulation_json, frame_kind
                 ) values ($1, $2, $3, $4, $5)
                 on conflict (match_id, tick) do update set
                    snapshot_hash = excluded.snapshot_hash,
                    simulation_json = excluded.simulation_json,
                    frame_kind = excluded.frame_kind",
            )
            .bind(match_id)
            .bind(sim.tick as i64)
            .bind(&snapshot_hash)
            .bind(serde_json::to_value(&sim).map_err(|error| error.to_string())?)
            .bind(if terminal { "terminal" } else { "checkpoint" })
            .execute(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
        }
        if terminal {
            let mut result = sim
                .clone()
                .into_result()
                .map_err(|error| error.to_string())?;
            if result.outcome == BattleOutcome::Victory && match_mode != "ranked_pvp" {
                // Match completion value is server policy, never a client field.
                result.resource_delta = result.resource_delta.max(25);
            }
            let result_hash = result.computed_hash().map_err(|error| error.to_string())?;
            let any_pending =
                apply_member_progression(&mut transaction, match_id, &result, &result_hash).await?;
            product_v2::apply_ranked_result(
                &mut transaction,
                match_id,
                result.outcome,
                &result_hash,
            )
            .await?;
            operations_v1::finalize_ranked_operations(
                &mut transaction,
                match_id,
                result.outcome,
                &result_hash,
                &snapshot_hash,
            )
            .await?;
            let settlement_state = if any_pending { "pending" } else { "settled" };
            let updated = sqlx::query(
                "update trnm_online_matches set phase = 'complete', simulation_json = $2,
                    result_json = $3, result_hash = $4, snapshot_hash = $5,
                    authoritative_tick = $6, settlement_state = $7, updated_at = now()
                 where match_id = $1 and assigned_instance_id = $8
                   and assigned_instance_epoch = $9",
            )
            .bind(match_id)
            .bind(serde_json::to_value(&sim).map_err(|error| error.to_string())?)
            .bind(serde_json::to_value(result).map_err(|error| error.to_string())?)
            .bind(result_hash)
            .bind(snapshot_hash)
            .bind(sim.tick as i64)
            .bind(settlement_state)
            .bind(state.instance_id.as_str())
            .bind(state.instance_epoch)
            .execute(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
            if updated.rows_affected() != 1 {
                return Err("match completion was fenced by a newer fleet epoch".to_string());
            }
        } else {
            let updated = sqlx::query(
                "update trnm_online_matches set simulation_json = $2, snapshot_hash = $3,
                    authoritative_tick = $4, updated_at = now()
                 where match_id = $1 and assigned_instance_id = $5
                   and assigned_instance_epoch = $6",
            )
            .bind(match_id)
            .bind(serde_json::to_value(&sim).map_err(|error| error.to_string())?)
            .bind(snapshot_hash)
            .bind(sim.tick as i64)
            .bind(state.instance_id.as_str())
            .bind(state.instance_epoch)
            .execute(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
            if updated.rows_affected() != 1 {
                return Err("match tick was fenced by a newer fleet epoch".to_string());
            }
        }
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        advanced = advanced.saturating_add(1);
    }
    Ok(advanced)
}

pub async fn settle_pending_matches(state: &AppState, limit: i64) -> Result<u64, String> {
    let ids = sqlx::query_scalar::<_, Uuid>(
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
        let Some(row) = sqlx::query(
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
        let campaign_ids = sqlx::query_scalar::<_, Option<String>>(
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
            let value: Value = sqlx::query_scalar(
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
        sqlx::query(
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    campaign: &CampaignSaveV1,
) -> Result<(), ApiError> {
    persist_campaign_string(transaction, campaign).await
}

async fn persist_campaign_string(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    campaign: &CampaignSaveV1,
) -> Result<(), ApiError> {
    let state_hash = hash_json(campaign)?;
    sqlx::query(
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    campaign_id: &str,
    player_id: &str,
    account_id: Uuid,
) -> Result<(), ApiError> {
    let owner = sqlx::query(
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    sqlx::query("select pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(format!("trnm-online-lobby:{player_id}"))
        .execute(&mut **transaction)
        .await
        .map_err(internal_db)?;
    Ok(())
}

async fn ensure_player_has_no_active_lobby(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    let active: bool = sqlx::query_scalar(
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    lobby_id: Uuid,
) -> Result<sqlx::postgres::PgRow, ApiError> {
    sqlx::query(
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
    lobby: &sqlx::postgres::PgRow,
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
    lobby: &sqlx::postgres::PgRow,
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
    let lobby = sqlx::query(
        "select display_name, owner_player_id, status, lobby_revision, map_id,
                queue_mode, match_id
         from trnm_online_lobbies where lobby_id = $1",
    )
    .bind(lobby_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "lobby not found", false))?;
    let rows = sqlx::query(
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
    let row = sqlx::query(
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
    let member_rows = sqlx::query(
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

fn campaign_view_from_row(row: &sqlx::postgres::PgRow) -> Result<OnlineCampaignView, ApiError> {
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
}
