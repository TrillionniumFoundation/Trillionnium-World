async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
            })?;
        let initial = serde_json::from_value::<MissionSimV1>(initial_value.clone())
            .map_err(|error| format!("decode running high-water initial simulation: {error}"))?;
        let frame = sqlx::query::query(
            "select tick, snapshot_hash, simulation_json from trnm_online_replay_frames
             where match_id = $1 and frame_kind = 'initial' order by tick limit 1",
        )
        .bind(high_water.match_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "running high-water recovery is missing initial replay frame".to_string())?;
        let frame_value = frame
            .try_get::<Value, _>("simulation_json")
            .map_err(|error| error.to_string())?;
        if frame_value != initial_value {
            return Err("initial simulation and replay frame disagree".to_string());
        }
        let initial_tick = u64::try_from(
            frame
                .try_get::<i64, _>("tick")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| "running high-water initial frame tick is negative".to_string())?;
        validate_recovery_simulation(
            &initial,
            Some(initial_tick),
            &frame
                .try_get::<String, _>("snapshot_hash")
                .map_err(|error| error.to_string())?,
            "running high-water initial state",
        )?;
        initial
    } else {
        let source = load_startup_command_recovery_evidence(
            &mut transaction,
            high_water.match_id,
            high_water.next_sequence.saturating_sub(1),
            "running high-water source",
        )
        .await?;
        if source.accepted_revision != high_water.match_revision
            || high_water
                .next_input_sequences
                .get(&source.player_id)
                .copied()
                != Some(source.input_sequence.saturating_add(1))
        {
            return Err(
                "running high-water source command cursor does not match journal".to_string(),
            );
        }
        source.simulation
    };

    let successor = if next_sequence == high_water.next_sequence {
        None
    } else {
        let successor = load_startup_command_recovery_evidence(
            &mut transaction,
            high_water.match_id,
            high_water.next_sequence,
            "terminal successor",
        )
        .await?;
        if successor.accepted_revision != match_revision
            || high_water
                .next_input_sequences
                .get(&successor.player_id)
                .copied()
                != Some(successor.input_sequence)
            || member_cursors.get(&successor.player_id).copied()
                != Some(successor.input_sequence.saturating_add(1))
        {
            return Err("terminal successor command cursor does not match authority".to_string());
        }
        Some(successor)
    };
    replay_running_high_water_to_terminal(
        source,
        high_water,
        successor.as_ref(),
        &terminal_simulation,
        &terminal_hash,
        &match_mode,
    )?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    journal
        .new_record(PublishedTickRecordInput {
            instance_id: high_water.instance_id.clone(),
            match_id: high_water.match_id,
            actor_generation: high_water.actor_generation,
            actor_epoch: high_water.actor_epoch,
            tick: terminal_simulation.tick,
            next_sequence,
            match_revision,
            next_input_sequences: member_cursors,
            phase: "complete".to_string(),
            receipts_replayable: true,
            snapshot_hash: terminal_hash,
        })
        .map(Some)
}

#[derive(Clone, Copy)]
struct FailedClosedMaintenanceExpectation<'a> {
    instance_id: &'a str,
    physical_host_id: &'a str,
    failure_reason: &'a str,
}

fn failed_closed_maintenance_expectation_matches(
    high_water: &PublishedTickHighWater,
    durable_failure_reason: &str,
    expected: FailedClosedMaintenanceExpectation<'_>,
) -> bool {
    high_water.instance_id == expected.instance_id
        && high_water.physical_host_id == expected.physical_host_id
        && durable_failure_reason == expected.failure_reason
}

async fn failed_closed_match_originates_before_v13(
    connection: &mut PgConnection,
    match_id: Uuid,
) -> Result<bool, String> {
    let migration_checksum = migration_checksum_sha256(MIGRATION_V13);
    sqlx::query_scalar::query_scalar::<_, bool>(
        "select m.updated_at < migration.applied_at
           from trnm_online_matches m
           join public.trnm_online_schema_migrations migration
             on migration.migration_version = 13
            and migration.migration_name = '0013_online_failed_closed_abandonment_v1'
            and migration.checksum_sha256 = $2
          where m.match_id = $1 and m.phase = 'failed_closed'",
    )
    .bind(match_id)
    .bind(migration_checksum)
    .fetch_optional(connection)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "failed-closed match or exact V13 migration ledger is missing".to_string())
}

async fn ensure_failed_closed_abandonment_marker(
    connection: &mut PgConnection,
    high_water: &PublishedTickHighWater,
    database_host_authority: &DatabaseHostAuthorityFence,
    adopt_legacy_pre_v13: bool,
    maintenance_expectation: Option<FailedClosedMaintenanceExpectation<'_>>,
) -> Result<Option<(AbandonmentDatabaseEvidence, DatabaseLineage, bool)>, String> {
    if high_water.phase != "running" || !high_water.receipts_replayable {
        return Ok(None);
    }
    let mut transaction = connection
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    let Some(row) = sqlx::query::query(
        "select m.phase, m.settlement_state, m.terminal_publication_state,
                m.simulation_json, m.snapshot_hash, m.authoritative_tick,
                m.next_sequence, m.checkpoint_sequence, m.match_revision,
                m.updated_at,
                m.assigned_instance_id,
                m.assigned_instance_epoch, m.assigned_physical_host_id,
                m.result_json, m.result_hash, m.failure_reason,
                m.terminal_publication_actor_generation,
                m.terminal_stage_simulation_json is null
                    and m.terminal_stage_result_json is null
                    and m.terminal_stage_result_hash is null
                    and m.terminal_stage_snapshot_hash is null
                    and m.terminal_stage_authoritative_tick is null
                    and m.terminal_stage_next_sequence is null
                    and m.terminal_stage_match_revision is null
                    and m.terminal_staged_at is null as terminal_stage_empty,
                exists(
                    select 1 from trnm_online_terminal_publication_acks a
                    where a.match_id = m.match_id
                ) as has_terminal_ack
           from trnm_online_matches m
          where m.match_id = $1
          for update",
    )
    .bind(high_water.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(None);
    };
    if row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?
        != "failed_closed"
    {
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(None);
    }
    let simulation_value = row
        .try_get::<Value, _>("simulation_json")
        .map_err(|error| error.to_string())?;
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_value)
        .map_err(|error| format!("decode failed-closed simulation: {error}"))?;
    let durable_tick = u64::try_from(
        row.try_get::<i64, _>("authoritative_tick")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "failed-closed authoritative tick is negative".to_string())?;
    let durable_next_sequence = u64::try_from(
        row.try_get::<i64, _>("next_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "failed-closed next sequence is negative".to_string())?;
    let durable_match_revision = u64::try_from(
        row.try_get::<i64, _>("match_revision")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "failed-closed match revision is negative".to_string())?;
    let durable_checkpoint_sequence = u64::try_from(
        row.try_get::<i64, _>("checkpoint_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
    Ok(final_high_water)
