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
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
