async fn persist_actor_command(
    state: &AppState,
    match_id: Uuid,
    job: ActorCommandPersistence,
) -> Result<OnlineCommandReceipt, ApiError> {
    if !state.database_host_authority.is_healthy() {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "PostgreSQL host authority is fail-closed",
            true,
        ));
    }
    let identity = &state.database_host_authority.identity;
    let order_json = serde_json::to_value(&job.order).map_err(internal_serialization)?;
    // This is intentionally one PostgreSQL statement. The migration function
    // revalidates the exact K1 leader, holds the pool session's lifetime K2
    // barrier, locks fleet -> match -> member, and either advances the command
    // event plus both cursors atomically or changes nothing.
    let database_started = Instant::now();
    let row = sqlx::query::query(ONLINE_COMMAND_COMMIT_V2_SQL)
        .bind(match_id)
        .bind(&job.request.command_id)
        .bind(&job.request.player_id)
        .bind(job.input_sequence as i64)
        .bind(&job.request_hash)
        .bind(job.effective_tick as i64)
        .bind(job.client_observed_tick.map(|value| value as i64))
        .bind(order_json)
        .bind(&job.snapshot_hash)
        .bind(job.accepted_revision as i64)
        .bind(&job.post_simulation)
        .bind(job.base_next_sequence as i64)
        .bind(job.base_match_revision as i64)
        .bind(state.instance_id.as_str())
        .bind(state.instance_epoch)
        .bind(state.physical_host_id.as_str())
        .bind(identity.owner_nonce)
        .bind(&identity.application_name)
        .bind(identity.backend_pid)
        .bind(identity.backend_started_at)
        .bind(&identity.database_system_identifier)
        .bind(identity.database_timeline_id)
        .bind(identity.database_postmaster_started_at)
        .bind(identity.leader_lock_key)
        .bind(identity.barrier_lock_key)
        .bind(&job.admission.bucket_key)
        .bind(job.admission.limit)
        .fetch_one(&state.pool)
        .await;
    let database_ms = u64::try_from(database_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    if database_ms > 150 {
        tracing::warn!(%match_id, database_ms, "slow atomic command database commit");
    }
    let row = row.map_err(internal_db)?;

    let outcome: String = row.try_get("result_outcome").map_err(internal_db)?;
    let durable_revision = row
        .try_get::<Option<i64>, _>("result_durable_match_revision")
        .map_err(internal_db)?
        .unwrap_or(job.base_match_revision as i64)
        .max(0) as u64;
    if outcome == "match_not_found" {
        return Err(api_error(StatusCode::NOT_FOUND, "match not found", false));
    }
    if outcome == "command_conflict" {
        return Err(conflict(
            "command_id was already used with a different authenticated request",
            durable_revision,
        ));
    }
    if outcome == "match_not_running" {
        return Err(conflict("match is not running", durable_revision));
    }
    if outcome == "rate_limited" {
        return Err(api_error(
            StatusCode::TOO_MANY_REQUESTS,
            "production request rate limit exceeded",
            true,
        ));
    }
    if outcome == "duplicate_publication_pending" {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "duplicate command is awaiting its durable publication barrier",
            true,
        ));
    }
    if outcome != "inserted" && outcome != "duplicate" {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            format!("atomic command commit failed closed: {outcome}"),
            true,
        ));
    }

    let stored_player_id: String = row.try_get("result_player_id").map_err(internal_db)?;
    let stored_request_hash: String = row.try_get("result_request_hash").map_err(internal_db)?;
    if stored_player_id != job.request.player_id || stored_request_hash != job.request_hash {
        return Err(conflict(
            "atomic command result did not bind the authenticated request",
            durable_revision,
        ));
    }
    let sequence = row
        .try_get::<i64, _>("result_sequence")
        .map_err(internal_db)? as u64;
    let input_sequence = row
        .try_get::<i64, _>("result_input_sequence")
        .map_err(internal_db)? as u64;
    let accepted_revision = row
        .try_get::<i64, _>("result_accepted_match_revision")
        .map_err(internal_db)? as u64;
    let accepted_snapshot_hash: String = row
        .try_get("result_accepted_snapshot_hash")
        .map_err(internal_db)?;
    let accepted_tick = row
        .try_get::<i64, _>("result_target_tick")
        .map_err(internal_db)? as u64;
    let stored_client_observed_tick = row
        .try_get::<Option<i64>, _>("result_client_observed_tick")
        .map_err(internal_db)?;
    let duplicate = outcome == "duplicate";
    if duplicate {
        let durable_next_sequence = row
            .try_get::<i64, _>("result_durable_next_sequence")
            .map_err(internal_db)? as u64;
        let checkpoint_sequence = row
            .try_get::<i64, _>("result_checkpoint_sequence")
            .map_err(internal_db)? as u64;
        let durable_member_input_sequence = row
            .try_get::<i64, _>("result_durable_member_input_sequence")
            .map_err(internal_db)? as u64;
        let phase: String = row.try_get("result_phase").map_err(internal_db)?;
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
            terminal_publication_acked: false,
            actor_cursor: actor_cursor.as_ref(),
        }) {
            return Err(api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "duplicate command is awaiting its durable publication barrier",
                true,
            ));
        }
    }
    Ok(OnlineCommandReceipt {
        protocol_version: receipt_protocol_for_observed_tick(stored_client_observed_tick)
            .to_string(),
        match_id: match_id.to_string(),
        player_id: stored_player_id,
        command_id: job.request.command_id,
        sequence,
        input_sequence,
        duplicate,
        accepted_tick,
        client_observed_tick: stored_client_observed_tick.map(|value| value as u64),
        match_revision: accepted_revision,
        snapshot_hash: accepted_snapshot_hash,
    })
}

async fn run_actor_command_persistence_worker(
    state: AppState,
    match_id: Uuid,
    mut jobs: mpsc::Receiver<ActorCommandPersistence>,
    completions: mpsc::Sender<ActorPersistenceCompletion>,
) {
    while let Some(job) = jobs.recv().await {
        let command_id = job.request.command_id.clone();
        let result = tokio::time::timeout(
            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
            persist_actor_command(&state, match_id, job),
        )
        .await
        .unwrap_or_else(|_| {
            Err(api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "command persistence exceeded its hard timeout",
                true,
            ))
        })
        .map_err(|error| {
            if error.status == StatusCode::INTERNAL_SERVER_ERROR {
                api_error(StatusCode::SERVICE_UNAVAILABLE, error.body.error, true)
            } else {
                error
            }
        });
        if !matches!(
            tokio::time::timeout(
                MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                completions.send(ActorPersistenceCompletion { command_id, result }),
            )
            .await,
            Ok(Ok(()))
        ) {
            break;
        }
    }
}

fn cached_actor_command_result(
    cache: &BTreeMap<String, ActorReceiptCacheEntry>,
    command_id: &str,
    player_id: &str,
    request_hash: &str,
    authoritative_revision: u64,
) -> Option<Result<OnlineCommandReceipt, ApiError>> {
    cache.get(command_id).map(|entry| {
        if entry.player_id != player_id || entry.request_hash != request_hash {
            Err(conflict(
                "command_id was already used with a different authenticated request",
                authoritative_revision,
            ))
        } else {
            let mut receipt = entry.receipt.clone();
            receipt.duplicate = true;
            Ok(receipt)
        }
    })
}

fn cache_actor_receipt(
    cache: &mut BTreeMap<String, ActorReceiptCacheEntry>,
    cache_order: &mut VecDeque<String>,
    request_hash: String,
    receipt: &OnlineCommandReceipt,
) {
    let command_id = receipt.command_id.clone();
    if !cache.contains_key(&command_id) {
        cache_order.push_back(command_id.clone());
    }
    cache.insert(
        command_id,
        ActorReceiptCacheEntry {
            player_id: receipt.player_id.clone(),
            request_hash,
            receipt: receipt.clone(),
        },
    );
    while cache_order.len() > MATCH_ACTOR_RECEIPT_CACHE {
        if let Some(expired) = cache_order.pop_front() {
            cache.remove(&expired);
        }
    }
}

fn committed_receipt_matches_pending(
    receipt: &OnlineCommandReceipt,
    command_id: &str,
    player_id: &str,
    sequence: u64,
    input_sequence: u64,
    accepted_revision: u64,
    accepted_snapshot_hash: &str,
) -> bool {
    !receipt.duplicate
        && receipt.command_id == command_id
        && receipt.player_id == player_id
        && receipt.sequence == sequence
        && receipt.input_sequence == input_sequence
        && receipt.match_revision == accepted_revision
        && receipt.snapshot_hash == accepted_snapshot_hash
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
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let durable_next_sequence = match_row
        .try_get::<i64, _>("next_sequence")
        .map_err(internal_db)? as u64;
    let durable_match_revision = match_row
        .try_get::<i64, _>("match_revision")
        .map_err(internal_db)? as u64;
    let durable_phase = match match_row
        .try_get::<String, _>("phase")
        .map_err(internal_db)?
        .as_str()
    {
        "waiting" => OnlineMatchPhase::Waiting,
        "running" => OnlineMatchPhase::Running,
        "complete" => OnlineMatchPhase::Complete,
        _ => OnlineMatchPhase::FailedClosed,
    };
    let durable_result_hash: Option<String> =
        match_row.try_get("result_hash").map_err(internal_db)?;
    let durable_settlement_state: String =
        match_row.try_get("settlement_state").map_err(internal_db)?;
    let durable_member_rows = sqlx::query::query(
        "select m.player_id, m.account_id, m.campaign_id, m.member_role,
                m.controlled_unit_ids, m.next_input_sequence,
                c.campaign_revision, c.campaign_json
         from trnm_online_match_members m
         join trnm_online_campaigns c on c.campaign_id = m.campaign_id
         where m.match_id = $1 order by m.member_role desc",
    )
    .bind(match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let mut identity_is_member = false;
    for row in &durable_member_rows {
        let member_player_id = row.try_get::<String, _>("player_id").map_err(internal_db)?;
        let member_account_id = row.try_get::<Uuid, _>("account_id").map_err(internal_db)?;
        identity_is_member |=
            member_player_id == request.player_id && member_account_id == account_id;
    }
    if !identity_is_member {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "identity is not a match member",
            false,
        ));
    }
    let mut durable_next_input_sequences = BTreeMap::new();
    for row in &durable_member_rows {
        durable_next_input_sequences.insert(
            row.try_get::<String, _>("player_id").map_err(internal_db)?,
            row.try_get::<i64, _>("next_input_sequence")
                .map_err(internal_db)? as u64,
        );
    }
    let exact_terminal_marker = if durable_phase == OnlineMatchPhase::Complete {
        exact_terminal_publication_marker_exists(&mut *transaction, match_id)
            .await
            .map_err(|error| {
                api_error(
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("terminal reconnect marker lookup failed: {error}"),
                    true,
                )
            })?
    } else {
        false
    };
    if !terminal_read_surface_is_releasable(durable_phase, exact_terminal_marker) {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "terminal reconnect is waiting for its exact publication marker",
            true,
        ));
    }

    let (published, next_sequence, match_revision, next_input_sequences, snapshot_hash, snapshot) =
        if let Some(actor) = actor {
            let published = actor.published.borrow().clone();
            let acknowledged = actor.publication_acked.borrow().clone();
            if !publication_tuple_matches(&published, &acknowledged)
                || !published_authority_matches_durable(
                    &published,
                    durable_phase,
                    durable_result_hash.as_deref(),
                    &durable_settlement_state,
                )
                || acknowledged.next_sequence > durable_next_sequence
                || acknowledged.match_revision > durable_match_revision
                || acknowledged.next_input_sequences.len() != durable_next_input_sequences.len()
                || acknowledged
                    .next_input_sequences
                    .iter()
                    .any(|(player_id, cursor)| {
                        durable_next_input_sequences
                            .get(player_id)
                            .is_none_or(|durable| cursor > durable)
                    })
            {
                return Err(api_error(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "reconnect crossed an unacknowledged publication transition",
                    true,
                ));
            }
            let snapshot = serde_json::to_value(published.simulation.as_ref())
                .map_err(internal_serialization)?;
            (
                Some(published.clone()),
                acknowledged.next_sequence,
                acknowledged.match_revision,
                acknowledged.next_input_sequences,
                acknowledged.snapshot_hash,
                snapshot,
            )
        } else {
            if durable_phase == OnlineMatchPhase::Running {
                return Err(api_error(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "running match has no acknowledged publication owner",
                    true,
                ));
            }
            let snapshot = match_row
                .try_get::<Option<Value>, _>("simulation_json")
                .map_err(internal_db)?
                .unwrap_or(Value::Null);
            let snapshot_hash: String = match_row.try_get("snapshot_hash").map_err(internal_db)?;
            if durable_phase == OnlineMatchPhase::Complete {
                let checkpoint_sequence = match_row
                    .try_get::<i64, _>("checkpoint_sequence")
                    .map_err(internal_db)? as u64;
                if checkpoint_sequence != durable_next_sequence || snapshot.is_null() {
                    return Err(api_error(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "terminal reconnect is missing its exact durable checkpoint",
                        true,
                    ));
                }
                let simulation = serde_json::from_value::<MissionSimV1>(snapshot.clone())
                    .map_err(internal_serialization)?;
                let computed_hash = simulation.snapshot_hash().map_err(|error| {
                    api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false)
                })?;
                let authoritative_tick = match_row
                    .try_get::<i64, _>("authoritative_tick")
                    .map_err(internal_db)? as u64;
                if !simulation.terminal()
                    || simulation.tick != authoritative_tick
                    || computed_hash != snapshot_hash
                {
                    return Err(api_error(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "terminal reconnect checkpoint failed hash validation",
                        true,
                    ));
                }
            }
            (
                None,
                durable_next_sequence,
                durable_match_revision,
                durable_next_input_sequences.clone(),
                snapshot_hash,
                snapshot,
            )
        };
    let replay_from_sequence = if request.protocol_version == ONLINE_AUTHORITY_PROTOCOL {
        request.next_receipt_sequence.ok_or_else(|| {
            api_error(
                StatusCode::BAD_REQUEST,
                "Authority v3 reconnect requires next_receipt_sequence",
                false,
            )
        })?
    } else {
        request
            .next_receipt_sequence
            .unwrap_or(request.last_acknowledged_sequence)
    };
    if replay_from_sequence > next_sequence {
        return Err(conflict(
            "client acknowledged a command sequence beyond server authority",
            match_revision,
        ));
    }
    let command_rows = sqlx::query::query(
        "select sequence, input_sequence, command_id, player_id, target_tick,
                client_observed_tick,
                accepted_match_revision, accepted_snapshot_hash
         from trnm_online_commands
         where match_id = $1 and sequence >= $2 and sequence < $3
         order by sequence asc limit 257",
    )
    .bind(match_id)
    .bind(replay_from_sequence as i64)
    .bind(next_sequence as i64)
    .fetch_all(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let replay_truncated = command_rows.len() > 256;
    let replayed_commands = command_rows
        .into_iter()
        .take(256)
        .map(|row| {
            let client_observed_tick = row
                .try_get::<Option<i64>, _>("client_observed_tick")
                .map_err(internal_db)?;
            Ok(OnlineCommandReceipt {
                protocol_version: receipt_protocol_for_observed_tick(client_observed_tick)
                    .to_string(),
                match_id: match_id.to_string(),
                player_id: row.try_get("player_id").map_err(internal_db)?,
                command_id: row.try_get("command_id").map_err(internal_db)?,
                sequence: row.try_get::<i64, _>("sequence").map_err(internal_db)? as u64,
                input_sequence: row
                    .try_get::<i64, _>("input_sequence")
                    .map_err(internal_db)? as u64,
                duplicate: true,
                accepted_tick: row.try_get::<i64, _>("target_tick").map_err(internal_db)? as u64,
                client_observed_tick: client_observed_tick.map(|value| value as u64),
                match_revision: row
                    .try_get::<i64, _>("accepted_match_revision")
                    .map_err(internal_db)? as u64,
                snapshot_hash: row.try_get("accepted_snapshot_hash").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let mut expected_replay_sequence = replay_from_sequence;
    for receipt in &replayed_commands {
        if receipt.sequence != expected_replay_sequence {
            return Err(api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "authoritative command replay contains a sequence gap",
                true,
            ));
        }
        expected_replay_sequence = expected_replay_sequence.saturating_add(1);
    }
    let next_receipt_sequence = replayed_commands
        .last()
        .map(|receipt| receipt.sequence.saturating_add(1))
        .unwrap_or(replay_from_sequence);
    if !replay_truncated && next_receipt_sequence != next_sequence {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "authoritative command replay does not reach the locked authority cursor",
            true,
        ));
    }
    // Release the match authority snapshot before updating reconnect telemetry.
    // Keeping the member row lock inside this transaction lets concurrent
    // reconnects queue while all of them still hold a match FOR SHARE lock,
    // which blocks the actor's canonical match FOR UPDATE command commit.
    transaction.commit().await.map_err(internal_db)?;
    let reconnect_count: i64 = sqlx::query_scalar::query_scalar(
        "update trnm_online_match_members set reconnect_count = reconnect_count + 1,
            last_acknowledged_sequence = greatest(last_acknowledged_sequence, $4),
            last_snapshot_hash = $5, last_seen_at = now()
         where match_id = $1 and player_id = $2 and account_id = $3
         returning reconnect_count",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(next_receipt_sequence as i64)
    .bind(&snapshot_hash)
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
    let reconnect_count = reconnect_count as u64;
    let mut view = match_view_from_rows(&match_row, &durable_member_rows)?;
    if !snapshot.is_null() {
        let simulation = serde_json::from_value::<MissionSimV1>(snapshot.clone())
            .map_err(internal_serialization)?;
        let computed_hash = simulation.snapshot_hash().map_err(|error| {
            api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false)
        })?;
        if computed_hash != snapshot_hash {
            return Err(api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "reconnect snapshot hash does not match the locked authority state",
                true,
            ));
        }
        view.authoritative_tick = simulation.tick;
    }
    view.next_sequence = next_sequence;
    view.match_revision = match_revision;
    view.snapshot_hash = snapshot_hash.clone();
    for member in &mut view.members {
        if let Some(cursor) = next_input_sequences.get(&member.player_id) {
            member.next_input_sequence = *cursor;
        }
    }
    apply_published_authority_view(
        &mut view,
        published
            .as_ref()
            .map_or(durable_phase, |state| state.phase),
        published
            .as_ref()
            .map_or(durable_result_hash, |state| state.result_hash.clone()),
        published
            .as_ref()
            .map_or(durable_settlement_state, |state| {
                state.settlement_state.clone()
            }),
    );
    Ok(Json(OnlineReconnectResponse {
        view,
        snapshot,
        replayed_commands,
        reconnect_count,
        full_snapshot_required: request.last_snapshot_hash != snapshot_hash || replay_truncated,
        replay_from_sequence,
        next_receipt_sequence,
        replay_truncated,
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
            .map_err(|error| error.body.error.clone())?;
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

