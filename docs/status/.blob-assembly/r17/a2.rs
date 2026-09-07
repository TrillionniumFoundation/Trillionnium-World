async fn compact_published_tick_journal_with_timeout(
        if !matches!(
                        ActorCheckpointBoundary::Shutdown,
                            format!("terminal durable publication failed: {error}"),
                            pending.input_sequence,
                        terminal: true,
                        snapshot_hash,
                        next_sequence: loaded.next_sequence,
                        match_revision: loaded.match_revision,
                        terminal: false,
                        completion: None,
                    };
                    if checkpoint_tx.try_send(job).is_err() {
                        tracing::warn!(%match_id, tick = loaded.simulation.tick, "checkpoint writer busy; latest durable checkpoint retained");
                    }
                }
            }
            envelope = commands.recv(), if pending_command.is_none()
                && pending_published_receipt.is_none()
                && pending_terminal_receipt.is_none()
                && terminal_publication_sequence.is_none() => {
                let Some(envelope) = envelope else {
                    break;
                };
                let ActorCommandEnvelope {
                    request,
                    request_hash,
                    admission,
                    controlled_unit_ids,
                    member_role,
                    response,
                } = envelope;
                if let Some(result) = cached_actor_command_result(
                    &receipt_cache,
                    &request.command_id,
                    &request.player_id,
                    &request_hash,
                    loaded.match_revision,
                ) {
                    let _ = response.send(result);
                    continue;
                }
                let prepared = match prepare_actor_command(
                    &loaded,
                    request,
                    request_hash,
                    admission,
                    &controlled_unit_ids,
                    &member_role,
                ) {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        let _ = response.send(Err(error));
                        continue;
                    }
                };
                let PreparedActorCommand {
                    candidate_simulation,
    drop(publication_candidates);
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
