use super::*;
use axum::http::HeaderValue;
use chrono::{DateTime, Utc};
use trnm_online_protocol::{
    OnlineProductionStatusView, OnlineReplayFrameView, OnlineSeasonAutomationRequest,
    OnlineSeasonAutomationView, OnlineSpectatorGrantView, OnlineSpectatorInviteAcceptRequest,
    OnlineSpectatorInviteCreateRequest, OnlineSpectatorInviteReceipt,
    OnlineSpectatorPlaybackRequest, OnlineSpectatorPlaybackView, ONLINE_OPERATIONS_BUILD,
    ONLINE_OPERATIONS_PROTOCOL,
};

const MODERATOR_HEADER: &str = "x-trnm-moderator";

fn require_production(protocol: &str, build: &str) -> Result<(), ApiError> {
    if protocol != ONLINE_OPERATIONS_PROTOCOL || build != ONLINE_OPERATIONS_BUILD {
        return Err(api_error(
            StatusCode::UPGRADE_REQUIRED,
            "Online Production endpoint requires the exact Production v1 protocol/build",
            false,
        ));
    }
    Ok(())
}

fn require_moderator(state: &AppState, headers: &HeaderMap) -> Result<(), ApiError> {
    let supplied = headers
        .get(MODERATOR_HEADER)
        .and_then(|value: &HeaderValue| value.to_str().ok())
        .unwrap_or_default();
    if supplied.is_empty() || supplied != state.moderator_token.as_str() {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "moderator credential is required",
            false,
        ));
    }
    Ok(())
}

pub(super) async fn configure_season_automation(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSeasonAutomationRequest>,
) -> Result<Json<OnlineSeasonAutomationView>, ApiError> {
    require_moderator(&state, &headers)?;
    if request.season_id.is_empty()
        || request.season_id.len() > 80
        || !request
            .season_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "season_id is invalid",
            false,
        ));
    }
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    sqlx::query("select pg_advisory_xact_lock(hashtext('trnm-online-season-admin'))")
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    let automation_state = if request.automatic_activation {
        "scheduled"
    } else {
        "manual"
    };
    let updated = sqlx::query(
        "update trnm_online_seasons set automatic_activation = $2,
            automation_state = $3, automation_deferred_reason = null,
            last_automation_attempt_at = null
         where season_id = $1 and status = 'scheduled'",
    )
    .bind(&request.season_id)
    .bind(request.automatic_activation)
    .bind(automation_state)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "only a scheduled season can change automation",
            false,
        ));
    }
    sqlx::query(
        "insert into trnm_online_season_automation_audit (
            audit_id, season_id, action, detail
         ) values ($1, $2, 'configure', $3)",
    )
    .bind(Uuid::new_v4())
    .bind(&request.season_id)
    .bind(json!({"automatic_activation": request.automatic_activation}))
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineSeasonAutomationView {
        season_id: request.season_id,
        automatic_activation: request.automatic_activation,
        automation_state: automation_state.to_string(),
        deferred_reason: None,
    }))
}

pub(super) async fn create_spectator_invite(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSpectatorInviteCreateRequest>,
) -> Result<(StatusCode, Json<OnlineSpectatorInviteReceipt>), ApiError> {
    require_production(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    if request.target_player_id.is_empty()
        || request.target_player_id == request.player_id
        || !(30..=600).contains(&request.delay_seconds)
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "target player or spectator delay is invalid",
            false,
        ));
    }
    let match_id = Uuid::parse_str(&request.match_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "match_id must be a UUID", false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let creator_member: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3)",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let target_exists: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_campaigns where player_id = $1)",
    )
    .bind(&request.target_player_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let target_is_member: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_match_members
         where match_id = $1 and player_id = $2)",
    )
    .bind(match_id)
    .bind(&request.target_player_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if !creator_member || !target_exists || target_is_member {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "spectator invite requires a member and a distinct known target",
            false,
        ));
    }
    let invite_id = Uuid::new_v4();
    let invite_token = format!(
        "{}{}{}",
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple()
    );
    let token_hash = format!("{:x}", Sha256::digest(invite_token.as_bytes()));
    let expires_at = Utc::now() + chrono::Duration::minutes(15);
    sqlx::query(
        "insert into trnm_online_spectator_invites (
            invite_id, match_id, creator_player_id, target_player_id,
            token_hash, delay_seconds, expires_at
         ) values ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(invite_id)
    .bind(match_id)
    .bind(&request.player_id)
    .bind(&request.target_player_id)
    .bind(token_hash)
    .bind(request.delay_seconds as i32)
    .bind(expires_at)
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    Ok((
        StatusCode::CREATED,
        Json(OnlineSpectatorInviteReceipt {
            invite_id: invite_id.to_string(),
            match_id: match_id.to_string(),
            target_player_id: request.target_player_id,
            invite_token,
            delay_seconds: request.delay_seconds,
            expires_at_epoch: expires_at.timestamp(),
        }),
    ))
}

pub(super) async fn accept_spectator_invite(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSpectatorInviteAcceptRequest>,
) -> Result<(StatusCode, Json<OnlineSpectatorGrantView>), ApiError> {
    require_production(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    if request.invite_token.len() != 96
        || !request
            .invite_token
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "invite token is invalid",
            false,
        ));
    }
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let token_hash = format!("{:x}", Sha256::digest(request.invite_token.as_bytes()));
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let row = sqlx::query(
        "select invite_id, match_id, target_player_id, delay_seconds
         from trnm_online_spectator_invites
         where token_hash = $1 and expires_at > now() and consumed_at is null
         for update",
    )
    .bind(token_hash)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| {
        api_error(
            StatusCode::CONFLICT,
            "spectator invite is invalid or consumed",
            false,
        )
    })?;
    let target: String = row.try_get("target_player_id").map_err(internal_db)?;
    let match_id: Uuid = row.try_get("match_id").map_err(internal_db)?;
    if target != request.player_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "spectator invite belongs to another player",
            false,
        ));
    }
    let already_member: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_match_members
         where match_id = $1 and player_id = $2)",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if already_member {
        return Err(api_error(
            StatusCode::CONFLICT,
            "match members do not need spectator grants",
            false,
        ));
    }
    let invite_id: Uuid = row.try_get("invite_id").map_err(internal_db)?;
    let delay_seconds: i32 = row.try_get("delay_seconds").map_err(internal_db)?;
    let grant_id = Uuid::new_v4();
    let expires_at = Utc::now() + chrono::Duration::hours(4);
    sqlx::query(
        "insert into trnm_online_spectator_grants (
            grant_id, invite_id, match_id, viewer_player_id, viewer_account_id,
            delay_seconds, expires_at
         ) values ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(grant_id)
    .bind(invite_id)
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(delay_seconds)
    .bind(expires_at)
    .execute(&mut *transaction)
    .await
    .map_err(|db| {
        if db
            .as_database_error()
            .is_some_and(|database| database.is_unique_violation())
        {
            api_error(
                StatusCode::CONFLICT,
                "spectator grant already exists",
                false,
            )
        } else {
            internal_db(db)
        }
    })?;
    sqlx::query(
        "update trnm_online_spectator_invites set consumed_at = now()
         where invite_id = $1 and consumed_at is null",
    )
    .bind(invite_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok((
        StatusCode::CREATED,
        Json(OnlineSpectatorGrantView {
            grant_id: grant_id.to_string(),
            match_id: match_id.to_string(),
            viewer_player_id: request.player_id,
            delay_seconds: delay_seconds as u32,
            expires_at_epoch: expires_at.timestamp(),
        }),
    ))
}

pub(super) async fn spectator_playback(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSpectatorPlaybackRequest>,
) -> Result<Json<OnlineSpectatorPlaybackView>, ApiError> {
    require_production(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let grant_id = Uuid::parse_str(&request.grant_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "grant_id must be a UUID", false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let row = sqlx::query(
        "select grant_id, match_id, viewer_player_id, delay_seconds, expires_at
         from trnm_online_spectator_grants
         where grant_id = $1 and viewer_player_id = $2 and viewer_account_id = $3
           and expires_at > now()",
    )
    .bind(grant_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::FORBIDDEN, "spectator grant is invalid", false))?;
    let match_id: Uuid = row.try_get("match_id").map_err(internal_db)?;
    let delay_seconds: i32 = row.try_get("delay_seconds").map_err(internal_db)?;
    let expires_at: DateTime<Utc> = row.try_get("expires_at").map_err(internal_db)?;
    let match_row = sqlx::query(
        "select phase, authoritative_tick from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let frame_rows = sqlx::query(
        "select tick, snapshot_hash, simulation_json, frame_kind
         from trnm_online_replay_frames
         where match_id = $1
           and created_at <= now() - make_interval(secs => $2::double precision)
         order by tick limit 513",
    )
    .bind(match_id)
    .bind(delay_seconds)
    .fetch_all(&state.pool)
    .await
    .map_err(internal_db)?;
    if frame_rows.len() > 512 {
        return Err(api_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "delayed spectator frame envelope exceeded",
            false,
        ));
    }
    let frames = frame_rows
        .iter()
        .map(|frame| {
            Ok(OnlineReplayFrameView {
                tick: frame.try_get::<i64, _>("tick").map_err(internal_db)? as u64,
                snapshot_hash: frame.try_get("snapshot_hash").map_err(internal_db)?,
                frame_kind: frame.try_get("frame_kind").map_err(internal_db)?,
                simulation: frame.try_get("simulation_json").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let visible_through_tick = frames.last().map(|frame| frame.tick).unwrap_or_default();
    let terminal_visible = frames
        .last()
        .is_some_and(|frame| frame.frame_kind == "terminal");
    Ok(Json(OnlineSpectatorPlaybackView {
        grant: OnlineSpectatorGrantView {
            grant_id: grant_id.to_string(),
            match_id: match_id.to_string(),
            viewer_player_id: request.player_id,
            delay_seconds: delay_seconds as u32,
            expires_at_epoch: expires_at.timestamp(),
        },
        match_phase: match_row.try_get("phase").map_err(internal_db)?,
        authoritative_tick: match_row
            .try_get::<i64, _>("authoritative_tick")
            .map_err(internal_db)? as u64,
        visible_through_tick,
        frames,
        terminal_visible,
    }))
}

pub(super) async fn production_status(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<OnlineProductionStatusView>, ApiError> {
    require_moderator(&state, &headers)?;
    let signer = state
        .cex
        .signer_readiness()
        .await
        .map_err(|message| api_error(StatusCode::SERVICE_UNAVAILABLE, message, true))?;
    let automatic_season_id: Option<String> = sqlx::query_scalar(
        "select season_id from trnm_online_seasons
         where status = 'scheduled' and automatic_activation order by starts_at limit 1",
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?;
    let pending_appeals: i64 = sqlx::query_scalar(
        "select count(*) from trnm_online_enforcement_appeals where status = 'pending'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let overdue_appeals: i64 = sqlx::query_scalar(
        "select count(*) from trnm_online_enforcement_appeals
         where status = 'pending' and due_at < now()",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let escalated_appeals: i64 = sqlx::query_scalar(
        "select count(*) from trnm_online_appeal_escalations where status = 'open'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let distinct_hosts: i64 = sqlx::query_scalar(
        "select count(distinct physical_host_id) from trnm_online_fleet_instances
         where status in ('active', 'draining') and lease_expires_at > now()",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    Ok(Json(OnlineProductionStatusView {
        protocol_version: ONLINE_OPERATIONS_PROTOCOL.to_string(),
        build_id: ONLINE_OPERATIONS_BUILD.to_string(),
        signer_ready: signer.status == "ok",
        signer_key_id: signer.key_id,
        signer_custody: signer.custody,
        request_rate_limit_per_minute: state.rate_limit_per_minute,
        request_body_limit_bytes: state.request_body_limit_bytes,
        automatic_season_id,
        pending_appeals: pending_appeals as u32,
        overdue_appeals: overdue_appeals as u32,
        escalated_appeals: escalated_appeals as u32,
        physical_host_id: state.physical_host_id.as_ref().clone(),
        distinct_healthy_physical_hosts: distinct_hosts as u32,
        public_edge_attested: false,
    }))
}

pub(super) async fn run_production_maintenance(state: &AppState) -> Result<(), String> {
    sqlx::query(
        "insert into trnm_online_appeal_escalations (
            escalation_id, appeal_id, escalation_kind, detail
         ) select gen_random_uuid(), appeal_id, 'sla_overdue',
                  jsonb_build_object('due_at', due_at, 'detected_at', now())
             from trnm_online_enforcement_appeals
            where status = 'pending' and due_at < now()
         on conflict (appeal_id) do nothing",
    )
    .execute(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    run_season_automation(state).await
}

async fn run_season_automation(state: &AppState) -> Result<(), String> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    let locked: bool = sqlx::query_scalar(
        "select pg_try_advisory_xact_lock(hashtext('trnm-online-season-admin'))",
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if !locked {
        return Ok(());
    }
    let Some(target) = sqlx::query(
        "select season_id from trnm_online_seasons
         where status = 'scheduled' and automatic_activation
           and starts_at <= now() and ends_at > now()
         order by starts_at, season_id limit 1 for update",
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(());
    };
    let season_id: String = target
        .try_get("season_id")
        .map_err(|error| error.to_string())?;
    let ranked_busy: bool = sqlx::query_scalar(
        "select exists(
            select 1 from trnm_online_matches
             where match_mode = 'ranked_pvp' and phase in ('created', 'running')
            union all
            select 1 from trnm_online_solo_queue where status = 'queued'
         )",
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if ranked_busy {
        let updated = sqlx::query(
            "update trnm_online_seasons set automation_state = 'deferred',
                automation_deferred_reason = 'ranked queue or match is active',
                last_automation_attempt_at = now()
             where season_id = $1 and (
                last_automation_attempt_at is null
                or last_automation_attempt_at < now() - interval '1 minute')",
        )
        .bind(&season_id)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        if updated.rows_affected() == 1 {
            sqlx::query(
                "insert into trnm_online_season_automation_audit (
                    audit_id, season_id, action, detail
                 ) values ($1, $2, 'deferred', $3)",
            )
            .bind(Uuid::new_v4())
            .bind(&season_id)
            .bind(json!({"reason": "ranked queue or match is active"}))
            .execute(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?;
        }
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(());
    }
    let previous_active: Option<String> = sqlx::query_scalar(
        "select season_id from trnm_online_seasons where status = 'active' for update",
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut archived_entries = 0;
    if let Some(previous) = previous_active.as_deref() {
        archived_entries = operations_v1::archive_season(&mut transaction, previous)
            .await
            .map_err(|error| error.1 .0.error)?;
        sqlx::query(
            "update trnm_online_seasons set status = 'closed',
                ends_at = greatest(starts_at + interval '1 second', now())
             where season_id = $1 and status = 'active'",
        )
        .bind(previous)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    sqlx::query(
        "update trnm_online_seasons set status = 'active',
            automation_state = 'activated', automation_deferred_reason = null,
            last_automation_attempt_at = now()
         where season_id = $1 and status = 'scheduled'",
    )
    .bind(&season_id)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    sqlx::query(
        "insert into trnm_online_season_automation_audit (
            audit_id, season_id, action, previous_active_season_id, detail
         ) values ($1, $2, 'activate', $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(&season_id)
    .bind(&previous_active)
    .bind(json!({"archived_entries": archived_entries}))
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())
}
