async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
                    select 1 from trnm_online_terminal_publication_acks terminal
        "running maintenance database checkpoint",
            .try_get::<Option<String>, _>("failure_reason")
            .map_err(|error| error.to_string())?
            .is_none()
        && row
            .try_get::<Option<Uuid>, _>("terminal_publication_actor_generation")
            .map_err(|error| error.to_string())?
            .is_none()
        && row
            .try_get::<bool, _>("terminal_stage_empty")
            .map_err(|error| error.to_string())?
        && !row
            .try_get::<bool, _>("has_terminal_ack")
            .map_err(|error| error.to_string())?
        && !row
            .try_get::<bool, _>("has_abandonment_marker")
            .map_err(|error| error.to_string())?
        && !simulation.terminal();
    if !exact_running_shape
        || !running_maintenance_successor_is_monotonic(
            high_water,
            durable_tick,
            durable_next_sequence,
            durable_match_revision,
            &durable_member_cursors,
            &durable_snapshot_hash,
        )
    {
        return Err(
            "running database authority is not an exact monotonic successor of its hot witness"
                .to_string(),
        );
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;

    let unchanged = durable_tick == high_water.tick
        && durable_next_sequence == high_water.next_sequence
        && durable_match_revision == high_water.match_revision
        && durable_member_cursors == high_water.next_input_sequences
        && durable_snapshot_hash == high_water.snapshot_hash;
    if unchanged {
        return Ok(high_water.clone());
    }
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
