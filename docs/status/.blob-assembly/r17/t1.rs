#[derive(Debug)]
    Ok(TerminalAckDatabaseEvidence {
            .try_get("journal_owner_id")
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
            reconciled.terminal_acknowledged.insert(evidence.match_id);
    let database_summary =
        load_cold_witness_database_summary(&mut *connection, physical_host_id).await?;
    let terminal_count = u64::try_from(journal.ack_tombstone_count()?)
        .map_err(|_| "terminal cold witness count exceeds u64".to_string())?;
    let abandonment_count = u64::try_from(journal.abandonment_tombstone_count()?)
        .map_err(|_| "abandonment cold witness count exceeds u64".to_string())?;
    let cold_count = u64::try_from(journal.cold_witness_count()?)
        .map_err(|_| "combined cold witness count exceeds u64".to_string())?;
    if !database_summary.all_sealed()
        || database_summary.terminal_total_count != terminal_count
        || database_summary.abandonment_total_count != abandonment_count
        || database_summary
            .terminal_total_count
            .checked_add(database_summary.abandonment_total_count)
            != Some(cold_count)
    {
        return Err(format!(
            "local cold witness seal mismatch: PostgreSQL terminal {}/{}, abandonment {}/{}, journal terminal {}, abandonment {}, combined {}",
            database_summary.terminal_sealed_count,
            database_summary.terminal_total_count,
            database_summary.abandonment_sealed_count,
            database_summary.abandonment_total_count,
            terminal_count,
            abandonment_count,
            cold_count,
        ));
    }
    Ok(reconciled)
}

async fn latest_cold_witness_sentinel_is_healthy(state: &AppState) -> Result<bool, String> {
    let Some(witness) = state.published_tick_journal.latest_cold_witness()? else {
        return Ok(true);
    };
    let mut connection = state
        .readiness_pool
        .acquire()
        .await
        .map_err(|error| error.to_string())?;
    let lineage = database_lineage(&mut *connection).await?;
    if !database_lineage_covers_cold_witness(&lineage, &witness) {
        return Ok(false);
    }
    match witness {
        PublishedTickColdWitness::TerminalAck(tombstone) => {
            let Some(evidence) = load_terminal_ack_database_evidence_by_match(
                &mut *connection,
                tombstone.high_water.match_id,
            )
            .await?
            else {
                return Ok(false);
            };
            Ok(evidence.local_tombstone_state == "sealed"
                && evidence.matches_tombstone(&tombstone, &lineage)
                && exact_terminal_publication_marker_exists(
                    &mut *connection,
                    tombstone.high_water.match_id,
                )
                .await?)
               and a.match_revision = $6
        .copied()
