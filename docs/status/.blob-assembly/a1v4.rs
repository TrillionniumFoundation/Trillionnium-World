__TRNM_A1V4_CHUNK_0__
    state.authority_clock.reset(started_at);
    let mut interval = tokio::time::interval_at(started_at + tick_interval, tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        let scheduled_at = interval.tick().await;
        state
            .authority_clock
            .record_wake(scheduled_at, Instant::now(), tick_interval);
__TRNM_A1V4_CHUNK_1__
        .into_iter()
        .filter(|match_id| !bounded_transitions.contains(match_id))
        .collect::<Vec<_>>();
    report.terminal_ack_gaps = terminal_ack_gaps.len() as u64;
    let unrecoverable =
        terminal_ack_gaps_without_high_water(&terminal_ack_gaps, &recorded_match_ids);
    report.quarantined_without_high_water = unrecoverable.len() as u64;
    for match_id in &terminal_ack_gaps {
__TRNM_A1V4_CHUNK_2__
                    state,
                    match_id,
                    initialization,
                    reservation,
                )
                .await;
            }
        }
__TRNM_A1V4_CHUNK_3__
            match_revision: loaded.match_revision,
            terminal: true,
            completion: None,
        };
        persist_terminal_actor_checkpoint(state, match_id, &terminal_job).await?;
        let (_result, result_hash) =
            derive_terminal_result(&loaded.simulation, &loaded.match_mode)?;
        let terminal_evidence = TerminalPublicationEvidence {
__TRNM_A1V4_CHUNK_4__
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
__TRNM_A1V4_CHUNK_5__
        let accepted_revision = recovered_row
            .try_get::<i64, _>("accepted_match_revision")
            .map_err(|error| error.to_string())? as u64;
        if accepted_revision != match_revision {
            return Err("latest command recovery revision does not match authority".to_string());
        }
        let accepted_hash: String = recovered_row
            .try_get("accepted_snapshot_hash")
__TRNM_A1V4_CHUNK_6__
            Some(checkpoint_simulation.clone())
        } else if high_water.next_sequence > checkpoint_sequence
            && high_water.next_sequence < next_sequence
        {
            let bridge_row = sqlx::query::query(
                "select post_simulation_json, accepted_snapshot_hash,
                        accepted_match_revision, target_tick, client_observed_tick, order_json
                 from trnm_online_commands
__TRNM_A1V4_CHUNK_7__
            "published-tick recovery cannot bridge a non-adjacent durable command boundary"
                .to_string(),
        );
    }
    let mut bridge = bridge_simulation.ok_or_else(|| {
        "published-tick recovery is missing the prior durable cursor bridge".to_string()
    })?;
    replay_recovery_to_tick(&mut bridge, high_water.tick)?;
__TRNM_A1V4_CHUNK_8__
    if let Some(high_water) = state.published_tick_journal.high_water(match_id)? {
        if high_water.actor_generation != actor_id || high_water.instance_id != *state.instance_id {
            let adopted = state
                .published_tick_journal
                .new_record(PublishedTickRecordInput {
                    instance_id: state.instance_id.as_str().to_string(),
                    match_id,
                    actor_generation: actor_id,
__TRNM_A1V4_CHUNK_9__
