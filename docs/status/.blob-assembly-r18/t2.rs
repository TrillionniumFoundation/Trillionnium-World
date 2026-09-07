@@TRNM_R18_T2_SEGMENT_0@@
            result_valid,
            settlement_state: settlement_state.clone(),
            next_input_sequences: member_cursors,
            assigned_instance_id: row
                .try_get("assigned_instance_id")
                .map_err(|error| error.to_string())?,
            assigned_instance_epoch: row
                .try_get("assigned_instance_epoch")
@@TRNM_R18_T2_SEGMENT_1@@
    Ok(StartupCommandRecoveryEvidence {
        simulation,
        accepted_revision: u64::try_from(
            row.try_get::<i64, _>("accepted_match_revision")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| format!("{label} accepted revision is negative"))?,
        player_id: row
@@TRNM_R18_T2_SEGMENT_2@@
    let source = if high_water.next_sequence == 0 {
        let initial_value = row
            .try_get::<Option<Value>, _>("initial_simulation_json")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| {
                "running high-water recovery is missing initial simulation".to_string()
            })?;
        let initial = serde_json::from_value::<MissionSimV1>(initial_value.clone())
@@TRNM_R18_T2_SEGMENT_3@@
          where match_id = $1
          order by player_id
          for update",
    )
    .bind(high_water.match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
@@TRNM_R18_T2_SEGMENT_4@@
        || durable_next_sequence < high_water.next_sequence
        || durable_match_revision < high_water.match_revision
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
        || high_water
            .next_input_sequences
            .iter()
            .any(|(player_id, cursor)| {
                durable_member_cursors
@@TRNM_R18_T2_SEGMENT_5@@
