pub fn production_authority_tick_interval() -> Duration {
    assert_eq!(1_000 % TICKS_PER_SECOND, 0);
    Duration::from_millis(1_000 / TICKS_PER_SECOND)
}

pub fn resolve_authority_tick_interval(
    requested_ms: Option<u64>,
    allow_accelerated_test_clock: bool,
) -> Result<Duration, String> {
    let production = production_authority_tick_interval();
    let requested = Duration::from_millis(
        requested_ms.unwrap_or_else(|| u64::try_from(production.as_millis()).unwrap_or(100)),
    );
    if requested.is_zero() || requested > Duration::from_secs(1) {
        return Err("TRNM_GAME_SERVER_TICK_MS must be between 1 and 1000".to_string());
    }
    if requested != production && !allow_accelerated_test_clock {
        return Err(format!(
            "production authority is fixed at {}ms ({}Hz); non-real-time clocks require TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1",
            production.as_millis(),
            TICKS_PER_SECOND
        ));
    }
    Ok(requested)
            return Err(
                "match reconciliation query returned a remote physical-host assignment".to_string(),
            );
        }
        match ensure_match_actor(state, match_id).await {
            Ok(Some(_)) => active = active.saturating_add(1),
            Ok(None) => {}
            Err(error) => {
                tracing::error!(%match_id, %error, "running match quarantined after actor recovery failure");
                // Rotate a bad match behind healthy candidates. One corrupt
                // recovery record must not permanently consume the first slot
                // of every bounded reconciliation batch.
                if let Err(update_error) = sqlx::query::query(
                    "update trnm_online_matches set updated_at = now()
                     where match_id = $1 and phase = 'running'
                       and assigned_instance_id is not distinct from $2
                       and assigned_instance_epoch = $3
                       and assigned_physical_host_id is not distinct from $4
                       and (assigned_instance_id is null or assigned_physical_host_id = $5)",
                )
                .bind(match_id)
                .bind(&assigned_instance_id)
                .bind(assigned_instance_epoch)
                .bind(&assigned_physical_host_id)
                    next_sequence, match_revision, checkpoint_sequence,
                    assigned_instance_id, assigned_region, assigned_instance_epoch,
                    assigned_physical_host_id,
                    terminal_stage_simulation_json, terminal_stage_result_json,
                    terminal_stage_result_hash, terminal_stage_snapshot_hash,
                    terminal_stage_authoritative_tick, terminal_stage_next_sequence,
                    terminal_stage_match_revision
             from trnm_online_matches
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    if row
        .try_get::<String, _>("phase")
        .map_err(|error| error.to_string())?
        != "running"
    {
        transaction
            .commit()
        }
    }
    Ok(Some(loaded))
}

fn match_assignment_uses_local_physical_host(
    assigned_instance: Option<&str>,
    assigned_physical_host: Option<&str>,
    local_physical_host: &str,
) -> bool {
    assigned_instance.is_none() || assigned_physical_host == Some(local_physical_host)
}

const MAX_PUBLISHED_TICK_RECOVERY_STEPS: u64 = 10_000;

fn recover_to_published_high_water(
    simulation: &mut MissionSimV1,
    durable_db_next_sequence: u64,
    durable_db_match_revision: u64,
    durable_db_next_input_sequences: &BTreeMap<String, u64>,
    bridge_simulation: Option<MissionSimV1>,
    high_water: &PublishedTickHighWater,
) -> Result<(), String> {
    if high_water.phase != "running" || !high_water.receipts_replayable {
async fn seal_abandonment_with_timeout(
    journal: &PublishedTickJournal,
    input: PublishedTickAbandonmentTombstoneInput,
) -> Result<PublishedTickAbandonmentTombstone, String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.seal_abandonment(input),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}
