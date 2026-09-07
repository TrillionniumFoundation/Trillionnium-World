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
