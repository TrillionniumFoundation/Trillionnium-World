async fn compact_published_tick_journal_with_timeout(
        if !matches!(
                        ActorCheckpointBoundary::Shutdown,
                            format!("terminal durable publication failed: {error}"),
                            pending.input_sequence,
                        terminal: true,
                        snapshot_hash,
                    candidate_simulation,
                    persistence,
                } = prepared;
                let command_id = persistence.request.command_id.clone();
                let player_id = persistence.request.player_id.clone();
                let request_hash = persistence.request_hash.clone();
                let input_sequence = persistence.input_sequence;
                let accepted_revision = persistence.accepted_revision;
                let sequence = persistence.base_next_sequence;
                let accepted_snapshot_hash = persistence.snapshot_hash.clone();
                let visible = loaded.clone();
                let durable_simulation_tick = candidate_simulation.tick;
                if persistence_tx.try_send(persistence).is_err() {
                    let _ = response.send(Err(api_error(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "command persistence queue is unavailable",
                        true,
                    )));
                    continue;
                }
                loaded.simulation = candidate_simulation;
                loaded.next_sequence = sequence.saturating_add(1);
                loaded.match_revision = accepted_revision;
                loaded
                    .next_input_sequences
                    .insert(player_id.clone(), input_sequence.saturating_add(1));
                pending_command = Some(PendingActorCommand {
                    command_id,
                    player_id,
                    request_hash,
                    input_sequence,
                    accepted_revision,
                    sequence,
                    accepted_snapshot_hash,
                    durable_simulation_tick,
                    visible,
                    response,
                });
            }
        }
    }
    publication_permit_tx.send_replace(false);
    commands.close();
    if let Some(handle) = terminal_checkpoint_handle.take() {
        stop_actor_worker(match_id, "terminal checkpoint", handle).await;
    }
    if let Some(handle) = terminal_ack_handle.take() {
        stop_actor_worker(match_id, "terminal ACK", handle).await;
    }
    drop(persistence_tx);
    drop(persistence_completion_rx);
    drop(publication_candidates);
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
