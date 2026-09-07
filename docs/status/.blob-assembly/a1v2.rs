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
__TRNM_CHUNK_2__
             from trnm_online_matches
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
__TRNM_CHUNK_3__
        let high_water = high_water.as_ref().ok_or_else(|| {
            "private terminal stage has no host-journal publication witness".to_string()
        })?;
        if high_water.phase != "complete"
            || high_water.instance_id != *state.instance_id
            || high_water.actor_epoch != state.instance_epoch
            || high_water.physical_host_id != *state.physical_host_id
            || high_water.tick != staged.authoritative_tick
__TRNM_CHUNK_4__
                "{label} realtime accepted/observed/order ticks do not match its post-command state"
            ));
        }
    } else if target_tick < simulation.tick
        || target_tick > simulation.tick.saturating_add(200)
        || order_tick != target_tick
    {
        return Err(format!(
__TRNM_CHUNK_5__
