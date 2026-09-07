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
@@TRNM_R18_A2_SEGMENT_2@@
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
                        break;
                    };
                    if terminal_ack_handle.is_some() {
                        tracing::error!(%match_id, "terminal publication produced two ACK tasks");
                        break;
                    }
                    let ack_state = state.clone();
                    let ack_loaded = loaded.clone();
                    let ack_completion = terminal_ack_completion_tx.clone();
                    terminal_ack_handle = Some(tokio::spawn(async move {
                        let result = tokio::time::timeout(
                            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
                            persist_terminal_publication_ack(
                                &ack_state,
                                match_id,
                                actor_id,
                                &ack_loaded,
                                &deferred_terminal.evidence,
                            ),
                        )
                        .await
                        .map_err(|_| "terminal publication ACK marker timed out".to_string())
                        .and_then(|result| result);
                        let _ = ack_completion
                            .send(TerminalAckCompletion {
                                deferred_terminal,
                                result,
                            })
                            .await;
                    }));
                    continue;
                }
                if pending_published_receipt
                    .as_ref()
                    .is_some_and(|pending| pending.state_sequence == completed_sequence)
                {
                    let pending = pending_published_receipt
                        .take()
                        .expect("publication sequence was checked above");
                    cache_actor_receipt(
                        &mut receipt_cache,
                        &mut receipt_cache_order,
                        pending.request_hash,
                        &pending.receipt,
                    );
                    let _ = pending.response.send(Ok(pending.receipt));
                }
            }
            completion = terminal_ack_completion_rx.recv(), if terminal_ack_handle.is_some() => {
                terminal_ack_handle.take();
                let Some(TerminalAckCompletion {
                    mut deferred_terminal,
                    result,
                }) = completion else {
                    tracing::error!(%match_id, "terminal ACK task stopped before acknowledgement");
                    break;
                };
                let commit = match result {
                    Ok(commit) => commit,
                    Err(error) => {
                        if let Some(pending) = pending_terminal_receipt.take() {
                            let _ = pending.response.send(Err(api_error(
                                StatusCode::SERVICE_UNAVAILABLE,
                                format!("terminal publication ACK marker failed: {error}"),
                                true,
                            )));
                        }
                        tracing::error!(%match_id, %error, "terminal publication ACK marker failed closed");
                        break;
                    }
                };
                if let Err(error) = bind_deferred_terminal_commit(&mut deferred_terminal, &commit) {
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            format!("terminal committed tuple binding failed: {error}"),
                            true,
                        )));
                    }
                    tracing::error!(%match_id, %error, "terminal commit could not bind its deferred HWM tuple");
                    break;
                }
                if let Err(error) = release_deferred_terminal_publication(
                    deferred_terminal,
                    &terminal_published,
                    &terminal_publication_acked,
                ) {
                    if let Some(pending) = pending_terminal_receipt.take() {
                        let _ = pending.response.send(Err(api_error(
                            StatusCode::SERVICE_UNAVAILABLE,
                            format!("terminal public release failed: {error}"),
                            true,
                        )));
                    }
                    tracing::error!(%match_id, %error, "terminal marker committed but public tuple release failed closed");
                    break;
                }
                let completed_sequence = terminal_publication_sequence
                    .expect("terminal ACK task requires a publication sequence");
                if pending_terminal_receipt
                    .as_ref()
                    .is_some_and(|pending| pending.state_sequence == completed_sequence)
                {
                    let pending = pending_terminal_receipt
                        .take()
                        .expect("terminal publication sequence was checked above");
                    cache_actor_receipt(
                        &mut receipt_cache,
                        &mut receipt_cache_order,
                        pending.request_hash,
                        &pending.receipt,
                    );
                    let _ = pending.response.send(Ok(pending.receipt));
                }
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
                    Ok(receipt) => {
                        debug_assert!(actor_command_lane_publish_allowed(true, true));
                        if !committed_receipt_matches_pending(
                            &receipt,
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
