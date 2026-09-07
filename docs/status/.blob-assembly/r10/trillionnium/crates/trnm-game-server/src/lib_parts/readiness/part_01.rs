__TRNM_SLOT_0__
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
__TRNM_SLOT_2__
__TRNM_SLOT_3__
