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
                        state.tick_interval,
                    )
                }
                MatchActorLifecycle::Running => {
                    match_actor_clock_is_operational(*clock, *stale_ms, state.tick_interval)
                }
                MatchActorLifecycle::Terminalizing { started_at } => {
                    actor_observed_at.saturating_duration_since(*started_at)
                        < MATCH_ACTOR_TERMINAL_TRANSITION_TIMEOUT
                        && authority_clock_is_operational(*clock, state.tick_interval)
                }
            });
    let authority_clock_snapshot = state
        .authority_clock
        .snapshot(Instant::now(), state.tick_interval);
    let authority_clock_elapsed_ms = authority_clock_snapshot
        .as_ref()
        .map(|snapshot| snapshot.elapsed_ms);
    let authority_clock_wake_count = authority_clock_snapshot
        .as_ref()
        .map_or(0, |snapshot| snapshot.wake_count);
    let authority_clock_drift_ticks = authority_clock_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.window_drift_ticks);
    let authority_clock_cumulative_drift_ticks = authority_clock_snapshot
        .as_ref()
        .map(|snapshot| snapshot.cumulative_drift_ticks);
    let authority_clock_latest_lateness_ticks = authority_clock_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.latest_lateness_ticks);
    let authority_clock_max_recent_lateness_ticks = authority_clock_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.max_recent_lateness_ticks);
    let authority_clock_last_wake_age_ms = authority_clock_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.last_wake_age_ms);
    let authority_clock_window_sample_count = authority_clock_snapshot
        .as_ref()
        .map_or(0, |snapshot| snapshot.window_sample_count);
    let authority_clock_operational =
        authority_clock_is_operational(authority_clock_snapshot.as_ref(), state.tick_interval);
    let database_pool_size = state.pool.size();
    let database_pool_idle_connections = state.pool.num_idle();
    let database_pool_saturation_healthy = database_pool_is_operational(
        database_pool_size,
        database_pool_idle_connections,
        GAME_SERVER_DATABASE_MAX_CONNECTIONS,
    );
    let readiness_database_pool_size = state.readiness_pool.size();
    let readiness_database_pool_idle_connections = state.readiness_pool.num_idle();
    let readiness_database_pool_saturation_healthy = database_pool_is_operational(
        readiness_database_pool_size,
        readiness_database_pool_idle_connections,
        READINESS_DATABASE_MAX_CONNECTIONS,
    );
    let published_tick_journal_operational = state.published_tick_journal.is_operational();
    let published_tick_records = state.published_tick_journal.recorded_match_ids();
    let published_tick_record_query_healthy = published_tick_records.is_ok();
    let published_tick_hot_record_count = published_tick_records
        .as_ref()
        .map(Vec::len)
        .unwrap_or(usize::MAX);
    let published_tick_terminal_tombstones = state.published_tick_journal.ack_tombstone_count();
    let published_tick_abandonment_tombstones =
        state.published_tick_journal.abandonment_tombstone_count();
    let published_tick_cold_tombstones = state.published_tick_journal.cold_witness_count();
    let published_tick_cold_tombstone_query_healthy = published_tick_terminal_tombstones.is_ok()
        && published_tick_abandonment_tombstones.is_ok()
        && published_tick_cold_tombstones.is_ok();
    let published_tick_terminal_tombstone_count =
        published_tick_terminal_tombstones.unwrap_or(usize::MAX);
    let published_tick_failed_closed_witness_count =
        published_tick_abandonment_tombstones.unwrap_or(usize::MAX);
    let published_tick_cold_tombstone_count = published_tick_cold_tombstones.unwrap_or(usize::MAX);
    let latest_cold_witness_sentinel_query_healthy = latest_cold_witness_sentinel.is_ok();
    let latest_cold_witness_sentinel_healthy = latest_cold_witness_sentinel.unwrap_or(false);
    let cold_witness_database_summary_query_healthy = database_summary_query_healthy;
    let cold_witness_database_summary = database_summary.cold_witness;
    let pending_terminal_tombstone_seal_count = cold_witness_database_summary
        .terminal_total_count
        .saturating_sub(cold_witness_database_summary.terminal_sealed_count);
    let pending_abandonment_tombstone_seal_count = cold_witness_database_summary
        .abandonment_total_count
        .saturating_sub(cold_witness_database_summary.abandonment_sealed_count);
    let pending_local_tombstone_seal_count = pending_terminal_tombstone_seal_count
        .saturating_add(pending_abandonment_tombstone_seal_count);
    let pending_local_tombstone_seal_query_healthy = cold_witness_database_summary_query_healthy;
    let sealed_local_tombstone_ack_query_healthy = cold_witness_database_summary_query_healthy;
    let sealed_local_tombstone_ack_count = cold_witness_database_summary.terminal_sealed_count;
    let sealed_local_abandonment_count = cold_witness_database_summary.abandonment_sealed_count;
    let local_tombstone_counts_exact = published_tick_cold_tombstone_query_healthy
        && cold_witness_database_summary_query_healthy
        && usize::try_from(cold_witness_database_summary.terminal_total_count).ok()
            == Some(published_tick_terminal_tombstone_count)
        && usize::try_from(cold_witness_database_summary.abandonment_total_count).ok()
            == Some(published_tick_failed_closed_witness_count)
        && published_tick_terminal_tombstone_count
            .checked_add(published_tick_failed_closed_witness_count)
            == Some(published_tick_cold_tombstone_count);
    let bounded_terminal_seal_transition_owned = pending_local_tombstone_seal_query_healthy
        && pending_terminal_tombstone_seal_count > 0
        && pending_abandonment_tombstone_seal_count == 0
        && database_summary
            .pending_abandonment_seal_match_ids
            .is_empty()
        && usize::try_from(pending_terminal_tombstone_seal_count)
            .ok()
            .is_some_and(|pending_count| {
                match_ids_are_exactly_owned(
                    &database_summary.pending_terminal_seal_match_ids,
                    pending_count,
                    &bounded_terminal_transition_match_ids,
                )
            });
    let bounded_transition_local_tombstone_count = if bounded_terminal_seal_transition_owned {
        database_summary
            .pending_terminal_seal_match_ids
            .iter()
            .try_fold(0_usize, |count, match_id| {
                state
                    .published_tick_journal
                    .ack_tombstone(*match_id)
                    .map(|tombstone| count + usize::from(tombstone.is_some()))
            })
            .ok()
    } else {
        None
    };
    let local_tombstone_counts_transition_exact = bounded_transition_local_tombstone_count
        .and_then(|local_pending_count| {
            database_summary
                .pending_terminal_seal_match_ids
                .len()
                .checked_sub(local_pending_count)
        })
        .is_some_and(|missing_local_terminal_count| {
            usize::try_from(cold_witness_database_summary.terminal_total_count).ok()
                == published_tick_terminal_tombstone_count.checked_add(missing_local_terminal_count)
                && usize::try_from(cold_witness_database_summary.abandonment_total_count).ok()
                    == Some(published_tick_failed_closed_witness_count)
                && published_tick_terminal_tombstone_count
                    .checked_add(published_tick_failed_closed_witness_count)
                    == Some(published_tick_cold_tombstone_count)
        });
    let latest_cold_witness_sentinel_operational =
        latest_cold_witness_sentinel_healthy || bounded_terminal_seal_transition_owned;
    let local_tombstone_counts_operational =
        local_tombstone_counts_exact || local_tombstone_counts_transition_exact;
    let local_tombstone_seal_operational = published_tick_cold_tombstone_query_healthy
        && latest_cold_witness_sentinel_query_healthy
        && latest_cold_witness_sentinel_operational
        && pending_local_tombstone_seal_query_healthy
        && sealed_local_tombstone_ack_query_healthy
        && ((pending_local_tombstone_seal_count == 0
            && cold_witness_database_summary.all_sealed()
            && local_tombstone_counts_exact)
            || (bounded_terminal_seal_transition_owned && local_tombstone_counts_transition_exact));
    let published_tick_untracked_records = published_tick_records
        .as_ref()
        .map(|match_ids| {
            count_untracked_published_tick_records(match_ids, &actor_tracked_match_ids)
        })
        .unwrap_or(usize::MAX);
    let terminal_orphan_recovery_operational = terminal_orphan_recovery_is_operational(
        published_tick_record_query_healthy,
        published_tick_untracked_records,
    );
    let accepting_commands = !*state.draining.borrow() && !*state.shutdown.borrow();
    let ready = postgres
        && database_host_authority_fresh
        && cex
        && signer.is_some()
        && signer_registry_verified
        && fleet_epoch_current
        && authority_clock_operational
        && active_matches_query_healthy
        && match_actor_clocks_operational
        && database_pool_saturation_healthy
        && readiness_database_pool_saturation_healthy
        && published_tick_journal_operational
        && local_tombstone_seal_operational
        && terminal_orphan_recovery_operational
        && terminal_ack_gap_recovery_operational
        && historical_projection_quarantine_query_healthy
        && accepting_commands;
    let operational_readiness = json!({
        "postgres": postgres,
        "database_host_authority": database_host_authority_fresh,
        "cex": cex,
        "signer": signer.is_some(),
        "signer_registry": signer_registry_verified,
        "fleet_epoch": fleet_epoch_current,
        "authority_clock": authority_clock_operational,
        "match_actor_clocks": match_actor_clocks_operational,
        "active_match_registry_query": active_matches_query_healthy,
        "database_pool": database_pool_saturation_healthy,
        "readiness_database_pool": readiness_database_pool_saturation_healthy,
        "published_tick_journal": published_tick_journal_operational,
        "local_cold_witness_seal": local_tombstone_seal_operational,
        "local_terminal_tombstone_seal": local_tombstone_seal_operational,
        "terminal_orphan_recovery": terminal_orphan_recovery_operational,
        "terminal_ack_gap_recovery": terminal_ack_gap_recovery_operational,
        "historical_projection_quarantine_query": historical_projection_quarantine_query_healthy,
        "accepting_commands": accepting_commands,
    });
    let mut readiness_body = json!({
            "status": if ready { "ok" } else { "blocked" },
            "protocol": ONLINE_AUTHORITY_PROTOCOL,
            "build_id": ONLINE_AUTHORITY_BUILD,
            "postgres_persistent": postgres,
            "database_host_authority_healthy": database_host_authority_healthy,
            "database_host_authority_fresh": database_host_authority_fresh,
            "database_host_authority_last_success_age_ms":
                database_host_authority_last_success_age_ms,
            "database_host_authority_freshness_limit_ms":
                database_host_authority_freshness_limit_ms,
            "database_host_authority_session_lock": database_host_authority_fresh,
            "database_host_authority_pool_barrier": "shared_session_per_physical_connection",
            "database_host_authority_incarnation_pinned": true,
            "fleet_epoch_current": fleet_epoch_current,
            "cex_identity_and_settlement": cex,
            "server_authoritative_campaign": true,
            "server_authoritative_rts": true,
            "published_tick_journal_operational": published_tick_journal_operational,
            "published_tick_hot_record_count": published_tick_hot_record_count,
            "published_tick_cold_tombstone_query_healthy":
                published_tick_cold_tombstone_query_healthy,
            "published_tick_cold_tombstone_count": published_tick_cold_tombstone_count,
            "published_tick_terminal_tombstone_count":
                published_tick_terminal_tombstone_count,
            "published_tick_abandonment_tombstone_count":
                published_tick_failed_closed_witness_count,
            "latest_cold_witness_sentinel_query_healthy":
                latest_cold_witness_sentinel_query_healthy,
            "latest_cold_witness_sentinel_healthy":
                latest_cold_witness_sentinel_healthy,
            "latest_cold_witness_sentinel_operational":
                latest_cold_witness_sentinel_operational,
            "latest_terminal_ack_sentinel_query_healthy":
                latest_cold_witness_sentinel_query_healthy,
            "latest_terminal_ack_sentinel_healthy":
                latest_cold_witness_sentinel_healthy,
            "latest_terminal_ack_sentinel_operational":
                latest_cold_witness_sentinel_operational,
            "pending_local_tombstone_seal_query_healthy":
                pending_local_tombstone_seal_query_healthy,
            "pending_local_tombstone_seal_count": pending_local_tombstone_seal_count,
            "pending_terminal_tombstone_seal_count":
                pending_terminal_tombstone_seal_count,
            "pending_abandonment_tombstone_seal_count":
                pending_abandonment_tombstone_seal_count,
            "bounded_terminal_seal_transition_owned":
                bounded_terminal_seal_transition_owned,
            "sealed_local_tombstone_ack_query_healthy":
                sealed_local_tombstone_ack_query_healthy,
            "sealed_local_tombstone_ack_count": sealed_local_tombstone_ack_count,
            "sealed_local_abandonment_count": sealed_local_abandonment_count,
            "cold_witness_database_summary_query_healthy":
                cold_witness_database_summary_query_healthy,
            "local_tombstone_counts_exact": local_tombstone_counts_exact,
            "local_tombstone_counts_transition_exact":
                local_tombstone_counts_transition_exact,
            "local_tombstone_counts_operational": local_tombstone_counts_operational,
            "local_tombstone_seal_operational": local_tombstone_seal_operational,
            "published_tick_terminal_orphan_recovery_operational": terminal_orphan_recovery_operational,
            "published_tick_untracked_records": published_tick_untracked_records,
            "published_tick_failed_closed_witness_count":
                published_tick_failed_closed_witness_count,
            "tracked_failed_closed_high_water_count":
                tracked_failed_closed_high_water_count,
            "terminal_publication_ack_gap_query_healthy": terminal_ack_gap_query_healthy,
            "terminal_publication_ack_gap_count": terminal_ack_gap_count,
            "terminal_publication_ack_gap_transition_owned":
                terminal_ack_gap_transition_owned,
            "terminal_publication_ack_gap_scan_limit": TERMINAL_ACK_GAP_SCAN_LIMIT,
            "terminal_publication_ack_gap_scan_saturated":
                terminal_ack_gap_scan_is_saturated(terminal_ack_gap_count),
            "legacy_terminal_publication_quarantine_count": historical_projection_quarantine
                .legacy_terminal_match_count,
            "historical_campaign_projection_polluted": historical_projection_quarantine
                .campaign_projection_polluted,
            "historical_rating_projection_polluted": historical_projection_quarantine
                .rating_projection_polluted,
            "historical_projection_quarantine_query_healthy":
                historical_projection_quarantine_query_healthy,
            "historical_projection_public_credit": historical_projection_public_credit,
            "published_tick_journal_host_local_single_process": true,
            "published_tick_journal_requires_one_canonical_directory_per_physical_host": true,
            "tick_rate_hz": 1.0 / state.tick_interval.as_secs_f64(),
            "simulation_ticks_per_wake": 1,
            "simulation_design_tick_rate_hz": TICKS_PER_SECOND,
            "tick_interval_ms": state.tick_interval.as_millis(),
            "clock_mode": if state.accelerated_test_clock {
                "accelerated_test_only_no_catch_up"
            } else {
                "real_time_no_catch_up"
            },
            "restart_grants_immediate_tick": false,
            "authority_clock_elapsed_ms": authority_clock_elapsed_ms,
            "authority_clock_wake_count": authority_clock_wake_count,
            "authority_clock_drift_ticks": authority_clock_drift_ticks,
            "command_event_log": "bounded_async_postgres_with_post_command_recovery_state",
            "command_persistence_queue_per_actor": MATCH_COMMAND_PERSISTENCE_QUEUE,
            "simulation_persistence": "validated_postgres_checkpoint_or_command_state_plus_host_local_fsync_publication_high_water_v2",
            "checkpoint_interval_ticks": MATCH_CHECKPOINT_INTERVAL_TICKS,
            "published_tick_max_recovery_steps": MAX_PUBLISHED_TICK_RECOVERY_STEPS,
            "published_tick_recovery_scope": "single_physical_host_only",
            "published_tick_cross_host_rpo_zero": false,
            "published_tick_fail_closed_hash_verification": true,
            "published_tick_receipt_barrier": "full_tick_hash_phase_global_revision_and_member_cursors",
            "single_authority_process_per_physical_host": true,
            "database_write_per_simulation_tick": false,
            "command_sequence_and_idempotency": true,
            "restart_recovery": true,
            "authenticated_reconnect": true,
            "bounded_command_replay": 256,
            "mode": "private coop plus ranked head-to-head authoritative PvP",
            "independent_member_progression": true,
            "inventory_event_provenance": true,
            "public_matchmaking": true,
            "online_product_protocol": ONLINE_PRODUCT_PROTOCOL,
            "online_product_build": ONLINE_PRODUCT_BUILD,
            "private_lobby_invites": true,
            "coop_vs_ai_match_allocation": true,
            "ranked_solo_queue": true,
            "authoritative_pvp": true,
            "persistent_mmr": historical_projection_public_credit,
            "friends_and_blocks": true,
            "report_and_moderation_workflow": true,
            "online_operations_protocol": trnm_online_protocol::ONLINE_OPERATIONS_PROTOCOL,
            "online_operations_build": trnm_online_protocol::ONLINE_OPERATIONS_BUILD,
            "native_text_login_and_kernel_keyring": true,
            "active_season_and_leaderboard": historical_projection_public_credit,
            "authoritative_replay_index": true,
            "replay_bound_reports": true,
            "integrity_signal_triage": true,
            "moderation_console_and_enforcement": true,
            "fleet_instance_id": state.instance_id.as_str(),
            "fleet_instance_epoch": state.instance_epoch,
            "fleet_region": state.region.as_str(),
            "fleet_capacity": state.capacity,
            "healthy_fleet_instances": healthy_fleet_instances,
            "cross_instance_failover": false,
            "database_fenced_cross_instance_assignment": true,
            "operations_v2_fenced_fleet_leases": true,
            "operations_v2_replay_playback_frames": true,
            "operations_v2_season_admin_and_archival": true,
            "operations_v2_enforcement_appeals_sla": true,
            "operations_v2_drain_and_capacity_control": true,
            "operations_v2_loopback_only_public_bind_gate": true,
            "online_production_protocol": trnm_online_protocol::ONLINE_OPERATIONS_PROTOCOL,
            "online_production_build": trnm_online_protocol::ONLINE_OPERATIONS_BUILD,
            "production_v1_isolated_entitlement_signer": signer.is_some(),
            "production_v1_signer_key_id": signer.as_ref().map(|value| value.key_id.as_str()),
            "production_v1_signer_private_key_exported_to_game_server": false,
            "production_v1_durable_signing_receipts": true,
            "production_v1_rate_limit_per_minute": state.rate_limit_per_minute,
            "production_v1_request_body_limit_bytes": state.request_body_limit_bytes,
            "production_v1_automatic_season_rotation": true,
            "production_v1_targeted_delayed_spectating": true,
            "production_v1_appeal_sla_escalation": true,
            "production_v2_distributed_admission": true,
            "production_v2_capacity_sampling": true,
            "production_v2_signer_key_possession": true,
            "production_v2_signer_registry_verified": signer_registry_verified,
            "production_v2_player_season_spectator_status": true,
            "production_v2_moderation_shift_ownership": true,
            "production_v2_host_challenge_evidence": true,
            "fleet_physical_host_id": state.physical_host_id.as_str(),
            "entitlement_key_custody": signer.as_ref().map(|value| value.custody.as_str())
                .unwrap_or("isolated_signer_unavailable"),
            "kms_hsm_attested": false,
            "public_edge_ddos_attested": false,
    });
    readiness_body["authority_clock_operational"] = Value::Bool(authority_clock_operational);
    readiness_body["authority_clock_max_abs_drift_ticks"] =
        json!(MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS);
    readiness_body["authority_clock_cumulative_drift_ticks"] =
        json!(authority_clock_cumulative_drift_ticks);
    readiness_body["authority_clock_latest_lateness_ticks"] =
        json!(authority_clock_latest_lateness_ticks);
    readiness_body["authority_clock_max_recent_lateness_ticks"] =
        json!(authority_clock_max_recent_lateness_ticks);
    readiness_body["authority_clock_last_wake_age_ms"] = json!(authority_clock_last_wake_age_ms);
    readiness_body["authority_clock_window_sample_count"] =
        json!(authority_clock_window_sample_count);
    readiness_body["authority_clock_window_ticks"] = json!(AUTHORITY_CLOCK_WINDOW_TICKS);
    readiness_body["active_matches"] = json!(active_matches);
    readiness_body["active_match_actors"] = json!(active_match_actors);
    readiness_body["running_match_actors"] = json!(running_match_actors);
    readiness_body["terminalizing_match_actors"] = json!(terminalizing_match_actors);
    readiness_body["warming_match_actor_clocks"] = json!(warming_match_actor_clocks);
    readiness_body["initializing_match_actors"] = json!(initializing_match_actors);
    readiness_body["terminal_recovery_initializations"] = json!(terminal_recovery_initializations);
    readiness_body["match_actor_initializations_operational"] =
        Value::Bool(match_actor_initializations_operational);
    readiness_body["match_actor_registry_coverage_operational"] =
        Value::Bool(match_actor_registry_coverage_operational);
    readiness_body["max_match_actor_initialization_age_ms"] =
        json!(max_match_actor_initialization_age_ms);
    readiness_body["max_match_actor_terminal_transition_age_ms"] =
        json!(max_match_actor_terminal_transition_age_ms);
    readiness_body["match_actor_terminal_transition_timeout_ms"] =
        json!(MATCH_ACTOR_TERMINAL_TRANSITION_TIMEOUT.as_millis());
    readiness_body["match_actor_clocks_operational"] = Value::Bool(match_actor_clocks_operational);
    readiness_body["max_actor_clock_abs_drift_ticks"] = json!(max_actor_clock_abs_drift_ticks);
    readiness_body["max_actor_clock_cumulative_abs_drift_ticks"] =
        json!(max_actor_clock_cumulative_abs_drift_ticks);
    readiness_body["max_actor_clock_recent_lateness_ticks"] =
        json!(max_actor_clock_recent_lateness_ticks);
    readiness_body["max_actor_clock_last_wake_age_ms"] = json!(max_actor_clock_last_wake_age_ms);
    readiness_body["max_actor_publish_stale_ms"] = json!(max_actor_publish_stale_ms);
    readiness_body["match_actor_publication_freshness_limit_ms"] =
        json!(MATCH_ACTOR_PUBLICATION_FRESHNESS.as_millis());
    readiness_body["match_actor_clock_warmup_limit_ms"] =
        json!(match_actor_clock_warmup_limit(state.tick_interval).as_millis());
    readiness_body["operational_readiness"] = operational_readiness;
    readiness_body["database_pool_saturation_healthy"] =
        Value::Bool(database_pool_saturation_healthy);
    readiness_body["database_pool_max_connections"] = json!(GAME_SERVER_DATABASE_MAX_CONNECTIONS);
    readiness_body["database_pool_size"] = json!(database_pool_size);
    readiness_body["database_pool_idle_connections"] = json!(database_pool_idle_connections);
    readiness_body["readiness_database_pool_saturation_healthy"] =
        Value::Bool(readiness_database_pool_saturation_healthy);
    readiness_body["readiness_database_pool_min_connections"] =
        json!(READINESS_DATABASE_MIN_CONNECTIONS);
    readiness_body["readiness_database_pool_max_connections"] =
        json!(READINESS_DATABASE_MAX_CONNECTIONS);
    readiness_body["readiness_database_pool_size"] = json!(readiness_database_pool_size);
    readiness_body["readiness_database_pool_idle_connections"] =
        json!(readiness_database_pool_idle_connections);
    (
        if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(readiness_body),
    )
        .into_response()
}

fn count_untracked_published_tick_records(
    recorded_match_ids: &[Uuid],
    actor_tracked_match_ids: &BTreeSet<Uuid>,
) -> usize {
    recorded_match_ids
        .iter()
        .filter(|match_id| !actor_tracked_match_ids.contains(match_id))
        .count()
}

fn actor_tracked_published_tick_match_ids(
    registry: &MatchActorRegistry,
    observed_at: Instant,
) -> BTreeSet<Uuid> {
    let mut match_ids = registry.actors.keys().copied().collect::<BTreeSet<_>>();
    // A bounded actor initialization owns the running HWM it writes before
    // the handle can be installed in the registry. Keep that deliberate
    // hand-off ready while the independent initialization timeout remains
    // healthy. Terminal-recovery reservations stay excluded: they are
    // reconciling an already-orphaned HWM and must not turn readiness green.
    match_ids.extend(
        registry
            .initializing
            .iter()
            .filter_map(|(match_id, initialization)| {
                (initialization.kind == MatchActorInitializationKind::Actor
                    && observed_at.saturating_duration_since(initialization.started_at)
                        < MATCH_ACTOR_INITIALIZATION_TIMEOUT)
                    .then_some(*match_id)
            }),
    );
    match_ids
}

fn bounded_match_actor_transition_ids(
    registry: &MatchActorRegistry,
    observed_at: Instant,
) -> BTreeSet<Uuid> {
    let mut match_ids = registry
        .actors
        .iter()
        .filter_map(|(match_id, actor)| match *actor.lifecycle.borrow() {
            MatchActorLifecycle::Warming { .. } => None,
            MatchActorLifecycle::Running => None,
            MatchActorLifecycle::Terminalizing { started_at }
                if observed_at.saturating_duration_since(started_at)
                    < MATCH_ACTOR_TERMINAL_TRANSITION_TIMEOUT =>
            {
                Some(*match_id)
            }
            MatchActorLifecycle::Terminalizing { .. } => None,
        })
        .collect::<BTreeSet<_>>();
    match_ids.extend(
        registry
            .initializing
            .iter()
            .filter_map(|(match_id, initialization)| {
                let timeout = match initialization.kind {
                    MatchActorInitializationKind::Actor => MATCH_ACTOR_INITIALIZATION_TIMEOUT,
                    MatchActorInitializationKind::TerminalRecovery => {
                        MATCH_ACTOR_TERMINAL_TRANSITION_TIMEOUT
                    }
                };
                (observed_at.saturating_duration_since(initialization.started_at) < timeout)
                    .then_some(*match_id)
            }),
    );
    match_ids
}

fn terminal_orphan_recovery_is_operational(
    published_tick_record_query_healthy: bool,
    published_tick_untracked_records: usize,
) -> bool {
    published_tick_record_query_healthy && published_tick_untracked_records == 0
}

fn match_actor_registry_coverage_is_operational(
    active_matches: i64,
    running_match_actors: usize,
    total_match_actors: usize,
    initializing_match_actors: usize,
) -> bool {
    usize::try_from(active_matches).ok().is_some_and(|active| {
        running_match_actors <= active
            && active <= total_match_actors.saturating_add(initializing_match_actors)
    })
}

fn match_ids_are_exactly_owned(
    match_ids: &[Uuid],
    expected_count: usize,
    bounded_owners: &BTreeSet<Uuid>,
) -> bool {
    let unique = match_ids.iter().copied().collect::<BTreeSet<_>>();
    match_ids.len() == expected_count
        && unique.len() == expected_count
        && unique.is_subset(bounded_owners)
}

fn authority_clock_is_operational(
    snapshot: Option<&AuthorityClockSnapshot>,
    tick_interval: Duration,
) -> bool {
    snapshot.is_some_and(|snapshot| {
        snapshot.window_sample_count >= AUTHORITY_CLOCK_MIN_SAMPLES
            && snapshot.window_drift_ticks.is_some_and(|drift| {
                drift.is_finite() && drift.abs() < MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS
            })
            && snapshot.latest_lateness_ticks.is_some_and(|lateness| {
                (0.0..MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS).contains(&lateness)
            })
            && snapshot.max_recent_lateness_ticks.is_some_and(|lateness| {
                (0.0..MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS).contains(&lateness)
            })
            && snapshot.last_wake_age_ms.is_some_and(|age_ms| {
                (0.0..tick_interval.as_secs_f64() * 2_000.0).contains(&age_ms)
            })
    })
}

fn match_actor_clock_is_operational(
    snapshot: Option<&AuthorityClockSnapshot>,
    published_stale_ms: f64,
    tick_interval: Duration,
) -> bool {
    authority_clock_is_operational(snapshot, tick_interval)
        && published_stale_ms.is_finite()
        && published_stale_ms < MATCH_ACTOR_PUBLICATION_FRESHNESS.as_secs_f64() * 1_000.0
}

fn match_actor_clock_warmup_limit(tick_interval: Duration) -> Duration {
    tick_interval.saturating_mul(
        u32::try_from(AUTHORITY_CLOCK_MIN_SAMPLES)
            .unwrap_or(u32::MAX)
            .saturating_add(MATCH_ACTOR_CLOCK_WARMUP_EXTRA_TICKS),
    )
}

fn match_actor_clock_warmup_is_operational(
    snapshot: Option<&AuthorityClockSnapshot>,
    warmup_age: Duration,
    published_stale_ms: f64,
    tick_interval: Duration,
) -> bool {
    let warmup_limit = match_actor_clock_warmup_limit(tick_interval);
    if warmup_age >= warmup_limit
        || !published_stale_ms.is_finite()
        || published_stale_ms >= MATCH_ACTOR_PUBLICATION_FRESHNESS.as_secs_f64() * 1_000.0
    {
        return false;
    }
    let first_wake_grace = tick_interval.saturating_mul(MATCH_ACTOR_CLOCK_FIRST_WAKE_GRACE_TICKS);
    let Some(snapshot) = snapshot else {
        return warmup_age < first_wake_grace;
    };
    if snapshot.window_sample_count >= AUTHORITY_CLOCK_MIN_SAMPLES {
        return authority_clock_is_operational(Some(snapshot), tick_interval);
    }
    if !snapshot.elapsed_ms.is_finite()
        || snapshot.elapsed_ms < 0.0
        || !snapshot.cumulative_drift_ticks.is_finite()
        || u64::try_from(snapshot.window_sample_count).ok() != Some(snapshot.wake_count)
    {
        return false;
    }
    if snapshot.window_sample_count == 0 {
        return warmup_age < first_wake_grace
            && snapshot.window_drift_ticks.is_none()
            && snapshot.latest_lateness_ticks.is_none()
            && snapshot.max_recent_lateness_ticks.is_none()
            && snapshot.last_wake_age_ms.is_none();
    }
    let lateness_is_healthy = |value: Option<f64>| {
        value.is_some_and(|value| (0.0..MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS).contains(&value))
    };
    let drift_is_healthy = match snapshot.window_drift_ticks {
        Some(drift) => {
            snapshot.window_sample_count >= 2
                && drift.is_finite()
                && drift.abs() < MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS
        }
        None => snapshot.window_sample_count == 1,
    };
    drift_is_healthy
        && lateness_is_healthy(snapshot.latest_lateness_ticks)
        && lateness_is_healthy(snapshot.max_recent_lateness_ticks)
        && snapshot
            .last_wake_age_ms
            .is_some_and(|age_ms| (0.0..tick_interval.as_secs_f64() * 2_000.0).contains(&age_ms))
}

fn receipt_protocol_for_observed_tick(client_observed_tick: Option<i64>) -> &'static str {
    if client_observed_tick.is_some() {
        ONLINE_AUTHORITY_PROTOCOL
    } else {
        ONLINE_AUTHORITY_V2_PROTOCOL
    }
}

fn database_pool_is_operational(
    pool_size: u32,
    idle_connections: usize,
    max_connections: u32,
) -> bool {
    idle_connections > 0 || pool_size < max_connections
}

