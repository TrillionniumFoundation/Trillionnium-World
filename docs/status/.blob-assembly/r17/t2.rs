async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
                    select 1 from trnm_online_terminal_publication_acks terminal
                     where terminal.match_id = m.match_id
                ) as has_terminal_ack,
                exists(
                    select 1 from trnm_online_failed_closed_abandonment_markers marker
                     where marker.match_id = m.match_id
                ) as has_abandonment_marker
           from trnm_online_matches m
          where m.match_id = $1
          for share",
    )
    .bind(high_water.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "running maintenance exact match UUID disappeared".to_string())?;
    let simulation_value = row
        .try_get::<Option<Value>, _>("simulation_json")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "running maintenance durable simulation is missing".to_string())?;
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_value)
        .map_err(|error| format!("decode running maintenance simulation: {error}"))?;
    let durable_tick = u64::try_from(
        row.try_get::<i64, _>("authoritative_tick")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "running maintenance authoritative tick is negative".to_string())?;
    let durable_next_sequence = u64::try_from(
        row.try_get::<i64, _>("next_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "running maintenance next sequence is negative".to_string())?;
    let durable_checkpoint_sequence = u64::try_from(
        row.try_get::<i64, _>("checkpoint_sequence")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "running maintenance checkpoint sequence is negative".to_string())?;
    let durable_match_revision = u64::try_from(
        row.try_get::<i64, _>("match_revision")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "running maintenance match revision is negative".to_string())?;
    let durable_snapshot_hash = row
        .try_get::<String, _>("snapshot_hash")
        .map_err(|error| error.to_string())?;
    validate_recovery_simulation(
        &simulation,
        Some(durable_tick),
        &durable_snapshot_hash,
        "running maintenance database checkpoint",
            .try_get::<Option<String>, _>("failure_reason")
    Ok(final_high_water)
