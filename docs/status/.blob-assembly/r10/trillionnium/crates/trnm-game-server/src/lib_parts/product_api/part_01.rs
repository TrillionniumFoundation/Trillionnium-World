__TRNM_SLOT_0__
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
    for member in &members {
        ensure_campaign_progression_is_published(
            &mut *transaction,
            &member
                .try_get::<String, _>("campaign_id")
                .map_err(internal_db)?,
        )
        .await?;
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
    ensure_campaign_progression_is_published(&state.pool, &request.campaign_id).await?;

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
    ensure_campaign_progression_is_published(&mut *transaction, &request.campaign_id).await?;
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
__TRNM_SLOT_2__
__TRNM_SLOT_3__
