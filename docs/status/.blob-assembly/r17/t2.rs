async fn terminal_high_water_is_durably_acknowledged(
    pool: &PgPool,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    runtime_state: Option<&AppState>,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<bool, String> {
    if high_water.phase != "complete" || !high_water.receipts_replayable {
        return Ok(false);
    }
    let mut connection = pool.acquire().await.map_err(|error| error.to_string())?;
    let mut transaction = (*connection)
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    let runtime_mutation_state = runtime_state.filter(|state| {
        terminal_runtime_revalidation_is_allowed(
            &high_water.instance_id,
            high_water.actor_epoch,
            &high_water.physical_host_id,
            state.instance_id.as_str(),
            state.instance_epoch,
            state.physical_host_id.as_str(),
            true,
        )
    });
    let mutation_allowed = runtime_state.is_none() || runtime_mutation_state.is_some();
    if let Some(state) = runtime_mutation_state {
        lock_current_fleet_epoch(&mut transaction, state, true).await?;
    }
    let staged_commit = if mutation_allowed {
        finalize_staged_terminal_authority(
            &mut transaction,
            TerminalFinalizationExpectation {
                match_id: high_water.match_id,
                actor_generation: high_water.actor_generation,
                actor_epoch: high_water.actor_epoch,
                instance_id: &high_water.instance_id,
                physical_host_id: &high_water.physical_host_id,
                authoritative_tick: high_water.tick,
                next_sequence: high_water.next_sequence,
                match_revision: high_water.match_revision,
                next_input_sequences: &high_water.next_input_sequences,
                snapshot_hash: &high_water.snapshot_hash,
                result_hash: None,
            },
        )
        .await?
    } else {
        None
    };
    if let Some(commit) = staged_commit {
        if !matches!(commit.settlement_state.as_str(), "pending" | "settled") {
            return Err("terminal recovery produced an invalid settlement state".to_string());
        }
        if let Some(state) = runtime_mutation_state {
            lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
        }
        require_database_host_authority(&mut transaction, database_host_authority).await?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        let lineage = database_lineage(&mut *connection).await?;
        let tombstone = ensure_cold_terminal_ack(journal, high_water, &commit, &lineage).await?;
        seal_terminal_ack_in_database(
            &mut connection,
            &tombstone,
            runtime_mutation_state,
            database_host_authority,
        )
        .await?;
        if let Some(state) = runtime_state {
            state
                .terminal_acknowledged_high_waters
                .write()
                .await
                .insert(high_water.match_id);
        }
        return Ok(true);
    }
    let Some(row) = sqlx::query::query(
        "select m.phase, m.simulation_json, m.snapshot_hash, m.authoritative_tick,
                m.next_sequence, m.match_revision, m.checkpoint_sequence,
                m.result_json, m.result_hash, m.settlement_state, m.match_mode,
                m.assigned_instance_id, m.assigned_instance_epoch,
                m.assigned_physical_host_id, m.terminal_publication_actor_generation,
                m.terminal_publication_state,
                a.actor_generation, a.instance_id as ack_instance_id,
                a.actor_epoch, a.physical_host_id as ack_physical_host_id,
                a.authoritative_tick as ack_tick,
                a.next_sequence as ack_next_sequence,
                a.match_revision as ack_match_revision,
                a.next_input_sequences as ack_next_input_sequences,
                a.snapshot_hash as ack_snapshot_hash,
                a.phase as ack_phase, a.result_hash as ack_result_hash,
                a.published_settlement_state as ack_settlement_state,
                floor(extract(epoch from a.acknowledged_at) * 1000)::bigint
                    as ack_acknowledged_at_unix_ms
         from trnm_online_matches m
         left join trnm_online_terminal_publication_acks a on a.match_id = m.match_id
         where m.match_id = $1 for share of m",
    )
    .bind(high_water.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(false);
    };
    let member_rows = sqlx::query::query(
        "select player_id, next_input_sequence from trnm_online_match_members
         where match_id = $1 order by player_id",
    )
    .bind(high_water.match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut member_cursors = BTreeMap::new();
    for member in member_rows {
        let cursor = member
            .try_get::<i64, _>("next_input_sequence")
            .map_err(|error| error.to_string())?;
        member_cursors.insert(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            u64::try_from(cursor).map_err(|_| "terminal member cursor is negative".to_string())?,
        );
    }
    let phase = row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?;
    let result_hash = row
        .try_get::<Option<String>, _>("result_hash")
        .map_err(|error| error.to_string())?;
    let result_value = row
        .try_get::<Option<Value>, _>("result_json")
        .map_err(|error| error.to_string())?;
    let settlement_state = row
        .try_get::<String, _>("settlement_state")
        .map_err(|error| error.to_string())?;
    let match_mode = row
        .try_get::<String, _>("match_mode")
        .map_err(|error| error.to_string())?;
    let simulation_value = row
        .try_get::<Option<Value>, _>("simulation_json")
        .map_err(|error| error.to_string())?;
    let Some(simulation_value) = simulation_value else {
        return Ok(false);
    };
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_value)
        .map_err(|error| format!("decode terminal compaction checkpoint: {error}"))?;
    let simulation_hash = simulation
        .snapshot_hash()
        .map_err(|error| format!("hash terminal compaction checkpoint: {error}"))?;
    let result_valid = terminal_result_matches_simulation(
        &simulation,
        result_value,
        result_hash.as_deref(),
        &match_mode,
    )?;

    let authoritative_tick = row
        .try_get::<i64, _>("authoritative_tick")
        .map_err(|error| error.to_string())?;
    let next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(|error| error.to_string())?;
    let checkpoint_sequence = row
        .try_get::<i64, _>("checkpoint_sequence")
        .map_err(|error| error.to_string())?;
    let match_revision = row
        .try_get::<i64, _>("match_revision")
        .map_err(|error| error.to_string())?;
    let durable_snapshot_hash = row
        .try_get::<String, _>("snapshot_hash")
        .map_err(|error| error.to_string())?;
    let durable_actor_generation = row
        .try_get::<Option<Uuid>, _>("terminal_publication_actor_generation")
        .map_err(|error| error.to_string())?;
    let durable_publication_state = row
        .try_get::<String, _>("terminal_publication_state")
        .map_err(|error| error.to_string())?;
    let marker_generation = row
        .try_get::<Option<Uuid>, _>("actor_generation")
        .map_err(|error| error.to_string())?;
    let marker_instance_id = row
        .try_get::<Option<String>, _>("ack_instance_id")
        .map_err(|error| error.to_string())?;
    let marker_epoch = row
        .try_get::<Option<i64>, _>("actor_epoch")
        .map_err(|error| error.to_string())?;
    let marker_physical_host_id = row
        .try_get::<Option<String>, _>("ack_physical_host_id")
        .map_err(|error| error.to_string())?;
    let marker_tick = row
        .try_get::<Option<i64>, _>("ack_tick")
        .map_err(|error| error.to_string())?;
    let marker_next_sequence = row
        .try_get::<Option<i64>, _>("ack_next_sequence")
        .map_err(|error| error.to_string())?;
    let marker_revision = row
        .try_get::<Option<i64>, _>("ack_match_revision")
        .map_err(|error| error.to_string())?;
    let marker_snapshot_hash = row
        .try_get::<Option<String>, _>("ack_snapshot_hash")
        .map_err(|error| error.to_string())?;
    let marker_phase = row
        .try_get::<Option<String>, _>("ack_phase")
        .map_err(|error| error.to_string())?;
    let marker_result_hash = row
        .try_get::<Option<String>, _>("ack_result_hash")
        .map_err(|error| error.to_string())?;
    let marker_settlement_state = row
        .try_get::<Option<String>, _>("ack_settlement_state")
        .map_err(|error| error.to_string())?;
    let marker_acknowledged_at_unix_ms = row
        .try_get::<Option<i64>, _>("ack_acknowledged_at_unix_ms")
        .map_err(|error| error.to_string())?;
    let marker_cursors = row
        .try_get::<Option<Value>, _>("ack_next_input_sequences")
        .map_err(|error| error.to_string())?
        .map(serde_json::from_value::<BTreeMap<String, u64>>)
        .transpose()
        .map_err(|error| format!("decode terminal publication ACK cursors: {error}"))?;
    let terminal_exact = terminal_authority_matches_high_water(
        &DurableTerminalCompactionView {
            phase,
            simulation_tick: simulation.tick,
            simulation_terminal: simulation.terminal(),
            simulation_hash,
            durable_snapshot_hash,
            authoritative_tick: u64::try_from(authoritative_tick).ok(),
            next_sequence: u64::try_from(next_sequence).ok(),
            checkpoint_sequence: u64::try_from(checkpoint_sequence).ok(),
            match_revision: u64::try_from(match_revision).ok(),
            result_valid,
            settlement_state: settlement_state.clone(),
            next_input_sequences: member_cursors,
            assigned_instance_id: row
                .try_get("assigned_instance_id")
                .map_err(|error| error.to_string())?,
            assigned_instance_epoch: row
                .try_get("assigned_instance_epoch")
                .map_err(|error| error.to_string())?,
            assigned_physical_host_id: row
                .try_get("assigned_physical_host_id")
                .map_err(|error| error.to_string())?,
        },
        high_water,
    );
    let marker_tuple_exact = marker_generation == Some(high_water.actor_generation)
        && marker_instance_id.as_deref() == Some(high_water.instance_id.as_str())
        && marker_epoch == Some(high_water.actor_epoch)
        && marker_physical_host_id.as_deref() == Some(high_water.physical_host_id.as_str())
        && marker_tick.and_then(|value| u64::try_from(value).ok()) == Some(high_water.tick)
        && marker_next_sequence.and_then(|value| u64::try_from(value).ok())
            == Some(high_water.next_sequence)
        && marker_revision.and_then(|value| u64::try_from(value).ok())
            == Some(high_water.match_revision)
        && marker_cursors.as_ref() == Some(&high_water.next_input_sequences)
        && marker_snapshot_hash.as_deref() == Some(high_water.snapshot_hash.as_str())
        && marker_acknowledged_at_unix_ms.is_some_and(|value| value > 0)
        && terminal_marker_metadata_matches(
            marker_phase.as_deref(),
            marker_result_hash.as_deref(),
            marker_settlement_state.as_deref(),
            result_hash.as_deref(),
            &settlement_state,
        );
    let durable_generation_exact = durable_actor_generation == Some(high_water.actor_generation);
    let durable_publication_acknowledged = durable_publication_state == "acknowledged";
    let recovered_marker = if terminal_exact && marker_generation.is_none() && mutation_allowed {
        let result_hash = result_hash
            .as_deref()
            .ok_or_else(|| "terminal publication ACK result hash is missing".to_string())?;
        let inserted = sqlx::query::query(
            "insert into trnm_online_terminal_publication_acks (
                match_id, actor_generation, actor_epoch, authoritative_tick,
                next_sequence, match_revision, next_input_sequences, snapshot_hash,
                phase, result_hash, published_settlement_state,
                instance_id, physical_host_id, local_tombstone_state
             ) values ($1, $2, $3, $4, $5, $6, $7, $8, 'complete', $9, $10, $11, $12,
                       'hot_pending')
             on conflict (match_id) do nothing",
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
        .bind(result_hash)
        .bind(&settlement_state)
        .bind(&high_water.instance_id)
        .bind(&high_water.physical_host_id)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?
        .rows_affected()
            == 1;
        inserted
            || terminal_publication_marker_matches_high_water(
                &mut *transaction,
                high_water,
                result_hash,
                &settlement_state,
            )
            .await?
    } else {
        false
    };
    let bound_terminal_authority = if terminal_exact
        && (marker_tuple_exact || recovered_marker)
        && (!durable_generation_exact || !durable_publication_acknowledged)
        && mutation_allowed
    {
        let bound = sqlx::query::query(
            "update trnm_online_matches
             set terminal_publication_actor_generation = $2,
                 terminal_publication_state = 'acknowledged',
                 updated_at = now()
             where match_id = $1 and phase = 'complete'
               and (
                   terminal_publication_actor_generation is null
                   or terminal_publication_actor_generation = $2
               )",
        )
        .bind(high_water.match_id)
        .bind(high_water.actor_generation)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        if bound.rows_affected() != 1 {
            return Err(
                "terminal publication ACK conflicts with durable actor generation".to_string(),
            );
        }
        true
    } else {
        false
    };
    if recovered_marker || bound_terminal_authority {
        if let Some(state) = runtime_mutation_state {
            lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
        }
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    let durable_acknowledgement = terminal_exact
        && (marker_tuple_exact || recovered_marker)
        && (durable_generation_exact || bound_terminal_authority)
        && (durable_publication_acknowledged || bound_terminal_authority);
    if !durable_acknowledgement {
        return Ok(false);
    }
    let evidence =
        load_terminal_ack_database_evidence_by_match(&mut *connection, high_water.match_id)
            .await?
            .ok_or_else(|| "terminal ACK disappeared after durable revalidation".to_string())?;
    if !evidence.matches_high_water(high_water) {
        return Err("terminal ACK no longer matches its hot witness".to_string());
    }
    let lineage = database_lineage(&mut *connection).await?;
    let commit = TerminalPublicationCommit {
        result_hash: evidence.result_hash.clone(),
        settlement_state: evidence.settlement_state.clone(),
        acknowledged_at_unix_ms: evidence.acknowledged_at_unix_ms,
    };
    match evidence.local_tombstone_state.as_str() {
        "sealed" => {
            let tombstone = journal.ack_tombstone(high_water.match_id)?.ok_or_else(|| {
                "sealed terminal ACK is missing its durable cold tombstone".to_string()
            })?;
            if !evidence.matches_tombstone(&tombstone, &lineage) {
                return Err(
                    "sealed terminal ACK conflicts with its durable cold tombstone".to_string(),
                );
            }
            if !exact_terminal_publication_marker_exists(&mut *connection, high_water.match_id)
                .await?
            {
                return Ok(false);
            }
        }
        "legacy_bootstrap_pending" | "hot_pending" => {
            if !mutation_allowed {
                return Ok(false);
            }
            let tombstone =
                ensure_cold_terminal_ack(journal, high_water, &commit, &lineage).await?;
            seal_terminal_ack_in_database(
                &mut connection,
                &tombstone,
                runtime_mutation_state,
                database_host_authority,
            )
            .await?;
        }
        _ => return Err("terminal ACK has an unsupported local tombstone state".to_string()),
    }
    if let Some(state) = runtime_state {
        state
            .terminal_acknowledged_high_waters
            .write()
            .await
            .insert(high_water.match_id);
    }
    Ok(true)
}

struct StartupCommandRecoveryEvidence {
    simulation: MissionSimV1,
    accepted_revision: u64,
    player_id: String,
    input_sequence: u64,
    member_role: String,
    order: trnm_rts_protocol::RtsFrameOrder,
}

async fn load_startup_command_recovery_evidence(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    match_id: Uuid,
    sequence: u64,
    label: &str,
) -> Result<StartupCommandRecoveryEvidence, String> {
    let sequence = i64::try_from(sequence)
        .map_err(|_| format!("{label} command sequence exceeds PostgreSQL range"))?;
    let row = sqlx::query::query(
        "select c.player_id, c.input_sequence, c.post_simulation_json,
                c.accepted_snapshot_hash, c.accepted_match_revision,
                c.target_tick, c.client_observed_tick, c.order_json,
                mm.member_role
         from trnm_online_commands c
         join trnm_online_match_members mm
           on mm.match_id = c.match_id and mm.player_id = c.player_id
         where c.match_id = $1 and c.sequence = $2",
    )
    .bind(match_id)
    .bind(sequence)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| format!("{label} command recovery event is missing"))?;
    let simulation_value = row
        .try_get::<Option<Value>, _>("post_simulation_json")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("{label} command recovery simulation is missing"))?;
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_value)
        .map_err(|error| format!("decode {label} command recovery simulation: {error}"))?;
    let accepted_hash = row
        .try_get::<String, _>("accepted_snapshot_hash")
        .map_err(|error| error.to_string())?;
    validate_recovery_simulation(&simulation, Some(simulation.tick), &accepted_hash, label)?;
    let order_value = row
        .try_get::<Value, _>("order_json")
        .map_err(|error| error.to_string())?;
    validate_recovered_command_timing(
        &simulation,
        row.try_get::<i64, _>("target_tick")
            .map_err(|error| error.to_string())?,
        row.try_get::<Option<i64>, _>("client_observed_tick")
            .map_err(|error| error.to_string())?,
        order_value.clone(),
        label,
    )?;
    let input_sequence = row
        .try_get::<Option<i64>, _>("input_sequence")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("{label} command input cursor is missing"))?;
    Ok(StartupCommandRecoveryEvidence {
        simulation,
        accepted_revision: u64::try_from(
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
    Ok(final_high_water)
