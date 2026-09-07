async fn compact_published_tick_journal_with_timeout(
        if !matches!(
                        ActorCheckpointBoundary::Shutdown,
                            format!("terminal durable publication failed: {error}"),
                            pending.input_sequence,
                        terminal: true,
                        snapshot_hash,
                    candidate_simulation,
    drop(publication_candidates);
    drop(publication_completion_rx);
    drop(checkpoint_tx);
    drop(terminal_checkpoint_completion_rx);
    drop(terminal_ack_completion_rx);
    drop(checkpoint_failed_rx);
    drop(fenced_rx);
    stop_actor_worker(match_id, "command persistence", persistence_handle).await;
    stop_actor_worker(match_id, "checkpoint", checkpoint_handle).await;
    stop_actor_worker(match_id, "publication", publication_handle).await;
    stop_actor_worker(match_id, "fence", fence_handle).await;
    let mut registry = state.match_actors.write().await;
    if registry
        .actors
        .get(&match_id)
        .is_some_and(|handle| handle.actor_id == actor_id)
    {
        registry.actors.remove(&match_id);
    }
}

async fn shutdown_changed_or_current(
    shutdown: &mut watch::Receiver<bool>,
) -> Result<(), watch::error::RecvError> {
    if *shutdown.borrow() {
        Ok(())
    } else {
        shutdown.changed().await
    }
}

async fn stop_actor_worker(
    match_id: Uuid,
    label: &'static str,
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
