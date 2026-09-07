async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
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
    let mut durable_member_cursors = BTreeMap::new();
    for member in member_rows {
        durable_member_cursors.insert(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            u64::try_from(
                member
                    .try_get::<i64, _>("next_input_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "failed-closed member cursor is negative".to_string())?,
        );
    }
    let durable_snapshot_hash = row
        .try_get::<String, _>("snapshot_hash")
        .map_err(|error| error.to_string())?;
    let failure_reason = row
        .try_get::<Option<String>, _>("failure_reason")
        .map_err(|error| error.to_string())?
        .unwrap_or_default();
    if let Some(expected) = maintenance_expectation {
        if !failed_closed_maintenance_expectation_matches(high_water, &failure_reason, expected) {
            return Err(
                "maintenance identity or failure reason conflicts with failed-closed authority"
                    .to_string(),
            );
        }
    }
    let exact = row
        .try_get::<String, _>("settlement_state")
        .map_err(|error| error.to_string())?
        == "failed_closed"
        && row
            .try_get::<String, _>("terminal_publication_state")
            .map_err(|error| error.to_string())?
            == "pending"
        && row
            .try_get::<Option<String>, _>("assigned_instance_id")
            .map_err(|error| error.to_string())?
            .as_deref()
            == Some(high_water.instance_id.as_str())
        && row
            .try_get::<Option<i64>, _>("assigned_instance_epoch")
            .map_err(|error| error.to_string())?
            == Some(high_water.actor_epoch)
        && row
            .try_get::<Option<String>, _>("assigned_physical_host_id")
            .map_err(|error| error.to_string())?
            .as_deref()
            == Some(high_water.physical_host_id.as_str())
        && row
            .try_get::<Option<String>, _>("result_hash")
            .map_err(|error| error.to_string())?
            .is_none()
        && row
            .try_get::<Option<Value>, _>("result_json")
            .map_err(|error| error.to_string())?
            .is_none()
        && row
            .try_get::<Option<Uuid>, _>("terminal_publication_actor_generation")
            .map_err(|error| error.to_string())?
            .is_none()
        && !row
            .try_get::<bool, _>("has_terminal_ack")
            .map_err(|error| error.to_string())?
        && row
            .try_get::<bool, _>("terminal_stage_empty")
            .map_err(|error| error.to_string())?
        && !simulation.terminal()
        && simulation.tick == durable_tick
        && simulation
            .snapshot_hash()
            .map_err(|error| format!("hash failed-closed simulation: {error}"))?
            == durable_snapshot_hash
        && durable_tick == high_water.tick
        && durable_next_sequence == high_water.next_sequence
        && durable_checkpoint_sequence == high_water.next_sequence
        && durable_match_revision == high_water.match_revision
        && durable_member_cursors == high_water.next_input_sequences
        && durable_snapshot_hash == high_water.snapshot_hash
        && valid_abandonment_failure_reason(&failure_reason);
    if !exact {
        return Err(
            "failed-closed authority does not exactly match its retained hot witness".to_string(),
        );
    }
    let marker_exists = sqlx::query_scalar::query_scalar::<_, bool>(
        "select exists(
            select 1 from trnm_online_failed_closed_abandonment_markers
             where match_id = $1
        )",
    )
    .bind(high_water.match_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let match_updated_at: DateTime<Utc> = row
        .try_get("updated_at")
        .map_err(|error| error.to_string())?;
    let migration_checksum = migration_checksum_sha256(MIGRATION_V13);
    let migration_applied_at = sqlx::query_scalar::query_scalar::<_, DateTime<Utc>>(
        "select applied_at
           from public.trnm_online_schema_migrations
          where migration_version = 13
            and migration_name = '0013_online_failed_closed_abandonment_v1'
            and checksum_sha256 = $1",
    )
    .bind(migration_checksum)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "V13 migration ledger entry is absent or drifted".to_string())?;
    let legacy_origin = match_updated_at < migration_applied_at;
    if !marker_exists {
        if !adopt_legacy_pre_v13 {
            return Err(format!(
                "failed-closed match {} has no atomic abandonment marker; use the explicit pre-V13 adoption maintenance path only for a proven legacy row",
                high_water.match_id
            ));
        }
        if !legacy_abandonment_adoption_allowed(
            adopt_legacy_pre_v13,
            match_updated_at,
            migration_applied_at,
        ) {
            return Err(format!(
                "failed-closed match {} is not strictly older than the V13 ledger and cannot use legacy adoption",
                high_water.match_id
            ));
        }
        if maintenance_expectation.is_none() {
            return Err(
                "legacy abandonment adoption requires explicit maintenance identity and reason"
                    .to_string(),
            );
        }
        sqlx::query::query(
            "insert into trnm_online_failed_closed_abandonment_markers (
            match_id, journal_owner_id, actor_generation, instance_id,
            actor_epoch, physical_host_id, authoritative_tick, next_sequence,
            match_revision, next_input_sequences, snapshot_hash, failure_reason
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
         ",
        )
        .bind(high_water.match_id)
        .bind(high_water.journal_owner_id)
        .bind(high_water.actor_generation)
        .bind(&high_water.instance_id)
        .bind(high_water.actor_epoch)
        .bind(&high_water.physical_host_id)
        .bind(
            i64::try_from(high_water.tick).map_err(|_| {
                "failed-closed high-water tick exceeds PostgreSQL range".to_string()
            })?,
        )
        .bind(i64::try_from(high_water.next_sequence).map_err(|_| {
            "failed-closed high-water sequence exceeds PostgreSQL range".to_string()
        })?)
        .bind(i64::try_from(high_water.match_revision).map_err(|_| {
            "failed-closed high-water revision exceeds PostgreSQL range".to_string()
        })?)
        .bind(
            serde_json::to_value(&high_water.next_input_sequences)
                .map_err(|error| error.to_string())?,
        )
        .bind(&high_water.snapshot_hash)
        .bind(&failure_reason)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    let evidence =
        load_abandonment_database_evidence_by_match(&mut *transaction, high_water.match_id)
            .await?
            .ok_or_else(|| {
                "failed-closed abandonment marker disappeared after insert".to_string()
            })?;
    if !evidence.matches_high_water(high_water) || evidence.failure_reason != failure_reason {
        return Err(
            "failed-closed abandonment marker conflicts with the exact hot witness".to_string(),
        );
    }
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    transaction
        .commit()
        .await
        .map_err(|error| error.to_string())?;
    let lineage = database_lineage(&mut *connection).await?;
    Ok(Some((evidence, lineage, legacy_origin)))
}

async fn lock_maintenance_fleet_assignment(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    instance_id: &str,
    physical_host_id: &str,
    expected_epoch: Option<i64>,
) -> Result<(), String> {
    let row = sqlx::query::query(
        "select instance_epoch, physical_host_id
           from trnm_online_fleet_instances
          where instance_id = $1
          for share",
    )
    .bind(instance_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "maintenance fleet assignment is missing".to_string())?;
    let durable_epoch: i64 = row
        .try_get("instance_epoch")
        .map_err(|error| error.to_string())?;
    let durable_host: String = row
        .try_get("physical_host_id")
        .map_err(|error| error.to_string())?;
    if durable_host != physical_host_id
        || expected_epoch.is_some_and(|epoch| epoch != durable_epoch)
    {
        return Err(
            "maintenance fleet assignment is fenced by a different host or epoch".to_string(),
        );
    }
    Ok(())
}

fn running_maintenance_successor_is_monotonic(
    high_water: &PublishedTickHighWater,
    durable_tick: u64,
    durable_next_sequence: u64,
    durable_match_revision: u64,
    durable_member_cursors: &BTreeMap<String, u64>,
    durable_snapshot_hash: &str,
) -> bool {
    if durable_tick < high_water.tick
        || durable_next_sequence < high_water.next_sequence
        || durable_match_revision < high_water.match_revision
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
    Ok(final_high_water)
