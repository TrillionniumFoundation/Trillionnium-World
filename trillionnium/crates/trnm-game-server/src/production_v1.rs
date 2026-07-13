use super::*;
use axum::http::HeaderValue;
use chrono::{DateTime, Utc};
use trnm_online_protocol::{
    OnlineHostAttestationRequest, OnlineHostAttestationView, OnlineModerationCaseClaimRequest,
    OnlineModerationCaseClaimView, OnlineModerationShiftAccessRequest,
    OnlineModerationShiftStartRequest, OnlineModerationShiftView,
    OnlineProductionPlayerStatusRequest, OnlineProductionPlayerStatusView,
    OnlineProductionStatusView, OnlineReplayFrameView, OnlineSeasonAutomationRequest,
    OnlineSeasonAutomationView, OnlineSpectatorGrantView, OnlineSpectatorInviteAcceptRequest,
    OnlineSpectatorInviteCreateRequest, OnlineSpectatorInviteReceipt,
    OnlineSpectatorPlaybackRequest, OnlineSpectatorPlaybackView, ONLINE_OPERATIONS_BUILD,
    ONLINE_OPERATIONS_PROTOCOL,
};

const MODERATOR_HEADER: &str = "x-trnm-moderator";

fn require_production(protocol: &str, build: &str) -> Result<(), ApiError> {
    if trnm_online_protocol::validate_production_contract(protocol, build).is_err() {
        return Err(api_error(
            StatusCode::UPGRADE_REQUIRED,
            "Online Production endpoint requires a supported exact protocol/build pair",
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
    sqlx::query::query("select pg_advisory_xact_lock(hashtext('trnm-online-season-admin'))")
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    let automation_state = if request.automatic_activation {
        "scheduled"
    } else {
        "manual"
    };
    let updated = sqlx::query::query(
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
    sqlx::query::query(
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
    let creator_member: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3)",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let target_exists: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_campaigns where player_id = $1)",
    )
    .bind(&request.target_player_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let target_is_member: bool = sqlx::query_scalar::query_scalar(
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
    sqlx::query::query(
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
    let row = sqlx::query::query(
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
    let already_member: bool = sqlx::query_scalar::query_scalar(
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
    sqlx::query::query(
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
    sqlx::query::query(
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
    let row = sqlx::query::query(
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
    let match_row = sqlx::query::query(
        "select phase, authoritative_tick from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let frame_rows = sqlx::query::query(
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

async fn moderation_shift_view(
    state: &AppState,
    shift_id: Uuid,
) -> Result<OnlineModerationShiftView, ApiError> {
    let row = sqlx::query::query(
        "select shift_id, moderator_id, status, starts_at, ends_at,
                last_heartbeat_at, note,
                (select count(*) from trnm_online_moderation_case_claims claim
                  where claim.shift_id = shift.shift_id and claim.status = 'claimed') as open_claims,
                (select count(*) from trnm_online_moderation_case_claims claim
                  where claim.shift_id = shift.shift_id and claim.status = 'resolved') as resolved_claims
           from trnm_online_moderation_shifts shift where shift_id = $1",
    )
    .bind(shift_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "moderation shift does not exist", false))?;
    Ok(OnlineModerationShiftView {
        shift_id: row
            .try_get::<Uuid, _>("shift_id")
            .map_err(internal_db)?
            .to_string(),
        moderator_id: row.try_get("moderator_id").map_err(internal_db)?,
        status: row.try_get("status").map_err(internal_db)?,
        starts_at_epoch: row
            .try_get::<DateTime<Utc>, _>("starts_at")
            .map_err(internal_db)?
            .timestamp(),
        ends_at_epoch: row
            .try_get::<DateTime<Utc>, _>("ends_at")
            .map_err(internal_db)?
            .timestamp(),
        last_heartbeat_epoch: row
            .try_get::<DateTime<Utc>, _>("last_heartbeat_at")
            .map_err(internal_db)?
            .timestamp(),
        open_claims: u32::try_from(row.try_get::<i64, _>("open_claims").map_err(internal_db)?)
            .unwrap_or(u32::MAX),
        resolved_claims: u32::try_from(
            row.try_get::<i64, _>("resolved_claims")
                .map_err(internal_db)?,
        )
        .unwrap_or(u32::MAX),
        note: row.try_get("note").map_err(internal_db)?,
    })
}

fn validate_moderator_id(moderator_id: &str) -> bool {
    !moderator_id.is_empty()
        && moderator_id.len() <= 96
        && moderator_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

pub(super) async fn start_moderation_shift(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineModerationShiftStartRequest>,
) -> Result<(StatusCode, Json<OnlineModerationShiftView>), ApiError> {
    require_moderator(&state, &headers)?;
    if !validate_moderator_id(&request.moderator_id)
        || !(15..=480).contains(&request.duration_minutes)
        || request.note.trim().is_empty()
        || request.note.len() > 500
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation shift identity, duration or note is invalid",
            false,
        ));
    }
    sqlx::query::query(
        "update trnm_online_moderation_shifts set status = 'expired', closed_at = now(),
            close_note = 'expired by Production v2 maintenance on new shift start'
         where status = 'active' and (
            ends_at <= now() or last_heartbeat_at < now() - interval '5 minutes')",
    )
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    let shift_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_moderation_shifts (
            shift_id, moderator_id, ends_at, note
         ) values ($1, $2, now() + make_interval(mins => $3), $4)",
    )
    .bind(shift_id)
    .bind(&request.moderator_id)
    .bind(request.duration_minutes as i32)
    .bind(request.note.trim())
    .execute(&state.pool)
    .await
    .map_err(|db| {
        if db
            .as_database_error()
            .is_some_and(|database| database.is_unique_violation())
        {
            api_error(
                StatusCode::CONFLICT,
                "moderator already owns an active shift",
                false,
            )
        } else {
            internal_db(db)
        }
    })?;
    Ok((
        StatusCode::CREATED,
        Json(moderation_shift_view(&state, shift_id).await?),
    ))
}

pub(super) async fn heartbeat_moderation_shift(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineModerationShiftAccessRequest>,
) -> Result<Json<OnlineModerationShiftView>, ApiError> {
    require_moderator(&state, &headers)?;
    let shift_id = Uuid::parse_str(&request.shift_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "shift_id must be a UUID", false))?;
    if !validate_moderator_id(&request.moderator_id) || request.note.len() > 500 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation shift heartbeat identity or note is invalid",
            false,
        ));
    }
    let updated = sqlx::query::query(
        "update trnm_online_moderation_shifts set last_heartbeat_at = now(),
            note = case when $3 = '' then note else $3 end
         where shift_id = $1 and moderator_id = $2 and status = 'active'
           and ends_at > now()",
    )
    .bind(shift_id)
    .bind(&request.moderator_id)
    .bind(request.note.trim())
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "moderation shift is not active or does not belong to this moderator",
            false,
        ));
    }
    Ok(Json(moderation_shift_view(&state, shift_id).await?))
}

pub(super) async fn claim_moderation_case(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineModerationCaseClaimRequest>,
) -> Result<(StatusCode, Json<OnlineModerationCaseClaimView>), ApiError> {
    require_moderator(&state, &headers)?;
    let shift_id = Uuid::parse_str(&request.shift_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "shift_id must be a UUID", false))?;
    let case_id = Uuid::parse_str(&request.case_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "case_id must be a UUID", false))?;
    if !validate_moderator_id(&request.moderator_id)
        || !matches!(request.case_kind.as_str(), "report" | "appeal")
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation claim identity or case kind is invalid",
            false,
        ));
    }
    let active_shift: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_moderation_shifts
          where shift_id = $1 and moderator_id = $2 and status = 'active'
            and ends_at > now() and last_heartbeat_at > now() - interval '2 minutes')",
    )
    .bind(shift_id)
    .bind(&request.moderator_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let case_open: bool = if request.case_kind == "report" {
        sqlx::query_scalar::query_scalar(
            "select exists(select 1 from trnm_online_reports where report_id = $1 and status = 'open')",
        )
        .bind(case_id)
        .fetch_one(&state.pool)
        .await
        .map_err(internal_db)?
    } else {
        sqlx::query_scalar::query_scalar(
            "select exists(select 1 from trnm_online_enforcement_appeals where appeal_id = $1 and status = 'pending')",
        )
        .bind(case_id)
        .fetch_one(&state.pool)
        .await
        .map_err(internal_db)?
    };
    if !active_shift || !case_open {
        return Err(api_error(
            StatusCode::CONFLICT,
            "an active fresh shift and an unresolved case are required",
            false,
        ));
    }
    let claim_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_moderation_case_claims (
            claim_id, shift_id, case_kind, case_id
         ) values ($1, $2, $3, $4)",
    )
    .bind(claim_id)
    .bind(shift_id)
    .bind(&request.case_kind)
    .bind(case_id)
    .execute(&state.pool)
    .await
    .map_err(|db| {
        if db
            .as_database_error()
            .is_some_and(|database| database.is_unique_violation())
        {
            api_error(
                StatusCode::CONFLICT,
                "moderation case is already claimed",
                false,
            )
        } else {
            internal_db(db)
        }
    })?;
    Ok((
        StatusCode::CREATED,
        Json(OnlineModerationCaseClaimView {
            claim_id: claim_id.to_string(),
            shift_id: shift_id.to_string(),
            case_kind: request.case_kind,
            case_id: case_id.to_string(),
            status: "claimed".to_string(),
            claimed_at_epoch: Utc::now().timestamp(),
            resolved_at_epoch: None,
        }),
    ))
}

pub(super) async fn close_moderation_shift(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineModerationShiftAccessRequest>,
) -> Result<Json<OnlineModerationShiftView>, ApiError> {
    require_moderator(&state, &headers)?;
    let shift_id = Uuid::parse_str(&request.shift_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "shift_id must be a UUID", false))?;
    if !validate_moderator_id(&request.moderator_id)
        || request.note.trim().is_empty()
        || request.note.len() > 500
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation shift close identity or note is invalid",
            false,
        ));
    }
    let open_claims: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_moderation_case_claims
         where shift_id = $1 and status = 'claimed'",
    )
    .bind(shift_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if open_claims != 0 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "moderation shift cannot close with unresolved claims",
            false,
        ));
    }
    let updated = sqlx::query::query(
        "update trnm_online_moderation_shifts set status = 'closed',
            closed_at = now(), close_note = $3
         where shift_id = $1 and moderator_id = $2 and status = 'active'",
    )
    .bind(shift_id)
    .bind(&request.moderator_id)
    .bind(request.note.trim())
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "moderation shift is not active or does not belong to this moderator",
            false,
        ));
    }
    Ok(Json(moderation_shift_view(&state, shift_id).await?))
}

pub(super) async fn host_attestation(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineHostAttestationRequest>,
) -> Result<Json<OnlineHostAttestationView>, ApiError> {
    require_moderator(&state, &headers)?;
    require_production(&request.protocol_version, &request.build_id)?;
    if !(32..=128).contains(&request.challenge.len())
        || !request
            .challenge
            .bytes()
            .all(|byte| byte.is_ascii_graphic())
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "host attestation challenge is invalid",
            false,
        ));
    }
    let observed_at_epoch = Utc::now().timestamp();
    let evidence_hash = format!(
        "{:x}",
        Sha256::digest(
            format!(
                "{}:{}:{}:{}:{}:{}:{}",
                request.protocol_version,
                request.build_id,
                state.instance_id,
                state.instance_epoch,
                state.physical_host_id,
                request.challenge,
                observed_at_epoch
            )
            .as_bytes()
        )
    );
    sqlx::query::query(
        "insert into trnm_online_host_attestation_audit (
            attestation_id, instance_id, instance_epoch, physical_host_id,
            region, challenge_hash, evidence_hash
         ) values ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(Uuid::new_v4())
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(state.physical_host_id.as_str())
    .bind(state.region.as_str())
    .bind(format!(
        "{:x}",
        Sha256::digest(request.challenge.as_bytes())
    ))
    .bind(&evidence_hash)
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    Ok(Json(OnlineHostAttestationView {
        protocol_version: request.protocol_version,
        build_id: request.build_id,
        instance_id: state.instance_id.as_ref().clone(),
        instance_epoch: state.instance_epoch,
        physical_host_id: state.physical_host_id.as_ref().clone(),
        region: state.region.as_ref().clone(),
        challenge: request.challenge,
        observed_at_epoch,
        evidence_hash,
        boundary: "durable live-instance challenge; not hardware identity or cross-host quorum"
            .to_string(),
    }))
}

pub(super) async fn player_production_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineProductionPlayerStatusRequest>,
) -> Result<Json<OnlineProductionPlayerStatusView>, ApiError> {
    require_production(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let signer = state
        .cex
        .signer_attestation()
        .await
        .map_err(|message| api_error(StatusCode::SERVICE_UNAVAILABLE, message, true))?;
    let active_season = sqlx::query::query(
        "select season_id, ends_at from trnm_online_seasons where status = 'active'",
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?;
    let automatic_season_id: Option<String> = sqlx::query_scalar::query_scalar(
        "select season_id from trnm_online_seasons
         where status = 'scheduled' and automatic_activation order by starts_at limit 1",
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?;
    let active_matches: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_matches where phase in ('created', 'running')",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let active_spectator_grants: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_spectator_grants
         where viewer_player_id = $1 and viewer_account_id = $2::uuid and expires_at > now()",
    )
    .bind(&request.player_id)
    .bind(&request.account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let distinct_hosts: i64 = sqlx::query_scalar::query_scalar(
        "select count(distinct physical_host_id) from trnm_online_fleet_instances
         where status in ('active', 'draining') and lease_expires_at > now()",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    Ok(Json(OnlineProductionPlayerStatusView {
        protocol_version: request.protocol_version,
        build_id: request.build_id,
        active_season_id: active_season
            .as_ref()
            .and_then(|row| row.try_get("season_id").ok()),
        active_season_ends_at_epoch: active_season.as_ref().and_then(|row| {
            row.try_get::<DateTime<Utc>, _>("ends_at")
                .ok()
                .map(|value| value.timestamp())
        }),
        automatic_season_id,
        region: state.region.as_ref().clone(),
        fleet_capacity: state.capacity as u32,
        active_matches: u32::try_from(active_matches).unwrap_or(u32::MAX),
        admission_state: if active_matches < i64::from(state.capacity) {
            "accepting".to_string()
        } else {
            "capacity_limited".to_string()
        },
        admission_limit_per_minute: state.rate_limit_per_minute,
        active_spectator_grants: u32::try_from(active_spectator_grants).unwrap_or(u32::MAX),
        signer_key_id: signer.key_id,
        signer_provider_kind: signer.provider_kind,
        signer_registry_verified: true,
        distinct_healthy_physical_hosts: u32::try_from(distinct_hosts).unwrap_or(u32::MAX),
        cross_host_failover_attested: false,
        public_edge_attested: false,
        kms_hsm_attested: false,
    }))
}

pub(super) async fn production_status(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<OnlineProductionStatusView>, ApiError> {
    require_moderator(&state, &headers)?;
    let signer_attestation = state
        .cex
        .signer_attestation()
        .await
        .map_err(|message| api_error(StatusCode::SERVICE_UNAVAILABLE, message, true))?;
    let signer = state
        .cex
        .signer_readiness()
        .await
        .map_err(|message| api_error(StatusCode::SERVICE_UNAVAILABLE, message, true))?;
    let automatic_season_id: Option<String> = sqlx::query_scalar::query_scalar(
        "select season_id from trnm_online_seasons
         where status = 'scheduled' and automatic_activation order by starts_at limit 1",
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?;
    let pending_appeals: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_enforcement_appeals where status = 'pending'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let overdue_appeals: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_enforcement_appeals
         where status = 'pending' and due_at < now()",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let escalated_appeals: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_appeal_escalations where status = 'open'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let distinct_hosts: i64 = sqlx::query_scalar::query_scalar(
        "select count(distinct physical_host_id) from trnm_online_fleet_instances
         where status in ('active', 'draining') and lease_expires_at > now()",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let admission = sqlx::query::query(
        "select coalesce(sum(request_count), 0)::bigint as requests,
                coalesce(sum(rejection_count), 0)::bigint as rejections
           from trnm_online_admission_windows
          where window_started_at >= date_trunc('minute', now()) - interval '1 minute'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let recent_capacity_samples: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_capacity_samples
         where sampled_at > now() - interval '1 minute'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let active_moderation_shifts: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_moderation_shifts
         where status = 'active' and ends_at > now()
           and last_heartbeat_at > now() - interval '5 minutes'",
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
        distributed_admission: true,
        current_admission_requests: u32::try_from(
            admission
                .try_get::<i64, _>("requests")
                .map_err(internal_db)?,
        )
        .unwrap_or(u32::MAX),
        current_admission_rejections: u32::try_from(
            admission
                .try_get::<i64, _>("rejections")
                .map_err(internal_db)?,
        )
        .unwrap_or(u32::MAX),
        recent_capacity_samples: u32::try_from(recent_capacity_samples).unwrap_or(u32::MAX),
        active_moderation_shifts: u32::try_from(active_moderation_shifts).unwrap_or(u32::MAX),
        signer_provider_kind: signer_attestation.provider_kind,
        signer_registry_verified: true,
        kms_hsm_attested: false,
        cross_host_failover_attested: false,
    }))
}

pub(super) async fn run_production_maintenance(state: &AppState) -> Result<(), String> {
    sqlx::query::query(
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
    sqlx::query::query(
        "update trnm_online_moderation_shifts set status = 'expired', closed_at = now(),
            close_note = 'shift expired without a fresh heartbeat'
         where status = 'active' and (
            ends_at <= now() or last_heartbeat_at < now() - interval '5 minutes')",
    )
    .execute(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    let active_matches: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_matches
         where phase in ('created', 'running') and assigned_instance_id = $1
           and assigned_instance_epoch = $2",
    )
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .fetch_one(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    let admission = sqlx::query::query(
        "select coalesce(sum(request_count), 0)::bigint as requests,
                coalesce(sum(rejection_count), 0)::bigint as rejections
           from trnm_online_admission_windows
          where window_started_at = date_trunc('minute', now())
            and last_instance_id = $1",
    )
    .bind(state.instance_id.as_str())
    .fetch_one(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    sqlx::query::query(
        "insert into trnm_online_capacity_samples (
            sample_id, instance_id, instance_epoch, physical_host_id, region,
            active_matches, fleet_capacity, admission_requests, admission_rejections
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(Uuid::new_v4())
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(state.physical_host_id.as_str())
    .bind(state.region.as_str())
    .bind(i32::try_from(active_matches).unwrap_or(i32::MAX))
    .bind(state.capacity)
    .bind(admission.try_get::<i64, _>("requests").unwrap_or_default())
    .bind(
        admission
            .try_get::<i64, _>("rejections")
            .unwrap_or_default(),
    )
    .execute(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    sqlx::query::query(
        "delete from trnm_online_admission_windows
          where window_started_at < now() - interval '10 minutes'",
    )
    .execute(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    sqlx::query::query(
        "delete from trnm_online_capacity_samples
          where sampled_at < now() - interval '24 hours'",
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
    let locked: bool = sqlx::query_scalar::query_scalar(
        "select pg_try_advisory_xact_lock(hashtext('trnm-online-season-admin'))",
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if !locked {
        return Ok(());
    }
    let Some(target) = sqlx::query::query(
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
    let ranked_busy: bool = sqlx::query_scalar::query_scalar(
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
        let updated = sqlx::query::query(
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
            sqlx::query::query(
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
    let previous_active: Option<String> = sqlx::query_scalar::query_scalar(
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
        sqlx::query::query(
            "update trnm_online_seasons set status = 'closed',
                ends_at = greatest(starts_at + interval '1 second', now())
             where season_id = $1 and status = 'active'",
        )
        .bind(previous)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    sqlx::query::query(
        "update trnm_online_seasons set status = 'active',
            automation_state = 'activated', automation_deferred_reason = null,
            last_automation_attempt_at = now()
         where season_id = $1 and status = 'scheduled'",
    )
    .bind(&season_id)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    sqlx::query::query(
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
