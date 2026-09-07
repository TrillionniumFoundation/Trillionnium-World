#[derive(Debug)]
    Ok(TerminalAckDatabaseEvidence {
            .try_get("journal_owner_id")
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
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
                .await?)
               and a.match_revision = $6
        .copied()
