// TRNM_R17_ACTOR2_CHUNK_0_PLACEHOLDER_0B2B9C63
    clock.reset(started_at);
    let mut shutdown = state.shutdown.subscribe();
    let mut interval =
        tokio::time::interval_at(started_at + state.tick_interval, state.tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            biased;
            changed = shutdown_changed_or_current(&mut shutdown) => {
// TRNM_R17_ACTOR2_CHUNK_1_PLACEHOLDER_0B2B9C63
                        )
                        .await
                        .map_err(|_| "terminal publication ACK marker timed out".to_string())
                        .and_then(|result| result);
                        let _ = ack_completion
                            .send(TerminalAckCompletion {
                                deferred_terminal,
                                result,
                            })
// TRNM_R17_ACTOR2_CHUNK_2_PLACEHOLDER_0B2B9C63
                            .max(latest_visible_candidate_tick);
                        let recovery = tokio::time::timeout(
                            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                            reload_visible_actor_after_persistence_failure(
                                &state,
                                match_id,
                                pending.visible,
                                recovery_target_tick,
                                latest_visible_candidate_tick,
// TRNM_R17_ACTOR2_CHUNK_3_PLACEHOLDER_0B2B9C63
