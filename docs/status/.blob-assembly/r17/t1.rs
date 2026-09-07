#[derive(Debug)]
    Ok(TerminalAckDatabaseEvidence {
            .try_get("journal_owner_id")
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
            reconciled.terminal_acknowledged.insert(evidence.match_id);
    let database_summary =
                .await?)
               and a.match_revision = $6
               and a.next_input_sequences = $7
               and a.snapshot_hash = $8
               and a.phase = 'complete'
               and a.result_hash = $9
               and a.instance_id = $11
               and a.physical_host_id = $12
               and a.published_settlement_state = $10
        )",
    )
    .bind(high_water.match_id)
    .bind(high_water.actor_generation)
    .bind(high_water.actor_epoch)
    .bind(high_water.tick as i64)
    .bind(high_water.next_sequence as i64)
    .bind(high_water.match_revision as i64)
    .bind(
        serde_json::to_value(&high_water.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(&high_water.snapshot_hash)
    .bind(durable_result_hash)
    .bind(durable_settlement_state)
    .bind(&high_water.instance_id)
    .bind(&high_water.physical_host_id)
    .fetch_one(executor)
    .await
    .map_err(|error| error.to_string())
}

fn terminal_read_surface_is_releasable(phase: OnlineMatchPhase, exact_marker_exists: bool) -> bool {
    phase != OnlineMatchPhase::Complete || exact_marker_exists
}

fn terminal_ack_gap_recovery_is_operational(
    query_healthy: bool,
    gap_count: usize,
    bounded_transition_owned: bool,
) -> bool {
    query_healthy && (gap_count == 0 || bounded_transition_owned)
}

fn terminal_runtime_revalidation_is_allowed(
    high_water_instance_id: &str,
    high_water_epoch: i64,
    high_water_physical_host_id: &str,
    runtime_instance_id: &str,
    runtime_epoch: i64,
    runtime_physical_host_id: &str,
    requires_mutation: bool,
) -> bool {
    !requires_mutation
        || (high_water_instance_id == runtime_instance_id
            && high_water_epoch == runtime_epoch
            && high_water_physical_host_id == runtime_physical_host_id)
}

fn terminal_ack_gaps_without_high_water(
    terminal_ack_gaps: &[Uuid],
    recorded_match_ids: &BTreeSet<Uuid>,
) -> Vec<Uuid> {
    terminal_ack_gaps
        .iter()
        .filter(|match_id| !recorded_match_ids.contains(match_id))
        .copied()
