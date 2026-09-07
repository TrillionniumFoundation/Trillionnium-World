async fn compact_published_tick_journal_with_timeout(
        if !matches!(
                        ActorCheckpointBoundary::Shutdown,
                            format!("terminal durable publication failed: {error}"),
                            pending.input_sequence,
                            pending.accepted_revision,
                            &pending.accepted_snapshot_hash,
                        ) {
                            let _ = pending.response.send(Err(api_error(
                                StatusCode::SERVICE_UNAVAILABLE,
                                "durable command receipt did not match the actor barrier",
                                true,
                            )));
                            tracing::error!(%match_id, command_id = %receipt.command_id, "durable command receipt failed actor correlation");
                            break;
                        }
                        loaded.durable_recovery_tick = loaded
                            .durable_recovery_tick
                            .max(pending.durable_simulation_tick);
                        command_durable_recovery_tick_tx
                            .send_replace(loaded.durable_recovery_tick);
                        if !committed_publication_tick_is_monotonic(
                            latest_visible_candidate_tick,
                            loaded.simulation.tick,
                        ) {
                            let _ = pending.response.send(Err(api_error(
                                StatusCode::SERVICE_UNAVAILABLE,
                                "committed command publication would regress the visible tick",
                                true,
                            )));
                            tracing::error!(
                                %match_id,
                                command_id = %receipt.command_id,
                                latest_visible_candidate_tick,
                                committed_tick = loaded.simulation.tick,
                                "committed command publication failed monotonicity barrier"
                            );
                            break;
                        }
                        let snapshot_hash = match loaded.simulation.snapshot_hash() {
                            Ok(value) => value,
                            Err(error) => {
                                let _ = pending.response.send(Err(api_error(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "committed actor state could not enter the publication journal",
                                    true,
                                )));
                                tracing::error!(%match_id, %error, "committed actor snapshot failed closed");
                                break;
                            }
                        };
                        if loaded.simulation.terminal() {
                            // A terminal command receipt is withheld until the
                            // phase/result/checkpoint transaction and the final
                            // terminal journal/publication record both ACK.
                            pending_terminal_receipt = Some(PendingPublishedCommandReceipt {
                                state_sequence: 0,
                                request_hash: pending.request_hash,
                                receipt,
                                response: pending.response,
                            });
                        } else {
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
                                    phase: OnlineMatchPhase::Running,
                                    result_hash: None,
                                    settlement_state: "not_ready".to_string(),
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
                            pending_published_receipt = Some(PendingPublishedCommandReceipt {
                                state_sequence,
                                request_hash: pending.request_hash,
                                receipt,
                                response: pending.response,
                            });
                        }
                    }
                    Err(error) => {
                        debug_assert!(!actor_command_lane_publish_allowed(true, false));
                        let persistence_error = error.body.error.clone();
                        let recovery_target_tick = loaded
                            .simulation
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
                                let _ = pending.response.send(Err(api_error(
                                    StatusCode::SERVICE_UNAVAILABLE,
                                    format!("command persistence recovery failed: {reload_error}"),
                                    true,
                                )));
                                tracing::error!(
                                    %match_id,
                                    %persistence_error,
                                    %reload_error,
                                    "command persistence failure could not be reconciled"
                                );
                                break;
                            }
                        }
                    }
                }
            }
            scheduled_at = interval.tick() => {
                let observed_at = Instant::now();
                clock.record_wake(scheduled_at, observed_at, state.tick_interval);
                let warming = matches!(*lifecycle.borrow(), MatchActorLifecycle::Warming { .. });
                if warming
                    && clock
                        .snapshot(observed_at, state.tick_interval)
                        .is_some_and(|snapshot| {
                            snapshot.window_sample_count >= AUTHORITY_CLOCK_MIN_SAMPLES
                        })
                {
                    lifecycle.send_replace(MatchActorLifecycle::Running);
                }
                if terminal_checkpoint_handle.is_some()
                    || terminal_publication_sequence.is_some()
                    || terminal_ack_handle.is_some()
                {
                    continue;
                }
                if !loaded.simulation.terminal()
                    && !publication_within_recovery_budget(
                        loaded.simulation.tick.saturating_add(1),
                        *command_durable_recovery_tick_tx.borrow(),
                    )
                {
                    // A stuck checkpoint writer cannot let the public high-water
                    // cross the bounded deterministic replay horizon. Resume
                    // automatically when a checkpoint or command event advances
                    // the durable recovery tick.
                    continue;
                }
                if pending_published_receipt.is_some() {
                    if !loaded.simulation.terminal() {
                        if let Err(error) = loaded.simulation.step() {
                            tracing::error!(%match_id, %error, "match simulation failed while awaiting durable publication");
                            break;
                        }
                    }
                    continue;
                }
                if pending_command.is_some() {
                    debug_assert!(!actor_command_lane_publish_allowed(true, false));
                    if !loaded.simulation.terminal() {
                        if let Err(error) = loaded.simulation.step() {
                            tracing::error!(%match_id, %error, "speculative match simulation failed closed");
                            break;
                        }
                    }
                    // Freeze the old public cursor while PostgreSQL decides the
                    // command. Publishing an independently stepped old lane can
                    // create an unrecoverable old-cursor/high-tick record after
                    // the database commits a new-cursor/low-tick event.
                    continue;
                }
                if !loaded.simulation.terminal() {
                    if let Err(error) = loaded.simulation.step() {
                        tracing::error!(%match_id, %error, "match actor simulation failed closed");
                        break;
                    }
                }
                let snapshot_hash = match loaded.simulation.snapshot_hash() {
                    Ok(value) => value,
                    Err(error) => {
                        tracing::error!(%match_id, %error, "match actor snapshot failed closed");
                        break;
                    }
                };
                if loaded.simulation.terminal() {
                    debug_assert!(actor_checkpoint_allowed(
                        ActorCheckpointBoundary::Terminal,
                        false,
                    ));
                    let job = MatchCheckpointJob {
                        simulation: loaded.simulation.clone(),
                        snapshot_hash: snapshot_hash.clone(),
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: true,
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
