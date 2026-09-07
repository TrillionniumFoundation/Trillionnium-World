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
                    tombstone.high_water.match_id
                )
            })?;
            if !abandonment_tombstone_matches_evidence(&tombstone, &evidence, lineage) {
                return Err(format!(
                    "latest cold abandonment {} conflicts with PostgreSQL",
                    tombstone.high_water.match_id
                ));
            }
            match evidence.local_tombstone_state.as_str() {
                "hot_pending" => {
                    seal_abandonment_in_database(
                        connection,
                        &tombstone,
                        None,
                        database_host_authority,
                    )
                    .await?;
                }
                "sealed" => {}
                _ => {
                    return Err(
                        "latest cold abandonment has an unsupported DB seal state".to_string()
                    );
    high_water_epoch: i64,
    high_water_physical_host_id: &str,
    runtime_instance_id: &str,
    runtime_epoch: i64,
    runtime_physical_host_id: &str,
    requires_mutation: bool,
) -> bool {
    !requires_mutation
        || (high_water_instance_id == runtime_instance_id
            && high_water_epoch == runtime_epoch
            && high_water_physical_host_id == runtime_physical_host_id)
}

fn terminal_ack_gaps_without_high_water(
    terminal_ack_gaps: &[Uuid],
    recorded_match_ids: &BTreeSet<Uuid>,
) -> Vec<Uuid> {
    terminal_ack_gaps
        .iter()
        .filter(|match_id| !recorded_match_ids.contains(match_id))
        .copied()
        .collect()
}
