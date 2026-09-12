async fn run_match_fence_monitor(
    state: AppState,
    match_id: Uuid,
    publication_permit: watch::Sender<bool>,
    fenced: watch::Sender<bool>,
) {
    let mut interval = tokio::time::interval(MATCH_ACTOR_FENCE_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = fenced.closed() => return,
            _ = interval.tick() => {
                let owns_match = tokio::time::timeout(
                    MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                    sqlx::query_scalar::query_scalar::<_, bool>(MATCH_ACTOR_FENCE_OWNERSHIP_SQL)
                .bind(match_id)
                .bind(state.instance_id.as_str())
                .bind(state.instance_epoch)
                .bind(state.physical_host_id.as_str())
                .fetch_one(&state.pool),
                )
                .await
                .map_err(|_| "match actor fence query exceeded its hard timeout".to_string())
                .and_then(|result| result.map_err(|error| error.to_string()));
                match owns_match {
                    Ok(true) => {}
                    Ok(false) => {
                        publication_permit.send_replace(false);
                        fenced.send_replace(true);
                        return;
                    }
                    Err(error) => {
                        tracing::error!(%match_id, %error, "match actor fence check failed closed");
                        publication_permit.send_replace(false);
                        fenced.send_replace(true);
                        return;
                    }
                }
            }
        }
    }
}

async fn run_match_checkpoint_writer(
    state: AppState,
    match_id: Uuid,
    mut checkpoints: mpsc::Receiver<MatchCheckpointJob>,
    failed: watch::Sender<Option<String>>,
    durable_recovery_tick: watch::Sender<u64>,
) {
    while let Some(mut job) = checkpoints.recv().await {
        let result = tokio::time::timeout(MATCH_ACTOR_WORKER_OPERATION_TIMEOUT, async {
            if job.terminal {
                persist_terminal_actor_checkpoint(&state, match_id, &job).await
            } else {
                persist_actor_checkpoint(&state, match_id, &job).await
            }
        })
        .await
        .unwrap_or_else(|_| Err("checkpoint persistence exceeded its hard timeout".to_string()));
        if let Some(completion) = job.completion.take() {
            let _ = completion.send(result.clone());
        }
        if let Err(error) = result {
            tracing::error!(%match_id, %error, terminal = job.terminal, "match actor checkpoint failed");
            failed.send_replace(Some(error));
            break;
        }
        let durable_tick = (*durable_recovery_tick.borrow()).max(job.simulation.tick);
        durable_recovery_tick.send_replace(durable_tick);
        // Keep the worker alive until the actor drains it. Closing this sender
        // immediately after a successful terminal checkpoint races the final
        // terminal journal/publication ACK and would look like a checkpoint
        // failure to the actor's fail-closed select loop.
    }
}

async fn persist_actor_checkpoint(
    state: &AppState,
    match_id: Uuid,
    job: &MatchCheckpointJob,
) -> Result<(), String> {
    let simulation_json =
        serde_json::to_value(&job.simulation).map_err(|error| error.to_string())?;
    if !state.database_host_authority.is_healthy() {
        return Err("PostgreSQL host authority is fail-closed".to_string());
    }
    let identity = &state.database_host_authority.identity;
    let persisted = sqlx::query_scalar::query_scalar::<_, bool>(ONLINE_CHECKPOINT_ACTOR_V1_SQL)
        .bind(match_id)
        .bind(job.simulation.tick as i64)
        .bind(&job.snapshot_hash)
        .bind(&simulation_json)
        .bind(job.next_sequence as i64)
        .bind(job.match_revision as i64)
        .bind(state.instance_id.as_str())
        .bind(state.instance_epoch)
        .bind(state.physical_host_id.as_str())
        .bind(identity.owner_nonce)
        .bind(&identity.application_name)
        .bind(identity.backend_pid)
        .bind(identity.backend_started_at)
        .bind(&identity.database_system_identifier)
        .bind(identity.database_timeline_id)
        .bind(identity.database_postmaster_started_at)
        .bind(identity.leader_lock_key)
        .bind(identity.barrier_lock_key)
        .fetch_one(&state.pool)
        .await
        .map_err(|error| error.to_string())?;
    if !persisted {
        return Err("checkpoint was fenced or superseded".to_string());
    }
    Ok(())
}

fn staged_terminal_authority_from_row(
    row: &sqlx_postgres::PgRow,
    match_mode: &str,
) -> Result<Option<StagedTerminalAuthority>, String> {
    let simulation_value = row
        .try_get::<Option<Value>, _>("terminal_stage_simulation_json")
        .map_err(|error| error.to_string())?;
    let result_value = row
        .try_get::<Option<Value>, _>("terminal_stage_result_json")
        .map_err(|error| error.to_string())?;
    let result_hash = row
        .try_get::<Option<String>, _>("terminal_stage_result_hash")
        .map_err(|error| error.to_string())?;
    let snapshot_hash = row
        .try_get::<Option<String>, _>("terminal_stage_snapshot_hash")
        .map_err(|error| error.to_string())?;
    let authoritative_tick = row
        .try_get::<Option<i64>, _>("terminal_stage_authoritative_tick")
        .map_err(|error| error.to_string())?;
    let next_sequence = row
        .try_get::<Option<i64>, _>("terminal_stage_next_sequence")
        .map_err(|error| error.to_string())?;
    let match_revision = row
        .try_get::<Option<i64>, _>("terminal_stage_match_revision")
        .map_err(|error| error.to_string())?;
    if simulation_value.is_none()
        && result_value.is_none()
        && result_hash.is_none()
        && snapshot_hash.is_none()
        && authoritative_tick.is_none()
        && next_sequence.is_none()
        && match_revision.is_none()
    {
        return Ok(None);
    }
    let simulation = serde_json::from_value::<MissionSimV1>(
        simulation_value.ok_or_else(|| "terminal stage simulation is missing".to_string())?,
    )
    .map_err(|error| format!("decode terminal stage simulation: {error}"))?;
    if !simulation.terminal() {
        return Err("terminal stage simulation is not terminal".to_string());
    }
    let result = serde_json::from_value::<BattleResultV1>(
        result_value.ok_or_else(|| "terminal stage result is missing".to_string())?,
    )
    .map_err(|error| format!("decode terminal stage result: {error}"))?;
    let result_hash =
        result_hash.ok_or_else(|| "terminal stage result hash is missing".to_string())?;
    let snapshot_hash =
        snapshot_hash.ok_or_else(|| "terminal stage snapshot hash is missing".to_string())?;
    let authoritative_tick = u64::try_from(
        authoritative_tick.ok_or_else(|| "terminal stage tick is missing".to_string())?,
    )
    .map_err(|_| "terminal stage tick is negative".to_string())?;
    let next_sequence = u64::try_from(
        next_sequence.ok_or_else(|| "terminal stage sequence is missing".to_string())?,
    )
    .map_err(|_| "terminal stage sequence is negative".to_string())?;
    let match_revision = u64::try_from(
        match_revision.ok_or_else(|| "terminal stage revision is missing".to_string())?,
    )
    .map_err(|_| "terminal stage revision is negative".to_string())?;
    let computed_snapshot_hash = simulation
        .snapshot_hash()
        .map_err(|error| format!("hash terminal stage simulation: {error}"))?;
    let (expected_result, expected_result_hash) = derive_terminal_result(&simulation, match_mode)?;
    if simulation.tick != authoritative_tick
        || computed_snapshot_hash != snapshot_hash
        || result != expected_result
        || result_hash != expected_result_hash
    {
        return Err("terminal stage does not match its deterministic result authority".to_string());
    }
    Ok(Some(StagedTerminalAuthority {
        simulation,
        result,
        result_hash,
        snapshot_hash,
        authoritative_tick,
        next_sequence,
        match_revision,
    }))
}

async fn finalize_staged_terminal_authority(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    expected: TerminalFinalizationExpectation<'_>,
) -> Result<Option<TerminalPublicationCommit>, String> {
    let row = sqlx::query::query(
        "select phase, match_mode, next_sequence, match_revision,
                assigned_instance_id, assigned_instance_epoch, assigned_physical_host_id,
                terminal_stage_simulation_json, terminal_stage_result_json,
                terminal_stage_result_hash, terminal_stage_snapshot_hash,
                terminal_stage_authoritative_tick, terminal_stage_next_sequence,
                terminal_stage_match_revision
         from trnm_online_matches where match_id = $1 for update",
    )
    .bind(expected.match_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "terminal finalization match disappeared".to_string())?;
    let phase = row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?;
    if phase != "running" {
        return Ok(None);
    }
    let match_mode = row
        .try_get::<String, _>("match_mode")
        .map_err(|error| error.to_string())?;
    let staged = staged_terminal_authority_from_row(&row, &match_mode)?
        .ok_or_else(|| "terminal finalization has no private staged authority".to_string())?;
    let assigned_instance = row
        .try_get::<Option<String>, _>("assigned_instance_id")
        .map_err(|error| error.to_string())?;
    let assigned_epoch = row
        .try_get::<i64, _>("assigned_instance_epoch")
        .map_err(|error| error.to_string())?;
    let assigned_physical_host = row
        .try_get::<Option<String>, _>("assigned_physical_host_id")
        .map_err(|error| error.to_string())?;
    let durable_next_sequence = u64::try_from(
        row.try_get::<i64, _>("next_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal finalization sequence is negative".to_string())?;
    let durable_revision = u64::try_from(
        row.try_get::<i64, _>("match_revision")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal finalization revision is negative".to_string())?;
    let member_rows = sqlx::query::query(
        "select player_id, next_input_sequence from trnm_online_match_members
         where match_id = $1 order by player_id for update",
    )
    .bind(expected.match_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut durable_cursors = BTreeMap::new();
    for member in member_rows {
        durable_cursors.insert(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            u64::try_from(
                member
                    .try_get::<i64, _>("next_input_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "terminal finalization member cursor is negative".to_string())?,
        );
    }
    if !match_assignment_matches_local(
        assigned_instance.as_deref(),
        assigned_epoch,
        assigned_physical_host.as_deref(),
        expected.instance_id,
        expected.actor_epoch,
        expected.physical_host_id,
    ) || staged.authoritative_tick != expected.authoritative_tick
        || staged.next_sequence != expected.next_sequence
        || staged.match_revision != expected.match_revision
        || staged.snapshot_hash != expected.snapshot_hash
        || expected
            .result_hash
            .is_some_and(|result_hash| result_hash != staged.result_hash)
        || durable_next_sequence != expected.next_sequence
        || durable_revision != expected.match_revision
        || durable_cursors != *expected.next_input_sequences
    {
        return Err(
            "terminal finalization was fenced or differed from its exact staged/HWM authority"
                .to_string(),
        );
    }

    let any_pending = apply_member_progression(
        transaction,
        expected.match_id,
        &staged.result,
        &staged.result_hash,
    )
    .await?;
    product_v2::apply_ranked_result(
        transaction,
        expected.match_id,
        staged.result.outcome,
        &staged.result_hash,
    )
    .await?;
    operations_v1::finalize_ranked_operations(
        transaction,
        expected.match_id,
        staged.result.outcome,
        &staged.result_hash,
        &staged.snapshot_hash,
    )
    .await?;
    sqlx::query::query(
        "insert into trnm_online_replay_frames (
            match_id, tick, snapshot_hash, simulation_json, frame_kind
         ) values ($1, $2, $3, $4, 'terminal')
         on conflict (match_id, tick) do update set
            snapshot_hash = excluded.snapshot_hash,
            simulation_json = excluded.simulation_json,
            frame_kind = excluded.frame_kind",
    )
    .bind(expected.match_id)
    .bind(staged.authoritative_tick as i64)
    .bind(&staged.snapshot_hash)
    .bind(serde_json::to_value(&staged.simulation).map_err(|error| error.to_string())?)
    .execute(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let settlement_state = if any_pending { "pending" } else { "settled" };
    let updated = sqlx::query::query(
        "update trnm_online_matches set phase = 'complete',
            simulation_json = terminal_stage_simulation_json,
            result_json = terminal_stage_result_json,
            result_hash = terminal_stage_result_hash,
            snapshot_hash = terminal_stage_snapshot_hash,
            authoritative_tick = terminal_stage_authoritative_tick,
            checkpoint_sequence = terminal_stage_next_sequence,
            settlement_state = $2,
            terminal_stage_simulation_json = null,
            terminal_stage_result_json = null,
            terminal_stage_result_hash = null,
            terminal_stage_snapshot_hash = null,
            terminal_stage_authoritative_tick = null,
            terminal_stage_next_sequence = null,
            terminal_stage_match_revision = null,
            terminal_staged_at = null,
            terminal_publication_actor_generation = $9,
            terminal_publication_state = 'acknowledged',
            updated_at = now()
         where match_id = $1 and phase = 'running'
           and assigned_instance_id = $3 and assigned_instance_epoch = $4
           and assigned_physical_host_id = $5
           and next_sequence = $6 and match_revision = $7
           and terminal_stage_snapshot_hash = $8",
    )
    .bind(expected.match_id)
    .bind(settlement_state)
    .bind(expected.instance_id)
    .bind(expected.actor_epoch)
    .bind(expected.physical_host_id)
    .bind(expected.next_sequence as i64)
    .bind(expected.match_revision as i64)
    .bind(expected.snapshot_hash)
    .bind(expected.actor_generation)
    .execute(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    if updated.rows_affected() != 1 {
        return Err("terminal finalization was fenced before commit".to_string());
    }
    let marker = sqlx::query::query(
        "insert into trnm_online_terminal_publication_acks (
            match_id, actor_generation, actor_epoch, authoritative_tick,
            next_sequence, match_revision, next_input_sequences, snapshot_hash,
            phase, result_hash, published_settlement_state,
            instance_id, physical_host_id, local_tombstone_state
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, 'complete', $9, $10, $11, $12,
                   'hot_pending')
         on conflict (match_id) do update set
            published_settlement_state = excluded.published_settlement_state
         where trnm_online_terminal_publication_acks.actor_generation = excluded.actor_generation
           and trnm_online_terminal_publication_acks.instance_id = excluded.instance_id
           and trnm_online_terminal_publication_acks.actor_epoch = excluded.actor_epoch
           and trnm_online_terminal_publication_acks.physical_host_id = excluded.physical_host_id
           and trnm_online_terminal_publication_acks.authoritative_tick = excluded.authoritative_tick
           and trnm_online_terminal_publication_acks.next_sequence = excluded.next_sequence
           and trnm_online_terminal_publication_acks.match_revision = excluded.match_revision
           and trnm_online_terminal_publication_acks.next_input_sequences = excluded.next_input_sequences
           and trnm_online_terminal_publication_acks.snapshot_hash = excluded.snapshot_hash
           and trnm_online_terminal_publication_acks.phase = excluded.phase
           and trnm_online_terminal_publication_acks.result_hash = excluded.result_hash
           and (
               trnm_online_terminal_publication_acks.published_settlement_state = excluded.published_settlement_state
               or (
                   trnm_online_terminal_publication_acks.published_settlement_state = 'pending'
                   and excluded.published_settlement_state = 'settled'
               )
           )
         returning floor(extract(epoch from acknowledged_at) * 1000)::bigint
                   as acknowledged_at_unix_ms",
    )
    .bind(expected.match_id)
    .bind(expected.actor_generation)
    .bind(expected.actor_epoch)
    .bind(expected.authoritative_tick as i64)
    .bind(expected.next_sequence as i64)
    .bind(expected.match_revision as i64)
    .bind(
        serde_json::to_value(expected.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(expected.snapshot_hash)
    .bind(&staged.result_hash)
    .bind(settlement_state)
    .bind(expected.instance_id)
    .bind(expected.physical_host_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let Some(marker) = marker else {
        return Err(
            "terminal finalization conflicts with immutable published evidence".to_string(),
        );
    };
    let acknowledged_at_unix_ms = u64::try_from(
        marker
            .try_get::<i64, _>("acknowledged_at_unix_ms")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal ACK acknowledgement timestamp is not positive".to_string())?;
    if acknowledged_at_unix_ms == 0 {
        return Err("terminal ACK acknowledgement timestamp is not positive".to_string());
    }
    Ok(Some(TerminalPublicationCommit {
        result_hash: staged.result_hash,
        settlement_state: settlement_state.to_string(),
        acknowledged_at_unix_ms,
    }))
}

async fn persist_terminal_actor_checkpoint(
    state: &AppState,
    match_id: Uuid,
    job: &MatchCheckpointJob,
) -> Result<(), String> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    lock_current_fleet_epoch(&mut transaction, state, true).await?;
    let row = sqlx::query::query(
        "select phase, match_mode, next_sequence, match_revision,
                assigned_instance_id, assigned_instance_epoch,
                assigned_physical_host_id,
                terminal_stage_simulation_json, terminal_stage_result_json,
                terminal_stage_result_hash, terminal_stage_snapshot_hash,
                terminal_stage_authoritative_tick, terminal_stage_next_sequence,
                terminal_stage_match_revision
         from trnm_online_matches where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "terminal match disappeared".to_string())?;
    let phase: String = row.try_get("phase").map_err(|error| error.to_string())?;
    let assigned_instance: Option<String> = row
        .try_get("assigned_instance_id")
        .map_err(|error| error.to_string())?;
    let assigned_epoch: i64 = row
        .try_get("assigned_instance_epoch")
        .map_err(|error| error.to_string())?;
    let assigned_physical_host: Option<String> = row
        .try_get("assigned_physical_host_id")
        .map_err(|error| error.to_string())?;
    let durable_next_sequence = row
        .try_get::<i64, _>("next_sequence")
        .map_err(|error| error.to_string())? as u64;
    let durable_revision = row
        .try_get::<i64, _>("match_revision")
        .map_err(|error| error.to_string())? as u64;
    let assignment_is_local = match_assignment_matches_local(
        assigned_instance.as_deref(),
        assigned_epoch,
        assigned_physical_host.as_deref(),
        state.instance_id.as_str(),
        state.instance_epoch,
        state.physical_host_id.as_str(),
    );
    if phase == "complete" {
        if !assignment_is_local
            || durable_next_sequence != job.next_sequence
            || durable_revision != job.match_revision
        {
            return Err("completed terminal checkpoint belongs to fenced authority".to_string());
        }
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(());
    }
    if phase != "running"
        || !assignment_is_local
        || durable_next_sequence != job.next_sequence
        || durable_revision != job.match_revision
    {
        return Err("terminal checkpoint was fenced by newer authority".to_string());
    }
    let match_mode: String = row
        .try_get("match_mode")
        .map_err(|error| error.to_string())?;
    let simulation_json =
        serde_json::to_value(&job.simulation).map_err(|error| error.to_string())?;
    let (result, result_hash) = derive_terminal_result(&job.simulation, &match_mode)?;
    if let Some(staged) = staged_terminal_authority_from_row(&row, &match_mode)? {
        if staged.simulation != job.simulation
            || staged.result != result
            || staged.result_hash != result_hash
            || staged.snapshot_hash != job.snapshot_hash
            || staged.authoritative_tick != job.simulation.tick
            || staged.next_sequence != job.next_sequence
            || staged.match_revision != job.match_revision
        {
            return Err("terminal checkpoint conflicts with an existing private stage".to_string());
        }
        lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(());
    }
    let updated = sqlx::query::query(
        "update trnm_online_matches set
            terminal_stage_simulation_json = $2,
            terminal_stage_result_json = $3,
            terminal_stage_result_hash = $4,
            terminal_stage_snapshot_hash = $5,
            terminal_stage_authoritative_tick = $6,
            terminal_stage_next_sequence = $7,
            terminal_stage_match_revision = $8,
            terminal_staged_at = now(), updated_at = now()
         where match_id = $1 and phase = 'running' and assigned_instance_id = $9
           and assigned_instance_epoch = $10 and next_sequence = $7
           and assigned_physical_host_id = $12 and match_revision = $11
           and terminal_stage_snapshot_hash is null
           and exists (
             select 1 from trnm_online_fleet_instances f
             where f.instance_id = $9 and f.instance_epoch = $10
               and f.physical_host_id = $12
               and f.status in ('active', 'draining')
               and f.lease_expires_at > now()
           )",
    )
    .bind(match_id)
    .bind(simulation_json)
    .bind(serde_json::to_value(result).map_err(|error| error.to_string())?)
    .bind(result_hash)
    .bind(&job.snapshot_hash)
    .bind(job.simulation.tick as i64)
    .bind(job.next_sequence as i64)
    .bind(job.match_revision as i64)
    .bind(state.instance_id.as_str())
    .bind(state.instance_epoch)
    .bind(job.match_revision as i64)
    .bind(state.physical_host_id.as_str())
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if updated.rows_affected() != 1 {
        return Err("terminal private staging was fenced by a newer fleet epoch".to_string());
    }
    lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())
}

async fn persist_terminal_publication_ack(
    state: &AppState,
    match_id: Uuid,
    actor_generation: Uuid,
    loaded: &LoadedMatchActor,
    evidence: &TerminalPublicationEvidence,
) -> Result<TerminalPublicationCommit, String> {
    let mut connection = state
        .pool
        .acquire()
        .await
        .map_err(|error| error.to_string())?;
    let mut transaction = (*connection)
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    lock_current_fleet_epoch(&mut transaction, state, true).await?;
    let loaded_hash = loaded
        .simulation
        .snapshot_hash()
        .map_err(|error| error.to_string())?;
    if evidence.phase != OnlineMatchPhase::Complete
        || evidence.result_hash.is_none()
        || evidence.authoritative_tick != loaded.simulation.tick
        || evidence.next_sequence != loaded.next_sequence
        || evidence.match_revision != loaded.match_revision
        || evidence.next_input_sequences != loaded.next_input_sequences
        || evidence.snapshot_hash != loaded_hash
    {
        return Err(
            "terminal publication ACK did not match its staged actor authority".to_string(),
        );
    }
    let commit = finalize_staged_terminal_authority(
        &mut transaction,
        TerminalFinalizationExpectation {
            match_id,
            actor_generation,
            actor_epoch: state.instance_epoch,
            instance_id: state.instance_id.as_str(),
            physical_host_id: state.physical_host_id.as_str(),
            authoritative_tick: evidence.authoritative_tick,
            next_sequence: evidence.next_sequence,
            match_revision: evidence.match_revision,
            next_input_sequences: &evidence.next_input_sequences,
            snapshot_hash: &evidence.snapshot_hash,
            result_hash: evidence.result_hash.as_deref(),
        },
    )
    .await?
    .ok_or_else(|| "terminal publication ACK found no private staged authority".to_string())?;
    lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    // Observe a post-commit WAL flush bound on this exact session, convert
    // the exact hot witness into a cold ACK tombstone, and only then publish
    // the DB marker by a second fenced transaction on the same connection.
    let lineage = database_lineage(&mut *connection).await?;
    let high_water =
        if let Some(tombstone) = state.published_tick_journal.ack_tombstone(match_id)? {
            tombstone.high_water
        } else {
            state
                .published_tick_journal
                .high_water(match_id)?
                .ok_or_else(|| "terminal publication ACK lost its exact hot witness".to_string())?
        };
    if high_water.instance_id != *state.instance_id
        || high_water.physical_host_id != *state.physical_host_id
        || high_water.match_id != match_id
        || high_water.actor_generation != actor_generation
        || high_water.actor_epoch != state.instance_epoch
        || high_water.tick != evidence.authoritative_tick
        || high_water.next_sequence != evidence.next_sequence
        || high_water.match_revision != evidence.match_revision
        || high_water.next_input_sequences != evidence.next_input_sequences
        || high_water.phase != "complete"
        || !high_water.receipts_replayable
        || high_water.snapshot_hash != evidence.snapshot_hash
    {
        return Err("terminal publication ACK hot witness is not exact".to_string());
    }
    let tombstone = ensure_cold_terminal_ack(
        &state.published_tick_journal,
        &high_water,
        &commit,
        &lineage,
    )
    .await?;
    seal_terminal_ack_in_database(
        &mut connection,
        &tombstone,
        Some(state),
        &state.database_host_authority,
    )
    .await?;
    state
        .terminal_acknowledged_high_waters
        .write()
        .await
        .insert(match_id);
    Ok(commit)
}

/// Compatibility API retained only to fail closed for downstream callers.
///
/// Terminal economic settlement is owned by the independently deployed
/// `trnm-settlement-worker`. The game-server process must never execute signer
/// or CEX I/O, mutate campaign economic queues, or advance the terminal
/// settlement marker itself.
pub async fn settle_pending_matches(_state: &AppState, _limit: i64) -> Result<u64, String> {
    Err(
        "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited"
            .to_string(),
    )
}

