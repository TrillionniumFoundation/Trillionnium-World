__TRNM_CHUNK_0__
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
                    .await
__TRNM_CHUNK_1__
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
            "{label} scheduled target/order tick does not match its post-command state"
        ));
    }
    Ok(())
}

fn is_single_command_cursor_successor(
    high_water: &PublishedTickHighWater,
    durable_next_sequence: u64,
    durable_match_revision: u64,
    durable_next_input_sequences: &BTreeMap<String, u64>,
) -> bool {
    if durable_next_sequence != high_water.next_sequence.saturating_add(1)
        || durable_match_revision != high_water.match_revision.saturating_add(1)
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
    Ok(durable)
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
            durable_db_match_revision,
            durable_db_next_input_sequences,
        ),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            // Once an fsync request was accepted, cancellation cannot prove
            // whether the blocking filesystem operation took effect. Poison
            // the host journal for this process so no replacement actor can
            // publish over an uncertain high-water until restart recovery.
            journal.fail_closed();
            Err(
                "published-tick journal operation exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}

async fn seal_terminal_ack_with_timeout(
    journal: &PublishedTickJournal,
    input: PublishedTickAckTombstoneInput,
) -> Result<PublishedTickAckTombstone, String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.seal_terminal_ack(input),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick ACK tombstone seal exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}

async fn seal_abandonment_with_timeout(
    journal: &PublishedTickJournal,
    input: PublishedTickAbandonmentTombstoneInput,
) -> Result<PublishedTickAbandonmentTombstone, String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.seal_abandonment(input),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}
