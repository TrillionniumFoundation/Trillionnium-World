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
    match initialized {
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
