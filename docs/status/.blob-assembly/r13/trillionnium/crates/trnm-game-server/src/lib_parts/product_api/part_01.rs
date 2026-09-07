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
