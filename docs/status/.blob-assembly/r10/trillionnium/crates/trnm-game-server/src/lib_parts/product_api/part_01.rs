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
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
