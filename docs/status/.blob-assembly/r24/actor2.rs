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

//B01
            ))
        } else {
//B02
            }
        };
//B03
}

//B04
        .await;
    });
//B05
        }
    }
//B06
                }
            }
//B07
                }
            }
//B08
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
                        )));
                    }
                    tracing::error!(%match_id, %error, "published-tick durability failed closed");
                    break;
                }
//B0B
                    continue;
                }
//B0C
                    }
                };
//B0D
                break;
            }
            completion = persistence_completion_rx.recv(), if pending_command.is_some() => {
                let Some(completion) = completion else {
                    if let Some(pending) = pending_command.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            "command persistence worker stopped before acknowledgement",
                            true,
                        )));
                    }
                    tracing::error!(%match_id, "match actor command persistence worker stopped unexpectedly");
                    break;
                };
                let Some(pending) = pending_command.take() else {
                    tracing::error!(%match_id, "command persistence completed without an actor barrier");
                    break;
                };
                if completion.command_id != pending.command_id {
                    let _ = pending.response.send(Err(api_error(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "command persistence completion did not match the actor barrier",
                        true,
                    )));
                    tracing::error!(
                        %match_id,
                        expected_command_id = %pending.command_id,
                        completed_command_id = %completion.command_id,
                        "match actor command persistence ordering failed closed"
                    );
                    break;
                }
                match completion.result {
                    Ok(receipt) if receipt.duplicate => {
                        debug_assert!(!actor_command_lane_publish_allowed(true, false));
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
                        .map_err(|_| "duplicate command recovery timed out".to_string())
                        .and_then(|result| result);
                        match recovery {
                            Ok(reloaded) => {
                                loaded = reloaded;
                                let recovered_at = Instant::now();
                                clock.reset(recovered_at);
                                interval.reset_at(recovered_at + state.tick_interval);
                                cache_actor_receipt(
                                    &mut receipt_cache,
                                    &mut receipt_cache_order,
                                    pending.request_hash,
                                    &receipt,
                                );
                                let _ = pending.response.send(Ok(receipt));
                            }
                            Err(error) => {
                                let _ = pending.response.send(Err(api_error(
                                    StatusCode::SERVICE_UNAVAILABLE,
                                    format!("duplicate command recovery failed: {error}"),
                                    true,
                                )));
                                tracing::error!(%match_id, %error, "duplicate command race failed closed");
                                break;
                            }
                        }
                    }
//B0F
                            break;
                        }
//B10
                            }
                        };
//B11
                        }
                    }
//B12
                }
            }
//B13
                    }
                }
//B14
                    continue;
                }
//B15
                    }
                };
//B16
        }
    }
//B17
