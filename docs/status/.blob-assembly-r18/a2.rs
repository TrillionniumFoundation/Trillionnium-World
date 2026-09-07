@@TRNM_R18_A2_SEGMENT_0@@
            }
            publication_acked.send_replace(acknowledged_cursor);
        }
        let failed = result.is_err();
        if !matches!(
            tokio::time::timeout(
                MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                completions.send(ActorPublicationCompletion {
@@TRNM_R18_A2_SEGMENT_1@@
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
                            true,
@@TRNM_R18_A2_SEGMENT_3@@
                            &pending.command_id,
                            &pending.player_id,
                            pending.sequence,
                            pending.input_sequence,
                            pending.accepted_revision,
                            &pending.accepted_snapshot_hash,
                        ) {
                            let _ = pending.response.send(Err(api_error(
@@TRNM_R18_A2_SEGMENT_4@@
                        simulation: loaded.simulation.clone(),
                        snapshot_hash: snapshot_hash.clone(),
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: true,
                        completion: None,
                    };
                    lifecycle.send_replace(MatchActorLifecycle::Terminalizing {
@@TRNM_R18_A2_SEGMENT_5@@
