async fn terminal_high_water_is_durably_acknowledged(
                .try_get("assigned_instance_id")
            row.try_get::<i64, _>("accepted_match_revision")
                "running high-water recovery is missing initial simulation".to_string()
    .map_err(|_| "failed-closed checkpoint sequence is negative".to_string())?;
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
        || high_water
            .next_input_sequences
            .iter()
            .any(|(player_id, cursor)| {
                durable_member_cursors
                    .get(player_id)
                    .is_none_or(|durable| durable < cursor)
            })
    {
        return false;
    }
    let same_position = durable_tick == high_water.tick
        && durable_next_sequence == high_water.next_sequence
        && durable_match_revision == high_water.match_revision
        && *durable_member_cursors == high_water.next_input_sequences;
    !same_position || durable_snapshot_hash == high_water.snapshot_hash
}

/// Rebind the retained hot witness to the exact durable running checkpoint
/// before making the irreversible failed-closed transition. PostgreSQL is the
/// authority here: maintenance never replays or invents state. It accepts only
/// the same tuple or a fully validated monotonic successor, fsyncs that exact
/// DB tuple to the existing journal ownership, then lets the transition
/// transaction re-check every field against the newly retained witness.
async fn prepare_final_abandonment_high_water(
    connection: &mut PgConnection,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<PublishedTickHighWater, String> {
    if high_water.phase != "running" || !high_water.receipts_replayable {
        return Err("maintenance requires a replayable running hot witness".to_string());
    }
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
    let row = sqlx::query::query(
        "select m.phase, m.settlement_state, m.terminal_publication_state,
                m.simulation_json, m.snapshot_hash, m.authoritative_tick,
                m.next_sequence, m.checkpoint_sequence, m.match_revision,
                m.assigned_instance_id, m.assigned_instance_epoch,
                m.assigned_physical_host_id, m.result_json, m.result_hash,
                m.failure_reason, m.terminal_publication_actor_generation,
                m.terminal_stage_simulation_json is null
                    and m.terminal_stage_result_json is null
                    and m.terminal_stage_result_hash is null
                    and m.terminal_stage_snapshot_hash is null
                    and m.terminal_stage_authoritative_tick is null
                    and m.terminal_stage_next_sequence is null
                    and m.terminal_stage_match_revision is null
                    and m.terminal_staged_at is null as terminal_stage_empty,
                exists(
                    select 1 from trnm_online_terminal_publication_acks terminal
        "running maintenance database checkpoint",
            .try_get::<Option<String>, _>("failure_reason")
    Ok(final_high_water)
