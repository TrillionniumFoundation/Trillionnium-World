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
            .map_err(|error| error.to_string())?,
        physical_host_id: row
            .try_get("physical_host_id")
            .map_err(|error| error.to_string())?,
        authoritative_tick,
        next_sequence,
        match_revision,
        next_input_sequences: serde_json::from_value(
            row.try_get::<Value, _>("next_input_sequences")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("decode abandonment input cursors: {error}"))?,
        snapshot_hash,
        failure_reason,
        local_tombstone_state,
        abandoned_at_unix_ms,
    })
}

async fn load_abandonment_database_evidence_by_match<'e, E>(
    executor: E,
    match_id: Uuid,
) -> Result<Option<AbandonmentDatabaseEvidence>, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query::query(ABANDONMENT_DATABASE_EVIDENCE_BY_MATCH_SQL)
        .bind(match_id)
        .fetch_optional(executor)
        .await
        .map_err(|error| error.to_string())?
        .as_ref()
        .map(abandonment_database_evidence_from_row)
        .transpose()
}

async fn load_local_abandonment_startup_page(
    pool: &PgPool,
    physical_host_id: &str,
    after_match_id: Option<Uuid>,
) -> Result<Vec<AbandonmentDatabaseEvidence>, String> {
    sqlx::query::query(LOCAL_ABANDONMENT_STARTUP_PAGE_SQL)
        .bind(physical_host_id)
        .bind(after_match_id)
        .bind(TERMINAL_ACK_STARTUP_PAGE_SIZE)
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?
        .iter()
        .map(abandonment_database_evidence_from_row)
        .collect()
}

async fn load_terminal_ack_database_evidence_by_match<'e, E>(
    executor: E,
    match_id: Uuid,
) -> Result<Option<TerminalAckDatabaseEvidence>, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let Some(row) = sqlx::query::query(TERMINAL_ACK_DATABASE_EVIDENCE_BY_MATCH_SQL)
        .bind(match_id)
        .fetch_optional(executor)
        .await
        .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let tuple_exact = row
        .try_get::<bool, _>("tuple_exact")
        .map_err(|error| error.to_string())?;
    let evidence = terminal_ack_database_evidence_from_row(&row)?;
    if !tuple_exact {
        return Err(format!(
            "terminal ACK {} does not exactly match its durable match authority",
            evidence.match_id
        ));
    }
    Ok(Some(evidence))
}

async fn load_local_terminal_ack_startup_page(
    pool: &PgPool,
    physical_host_id: &str,
    after_match_id: Option<Uuid>,
) -> Result<Vec<TerminalAckDatabaseEvidence>, String> {
    let rows = sqlx::query::query(LOCAL_TERMINAL_ACK_STARTUP_PAGE_SQL)
        .bind(physical_host_id)
        .bind(after_match_id)
        .bind(TERMINAL_ACK_STARTUP_PAGE_SIZE)
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?;
    let mut evidence = Vec::with_capacity(rows.len());
    for row in rows {
        let tuple_exact = row
            .try_get::<bool, _>("tuple_exact")
            .map_err(|error| error.to_string())?;
        let item = terminal_ack_database_evidence_from_row(&row)?;
        if !tuple_exact {
            return Err(format!(
                "terminal ACK {} does not exactly match its durable match authority",
                item.match_id
            ));
        }
        evidence.push(item);
    }
    Ok(evidence)
}

async fn mark_terminal_ack_sealed<'e, E>(
    executor: E,
    tombstone: &PublishedTickAckTombstone,
) -> Result<(), String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let high_water = &tombstone.high_water;
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "terminal tombstone tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "terminal tombstone sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "terminal tombstone revision exceeds PostgreSQL range".to_string())?;
    let acknowledged_at_unix_ms = i64::try_from(tombstone.acknowledged_at_unix_ms)
        .map_err(|_| "terminal tombstone timestamp exceeds PostgreSQL range".to_string())?;
    let sealed = sqlx::query::query(
        "update trnm_online_terminal_publication_acks a
            set local_tombstone_state = 'sealed'
           from trnm_online_matches m
          where a.match_id = $1 and m.match_id = a.match_id
            and a.local_tombstone_state in (
                'legacy_bootstrap_pending', 'hot_pending', 'sealed'
            )
            and a.actor_generation = $2
            and a.instance_id = $3
            and a.actor_epoch = $4
            and a.physical_host_id = $5
            and a.authoritative_tick = $6
            and a.next_sequence = $7
            and a.match_revision = $8
            and a.next_input_sequences = $9
            and a.snapshot_hash = $10
            and a.phase = 'complete'
            and a.result_hash = $11
            and (
                a.published_settlement_state = $12
                or ($12 = 'pending' and a.published_settlement_state = 'settled')
            )
            and floor(extract(epoch from a.acknowledged_at) * 1000)::bigint = $13
            and m.phase = 'complete'
            and m.checkpoint_sequence = m.next_sequence
            and m.result_hash is not null
            and m.settlement_state in ('pending', 'settled')
            and m.terminal_publication_state = 'acknowledged'
            and m.terminal_publication_actor_generation = a.actor_generation
            and m.assigned_instance_id = a.instance_id
            and m.assigned_instance_epoch = a.actor_epoch
            and m.assigned_physical_host_id = a.physical_host_id
            and m.authoritative_tick = a.authoritative_tick
            and m.next_sequence = a.next_sequence
            and m.match_revision = a.match_revision
            and a.next_input_sequences = coalesce(
                (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                 )
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and m.snapshot_hash = a.snapshot_hash
            and m.result_hash = a.result_hash
            and m.settlement_state = a.published_settlement_state
        returning a.match_id",
    )
    .bind(high_water.match_id)
    .bind(high_water.actor_generation)
    .bind(&high_water.instance_id)
    .bind(high_water.actor_epoch)
    .bind(&high_water.physical_host_id)
    .bind(tick)
    .bind(next_sequence)
    .bind(match_revision)
    .bind(
        serde_json::to_value(&high_water.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(&high_water.snapshot_hash)
    .bind(&tombstone.result_hash)
    .bind(&tombstone.settlement_state)
    .bind(acknowledged_at_unix_ms)
    .fetch_optional(executor)
    .await
    .map_err(|error| error.to_string())?;
    if sealed.is_none() {
        return Err(
            "cold terminal tombstone could not seal its exact durable ACK authority".to_string(),
        );
    }
    Ok(())
}

fn terminal_tombstone_matches_commit(
    tombstone: &PublishedTickAckTombstone,
    high_water: &PublishedTickHighWater,
    commit: &TerminalPublicationCommit,
    lineage: &DatabaseLineage,
) -> bool {
    tombstone.high_water == *high_water
        && tombstone.result_hash == commit.result_hash
        && terminal_tombstone_settlement_matches(
            &tombstone.settlement_state,
            &commit.settlement_state,
        )
        && tombstone.acknowledged_at_unix_ms == commit.acknowledged_at_unix_ms
        && database_lineage_covers_tombstone(lineage, tombstone)
}

async fn ensure_cold_terminal_ack(
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    commit: &TerminalPublicationCommit,
    lineage: &DatabaseLineage,
) -> Result<PublishedTickAckTombstone, String> {
    if high_water.phase != "complete" || !high_water.receipts_replayable {
        return Err("cold terminal ACK requires a replayable terminal high-water".to_string());
    }
    if let Some(existing) = journal.ack_tombstone(high_water.match_id)? {
        if !terminal_tombstone_matches_commit(&existing, high_water, commit, lineage) {
            return Err(
                "durable cold terminal ACK conflicts with PostgreSQL authority or lineage"
                    .to_string(),
            );
        }
        return Ok(existing);
    }
    let hot = journal
        .high_water(high_water.match_id)?
        .ok_or_else(|| "cold terminal ACK is missing its exact hot witness".to_string())?;
    if hot != *high_water {
        return Err("cold terminal ACK hot witness changed before sealing".to_string());
    }
    let tombstone = seal_terminal_ack_with_timeout(
        journal,
        PublishedTickAckTombstoneInput {
            high_water: hot,
            result_hash: commit.result_hash.clone(),
            settlement_state: commit.settlement_state.clone(),
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
            database_system_identifier: lineage.system_identifier.clone(),
            database_timeline_id: lineage.timeline_id,
            database_wal_lsn: lineage.wal_flush_lsn.clone(),
        },
    )
    .await?;
    if !terminal_tombstone_matches_commit(&tombstone, high_water, commit, lineage) {
        return Err("sealed cold terminal ACK did not preserve exact authority".to_string());
    }
    Ok(tombstone)
}

async fn seal_terminal_ack_in_database(
    connection: &mut PgConnection,
    tombstone: &PublishedTickAckTombstone,
    runtime_state: Option<&AppState>,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<(), String> {
    let mut transaction = connection
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch(&mut transaction, state, true).await?;
    }
    mark_terminal_ack_sealed(&mut *transaction, tombstone).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    if !exact_terminal_publication_marker_exists(&mut *connection, tombstone.high_water.match_id)
        .await?
    {
        return Err(
            "cold terminal tombstone sealed state is not an exact public ACK marker".to_string(),
        );
    }
    Ok(())
}

fn abandonment_tombstone_matches_evidence(
    tombstone: &PublishedTickAbandonmentTombstone,
    evidence: &AbandonmentDatabaseEvidence,
    lineage: &DatabaseLineage,
) -> bool {
    evidence.matches_high_water(&tombstone.high_water)
        && tombstone.failure_reason == evidence.failure_reason
        && tombstone.abandoned_at_unix_ms == evidence.abandoned_at_unix_ms
        && database_lineage_covers_abandonment(lineage, tombstone)
}

async fn ensure_cold_abandonment(
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    evidence: &AbandonmentDatabaseEvidence,
    lineage: &DatabaseLineage,
) -> Result<PublishedTickAbandonmentTombstone, String> {
    if !evidence.matches_high_water(high_water) {
        return Err(
            "failed-closed abandonment evidence does not match its exact hot witness".to_string(),
        );
    }
    if let Some(existing) = journal.abandonment_tombstone(high_water.match_id)? {
        if !abandonment_tombstone_matches_evidence(&existing, evidence, lineage) {
            return Err(
                "durable cold abandonment conflicts with PostgreSQL authority or lineage"
                    .to_string(),
            );
        }
        return Ok(existing);
    }
    if evidence.local_tombstone_state == "sealed" {
        return Err("sealed failed-closed abandonment lost its durable cold tombstone".to_string());
    }
    let hot = journal
        .high_water(high_water.match_id)?
        .ok_or_else(|| "cold abandonment is missing its exact hot witness".to_string())?;
    if hot != *high_water {
        return Err("cold abandonment hot witness changed before sealing".to_string());
    }
    let tombstone = seal_abandonment_with_timeout(
        journal,
        PublishedTickAbandonmentTombstoneInput {
            high_water: hot,
            failure_reason: evidence.failure_reason.clone(),
            abandoned_at_unix_ms: evidence.abandoned_at_unix_ms,
            database_system_identifier: lineage.system_identifier.clone(),
            database_timeline_id: lineage.timeline_id,
            database_wal_lsn: lineage.wal_flush_lsn.clone(),
        },
    )
    .await?;
    if !abandonment_tombstone_matches_evidence(&tombstone, evidence, lineage) {
        return Err("sealed cold abandonment did not preserve exact authority".to_string());
    }
    Ok(tombstone)
}

async fn mark_abandonment_sealed<'e, E>(
    executor: E,
    tombstone: &PublishedTickAbandonmentTombstone,
) -> Result<(), String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let high_water = &tombstone.high_water;
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "abandonment tombstone tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "abandonment tombstone sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "abandonment tombstone revision exceeds PostgreSQL range".to_string())?;
    let abandoned_at_unix_ms = i64::try_from(tombstone.abandoned_at_unix_ms)
        .map_err(|_| "abandonment tombstone timestamp exceeds PostgreSQL range".to_string())?;
    let sealed = sqlx::query::query(
        "update trnm_online_failed_closed_abandonment_markers a
            set local_tombstone_state = 'sealed'
           from trnm_online_matches m
          where a.match_id = $1 and m.match_id = a.match_id
            and a.local_tombstone_state in ('hot_pending', 'sealed')
            and a.journal_owner_id = $2
            and a.actor_generation = $3
            and a.instance_id = $4
            and a.actor_epoch = $5
            and a.physical_host_id = $6
            and a.authoritative_tick = $7
            and a.next_sequence = $8
            and a.match_revision = $9
            and a.next_input_sequences = $10
            and a.snapshot_hash = $11
            and a.failure_reason = $12
            and floor(extract(epoch from a.abandoned_at) * 1000)::bigint = $13
            and m.phase = 'failed_closed'
            and m.settlement_state = 'failed_closed'
            and m.terminal_publication_state = 'pending'
            and m.checkpoint_sequence = m.next_sequence
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.failure_reason = a.failure_reason
            and m.assigned_instance_id = a.instance_id
            and m.assigned_instance_epoch = a.actor_epoch
            and m.assigned_physical_host_id = a.physical_host_id
            and m.authoritative_tick = a.authoritative_tick
            and m.next_sequence = a.next_sequence
            and m.match_revision = a.match_revision
            and m.snapshot_hash = a.snapshot_hash
            and a.next_input_sequences = coalesce(
                (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                 )
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                 where terminal.match_id = m.match_id
            )
        returning a.match_id",
    )
    .bind(high_water.match_id)
    .bind(high_water.journal_owner_id)
    .bind(high_water.actor_generation)
    .bind(&high_water.instance_id)
    .bind(high_water.actor_epoch)
    .bind(&high_water.physical_host_id)
    .bind(tick)
    .bind(next_sequence)
    .bind(match_revision)
    .bind(
        serde_json::to_value(&high_water.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(&high_water.snapshot_hash)
    .bind(&tombstone.failure_reason)
    .bind(abandoned_at_unix_ms)
    .fetch_optional(executor)
    .await
    .map_err(|error| error.to_string())?;
    if sealed.is_none() {
        return Err(
            "cold abandonment could not seal its exact durable failed-closed authority".to_string(),
        );
    }
    Ok(())
}

async fn seal_abandonment_in_database(
    connection: &mut PgConnection,
    tombstone: &PublishedTickAbandonmentTombstone,
    runtime_state: Option<&AppState>,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<(), String> {
    let mut transaction = connection
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch(&mut transaction, state, true).await?;
    }
    mark_abandonment_sealed(&mut *transaction, tombstone).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    let evidence = load_abandonment_database_evidence_by_match(
        &mut *connection,
        tombstone.high_water.match_id,
    )
    .await?
    .ok_or_else(|| "sealed abandonment marker disappeared from PostgreSQL".to_string())?;
    let lineage = database_lineage(&mut *connection).await?;
    if evidence.local_tombstone_state != "sealed"
        || !abandonment_tombstone_matches_evidence(tombstone, &evidence, &lineage)
    {
        return Err("cold abandonment sealed state is not an exact durable marker".to_string());
    }
    Ok(())
}

async fn reconcile_failed_closed_high_water(
    pool: &PgPool,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    runtime_state: Option<&AppState>,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<bool, String> {
    let mut connection = pool.acquire().await.map_err(|error| error.to_string())?;
    let Some((evidence, lineage, _legacy_adopted)) = ensure_failed_closed_abandonment_marker(
        &mut connection,
        high_water,
            reconciled.terminal_acknowledged.insert(evidence.match_id);
        .copied()
