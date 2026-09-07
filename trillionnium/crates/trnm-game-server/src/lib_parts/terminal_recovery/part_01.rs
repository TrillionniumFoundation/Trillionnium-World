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
        || database_summary
            .terminal_total_count
            .checked_add(database_summary.abandonment_total_count)
            != Some(cold_count)
    {
        return Err(format!(
            "local cold witness seal mismatch: PostgreSQL terminal {}/{}, abandonment {}/{}, journal terminal {}, abandonment {}, combined {}",
            database_summary.terminal_sealed_count,
            database_summary.terminal_total_count,
            database_summary.abandonment_sealed_count,
            database_summary.abandonment_total_count,
            terminal_count,
            abandonment_count,
            cold_count,
        ));
    }
    Ok(reconciled)
}

async fn latest_cold_witness_sentinel_is_healthy(state: &AppState) -> Result<bool, String> {
    let Some(witness) = state.published_tick_journal.latest_cold_witness()? else {
        return Ok(true);
    };
    let mut connection = state
        .readiness_pool
        .acquire()
        .await
        .map_err(|error| error.to_string())?;
    let lineage = database_lineage(&mut *connection).await?;
    if !database_lineage_covers_cold_witness(&lineage, &witness) {
        return Ok(false);
    }
    match witness {
        PublishedTickColdWitness::TerminalAck(tombstone) => {
            let Some(evidence) = load_terminal_ack_database_evidence_by_match(
                &mut *connection,
                tombstone.high_water.match_id,
            )
            .await?
            else {
                return Ok(false);
            };
            Ok(evidence.local_tombstone_state == "sealed"
                && evidence.matches_tombstone(&tombstone, &lineage)
                && exact_terminal_publication_marker_exists(
                    &mut *connection,
                    tombstone.high_water.match_id,
                )
                .await?)
        }
        PublishedTickColdWitness::FailedClosedAbandonment(tombstone) => {
            let Some(evidence) = load_abandonment_database_evidence_by_match(
                &mut *connection,
                tombstone.high_water.match_id,
            )
            .await?
            else {
                return Ok(false);
            };
            Ok(evidence.local_tombstone_state == "sealed"
                && abandonment_tombstone_matches_evidence(&tombstone, &evidence, &lineage))
        }
    }
}

async fn campaign_has_unacknowledged_progression<'e, E>(
    executor: E,
    campaign_id: &str,
) -> Result<bool, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query_scalar::query_scalar(CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL)
        .bind(campaign_id)
        .fetch_one(executor)
        .await
        .map_err(|error| error.to_string())
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct HistoricalTerminalProjectionQuarantine {
    legacy_terminal_match_count: i64,
    campaign_projection_polluted: bool,
    rating_projection_polluted: bool,
}

impl HistoricalTerminalProjectionQuarantine {
    fn public_credit_is_clean(self) -> bool {
        self.legacy_terminal_match_count == 0
            && !self.campaign_projection_polluted
            && !self.rating_projection_polluted
    }
}

async fn terminal_publication_marker_matches_high_water<'e, E>(
    executor: E,
    high_water: &PublishedTickHighWater,
    durable_result_hash: &str,
    durable_settlement_state: &str,
) -> Result<bool, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query_scalar::query_scalar(
        "select exists(
            select 1
              from trnm_online_terminal_publication_acks a
             where a.match_id = $1
               and a.actor_generation = $2
               and a.actor_epoch = $3
               and a.authoritative_tick = $4
               and a.next_sequence = $5
               and a.match_revision = $6
               and a.next_input_sequences = $7
               and a.snapshot_hash = $8
               and a.phase = 'complete'
               and a.result_hash = $9
               and a.instance_id = $11
               and a.physical_host_id = $12
               and a.published_settlement_state = $10
        )",
    )
    .bind(high_water.match_id)
    .bind(high_water.actor_generation)
    .bind(high_water.actor_epoch)
    .bind(high_water.tick as i64)
    .bind(high_water.next_sequence as i64)
    .bind(high_water.match_revision as i64)
    .bind(
        serde_json::to_value(&high_water.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(&high_water.snapshot_hash)
    .bind(durable_result_hash)
    .bind(durable_settlement_state)
    .bind(&high_water.instance_id)
    .bind(&high_water.physical_host_id)
    .fetch_one(executor)
    .await
    .map_err(|error| error.to_string())
}

fn terminal_read_surface_is_releasable(phase: OnlineMatchPhase, exact_marker_exists: bool) -> bool {
    phase != OnlineMatchPhase::Complete || exact_marker_exists
}

fn terminal_ack_gap_recovery_is_operational(
    query_healthy: bool,
    gap_count: usize,
    bounded_transition_owned: bool,
) -> bool {
    query_healthy && (gap_count == 0 || bounded_transition_owned)
}

fn terminal_runtime_revalidation_is_allowed(
    high_water_instance_id: &str,
    high_water_epoch: i64,
    high_water_physical_host_id: &str,
    runtime_instance_id: &str,
    runtime_epoch: i64,
    runtime_physical_host_id: &str,
    requires_mutation: bool,
) -> bool {
    !requires_mutation
        || (high_water_instance_id == runtime_instance_id
            && high_water_epoch == runtime_epoch
            && high_water_physical_host_id == runtime_physical_host_id)
}

fn terminal_ack_gaps_without_high_water(
    terminal_ack_gaps: &[Uuid],
    recorded_match_ids: &BTreeSet<Uuid>,
) -> Vec<Uuid> {
    terminal_ack_gaps
        .iter()
        .filter(|match_id| !recorded_match_ids.contains(match_id))
        .copied()
        .collect()
}
