@@TRNM_R18_T1_SEGMENT_0@@
    )
    .map_err(|error| format!("decode terminal ACK input cursors: {error}"))?;
    Ok(TerminalAckDatabaseEvidence {
        match_id: row.try_get("match_id").map_err(|error| error.to_string())?,
        actor_generation: row
            .try_get("actor_generation")
            .map_err(|error| error.to_string())?,
        instance_id: row
            .try_get("instance_id")
            .map_err(|error| error.to_string())?,
        actor_epoch: row
            .try_get("actor_epoch")
            .map_err(|error| error.to_string())?,
        physical_host_id: row
            .try_get("physical_host_id")
            .map_err(|error| error.to_string())?,
        authoritative_tick,
        next_sequence,
        match_revision,
        next_input_sequences,
        snapshot_hash: row
            .try_get("snapshot_hash")
            .map_err(|error| error.to_string())?,
        result_hash: row
            .try_get("result_hash")
            .map_err(|error| error.to_string())?,
        settlement_state: row
            .try_get("published_settlement_state")
            .map_err(|error| error.to_string())?,
        local_tombstone_state: row
            .try_get("local_tombstone_state")
            .map_err(|error| error.to_string())?,
        acknowledged_at_unix_ms,
    })
}

async fn database_lineage<'e, E>(executor: E) -> Result<DatabaseLineage, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let row = sqlx::query::query(DATABASE_LINEAGE_SQL)
        .fetch_one(executor)
        .await
        .map_err(|error| format!("read PostgreSQL lineage after terminal ACK commit: {error}"))?;
    let timeline_id = u32::try_from(
        row.try_get::<i32, _>("timeline_id")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "PostgreSQL timeline identifier is not positive".to_string())?;
    if timeline_id == 0 {
        return Err("PostgreSQL timeline identifier is not positive".to_string());
    }
    let system_identifier: String = row
        .try_get("system_identifier")
        .map_err(|error| error.to_string())?;
    if !canonical_database_system_identifier(&system_identifier) {
        return Err("PostgreSQL system identifier is not canonical".to_string());
    }
    let wal_flush_lsn: String = row
        .try_get("wal_flush_lsn")
        .map_err(|error| error.to_string())?;
    if parse_postgres_wal_lsn(&wal_flush_lsn).is_none_or(|lsn| lsn == 0) {
        return Err("PostgreSQL WAL flush LSN is not canonical and positive".to_string());
    }
    Ok(DatabaseLineage {
        system_identifier,
        timeline_id,
        wal_flush_lsn,
    })
}

async fn load_cold_witness_database_summary<'e, E>(
    executor: E,
    physical_host_id: &str,
) -> Result<ColdWitnessDatabaseSummary, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let row = sqlx::query::query(
        "select terminal_total_count, terminal_sealed_count,
                abandonment_total_count, abandonment_sealed_count
           from trnm_online_local_cold_witness_summaries
          where physical_host_id = $1",
    )
    .bind(physical_host_id)
    .fetch_optional(executor)
    .await
    .map_err(|error| error.to_string())?;
    let Some(row) = row else {
        return Ok(ColdWitnessDatabaseSummary::default());
    };
    let read_count = |column| -> Result<u64, String> {
        u64::try_from(
            row.try_get::<i64, _>(column)
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| format!("cold witness summary {column} is negative"))
    };
    let summary = ColdWitnessDatabaseSummary {
        terminal_total_count: read_count("terminal_total_count")?,
        terminal_sealed_count: read_count("terminal_sealed_count")?,
        abandonment_total_count: read_count("abandonment_total_count")?,
        abandonment_sealed_count: read_count("abandonment_sealed_count")?,
    };
    if summary.terminal_sealed_count > summary.terminal_total_count
        || summary.abandonment_sealed_count > summary.abandonment_total_count
    {
        return Err("cold witness summary sealed count exceeds its total".to_string());
    }
    Ok(summary)
}

async fn load_readiness_database_summary(
    pool: &PgPool,
    instance_id: &str,
    instance_epoch: i64,
    physical_host_id: &str,
) -> Result<ReadinessDatabaseSummary, String> {
    let row = sqlx::query::query(READINESS_DATABASE_SUMMARY_SQL)
        .bind(instance_id)
        .bind(instance_epoch)
        .bind(physical_host_id)
        .bind(TERMINAL_ACK_GAP_SCAN_LIMIT.saturating_add(1))
        .fetch_one(pool)
        .await
        .map_err(|error| error.to_string())?;
    let read_nonnegative = |column| -> Result<u64, String> {
        u64::try_from(
            row.try_get::<i64, _>(column)
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| format!("readiness database summary {column} is negative"))
    };
    let cold_witness = ColdWitnessDatabaseSummary {
        terminal_total_count: read_nonnegative("terminal_total_count")?,
        terminal_sealed_count: read_nonnegative("terminal_sealed_count")?,
        abandonment_total_count: read_nonnegative("abandonment_total_count")?,
        abandonment_sealed_count: read_nonnegative("abandonment_sealed_count")?,
    };
    if cold_witness.terminal_sealed_count > cold_witness.terminal_total_count
        || cold_witness.abandonment_sealed_count > cold_witness.abandonment_total_count
    {
        return Err("readiness cold witness summary sealed count exceeds its total".to_string());
    }
    Ok(ReadinessDatabaseSummary {
        postgres_healthy: row
            .try_get("postgres_healthy")
            .map_err(|error| error.to_string())?,
        fleet_epoch_current: row
            .try_get("fleet_epoch_current")
            .map_err(|error| error.to_string())?,
        healthy_fleet_instances: row
            .try_get("healthy_fleet_instances")
            .map_err(|error| error.to_string())?,
        active_matches: row
            .try_get("active_matches")
            .map_err(|error| error.to_string())?,
        terminal_ack_gap_count: usize::try_from(read_nonnegative("terminal_ack_gap_count")?)
            .map_err(|_| "readiness terminal ACK gap count exceeds usize".to_string())?,
        terminal_ack_gap_match_ids: row
            .try_get("terminal_ack_gap_match_ids")
            .map_err(|error| error.to_string())?,
        pending_terminal_seal_match_ids: row
            .try_get("pending_terminal_seal_match_ids")
            .map_err(|error| error.to_string())?,
        pending_abandonment_seal_match_ids: row
            .try_get("pending_abandonment_seal_match_ids")
            .map_err(|error| error.to_string())?,
        historical_projection: HistoricalTerminalProjectionQuarantine {
            legacy_terminal_match_count: row
                .try_get("legacy_terminal_match_count")
                .map_err(|error| error.to_string())?,
            campaign_projection_polluted: row
                .try_get("campaign_projection_polluted")
                .map_err(|error| error.to_string())?,
            rating_projection_polluted: row
                .try_get("rating_projection_polluted")
                .map_err(|error| error.to_string())?,
        },
        cold_witness,
    })
}

fn abandonment_database_evidence_from_row(
    row: &sqlx_postgres::PgRow,
) -> Result<AbandonmentDatabaseEvidence, String> {
    if !row
        .try_get::<bool, _>("tuple_exact")
        .map_err(|error| error.to_string())?
    {
        return Err(
            "failed-closed abandonment marker does not match durable authority".to_string(),
        );
    }
    let authoritative_tick = u64::try_from(
        row.try_get::<i64, _>("authoritative_tick")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "abandonment authoritative tick is negative".to_string())?;
    let next_sequence = u64::try_from(
        row.try_get::<i64, _>("next_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "abandonment next sequence is negative".to_string())?;
    let match_revision = u64::try_from(
        row.try_get::<i64, _>("match_revision")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "abandonment match revision is negative".to_string())?;
    let abandoned_at_unix_ms = u64::try_from(
        row.try_get::<i64, _>("abandoned_at_unix_ms")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "abandonment timestamp is negative".to_string())?;
    if abandoned_at_unix_ms == 0 {
        return Err("abandonment timestamp is not positive".to_string());
    }
    let snapshot_hash: String = row
        .try_get("snapshot_hash")
        .map_err(|error| error.to_string())?;
    let simulation_value = row
        .try_get::<Option<Value>, _>("simulation_json")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "failed-closed abandonment lost its simulation".to_string())?;
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_value)
        .map_err(|error| format!("decode failed-closed abandonment simulation: {error}"))?;
    if simulation.terminal()
        || simulation.tick != authoritative_tick
        || simulation
            .snapshot_hash()
            .map_err(|error| format!("hash failed-closed abandonment simulation: {error}"))?
            != snapshot_hash
    {
        return Err(
            "failed-closed abandonment requires an exact non-terminal simulation".to_string(),
        );
    }
    let local_tombstone_state: String = row
        .try_get("local_tombstone_state")
        .map_err(|error| error.to_string())?;
    if !matches!(local_tombstone_state.as_str(), "hot_pending" | "sealed") {
        return Err("failed-closed abandonment marker has an unsupported seal state".to_string());
    }
    let failure_reason: String = row
        .try_get("failure_reason")
        .map_err(|error| error.to_string())?;
    if !valid_abandonment_failure_reason(&failure_reason) {
        return Err("failed-closed abandonment reason is invalid".to_string());
    }
    Ok(AbandonmentDatabaseEvidence {
        match_id: row.try_get("match_id").map_err(|error| error.to_string())?,
        journal_owner_id: row
            .try_get("journal_owner_id")
            .map_err(|error| error.to_string())?,
        actor_generation: row
            .try_get("actor_generation")
            .map_err(|error| error.to_string())?,
        instance_id: row
            .try_get("instance_id")
            .map_err(|error| error.to_string())?,
        actor_epoch: row
            .try_get("actor_epoch")
@@TRNM_R18_T1_SEGMENT_2@@
        PublishedTickAckTombstoneInput {
            high_water: hot,
            result_hash: commit.result_hash.clone(),
            settlement_state: commit.settlement_state.clone(),
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
            database_system_identifier: lineage.system_identifier.clone(),
            database_timeline_id: lineage.timeline_id,
            database_wal_lsn: lineage.wal_flush_lsn.clone(),
@@TRNM_R18_T1_SEGMENT_3@@
        &mut connection,
        high_water,
        database_host_authority,
        false,
        None,
    )
    .await?
    else {
@@TRNM_R18_T1_SEGMENT_4@@
            .await?;
            reconciled.terminal_acknowledged.insert(evidence.match_id);
        }
        if page_len < usize::try_from(TERMINAL_ACK_STARTUP_PAGE_SIZE).unwrap_or(usize::MAX) {
            break;
        }
    }

@@TRNM_R18_T1_SEGMENT_5@@
