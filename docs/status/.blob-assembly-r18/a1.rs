@@TRNM_R18_A1_SEGMENT_0@@
                state
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
@@TRNM_R18_A1_SEGMENT_1@@
    }
    registry.initializing.remove(&match_id);
    reservation.disarm();
    match initialized {
        Ok(Some(initialized)) => {
            let handle = initialized.handle.clone();
            registry.actors.insert(match_id, handle.clone());
            initialization.ready.send_replace(true);
@@TRNM_R18_A1_SEGMENT_2@@
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    if row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?
        != "running"
    {
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(None);
    }
    let previous_instance: Option<String> = row
        .try_get("assigned_instance_id")
        .map_err(|error| error.to_string())?;
    let previous_region: Option<String> = row
        .try_get("assigned_region")
        .map_err(|error| error.to_string())?;
    let previous_epoch: i64 = row
        .try_get("assigned_instance_epoch")
        .map_err(|error| error.to_string())?;
    let previous_physical_host: Option<String> = row
        .try_get("assigned_physical_host_id")
        .map_err(|error| error.to_string())?;
    let terminal_staged = row
        .try_get::<Option<String>, _>("terminal_stage_snapshot_hash")
        .map_err(|error| error.to_string())?
        .is_some();
    if !match_assignment_uses_local_physical_host(
        previous_instance.as_deref(),
        previous_physical_host.as_deref(),
        state.physical_host_id.as_str(),
    ) {
        return Err(
            "match assignment physical host differs from the host-local publication journal; cross-host takeover is blocked"
                .to_string(),
        );
    }
    if previous_instance.as_deref() != Some(state.instance_id.as_str())
        || previous_epoch != state.instance_epoch
    {
        if terminal_staged {
            return Err(
                "private terminal stage must be finalized by its exact journal ownership before takeover"
                    .to_string(),
            );
        }
        let previous_healthy: bool = if let Some(previous) = previous_instance.as_deref() {
            sqlx::query_scalar::query_scalar(
                "select exists(select 1 from trnm_online_fleet_instances
                 where instance_id = $1 and instance_epoch = $2
                   and status in ('active', 'draining')
                   and lease_expires_at > now())",
            )
            .bind(previous)
            .bind(previous_epoch)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|error| error.to_string())?
        } else {
            false
        };
        if previous_healthy {
            transaction
                .commit()
                .await
                .map_err(|error| error.to_string())?;
            return Ok(None);
        }
        sqlx::query::query(
            "update trnm_online_matches set assigned_instance_id = $2,
                assigned_region = $3, assigned_instance_epoch = $4,
                assigned_physical_host_id = $5, updated_at = now() where match_id = $1",
        )
        .bind(match_id)
        .bind(state.instance_id.as_str())
        .bind(state.region.as_str())
        .bind(state.instance_epoch)
        .bind(state.physical_host_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        sqlx::query::query(
            "insert into trnm_online_fleet_failovers (
                failover_id, match_id, previous_instance_id, new_instance_id,
                previous_region, new_region, reason,
                previous_instance_epoch, new_instance_epoch,
                previous_physical_host_id, new_physical_host_id
             ) values ($1, $2, $3, $4, $5, $6, 'owner lease expired or epoch fenced', $7, $8, $9, $10)
             on conflict do nothing",
        )
        .bind(Uuid::new_v4())
        .bind(match_id)
        .bind(&previous_instance)
        .bind(state.instance_id.as_str())
        .bind(&previous_region)
        .bind(state.region.as_str())
        .bind(previous_epoch)
        .bind(state.instance_epoch)
        .bind(&previous_physical_host)
        .bind(state.physical_host_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    let checkpoint_sequence = row
        .try_get::<i64, _>("checkpoint_sequence")
        .map_err(|error| error.to_string())? as u64;
    let next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(|error| error.to_string())? as u64;
    let match_revision = row
        .try_get::<i64, _>("match_revision")
        .map_err(|error| error.to_string())? as u64;
    let value: Value = row
        .try_get("simulation_json")
        .map_err(|error| error.to_string())?;
    let checkpoint_simulation: MissionSimV1 =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    let checkpoint_hash: String = row
        .try_get("snapshot_hash")
        .map_err(|error| error.to_string())?;
    let checkpoint_tick = row
        .try_get::<i64, _>("authoritative_tick")
        .map_err(|error| error.to_string())?;
    let checkpoint_tick = u64::try_from(checkpoint_tick)
        .map_err(|_| "durable checkpoint tick is negative".to_string())?;
    validate_recovery_simulation(
        &checkpoint_simulation,
        Some(checkpoint_tick),
        &checkpoint_hash,
        "database checkpoint",
    )?;
    let mut simulation = checkpoint_simulation.clone();
    if checkpoint_sequence < next_sequence {
        let recovered_row = sqlx::query::query(
            "select post_simulation_json, accepted_snapshot_hash, accepted_match_revision,
                    target_tick, client_observed_tick, order_json
             from trnm_online_commands
             where match_id = $1 and sequence = $2",
        )
        .bind(match_id)
        .bind(next_sequence.saturating_sub(1) as i64)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "latest command recovery event is missing".to_string())?;
        let accepted_revision = recovered_row
            .try_get::<i64, _>("accepted_match_revision")
            .map_err(|error| error.to_string())? as u64;
        if accepted_revision != match_revision {
            return Err("latest command recovery revision does not match authority".to_string());
        }
        let accepted_hash: String = recovered_row
            .try_get("accepted_snapshot_hash")
            .map_err(|error| error.to_string())?;
        let recovered: Option<Value> = recovered_row
            .try_get("post_simulation_json")
            .map_err(|error| error.to_string())?;
        let recovered = recovered.ok_or_else(|| {
            "command recovery event is missing its post-command simulation".to_string()
        })?;
        simulation = serde_json::from_value(recovered).map_err(|error| error.to_string())?;
        validate_recovery_simulation(
            &simulation,
            Some(simulation.tick),
            &accepted_hash,
            "latest command recovery state",
        )?;
        validate_recovered_command_timing(
            &simulation,
            recovered_row
                .try_get::<i64, _>("target_tick")
                .map_err(|error| error.to_string())?,
            recovered_row
                .try_get::<Option<i64>, _>("client_observed_tick")
                .map_err(|error| error.to_string())?,
            recovered_row
                .try_get::<Value, _>("order_json")
                .map_err(|error| error.to_string())?,
            "latest command recovery state",
        )?;
    }
    let match_mode: String = row
        .try_get("match_mode")
        .map_err(|error| error.to_string())?;
    let staged_terminal = staged_terminal_authority_from_row(&row, &match_mode)?;
    let input_rows = sqlx::query::query(
        "select player_id, account_id, controlled_unit_ids, member_role,
                next_input_sequence
         from trnm_online_match_members where match_id = $1",
    )
    .bind(match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut next_input_sequences = BTreeMap::new();
    let mut members = BTreeMap::new();
    for input_row in input_rows {
        let player_id: String = input_row
            .try_get("player_id")
            .map_err(|error| error.to_string())?;
        let next_input_sequence = input_row
            .try_get::<i64, _>("next_input_sequence")
            .map_err(|error| error.to_string())?;
        let next_input_sequence = u64::try_from(next_input_sequence)
            .map_err(|_| "member input sequence is negative".to_string())?;
        let controlled_unit_ids = serde_json::from_value::<Vec<String>>(
            input_row
                .try_get::<Value, _>("controlled_unit_ids")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("decode match actor member controls: {error}"))?
        .into_iter()
        .collect::<BTreeSet<_>>();
        let authority = ActorMemberAuthority {
            account_id: input_row
                .try_get("account_id")
                .map_err(|error| error.to_string())?,
            controlled_unit_ids,
            member_role: input_row
                .try_get("member_role")
                .map_err(|error| error.to_string())?,
        };
        if next_input_sequences
            .insert(player_id.clone(), next_input_sequence)
            .is_some()
            || members.insert(player_id, authority).is_some()
        {
            return Err("match actor member authority is duplicated".to_string());
        }
    }
    if members.is_empty() || members.len() != next_input_sequences.len() {
        return Err("match actor member authority is incomplete".to_string());
    }
    let bridge_simulation = if let Some(staged) = staged_terminal.as_ref() {
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
@@TRNM_R18_A1_SEGMENT_4@@
        || durable_match_revision != high_water.match_revision.saturating_add(1)
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
    {
        return false;
    }
    let mut advanced_members = 0usize;
    for (player_id, high_water_cursor) in &high_water.next_input_sequences {
        let Some(durable_cursor) = durable_next_input_sequences.get(player_id) else {
@@TRNM_R18_A1_SEGMENT_5@@
