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
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    if !exact_terminal_publication_marker_exists(&mut *connection, tombstone.high_water.match_id)
        .await?
    {
        return Err(
            "cold terminal tombstone sealed state is not an exact public ACK marker".to_string(),
        );
    }
    Ok(())
}

fn abandonment_tombstone_matches_evidence(
    tombstone: &PublishedTickAbandonmentTombstone,
    evidence: &AbandonmentDatabaseEvidence,
    lineage: &DatabaseLineage,
) -> bool {
    evidence.matches_high_water(&tombstone.high_water)
        && tombstone.failure_reason == evidence.failure_reason
        && tombstone.abandoned_at_unix_ms == evidence.abandoned_at_unix_ms
        && database_lineage_covers_abandonment(lineage, tombstone)
}

async fn ensure_cold_abandonment(
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    evidence: &AbandonmentDatabaseEvidence,
    lineage: &DatabaseLineage,
) -> Result<PublishedTickAbandonmentTombstone, String> {
    if !evidence.matches_high_water(high_water) {
        return Err(
            "failed-closed abandonment evidence does not match its exact hot witness".to_string(),
        );
    }
    if let Some(existing) = journal.abandonment_tombstone(high_water.match_id)? {
        if !abandonment_tombstone_matches_evidence(&existing, evidence, lineage) {
            return Err(
                "durable cold abandonment conflicts with PostgreSQL authority or lineage"
                    .to_string(),
            );
        }
        return Ok(existing);
    }
    if evidence.local_tombstone_state == "sealed" {
        return Err("sealed failed-closed abandonment lost its durable cold tombstone".to_string());
    }
    let hot = journal
        .high_water(high_water.match_id)?
        .ok_or_else(|| "cold abandonment is missing its exact hot witness".to_string())?;
    if hot != *high_water {
        return Err("cold abandonment hot witness changed before sealing".to_string());
    }
    let tombstone = seal_abandonment_with_timeout(
        journal,
        PublishedTickAbandonmentTombstoneInput {
            high_water: hot,
            failure_reason: evidence.failure_reason.clone(),
            abandoned_at_unix_ms: evidence.abandoned_at_unix_ms,
            database_system_identifier: lineage.system_identifier.clone(),
            database_timeline_id: lineage.timeline_id,
            database_wal_lsn: lineage.wal_flush_lsn.clone(),
        },
    )
    .await?;
    if !abandonment_tombstone_matches_evidence(&tombstone, evidence, lineage) {
        return Err("sealed cold abandonment did not preserve exact authority".to_string());
    }
    Ok(tombstone)
}

async fn mark_abandonment_sealed<'e, E>(
    executor: E,
    tombstone: &PublishedTickAbandonmentTombstone,
) -> Result<(), String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let high_water = &tombstone.high_water;
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "abandonment tombstone tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "abandonment tombstone sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "abandonment tombstone revision exceeds PostgreSQL range".to_string())?;
    let abandoned_at_unix_ms = i64::try_from(tombstone.abandoned_at_unix_ms)
        .map_err(|_| "abandonment tombstone timestamp exceeds PostgreSQL range".to_string())?;
    let sealed = sqlx::query::query(
        "update trnm_online_failed_closed_abandonment_markers a
            set local_tombstone_state = 'sealed'
           from trnm_online_matches m
          where a.match_id = $1 and m.match_id = a.match_id
            and a.local_tombstone_state in ('hot_pending', 'sealed')
            and a.journal_owner_id = $2
            and a.actor_generation = $3
            and a.instance_id = $4
            and a.actor_epoch = $5
            and a.physical_host_id = $6
            and a.authoritative_tick = $7
            and a.next_sequence = $8
            and a.match_revision = $9
            and a.next_input_sequences = $10
            and a.snapshot_hash = $11
            and a.failure_reason = $12
            and floor(extract(epoch from a.abandoned_at) * 1000)::bigint = $13
            and m.phase = 'failed_closed'
            and m.settlement_state = 'failed_closed'
            and m.terminal_publication_state = 'pending'
            and m.checkpoint_sequence = m.next_sequence
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.failure_reason = a.failure_reason
            and m.assigned_instance_id = a.instance_id
            and m.assigned_instance_epoch = a.actor_epoch
            and m.assigned_physical_host_id = a.physical_host_id
            and m.authoritative_tick = a.authoritative_tick
            and m.next_sequence = a.next_sequence
            and m.match_revision = a.match_revision
            and m.snapshot_hash = a.snapshot_hash
            and a.next_input_sequences = coalesce(
                (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                 )
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                 where terminal.match_id = m.match_id
            )
        returning a.match_id",
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
    .bind(
        serde_json::to_value(&high_water.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(&high_water.snapshot_hash)
    .bind(&tombstone.failure_reason)
    .bind(abandoned_at_unix_ms)
    .fetch_optional(executor)
    .await
    .map_err(|error| error.to_string())?;
    if sealed.is_none() {
        return Err(
            "cold abandonment could not seal its exact durable failed-closed authority".to_string(),
        );
    }
    Ok(())
}

async fn seal_abandonment_in_database(
    connection: &mut PgConnection,
    tombstone: &PublishedTickAbandonmentTombstone,
    runtime_state: Option<&AppState>,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<(), String> {
    let mut transaction = connection
        .begin()
        .await
        .map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch(&mut transaction, state, true).await?;
    }
    mark_abandonment_sealed(&mut *transaction, tombstone).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    let evidence = load_abandonment_database_evidence_by_match(
        &mut *connection,
        tombstone.high_water.match_id,
    )
    .await?
    .ok_or_else(|| "sealed abandonment marker disappeared from PostgreSQL".to_string())?;
    let lineage = database_lineage(&mut *connection).await?;
    if evidence.local_tombstone_state != "sealed"
        || !abandonment_tombstone_matches_evidence(tombstone, &evidence, &lineage)
    {
        return Err("cold abandonment sealed state is not an exact durable marker".to_string());
    }
    Ok(())
}

async fn reconcile_failed_closed_high_water(
    pool: &PgPool,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    runtime_state: Option<&AppState>,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<bool, String> {
    let mut connection = pool.acquire().await.map_err(|error| error.to_string())?;
    let Some((evidence, lineage, _legacy_adopted)) = ensure_failed_closed_abandonment_marker(
        &mut connection,
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
@@TRNM_TERM1_SLOT_3@@
