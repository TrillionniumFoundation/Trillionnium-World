async fn transition_running_to_failed_closed(
    connection: &mut PgConnection,
    high_water: &PublishedTickHighWater,
    failure_reason: &str,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<(AbandonmentDatabaseEvidence, DatabaseLineage), String> {
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "maintenance high-water tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "maintenance high-water sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "maintenance high-water revision exceeds PostgreSQL range".to_string())?;
    let next_input_sequences = serde_json::to_value(&high_water.next_input_sequences)
        .map_err(|error| error.to_string())?;
    let mut transaction = connection
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    lock_maintenance_fleet_assignment(
        &mut transaction,
        &high_water.instance_id,
        &high_water.physical_host_id,
        Some(high_water.actor_epoch),
    )
    .await?;
    let locked_match = sqlx::query::query(
        "select simulation_json from trnm_online_matches where match_id = $1 for update",
    )
    .bind(high_water.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let locked_match = locked_match
        .ok_or_else(|| "running maintenance exact match UUID disappeared".to_string())?;
    let locked_simulation = serde_json::from_value::<MissionSimV1>(
        locked_match
            .try_get::<Option<Value>, _>("simulation_json")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "running maintenance durable simulation is missing".to_string())?,
    )
    .map_err(|error| format!("decode running maintenance simulation: {error}"))?;
    validate_recovery_simulation(
        &locked_simulation,
        Some(high_water.tick),
        &high_water.snapshot_hash,
        "running maintenance transition checkpoint",
    )?;
    if locked_simulation.terminal() {
        return Err("running maintenance checkpoint is terminal".to_string());
    }
    let member_rows = sqlx::query::query(
        "select player_id, next_input_sequence
           from trnm_online_match_members
          where match_id = $1
          order by player_id
          for update",
    )
    .bind(high_water.match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut locked_member_cursors = BTreeMap::new();
    for member in member_rows {
        locked_member_cursors.insert(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            u64::try_from(
                member
                    .try_get::<i64, _>("next_input_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "maintenance member cursor is negative".to_string())?,
        );
    }
    if locked_member_cursors != high_water.next_input_sequences {
        return Err(
            "running maintenance member cursors do not match the exact hot witness".to_string(),
        );
    }
    let transitioned = sqlx::query::query(
        "update trnm_online_matches m
            set phase = 'failed_closed', settlement_state = 'failed_closed',
                failure_reason = $2, updated_at = clock_timestamp()
          where m.match_id = $1
            and m.phase = 'running'
            and m.settlement_state = 'not_ready'
            and m.terminal_publication_state = 'pending'
            and m.assigned_instance_id = $3
            and m.assigned_instance_epoch = $4
            and m.assigned_physical_host_id = $5
            and m.authoritative_tick = $6
            and m.next_sequence = $7
            and m.checkpoint_sequence = $7
            and m.match_revision = $8
            and m.snapshot_hash = $10
            and m.simulation_json is not null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and $9 = coalesce(
                (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                 ) from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                 where terminal.match_id = m.match_id
            )
            and not exists (
                select 1 from trnm_online_failed_closed_abandonment_markers marker
                 where marker.match_id = m.match_id
            )
        returning m.match_id",
    )
    .bind(high_water.match_id)
    .bind(failure_reason)
    .bind(&high_water.instance_id)
    .bind(high_water.actor_epoch)
    .bind(&high_water.physical_host_id)
    .bind(tick)
    .bind(next_sequence)
    .bind(match_revision)
    .bind(next_input_sequences.clone())
    .bind(&high_water.snapshot_hash)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if transitioned.is_none() {
        return Err(
            "running maintenance selector did not match one exact durable authority tuple"
                .to_string(),
        );
    }
    sqlx::query::query(
        "insert into trnm_online_failed_closed_abandonment_markers (
            match_id, journal_owner_id, actor_generation, instance_id,
            actor_epoch, physical_host_id, authoritative_tick, next_sequence,
            match_revision, next_input_sequences, snapshot_hash, failure_reason
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    )
    .bind(high_water.match_id)
    .bind(high_water.journal_owner_id)
    .bind(high_water.actor_generation)
    .bind(&high_water.instance_id)
    .bind(high_water.actor_epoch)
    .bind(&high_water.physical_host_id)
    .bind(tick)
    .bind(next_sequence)
    .bind(match_revision)
    .bind(next_input_sequences)
    .bind(&high_water.snapshot_hash)
    .bind(failure_reason)
    .execute(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let evidence =
        load_abandonment_database_evidence_by_match(&mut *transaction, high_water.match_id)
            .await?
            .ok_or_else(|| "atomic maintenance marker disappeared before commit".to_string())?;
    if !evidence.matches_high_water(high_water) || evidence.failure_reason != failure_reason {
        return Err("atomic maintenance marker is not exact before commit".to_string());
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    let lineage = database_lineage(&mut *connection).await?;
    Ok((evidence, lineage))
}

async fn transition_waiting_to_failed_closed(
    connection: &mut PgConnection,
    config: &FailCloseMaintenanceConfig,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<(), String> {
    let mut transaction = connection
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    lock_maintenance_fleet_assignment(
        &mut transaction,
        config.instance_id.trim(),
        config.physical_host_id.trim(),
        None,
    )
    .await?;
    let locked = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_id from trnm_online_matches where match_id = $1 for update",
    )
    .bind(config.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if locked.is_none() {
        return Err("waiting maintenance exact match UUID disappeared".to_string());
    }
    let _transitioned = sqlx::query::query(
        "update trnm_online_matches m
            set phase = 'failed_closed', settlement_state = 'failed_closed',
                failure_reason = $2, updated_at = clock_timestamp()
          where m.match_id = $1
            and m.phase = 'waiting'
            and m.settlement_state = 'not_ready'
            and m.terminal_publication_state = 'pending'
            and m.assigned_instance_id is null
            and m.assigned_instance_epoch = 0
            and m.assigned_physical_host_id is null
            and m.simulation_json is null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                 where terminal.match_id = m.match_id
            )
            and not exists (
                select 1 from trnm_online_failed_closed_abandonment_markers marker
                 where marker.match_id = m.match_id
            )
        returning m.match_id",
    )
    .bind(config.match_id)
    .bind(&config.failure_reason)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let exact_db_only = sqlx::query_scalar::query_scalar::<_, bool>(
        "select exists(
            select 1 from trnm_online_matches m
             where m.match_id = $1
               and m.phase = 'failed_closed'
               and m.settlement_state = 'failed_closed'
               and m.failure_reason = $2
               and m.terminal_publication_state = 'pending'
               and m.assigned_instance_id is null
               and m.assigned_instance_epoch = 0
               and m.assigned_physical_host_id is null
               and m.simulation_json is null
               and m.snapshot_hash = ''
               and m.authoritative_tick = 0
               and m.next_sequence = 0
               and m.checkpoint_sequence = 0
               and m.match_revision = 0
               and m.result_json is null
               and m.result_hash is null
               and m.terminal_publication_actor_generation is null
               and m.terminal_stage_simulation_json is null
               and m.terminal_stage_result_json is null
               and m.terminal_stage_result_hash is null
               and m.terminal_stage_snapshot_hash is null
               and m.terminal_stage_authoritative_tick is null
               and m.terminal_stage_next_sequence is null
               and m.terminal_stage_match_revision is null
               and m.terminal_staged_at is null
               and not exists (
                   select 1 from trnm_online_match_members member
                    where member.match_id = m.match_id
                      and member.next_input_sequence <> 0
               )
               and not exists (
                   select 1 from trnm_online_commands command
                    where command.match_id = m.match_id
               )
               and not exists (
                   select 1 from trnm_online_terminal_publication_acks terminal
                    where terminal.match_id = m.match_id
               )
               and not exists (
                   select 1 from trnm_online_failed_closed_abandonment_markers marker
                    where marker.match_id = m.match_id
               )
        )",
    )
    .bind(config.match_id)
    .bind(&config.failure_reason)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    if !exact_db_only {
        return Err("waiting maintenance selector is not an exact DB-only match".to_string());
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())
}

pub async fn run_fail_close_maintenance(
    config: FailCloseMaintenanceConfig,
) -> Result<FailCloseMaintenanceReport, String> {
    if config.match_id.is_nil()
        || config.instance_id.trim().is_empty()
        || config.instance_id.trim() != config.instance_id
        || config.physical_host_id.trim().is_empty()
        || config.physical_host_id.trim() != config.physical_host_id
        || !valid_abandonment_failure_reason(&config.failure_reason)
    {
        return Err(
            "maintenance identity, exact match UUID and failure reason are invalid".to_string(),
        );
    }
    let journal = PublishedTickJournal::open(
        config.published_tick_journal_dir.clone(),
        config.physical_host_id.clone(),
    )?;
    if journal.ack_tombstone(config.match_id)?.is_some() {
        return Err("maintenance target already has terminal ACK cold evidence".to_string());
    }
    let hot = journal.high_water(config.match_id)?;
    let cold = journal.abandonment_tombstone(config.match_id)?;
    let hot_witness_present_before = hot.is_some();
    let database_host_authority =
        bootstrap_database_host_authority(&config.database_url, config.physical_host_id.trim())
            .await?;
    let mut connection = database_host_authority.connection.lock().await;
    let previous_phase = sqlx::query_scalar::query_scalar::<_, String>(
        "select phase from trnm_online_matches where match_id = $1",
    )
    .bind(config.match_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "maintenance exact match UUID does not exist".to_string())?;
    if config.adopt_legacy_pre_v13 && previous_phase != "failed_closed" {
        return Err(
            "--adopt-legacy-pre-v13 is valid only for an already failed_closed row".to_string(),
        );
    }
    let mut legacy_adoption = false;
    let (waiting_db_only, cold_witness_sealed, local_marker_state) = match previous_phase.as_str() {
        "waiting" => {
            if hot.is_some() || cold.is_some() {
                return Err(
                    "waiting maintenance target must not have hot or cold journal evidence"
                        .to_string(),
                );
            }
            transition_waiting_to_failed_closed(&mut connection, &config, &database_host_authority)
                .await?;
            (true, false, None)
        }
        "running" => {
            if cold.is_some() {
                return Err(
                    "running maintenance target already has abandonment cold evidence".to_string(),
                );
            }
            let retained_high_water = hot.as_ref().ok_or_else(|| {
                "running maintenance requires its retained exact hot witness".to_string()
            })?;
            if retained_high_water.instance_id != config.instance_id
                || retained_high_water.physical_host_id != config.physical_host_id
                || retained_high_water.phase != "running"
                || !retained_high_water.receipts_replayable
            {
                return Err(
                    "running maintenance hot witness has different ownership or phase".to_string(),
                );
            }
            let high_water = prepare_final_abandonment_high_water(
                &mut connection,
                &journal,
                retained_high_water,
                &database_host_authority,
            )
            .await?;
            let (evidence, lineage) = transition_running_to_failed_closed(
                &mut connection,
                &high_water,
                &config.failure_reason,
                &database_host_authority,
            )
            .await?;
            let tombstone =
                ensure_cold_abandonment(&journal, &high_water, &evidence, &lineage).await?;
            seal_abandonment_in_database(
                &mut connection,
                &tombstone,
                None,
                &database_host_authority,
            )
            .await?;
            (false, true, Some("sealed".to_string()))
        }
        "failed_closed" => {
            if hot.is_none() && cold.is_none() {
                if config.adopt_legacy_pre_v13 {
                    return Err(
                        "legacy abandonment adoption requires an exact retained hot witness"
                            .to_string(),
                    );
                }
                transition_waiting_to_failed_closed(
                    &mut connection,
                    &config,
                    &database_host_authority,
                )
                .await?;
                (true, false, None)
            } else {
                let (evidence, lineage, adopted) = if let Some(high_water) = hot.as_ref() {
                    let Some(resolved) = ensure_failed_closed_abandonment_marker(
                        &mut connection,
                        high_water,
                        &database_host_authority,
                        config.adopt_legacy_pre_v13,
                        Some(FailedClosedMaintenanceExpectation {
                            instance_id: &config.instance_id,
                            physical_host_id: &config.physical_host_id,
                            failure_reason: &config.failure_reason,
                        }),
                    )
                    .await?
                    else {
                        return Err(
                            "failed-closed maintenance hot witness is not running".to_string()
                        );
                    };
                    resolved
                } else if let Some(tombstone) = cold.as_ref() {
                    let legacy_origin =
                        failed_closed_match_originates_before_v13(&mut connection, config.match_id)
                            .await?;
                    if config.adopt_legacy_pre_v13 && !legacy_origin {
                        return Err(
                            "post-V13 failed-closed authority cannot use legacy adoption"
                                .to_string(),
                        );
                    }
                    let evidence = load_abandonment_database_evidence_by_match(
                        &mut *connection,
                        config.match_id,
                    )
                    .await?
                    .ok_or_else(|| "cold abandonment is absent from PostgreSQL".to_string())?;
                    let lineage = database_lineage(&mut *connection).await?;
                    if !abandonment_tombstone_matches_evidence(tombstone, &evidence, &lineage) {
                        return Err("cold abandonment conflicts with PostgreSQL".to_string());
                    }
                    (evidence, lineage, legacy_origin)
                } else {
                    unreachable!("hot or cold presence checked above")
                };
                if evidence.failure_reason != config.failure_reason {
                    return Err(
                        "maintenance failure reason conflicts with existing authority".to_string(),
                    );
                }
                legacy_adoption = adopted;
                if cold.as_ref().is_some_and(|tombstone| {
                    !abandonment_tombstone_matches_evidence(tombstone, &evidence, &lineage)
                }) {
                    return Err(
                        "retained cold abandonment conflicts with exact authority".to_string()
                    );
                }
                let tombstone = if let Some(tombstone) = cold {
                    tombstone
                } else {
                    ensure_cold_abandonment(
                        &journal,
                        hot.as_ref().expect("hot branch checked above"),
                        &evidence,
                        &lineage,
                    )
                    .await?
                };
                if evidence.local_tombstone_state == "hot_pending" {
                    seal_abandonment_in_database(
                        &mut connection,
                        &tombstone,
                        None,
                        &database_host_authority,
                    )
                    .await?;
                }
                (false, true, Some("sealed".to_string()))
            }
        }
        "complete" => {
            return Err("maintenance may not fail-close a complete match".to_string());
        }
        phase => return Err(format!("maintenance found unsupported match phase {phase}")),
    };
    let final_phase = sqlx::query_scalar::query_scalar::<_, String>(
        "select phase from trnm_online_matches where match_id = $1",
    )
    .bind(config.match_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| error.to_string())?;
    if final_phase != "failed_closed" || journal.high_water(config.match_id)?.is_some() {
        return Err(
            "maintenance postcondition did not reach failed_closed with no hot witness".to_string(),
        );
    }
    Ok(FailCloseMaintenanceReport {
        contract_version: "trnm_online_maintenance_fail_close_v1",
        status: "completed",
        match_id: config.match_id,
        selector: "exact_match_id",
        transition_atomic: !legacy_adoption,
        previous_phase,
        final_phase: "failed_closed",
        waiting_db_only,
        hot_witness_present_before,
        cold_witness_sealed,
        local_marker_state,
        legacy_adoption,
        adoption_contract: legacy_adoption.then_some("legacy_pre_v13_adoption_v1"),
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerminalJournalReconciliationOutcome {
    NotTerminal,
    FailedClosed,
    Recovered,
}

trait TerminalOrphanAuthority {
    async fn recover_running_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<Option<PublishedTickHighWater>, String>;

    async fn acknowledge_terminal_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<bool, String>;

    async fn reconcile_failed_closed_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<bool, String>;
}

struct PostgresTerminalOrphanAuthority<'a> {
    pool: &'a PgPool,
    journal: &'a PublishedTickJournal,
    runtime_state: Option<&'a AppState>,
    database_host_authority: &'a DatabaseHostAuthorityFence,
}

impl TerminalOrphanAuthority for PostgresTerminalOrphanAuthority<'_> {
    async fn recover_running_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<Option<PublishedTickHighWater>, String> {
        tokio::time::timeout(
            MATCH_ACTOR_INITIALIZATION_TIMEOUT,
            recover_terminal_high_water_after_running_crash(
                self.pool,
                self.journal,
                high_water,
                self.database_host_authority,
            ),
        )
        .await
        .map_err(|_| "running high-water terminal recovery exceeded its hard timeout".to_string())?
    }

    async fn acknowledge_terminal_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<bool, String> {
        tokio::time::timeout(
            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
            terminal_high_water_is_durably_acknowledged(
                self.pool,
                self.journal,
                high_water,
                self.runtime_state,
                self.database_host_authority,
            ),
        )
        .await
        .map_err(|_| "terminal high-water validation exceeded its hard timeout".to_string())?
    }

    async fn reconcile_failed_closed_high_water(
        &self,
        high_water: &PublishedTickHighWater,
    ) -> Result<bool, String> {
        tokio::time::timeout(
            MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
            reconcile_failed_closed_high_water(
                self.pool,
                self.journal,
                high_water,
                self.runtime_state,
                self.database_host_authority,
            ),
        )
        .await
        .map_err(|_| "failed-closed high-water validation exceeded its hard timeout".to_string())?
    }
}

/// Closes all three durable terminal crash windows with the same transition
/// during startup and while the process remains alive:
///
/// 1. DB complete + running HWM -> exact terminal HWM;
/// 2. terminal HWM + missing marker -> exact durable marker;
/// 3. exact marker + retained HWM -> exact durable revalidation.
///
/// A sealed terminal hot record is atomically replaced by a cold tombstone.
/// That immutable local witness detects a PostgreSQL PITR rollback to a
/// pre-terminal state and is not garbage-collected by live reconciliation.
async fn reconcile_terminal_journal_record<A: TerminalOrphanAuthority>(
    authority: &A,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
) -> Result<TerminalJournalReconciliationOutcome, String> {
    let terminal_high_water = if high_water.phase == "running" {
        let Some(recovered) = authority.recover_running_high_water(high_water).await? else {
            return if authority
                .reconcile_failed_closed_high_water(high_water)
                .await?
            {
                Ok(TerminalJournalReconciliationOutcome::FailedClosed)
            } else {
                Ok(TerminalJournalReconciliationOutcome::NotTerminal)
            };
        };
        record_published_tick_with_timeout(
            journal,
            recovered.clone(),
            recovered.next_sequence,
            recovered.match_revision,
            recovered.next_input_sequences.clone(),
        )
        .await?;
        recovered
    } else if high_water.phase == "complete" {
        high_water.clone()
    } else {
        return Err("published-tick journal contains an unsupported phase".to_string());
    };

    if !authority
        .acknowledge_terminal_high_water(&terminal_high_water)
        .await?
    {
        return Err(
            "terminal high-water did not create or match its exact durable marker".to_string(),
        );
    }
    Ok(TerminalJournalReconciliationOutcome::Recovered)
}

