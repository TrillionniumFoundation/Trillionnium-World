pub fn production_authority_tick_interval() -> Duration {
    assert_eq!(1_000 % TICKS_PER_SECOND, 0);
    Duration::from_millis(1_000 / TICKS_PER_SECOND)
}

pub fn resolve_authority_tick_interval(
    requested_ms: Option<u64>,
    allow_accelerated_test_clock: bool,
) -> Result<Duration, String> {
    let production = production_authority_tick_interval();
    let requested = Duration::from_millis(
        requested_ms.unwrap_or_else(|| u64::try_from(production.as_millis()).unwrap_or(100)),
    );
    if requested.is_zero() || requested > Duration::from_secs(1) {
        return Err("TRNM_GAME_SERVER_TICK_MS must be between 1 and 1000".to_string());
    }
    if requested != production && !allow_accelerated_test_clock {
        return Err(format!(
            "production authority is fixed at {}ms ({}Hz); non-real-time clocks require TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1",
            production.as_millis(),
            TICKS_PER_SECOND
        ));
    }
    Ok(requested)
}

pub async fn run_authority_loop(state: AppState, tick_interval: Duration) {
    let initial_database_fence = tokio::time::timeout(
        DATABASE_HOST_FENCE_MONITOR_TIMEOUT,
        state.verify_database_host_authority_session(),
    )
    .await;
    if !matches!(initial_database_fence, Ok(Ok(()))) {
        tracing::error!("initial PostgreSQL host-authority verification failed closed");
        state.fail_database_host_authority();
        return;
    }

    let database_fence_state = state.clone();
    let database_fence_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(DATABASE_HOST_FENCE_MONITOR_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        interval.tick().await;
        loop {
            interval.tick().await;
            let verification = tokio::time::timeout(
                DATABASE_HOST_FENCE_MONITOR_TIMEOUT,
                database_fence_state.verify_database_host_authority_session(),
            )
            .await;
            match verification {
                Ok(Ok(())) => {}
                Ok(Err(error)) => {
                    tracing::error!(%error, "PostgreSQL host-authority monitor failed closed");
                    database_fence_state.fail_database_host_authority();
                    return;
                }
                Err(_) => {
                    tracing::error!("PostgreSQL host-authority monitor exceeded its hard timeout");
                    database_fence_state.fail_database_host_authority();
                    return;
                }
            }
        }
    });

    if let Err(error) = operations_v1::heartbeat_fleet(&state).await {
        tracing::error!(%error, "initial online fleet heartbeat failed closed");
        state.fail_database_host_authority();
        database_fence_task.abort();
        return;
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
