pub fn production_authority_tick_interval() -> Duration {
            Ok(TerminalJournalReconciliationOutcome::FailedClosed) => {
                state
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
                    .await
                    .insert(match_id);
                report.failed_closed = report.failed_closed.saturating_add(1);
                tracing::info!(%match_id, "failed-closed published-tick high-water retained as rollback witness");
            }
            Ok(TerminalJournalReconciliationOutcome::NotTerminal) => {
                state
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                report.pending = report.pending.saturating_add(1);
            }
            Err(error) => {
                state
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                report.failed = report.failed.saturating_add(1);
                tracing::error!(%match_id, %error, "terminal publication orphan quarantined");
            }
        }
    }
    let terminal_ack_gaps =
        local_terminal_publication_ack_gaps(&state.pool, state.physical_host_id.as_str()).await?;
    let recorded_match_ids = state
        .published_tick_journal
        .recorded_match_ids()?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let bounded_transitions = {
        let registry = state.match_actors.read().await;
        bounded_match_actor_transition_ids(&registry, Instant::now())
    };
    let terminal_ack_gaps = terminal_ack_gaps
        .into_iter()
        .filter(|match_id| !bounded_transitions.contains(match_id))
        .collect::<Vec<_>>();
    report.terminal_ack_gaps = terminal_ack_gaps.len() as u64;
    let unrecoverable =
        terminal_ack_gaps_without_high_water(&terminal_ack_gaps, &recorded_match_ids);
    report.quarantined_without_high_water = unrecoverable.len() as u64;
    for match_id in &terminal_ack_gaps {
        if recorded_match_ids.contains(match_id) {
            tracing::warn!(%match_id, "terminal publication ACK remains pending behind host journal recovery");
        }
    }
    for match_id in unrecoverable {
        tracing::error!(%match_id, "terminal publication gap has no recoverable host high-water and remains quarantined");
    }
    Ok(report)
}

pub async fn advance_running_matches(state: &AppState, limit: i64) -> Result<u64, String> {
    if *state.draining.borrow() || *state.shutdown.borrow() {
        return Ok(0);
    }
    let candidates = sqlx::query::query(
        "select m.match_id, m.assigned_instance_id, m.assigned_instance_epoch,
                m.assigned_physical_host_id
         from trnm_online_matches m
         left join trnm_online_fleet_instances f
           on f.instance_id = m.assigned_instance_id
          and f.instance_epoch = m.assigned_instance_epoch
          and f.physical_host_id = m.assigned_physical_host_id
         where m.phase = 'running'
           and (m.assigned_instance_id is null or m.assigned_physical_host_id = $4)
           and (
            (m.assigned_instance_id = $2 and m.assigned_instance_epoch = $3
             and m.assigned_physical_host_id = $4)
            or m.assigned_instance_id is null
            or f.status is null or f.status = 'offline'
            or f.lease_expires_at <= now()
         ) order by m.updated_at limit $1",
    )
    .bind(limit)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(state.physical_host_id.as_str())
    .fetch_all(&state.pool)
    .await
    .map_err(|error| error.to_string())?;
    let mut active = 0u64;
    for candidate in candidates {
        let match_id = candidate
            .try_get::<Uuid, _>("match_id")
            .map_err(|error| error.to_string())?;
        let assigned_instance_id = candidate
            .try_get::<Option<String>, _>("assigned_instance_id")
            .map_err(|error| error.to_string())?;
        let assigned_instance_epoch = candidate
            .try_get::<i64, _>("assigned_instance_epoch")
            .map_err(|error| error.to_string())?;
        let assigned_physical_host_id = candidate
            .try_get::<Option<String>, _>("assigned_physical_host_id")
            .map_err(|error| error.to_string())?;
        if !reconciliation_candidate_is_local(
            assigned_instance_id.as_deref(),
            assigned_physical_host_id.as_deref(),
            state.physical_host_id.as_str(),
        ) {
            return Err(
                "match reconciliation query returned a remote physical-host assignment".to_string(),
            );
        }
        match ensure_match_actor(state, match_id).await {
            Ok(Some(_)) => active = active.saturating_add(1),
            Ok(None) => {}
            Err(error) => {
                tracing::error!(%match_id, %error, "running match quarantined after actor recovery failure");
                // Rotate a bad match behind healthy candidates. One corrupt
                // recovery record must not permanently consume the first slot
                // of every bounded reconciliation batch.
                if let Err(update_error) = sqlx::query::query(
                    "update trnm_online_matches set updated_at = now()
                     where match_id = $1 and phase = 'running'
                       and assigned_instance_id is not distinct from $2
                       and assigned_instance_epoch = $3
                       and assigned_physical_host_id is not distinct from $4
                       and (assigned_instance_id is null or assigned_physical_host_id = $5)",
                )
                .bind(match_id)
                .bind(&assigned_instance_id)
                .bind(assigned_instance_epoch)
                .bind(&assigned_physical_host_id)
                .bind(state.physical_host_id.as_str())
                .execute(&state.pool)
                .await
                {
                    tracing::error!(%match_id, %update_error, "failed to rotate quarantined match");
                }
            }
        }
    }
    Ok(active)
}

async fn ensure_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<MatchActorHandle>, String> {
    loop {
        if *state.draining.borrow() || *state.shutdown.borrow() {
            return Err(
                "game server is draining and cannot initialize match authority".to_string(),
            );
        }
        let decision = {
            let mut registry = state.match_actors.write().await;
            reserve_match_actor_initialization(&mut registry, match_id, state.capacity)?
        };
        match decision {
            MatchActorEnsureDecision::Existing(handle) => return Ok(Some(handle)),
            MatchActorEnsureDecision::Wait(mut ready) => {
                tokio::time::timeout(MATCH_ACTOR_INITIALIZATION_TIMEOUT, ready.changed())
                    .await
                    .map_err(|_| {
                        "timed out waiting for this match actor initialization".to_string()
                    })?
                    .map_err(|_| "match actor initializer stopped without a result".to_string())?;
            }
            MatchActorEnsureDecision::Initialize(initialization) => {
                let reservation = MatchActorInitializationReservation::new(
                    state.match_actors.clone(),
                    match_id,
                    &initialization,
                );
                return initialize_reserved_match_actor(
                    state,
                    match_id,
                    initialization,
                    reservation,
                )
                .await;
            }
        }
    }
}

async fn reserve_starting_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<
    (
        MatchActorInitialization,
        MatchActorInitializationReservation,
    ),
    String,
> {
    let initialization = {
        let mut registry = state.match_actors.write().await;
        match reserve_match_actor_initialization(&mut registry, match_id, state.capacity)? {
            MatchActorEnsureDecision::Initialize(initialization) => initialization,
            MatchActorEnsureDecision::Existing(_) => {
                return Err("waiting match already has a local actor".to_string());
            }
            MatchActorEnsureDecision::Wait(_) => {
                return Err("waiting match actor initialization is already reserved".to_string());
            }
        }
    };
    let reservation = MatchActorInitializationReservation::new(
        state.match_actors.clone(),
        match_id,
        &initialization,
    );
    Ok((initialization, reservation))
}

async fn initialize_reserved_match_actor(
    state: &AppState,
    match_id: Uuid,
    initialization: MatchActorInitialization,
    mut reservation: MatchActorInitializationReservation,
) -> Result<Option<MatchActorHandle>, String> {
    let initialized = tokio::time::timeout(
        MATCH_ACTOR_INITIALIZATION_TIMEOUT,
        initialize_match_actor(state, match_id),
    )
    .await
    .map_err(|_| "match actor initialization exceeded its hard timeout".to_string())
    .and_then(|result| result);
    let mut registry = state.match_actors.write().await;
    let owns_reservation = registry
        .initializing
        .get(&match_id)
        .is_some_and(|current| current.token == initialization.token);
    if !owns_reservation {
        reservation.disarm();
        initialization.ready.send_replace(true);
        return Err("match actor initialization reservation was replaced".to_string());
    }
    if !match_actor_install_is_allowed(*state.draining.borrow(), *state.shutdown.borrow()) {
        registry.initializing.remove(&match_id);
        reservation.disarm();
        initialization.ready.send_replace(true);
        return Err("game server began draining before match actor installation".to_string());
    }
    registry.initializing.remove(&match_id);
    reservation.disarm();
    match initialized {
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
