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
//D01
        );
    }
//D02
    )?;

//D03
        high_water,
    );
//D04
            &settlement_state,
        );
//D05
        }
    }
//D06
    Ok(true)
}
//D07
        label,
    )?;
//D08
        );
    }
    Ok(())
}

async fn recover_terminal_high_water_after_running_crash(
    pool: &PgPool,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<Option<PublishedTickHighWater>, String> {
    if high_water.phase != "running" || !high_water.receipts_replayable {
        return Ok(None);
    }
    let mut transaction = pool.begin().await.map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    let Some(row) = sqlx::query::query(
        "select phase, simulation_json, initial_simulation_json, snapshot_hash,
                authoritative_tick, next_sequence, match_revision, checkpoint_sequence,
                result_json, result_hash, settlement_state, match_mode,
                assigned_instance_id, assigned_instance_epoch, assigned_physical_host_id,
                terminal_stage_simulation_json, terminal_stage_result_json,
                terminal_stage_result_hash, terminal_stage_snapshot_hash,
                terminal_stage_authoritative_tick, terminal_stage_next_sequence,
                terminal_stage_match_revision
         from trnm_online_matches where match_id = $1 for share",
    )
    .bind(high_water.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let durable_phase = row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?;
    let match_mode = row
        .try_get::<String, _>("match_mode")
        .map_err(|error| error.to_string())?;
    let (
        terminal_simulation,
        terminal_hash,
        durable_hash,
        authoritative_tick,
        next_sequence,
        match_revision,
        checkpoint_sequence,
        result_valid,
        terminal_settlement_state,
    ) = if durable_phase == "complete" {
        let terminal_value = row
            .try_get::<Option<Value>, _>("simulation_json")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "completed match is missing terminal simulation".to_string())?;
        let terminal_simulation = serde_json::from_value::<MissionSimV1>(terminal_value)
            .map_err(|error| format!("decode completed terminal simulation: {error}"))?;
        let terminal_hash = terminal_simulation
            .snapshot_hash()
            .map_err(|error| format!("hash completed terminal simulation: {error}"))?;
        let result_hash = row
            .try_get::<Option<String>, _>("result_hash")
            .map_err(|error| error.to_string())?;
        let result_valid = terminal_result_matches_simulation(
            &terminal_simulation,
            row.try_get::<Option<Value>, _>("result_json")
                .map_err(|error| error.to_string())?,
            result_hash.as_deref(),
            &match_mode,
        )?;
        (
            terminal_simulation,
            terminal_hash,
            row.try_get::<String, _>("snapshot_hash")
                .map_err(|error| error.to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("authoritative_tick")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed terminal tick is negative".to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("next_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed terminal sequence is negative".to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("match_revision")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed terminal revision is negative".to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("checkpoint_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed checkpoint sequence is negative".to_string())?,
            result_valid,
            row.try_get::<String, _>("settlement_state")
                .map_err(|error| error.to_string())?,
        )
    } else if durable_phase == "running" {
        let Some(staged) = staged_terminal_authority_from_row(&row, &match_mode)? else {
            transaction
                .commit()
                .await
                .map_err(|error| error.to_string())?;
            return Ok(None);
        };
        (
//D0B
    }

//D0C
    };

//D0D
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
//D0F
          for update",
    )
//D10
            .is_none()
        && row
//D11
            ));
        }
//D12
}

//D13
}

//D14
    )
    .await?;
//D15
          for share",
    )
//D16
            .is_none()
        && row
//D17
