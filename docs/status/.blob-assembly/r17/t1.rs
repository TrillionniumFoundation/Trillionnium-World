#[derive(Debug)]
    Ok(TerminalAckDatabaseEvidence {
            .try_get("journal_owner_id")
            .map_err(|error| error.to_string())?,
        actor_generation: row
            .try_get("actor_generation")
            .map_err(|error| error.to_string())?,
        instance_id: row
            .try_get("instance_id")
            .map_err(|error| error.to_string())?,
        actor_epoch: row
            .try_get("actor_epoch")
            .map_err(|error| error.to_string())?,
        physical_host_id: row
            .try_get("physical_host_id")
            .map_err(|error| error.to_string())?,
        authoritative_tick,
        next_sequence,
        match_revision,
        next_input_sequences: serde_json::from_value(
            row.try_get::<Value, _>("next_input_sequences")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("decode abandonment input cursors: {error}"))?,
        snapshot_hash,
        failure_reason,
        local_tombstone_state,
        abandoned_at_unix_ms,
    })
}

async fn load_abandonment_database_evidence_by_match<'e, E>(
    executor: E,
    match_id: Uuid,
) -> Result<Option<AbandonmentDatabaseEvidence>, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query::query(ABANDONMENT_DATABASE_EVIDENCE_BY_MATCH_SQL)
        .bind(match_id)
        .fetch_optional(executor)
        .await
        .map_err(|error| error.to_string())?
        .as_ref()
        .map(abandonment_database_evidence_from_row)
        .transpose()
}

async fn load_local_abandonment_startup_page(
    pool: &PgPool,
    physical_host_id: &str,
    after_match_id: Option<Uuid>,
) -> Result<Vec<AbandonmentDatabaseEvidence>, String> {
    sqlx::query::query(LOCAL_ABANDONMENT_STARTUP_PAGE_SQL)
        .bind(physical_host_id)
        .bind(after_match_id)
        .bind(TERMINAL_ACK_STARTUP_PAGE_SIZE)
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?
        .iter()
        .map(abandonment_database_evidence_from_row)
        .collect()
}

async fn load_terminal_ack_database_evidence_by_match<'e, E>(
    executor: E,
    match_id: Uuid,
) -> Result<Option<TerminalAckDatabaseEvidence>, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let Some(row) = sqlx::query::query(TERMINAL_ACK_DATABASE_EVIDENCE_BY_MATCH_SQL)
        .bind(match_id)
        .fetch_optional(executor)
        .await
        .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let tuple_exact = row
        .try_get::<bool, _>("tuple_exact")
        .map_err(|error| error.to_string())?;
    let evidence = terminal_ack_database_evidence_from_row(&row)?;
    if !tuple_exact {
        return Err(format!(
            "terminal ACK {} does not exactly match its durable match authority",
            evidence.match_id
        ));
    }
    Ok(Some(evidence))
}

async fn load_local_terminal_ack_startup_page(
    pool: &PgPool,
    physical_host_id: &str,
    after_match_id: Option<Uuid>,
) -> Result<Vec<TerminalAckDatabaseEvidence>, String> {
    let rows = sqlx::query::query(LOCAL_TERMINAL_ACK_STARTUP_PAGE_SQL)
        .bind(physical_host_id)
        .bind(after_match_id)
        .bind(TERMINAL_ACK_STARTUP_PAGE_SIZE)
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?;
    let mut evidence = Vec::with_capacity(rows.len());
    for row in rows {
        let tuple_exact = row
            .try_get::<bool, _>("tuple_exact")
            .map_err(|error| error.to_string())?;
        let item = terminal_ack_database_evidence_from_row(&row)?;
        if !tuple_exact {
            return Err(format!(
                "terminal ACK {} does not exactly match its durable match authority",
                item.match_id
            ));
        }
        evidence.push(item);
    }
    Ok(evidence)
}

async fn mark_terminal_ack_sealed<'e, E>(
    executor: E,
    tombstone: &PublishedTickAckTombstone,
) -> Result<(), String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let high_water = &tombstone.high_water;
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "terminal tombstone tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "terminal tombstone sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "terminal tombstone revision exceeds PostgreSQL range".to_string())?;
    let acknowledged_at_unix_ms = i64::try_from(tombstone.acknowledged_at_unix_ms)
        .map_err(|_| "terminal tombstone timestamp exceeds PostgreSQL range".to_string())?;
    let sealed = sqlx::query::query(
        "update trnm_online_terminal_publication_acks a
            set local_tombstone_state = 'sealed'
           from trnm_online_matches m
          where a.match_id = $1 and m.match_id = a.match_id
            and a.local_tombstone_state in (
                'legacy_bootstrap_pending', 'hot_pending', 'sealed'
            )
            and a.actor_generation = $2
            and a.instance_id = $3
            and a.actor_epoch = $4
            and a.physical_host_id = $5
            and a.authoritative_tick = $6
            and a.next_sequence = $7
            and a.match_revision = $8
            and a.next_input_sequences = $9
            and a.snapshot_hash = $10
            and a.phase = 'complete'
            and a.result_hash = $11
            and (
                a.published_settlement_state = $12
                or ($12 = 'pending' and a.published_settlement_state = 'settled')
            )
            and floor(extract(epoch from a.acknowledged_at) * 1000)::bigint = $13
            and m.phase = 'complete'
            and m.checkpoint_sequence = m.next_sequence
            and m.result_hash is not null
            and m.settlement_state in ('pending', 'settled')
            and m.terminal_publication_state = 'acknowledged'
            and m.terminal_publication_actor_generation = a.actor_generation
            and m.assigned_instance_id = a.instance_id
            and m.assigned_instance_epoch = a.actor_epoch
            and m.assigned_physical_host_id = a.physical_host_id
            and m.authoritative_tick = a.authoritative_tick
            and m.next_sequence = a.next_sequence
            and m.match_revision = a.match_revision
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
            and m.snapshot_hash = a.snapshot_hash
            and m.result_hash = a.result_hash
            and m.settlement_state = a.published_settlement_state
        returning a.match_id",
    )
    .bind(high_water.match_id)
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
    .bind(&tombstone.result_hash)
    .bind(&tombstone.settlement_state)
    .bind(acknowledged_at_unix_ms)
    .fetch_optional(executor)
    .await
    .map_err(|error| error.to_string())?;
    if sealed.is_none() {
        return Err(
            "cold terminal tombstone could not seal its exact durable ACK authority".to_string(),
        );
    }
    Ok(())
}

fn terminal_tombstone_matches_commit(
    tombstone: &PublishedTickAckTombstone,
    high_water: &PublishedTickHighWater,
    commit: &TerminalPublicationCommit,
    lineage: &DatabaseLineage,
) -> bool {
    tombstone.high_water == *high_water
        && tombstone.result_hash == commit.result_hash
        && terminal_tombstone_settlement_matches(
            &tombstone.settlement_state,
            &commit.settlement_state,
        )
        && tombstone.acknowledged_at_unix_ms == commit.acknowledged_at_unix_ms
        && database_lineage_covers_tombstone(lineage, tombstone)
}

async fn ensure_cold_terminal_ack(
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    commit: &TerminalPublicationCommit,
    lineage: &DatabaseLineage,
) -> Result<PublishedTickAckTombstone, String> {
    if high_water.phase != "complete" || !high_water.receipts_replayable {
        return Err("cold terminal ACK requires a replayable terminal high-water".to_string());
    }
    if let Some(existing) = journal.ack_tombstone(high_water.match_id)? {
        if !terminal_tombstone_matches_commit(&existing, high_water, commit, lineage) {
            return Err(
                "durable cold terminal ACK conflicts with PostgreSQL authority or lineage"
                    .to_string(),
            );
        }
        return Ok(existing);
    }
    let hot = journal
        .high_water(high_water.match_id)?
        .ok_or_else(|| "cold terminal ACK is missing its exact hot witness".to_string())?;
    if hot != *high_water {
        return Err("cold terminal ACK hot witness changed before sealing".to_string());
    }
    let tombstone = seal_terminal_ack_with_timeout(
        journal,
        PublishedTickAckTombstoneInput {
            high_water: hot,
            result_hash: commit.result_hash.clone(),
            settlement_state: commit.settlement_state.clone(),
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
            reconciled.terminal_acknowledged.insert(evidence.match_id);
        .copied()
