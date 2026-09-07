async fn persist_actor_command(
    state: &AppState,
    match_id: Uuid,
    job: ActorCommandPersistence,
) -> Result<OnlineCommandReceipt, ApiError> {
    if !state.database_host_authority.is_healthy() {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "PostgreSQL host authority is fail-closed",
            true,
        ));
    }
    let identity = &state.database_host_authority.identity;
    let order_json = serde_json::to_value(&job.order).map_err(internal_serialization)?;
    // This is intentionally one PostgreSQL statement. The migration function
    // revalidates the exact K1 leader, holds the pool session's lifetime K2
    // barrier, locks fleet -> match -> member, and either advances the command
    // event plus both cursors atomically or changes nothing.
    let database_started = Instant::now();
    let row = sqlx::query::query(ONLINE_COMMAND_COMMIT_V2_SQL)
        .bind(match_id)
        .bind(&job.request.command_id)
        .bind(&job.request.player_id)
        .bind(job.input_sequence as i64)
        "enemy-player".to_string()
    } else {
        "player".to_string()
    };
    order.frame = u32::try_from(target_tick).map_err(|_| {
        api_error(
            StatusCode::BAD_REQUEST,
            "target_tick exceeds frame range",
            false,
        )
    })?;
    order.source = RtsOrderSource::LocalInput;
    if match_mode == "ranked_pvp" && order.kind == trnm_rts_protocol::RtsOrderKind::Extract {
        return Err(api_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "ranked PvP does not allow withdrawal",
            false,
        ));
    }
    let merged_with_active_coop_order = match_mode == "coop_vs_ai"
        && order.queued
        && simulation.active_order.as_ref().is_some_and(|active| {
            active.kind == order.kind
                && active.target_tile == order.target_tile
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "terminal match is missing its exact durable checkpoint",
            true,
        ));
    }
    let simulation = serde_json::from_value::<MissionSimV1>(simulation_json.clone())
        .map_err(internal_serialization)?;
    let expected_tick = row
        .try_get::<i64, _>("authoritative_tick")
        .map_err(internal_db)? as u64;
    let expected_hash: String = row.try_get("snapshot_hash").map_err(internal_db)?;
    let computed_hash = simulation
        .snapshot_hash()
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    if !simulation.terminal() || simulation.tick != expected_tick || computed_hash != expected_hash
    {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "terminal durable checkpoint failed tick/hash validation",
            true,
        ));
    }
    Ok(simulation_json)
        ));
    }
    // Release the match authority snapshot before updating reconnect telemetry.
    // Keeping the member row lock inside this transaction lets concurrent
    // reconnects queue while all of them still hold a match FOR SHARE lock,
    // which blocks the actor's canonical match FOR UPDATE command commit.
    transaction.commit().await.map_err(internal_db)?;
    let reconnect_count: i64 = sqlx::query_scalar::query_scalar(
        "update trnm_online_match_members set reconnect_count = reconnect_count + 1,
            last_acknowledged_sequence = greatest(last_acknowledged_sequence, $4),
            last_snapshot_hash = $5, last_seen_at = now()
         where match_id = $1 and player_id = $2 and account_id = $3
         returning reconnect_count",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(next_receipt_sequence as i64)
    .bind(&snapshot_hash)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| {
        api_error(
            "insert into trnm_online_progression_events (
                event_id, match_id, player_id, account_id, campaign_id, result_hash,
                experience_delta, reputation_delta, inventory_delta, campaign_revision
             ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(format!("online-progression:{match_id}:{player_id}"))
        .bind(match_id)
        .bind(&player_id)
        .bind(account_id)
        .bind(&campaign_id)
        .bind(combined_result_hash)
        .bind(experience_delta as i64)
        .bind(reputation_delta)
        .bind(serde_json::to_value(&combined_result.loot).map_err(|error| error.to_string())?)
        .bind(campaign.revision as i64)
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
        any_pending |= !campaign.pending_economic_intents.is_empty()
            || !campaign.pending_economic_compensations.is_empty();
    }
    Ok(any_pending)
}
