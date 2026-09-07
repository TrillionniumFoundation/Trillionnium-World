@@TRNM_R18_T2_SEGMENT_0@@
            result_valid,
            settlement_state: settlement_state.clone(),
            next_input_sequences: member_cursors,
            assigned_instance_id: row
                .try_get("assigned_instance_id")
                .map_err(|error| error.to_string())?,
            assigned_instance_epoch: row
                .try_get("assigned_instance_epoch")
@@TRNM_R18_T2_SEGMENT_1@@
    Ok(StartupCommandRecoveryEvidence {
        simulation,
        accepted_revision: u64::try_from(
            row.try_get::<i64, _>("accepted_match_revision")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| format!("{label} accepted revision is negative"))?,
        player_id: row
            .try_get("player_id")
            .map_err(|error| error.to_string())?,
        input_sequence: u64::try_from(input_sequence)
            .map_err(|_| format!("{label} input cursor is negative"))?,
        member_role: row
            .try_get("member_role")
            .map_err(|error| error.to_string())?,
        order: serde_json::from_value(order_value)
            .map_err(|error| format!("decode {label} command order: {error}"))?,
    })
}

fn replay_running_high_water_to_terminal(
    mut source: MissionSimV1,
    high_water: &PublishedTickHighWater,
    successor: Option<&StartupCommandRecoveryEvidence>,
    terminal_simulation: &MissionSimV1,
    terminal_hash: &str,
    match_mode: &str,
) -> Result<(), String> {
    replay_recovery_to_tick(&mut source, high_water.tick)?;
    verify_recovery_hash(&source, &high_water.snapshot_hash)?;
    if source.terminal() {
        return Err("running high-water already contains terminal simulation".to_string());
    }
    if let Some(successor) = successor {
        replay_recovery_to_tick(&mut source, successor.simulation.tick)?;
        let reconstructed = prepare_and_apply_actor_order(
            &mut source,
            match_mode,
            &successor.member_role,
            u64::from(successor.order.frame),
            successor.order.clone(),
        )
        .map_err(|error| error.body.error)?;
        if reconstructed != successor.order {
            return Err(
                "terminal successor command order changed during reconstruction".to_string(),
            );
        }
        verify_recovery_hash(
            &source,
            &successor
                .simulation
                .snapshot_hash()
                .map_err(|error| error.to_string())?,
        )?;
        source = successor.simulation.clone();
    }
    replay_recovery_to_tick(&mut source, terminal_simulation.tick)?;
    verify_recovery_hash(&source, terminal_hash)?;
    if !source.terminal()
        || source.snapshot_hash().map_err(|error| error.to_string())? != terminal_hash
    {
        return Err(
            "running high-water did not deterministically reach exact terminal state".to_string(),
        );
    }
    Ok(())
}

async fn recover_terminal_high_water_after_running_crash(
    pool: &PgPool,
    journal: &PublishedTickJournal,
    high_water: &PublishedTickHighWater,
    database_host_authority: &DatabaseHostAuthorityFence,
) -> Result<Option<PublishedTickHighWater>, String> {
    if high_water.phase != "running" || !high_water.receipts_replayable {
        return Ok(None);
    }
    let mut transaction = pool.begin().await.map_err(|error| error.to_string())?;
    require_database_host_authority(&mut transaction, database_host_authority).await?;
    let Some(row) = sqlx::query::query(
        "select phase, simulation_json, initial_simulation_json, snapshot_hash,
                authoritative_tick, next_sequence, match_revision, checkpoint_sequence,
                result_json, result_hash, settlement_state, match_mode,
                assigned_instance_id, assigned_instance_epoch, assigned_physical_host_id,
                terminal_stage_simulation_json, terminal_stage_result_json,
                terminal_stage_result_hash, terminal_stage_snapshot_hash,
                terminal_stage_authoritative_tick, terminal_stage_next_sequence,
                terminal_stage_match_revision
         from trnm_online_matches where match_id = $1 for share",
    )
    .bind(high_water.match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let durable_phase = row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?;
    let match_mode = row
        .try_get::<String, _>("match_mode")
        .map_err(|error| error.to_string())?;
    let (
        terminal_simulation,
        terminal_hash,
        durable_hash,
        authoritative_tick,
        next_sequence,
        match_revision,
        checkpoint_sequence,
        result_valid,
        terminal_settlement_state,
    ) = if durable_phase == "complete" {
        let terminal_value = row
            .try_get::<Option<Value>, _>("simulation_json")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "completed match is missing terminal simulation".to_string())?;
        let terminal_simulation = serde_json::from_value::<MissionSimV1>(terminal_value)
            .map_err(|error| format!("decode completed terminal simulation: {error}"))?;
        let terminal_hash = terminal_simulation
            .snapshot_hash()
            .map_err(|error| format!("hash completed terminal simulation: {error}"))?;
        let result_hash = row
            .try_get::<Option<String>, _>("result_hash")
            .map_err(|error| error.to_string())?;
        let result_valid = terminal_result_matches_simulation(
            &terminal_simulation,
            row.try_get::<Option<Value>, _>("result_json")
                .map_err(|error| error.to_string())?,
            result_hash.as_deref(),
            &match_mode,
        )?;
        (
            terminal_simulation,
            terminal_hash,
            row.try_get::<String, _>("snapshot_hash")
                .map_err(|error| error.to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("authoritative_tick")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed terminal tick is negative".to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("next_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed terminal sequence is negative".to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("match_revision")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed terminal revision is negative".to_string())?,
            u64::try_from(
                row.try_get::<i64, _>("checkpoint_sequence")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "completed checkpoint sequence is negative".to_string())?,
            result_valid,
            row.try_get::<String, _>("settlement_state")
                .map_err(|error| error.to_string())?,
        )
    } else if durable_phase == "running" {
        let Some(staged) = staged_terminal_authority_from_row(&row, &match_mode)? else {
            transaction
                .commit()
                .await
                .map_err(|error| error.to_string())?;
            return Ok(None);
        };
        (
            staged.simulation,
            staged.snapshot_hash.clone(),
            staged.snapshot_hash,
            staged.authoritative_tick,
            staged.next_sequence,
            staged.match_revision,
            staged.next_sequence,
            true,
            "staged".to_string(),
        )
    } else {
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;
        return Ok(None);
    };
    let member_rows = sqlx::query::query(
        "select player_id, next_input_sequence from trnm_online_match_members
         where match_id = $1 order by player_id",
    )
    .bind(high_water.match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut member_cursors = BTreeMap::new();
    for member in member_rows {
        let cursor = u64::try_from(
            member
                .try_get::<i64, _>("next_input_sequence")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| "completed member cursor is negative".to_string())?;
        member_cursors.insert(
            member
                .try_get("player_id")
                .map_err(|error| error.to_string())?,
            cursor,
        );
    }
    let view = DurableTerminalCompactionView {
        phase: "complete".to_string(),
        simulation_tick: terminal_simulation.tick,
        simulation_terminal: terminal_simulation.terminal(),
        simulation_hash: terminal_hash.clone(),
        durable_snapshot_hash: durable_hash,
        authoritative_tick: Some(authoritative_tick),
        next_sequence: Some(next_sequence),
        checkpoint_sequence: Some(checkpoint_sequence),
        match_revision: Some(match_revision),
        result_valid,
        settlement_state: terminal_settlement_state,
        next_input_sequences: member_cursors.clone(),
        assigned_instance_id: row
            .try_get("assigned_instance_id")
            .map_err(|error| error.to_string())?,
        assigned_instance_epoch: row
            .try_get("assigned_instance_epoch")
            .map_err(|error| error.to_string())?,
        assigned_physical_host_id: row
            .try_get("assigned_physical_host_id")
            .map_err(|error| error.to_string())?,
    };
    if !terminal_authority_succeeds_running_high_water(&view, high_water) {
        return Err(
            "completed database authority is not an exact successor of its running high-water"
                .to_string(),
        );
    }

    let source = if high_water.next_sequence == 0 {
        let initial_value = row
            .try_get::<Option<Value>, _>("initial_simulation_json")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| {
                "running high-water recovery is missing initial simulation".to_string()
            })?;
        let initial = serde_json::from_value::<MissionSimV1>(initial_value.clone())
@@TRNM_R18_T2_SEGMENT_3@@
          where match_id = $1
          order by player_id
          for update",
    )
    .bind(high_water.match_id)
    .fetch_all(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?;
@@TRNM_R18_T2_SEGMENT_4@@
        || durable_next_sequence < high_water.next_sequence
        || durable_match_revision < high_water.match_revision
        || durable_member_cursors.len() != high_water.next_input_sequences.len()
        || high_water
            .next_input_sequences
            .iter()
            .any(|(player_id, cursor)| {
                durable_member_cursors
@@TRNM_R18_T2_SEGMENT_5@@
