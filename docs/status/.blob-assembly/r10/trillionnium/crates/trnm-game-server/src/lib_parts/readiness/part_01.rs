async fn health() -> &'static str {
    "trnm-game-server ok"
}

async fn readiness(State(state): State<AppState>) -> Response {
    let database_host_authority_observed_at = Instant::now();
    let database_host_authority_healthy = state.database_host_authority_is_healthy();
    let database_host_authority_fresh = state
        .database_host_authority
        .is_fresh(database_host_authority_observed_at);
    let database_host_authority_last_success_age_ms = state
        .database_host_authority
        .last_success_age(database_host_authority_observed_at)
        .map(|age| u64::try_from(age.as_millis()).unwrap_or(u64::MAX));
    let database_host_authority_freshness_limit_ms =
        u64::try_from(DATABASE_HOST_FENCE_FRESHNESS.as_millis()).unwrap_or(u64::MAX);
    // Readiness is a hot operational surface. Under WAN-like database RTT,
    // serial probes used to accumulate more than five seconds of round trips
    // and could consume the command pool while reporting the service dead.
    // One prepared aggregate query owns all independent database counters;
    // the external dependencies and the exact latest cold-witness check run
    // concurrently, keeping the endpoint both fail-closed and non-disruptive.
    let (
        database_summary,
        cex_readiness,
        signer_readiness,
        signer_attestation,
        latest_cold_witness_sentinel,
    ) = tokio::join!(
        load_readiness_database_summary(
            &state.readiness_pool,
            state.instance_id.as_str(),
            state.instance_epoch,
            state.physical_host_id.as_str(),
        ),
        state.cex.readiness(),
        state.cex.signer_readiness(),
        state.cex.signer_attestation(),
        latest_cold_witness_sentinel_is_healthy(&state),
    );
    let database_summary_query_healthy = database_summary.is_ok();
    let database_summary = database_summary.unwrap_or_else(|error| {
        tracing::warn!(%error, "readiness database summary failed closed");
        ReadinessDatabaseSummary::blocked()
    });
    let postgres = database_summary_query_healthy && database_summary.postgres_healthy;
    let cex = cex_readiness.is_ok();
    let signer = signer_readiness.ok();
    let signer_registry_verified = signer_attestation.is_ok();
    let fleet_epoch_current =
        database_summary_query_healthy && database_summary.fleet_epoch_current;
    let healthy_fleet_instances = database_summary.healthy_fleet_instances;
    let active_matches_query_healthy = database_summary_query_healthy;
    let active_matches = database_summary.active_matches;
    let terminal_ack_gap_scan_saturated =
        terminal_ack_gap_scan_is_saturated(database_summary.terminal_ack_gap_count);
    let terminal_ack_gap_query_healthy =
        database_summary_query_healthy && !terminal_ack_gap_scan_saturated;
    let terminal_ack_gap_count = if terminal_ack_gap_query_healthy {
        database_summary.terminal_ack_gap_count
    } else {
        usize::MAX
    };
    let historical_projection_quarantine_query_healthy = database_summary_query_healthy;
    let historical_projection_quarantine = database_summary.historical_projection;
    let actor_observed_at = Instant::now();
    let (
        actor_states,
        actor_initializations,
        mut actor_tracked_match_ids,
        bounded_terminal_transition_match_ids,
    ) = {
        let registry = state.match_actors.read().await;
        let actor_states = registry
            .actors
            .iter()
            .map(|(match_id, actor)| {
                (
                    *match_id,
                    actor.published.borrow().clone(),
                    actor.clock.snapshot(actor_observed_at, state.tick_interval),
                    *actor.lifecycle.borrow(),
                )
            })
            .collect::<Vec<_>>();
        let actor_initializations = registry
            .initializing
            .iter()
            .map(|(match_id, initialization)| {
                (
                    *match_id,
                    initialization.kind,
                    actor_observed_at.saturating_duration_since(initialization.started_at),
                )
            })
            .collect::<Vec<_>>();
        let actor_tracked_match_ids =
            actor_tracked_published_tick_match_ids(&registry, actor_observed_at);
        let bounded_terminal_transition_match_ids =
            bounded_match_actor_transition_ids(&registry, actor_observed_at);
        (
            actor_states,
            actor_initializations,
            actor_tracked_match_ids,
            bounded_terminal_transition_match_ids,
        )
    };
    let initializing_match_actors = actor_initializations
        .iter()
        .filter(|(_, kind, _)| *kind == MatchActorInitializationKind::Actor)
        .count();
    let terminal_recovery_initializations = actor_initializations
        .iter()
        .filter(|(_, kind, _)| *kind == MatchActorInitializationKind::TerminalRecovery)
        .count();
    let max_match_actor_initialization_age_ms = actor_initializations
        .iter()
        .filter(|(_, kind, _)| *kind == MatchActorInitializationKind::Actor)
        .map(|(_, _, age)| age.as_secs_f64() * 1_000.0)
        .fold(0.0_f64, f64::max);
    let match_actor_initializations_operational = actor_initializations
        .iter()
        .filter(|(_, kind, _)| *kind == MatchActorInitializationKind::Actor)
        .all(|(_, _, age)| *age < MATCH_ACTOR_INITIALIZATION_TIMEOUT);
    let warming_match_actor_clocks = actor_states
        .iter()
        .filter(|(_, _, _, lifecycle)| matches!(lifecycle, MatchActorLifecycle::Warming { .. }))
        .count();
    let terminalizing_match_actors = actor_states
        .iter()
        .filter(|(_, _, _, lifecycle)| {
            matches!(lifecycle, MatchActorLifecycle::Terminalizing { .. })
        })
        .count();
    let running_match_actors = actor_states
        .iter()
        .filter(|(_, _, _, lifecycle)| matches!(lifecycle, MatchActorLifecycle::Running))
        .count();
    let max_match_actor_terminal_transition_age_ms = actor_states
        .iter()
        .filter_map(|(_, _, _, lifecycle)| match lifecycle {
            MatchActorLifecycle::Warming { .. } => None,
            MatchActorLifecycle::Running => None,
            MatchActorLifecycle::Terminalizing { started_at } => Some(
                actor_observed_at
                    .saturating_duration_since(*started_at)
                    .as_secs_f64()
                    * 1_000.0,
            ),
        })
        .fold(0.0_f64, f64::max);
    let terminal_ack_gap_transition_owned = terminal_ack_gap_query_healthy
        && match_ids_are_exactly_owned(
            &database_summary.terminal_ack_gap_match_ids,
            terminal_ack_gap_count,
            &bounded_terminal_transition_match_ids,
        );
    let terminal_ack_gap_recovery_operational = terminal_ack_gap_recovery_is_operational(
        terminal_ack_gap_query_healthy,
        terminal_ack_gap_count,
        terminal_ack_gap_transition_owned,
    );
    let historical_projection_public_credit = historical_projection_quarantine_query_healthy
        && historical_projection_quarantine.public_credit_is_clean()
        && terminal_ack_gap_recovery_operational;
    actor_tracked_match_ids.extend(
        state
            .terminal_acknowledged_high_waters
            .read()
            .await
            .iter()
            .copied(),
    );
    let failed_closed_high_waters = state.failed_closed_high_waters.read().await;
    actor_tracked_match_ids.extend(failed_closed_high_waters.iter().copied());
    let tracked_failed_closed_high_water_count = failed_closed_high_waters.len();
    drop(failed_closed_high_waters);
    let active_match_actors = actor_states.len();
    let actor_clock_samples = actor_states
        .iter()
        .map(|(_, published, clock, lifecycle)| {
            let stale_ms = actor_observed_at
                .saturating_duration_since(published.published_at)
                .as_secs_f64()
                * 1_000.0;
            // A newly installed actor needs three 10 Hz observations before a
            // drift window exists. Keep this transition numeric and ready only
            // while its independently bounded warm-up remains healthy; an old,
            // silent, late, or malformed clock still maps to infinity and
            // therefore fails closed.
            let drift_ticks = match lifecycle {
                MatchActorLifecycle::Warming { started_at }
                    if match_actor_clock_warmup_is_operational(
                        clock.as_ref(),
                        actor_observed_at.saturating_duration_since(*started_at),
                        stale_ms,
                        state.tick_interval,
                    ) =>
                {
                    clock
                        .as_ref()
                        .and_then(|snapshot| snapshot.window_drift_ticks)
                        .unwrap_or(0.0)
                }
                _ => clock
                    .as_ref()
                    .and_then(|snapshot| snapshot.window_drift_ticks)
                    .unwrap_or(f64::INFINITY),
            };
            (drift_ticks, stale_ms, clock.as_ref(), *lifecycle)
        })
        .collect::<Vec<_>>();
    let max_actor_clock_abs_drift_ticks = actor_clock_samples
        .iter()
        .map(|(drift, _, _, _)| drift.abs())
        .fold(0.0_f64, f64::max);
    let max_actor_clock_cumulative_abs_drift_ticks = actor_clock_samples
        .iter()
        .filter_map(|(_, _, clock, _)| clock.map(|clock| clock.cumulative_drift_ticks.abs()))
        .fold(0.0_f64, f64::max);
    let max_actor_clock_recent_lateness_ticks = actor_clock_samples
        .iter()
        .filter_map(|(_, _, clock, _)| clock.and_then(|clock| clock.max_recent_lateness_ticks))
        .fold(0.0_f64, f64::max);
    let max_actor_clock_last_wake_age_ms = actor_clock_samples
        .iter()
        .filter_map(|(_, _, clock, _)| clock.and_then(|clock| clock.last_wake_age_ms))
        .fold(0.0_f64, f64::max);
    let max_actor_publish_stale_ms = actor_clock_samples
        .iter()
        .map(|(_, stale_ms, _, _)| *stale_ms)
        .fold(0.0_f64, f64::max);
    let match_actor_registry_coverage_operational = active_matches_query_healthy
        && match_actor_registry_coverage_is_operational(
            active_matches,
            running_match_actors,
            active_match_actors,
            initializing_match_actors,
        );
    let match_actor_clocks_operational = match_actor_registry_coverage_operational
        && match_actor_initializations_operational
        && actor_clock_samples
            .iter()
            .all(|(_, stale_ms, clock, lifecycle)| match lifecycle {
                MatchActorLifecycle::Warming { started_at } => {
                    match_actor_clock_warmup_is_operational(
                        *clock,
                        actor_observed_at.saturating_duration_since(*started_at),
                        *stale_ms,
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
