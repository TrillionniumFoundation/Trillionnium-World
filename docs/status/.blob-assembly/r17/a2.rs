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

fn bind_deferred_terminal_commit(
    deferred: &mut DeferredTerminalPublication,
    commit: &TerminalPublicationCommit,
) -> Result<(), String> {
    if deferred.state.phase != OnlineMatchPhase::Complete
        || deferred.state.result_hash.as_deref() != Some(commit.result_hash.as_str())
        || !matches!(commit.settlement_state.as_str(), "pending" | "settled")
        || commit.acknowledged_at_unix_ms == 0
    {
        return Err("terminal database commit did not match its deferred HWM tuple".to_string());
    }
    deferred.state.settlement_state = commit.settlement_state.clone();
    deferred.evidence = TerminalPublicationEvidence::from_state(&deferred.state)
        .ok_or_else(|| "terminal commit could not rebuild publication evidence".to_string())?;
    if !terminal_publication_metadata_matches_durable(
        &deferred.evidence,
        Some(&commit.result_hash),
        &commit.settlement_state,
    ) {
        return Err("terminal commit metadata did not match its deferred publication".to_string());
    }
    Ok(())
}

async fn run_actor_publication_worker(worker: ActorPublicationWorker) {
    let ActorPublicationWorker {
        journal,
        instance_id,
        match_id,
        actor_id,
        actor_epoch,
        mut candidates,
        mut permit,
        durable_recovery_tick,
        published,
        publication_acked,
        completions,
    } = worker;
    loop {
        tokio::select! {
            changed = permit.changed() => {
                if changed.is_err() || !*permit.borrow() {
                    return;
                }
                continue;
            }
            changed = candidates.changed() => {
                if changed.is_err() {
                    return;
                }
            }
        }
        if !*permit.borrow() {
            return;
        }
        let Some(candidate) = candidates.borrow_and_update().clone() else {
            continue;
        };
        let state_sequence = candidate.state.state_sequence;
        let acknowledged_cursor = ActorPublicationCursor {
            tick: candidate.state.simulation.tick,
            next_sequence: candidate.state.next_sequence,
            match_revision: candidate.state.match_revision,
            next_input_sequences: candidate.state.next_input_sequences.as_ref().clone(),
            phase: candidate.state.phase,
            receipts_replayable: candidate.receipts_replayable,
            snapshot_hash: candidate.state.snapshot_hash.as_ref().clone(),
        };
        let journal_started = Instant::now();
        let result = if !publication_within_recovery_budget(
            candidate.state.simulation.tick,
            *durable_recovery_tick.borrow(),
        ) {
            Err(format!(
                "published tick {} exceeded durable recovery tick {} plus the bounded {}-tick replay budget",
                candidate.state.simulation.tick,
                *durable_recovery_tick.borrow(),
                MAX_PUBLISHED_TICK_RECOVERY_STEPS,
            ))
        } else {
            let record = journal.new_record(PublishedTickRecordInput {
                instance_id: instance_id.as_str().to_string(),
                match_id,
                actor_generation: actor_id,
                actor_epoch,
                tick: candidate.state.simulation.tick,
                next_sequence: candidate.state.next_sequence,
                match_revision: candidate.state.match_revision,
                next_input_sequences: candidate.state.next_input_sequences.as_ref().clone(),
                phase: match candidate.state.phase {
                    OnlineMatchPhase::Running => "running".to_string(),
                    OnlineMatchPhase::Complete => "complete".to_string(),
                    _ => "invalid".to_string(),
                },
                receipts_replayable: candidate.receipts_replayable,
                snapshot_hash: candidate.state.snapshot_hash.as_ref().clone(),
            });
            match record {
                Ok(record) => {
                    record_published_tick_with_timeout(
                        &journal,
                        record,
                        candidate.durable_db_next_sequence,
                        candidate.durable_db_match_revision,
                        candidate.durable_db_next_input_sequences,
                    )
                    .await
                }
                Err(error) => Err(error),
            }
        };
        let journal_ms = u64::try_from(journal_started.elapsed().as_millis()).unwrap_or(u64::MAX);
        if journal_ms > 100 {
            tracing::warn!(%match_id, state_sequence, journal_ms, "slow published-tick journal commit");
        }
        if !*permit.borrow() {
            return;
        }
        // A terminal HWM proves what may be recovered after a crash, but it is
        // not yet the semantic publication ACK. Keep both public channels on
        // their prior running tuple until the actor commits the exact database
        // marker and explicitly releases this deferred terminal value.
        let deferred_terminal = result
            .is_ok()
            .then(|| TerminalPublicationEvidence::from_state(&candidate.state))
            .flatten()
            .map(|evidence| DeferredTerminalPublication {
                state: candidate.state.clone(),
                cursor: acknowledged_cursor.clone(),
                evidence,
                publish_to_watch: candidate.publish_to_watch,
            });
        if result.is_ok() && deferred_terminal.is_none() {
            if candidate.publish_to_watch {
                published.send_replace(candidate.state);
            }
            publication_acked.send_replace(acknowledged_cursor);
        }
        let failed = result.is_err();
        if !matches!(
                        ActorCheckpointBoundary::Shutdown,
                            format!("terminal durable publication failed: {error}"),
                            pending.input_sequence,
                        terminal: true,
                    tracing::error!(%match_id, worker = label, "cancelled match actor worker did not stop within hard timeout");
