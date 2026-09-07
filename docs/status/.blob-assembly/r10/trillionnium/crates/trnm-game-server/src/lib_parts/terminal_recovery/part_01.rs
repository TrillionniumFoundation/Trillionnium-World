#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    body: OnlineAuthorityError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        let mut response = (status, Json(self.body)).into_response();
        if matches!(
            status,
            StatusCode::SERVICE_UNAVAILABLE | StatusCode::TOO_MANY_REQUESTS
        ) {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
        }
        response
    }
}

struct DurableTerminalCompactionView {
    phase: String,
    simulation_tick: u64,
    simulation_terminal: bool,
    simulation_hash: String,
    durable_snapshot_hash: String,
    authoritative_tick: Option<u64>,
    next_sequence: Option<u64>,
    checkpoint_sequence: Option<u64>,
    match_revision: Option<u64>,
    result_valid: bool,
    settlement_state: String,
    next_input_sequences: BTreeMap<String, u64>,
    assigned_instance_id: Option<String>,
    assigned_instance_epoch: Option<i64>,
    assigned_physical_host_id: Option<String>,
}

fn terminal_result_matches_simulation(
    simulation: &MissionSimV1,
    result_value: Option<Value>,
    result_hash: Option<&str>,
    match_mode: &str,
) -> Result<bool, String> {
    let Some(result_value) = result_value else {
        return Ok(false);
    };
    let result = serde_json::from_value::<BattleResultV1>(result_value)
        .map_err(|error| format!("decode terminal result: {error}"))?;
    let (expected, expected_hash) = derive_terminal_result(simulation, match_mode)?;
    Ok(result == expected && expected_hash == result_hash.unwrap_or_default())
}

fn derive_terminal_result(
    simulation: &MissionSimV1,
    match_mode: &str,
) -> Result<(BattleResultV1, String), String> {
    let mut result = simulation
        .clone()
        .into_result()
        .map_err(|error| format!("derive terminal result: {error}"))?;
    if result.outcome == BattleOutcome::Victory && match_mode != "ranked_pvp" {
        result.resource_delta = result.resource_delta.max(25);
    }
    let result_hash = result
        .computed_hash()
        .map_err(|error| format!("hash terminal result: {error}"))?;
    Ok((result, result_hash))
}

fn terminal_authority_matches_high_water(
    view: &DurableTerminalCompactionView,
    high_water: &PublishedTickHighWater,
) -> bool {
    high_water.phase == "complete"
        && high_water.receipts_replayable
        && view.phase == "complete"
        && view.result_valid
        && matches!(view.settlement_state.as_str(), "pending" | "settled")
        && view.simulation_terminal
        && view.simulation_tick == high_water.tick
        && view.simulation_hash == high_water.snapshot_hash
        && view.durable_snapshot_hash == high_water.snapshot_hash
        && view.authoritative_tick == Some(high_water.tick)
        && view.next_sequence == Some(high_water.next_sequence)
        && view.checkpoint_sequence == Some(high_water.next_sequence)
        && view.match_revision == Some(high_water.match_revision)
        && view.next_input_sequences == high_water.next_input_sequences
        && view.assigned_instance_id.as_deref() == Some(high_water.instance_id.as_str())
        && view.assigned_instance_epoch == Some(high_water.actor_epoch)
        && view.assigned_physical_host_id.as_deref() == Some(high_water.physical_host_id.as_str())
}

fn terminal_authority_succeeds_running_high_water(
    view: &DurableTerminalCompactionView,
    high_water: &PublishedTickHighWater,
) -> bool {
    if high_water.phase != "running"
        || !high_water.receipts_replayable
        || view.phase != "complete"
        || !view.result_valid
        || !matches!(
            view.settlement_state.as_str(),
            "staged" | "pending" | "settled"
        )
        || !view.simulation_terminal
        || view.simulation_hash != view.durable_snapshot_hash
        || view.authoritative_tick != Some(view.simulation_tick)
        || view.simulation_tick < high_water.tick
        || view.checkpoint_sequence != view.next_sequence
        || view.assigned_instance_id.as_deref() != Some(high_water.instance_id.as_str())
        || view.assigned_instance_epoch != Some(high_water.actor_epoch)
        || view.assigned_physical_host_id.as_deref() != Some(high_water.physical_host_id.as_str())
    {
        return false;
    }
    let (Some(next_sequence), Some(match_revision)) = (view.next_sequence, view.match_revision)
    else {
        return false;
    };
    (next_sequence == high_water.next_sequence
        && match_revision == high_water.match_revision
        && view.next_input_sequences == high_water.next_input_sequences)
        || is_single_command_cursor_successor(
            high_water,
            next_sequence,
            match_revision,
            &view.next_input_sequences,
        )
}

fn terminal_marker_metadata_matches(
    marker_phase: Option<&str>,
    marker_result_hash: Option<&str>,
    marker_settlement_state: Option<&str>,
    durable_result_hash: Option<&str>,
    durable_settlement_state: &str,
) -> bool {
    marker_phase == Some("complete")
        && marker_result_hash.is_some()
        && marker_result_hash == durable_result_hash
        && matches!(durable_settlement_state, "pending" | "settled")
        && marker_settlement_state.is_some_and(|published| {
            matches!(published, "pending" | "settled") && published == durable_settlement_state
        })
}

fn terminal_publication_metadata_matches_durable(
    evidence: &TerminalPublicationEvidence,
    durable_result_hash: Option<&str>,
    durable_settlement_state: &str,
) -> bool {
    evidence.phase == OnlineMatchPhase::Complete
        && terminal_marker_metadata_matches(
            Some("complete"),
            evidence.result_hash.as_deref(),
            Some(&evidence.settlement_state),
            durable_result_hash,
            durable_settlement_state,
        )
}

fn match_assignment_matches_local(
    assigned_instance_id: Option<&str>,
    assigned_instance_epoch: i64,
    assigned_physical_host_id: Option<&str>,
    local_instance_id: &str,
    local_instance_epoch: i64,
    local_physical_host_id: &str,
) -> bool {
    assigned_instance_id == Some(local_instance_id)
        && assigned_instance_epoch == local_instance_epoch
        && assigned_physical_host_id == Some(local_physical_host_id)
}

fn reconciliation_candidate_is_local(
    assigned_instance_id: Option<&str>,
    assigned_physical_host_id: Option<&str>,
    local_physical_host_id: &str,
) -> bool {
    assigned_instance_id.is_none() || assigned_physical_host_id == Some(local_physical_host_id)
}

async fn local_terminal_publication_ack_gaps(
    pool: &PgPool,
    physical_host_id: &str,
) -> Result<Vec<Uuid>, String> {
    let gaps = sqlx::query_scalar::query_scalar(LOCAL_TERMINAL_ACK_GAPS_SQL)
        .bind(physical_host_id)
        .bind(TERMINAL_ACK_GAP_SCAN_LIMIT.saturating_add(1))
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?;
    if terminal_ack_gap_scan_is_saturated(gaps.len()) {
        return Err(format!(
            "terminal ACK gap scan saturated its bounded {TERMINAL_ACK_GAP_SCAN_LIMIT}-row quarantine window"
        ));
    }
    Ok(gaps)
}

fn terminal_ack_gap_scan_is_saturated(row_count: usize) -> bool {
    row_count > usize::try_from(TERMINAL_ACK_GAP_SCAN_LIMIT).unwrap_or(usize::MAX)
}

async fn exact_terminal_publication_marker_exists<'e, E>(
    executor: E,
    match_id: Uuid,
) -> Result<bool, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query_scalar::query_scalar(EXACT_TERMINAL_PUBLICATION_MARKER_SQL)
        .bind(match_id)
        .fetch_one(executor)
        .await
        .map_err(|error| error.to_string())
}

fn terminal_ack_database_evidence_from_row(
    row: &sqlx_postgres::PgRow,
) -> Result<TerminalAckDatabaseEvidence, String> {
    let acknowledged_at_unix_ms = u64::try_from(
        row.try_get::<i64, _>("acknowledged_at_unix_ms")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal ACK acknowledgement timestamp is negative".to_string())?;
    if acknowledged_at_unix_ms == 0 {
        return Err("terminal ACK acknowledgement timestamp is not positive".to_string());
    }
    let authoritative_tick = u64::try_from(
        row.try_get::<i64, _>("authoritative_tick")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal ACK authoritative tick is negative".to_string())?;
    let next_sequence = u64::try_from(
        row.try_get::<i64, _>("next_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal ACK sequence is negative".to_string())?;
    let match_revision = u64::try_from(
        row.try_get::<i64, _>("match_revision")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "terminal ACK revision is negative".to_string())?;
    let next_input_sequences = serde_json::from_value::<BTreeMap<String, u64>>(
        row.try_get::<Value, _>("next_input_sequences")
            .map_err(|error| error.to_string())?,
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
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
