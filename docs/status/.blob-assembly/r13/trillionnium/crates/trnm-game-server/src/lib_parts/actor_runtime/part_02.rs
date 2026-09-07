async fn compact_published_tick_journal_with_timeout(
    journal: &PublishedTickJournal,
    active_matches: BTreeSet<Uuid>,
) -> Result<(), String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.compact_to_active(active_matches),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick compaction exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}

        run_actor_command_persistence_worker(
            persistence_state,
            match_id,
            persistence_rx,
            persistence_completion_tx,
        )
        .await;
    });
    let (publication_candidates, publication_candidate_rx) =
        watch::channel(None::<ActorPublicationCandidate>);
    let (publication_completion_tx, mut publication_completion_rx) =
        mpsc::channel(MATCH_ACTOR_PUBLICATION_COMPLETION_QUEUE);
    let publication_journal = state.published_tick_journal.clone();
    let publication_instance_id = state.instance_id.clone();
    let publication_epoch = state.instance_epoch;
    let (publication_permit_tx, publication_permit_rx) = watch::channel(true);
    let terminal_published = published.clone();
    let terminal_publication_acked = publication_acked.clone();
    let (durable_recovery_tick_tx, durable_recovery_tick_rx) =
        watch::channel(loaded.durable_recovery_tick);
    let command_durable_recovery_tick_tx = durable_recovery_tick_tx.clone();
    let publication_handle = tokio::spawn(async move {
        run_actor_publication_worker(ActorPublicationWorker {
            journal: publication_journal,
                    deferred_terminal,
                    result,
                } = completion;
                if let Err(error) = result {
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            format!("durable publication failed: {error}"),
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            format!("terminal durable publication failed: {error}"),
                            true,
                        )));
                    }
                    tracing::error!(%match_id, %error, "published-tick durability failed closed");
                    break;
                }
                if terminal_publication_sequence == Some(completed_sequence) {
                    let Some(deferred_terminal) = deferred_terminal else {
                        tracing::error!(%match_id, "terminal publication completion omitted its deferred public tuple");
                            .tick
                            .max(latest_visible_candidate_tick);
                        let recovery = tokio::time::timeout(
                            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                            reload_visible_actor_after_persistence_failure(
                                &state,
                                match_id,
                                pending.visible,
                                recovery_target_tick,
                                latest_visible_candidate_tick,
                            ),
                        )
                        .await
                        .map_err(|_| "command persistence recovery timed out".to_string())
                        .and_then(|result| result);
                        match recovery {
                            Ok(reloaded) => {
                                loaded = reloaded;
                                let recovered_at = Instant::now();
                                clock.reset(recovered_at);
                                interval.reset_at(recovered_at + state.tick_interval);
                                let _ = pending.response.send(Err(error));
                            }
                            Err(reload_error) => {
    mut handle: tokio::task::JoinHandle<()>,
) {
    match tokio::time::timeout(MATCH_ACTOR_WORKER_JOIN_TIMEOUT, &mut handle).await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            tracing::error!(%match_id, %error, worker = label, "match actor worker join failed closed");
        }
        Err(_) => {
            tracing::error!(%match_id, worker = label, "match actor worker join timed out; cancelling");
            handle.abort();
            match tokio::time::timeout(MATCH_ACTOR_WORKER_ABORT_TIMEOUT, handle).await {
                Ok(Ok(())) => {}
                Ok(Err(error)) if error.is_cancelled() => {}
                Ok(Err(error)) => {
                    tracing::error!(%match_id, %error, worker = label, "cancelled match actor worker join failed");
                }
                Err(_) => {
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
                }
            }
        }
    }
}
