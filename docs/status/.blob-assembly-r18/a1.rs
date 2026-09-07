@@TRNM_R18_A1_SEGMENT_0@@
                state
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
@@TRNM_R18_A1_SEGMENT_1@@
    }
    registry.initializing.remove(&match_id);
    reservation.disarm();
    match initialized {
        Ok(Some(initialized)) => {
            let handle = initialized.handle.clone();
            registry.actors.insert(match_id, handle.clone());
            initialization.ready.send_replace(true);
@@TRNM_R18_A1_SEGMENT_2@@
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
@@TRNM_R18_A1_SEGMENT_3@@
            || high_water.snapshot_hash != staged.snapshot_hash
        {
            return Err(
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
            );
        }
        None
    } else if let Some(high_water) = high_water.as_ref() {
@@TRNM_R18_A1_SEGMENT_4@@
        || durable_match_revision != high_water.match_revision.saturating_add(1)
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
    {
        return false;
    }
    let mut advanced_members = 0usize;
    for (player_id, high_water_cursor) in &high_water.next_input_sequences {
        let Some(durable_cursor) = durable_next_input_sequences.get(player_id) else {
@@TRNM_R18_A1_SEGMENT_5@@
