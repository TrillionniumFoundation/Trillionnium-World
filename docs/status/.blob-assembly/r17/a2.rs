async fn compact_published_tick_journal_with_timeout(
        if !matches!(
                        ActorCheckpointBoundary::Shutdown,
                            format!("terminal durable publication failed: {error}"),
                            pending.input_sequence,
                        terminal: true,
                        completion: None,
                    };
                    lifecycle.send_replace(MatchActorLifecycle::Terminalizing {
                        started_at: Instant::now(),
                    });
                    let terminal_checkpoints = checkpoint_tx.clone();
                    let terminal_checkpoint_completion =
                        terminal_checkpoint_completion_tx.clone();
                    terminal_checkpoint_handle = Some(tokio::spawn(async move {
                        let result = checkpoint_barrier_with_timeout(
                            &terminal_checkpoints,
                            job,
                            "terminal",
                        )
                        .await;
                        let _ = terminal_checkpoint_completion
                            .send(TerminalCheckpointCompletion {
                                snapshot_hash,
                                result,
                            })
                            .await;
                    }));
                    continue;
                }
                state_sequence = state_sequence.saturating_add(1);
                latest_visible_candidate_tick = loaded.simulation.tick;
                publication_candidates.send_replace(Some(ActorPublicationCandidate {
                    state: PublishedMatchState {
                        simulation: Arc::new(loaded.simulation.clone()),
                        snapshot_hash: Arc::new(snapshot_hash.clone()),
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        next_input_sequences: Arc::new(loaded.next_input_sequences.clone()),
                        phase: OnlineMatchPhase::Running,
                        result_hash: None,
                        settlement_state: "not_ready".to_string(),
                        state_sequence,
                        published_at: Instant::now(),
                    },
                    durable_db_next_sequence: loaded.next_sequence,
                    durable_db_match_revision: loaded.match_revision,
                    durable_db_next_input_sequences: loaded.next_input_sequences.clone(),
                    receipts_replayable: true,
                    publish_to_watch: true,
                }));
                if loaded.simulation.tick.is_multiple_of(MATCH_CHECKPOINT_INTERVAL_TICKS)
                    && actor_checkpoint_allowed(ActorCheckpointBoundary::Periodic, false)
                {
                    let job = MatchCheckpointJob {
                        simulation: loaded.simulation.clone(),
                        snapshot_hash,
                    candidate_simulation,
    drop(publication_candidates);
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
