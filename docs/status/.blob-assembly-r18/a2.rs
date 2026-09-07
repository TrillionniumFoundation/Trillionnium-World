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
