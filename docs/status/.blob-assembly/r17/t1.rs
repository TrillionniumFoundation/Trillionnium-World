#[derive(Debug)]
    Ok(TerminalAckDatabaseEvidence {
            .try_get("journal_owner_id")
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
        database_host_authority,
        false,
        None,
    )
    .await?
    else {
        return Ok(false);
    };
    let tombstone = ensure_cold_abandonment(journal, high_water, &evidence, &lineage).await?;
    seal_abandonment_in_database(
        &mut connection,
        &tombstone,
        runtime_state,
        database_host_authority,
    )
    .await?;
    Ok(true)
}

#[derive(Default)]
struct StartupColdWitnesses {
    terminal_acknowledged: BTreeSet<Uuid>,
    failed_closed: BTreeSet<Uuid>,
}

impl StartupColdWitnesses {
    fn extend(&mut self, other: Self) {
        self.terminal_acknowledged
            .extend(other.terminal_acknowledged);
        self.failed_closed.extend(other.failed_closed);
    }
}

async fn reconcile_latest_cold_witness(
    connection: &mut PgConnection,
    journal: &PublishedTickJournal,
    lineage: &DatabaseLineage,
    physical_host_id: &str,
    database_host_authority: &DatabaseHostAuthorityFence,
    reconciled: &mut StartupColdWitnesses,
) -> Result<(), String> {
    let Some(witness) = journal.latest_cold_witness()? else {
        return Ok(());
    };
    let high_water = match &witness {
        PublishedTickColdWitness::TerminalAck(tombstone) => &tombstone.high_water,
        PublishedTickColdWitness::FailedClosedAbandonment(tombstone) => &tombstone.high_water,
    };
    if high_water.physical_host_id != physical_host_id {
        return Err("latest cold witness belongs to a different physical host".to_string());
    }
    if !database_lineage_covers_cold_witness(lineage, &witness) {
        return Err(format!(
            "latest cold witness {} is ahead of or belongs to a different PostgreSQL lineage",
            high_water.match_id
        ));
    }

    match witness {
        PublishedTickColdWitness::TerminalAck(tombstone) => {
            let evidence = load_terminal_ack_database_evidence_by_match(
                &mut *connection,
                tombstone.high_water.match_id,
            )
            .await?
            .ok_or_else(|| {
                format!(
                    "latest cold terminal ACK {} is absent from PostgreSQL",
                    tombstone.high_water.match_id
                )
            })?;
            if !evidence.matches_tombstone(&tombstone, lineage) {
                return Err(format!(
                    "latest cold terminal ACK {} conflicts with PostgreSQL",
                    tombstone.high_water.match_id
                ));
            }
            match evidence.local_tombstone_state.as_str() {
                "legacy_bootstrap_pending" | "hot_pending" => {
                    seal_terminal_ack_in_database(
                        connection,
                        &tombstone,
                        None,
                        database_host_authority,
                    )
                    .await?;
                }
                "sealed" => {
                    if !exact_terminal_publication_marker_exists(
                        &mut *connection,
                        tombstone.high_water.match_id,
                    )
                    .await?
                    {
                        return Err("latest cold terminal ACK is not publicly exact".to_string());
                    }
                }
                _ => {
                    return Err(
                        "latest cold terminal ACK has an unsupported DB seal state".to_string()
                    );
                }
            }
            reconciled
                .terminal_acknowledged
                .insert(tombstone.high_water.match_id);
        }
        PublishedTickColdWitness::FailedClosedAbandonment(tombstone) => {
            let evidence = load_abandonment_database_evidence_by_match(
                &mut *connection,
                tombstone.high_water.match_id,
            )
            .await?
            .ok_or_else(|| {
                format!(
                    "latest cold abandonment {} is absent from PostgreSQL",
                    tombstone.high_water.match_id
                )
            })?;
            if !abandonment_tombstone_matches_evidence(&tombstone, &evidence, lineage) {
                return Err(format!(
                    "latest cold abandonment {} conflicts with PostgreSQL",
                    tombstone.high_water.match_id
                ));
            }
            match evidence.local_tombstone_state.as_str() {
                "hot_pending" => {
                    seal_abandonment_in_database(
                        connection,
                        &tombstone,
                        None,
                        database_host_authority,
                    )
                    .await?;
                }
                "sealed" => {}
                _ => {
                    return Err(
                        "latest cold abandonment has an unsupported DB seal state".to_string()
                    );
                }
            }
            reconciled
                .failed_closed
                .insert(tombstone.high_water.match_id);
        }
    }
    Ok(())
}

async fn reconcile_startup_cold_witnesses(
    pool: &PgPool,
    journal: &PublishedTickJournal,
    physical_host_id: &str,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<StartupColdWitnesses, String> {
    let mut connection = pool.acquire().await.map_err(|error| error.to_string())?;
    let startup_lineage = database_lineage(&mut *connection).await?;
    let mut reconciled = StartupColdWitnesses::default();
    reconcile_latest_cold_witness(
        &mut connection,
        journal,
        &startup_lineage,
        physical_host_id,
        database_host_authority,
        &mut reconciled,
    )
    .await?;

    let mut after_match_id = None;
    loop {
        let page =
            load_local_terminal_ack_startup_page(pool, physical_host_id, after_match_id).await?;
        if page.is_empty() {
            break;
        }
        let page_len = page.len();
        for evidence in page {
            after_match_id = Some(evidence.match_id);
            let cold = journal.ack_tombstone(evidence.match_id)?;
            let hot = journal.high_water(evidence.match_id)?;
            let source = startup_terminal_ack_seal_source(
                &evidence.local_tombstone_state,
                hot.is_some(),
                cold.is_some(),
            )
            .ok_or_else(|| {
                format!(
                    "terminal ACK {} in {} has neither an allowed hot nor cold witness",
                    evidence.match_id, evidence.local_tombstone_state
                )
            })?;
            let tombstone = match source {
                StartupTerminalAckSealSource::Cold => {
                    let tombstone = cold.expect("cold source requires a checked tombstone");
                    if !evidence.matches_tombstone(&tombstone, &startup_lineage) {
                        return Err(format!(
                            "cold terminal ACK {} conflicts with its exact PostgreSQL tuple",
                            evidence.match_id
                        ));
                    }
                    tombstone
                }
                StartupTerminalAckSealSource::Hot => {
                    let high_water = hot.expect("hot source requires a checked high-water");
                    if !evidence.matches_high_water(&high_water) {
                        return Err(format!(
                            "hot terminal ACK {} conflicts with its exact PostgreSQL tuple",
                            evidence.match_id
                        ));
                    }
                    ensure_cold_terminal_ack(
                        journal,
                        &high_water,
                        &TerminalPublicationCommit {
                            result_hash: evidence.result_hash.clone(),
                            settlement_state: evidence.settlement_state.clone(),
                            acknowledged_at_unix_ms: evidence.acknowledged_at_unix_ms,
                        },
                        &startup_lineage,
                    )
                    .await?
                }
                StartupTerminalAckSealSource::LegacyDatabaseBootstrap => {
                    let high_water = journal.new_record(evidence.record_input())?;
                    record_published_tick_with_timeout(
                        journal,
                        high_water.clone(),
                        high_water.next_sequence,
                        high_water.match_revision,
                        high_water.next_input_sequences.clone(),
                    )
                    .await?;
                    ensure_cold_terminal_ack(
                        journal,
                        &high_water,
                        &TerminalPublicationCommit {
                            result_hash: evidence.result_hash.clone(),
                            settlement_state: evidence.settlement_state.clone(),
                            acknowledged_at_unix_ms: evidence.acknowledged_at_unix_ms,
                        },
                        &startup_lineage,
                    )
                    .await?
                }
            };
            seal_terminal_ack_in_database(
                &mut connection,
                &tombstone,
                None,
                database_host_authority,
            )
            .await?;
            reconciled.terminal_acknowledged.insert(evidence.match_id);
        .copied()
