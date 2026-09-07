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
            || high_water.next_sequence != staged.next_sequence
            || high_water.match_revision != staged.match_revision
            || high_water.next_input_sequences != next_input_sequences
            || high_water.snapshot_hash != staged.snapshot_hash
        {
            return Err(
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
            );
        }
        None
    } else if let Some(high_water) = high_water.as_ref() {
        if high_water.phase != "running" {
            return Err(
                "a running database match has a terminal published-tick high-water".to_string(),
            );
        }
        if high_water.next_sequence == next_sequence
            && high_water.match_revision == match_revision
            && high_water.next_input_sequences == next_input_sequences
        {
            None
        } else if high_water.next_sequence == checkpoint_sequence {
            let commands_after_checkpoint = next_sequence
                .checked_sub(checkpoint_sequence)
                .ok_or_else(|| "checkpoint sequence is ahead of authority".to_string())?;
            let checkpoint_revision = match_revision
                .checked_sub(commands_after_checkpoint)
                .ok_or_else(|| "checkpoint revision underflowed authority".to_string())?;
            if high_water.match_revision != checkpoint_revision {
                return Err(
                    "published-tick checkpoint bridge revision does not match authority"
                        .to_string(),
                );
            }
            Some(checkpoint_simulation.clone())
        } else if high_water.next_sequence > checkpoint_sequence
            && high_water.next_sequence < next_sequence
        {
            let bridge_row = sqlx::query::query(
                "select post_simulation_json, accepted_snapshot_hash,
                        accepted_match_revision, target_tick, client_observed_tick, order_json
                 from trnm_online_commands
                 where match_id = $1 and sequence = $2",
            )
            .bind(match_id)
            .bind(high_water.next_sequence.saturating_sub(1) as i64)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "published-tick bridge command is missing".to_string())?;
            let bridge_revision = bridge_row
                .try_get::<i64, _>("accepted_match_revision")
                .map_err(|error| error.to_string())? as u64;
            if bridge_revision != high_water.match_revision {
                return Err(
                    "published-tick bridge command revision does not match high-water".to_string(),
                );
            }
            let bridge_hash: String = bridge_row
                .try_get("accepted_snapshot_hash")
                .map_err(|error| error.to_string())?;
            let bridge_value = bridge_row
                .try_get::<Option<Value>, _>("post_simulation_json")
                .map_err(|error| error.to_string())?
                .ok_or_else(|| {
                    "published-tick bridge command has no recovery simulation".to_string()
                })?;
            let bridge = serde_json::from_value::<MissionSimV1>(bridge_value)
                .map_err(|error| error.to_string())?;
            validate_recovery_simulation(
                &bridge,
                Some(bridge.tick),
                &bridge_hash,
                "published-tick bridge command",
            )?;
            validate_recovered_command_timing(
                &bridge,
                bridge_row
                    .try_get::<i64, _>("target_tick")
                    .map_err(|error| error.to_string())?,
                bridge_row
                    .try_get::<Option<i64>, _>("client_observed_tick")
                    .map_err(|error| error.to_string())?,
                bridge_row
                    .try_get::<Value, _>("order_json")
                    .map_err(|error| error.to_string())?,
                "published-tick bridge command",
            )?;
            Some(bridge)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(staged) = staged_terminal.as_ref() {
        simulation = staged.simulation.clone();
    }
    let durable_recovery_tick = simulation.tick;
    let mut loaded = LoadedMatchActor {
        simulation,
        match_mode,
        members: Arc::new(members),
        next_sequence,
        match_revision,
        next_input_sequences,
        durable_recovery_tick,
    };
    lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    if staged_terminal.is_none() {
        if let Some(high_water) = high_water.as_ref() {
            recover_to_published_high_water(
                &mut loaded.simulation,
                loaded.next_sequence,
                loaded.match_revision,
                &loaded.next_input_sequences,
                bridge_simulation,
                high_water,
            )?;
        }
    }
    Ok(Some(loaded))
}

fn match_assignment_uses_local_physical_host(
    assigned_instance: Option<&str>,
    assigned_physical_host: Option<&str>,
    local_physical_host: &str,
) -> bool {
    assigned_instance.is_none() || assigned_physical_host == Some(local_physical_host)
}

const MAX_PUBLISHED_TICK_RECOVERY_STEPS: u64 = 10_000;

fn recover_to_published_high_water(
    simulation: &mut MissionSimV1,
    durable_db_next_sequence: u64,
    durable_db_match_revision: u64,
    durable_db_next_input_sequences: &BTreeMap<String, u64>,
    bridge_simulation: Option<MissionSimV1>,
    high_water: &PublishedTickHighWater,
) -> Result<(), String> {
    if high_water.phase != "running" || !high_water.receipts_replayable {
        return Err(
            "running recovery requires a replayable running published-tick record".to_string(),
        );
    }
    if high_water.next_sequence > durable_db_next_sequence
        || high_water.match_revision > durable_db_match_revision
        || high_water.next_input_sequences.len() != durable_db_next_input_sequences.len()
        || high_water
            .next_input_sequences
            .iter()
            .any(|(player_id, cursor)| {
                durable_db_next_input_sequences
                    .get(player_id)
                    .is_none_or(|durable| cursor > durable)
            })
    {
        return Err(
            "published-tick journal is ahead of durable database command cursors; recovery failed closed"
                .to_string(),
        );
    }
    if simulation.tick > high_water.tick {
        // The caller strictly validated this newer database checkpoint/event
        // against its own persisted tick/hash before entering recovery.
        return Ok(());
    }
    let same_cursor = high_water.next_sequence == durable_db_next_sequence
        && high_water.match_revision == durable_db_match_revision
        && high_water.next_input_sequences == *durable_db_next_input_sequences;
    if same_cursor {
        replay_recovery_to_tick(simulation, high_water.tick)?;
        verify_recovery_hash(simulation, &high_water.snapshot_hash)?;
        return Ok(());
    }
    if !is_single_command_cursor_successor(
        high_water,
        durable_db_next_sequence,
        durable_db_match_revision,
        durable_db_next_input_sequences,
    ) {
        return Err(
            "published-tick recovery cannot bridge a non-adjacent durable command boundary"
                .to_string(),
        );
    }
    let mut bridge = bridge_simulation.ok_or_else(|| {
        "published-tick recovery is missing the prior durable cursor bridge".to_string()
    })?;
    replay_recovery_to_tick(&mut bridge, high_water.tick)?;
    verify_recovery_hash(&bridge, &high_water.snapshot_hash)?;
    replay_recovery_to_tick(simulation, high_water.tick)
}

fn validate_recovery_simulation(
    simulation: &MissionSimV1,
    expected_tick: Option<u64>,
    expected_hash: &str,
    label: &str,
) -> Result<(), String> {
    if expected_tick.is_some_and(|tick| simulation.tick != tick) {
        return Err(format!("{label} tick does not match persisted authority"));
    }
    let computed_hash = simulation
        .snapshot_hash()
        .map_err(|error| format!("hash {label}: {error}"))?;
    if computed_hash != expected_hash {
        return Err(format!("{label} hash does not match persisted authority"));
    }
    Ok(())
}

fn validate_recovered_command_timing(
    simulation: &MissionSimV1,
    persisted_target_tick: i64,
    persisted_client_observed_tick: Option<i64>,
    persisted_order: Value,
    label: &str,
) -> Result<(), String> {
    let target_tick = u64::try_from(persisted_target_tick)
        .map_err(|_| format!("{label} target tick is negative"))?;
    let observed_tick = persisted_client_observed_tick
        .map(u64::try_from)
        .transpose()
        .map_err(|_| format!("{label} observed tick is negative"))?;
    let order = serde_json::from_value::<trnm_rts_protocol::RtsFrameOrder>(persisted_order)
        .map_err(|error| format!("decode {label} order: {error}"))?;
    let order_tick = u64::from(order.frame);
    if let Some(observed_tick) = observed_tick {
        if simulation.tick != target_tick || observed_tick > target_tick || order_tick < target_tick
        {
            return Err(format!(
                "{label} realtime accepted/observed/order ticks do not match its post-command state"
            ));
        }
    } else if target_tick < simulation.tick
        || target_tick > simulation.tick.saturating_add(200)
        || order_tick != target_tick
    {
        return Err(format!(
__TRNM_CHUNK_5__
