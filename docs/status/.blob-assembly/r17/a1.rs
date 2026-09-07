pub fn production_authority_tick_interval() -> Duration {
            Ok(TerminalJournalReconciliationOutcome::FailedClosed) => {
    match initialized {
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
    let initial = state
async fn reload_visible_actor_after_persistence_failure(
    state: &AppState,
    match_id: Uuid,
    visible: LoadedMatchActor,
    recovery_target_tick: u64,
    minimum_public_tick: u64,
) -> Result<LoadedMatchActor, String> {
    let Some(mut durable) = load_match_actor(state, match_id).await? else {
        return Err("running match disappeared during command recovery".to_string());
    };
    if durable.match_mode != visible.match_mode
        || durable.next_sequence != visible.next_sequence
        || durable.match_revision != visible.match_revision
        || durable.next_input_sequences != visible.next_input_sequences
    {
        return Err(
            "durable authority advanced while speculative command recovery was in progress"
                .to_string(),
        );
    }
    // Rebuild from the durable pre-command cursor, never from the speculative
    // command lane. Catch autonomous simulation up to the highest internally
    // reached safe tick so a multi-second persistence wait cannot regress time.
    // A deterministic terminal reached during replay may stop earlier, but it
    // must still be at or beyond the last publicly visible tick.
    replay_durable_actor_after_pending(&mut durable, recovery_target_tick, minimum_public_tick)?;
    Ok(durable)
}

fn replay_durable_actor_after_pending(
    durable: &mut LoadedMatchActor,
    recovery_target_tick: u64,
    minimum_public_tick: u64,
) -> Result<(), String> {
    if !publication_within_recovery_budget(recovery_target_tick, durable.durable_recovery_tick) {
        return Err(
            "command recovery target exceeded the bounded durable replay horizon".to_string(),
        );
    }
    while durable.simulation.tick < recovery_target_tick && !durable.simulation.terminal() {
        durable
            .simulation
            .step()
            .map_err(|error| format!("command recovery deterministic step failed: {error}"))?;
    }
    if durable.simulation.tick < minimum_public_tick {
        return Err("command recovery would regress the last publicly safe tick".to_string());
    }
    Ok(())
}

async fn record_published_tick_with_timeout(
    journal: &PublishedTickJournal,
    record: PublishedTickHighWater,
    durable_db_next_sequence: u64,
    durable_db_match_revision: u64,
    durable_db_next_input_sequences: BTreeMap<String, u64>,
) -> Result<(), String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.record(
            record,
            durable_db_next_sequence,
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
