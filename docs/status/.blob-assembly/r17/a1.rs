pub fn production_authority_tick_interval() -> Duration {
    assert_eq!(1_000 % TICKS_PER_SECOND, 0);
    Duration::from_millis(1_000 / TICKS_PER_SECOND)
}

pub fn resolve_authority_tick_interval(
    requested_ms: Option<u64>,
    allow_accelerated_test_clock: bool,
) -> Result<Duration, String> {
    let production = production_authority_tick_interval();
    let requested = Duration::from_millis(
        requested_ms.unwrap_or_else(|| u64::try_from(production.as_millis()).unwrap_or(100)),
    );
    if requested.is_zero() || requested > Duration::from_secs(1) {
        return Err("TRNM_GAME_SERVER_TICK_MS must be between 1 and 1000".to_string());
    }
    if requested != production && !allow_accelerated_test_clock {
        return Err(format!(
            "production authority is fixed at {}ms ({}Hz); non-real-time clocks require TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1",
            production.as_millis(),
            TICKS_PER_SECOND
        ));
    }
    Ok(requested)
}

pub async fn run_authority_loop(state: AppState, tick_interval: Duration) {
    let initial_database_fence = tokio::time::timeout(
        DATABASE_HOST_FENCE_MONITOR_TIMEOUT,
        state.verify_database_host_authority_session(),
    )
    .await;
    if !matches!(initial_database_fence, Ok(Ok(()))) {
        tracing::error!("initial PostgreSQL host-authority verification failed closed");
        state.fail_database_host_authority();
        return;
    }

    let database_fence_state = state.clone();
    let database_fence_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(DATABASE_HOST_FENCE_MONITOR_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        interval.tick().await;
        loop {
            interval.tick().await;
            let verification = tokio::time::timeout(
                DATABASE_HOST_FENCE_MONITOR_TIMEOUT,
                database_fence_state.verify_database_host_authority_session(),
            )
            .await;
            match verification {
                Ok(Ok(())) => {}
                Ok(Err(error)) => {
                    tracing::error!(%error, "PostgreSQL host-authority monitor failed closed");
                    database_fence_state.fail_database_host_authority();
                    return;
                }
                Err(_) => {
                    tracing::error!("PostgreSQL host-authority monitor exceeded its hard timeout");
                    database_fence_state.fail_database_host_authority();
                    return;
                }
            }
        }
    });

    if let Err(error) = operations_v1::heartbeat_fleet(&state).await {
        tracing::error!(%error, "initial online fleet heartbeat failed closed");
        state.fail_database_host_authority();
        database_fence_task.abort();
        return;
    }

    let heartbeat_state = state.clone();
    let heartbeat_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        interval.tick().await;
        loop {
            interval.tick().await;
            if let Err(error) = operations_v1::heartbeat_fleet(&heartbeat_state).await {
                tracing::error!(%error, "online fleet heartbeat failed closed");
                heartbeat_state.fail_database_host_authority();
                return;
            }
        }
    });

    let initial_recovery = tokio::time::timeout(INITIAL_AUTHORITY_RECOVERY_TIMEOUT, async {
        reconcile_terminal_publication_orphans(&state).await?;
        advance_running_matches(&state, i64::from(state.capacity)).await?;
        Ok::<(), String>(())
    })
    .await;
    let initial_recovery_error = match initial_recovery {
        Ok(Ok(())) => None,
        Ok(Err(error)) => Some(error),
        Err(_) => Some(format!(
            "initial authority recovery exceeded its {} second total budget",
            INITIAL_AUTHORITY_RECOVERY_TIMEOUT.as_secs()
        )),
    };
    if let Some(error) = initial_recovery_error {
        tracing::error!(%error, "initial online authority recovery failed closed");
        heartbeat_task.abort();
        database_fence_task.abort();
        state.begin_draining().await;
        state.shutdown.send_replace(true);
        state.fatal_shutdown.send_replace(true);
        return;
    }

    let maintenance_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = production_v1::run_production_maintenance(&maintenance_state).await
            {
                tracing::error!(%error, "online production maintenance failed closed");
            }
        }
    });

    let reconciliation_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = reconcile_terminal_publication_orphans(&reconciliation_state).await
            {
                tracing::error!(%error, "terminal publication orphan recovery failed closed");
            }
            if let Err(error) = advance_running_matches(
                &reconciliation_state,
                i64::from(reconciliation_state.capacity),
            )
            .await
            {
                tracing::error!(%error, "online match actor reconciliation failed closed");
            }
        }
    });

    // This clock does no database or simulation work. Match actors own their
    // independent 10 Hz loops, while heartbeat, maintenance, settlement and
    // discovery run in separate tasks and cannot stall the realtime cadence.
    let started_at = Instant::now();
    state.authority_clock.reset(started_at);
    let mut interval = tokio::time::interval_at(started_at + tick_interval, tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        let scheduled_at = interval.tick().await;
        state
            .authority_clock
            .record_wake(scheduled_at, Instant::now(), tick_interval);
    }
}

#[derive(Default)]
struct TerminalOrphanReconciliationReport {
    scanned: u64,
    recovered: u64,
    failed_closed: u64,
    pending: u64,
    failed: u64,
    terminal_ack_gaps: u64,
    quarantined_without_high_water: u64,
}

async fn reconcile_terminal_publication_orphans(
    state: &AppState,
) -> Result<TerminalOrphanReconciliationReport, String> {
    let high_waters = state
        .published_tick_journal
        .recorded_match_ids()?
        .into_iter()
        .map(|match_id| state.published_tick_journal.high_water(match_id))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let mut attempts = futures_util::stream::iter(high_waters)
        .map(|high_water| async move {
            let initialization = {
                let mut registry = state.match_actors.write().await;
                reserve_terminal_orphan_recovery(&mut registry, high_water.match_id)
            };
            let Some(initialization) = initialization else {
                return (high_water.match_id, None);
            };
            let mut reservation = MatchActorInitializationReservation::new(
                state.match_actors.clone(),
                high_water.match_id,
                &initialization,
            );
            let authority = PostgresTerminalOrphanAuthority {
                pool: &state.pool,
                journal: &state.published_tick_journal,
                runtime_state: Some(state),
                database_host_authority: &state.database_host_authority,
            };
            let result = reconcile_terminal_journal_record(
                &authority,
                &state.published_tick_journal,
                &high_water,
            )
            .await;
            let mut registry = state.match_actors.write().await;
            let owns_reservation = registry
                .initializing
                .get(&high_water.match_id)
                .is_some_and(|current| current.token == initialization.token);
            if owns_reservation {
                registry.initializing.remove(&high_water.match_id);
            }
            reservation.disarm();
            initialization.ready.send_replace(true);
            let result = if owns_reservation {
                result
            } else {
                Err("terminal recovery reservation was replaced".to_string())
            };
            (high_water.match_id, Some(result))
        })
        .buffer_unordered(TERMINAL_ORPHAN_RECONCILIATION_CONCURRENCY);

    let mut report = TerminalOrphanReconciliationReport::default();
    while let Some((match_id, result)) = attempts.next().await {
        let Some(result) = result else {
            continue;
        };
        report.scanned = report.scanned.saturating_add(1);
        match result {
            Ok(TerminalJournalReconciliationOutcome::Recovered) => {
                state
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .insert(match_id);
                state
                    .failed_closed_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                report.recovered = report.recovered.saturating_add(1);
                tracing::info!(%match_id, "terminal publication high-water revalidated in-process");
            }
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
                    terminal_stage_match_revision
             from trnm_online_matches
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
