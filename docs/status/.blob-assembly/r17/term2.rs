// TRNM_R17_TERM2_CHUNK_0_PLACEHOLDER_0B2B9C63
                   or terminal_publication_actor_generation = $2
               )",
        )
        .bind(high_water.match_id)
        .bind(high_water.actor_generation)
        .execute(&mut *transaction)
        .await
        .map_err(|error| error.to_string())?;
        if bound.rows_affected() != 1 {
// TRNM_R17_TERM2_CHUNK_1_PLACEHOLDER_0B2B9C63
            .try_get("assigned_instance_id")
            .map_err(|error| error.to_string())?,
        assigned_instance_epoch: row
            .try_get("assigned_instance_epoch")
            .map_err(|error| error.to_string())?,
        assigned_physical_host_id: row
            .try_get("assigned_physical_host_id")
            .map_err(|error| error.to_string())?,
    };
// TRNM_R17_TERM2_CHUNK_2_PLACEHOLDER_0B2B9C63
    .bind(migration_checksum)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "V13 migration ledger entry is absent or drifted".to_string())?;
    let legacy_origin = match_updated_at < migration_applied_at;
    if !marker_exists {
        if !adopt_legacy_pre_v13 {
            return Err(format!(
// TRNM_R17_TERM2_CHUNK_3_PLACEHOLDER_0B2B9C63
