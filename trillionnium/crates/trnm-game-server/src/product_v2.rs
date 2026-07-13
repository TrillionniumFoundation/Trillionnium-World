use super::*;
use axum::http::HeaderValue;
use trnm_online_protocol::{
    OnlineBlockRequest, OnlineFriendRequest, OnlineFriendResolveRequest, OnlineRatingView,
    OnlineReportCreateRequest, OnlineReportResolveRequest, OnlineReportView,
    OnlineSocialAccessRequest, OnlineSocialView, OnlineSoloQueueAccessRequest,
    OnlineSoloQueueJoinRequest, OnlineSoloQueueStatus, OnlineSoloQueueView,
};

const MODERATOR_HEADER: &str = "x-trnm-moderator";

fn require_v2(protocol: &str, build: &str) -> Result<(), ApiError> {
    if protocol != ONLINE_PRODUCT_PROTOCOL || build != ONLINE_PRODUCT_BUILD {
        return Err(api_error(
            StatusCode::UPGRADE_REQUIRED,
            "Online Product v2 endpoint requires the current product protocol and build",
            false,
        ));
    }
    Ok(())
}

async fn ensure_rating(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    player_id: &str,
    account_id: Uuid,
) -> Result<i32, ApiError> {
    sqlx::query(
        "insert into trnm_online_ratings (player_id, account_id)
         values ($1, $2) on conflict (player_id) do nothing",
    )
    .bind(player_id)
    .bind(account_id)
    .execute(&mut **transaction)
    .await
    .map_err(internal_db)?;
    let row = sqlx::query(
        "select account_id, rating from trnm_online_ratings where player_id = $1 for update",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(internal_db)?;
    if row.try_get::<Uuid, _>("account_id").map_err(internal_db)? != account_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "rating identity belongs to another account",
            false,
        ));
    }
    row.try_get("rating").map_err(internal_db)
}

fn queue_view_from_row(row: &sqlx::postgres::PgRow) -> Result<OnlineSoloQueueView, ApiError> {
    let status: String = row.try_get("status").map_err(internal_db)?;
    Ok(OnlineSoloQueueView {
        protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
        build_id: ONLINE_PRODUCT_BUILD.to_string(),
        ticket_id: row
            .try_get::<Uuid, _>("ticket_id")
            .map_err(internal_db)?
            .to_string(),
        player_id: row.try_get("player_id").map_err(internal_db)?,
        status: match status.as_str() {
            "queued" => OnlineSoloQueueStatus::Queued,
            "matched" => OnlineSoloQueueStatus::Matched,
            _ => OnlineSoloQueueStatus::Cancelled,
        },
        queue_mode: row.try_get("queue_mode").map_err(internal_db)?,
        map_id: row.try_get("map_id").map_err(internal_db)?,
        rating: row.try_get("rating_at_join").map_err(internal_db)?,
        matched_lobby_id: row
            .try_get::<Option<Uuid>, _>("matched_lobby_id")
            .map_err(internal_db)?
            .map(|value| value.to_string()),
        match_id: row
            .try_get::<Option<Uuid>, _>("match_id")
            .map_err(internal_db)?
            .map(|value| value.to_string()),
        opponent_player_id: row.try_get("opponent_player_id").map_err(internal_db)?,
    })
}

async fn fetch_latest_ticket(
    pool: &PgPool,
    player_id: &str,
) -> Result<OnlineSoloQueueView, ApiError> {
    let row = sqlx::query(
        "select ticket_id, player_id, status, queue_mode, map_id, rating_at_join,
                matched_lobby_id, match_id, opponent_player_id
         from trnm_online_solo_queue where player_id = $1
         order by created_at desc limit 1",
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "solo queue ticket not found", false))?;
    queue_view_from_row(&row)
}

pub(super) async fn join_solo_queue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSoloQueueJoinRequest>,
) -> Result<Json<OnlineSoloQueueView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    mission_for_map(&request.map_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    sqlx::query("select pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(format!("trnm-ranked-pvp:{}", request.map_id))
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    lock_player_lobby_scope(&mut transaction, &request.player_id).await?;
    sqlx::query(
        "update trnm_online_solo_queue set status = 'cancelled', updated_at = now()
         where status = 'queued' and created_at < now() - interval '15 minutes'",
    )
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    ensure_campaign_owner(
        &mut transaction,
        &request.campaign_id,
        &request.player_id,
        account_id,
    )
    .await?;
    ensure_player_has_no_active_lobby(&mut transaction, &request.player_id).await?;
    let already_queued: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_solo_queue
         where player_id = $1 and status = 'queued')",
    )
    .bind(&request.player_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if already_queued {
        return Err(api_error(
            StatusCode::CONFLICT,
            "player already has an active solo queue ticket",
            true,
        ));
    }
    let rating = ensure_rating(&mut transaction, &request.player_id, account_id).await?;
    let candidate = sqlx::query(
        "select q.ticket_id, q.player_id, q.account_id, q.campaign_id, q.rating_at_join
         from trnm_online_solo_queue q
         where q.status = 'queued' and q.queue_mode = 'ranked_pvp' and q.map_id = $1
           and q.player_id <> $2 and abs(q.rating_at_join - $3) <= 400
           and not exists (
             select 1 from trnm_online_blocks b
             where (b.blocker_player_id = q.player_id and b.blocked_player_id = $2)
                or (b.blocker_player_id = $2 and b.blocked_player_id = q.player_id)
           )
           and not exists (
             select 1 from trnm_online_rating_events e
             where e.created_at > now() - interval '10 minutes'
               and ((e.player_id = q.player_id and e.opponent_player_id = $2)
                 or (e.player_id = $2 and e.opponent_player_id = q.player_id))
           )
         order by q.created_at asc for update skip locked limit 1",
    )
    .bind(&request.map_id)
    .bind(&request.player_id)
    .bind(rating)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let ticket_id = Uuid::new_v4();
    let Some(candidate) = candidate else {
        sqlx::query(
            "insert into trnm_online_solo_queue (
                ticket_id, player_id, account_id, campaign_id, map_id, rating_at_join
             ) values ($1, $2, $3, $4, $5, $6)",
        )
        .bind(ticket_id)
        .bind(&request.player_id)
        .bind(account_id)
        .bind(&request.campaign_id)
        .bind(&request.map_id)
        .bind(rating)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
        transaction.commit().await.map_err(internal_db)?;
        return Ok(Json(
            fetch_latest_ticket(&state.pool, &request.player_id).await?,
        ));
    };

    let opponent_player_id: String = candidate.try_get("player_id").map_err(internal_db)?;
    let opponent_account_id: Uuid = candidate.try_get("account_id").map_err(internal_db)?;
    let opponent_campaign_id: String = candidate.try_get("campaign_id").map_err(internal_db)?;
    lock_player_lobby_scope(&mut transaction, &opponent_player_id).await?;
    ensure_player_has_no_active_lobby(&mut transaction, &opponent_player_id).await?;
    let lobby_id = Uuid::new_v4();
    let match_id = Uuid::new_v4();
    let allocation_id = Uuid::new_v4();
    let join_code = match_id.simple().to_string()[..10].to_ascii_uppercase();
    sqlx::query(
        "insert into trnm_online_lobbies (
            lobby_id, display_name, owner_player_id, owner_account_id, status,
            lobby_revision, map_id, queue_mode
         ) values ($1, $2, $3, $4, 'queued', 1, $5, 'ranked_pvp')",
    )
    .bind(lobby_id)
    .bind(format!("Ranked PvP {}", lobby_id.simple()))
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.map_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    for (player_id, member_account, campaign_id, role) in [
        (
            request.player_id.as_str(),
            account_id,
            request.campaign_id.as_str(),
            "owner",
        ),
        (
            opponent_player_id.as_str(),
            opponent_account_id,
            opponent_campaign_id.as_str(),
            "member",
        ),
    ] {
        sqlx::query(
            "insert into trnm_online_lobby_members (
                lobby_id, player_id, account_id, campaign_id, member_role, ready
             ) values ($1, $2, $3, $4, $5, true)",
        )
        .bind(lobby_id)
        .bind(player_id)
        .bind(member_account)
        .bind(campaign_id)
        .bind(role)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    }
    sqlx::query(
        "insert into trnm_online_matches (
            match_id, campaign_id, host_player_id, host_account_id, join_code,
            phase, build_id, map_id, rules_version, match_mode
         ) values ($1, $2, $3, $4, $5, 'waiting', $6, $7, $8, 'ranked_pvp')",
    )
    .bind(match_id)
    .bind(&request.campaign_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&join_code)
    .bind(ONLINE_AUTHORITY_BUILD)
    .bind(&request.map_id)
    .bind(trnm_campaign_core::FIRST_CONTACT_RULES_VERSION)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    for (player_id, member_account, campaign_id, role) in [
        (
            request.player_id.as_str(),
            account_id,
            request.campaign_id.as_str(),
            "host",
        ),
        (
            opponent_player_id.as_str(),
            opponent_account_id,
            opponent_campaign_id.as_str(),
            "coop_guest",
        ),
    ] {
        sqlx::query(
            "insert into trnm_online_match_members (
                match_id, player_id, account_id, campaign_id, member_role
             ) values ($1, $2, $3, $4, $5)",
        )
        .bind(match_id)
        .bind(player_id)
        .bind(member_account)
        .bind(campaign_id)
        .bind(role)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    }
    sqlx::query(
        "update trnm_online_solo_queue set status = 'matched', matched_lobby_id = $2,
            match_id = $3, opponent_player_id = $4, updated_at = now()
         where ticket_id = $1",
    )
    .bind(
        candidate
            .try_get::<Uuid, _>("ticket_id")
            .map_err(internal_db)?,
    )
    .bind(lobby_id)
    .bind(match_id)
    .bind(&request.player_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query(
        "insert into trnm_online_solo_queue (
            ticket_id, player_id, account_id, campaign_id, map_id, status,
            rating_at_join, matched_lobby_id, match_id, opponent_player_id
         ) values ($1, $2, $3, $4, $5, 'matched', $6, $7, $8, $9)",
    )
    .bind(ticket_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.campaign_id)
    .bind(&request.map_id)
    .bind(rating)
    .bind(lobby_id)
    .bind(match_id)
    .bind(&opponent_player_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;

    let started = start_match(
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
    .await;
    if let Err(error) = started {
        let mut cleanup = state.pool.begin().await.map_err(internal_db)?;
        sqlx::query(
            "update trnm_online_solo_queue set status = 'cancelled',
                matched_lobby_id = null, match_id = null, opponent_player_id = null,
                updated_at = now() where match_id = $1",
        )
        .bind(match_id)
        .execute(&mut *cleanup)
        .await
        .map_err(internal_db)?;
        sqlx::query("delete from trnm_online_matches where match_id = $1")
            .bind(match_id)
            .execute(&mut *cleanup)
            .await
            .map_err(internal_db)?;
        sqlx::query("delete from trnm_online_lobbies where lobby_id = $1")
            .bind(lobby_id)
            .execute(&mut *cleanup)
            .await
            .map_err(internal_db)?;
        cleanup.commit().await.map_err(internal_db)?;
        return Err(error);
    }
    let mut allocated = state.pool.begin().await.map_err(internal_db)?;
    sqlx::query(
        "insert into trnm_online_matchmaking_allocations (
            allocation_id, lobby_id, match_id, queue_mode, member_count
         ) values ($1, $2, $3, 'ranked_pvp', 2)",
    )
    .bind(allocation_id)
    .bind(lobby_id)
    .bind(match_id)
    .execute(&mut *allocated)
    .await
    .map_err(internal_db)?;
    sqlx::query(
        "update trnm_online_lobbies set status = 'matched', match_id = $2,
            lobby_revision = lobby_revision + 1, updated_at = now() where lobby_id = $1",
    )
    .bind(lobby_id)
    .bind(match_id)
    .execute(&mut *allocated)
    .await
    .map_err(internal_db)?;
    allocated.commit().await.map_err(internal_db)?;
    Ok(Json(
        fetch_latest_ticket(&state.pool, &request.player_id).await?,
    ))
}

pub(super) async fn get_solo_queue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSoloQueueAccessRequest>,
) -> Result<Json<OnlineSoloQueueView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    Ok(Json(
        fetch_latest_ticket(&state.pool, &request.player_id).await?,
    ))
}

pub(super) async fn cancel_solo_queue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSoloQueueAccessRequest>,
) -> Result<Json<OnlineSoloQueueView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let changed = sqlx::query(
        "update trnm_online_solo_queue set status = 'cancelled', updated_at = now()
         where ticket_id = (
             select ticket_id from trnm_online_solo_queue
             where player_id = $1 and status = 'queued' order by created_at desc limit 1
         )",
    )
    .bind(&request.player_id)
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    if changed.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "player has no cancellable solo queue ticket",
            false,
        ));
    }
    Ok(Json(
        fetch_latest_ticket(&state.pool, &request.player_id).await?,
    ))
}

pub(super) async fn get_rating(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSoloQueueAccessRequest>,
) -> Result<Json<OnlineRatingView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    ensure_rating(&mut transaction, &request.player_id, account_id).await?;
    let row = sqlx::query(
        "select rating, wins, losses, provisional_matches from trnm_online_ratings
         where player_id = $1",
    )
    .bind(&request.player_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineRatingView {
        protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
        build_id: ONLINE_PRODUCT_BUILD.to_string(),
        player_id: request.player_id,
        rating: row.try_get("rating").map_err(internal_db)?,
        wins: row.try_get::<i32, _>("wins").map_err(internal_db)? as u32,
        losses: row.try_get::<i32, _>("losses").map_err(internal_db)? as u32,
        provisional_matches: row
            .try_get::<i32, _>("provisional_matches")
            .map_err(internal_db)? as u32,
    }))
}

async fn verify_social_target(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    let exists: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_campaigns where player_id = $1)",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(internal_db)?;
    if !exists {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "online player not found",
            false,
        ));
    }
    Ok(())
}

async fn lock_social_pair(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    left: &str,
    right: &str,
) -> Result<(), ApiError> {
    let pair = if left < right {
        format!("{left}:{right}")
    } else {
        format!("{right}:{left}")
    };
    sqlx::query("select pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(format!("trnm-social:{pair}"))
        .execute(&mut **transaction)
        .await
        .map_err(internal_db)?;
    Ok(())
}

async fn ensure_not_blocked(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    left: &str,
    right: &str,
) -> Result<(), ApiError> {
    let blocked: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_blocks
         where (blocker_player_id = $1 and blocked_player_id = $2)
            or (blocker_player_id = $2 and blocked_player_id = $1))",
    )
    .bind(left)
    .bind(right)
    .fetch_one(&mut **transaction)
    .await
    .map_err(internal_db)?;
    if blocked {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "social interaction is blocked",
            false,
        ));
    }
    Ok(())
}

pub(super) async fn request_friend(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineFriendRequest>,
) -> Result<Json<OnlineSocialView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    if request.player_id == request.target_player_id {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "cannot friend self",
            false,
        ));
    }
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    lock_social_pair(
        &mut transaction,
        &request.player_id,
        &request.target_player_id,
    )
    .await?;
    verify_social_target(&mut transaction, &request.target_player_id).await?;
    ensure_not_blocked(
        &mut transaction,
        &request.player_id,
        &request.target_player_id,
    )
    .await?;
    let exists: bool = sqlx::query_scalar(
        "select exists(select 1 from trnm_online_friendships
         where (requester_player_id = $1 and target_player_id = $2)
            or (requester_player_id = $2 and target_player_id = $1))",
    )
    .bind(&request.player_id)
    .bind(&request.target_player_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if exists {
        return Err(api_error(
            StatusCode::CONFLICT,
            "friend relationship already exists",
            false,
        ));
    }
    sqlx::query(
        "insert into trnm_online_friendships (requester_player_id, target_player_id)
         values ($1, $2)",
    )
    .bind(&request.player_id)
    .bind(&request.target_player_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_social(&state.pool, &request.player_id).await?))
}

pub(super) async fn resolve_friend(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineFriendResolveRequest>,
) -> Result<Json<OnlineSocialView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let status = if request.accept {
        "accepted"
    } else {
        "rejected"
    };
    let updated = sqlx::query(
        "update trnm_online_friendships set status = $3, updated_at = now()
         where requester_player_id = $1 and target_player_id = $2 and status = 'pending'",
    )
    .bind(&request.requester_player_id)
    .bind(&request.player_id)
    .bind(status)
    .execute(&state.pool)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "pending friend request not found",
            false,
        ));
    }
    Ok(Json(fetch_social(&state.pool, &request.player_id).await?))
}

pub(super) async fn set_block(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineBlockRequest>,
) -> Result<Json<OnlineSocialView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    if request.player_id == request.target_player_id {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "cannot block self",
            false,
        ));
    }
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    lock_social_pair(
        &mut transaction,
        &request.player_id,
        &request.target_player_id,
    )
    .await?;
    verify_social_target(&mut transaction, &request.target_player_id).await?;
    if request.blocked {
        sqlx::query(
            "insert into trnm_online_blocks (blocker_player_id, blocked_player_id)
             values ($1, $2) on conflict do nothing",
        )
        .bind(&request.player_id)
        .bind(&request.target_player_id)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
        sqlx::query(
            "update trnm_online_friendships set status = 'rejected', updated_at = now()
             where status in ('pending', 'accepted') and
               ((requester_player_id = $1 and target_player_id = $2)
                 or (requester_player_id = $2 and target_player_id = $1))",
        )
        .bind(&request.player_id)
        .bind(&request.target_player_id)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
        sqlx::query(
            "update trnm_online_lobby_invites set status = 'revoked'
             where status = 'pending' and
               ((inviter_player_id = $1 and target_player_id = $2)
                 or (inviter_player_id = $2 and target_player_id = $1))",
        )
        .bind(&request.player_id)
        .bind(&request.target_player_id)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    } else {
        sqlx::query(
            "delete from trnm_online_blocks where blocker_player_id = $1 and blocked_player_id = $2",
        )
        .bind(&request.player_id)
        .bind(&request.target_player_id)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    }
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_social(&state.pool, &request.player_id).await?))
}

pub(super) async fn get_social(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSocialAccessRequest>,
) -> Result<Json<OnlineSocialView>, ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    Ok(Json(fetch_social(&state.pool, &request.player_id).await?))
}

async fn fetch_social(pool: &PgPool, player_id: &str) -> Result<OnlineSocialView, ApiError> {
    let friendship_rows = sqlx::query(
        "select requester_player_id, target_player_id, status
         from trnm_online_friendships
         where requester_player_id = $1 or target_player_id = $1",
    )
    .bind(player_id)
    .fetch_all(pool)
    .await
    .map_err(internal_db)?;
    let mut friends = Vec::new();
    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();
    for row in friendship_rows {
        let requester: String = row.try_get("requester_player_id").map_err(internal_db)?;
        let target: String = row.try_get("target_player_id").map_err(internal_db)?;
        let status: String = row.try_get("status").map_err(internal_db)?;
        if status == "accepted" {
            friends.push(if requester == player_id {
                target
            } else {
                requester
            });
        } else if status == "pending" && target == player_id {
            incoming.push(requester);
        } else if status == "pending" && requester == player_id {
            outgoing.push(target);
        }
    }
    let blocked_players = sqlx::query_scalar::<_, String>(
        "select blocked_player_id from trnm_online_blocks
         where blocker_player_id = $1 order by blocked_player_id",
    )
    .bind(player_id)
    .fetch_all(pool)
    .await
    .map_err(internal_db)?;
    friends.sort();
    incoming.sort();
    outgoing.sort();
    Ok(OnlineSocialView {
        protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
        build_id: ONLINE_PRODUCT_BUILD.to_string(),
        player_id: player_id.to_string(),
        friends,
        incoming_requests: incoming,
        outgoing_requests: outgoing,
        blocked_players,
    })
}

fn report_view(row: &sqlx::postgres::PgRow) -> Result<OnlineReportView, ApiError> {
    Ok(OnlineReportView {
        report_id: row
            .try_get::<Uuid, _>("report_id")
            .map_err(internal_db)?
            .to_string(),
        reporter_player_id: row.try_get("reporter_player_id").map_err(internal_db)?,
        target_player_id: row.try_get("target_player_id").map_err(internal_db)?,
        match_id: row
            .try_get::<Uuid, _>("match_id")
            .map_err(internal_db)?
            .to_string(),
        category: row.try_get("category").map_err(internal_db)?,
        status: row.try_get("status").map_err(internal_db)?,
        resolution: row.try_get("resolution").map_err(internal_db)?,
    })
}

pub(super) async fn create_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineReportCreateRequest>,
) -> Result<(StatusCode, Json<OnlineReportView>), ApiError> {
    require_v2(&request.protocol_version, &request.build_id)?;
    if !matches!(
        request.category.as_str(),
        "cheating" | "harassment" | "griefing" | "name" | "other"
    ) || request.detail.chars().count() < 10
        || request.detail.chars().count() > 2000
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "report category or detail is invalid",
            false,
        ));
    }
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let match_id = Uuid::parse_str(&request.match_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "match_id must be a UUID", false))?;
    let valid_pair: bool = sqlx::query_scalar(
        "select exists(
            select 1 from trnm_online_match_members reporter
            join trnm_online_match_members target on target.match_id = reporter.match_id
            where reporter.match_id = $1 and reporter.player_id = $2 and reporter.account_id = $3
              and target.player_id = $4
         )",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(&request.target_player_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if !valid_pair || request.player_id == request.target_player_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "reporter and target must be distinct members of the match",
            false,
        ));
    }
    let report_id = Uuid::new_v4();
    let row = sqlx::query(
        "insert into trnm_online_reports (
            report_id, reporter_player_id, target_player_id, match_id, category, detail
         ) values ($1, $2, $3, $4, $5, $6)
         returning report_id, reporter_player_id, target_player_id, match_id,
                   category, status, resolution",
    )
    .bind(report_id)
    .bind(&request.player_id)
    .bind(&request.target_player_id)
    .bind(match_id)
    .bind(&request.category)
    .bind(&request.detail)
    .fetch_one(&state.pool)
    .await
    .map_err(|error| {
        if error
            .as_database_error()
            .is_some_and(|database| database.is_unique_violation())
        {
            return api_error(
                StatusCode::CONFLICT,
                "duplicate report for this match and category",
                false,
            );
        }
        internal_db(error)
    })?;
    Ok((StatusCode::CREATED, Json(report_view(&row)?)))
}

pub(super) async fn resolve_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineReportResolveRequest>,
) -> Result<Json<OnlineReportView>, ApiError> {
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
    if !matches!(
        request.decision.as_str(),
        "reviewed" | "actioned" | "dismissed"
    ) || request.resolution.trim().chars().count() < 10
        || request.resolution.chars().count() > 2000
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation decision or resolution is invalid",
            false,
        ));
    }
    let report_id = Uuid::parse_str(&request.report_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "report_id must be a UUID", false))?;
    let row = sqlx::query(
        "update trnm_online_reports set status = $2, resolution = $3, resolved_at = now()
         where report_id = $1 and status = 'open'
         returning report_id, reporter_player_id, target_player_id, match_id,
                   category, status, resolution",
    )
    .bind(report_id)
    .bind(&request.decision)
    .bind(request.resolution.trim())
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::CONFLICT, "report is not open", false))?;
    Ok(Json(report_view(&row)?))
}

pub(super) async fn apply_ranked_result(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    match_id: Uuid,
    outcome: BattleOutcome,
    result_hash: &str,
) -> Result<(), String> {
    let mode: String =
        sqlx::query_scalar("select match_mode from trnm_online_matches where match_id = $1")
            .bind(match_id)
            .fetch_one(&mut **transaction)
            .await
            .map_err(|error| error.to_string())?;
    if mode != "ranked_pvp" {
        return Ok(());
    }
    let members = sqlx::query(
        "select player_id, account_id, member_role from trnm_online_match_members
         where match_id = $1 order by case member_role when 'host' then 0 else 1 end",
    )
    .bind(match_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    if members.len() != 2 {
        return Err("ranked match does not contain exactly two players".to_string());
    }
    let host = members[0]
        .try_get::<String, _>("player_id")
        .map_err(|error| error.to_string())?;
    let guest = members[1]
        .try_get::<String, _>("player_id")
        .map_err(|error| error.to_string())?;
    for member in &members {
        sqlx::query(
            "insert into trnm_online_ratings (player_id, account_id) values ($1, $2)
             on conflict (player_id) do nothing",
        )
        .bind(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
        )
        .bind(
            member
                .try_get::<Uuid, _>("account_id")
                .map_err(|error| error.to_string())?,
        )
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    let ratings = sqlx::query(
        "select player_id, rating from trnm_online_ratings
         where player_id = any($1) order by player_id for update",
    )
    .bind(vec![host.clone(), guest.clone()])
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut before = BTreeMap::new();
    for row in ratings {
        before.insert(
            row.try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            row.try_get::<i32, _>("rating")
                .map_err(|error| error.to_string())?,
        );
    }
    let host_rating = *before
        .get(&host)
        .ok_or_else(|| "host rating is missing".to_string())?;
    let guest_rating = *before
        .get(&guest)
        .ok_or_else(|| "guest rating is missing".to_string())?;
    let host_won = outcome == BattleOutcome::Victory;
    let expected_host = 1.0 / (1.0 + 10_f64.powf((guest_rating - host_rating) as f64 / 400.0));
    let host_delta = (32.0 * (if host_won { 1.0 } else { 0.0 } - expected_host)).round() as i32;
    let guest_delta = -host_delta;
    for (player, opponent, won, rating_before, delta) in [
        (&host, &guest, host_won, host_rating, host_delta),
        (&guest, &host, !host_won, guest_rating, guest_delta),
    ] {
        let rating_after = (rating_before + delta).clamp(0, 5000);
        sqlx::query(
            "update trnm_online_ratings set rating = $2,
                wins = wins + case when $3 then 1 else 0 end,
                losses = losses + case when $3 then 0 else 1 end,
                provisional_matches = provisional_matches + 1, updated_at = now()
             where player_id = $1",
        )
        .bind(player)
        .bind(rating_after)
        .bind(won)
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
        sqlx::query(
            "insert into trnm_online_rating_events (
                event_id, match_id, player_id, opponent_player_id, result,
                rating_before, rating_after, rating_delta, result_hash
             ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(Uuid::new_v4())
        .bind(match_id)
        .bind(player)
        .bind(opponent)
        .bind(if won { "win" } else { "loss" })
        .bind(rating_before)
        .bind(rating_after)
        .bind(rating_after - rating_before)
        .bind(result_hash)
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}
