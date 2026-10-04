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
        let campaign_id: String = row.try_get("campaign_id").map_err(internal_db)?;
        ensure_campaign_progression_is_published(&state.pool, &campaign_id).await?;
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
                    false,
                );
            }
        }
        internal_db(error)
    })?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(fetch_match_view(&state.pool, match_id).await?))
}

fn start_match_retry_is_idempotent(
    phase: &str,
    durable_revision: u64,
    expected_revision: u64,
    assignment_matches_current_authority: bool,
) -> bool {
    phase == "running"
        && expected_revision.checked_add(1) == Some(durable_revision)
        && assignment_matches_current_authority
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
                match_mode, assigned_instance_id, assigned_instance_epoch,
                assigned_physical_host_id
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
    let revision = u64::try_from(
        match_row
            .try_get::<i64, _>("match_revision")
            .map_err(internal_db)?,
    )
    .map_err(|_| {
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "match revision is negative",
            false,
        )
    })?;
    if phase != "waiting" {
        let assignment_matches_current_authority = match_row
            .try_get::<Option<String>, _>("assigned_instance_id")
            .map_err(internal_db)?
            .as_deref()
            == Some(state.instance_id.as_str())
            && match_row
                .try_get::<Option<i64>, _>("assigned_instance_epoch")
                .map_err(internal_db)?
                == Some(state.instance_epoch)
            && match_row
                .try_get::<Option<String>, _>("assigned_physical_host_id")
                .map_err(internal_db)?
                .as_deref()
                == Some(state.physical_host_id.as_str());
        if start_match_retry_is_idempotent(
            &phase,
            revision,
            request.expected_match_revision,
            assignment_matches_current_authority,
        ) {
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
            return Ok(Json(fetch_match_view(&state.pool, match_id).await?));
        }
        return Err(api_error(
            StatusCode::CONFLICT,
            "match is not waiting",
            false,
        ));
    }
    if revision != request.expected_match_revision {
        return Err(conflict("match revision changed", revision));
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
            "Online Authority requires exactly host + one co-op guest",
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
        ensure_campaign_progression_is_published(&mut *transaction, &campaign_id).await?;
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
    lock_current_fleet_epoch_for_commit(&mut transaction, &state, false)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?;
    // Reserve local authority before the running phase becomes visible. The
    // reservation is a bounded readiness owner and is released automatically
    // if the database commit or the request future is cancelled.
    let (actor_initialization, actor_reservation) = reserve_starting_match_actor(&state, match_id)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?;
    transaction.commit().await.map_err(internal_db)?;
    initialize_reserved_match_actor(&state, match_id, actor_initialization, actor_reservation)
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
    let handler_started = Instant::now();
    reject_command_during_drain(&state)?;
    validate_client_contract(&request.protocol_version, &request.build_id)
        .map_err(|error| api_error(StatusCode::UPGRADE_REQUIRED, error, false))?;
    validate_command_id(&request.command_id)?;
    let admission = ActorCommandAdmission {
        bucket_key: distributed_admission_bucket_key(
            session_header(&headers)?,
            "POST",
            &format!("/v1/online/matches/{match_id}"),
        ),
        limit: i64::from(state.rate_limit_per_minute.saturating_mul(20)),
    };
    let identity_started = Instant::now();
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let identity_ms = u64::try_from(identity_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let request_hash = hash_json(&request)?;
    let actor_lookup_started = Instant::now();
    let existing_actor = state
        .match_actors
        .read()
        .await
        .actors
        .get(&match_id)
        .cloned();
    let actor = if let Some(actor) = existing_actor {
        Some(actor)
    } else {
        // Completed actors intentionally leave the registry. Resolve an exact
        // retry from the prepared receipt query before invoking recovery; the
        // latter owns several fenced terminal transactions and must not sit on
        // the latency path of an already-durable receipt.
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
        ensure_match_actor(&state, match_id)
            .await
            .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?
    };
    let actor_lookup_ms =
        u64::try_from(actor_lookup_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let Some(actor) = actor else {
        return Err(api_error(
            StatusCode::CONFLICT,
            "match is not running",
            false,
        ));
    };
    // Running-match membership, account binding, role and unit controls are
    // immutable. The actor loaded them under the same fenced transaction as
    // its simulation and cursors, so the hot command path does not spend a
    // separate database RTT rediscovering immutable authority.
    let member = actor.members.get(&request.player_id).ok_or_else(|| {
        api_error(
            StatusCode::FORBIDDEN,
            "identity is not a match member",
            false,
        )
    })?;
    if member.account_id != account_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "identity is not bound to this match account",
            false,
        ));
    }
    let published_cursor = actor.publication_acked.borrow().clone();
    let duplicate_lookup_needed = if request.protocol_version == ONLINE_AUTHORITY_PROTOCOL {
        request.input_sequence.is_some_and(|input_sequence| {
            published_cursor
                .next_input_sequences
                .get(&request.player_id)
                .is_some_and(|current| input_sequence < *current)
        })
    } else {
        request.sequence < published_cursor.next_sequence
    };
    if duplicate_lookup_needed {
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
    }
    let requested_subjects = request
        .order
        .subject_actor_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if requested_subjects.is_empty() || !requested_subjects.is_subset(&member.controlled_unit_ids) {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "command subjects are outside this member's authoritative control set",
            false,
        ));
    }
    let controlled = member.controlled_unit_ids.clone();
    let member_role = member.member_role.clone();
    reject_command_during_drain(&state)?;
    let (response_tx, response_rx) = oneshot::channel();
    let actor_wait_started = Instant::now();
    tokio::time::timeout(
        Duration::from_secs(5),
        actor.commands.send(ActorCommandEnvelope {
            request,
            request_hash,
            admission,
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
    let actor_wait_ms = u64::try_from(actor_wait_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let handler_ms = u64::try_from(handler_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    if handler_ms > 200 {
        tracing::warn!(
            %match_id,
            handler_ms,
            identity_ms,
            actor_lookup_ms,
            actor_wait_ms,
            "slow online command handler"
        );
    }
    Ok(Json(receipt))
}

async fn fetch_duplicate_command_receipt(
    state: &AppState,
    match_id: Uuid,
    command_id: &str,
    player_id: &str,
    request_hash: &str,
) -> Result<Option<OnlineCommandReceipt>, ApiError> {
    let Some(row) = sqlx::query::query(DUPLICATE_COMMAND_RECEIPT_SQL)
        .bind(match_id)
        .bind(command_id)
        .bind(state.physical_host_id.as_str())
        .fetch_optional(&state.pool)
        .await
        .map_err(internal_db)?
    else {
        return Ok(None);
    };
    if row
        .try_get::<Option<String>, _>("assigned_physical_host_id")
        .map_err(internal_db)?
        .as_deref()
        != Some(state.physical_host_id.as_str())
    {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "duplicate command authority physical host is fenced",
            true,
        ));
    }
    let stored_player_id: String = row.try_get("player_id").map_err(internal_db)?;
    let stored_request_hash: Option<String> = row.try_get("request_hash").map_err(internal_db)?;
    if stored_player_id != player_id || stored_request_hash.as_deref() != Some(request_hash) {
        return Err(conflict(
            "command_id was already used with a different authenticated request",
            row.try_get::<i64, _>("current_match_revision")
                .map_err(internal_db)? as u64,
        ));
    }
    let client_observed_tick = row
        .try_get::<Option<i64>, _>("client_observed_tick")
        .map_err(internal_db)?;
    let sequence = row.try_get::<i64, _>("sequence").map_err(internal_db)? as u64;
    let accepted_revision = row
        .try_get::<i64, _>("accepted_match_revision")
        .map_err(internal_db)? as u64;
    let durable_next_sequence = row
        .try_get::<i64, _>("current_next_sequence")
        .map_err(internal_db)? as u64;
    let checkpoint_sequence = row
        .try_get::<i64, _>("checkpoint_sequence")
        .map_err(internal_db)? as u64;
    let phase: String = row.try_get("phase").map_err(internal_db)?;
    let terminal_publication_acked: bool = row
        .try_get("terminal_publication_acked")
        .map_err(internal_db)?;
    let input_sequence = row
        .try_get::<i64, _>("input_sequence")
        .map_err(internal_db)? as u64;
    let durable_member_input_sequence = row
        .try_get::<i64, _>("current_member_input_sequence")
        .map_err(internal_db)? as u64;
    let actor_cursor = state
        .match_actors
        .read()
        .await
        .actors
        .get(&match_id)
        .map(|actor| actor.publication_acked.borrow().clone());
    if !command_receipt_publication_is_acked(CommandReceiptPublicationBarrier {
        sequence,
        accepted_revision,
        durable_next_sequence,
        checkpoint_sequence,
        player_id: &stored_player_id,
        input_sequence,
        durable_member_input_sequence,
        phase: &phase,
        terminal_publication_acked,
        actor_cursor: actor_cursor.as_ref(),
    }) {
        return Ok(None);
    }
    Ok(Some(OnlineCommandReceipt {
        protocol_version: receipt_protocol_for_observed_tick(client_observed_tick).to_string(),
        match_id: match_id.to_string(),
        player_id: stored_player_id,
        command_id: command_id.to_string(),
        sequence,
        input_sequence,
        duplicate: true,
        accepted_tick: row.try_get::<i64, _>("target_tick").map_err(internal_db)? as u64,
        client_observed_tick: client_observed_tick.map(|value| value as u64),
        match_revision: accepted_revision,
        snapshot_hash: row.try_get("accepted_snapshot_hash").map_err(internal_db)?,
    }))
}

fn command_receipt_publication_is_acked(barrier: CommandReceiptPublicationBarrier<'_>) -> bool {
    let command_next_sequence = barrier.sequence.saturating_add(1);
    let command_next_input_sequence = barrier.input_sequence.saturating_add(1);
    if command_next_sequence > barrier.durable_next_sequence
        || command_next_input_sequence > barrier.durable_member_input_sequence
    {
        return false;
    }
    if barrier.phase == "complete" {
        return barrier.terminal_publication_acked
            && barrier.checkpoint_sequence >= command_next_sequence
            && barrier.checkpoint_sequence == barrier.durable_next_sequence;
    }
    if barrier.phase != "running" {
        return false;
    }
    barrier.actor_cursor.is_some_and(|cursor| {
        cursor.phase == OnlineMatchPhase::Running
            && cursor.receipts_replayable
            && cursor.next_sequence >= command_next_sequence
            && cursor.next_sequence <= barrier.durable_next_sequence
            && cursor.match_revision >= barrier.accepted_revision
            && cursor
                .next_input_sequences
                .get(barrier.player_id)
                .is_some_and(|cursor| *cursor >= command_next_input_sequence)
    })
}

fn prepare_actor_command(
    loaded: &LoadedMatchActor,
    request: OnlineCommandSubmitRequest,
    request_hash: String,
    admission: ActorCommandAdmission,
    controlled_unit_ids: &BTreeSet<String>,
    member_role: &str,
) -> Result<PreparedActorCommand, ApiError> {
    let realtime_v3 = request.protocol_version == ONLINE_AUTHORITY_PROTOCOL;
    if (!realtime_v3 && request.expected_match_revision != loaded.match_revision)
        || (realtime_v3 && request.expected_match_revision > loaded.match_revision)
    {
        return Err(conflict("match revision changed", loaded.match_revision));
    }
    if !realtime_v3 && request.sequence != loaded.next_sequence {
        return Err(conflict(
            format!("expected command sequence {}", loaded.next_sequence),
            loaded.match_revision,
        ));
    }
    let member_input_sequence = loaded
        .next_input_sequences
        .get(&request.player_id)
        .copied()
        .ok_or_else(|| {
            api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "match actor member input cursor is unavailable",
                true,
            )
        })?;
    let input_sequence = if realtime_v3 {
        request.input_sequence.ok_or_else(|| {
            api_error(
                StatusCode::BAD_REQUEST,
                "Authority v3 requires input_sequence",
                false,
            )
        })?
    } else {
        member_input_sequence
    };
    if input_sequence != member_input_sequence {
        return Err(conflict(
            format!("expected player input sequence {member_input_sequence}"),
            loaded.match_revision,
        ));
    }
    let client_observed_tick = if realtime_v3 {
        let observed = request.client_observed_tick.ok_or_else(|| {
            api_error(
                StatusCode::BAD_REQUEST,
                "Authority v3 requires client_observed_tick",
                false,
            )
        })?;
        if observed > loaded.simulation.tick {
            return Err(conflict(
                "client observed a tick beyond server authority",
                loaded.match_revision,
            ));
        }
        Some(observed)
    } else {
        if request.target_tick < loaded.simulation.tick
            || request.target_tick > loaded.simulation.tick.saturating_add(200)
        {
            return Err(conflict(
                "target_tick is outside the authoritative window",
                loaded.match_revision,
            ));
        }
        None
    };
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
    let effective_tick = if realtime_v3 {
        candidate.tick
    } else {
        request.target_tick
    };
    let order_frame = if realtime_v3 {
        let last_member_order_frame =
            if loaded.match_mode == "ranked_pvp" && member_role == "coop_guest" {
                candidate.enemy_last_order_frame
            } else {
                candidate.last_order_frame
            };
        effective_tick.max(u64::from(last_member_order_frame.unwrap_or_default()))
    } else {
        effective_tick
    };
    let order = prepare_and_apply_actor_order(
        &mut candidate,
        &loaded.match_mode,
        member_role,
        order_frame,
        request.order.clone(),
    )?;
    let snapshot_hash = candidate
        .snapshot_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    let accepted_revision = loaded.match_revision.saturating_add(1);
    let post_simulation = serde_json::to_value(&candidate).map_err(internal_serialization)?;
    Ok(PreparedActorCommand {
        candidate_simulation: candidate,
        persistence: ActorCommandPersistence {
            request,
            request_hash,
            admission,
            order,
            post_simulation,
            snapshot_hash,
            input_sequence,
            client_observed_tick,
            effective_tick,
            base_next_sequence: loaded.next_sequence,
            base_match_revision: loaded.match_revision,
            accepted_revision,
        },
    })
}

