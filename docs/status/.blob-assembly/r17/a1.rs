pub fn production_authority_tick_interval() -> Duration {
            Ok(TerminalJournalReconciliationOutcome::FailedClosed) => {
    match initialized {
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
    let initial = state
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
            phase: "running".to_string(),
            receipts_replayable: true,
            snapshot_hash: snapshot_hash.to_string(),
        })?;
    record_published_tick_with_timeout(
        &state.published_tick_journal,
        initial,
        loaded.next_sequence,
        loaded.match_revision,
        loaded.next_input_sequences.clone(),
    )
    .await
}

async fn adopt_actor_high_water(
    state: &AppState,
    match_id: Uuid,
    actor_id: Uuid,
    loaded: &LoadedMatchActor,
) -> Result<(), String> {
    if let Some(high_water) = state.published_tick_journal.high_water(match_id)? {
        if high_water.actor_generation != actor_id || high_water.instance_id != *state.instance_id {
            let adopted = state
                .published_tick_journal
                .new_record(PublishedTickRecordInput {
                    instance_id: state.instance_id.as_str().to_string(),
                    match_id,
                    actor_generation: actor_id,
                    actor_epoch: state.instance_epoch,
                    tick: high_water.tick,
                    next_sequence: high_water.next_sequence,
                    match_revision: high_water.match_revision,
                    next_input_sequences: high_water.next_input_sequences.clone(),
                    phase: high_water.phase.clone(),
                    receipts_replayable: high_water.receipts_replayable,
                    snapshot_hash: high_water.snapshot_hash,
                })?;
            record_published_tick_with_timeout(
                &state.published_tick_journal,
                adopted,
                loaded.next_sequence,
                loaded.match_revision,
                loaded.next_input_sequences.clone(),
            )
            .await?;
        }
    }
    Ok(())
}

async fn reload_visible_actor_after_persistence_failure(
            durable_db_next_sequence,
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
