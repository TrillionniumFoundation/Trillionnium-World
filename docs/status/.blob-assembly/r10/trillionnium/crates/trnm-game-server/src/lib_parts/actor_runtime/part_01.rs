__TRNM_SLOT_0__
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
        Ok(Some(initialized)) => {
            let handle = initialized.handle.clone();
            registry.actors.insert(match_id, handle.clone());
            initialization.ready.send_replace(true);
            drop(registry);
            let actor_state = state.clone();
            tokio::spawn(async move {
                run_match_actor(actor_state, match_id, initialized).await;
            });
            Ok(Some(handle))
        }
        Ok(None) => {
            initialization.ready.send_replace(true);
            Ok(None)
        }
        Err(error) => {
            initialization.ready.send_replace(true);
            Err(error)
        }
    }
}

fn match_actor_install_is_allowed(draining: bool, shutdown: bool) -> bool {
    !draining && !shutdown
}

fn reserve_match_actor_initialization(
    registry: &mut MatchActorRegistry,
    match_id: Uuid,
    capacity: i32,
) -> Result<MatchActorEnsureDecision, String> {
    if let Some(handle) = registry.actors.get(&match_id).cloned() {
        return Ok(MatchActorEnsureDecision::Existing(handle));
    }
    if let Some(initialization) = registry.initializing.get(&match_id) {
        return Ok(MatchActorEnsureDecision::Wait(
            initialization.ready.subscribe(),
        ));
    }
    let reserved = registry.actors.len().saturating_add(
        registry
            .initializing
            .values()
            .filter(|initialization| initialization.kind == MatchActorInitializationKind::Actor)
            .count(),
    );
    if reserved >= usize::try_from(capacity).unwrap_or(usize::MAX) {
        return Err(format!(
            "local match actor capacity {capacity} is exhausted"
        ));
    }
    let (ready, _) = watch::channel(false);
    let initialization = MatchActorInitialization {
        token: Uuid::new_v4(),
        ready,
        kind: MatchActorInitializationKind::Actor,
        started_at: Instant::now(),
    };
    registry
        .initializing
        .insert(match_id, initialization.clone());
    Ok(MatchActorEnsureDecision::Initialize(initialization))
}

fn reserve_terminal_orphan_recovery(
    registry: &mut MatchActorRegistry,
    match_id: Uuid,
) -> Option<MatchActorInitialization> {
    if registry.actors.contains_key(&match_id) || registry.initializing.contains_key(&match_id) {
        return None;
    }
    let (ready, _) = watch::channel(false);
    let initialization = MatchActorInitialization {
        token: Uuid::new_v4(),
        ready,
        kind: MatchActorInitializationKind::TerminalRecovery,
        started_at: Instant::now(),
    };
    registry
        .initializing
        .insert(match_id, initialization.clone());
    Some(initialization)
}

async fn initialize_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<InitializedMatchActor>, String> {
    let actor_id = Uuid::new_v4();
    let Some(loaded) = load_match_actor(state, match_id).await? else {
        return Ok(None);
    };
    let initial_hash = loaded
        .simulation
        .snapshot_hash()
        .map_err(|error| error.to_string())?;
    if !match_actor_assignment_is_current(state, match_id).await? {
        return Ok(None);
    }
    if loaded.simulation.terminal() {
        adopt_actor_high_water(state, match_id, actor_id, &loaded).await?;
        let terminal_job = MatchCheckpointJob {
            simulation: loaded.simulation.clone(),
            snapshot_hash: initial_hash.clone(),
            next_sequence: loaded.next_sequence,
            match_revision: loaded.match_revision,
            terminal: true,
            completion: None,
        };
        persist_terminal_actor_checkpoint(state, match_id, &terminal_job).await?;
        let (_result, result_hash) =
            derive_terminal_result(&loaded.simulation, &loaded.match_mode)?;
        let terminal_evidence = TerminalPublicationEvidence {
            authoritative_tick: loaded.simulation.tick,
            next_sequence: loaded.next_sequence,
            match_revision: loaded.match_revision,
            next_input_sequences: loaded.next_input_sequences.clone(),
            snapshot_hash: initial_hash.clone(),
            phase: OnlineMatchPhase::Complete,
            result_hash: Some(result_hash),
            settlement_state: "staged".to_string(),
        };
        let terminal_record =
            state
                .published_tick_journal
                .new_record(PublishedTickRecordInput {
                    instance_id: state.instance_id.as_str().to_string(),
                    match_id,
                    actor_generation: actor_id,
                    actor_epoch: state.instance_epoch,
                    tick: loaded.simulation.tick,
                    next_sequence: loaded.next_sequence,
                    match_revision: loaded.match_revision,
                    next_input_sequences: loaded.next_input_sequences.clone(),
                    phase: "complete".to_string(),
                    receipts_replayable: true,
                    snapshot_hash: initial_hash,
                })?;
        record_published_tick_with_timeout(
            &state.published_tick_journal,
            terminal_record,
            loaded.next_sequence,
            loaded.match_revision,
            loaded.next_input_sequences.clone(),
        )
        .await?;
        persist_terminal_publication_ack(state, match_id, actor_id, &loaded, &terminal_evidence)
            .await?;
        return Ok(None);
    }
    record_initial_actor_publication(state, match_id, actor_id, &loaded, &initial_hash).await?;
    if !match_actor_assignment_is_current(state, match_id).await? {
        return Ok(None);
    }
    let published_at = Instant::now();
    let initial = PublishedMatchState {
        simulation: Arc::new(loaded.simulation.clone()),
        snapshot_hash: Arc::new(initial_hash.clone()),
        next_sequence: loaded.next_sequence,
        match_revision: loaded.match_revision,
        next_input_sequences: Arc::new(loaded.next_input_sequences.clone()),
        phase: OnlineMatchPhase::Running,
        result_hash: None,
        settlement_state: "not_ready".to_string(),
        state_sequence: 0,
        published_at,
    };
    let (published_tx, published_rx) = watch::channel(initial);
    let (publication_acked_tx, publication_acked_rx) = watch::channel(ActorPublicationCursor {
        tick: loaded.simulation.tick,
        next_sequence: loaded.next_sequence,
        match_revision: loaded.match_revision,
        next_input_sequences: loaded.next_input_sequences.clone(),
        phase: OnlineMatchPhase::Running,
        receipts_replayable: true,
        snapshot_hash: initial_hash,
    });
    let (command_tx, command_rx) = mpsc::channel(MATCH_ACTOR_COMMAND_QUEUE);
    let clock = Arc::new(AuthorityClockTelemetry::default());
    let (lifecycle_tx, lifecycle_rx) = watch::channel(MatchActorLifecycle::Warming {
        started_at: Instant::now(),
    });
    let handle = MatchActorHandle {
        actor_id,
        commands: command_tx,
        members: loaded.members.clone(),
        published: published_rx,
        publication_acked: publication_acked_rx,
        clock,
        lifecycle: lifecycle_rx,
    };
    Ok(Some(InitializedMatchActor {
        actor_id,
        handle,
        loaded,
        commands: command_rx,
        published: published_tx,
        publication_acked: publication_acked_tx,
        lifecycle: lifecycle_tx,
    }))
}

async fn match_actor_assignment_is_current(
    state: &AppState,
    match_id: Uuid,
) -> Result<bool, String> {
    sqlx::query_scalar::query_scalar(
        "select exists(
            select 1 from trnm_online_matches m
            join trnm_online_fleet_instances f
              on f.instance_id = m.assigned_instance_id
             and f.instance_epoch = m.assigned_instance_epoch
            where m.match_id = $1 and m.phase = 'running'
              and m.assigned_instance_id = $2
              and m.assigned_instance_epoch = $3
              and m.assigned_physical_host_id = $4
              and f.physical_host_id = $4
              and f.status in ('active', 'draining')
              and f.lease_expires_at > now()
        )",
    )
    .bind(match_id)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(state.physical_host_id.as_str())
    .fetch_one(&state.pool)
    .await
    .map_err(|error| error.to_string())
}

async fn load_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<LoadedMatchActor>, String> {
    let high_water = state.published_tick_journal.high_water(match_id)?;
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    lock_current_fleet_epoch(&mut transaction, state, true).await?;
    let Some(row) = sqlx::query::query(
        "select campaign_id, phase, simulation_json, snapshot_hash, authoritative_tick, match_mode,
                    next_sequence, match_revision, checkpoint_sequence,
                    assigned_instance_id, assigned_region, assigned_instance_epoch,
                    assigned_physical_host_id,
                    terminal_stage_simulation_json, terminal_stage_result_json,
                    terminal_stage_result_hash, terminal_stage_snapshot_hash,
                    terminal_stage_authoritative_tick, terminal_stage_next_sequence,
__TRNM_SLOT_2__
__TRNM_SLOT_3__
