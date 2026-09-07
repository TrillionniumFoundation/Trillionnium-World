async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
                    select 1 from trnm_online_terminal_publication_acks terminal
        "running maintenance database checkpoint",
    )?;
    let member_rows = sqlx::query::query(
        "select player_id, next_input_sequence
           from trnm_online_match_members
          where match_id = $1
          order by player_id
          for share",
    )
    .bind(high_water.match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut durable_member_cursors = BTreeMap::new();
    for member in member_rows {
        durable_member_cursors.insert(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            u64::try_from(
                member
                    .try_get::<i64, _>("next_input_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "running maintenance member cursor is negative".to_string())?,
        );
    }
    let exact_running_shape = row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?
        == "running"
        && row
            .try_get::<String, _>("settlement_state")
            .map_err(|error| error.to_string())?
            == "not_ready"
        && row
            .try_get::<String, _>("terminal_publication_state")
            .map_err(|error| error.to_string())?
            == "pending"
        && row
            .try_get::<Option<String>, _>("assigned_instance_id")
            .map_err(|error| error.to_string())?
            .as_deref()
            == Some(high_water.instance_id.as_str())
        && row
            .try_get::<i64, _>("assigned_instance_epoch")
            .map_err(|error| error.to_string())?
            == high_water.actor_epoch
        && row
            .try_get::<Option<String>, _>("assigned_physical_host_id")
            .map_err(|error| error.to_string())?
            .as_deref()
            == Some(high_water.physical_host_id.as_str())
        && durable_checkpoint_sequence == durable_next_sequence
        && row
            .try_get::<Option<Value>, _>("result_json")
            .map_err(|error| error.to_string())?
            .is_none()
        && row
            .try_get::<Option<String>, _>("result_hash")
            .map_err(|error| error.to_string())?
            .is_none()
        && row
            .try_get::<Option<String>, _>("failure_reason")
    Ok(final_high_water)
