const DEFAULT_SHUTDOWN_GRACE_MILLISECONDS_V2: u64 = 20_000;
const MAX_SHUTDOWN_GRACE_MILLISECONDS_V2: u64 = 120_000;
const DEFAULT_QUARANTINE_RETRY_SECONDS_V2: i64 = 300;
const MIGRATION_V18_V2: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");
const MIGRATION_V19_V2: &str =
    include_str!("../migrations/0019_online_settlement_quarantine_v1.sql");

/// Runtime v2 stops admission on SIGINT/SIGTERM, drains already-started remote
/// work for a bounded interval, and leaves unfinished leases to expire. Capture,
/// claim/decode, remote execution, and apply failures are isolated so one bad
/// match/account cannot block unrelated settlement work.
pub async fn run_v2(config: WorkerConfig) -> Result<(), String> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(config.pool_max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.database_url)
        .await
        .map_err(|error| format!("connect settlement worker PostgreSQL pool: {error}"))?;
    apply_worker_migrations_v2(&pool).await?;

    let cex = CexClient::new(
        config.cex_url.clone(),
        config.game_authority_token.clone(),
        config.signer_url.clone(),
        config.signer_token.clone(),
    )?;
    cex.readiness().await?;

    let max_in_flight = parse_env_range(
        "TRNM_SETTLEMENT_MAX_IN_FLIGHT",
        config.batch_size.clamp(1, 8),
        1_usize,
        64_usize,
    )?;
    let shutdown_grace_milliseconds = parse_env_range(
        "TRNM_SETTLEMENT_SHUTDOWN_GRACE_MILLISECONDS",
        DEFAULT_SHUTDOWN_GRACE_MILLISECONDS_V2,
        1_000_u64,
        MAX_SHUTDOWN_GRACE_MILLISECONDS_V2,
    )?;
    let shutdown_grace = Duration::from_millis(shutdown_grace_milliseconds);

    tracing::info!(
        worker_id = %config.worker_id,
        batch_size = config.batch_size,
        max_in_flight,
        lease_milliseconds = config.lease_milliseconds,
        shutdown_grace_milliseconds,
        "transaction-free settlement worker runtime v2 is ready"
    );

    let mut shutdown = Box::pin(settlement_shutdown_signal_v2());
    let mut in_flight = tokio::task::JoinSet::<Result<(), String>>::new();

    loop {
        reap_finished_remote_work_v2(&mut in_flight);

        let mut progress = capture_pending_matches_isolated_v2(&pool, config.batch_size).await?;

        while in_flight.len() < max_in_flight {
            match claim_settlement_job_isolated_v2(
                &pool,
                &config.worker_id,
                config.lease_milliseconds,
            )
            .await?
            {
                Some(job) => {
                    progress = progress.saturating_add(1);
                    let task_pool = pool.clone();
                    let task_cex = cex.clone();
                    in_flight.spawn(async move {
                        process_claimed_job(&task_pool, &task_cex, job).await
                    });
                }
                None => break,
            }
        }

        progress = progress.saturating_add(
            apply_ready_captures_isolated_v2(&pool, config.batch_size).await?,
        );

        if progress == 0 {
            tokio::select! {
                signal = &mut shutdown => {
                    signal?;
                    break;
                }
                joined = in_flight.join_next(), if !in_flight.is_empty() => {
                    if let Some(joined) = joined {
                        report_remote_join_v2(joined);
                    }
                }
                _ = sleep(config.poll_interval) => {}
            }
        } else {
            tokio::select! {
                biased;
                signal = &mut shutdown => {
                    signal?;
                    break;
                }
                _ = tokio::task::yield_now() => {}
            }
        }
    }

    tracing::info!(
        worker_id = %config.worker_id,
        in_flight = in_flight.len(),
        "settlement shutdown stopped new capture and claim admission"
    );
    drain_remote_work_v2(&mut in_flight, shutdown_grace).await;
    tracing::info!(worker_id = %config.worker_id, "settlement worker shutdown complete");
    Ok(())
}

async fn apply_worker_migrations_v2(pool: &PgPool) -> Result<(), String> {
    apply_worker_migrations(pool).await?;

    let mut connection = pool
        .acquire()
        .await
        .map_err(|error| format!("acquire v2 migration connection: {error}"))?;
    connection
        .execute(MIGRATION_LEDGER_DDL)
        .await
        .map_err(|error| format!("create v2 migration ledger: {error}"))?;
    sqlx::query::query("select pg_advisory_lock($1)")
        .bind(MIGRATION_ADVISORY_LOCK)
        .execute(&mut *connection)
        .await
        .map_err(|error| format!("acquire v2 migration advisory lock: {error}"))?;

    let result = apply_worker_migrations_v2_locked(&mut connection).await;
    let unlock = sqlx::query_scalar::query_scalar::<_, bool>("select pg_advisory_unlock($1)")
        .bind(MIGRATION_ADVISORY_LOCK)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| format!("release v2 migration advisory lock: {error}"));
    match (result, unlock) {
        (Err(error), _) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(true)) => Ok(()),
        (Ok(()), Ok(false)) => Err("v2 migration advisory lock was not held".to_string()),
    }
}

async fn apply_worker_migrations_v2_locked(
    connection: &mut PgConnection,
) -> Result<(), String> {
    for (version, name, sql) in [
        (
            18_i32,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18_V2,
        ),
        (
            19_i32,
            "0019_online_settlement_quarantine_v1",
            MIGRATION_V19_V2,
        ),
    ] {
        let checksum = hash_bytes(sql.as_bytes());
        let recorded = sqlx::query::query(
            "select migration_name, checksum_sha256
               from public.trnm_online_schema_migrations
              where migration_version = $1",
        )
        .bind(version)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| format!("read settlement migration {version}: {error}"))?;
        if let Some(recorded) = recorded {
            let recorded_name: String = recorded
                .try_get("migration_name")
                .map_err(|error| error.to_string())?;
            let recorded_checksum: String = recorded
                .try_get("checksum_sha256")
                .map_err(|error| error.to_string())?;
            if recorded_name != name || recorded_checksum != checksum {
                return Err(format!(
                    "settlement migration {version} checksum/name drift: {recorded_name} {recorded_checksum}"
                ));
            }
            continue;
        }

        let mut transaction = connection
            .begin()
            .await
            .map_err(|error| format!("begin settlement migration {version}: {error}"))?;
        transaction
            .execute(sql)
            .await
            .map_err(|error| format!("execute settlement migration {version}: {error}"))?;
        sqlx::query::query(
            "insert into public.trnm_online_schema_migrations (
                migration_version, migration_name, checksum_sha256
             ) values ($1, $2, $3)",
        )
        .bind(version)
        .bind(name)
        .bind(checksum)
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("record settlement migration {version}: {error}"))?;
        transaction
            .commit()
            .await
            .map_err(|error| format!("commit settlement migration {version}: {error}"))?;
    }
    Ok(())
}

fn reap_finished_remote_work_v2(
    in_flight: &mut tokio::task::JoinSet<Result<(), String>>,
) {
    while let Some(joined) = in_flight.try_join_next() {
        report_remote_join_v2(joined);
    }
}

fn report_remote_join_v2(
    joined: Result<Result<(), String>, tokio::task::JoinError>,
) {
    match joined {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            tracing::error!(%error, "isolated settlement remote task failed; lease remains recoverable")
        }
        Err(error) => {
            tracing::error!(%error, "isolated settlement remote task panicked or was cancelled; lease will expire")
        }
    }
}

async fn drain_remote_work_v2(
    in_flight: &mut tokio::task::JoinSet<Result<(), String>>,
    grace: Duration,
) {
    let drain = async {
        while let Some(joined) = in_flight.join_next().await {
            report_remote_join_v2(joined);
        }
    };
    if tokio::time::timeout(grace, drain).await.is_err() {
        let remaining = in_flight.len();
        tracing::warn!(
            remaining,
            grace_milliseconds = grace.as_millis(),
            "settlement shutdown grace expired; aborting tasks and relying on lease expiry"
        );
        in_flight.abort_all();
        while let Some(joined) = in_flight.join_next().await {
            report_remote_join_v2(joined);
        }
    }
}

async fn settlement_shutdown_signal_v2() -> Result<(), String> {
    #[cfg(unix)]
    {
        let mut terminate = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .map_err(|error| format!("install SIGTERM handler: {error}"))?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                result.map_err(|error| format!("install Ctrl-C handler: {error}"))
            }
            signal = terminate.recv() => {
                signal.ok_or_else(|| "SIGTERM stream closed unexpectedly".to_string()).map(|_| ())
            }
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .map_err(|error| format!("install shutdown handler: {error}"))
    }
}

async fn capture_pending_matches_isolated_v2(
    pool: &PgPool,
    limit: usize,
) -> Result<u64, String> {
    let limit = i64::try_from(limit).map_err(|_| "capture limit is too large".to_string())?;
    let match_ids = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_row.match_id
           from public.trnm_online_matches match_row
          where public.trnm_online_settlement_match_ready_v1(match_row.match_id)
            and not public.trnm_online_settlement_scope_quarantined_v1(
                'match', match_row.match_id::text, 'capture'
            )
            and not exists (
                select 1
                  from public.trnm_online_settlement_captures capture
                 where capture.match_id = match_row.match_id
                   and capture.state in ('active', 'dead_letter')
            )
          order by match_row.updated_at, match_row.match_id
          limit $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|error| format!("scan settlement capture candidates: {error}"))?;

    let mut progress = 0_u64;
    for match_id in match_ids {
        match capture_match(pool, match_id).await {
            Ok(captured) => progress = progress.saturating_add(captured),
            Err(error) => {
                record_quarantine_v2(
                    pool,
                    "match",
                    &match_id.to_string(),
                    "capture",
                    &error,
                    DEFAULT_QUARANTINE_RETRY_SECONDS_V2,
                )
                .await?;
                tracing::error!(%match_id, %error, "settlement capture candidate quarantined");
                progress = progress.saturating_add(1);
            }
        }
    }
    Ok(progress)
}

async fn claim_settlement_job_isolated_v2(
    pool: &PgPool,
    owner: &str,
    lease_milliseconds: i64,
) -> Result<Option<SettlementJob>, String> {
    let row = sqlx::query::query(
        "select * from public.trnm_online_claim_settlement_job_v2($1, $2)",
    )
    .bind(owner)
    .bind(lease_milliseconds)
    .fetch_optional(pool)
    .await
    .map_err(|error| format!("claim settlement job: {error}"))?;
    let Some(row) = row else {
        return Ok(None);
    };

    match settlement_job_from_row(&row) {
        Ok(job) => Ok(Some(job)),
        Err(error) => {
            let job_id = row.try_get::<String, _>("job_id").map_err(|read| {
                format!("claimed poison job omitted job_id after decode failure: {read}; {error}")
            })?;
            let lease_owner = row
                .try_get::<Option<String>, _>("lease_owner")
                .map_err(|read| format!("read poison job lease owner: {read}"))?
                .ok_or_else(|| "claimed poison job has no lease owner".to_string())?;
            let lease_generation = row
                .try_get::<i64, _>("lease_generation")
                .map_err(|read| format!("read poison job lease generation: {read}"))?;
            let quarantined = sqlx::query_scalar::query_scalar::<_, bool>(
                "select public.trnm_online_quarantine_claimed_settlement_job_v1(
                    $1, $2, $3, $4
                 )",
            )
            .bind(&job_id)
            .bind(&lease_owner)
            .bind(lease_generation)
            .bind(&error)
            .fetch_one(pool)
            .await
            .map_err(|db_error| format!("quarantine poison claimed job {job_id}: {db_error}"))?;
            if !quarantined {
                return Err(format!(
                    "poison claimed job {job_id} lost its live lease fence: {error}"
                ));
            }
            tracing::error!(%job_id, %error, "poison settlement job quarantined");
            Ok(None)
        }
    }
}

async fn apply_ready_captures_isolated_v2(
    pool: &PgPool,
    limit: usize,
) -> Result<u64, String> {
    let limit = i64::try_from(limit).map_err(|_| "apply limit is too large".to_string())?;
    let capture_ids = sqlx::query_scalar::query_scalar::<_, String>(
        "select capture.capture_id
           from public.trnm_online_settlement_captures capture
          where capture.state = 'active'
            and not public.trnm_online_settlement_scope_quarantined_v1(
                'capture', capture.capture_id, 'apply'
            )
            and exists (
                select 1 from public.trnm_online_settlement_jobs job
                 where job.capture_id = capture.capture_id
            )
            and not exists (
                select 1 from public.trnm_online_settlement_jobs job
                 where job.capture_id = capture.capture_id
                   and (job.state <> 'succeeded' or job.campaign_applied_at is not null)
            )
          order by capture.created_at, capture.capture_id
          limit $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|error| format!("scan ready settlement captures: {error}"))?;

    let mut progress = 0_u64;
    for capture_id in capture_ids {
        match apply_capture(pool, &capture_id).await {
            Ok(ApplyCaptureResult::NotReady) => {}
            Ok(ApplyCaptureResult::Applied { finalized }) => {
                progress = progress.saturating_add(1);
                tracing::info!(%capture_id, finalized, "settlement capture applied");
            }
            Ok(ApplyCaptureResult::Stale(reason)) => {
                mark_capture_state(pool, &capture_id, "stale", &reason).await?;
                progress = progress.saturating_add(1);
                tracing::warn!(%capture_id, %reason, "settlement capture became stale");
            }
            Ok(ApplyCaptureResult::DeadLetter(reason)) => {
                mark_capture_state(pool, &capture_id, "dead_letter", &reason).await?;
                progress = progress.saturating_add(1);
                tracing::error!(%capture_id, %reason, "settlement capture dead-lettered");
            }
            Err(error) => {
                record_quarantine_v2(
                    pool,
                    "capture",
                    &capture_id,
                    "apply",
                    &error,
                    DEFAULT_QUARANTINE_RETRY_SECONDS_V2,
                )
                .await?;
                progress = progress.saturating_add(1);
                tracing::error!(%capture_id, %error, "settlement apply candidate quarantined");
            }
        }
    }
    Ok(progress)
}

async fn record_quarantine_v2(
    pool: &PgPool,
    scope_kind: &str,
    scope_id: &str,
    phase: &str,
    error: &str,
    retry_after_seconds: i64,
) -> Result<(), String> {
    sqlx::query::query(
        "select public.trnm_online_record_settlement_quarantine_v1(
            $1, $2, $3, $4, $5
         )",
    )
    .bind(scope_kind)
    .bind(scope_id)
    .bind(phase)
    .bind(error)
    .bind(retry_after_seconds)
    .execute(pool)
    .await
    .map_err(|db_error| {
        format!(
            "record settlement quarantine {scope_kind}/{scope_id}/{phase}: {db_error}"
        )
    })?;
    Ok(())
}
