// TRNM_R17_TERM1_CHUNK_0_PLACEHOLDER_0B2B9C63
        terminal_total_count: read_nonnegative("terminal_total_count")?,
        terminal_sealed_count: read_nonnegative("terminal_sealed_count")?,
        abandonment_total_count: read_nonnegative("abandonment_total_count")?,
        abandonment_sealed_count: read_nonnegative("abandonment_sealed_count")?,
    };
    if cold_witness.terminal_sealed_count > cold_witness.terminal_total_count
        || cold_witness.abandonment_sealed_count > cold_witness.abandonment_total_count
    {
        return Err("readiness cold witness summary sealed count exceeds its total".to_string());
// TRNM_R17_TERM1_CHUNK_1_PLACEHOLDER_0B2B9C63
    }
    let tombstone = seal_terminal_ack_with_timeout(
        journal,
        PublishedTickAckTombstoneInput {
            high_water: hot,
            result_hash: commit.result_hash.clone(),
            settlement_state: commit.settlement_state.clone(),
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
            database_system_identifier: lineage.system_identifier.clone(),
// TRNM_R17_TERM1_CHUNK_2_PLACEHOLDER_0B2B9C63
            .await?
            .ok_or_else(|| {
                format!(
                    "latest cold abandonment {} is absent from PostgreSQL",
                    tombstone.high_water.match_id
                )
            })?;
            if !abandonment_tombstone_matches_evidence(&tombstone, &evidence, lineage) {
                return Err(format!(
// TRNM_R17_TERM1_CHUNK_3_PLACEHOLDER_0B2B9C63
