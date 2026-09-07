@@TRNM_R18_T1_SEGMENT_0@@
    )
    .map_err(|error| format!("decode terminal ACK input cursors: {error}"))?;
    Ok(TerminalAckDatabaseEvidence {
        match_id: row.try_get("match_id").map_err(|error| error.to_string())?,
        actor_generation: row
            .try_get("actor_generation")
            .map_err(|error| error.to_string())?,
        instance_id: row
@@TRNM_R18_T1_SEGMENT_1@@
        actor_generation: row
            .try_get("actor_generation")
            .map_err(|error| error.to_string())?,
        instance_id: row
            .try_get("instance_id")
            .map_err(|error| error.to_string())?,
        actor_epoch: row
            .try_get("actor_epoch")
@@TRNM_R18_T1_SEGMENT_2@@
        PublishedTickAckTombstoneInput {
            high_water: hot,
            result_hash: commit.result_hash.clone(),
            settlement_state: commit.settlement_state.clone(),
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
            database_system_identifier: lineage.system_identifier.clone(),
            database_timeline_id: lineage.timeline_id,
            database_wal_lsn: lineage.wal_flush_lsn.clone(),
@@TRNM_R18_T1_SEGMENT_3@@
        &mut connection,
        high_water,
        database_host_authority,
        false,
        None,
    )
    .await?
    else {
@@TRNM_R18_T1_SEGMENT_4@@
            .await?;
            reconciled.terminal_acknowledged.insert(evidence.match_id);
        }
        if page_len < usize::try_from(TERMINAL_ACK_STARTUP_PAGE_SIZE).unwrap_or(usize::MAX) {
            break;
        }
    }

@@TRNM_R18_T1_SEGMENT_5@@
