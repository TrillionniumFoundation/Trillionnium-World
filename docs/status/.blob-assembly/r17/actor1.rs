// TRNM_R17_ACTOR1_CHUNK_0_PLACEHOLDER_0B2B9C63
            assigned_instance_id.as_deref(),
            assigned_physical_host_id.as_deref(),
            state.physical_host_id.as_str(),
        ) {
            return Err(
                "match reconciliation query returned a remote physical-host assignment".to_string(),
            );
        }
        match ensure_match_actor(state, match_id).await {
// TRNM_R17_ACTOR1_CHUNK_1_PLACEHOLDER_0B2B9C63
                    terminal_stage_simulation_json, terminal_stage_result_json,
                    terminal_stage_result_hash, terminal_stage_snapshot_hash,
                    terminal_stage_authoritative_tick, terminal_stage_next_sequence,
                    terminal_stage_match_revision
             from trnm_online_matches
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
// TRNM_R17_ACTOR1_CHUNK_2_PLACEHOLDER_0B2B9C63
        match_mode,
        members: Arc::new(members),
        next_sequence,
        match_revision,
        next_input_sequences,
        durable_recovery_tick,
    };
    lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    transaction
// TRNM_R17_ACTOR1_CHUNK_3_PLACEHOLDER_0B2B9C63
