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

fn reserve_terminal_orphan_recovery(
    registry: &mut MatchActorRegistry,
    match_id: Uuid,
) -> Option<MatchActorInitialization> {
    if registry.actors.contains_key(&match_id) || registry.initializing.contains_key(&match_id) {
        return None;
    }
    let (ready, _) = watch::channel(false);
    let initialization = MatchActorInitialization {
        token: Uuid::new_v4(),
        ready,
        kind: MatchActorInitializationKind::TerminalRecovery,
        started_at: Instant::now(),
    };
    registry
        .initializing
        .insert(match_id, initialization.clone());
    Some(initialization)
}

async fn initialize_match_actor(
    state: &AppState,
    match_id: Uuid,
) -> Result<Option<InitializedMatchActor>, String> {
    let actor_id = Uuid::new_v4();
    let Some(loaded) = load_match_actor(state, match_id).await? else {
        return Ok(None);
    };
    let initial_hash = loaded
        .simulation
        .snapshot_hash()
        .map_err(|error| error.to_string())?;
    if !match_actor_assignment_is_current(state, match_id).await? {
        return Ok(None);
    }
    if loaded.simulation.terminal() {
        adopt_actor_high_water(state, match_id, actor_id, &loaded).await?;
        let terminal_job = MatchCheckpointJob {
            simulation: loaded.simulation.clone(),
            snapshot_hash: initial_hash.clone(),
            next_sequence: loaded.next_sequence,
            match_revision: loaded.match_revision,
            terminal: true,
            completion: None,
        };
        persist_terminal_actor_checkpoint(state, match_id, &terminal_job).await?;
        let (_result, result_hash) =
            derive_terminal_result(&loaded.simulation, &loaded.match_mode)?;
        let terminal_evidence = TerminalPublicationEvidence {
            authoritative_tick: loaded.simulation.tick,
            next_sequence: loaded.next_sequence,
            match_revision: loaded.match_revision,
            next_input_sequences: loaded.next_input_sequences.clone(),
            snapshot_hash: initial_hash.clone(),
            phase: OnlineMatchPhase::Complete,
            result_hash: Some(result_hash),
            settlement_state: "staged".to_string(),
        };
        let terminal_record =
            state
                .published_tick_journal
                .new_record(PublishedTickRecordInput {
                    instance_id: state.instance_id.as_str().to_string(),
                    match_id,
                    actor_generation: actor_id,
                    actor_epoch: state.instance_epoch,
                    tick: loaded.simulation.tick,
                    next_sequence: loaded.next_sequence,
                    match_revision: loaded.match_revision,
                    next_input_sequences: loaded.next_input_sequences.clone(),
                    phase: "complete".to_string(),
                    receipts_replayable: true,
                    snapshot_hash: initial_hash,
                })?;
        record_published_tick_with_timeout(
            &state.published_tick_journal,
            terminal_record,
            loaded.next_sequence,
            loaded.match_revision,
            loaded.next_input_sequences.clone(),
        )
        .await?;
//A0A
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
        .bind(next_sequence.saturating_sub(1) as i64)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "latest command recovery event is missing".to_string())?;
        let accepted_revision = recovered_row
            .try_get::<i64, _>("accepted_match_revision")
            .map_err(|error| error.to_string())? as u64;
        if accepted_revision != match_revision {
            return Err("latest command recovery revision does not match authority".to_string());
        }
        let accepted_hash: String = recovered_row
            .try_get("accepted_snapshot_hash")
            .map_err(|error| error.to_string())?;
        let recovered: Option<Value> = recovered_row
            .try_get("post_simulation_json")
            .map_err(|error| error.to_string())?;
        let recovered = recovered.ok_or_else(|| {
            "command recovery event is missing its post-command simulation".to_string()
        })?;
        simulation = serde_json::from_value(recovered).map_err(|error| error.to_string())?;
        validate_recovery_simulation(
            &simulation,
            Some(simulation.tick),
            &accepted_hash,
            "latest command recovery state",
        )?;
        validate_recovered_command_timing(
            &simulation,
            recovered_row
                .try_get::<i64, _>("target_tick")
                .map_err(|error| error.to_string())?,
            recovered_row
                .try_get::<Option<i64>, _>("client_observed_tick")
                .map_err(|error| error.to_string())?,
            recovered_row
                .try_get::<Value, _>("order_json")
                .map_err(|error| error.to_string())?,
            "latest command recovery state",
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
