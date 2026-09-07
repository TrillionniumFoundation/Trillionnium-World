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
    };
    if !terminal_authority_succeeds_running_high_water(&view, high_water) {
        return Err(
            "completed database authority is not an exact successor of its running high-water"
                .to_string(),
        );
    }

    let source = if high_water.next_sequence == 0 {
        let initial_value = row
            .try_get::<Option<Value>, _>("initial_simulation_json")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| {
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
          where migration_version = 13
            and migration_name = '0013_online_failed_closed_abandonment_v1'
            and checksum_sha256 = $1",
    )
    .bind(migration_checksum)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "V13 migration ledger entry is absent or drifted".to_string())?;
    let legacy_origin = match_updated_at < migration_applied_at;
    if !marker_exists {
        if !adopt_legacy_pre_v13 {
            return Err(format!(
                "failed-closed match {} has no atomic abandonment marker; use the explicit pre-V13 adoption maintenance path only for a proven legacy row",
                high_water.match_id
            ));
        }
        if !legacy_abandonment_adoption_allowed(
            adopt_legacy_pre_v13,
            match_updated_at,
            migration_applied_at,
        ) {
            return Err(format!(
                "failed-closed match {} is not strictly older than the V13 ledger and cannot use legacy adoption",
    let final_high_water = journal.new_record(PublishedTickRecordInput {
        instance_id: high_water.instance_id.clone(),
        match_id: high_water.match_id,
        actor_generation: high_water.actor_generation,
        actor_epoch: high_water.actor_epoch,
        tick: durable_tick,
        next_sequence: durable_next_sequence,
        match_revision: durable_match_revision,
        next_input_sequences: durable_member_cursors.clone(),
        phase: "running".to_string(),
        receipts_replayable: true,
        snapshot_hash: durable_snapshot_hash,
    })?;
    record_published_tick_with_timeout(
        journal,
        final_high_water.clone(),
        durable_next_sequence,
        durable_match_revision,
        durable_member_cursors,
    )
    .await?;
    Ok(final_high_water)
}
