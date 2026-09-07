__TRNM_SLOT_0__
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
    let Some(actor) = handle else {
        return Ok(None);
    };
    let published = actor.published.borrow().clone();
    let acknowledged = actor.publication_acked.borrow().clone();
    if !publication_tuple_matches(&published, &acknowledged) {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "match authority is crossing its durable publication barrier",
            true,
        ));
    }
    Ok(Some(published))
}

fn publication_tuple_matches(
    published: &PublishedMatchState,
    acknowledged: &ActorPublicationCursor,
) -> bool {
    acknowledged.tick == published.simulation.tick
        && acknowledged.next_sequence == published.next_sequence
        && acknowledged.match_revision == published.match_revision
        && acknowledged.next_input_sequences == *published.next_input_sequences
        && acknowledged.phase == published.phase
        && acknowledged.receipts_replayable
        && acknowledged.snapshot_hash == *published.snapshot_hash
}

fn apply_published_actor_view(view: &mut OnlineMatchView, published: &PublishedMatchState) {
    view.authoritative_tick = published.simulation.tick;
    view.next_sequence = published.next_sequence;
    view.match_revision = published.match_revision;
    view.snapshot_hash = published.snapshot_hash.as_ref().clone();
    for member in &mut view.members {
        if let Some(cursor) = published.next_input_sequences.get(&member.player_id) {
            member.next_input_sequence = *cursor;
        }
    }
    apply_published_authority_view(
        view,
        published.phase,
        published.result_hash.clone(),
        published.settlement_state.clone(),
    );
}

fn apply_published_authority_view(
    view: &mut OnlineMatchView,
    phase: OnlineMatchPhase,
    result_hash: Option<String>,
    settlement_state: String,
) {
    let effective_settlement_state =
        effective_published_settlement_state(&settlement_state, &view.settlement_state);
    view.phase = phase;
    view.result_hash = result_hash;
    view.settlement_state = effective_settlement_state;
}

fn effective_published_settlement_state(
    published_settlement_state: &str,
    durable_settlement_state: &str,
) -> String {
    if published_settlement_state == "pending" && durable_settlement_state == "settled" {
        durable_settlement_state.to_string()
    } else {
        published_settlement_state.to_string()
    }
}

fn published_authority_matches_view(
    published: &PublishedMatchState,
    view: &OnlineMatchView,
) -> bool {
    authority_metadata_matches(
        published.phase,
        published.result_hash.as_deref(),
        &published.settlement_state,
        view.phase,
        view.result_hash.as_deref(),
        &view.settlement_state,
    )
}

fn published_cursor_is_within_durable_view(
    published: &PublishedMatchState,
    view: &OnlineMatchView,
) -> bool {
    published.next_sequence <= view.next_sequence
        && published.match_revision <= view.match_revision
        && published.next_input_sequences.len() == view.members.len()
        && published
            .next_input_sequences
            .iter()
            .all(|(player_id, cursor)| {
                view.members
                    .iter()
                    .find(|member| member.player_id == *player_id)
                    .is_some_and(|member| *cursor <= member.next_input_sequence)
            })
}

fn published_authority_matches_durable(
    published: &PublishedMatchState,
    phase: OnlineMatchPhase,
    result_hash: Option<&str>,
    settlement_state: &str,
) -> bool {
    authority_metadata_matches(
        published.phase,
        published.result_hash.as_deref(),
        &published.settlement_state,
        phase,
        result_hash,
        settlement_state,
    )
}

fn authority_metadata_matches(
    published_phase: OnlineMatchPhase,
    published_result_hash: Option<&str>,
    published_settlement_state: &str,
    durable_phase: OnlineMatchPhase,
    durable_result_hash: Option<&str>,
    durable_settlement_state: &str,
) -> bool {
    published_phase == durable_phase
        && published_result_hash == durable_result_hash
        && (published_settlement_state == durable_settlement_state
            || (published_settlement_state == "pending" && durable_settlement_state == "settled"))
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
    let snapshot = if let Some(published) = published_actor_state(&state, match_id).await? {
        if !published_authority_matches_view(&published, &view) {
            view = fetch_match_view(&state.pool, match_id).await?;
        }
        if !published_authority_matches_view(&published, &view) {
            return Err(api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "match phase is crossing its durable terminal publication barrier",
                true,
            ));
        }
        if !published_cursor_is_within_durable_view(&published, &view) {
            return Err(api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "snapshot crossed a durable database cursor rollback",
                true,
            ));
        }
        apply_published_actor_view(&mut view, &published);
        serde_json::to_value(published.simulation.as_ref()).map_err(internal_serialization)?
    } else {
        let snapshot = load_durable_nonrunning_snapshot(&state.pool, match_id).await?;
        view = fetch_match_view(&state.pool, match_id).await?;
        snapshot
    };
    Ok(Json(OnlineSnapshotResponse { view, snapshot }))
}

async fn load_durable_nonrunning_snapshot(
    pool: &PgPool,
    match_id: Uuid,
) -> Result<Value, ApiError> {
    let row = sqlx::query::query(
        "select phase, simulation_json, snapshot_hash, authoritative_tick,
                next_sequence, checkpoint_sequence
         from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let phase: String = row.try_get("phase").map_err(internal_db)?;
    if phase == "running" {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "running match has no acknowledged publication owner",
            true,
        ));
    }
    let simulation_json = row
        .try_get::<Option<Value>, _>("simulation_json")
        .map_err(internal_db)?
        .unwrap_or(Value::Null);
    if phase != "complete" {
        return Ok(simulation_json);
    }
    let exact_marker_exists = exact_terminal_publication_marker_exists(pool, match_id)
        .await
        .map_err(|error| {
            api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                format!("terminal publication marker lookup failed: {error}"),
                true,
            )
        })?;
    if !terminal_read_surface_is_releasable(OnlineMatchPhase::Complete, exact_marker_exists) {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "terminal snapshot is waiting for its exact publication marker",
            true,
        ));
    }
    let next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(internal_db)? as u64;
    let checkpoint_sequence = row
        .try_get::<i64, _>("checkpoint_sequence")
        .map_err(internal_db)? as u64;
    if checkpoint_sequence != next_sequence || simulation_json.is_null() {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "terminal match is missing its exact durable checkpoint",
            true,
        ));
    }
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_json.clone())
        .map_err(internal_serialization)?;
    let expected_tick = row
        .try_get::<i64, _>("authoritative_tick")
        .map_err(internal_db)? as u64;
    let expected_hash: String = row.try_get("snapshot_hash").map_err(internal_db)?;
    let computed_hash = simulation
        .snapshot_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    if !simulation.terminal() || simulation.tick != expected_tick || computed_hash != expected_hash
    {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "terminal durable checkpoint failed tick/hash validation",
            true,
        ));
    }
    Ok(simulation_json)
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
    let actor = ensure_match_actor(&state, match_id)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let match_row = sqlx::query::query(
        "select match_id, join_code, phase, build_id, map_id, match_mode, rules_version,
                seed_hash, snapshot_hash, authoritative_tick, next_sequence, match_revision,
                result_hash, settlement_state, simulation_json, checkpoint_sequence
         from trnm_online_matches where match_id = $1 for share",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
__TRNM_SLOT_2__
__TRNM_SLOT_3__
