pub fn production_authority_tick_interval() -> Duration {
            Ok(TerminalJournalReconciliationOutcome::FailedClosed) => {
    match initialized {
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
    {
        return false;
    }
    let mut advanced_members = 0usize;
    for (player_id, high_water_cursor) in &high_water.next_input_sequences {
        let Some(durable_cursor) = durable_next_input_sequences.get(player_id) else {
            return false;
        };
        if *durable_cursor == high_water_cursor.saturating_add(1) {
            advanced_members += 1;
        } else if durable_cursor != high_water_cursor {
            return false;
        }
    }
    advanced_members == 1
}

fn replay_recovery_to_tick(simulation: &mut MissionSimV1, target_tick: u64) -> Result<(), String> {
    if simulation.tick > target_tick {
        return Err("published-tick recovery attempted to regress a validated source".to_string());
    }
    let recovery_steps = target_tick.saturating_sub(simulation.tick);
    if recovery_steps > MAX_PUBLISHED_TICK_RECOVERY_STEPS {
        return Err(format!(
            "published-tick recovery requires {recovery_steps} steps, exceeding the bounded {MAX_PUBLISHED_TICK_RECOVERY_STEPS}-step limit"
        ));
    }
    while simulation.tick < target_tick {
        if simulation.terminal() {
            return Err(
                "published-tick recovery reached terminal state before its high-water tick"
                    .to_string(),
            );
        }
        simulation.step().map_err(|error| {
            format!("deterministic published-tick recovery step failed: {error}")
        })?;
    }
    Ok(())
}

fn verify_recovery_hash(simulation: &MissionSimV1, expected_hash: &str) -> Result<(), String> {
    let recovered_hash = simulation
        .snapshot_hash()
        .map_err(|error| error.to_string())?;
    if recovered_hash != expected_hash {
        return Err(
            "published-tick recovery hash mismatched after deterministic replay".to_string(),
        );
    }
    Ok(())
}

async fn record_initial_actor_publication(
    state: &AppState,
    match_id: Uuid,
    actor_id: Uuid,
    loaded: &LoadedMatchActor,
    snapshot_hash: &str,
) -> Result<(), String> {
    adopt_actor_high_water(state, match_id, actor_id, loaded).await?;
    let initial = state
async fn reload_visible_actor_after_persistence_failure(
            durable_db_next_sequence,
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
