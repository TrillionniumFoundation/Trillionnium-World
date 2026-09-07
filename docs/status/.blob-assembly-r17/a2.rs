@@TRNM_A2_R17_CHUNK_0@@
    if *state.shutdown.borrow() {
        let mut registry = state.match_actors.write().await;
        if registry
            .actors
            .get(&match_id)
            .is_some_and(|handle| handle.actor_id == actor_id)
        {
            registry.actors.remove(&match_id);
        }
        return;
    }
    let mut state_sequence = 0_u64;
    let mut latest_visible_candidate_tick = loaded.simulation.tick;
    let mut pending_command = None::<PendingActorCommand>;
    let mut pending_published_receipt = None::<PendingPublishedCommandReceipt>;
    let mut pending_terminal_receipt = None::<PendingPublishedCommandReceipt>;
    let mut terminal_publication_sequence = None::<u64>;
    let mut receipt_cache = BTreeMap::<String, ActorReceiptCacheEntry>::new();
    let mut receipt_cache_order = VecDeque::<String>::new();
    let (persistence_tx, persistence_rx) = mpsc::channel(MATCH_COMMAND_PERSISTENCE_QUEUE);
    let (persistence_completion_tx, mut persistence_completion_rx) =
        mpsc::channel(MATCH_COMMAND_PERSISTENCE_QUEUE);
    let persistence_state = state.clone();
    let persistence_handle = tokio::spawn(async move {
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
            instance_id: publication_instance_id,
            match_id,
            actor_id,
            actor_epoch: publication_epoch,
            candidates: publication_candidate_rx,
            permit: publication_permit_rx,
            durable_recovery_tick: durable_recovery_tick_rx,
            published,
            publication_acked,
            completions: publication_completion_tx,
        })
        .await;
    });
    let (checkpoint_tx, checkpoint_rx) = mpsc::channel(MATCH_CHECKPOINT_QUEUE);
    let (terminal_checkpoint_completion_tx, mut terminal_checkpoint_completion_rx) =
        mpsc::channel::<TerminalCheckpointCompletion>(1);
    let mut terminal_checkpoint_handle = None::<tokio::task::JoinHandle<()>>;
    let (terminal_ack_completion_tx, mut terminal_ack_completion_rx) =
        mpsc::channel::<TerminalAckCompletion>(1);
    let mut terminal_ack_handle = None::<tokio::task::JoinHandle<()>>;
    let (checkpoint_failed_tx, mut checkpoint_failed_rx) = watch::channel(None::<String>);
    let checkpoint_state = state.clone();
    let checkpoint_handle = tokio::spawn(async move {
        run_match_checkpoint_writer(
            checkpoint_state,
            match_id,
            checkpoint_rx,
            checkpoint_failed_tx,
            durable_recovery_tick_tx,
        )
        .await;
    });
    let (fenced_tx, mut fenced_rx) = watch::channel(false);
    let fence_state = state.clone();
    let fence_publication_permit = publication_permit_tx.clone();
    let fence_handle = tokio::spawn(async move {
        run_match_fence_monitor(fence_state, match_id, fence_publication_permit, fenced_tx).await;
    });
    if loaded.simulation.tick > loaded.durable_recovery_tick {
        match loaded.simulation.snapshot_hash() {
            Ok(snapshot_hash) => {
                let _ = checkpoint_tx.try_send(MatchCheckpointJob {
                    simulation: loaded.simulation.clone(),
                    snapshot_hash,
                    next_sequence: loaded.next_sequence,
                    match_revision: loaded.match_revision,
                    terminal: false,
                    completion: None,
                });
            }
            Err(error) => {
                tracing::error!(%match_id, %error, "recovered actor startup checkpoint hash failed");
            }
        }
    }
    let started_at = Instant::now();
    clock.reset(started_at);
    let mut shutdown = state.shutdown.subscribe();
    let mut interval =
        tokio::time::interval_at(started_at + state.tick_interval, state.tick_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            biased;
            changed = shutdown_changed_or_current(&mut shutdown) => {
                if changed.is_err() || *shutdown.borrow() {
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "server shutdown interrupted the durable publication barrier",
                            true,
                        )));
                        break;
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "server shutdown interrupted terminal publication",
                            true,
                        )));
                        break;
                    }
                    if terminal_checkpoint_handle.is_some()
                        || terminal_publication_sequence.is_some()
                        || terminal_ack_handle.is_some()
                    {
                        // The terminal pipeline is already fenced and durable at
                        // one of its crash-recoverable boundaries. Do not start a
                        // second checkpoint while shutdown drains its workers.
                        break;
                    }
                    if !actor_checkpoint_allowed(
                        ActorCheckpointBoundary::Shutdown,
                        pending_command.is_some(),
                    ) {
                        let pending = pending_command
                            .take()
                            .expect("pending command must own the shutdown checkpoint barrier");
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "server shutdown interrupted command persistence",
                            true,
                        )));
                        break;
                    }
                    let snapshot_hash = match loaded.simulation.snapshot_hash() {
                        Ok(value) => value,
                        Err(error) => {
                            tracing::error!(%match_id, %error, "shutdown actor snapshot failed closed");
                            break;
                        }
                    };
                    let job = MatchCheckpointJob {
                        simulation: loaded.simulation.clone(),
                        snapshot_hash: snapshot_hash.clone(),
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: false,
                        completion: None,
                    };
                    let result = checkpoint_barrier_with_timeout(&checkpoint_tx, job, "shutdown").await;
                    if let Err(error) = result {
                        tracing::error!(%match_id, %error, "shutdown actor checkpoint failed closed");
                    }
                    break;
                }
            }
            changed = fenced_rx.changed() => {
                if changed.is_err() || *fenced_rx.borrow() {
                    debug_assert!(!actor_checkpoint_allowed(
                        ActorCheckpointBoundary::Fenced,
                        pending_command.is_some(),
                    ));
                    if let Some(pending) = pending_command.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "match actor was fenced during command persistence",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "match actor was fenced during the durable publication barrier",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "match actor was fenced during terminal publication",
                            true,
                        )));
                    }
                    tracing::warn!(%match_id, "match actor stopped after fleet epoch fencing");
                    break;
                }
            }
@@TRNM_A2_R17_CHUNK_2@@
@@TRNM_A2_R17_CHUNK_3@@
@@TRNM_A2_R17_CHUNK_4@@
@@TRNM_A2_R17_CHUNK_5@@
