async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
                    select 1 from trnm_online_terminal_publication_acks terminal
        "running maintenance database checkpoint",
            .try_get::<Option<String>, _>("failure_reason")
    Ok(final_high_water)
