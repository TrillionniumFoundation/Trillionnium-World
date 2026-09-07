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
            .try_get("journal_owner_id")
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
            reconciled.terminal_acknowledged.insert(evidence.match_id);
        .copied()
