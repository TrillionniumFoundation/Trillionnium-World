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

struct JournalOperationGuard<'a> {
    journal: &'a PublishedTickJournal,
    completed: bool,
}

impl<'a> JournalOperationGuard<'a> {
    fn new(journal: &'a PublishedTickJournal) -> Self {
        Self {
            journal,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

impl Drop for JournalOperationGuard<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.journal.fail_closed();
        }
    }
}

fn release_deferred_terminal_publication(
    deferred: DeferredTerminalPublication,
    published: &watch::Sender<PublishedMatchState>,
    publication_acked: &watch::Sender<ActorPublicationCursor>,
) -> Result<(), String> {
    if deferred.state.phase != OnlineMatchPhase::Complete
        || deferred.cursor.phase != OnlineMatchPhase::Complete
        || !publication_tuple_matches(&deferred.state, &deferred.cursor)
        || TerminalPublicationEvidence::from_state(&deferred.state).as_ref()
            != Some(&deferred.evidence)
    {
        return Err(
            "deferred terminal publication tuple changed before marker acknowledgement".to_string(),
        );
    }
    if deferred.publish_to_watch {
        published.send_replace(deferred.state);
    }
    publication_acked.send_replace(deferred.cursor);
    Ok(())
}

fn bind_deferred_terminal_commit(
    deferred: &mut DeferredTerminalPublication,
    commit: &TerminalPublicationCommit,
) -> Result<(), String> {
    if deferred.state.phase != OnlineMatchPhase::Complete
        || deferred.state.result_hash.as_deref() != Some(commit.result_hash.as_str())
        || !matches!(commit.settlement_state.as_str(), "pending" | "settled")
        || commit.acknowledged_at_unix_ms == 0
    {
        return Err("terminal database commit did not match its deferred HWM tuple".to_string());
    }
    deferred.state.settlement_state = commit.settlement_state.clone();
    deferred.evidence = TerminalPublicationEvidence::from_state(&deferred.state)
        .ok_or_else(|| "terminal commit could not rebuild publication evidence".to_string())?;
    if !terminal_publication_metadata_matches_durable(
        &deferred.evidence,
        Some(&commit.result_hash),
        &commit.settlement_state,
    ) {
        return Err("terminal commit metadata did not match its deferred publication".to_string());
    }
    Ok(())
}

async fn run_actor_publication_worker(worker: ActorPublicationWorker) {
    let ActorPublicationWorker {
        journal,
        instance_id,
        match_id,
        actor_id,
        actor_epoch,
        mut candidates,
        mut permit,
        durable_recovery_tick,
        published,
        publication_acked,
        completions,
    } = worker;
    loop {
        tokio::select! {
            changed = permit.changed() => {
                if changed.is_err() || !*permit.borrow() {
                    return;
                }
                continue;
            }
            changed = candidates.changed() => {
                if changed.is_err() {
                    return;
                }
            }
        }
        if !*permit.borrow() {
            return;
        }
        let Some(candidate) = candidates.borrow_and_update().clone() else {
            continue;
        };
        let state_sequence = candidate.state.state_sequence;
        let acknowledged_cursor = ActorPublicationCursor {
            tick: candidate.state.simulation.tick,
            next_sequence: candidate.state.next_sequence,
            match_revision: candidate.state.match_revision,
            next_input_sequences: candidate.state.next_input_sequences.as_ref().clone(),
            phase: candidate.state.phase,
            receipts_replayable: candidate.receipts_replayable,
            snapshot_hash: candidate.state.snapshot_hash.as_ref().clone(),
        };
        let journal_started = Instant::now();
        let result = if !publication_within_recovery_budget(
            candidate.state.simulation.tick,
            *durable_recovery_tick.borrow(),
        ) {
            Err(format!(
                "published tick {} exceeded durable recovery tick {} plus the bounded {}-tick replay budget",
                candidate.state.simulation.tick,
                *durable_recovery_tick.borrow(),
                MAX_PUBLISHED_TICK_RECOVERY_STEPS,
            ))
        } else {
            let record = journal.new_record(PublishedTickRecordInput {
                instance_id: instance_id.as_str().to_string(),
                match_id,
                actor_generation: actor_id,
                actor_epoch,
                tick: candidate.state.simulation.tick,
                next_sequence: candidate.state.next_sequence,
                match_revision: candidate.state.match_revision,
                next_input_sequences: candidate.state.next_input_sequences.as_ref().clone(),
                phase: match candidate.state.phase {
                    OnlineMatchPhase::Running => "running".to_string(),
                    OnlineMatchPhase::Complete => "complete".to_string(),
                    _ => "invalid".to_string(),
                },
                receipts_replayable: candidate.receipts_replayable,
                snapshot_hash: candidate.state.snapshot_hash.as_ref().clone(),
            });
            match record {
                Ok(record) => {
                    record_published_tick_with_timeout(
                        &journal,
                        record,
                        candidate.durable_db_next_sequence,
                        candidate.durable_db_match_revision,
                        candidate.durable_db_next_input_sequences,
                    )
                    .await
                }
                Err(error) => Err(error),
            }
        };
        let journal_ms = u64::try_from(journal_started.elapsed().as_millis()).unwrap_or(u64::MAX);
        if journal_ms > 100 {
            tracing::warn!(%match_id, state_sequence, journal_ms, "slow published-tick journal commit");
        }
        if !*permit.borrow() {
            return;
        }
        // A terminal HWM proves what may be recovered after a crash, but it is
        // not yet the semantic publication ACK. Keep both public channels on
        // their prior running tuple until the actor commits the exact database
        // marker and explicitly releases this deferred terminal value.
        let deferred_terminal = result
            .is_ok()
            .then(|| TerminalPublicationEvidence::from_state(&candidate.state))
            .flatten()
            .map(|evidence| DeferredTerminalPublication {
                state: candidate.state.clone(),
                cursor: acknowledged_cursor.clone(),
                evidence,
                publish_to_watch: candidate.publish_to_watch,
            });
        if result.is_ok() && deferred_terminal.is_none() {
            if candidate.publish_to_watch {
                published.send_replace(candidate.state);
            }
            publication_acked.send_replace(acknowledged_cursor);
        }
        let failed = result.is_err();
        if !matches!(
            tokio::time::timeout(
                MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                completions.send(ActorPublicationCompletion {
                    state_sequence,
                    deferred_terminal,
                    result,
                }),
            )
            .await,
            Ok(Ok(()))
        ) {
            return;
        }
        if failed {
            return;
        }
    }
}

fn publication_within_recovery_budget(candidate_tick: u64, durable_tick: u64) -> bool {
    candidate_tick <= durable_tick.saturating_add(MAX_PUBLISHED_TICK_RECOVERY_STEPS)
}

async fn checkpoint_barrier_with_timeout(
    checkpoints: &mpsc::Sender<MatchCheckpointJob>,
    job: MatchCheckpointJob,
    label: &str,
) -> Result<(), String> {
    checkpoint_barrier_with_deadline(
        checkpoints,
        job,
        label,
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
    )
    .await
}

async fn checkpoint_barrier_with_deadline(
    checkpoints: &mpsc::Sender<MatchCheckpointJob>,
    mut job: MatchCheckpointJob,
    label: &str,
    deadline: Duration,
) -> Result<(), String> {
    let (completion, completed) = oneshot::channel();
    job.completion = Some(completion);
    tokio::time::timeout(deadline, async {
        checkpoints
            .send(job)
            .await
            .map_err(|_| format!("{label} checkpoint worker is unavailable"))?;
        completed
            .await
            .map_err(|_| format!("{label} checkpoint worker stopped before acknowledgement"))?
    })
    .await
    .map_err(|_| format!("{label} checkpoint barrier exceeded its hard timeout"))?
}

async fn run_match_actor(state: AppState, match_id: Uuid, initialized: InitializedMatchActor) {
    let InitializedMatchActor {
        actor_id,
        handle,
        mut loaded,
        mut commands,
        published,
        publication_acked,
        lifecycle,
    } = initialized;
    let MatchActorHandle { clock, .. } = handle;
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
            changed = checkpoint_failed_rx.changed() => {
                if changed.is_err() {
                    debug_assert!(!actor_checkpoint_allowed(
                        ActorCheckpointBoundary::CheckpointFailure,
                        pending_command.is_some(),
                    ));
                    if let Some(pending) = pending_command.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "checkpoint worker stopped during command persistence",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "publication checkpoint worker stopped during command acknowledgement",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "checkpoint worker stopped during terminal publication",
                            true,
                        )));
                    }
                    tracing::error!(%match_id, "match actor checkpoint writer stopped unexpectedly");
                    break;
                }
                if let Some(error) = checkpoint_failed_rx.borrow().as_deref() {
                    debug_assert!(!actor_checkpoint_allowed(
                        ActorCheckpointBoundary::CheckpointFailure,
                        pending_command.is_some(),
                    ));
                    if let Some(pending) = pending_command.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "checkpoint writer failed during command persistence",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "checkpoint writer failed during the durable publication barrier",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "checkpoint writer failed during terminal publication",
                            true,
                        )));
                    }
                    tracing::error!(%match_id, %error, "match actor stopped after checkpoint failure");
                    break;
                }
            }
            completion = terminal_checkpoint_completion_rx.recv(),
                if terminal_checkpoint_handle.is_some() => {
                terminal_checkpoint_handle.take();
                let Some(TerminalCheckpointCompletion { snapshot_hash, result }) = completion else {
                    tracing::error!(%match_id, "terminal checkpoint task stopped before acknowledgement");
                    break;
                };
                if let Err(error) = result {
                    tracing::error!(%match_id, %error, "terminal actor checkpoint failed closed");
                    break;
                }
                match derive_terminal_result(&loaded.simulation, &loaded.match_mode) {
                    Ok((_result, result_hash)) => {
                        state_sequence = state_sequence.saturating_add(1);
                        publication_candidates.send_replace(Some(ActorPublicationCandidate {
                            state: PublishedMatchState {
                                simulation: Arc::new(loaded.simulation.clone()),
                                snapshot_hash: Arc::new(snapshot_hash),
                                next_sequence: loaded.next_sequence,
                                match_revision: loaded.match_revision,
                                next_input_sequences: Arc::new(
                                    loaded.next_input_sequences.clone(),
                                ),
                                phase: OnlineMatchPhase::Complete,
                                result_hash: Some(result_hash),
                                // Internal-only until the fenced ACK transaction
                                // returns the committed settlement state.
                                settlement_state: "staged".to_string(),
                                state_sequence,
                                published_at: Instant::now(),
                            },
                            durable_db_next_sequence: loaded.next_sequence,
                            durable_db_match_revision: loaded.match_revision,
                            durable_db_next_input_sequences: loaded
                                .next_input_sequences
                                .clone(),
                            receipts_replayable: true,
                            publish_to_watch: true,
                        }));
                        terminal_publication_sequence = Some(state_sequence);
                        if let Some(pending) = pending_terminal_receipt.as_mut() {
                            pending.state_sequence = state_sequence;
                        }
                    }
                    Err(error) => {
                        tracing::error!(
                            %match_id,
                            %error,
                            "terminal stage committed but its deterministic result could not be derived"
                        );
                        break;
                    }
                }
            }
            completion = publication_completion_rx.recv() => {
                let Some(completion) = completion else {
                    if let Some(pending) = pending_published_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "published-tick journal stopped before command acknowledgement",
                            true,
                        )));
                    }
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "published-tick journal stopped before terminal acknowledgement",
                            true,
                        )));
                    }
                    tracing::error!(%match_id, "published-tick journal worker stopped unexpectedly");
                    break;
                };
                let ActorPublicationCompletion {
                    state_sequence: completed_sequence,
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
                            pending.input_sequence,
                        terminal: true,
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
