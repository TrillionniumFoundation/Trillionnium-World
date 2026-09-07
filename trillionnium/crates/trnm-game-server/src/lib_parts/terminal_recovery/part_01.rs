@@TRNM_TERM1_SLOT_0@@
            .try_get("fleet_epoch_current")
            .map_err(|error| error.to_string())?,
        healthy_fleet_instances: row
            .try_get("healthy_fleet_instances")
            .map_err(|error| error.to_string())?,
        active_matches: row
@@TRNM_TERM1_SLOT_1@@
        lock_current_fleet_epoch(&mut transaction, state, true).await?;
    }
    mark_terminal_ack_sealed(&mut *transaction, tombstone).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    }
@@TRNM_TERM1_SLOT_2@@
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
        }
        if page_len < usize::try_from(TERMINAL_ACK_STARTUP_PAGE_SIZE).unwrap_or(usize::MAX) {
            break;
        }
    }

    let mut after_match_id = None;
    loop {
        let page =
            load_local_abandonment_startup_page(pool, physical_host_id, after_match_id).await?;
        if page.is_empty() {
            break;
        }
        let page_len = page.len();
        for evidence in page {
            after_match_id = Some(evidence.match_id);
            let cold = journal.abandonment_tombstone(evidence.match_id)?;
            let hot = journal.high_water(evidence.match_id)?;
            let tombstone = if let Some(tombstone) = cold {
                if !abandonment_tombstone_matches_evidence(&tombstone, &evidence, &startup_lineage)
                {
                    return Err(format!(
                        "cold abandonment {} conflicts with its exact PostgreSQL tuple",
                        evidence.match_id
                    ));
                }
                tombstone
            } else if let Some(high_water) = hot {
                if !evidence.matches_high_water(&high_water) {
                    return Err(format!(
                        "hot abandonment {} conflicts with its exact PostgreSQL tuple",
                        evidence.match_id
                    ));
                }
                ensure_cold_abandonment(journal, &high_water, &evidence, &startup_lineage).await?
            } else {
                return Err(format!(
                    "failed-closed abandonment {} has neither its exact hot nor cold witness",
                    evidence.match_id
                ));
            };
            seal_abandonment_in_database(
                &mut connection,
                &tombstone,
                None,
                database_host_authority,
            )
            .await?;
            reconciled.failed_closed.insert(evidence.match_id);
        }
        if page_len < usize::try_from(TERMINAL_ACK_STARTUP_PAGE_SIZE).unwrap_or(usize::MAX) {
            break;
        }
    }

    let current_lineage = database_lineage(&mut *connection).await?;
    reconcile_latest_cold_witness(
        &mut connection,
        journal,
        &current_lineage,
        physical_host_id,
        database_host_authority,
        &mut reconciled,
    )
    .await?;
    let database_summary =
        load_cold_witness_database_summary(&mut *connection, physical_host_id).await?;
    let terminal_count = u64::try_from(journal.ack_tombstone_count()?)
        .map_err(|_| "terminal cold witness count exceeds u64".to_string())?;
    let abandonment_count = u64::try_from(journal.abandonment_tombstone_count()?)
        .map_err(|_| "abandonment cold witness count exceeds u64".to_string())?;
    let cold_count = u64::try_from(journal.cold_witness_count()?)
        .map_err(|_| "combined cold witness count exceeds u64".to_string())?;
    if !database_summary.all_sealed()
        || database_summary.terminal_total_count != terminal_count
        || database_summary.abandonment_total_count != abandonment_count
@@TRNM_TERM1_SLOT_3B@@
