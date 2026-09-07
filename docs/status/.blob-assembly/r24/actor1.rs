//A00
    }

//A01
    }

//A02
            )
            .await;
//A03
                    .write()
                    .await
//A04
}

//A05
            );
        }
//A06
}

//A07
    )
    .await
//A08
}

//A09
        )
        .await?;
        persist_terminal_publication_ack(state, match_id, actor_id, &loaded, &terminal_evidence)
            .await?;
        return Ok(None);
    }
    record_initial_actor_publication(state, match_id, actor_id, &loaded, &initial_hash).await?;
    if !match_actor_assignment_is_current(state, match_id).await? {
        return Ok(None);
    }
    let published_at = Instant::now();
    let initial = PublishedMatchState {
        simulation: Arc::new(loaded.simulation.clone()),
        snapshot_hash: Arc::new(initial_hash.clone()),
        next_sequence: loaded.next_sequence,
        match_revision: loaded.match_revision,
        next_input_sequences: Arc::new(loaded.next_input_sequences.clone()),
        phase: OnlineMatchPhase::Running,
        result_hash: None,
        settlement_state: "not_ready".to_string(),
        state_sequence: 0,
        published_at,
    };
    let (published_tx, published_rx) = watch::channel(initial);
    let (publication_acked_tx, publication_acked_rx) = watch::channel(ActorPublicationCursor {
        tick: loaded.simulation.tick,
        next_sequence: loaded.next_sequence,
        match_revision: loaded.match_revision,
        next_input_sequences: loaded.next_input_sequences.clone(),
        phase: OnlineMatchPhase::Running,
        receipts_replayable: true,
        snapshot_hash: initial_hash,
    });
    let (command_tx, command_rx) = mpsc::channel(MATCH_ACTOR_COMMAND_QUEUE);
    let clock = Arc::new(AuthorityClockTelemetry::default());
    let (lifecycle_tx, lifecycle_rx) = watch::channel(MatchActorLifecycle::Warming {
        started_at: Instant::now(),
    });
    let handle = MatchActorHandle {
        actor_id,
        commands: command_tx,
        members: loaded.members.clone(),
        published: published_rx,
        publication_acked: publication_acked_rx,
        clock,
        lifecycle: lifecycle_rx,
    };
    Ok(Some(InitializedMatchActor {
        actor_id,
        handle,
        loaded,
        commands: command_rx,
        published: published_tx,
        publication_acked: publication_acked_tx,
        lifecycle: lifecycle_tx,
    }))
}

//A0B
    };
    if row
//A0C
            );
        }
//A0D
        )
        .bind(match_id)
//A0E
        )?;
    }
//A0F
        }
    }
//A10
                );
            }
//A11
}

//A12
        );
    }
//A13
}

//A14
}

//A15
}

//A16
}

//A17
