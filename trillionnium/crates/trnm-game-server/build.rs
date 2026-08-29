use std::{
    env, fs,
    path::{Path, PathBuf},
};

const LIB_HEADER: &str = r#"#![recursion_limit = "512"]

mod cex;
mod map;
mod operations_v1;
mod product_v2;
mod production_v1;
mod published_tick_journal;
pub mod signer_protocol;
mod stream;

"#;

fn fail(message: impl AsRef<str>) -> ! {
    panic!(
        "WORLD-P0 source transform failed closed: {}",
        message.as_ref()
    );
}

fn replace_once(source: &mut String, old: &str, new: &str, label: &str) {
    let count = source.matches(old).count();
    match count {
        1 => *source = source.replacen(old, new, 1),
        0 if source.contains(new) => {}
        _ => fail(format!(
            "{label}: expected one reviewed source shape, found {count}"
        )),
    }
}

fn replace_range_once(
    source: &mut String,
    start_marker: &str,
    end_marker: &str,
    replacement: &str,
    label: &str,
) {
    let Some(start) = source.find(start_marker) else {
        if source.contains(replacement) {
            return;
        }
        fail(format!("{label}: start marker is missing"));
    };
    let search_from = start + start_marker.len();
    let Some(relative_end) = source[search_from..].find(end_marker) else {
        fail(format!("{label}: end marker is missing"));
    };
    let end = search_from + relative_end;
    source.replace_range(start..end, replacement);
}

fn rewrite_migration_includes(source: &str) -> String {
    let needle = "include_str!(\"../migrations/";
    source
        .lines()
        .map(|line| {
            let Some(start) = line.find(needle) else {
                return line.to_string();
            };
            let path_start = start + needle.len();
            let Some(end_offset) = line[path_start..].find("\");") else {
                fail(format!("malformed migration include: {line}"));
            };
            let path_end = path_start + end_offset;
            let relative = &line[path_start..path_end];
            format!(
                "{}include_str!(concat!(::std::env!(\"CARGO_MANIFEST_DIR\"), \"/migrations/{}\"));{}",
                &line[..start],
                relative,
                &line[path_end + 3..]
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn generate_game_server(out_dir: &Path) {
    let template_path = PathBuf::from("src/lib.rs.in");
    let mut source = fs::read_to_string(&template_path)
        .unwrap_or_else(|error| fail(format!("read {}: {error}", template_path.display())));

    replace_once(
        &mut source,
        "const MIGRATION_V15: &str = include_str!(\"../migrations/0015_online_realtime_hot_path_v1.sql\");",
        r#"const MIGRATION_V15: &str = include_str!("../migrations/0015_online_realtime_hot_path_v1.sql");
const MIGRATION_V16: &str = include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const MIGRATION_V17: &str =
    include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_V18: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");
const MIGRATION_V19: &str =
    include_str!("../migrations/0019_online_settlement_runtime_hardening_v1.sql");"#,
        "game-server migration constants",
    );
    replace_once(
        &mut source,
        "        (15, \"0015_online_realtime_hot_path_v1\", MIGRATION_V15),\n",
        r#"        (15, "0015_online_realtime_hot_path_v1", MIGRATION_V15),
        (16, "0016_online_settlement_outbox_v1", MIGRATION_V16),
        (
            17,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
        (
            19,
            "0019_online_settlement_runtime_hardening_v1",
            MIGRATION_V19,
        ),
"#,
        "game-server migration ledger",
    );

    let legacy_loop = r#"    let settlement_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = settle_pending_matches(&settlement_state, 2).await {
                tracing::error!(%error, "online authority settlement remains pending");
            }
        }
    });

"#;
    if source.contains(legacy_loop) {
        source = source.replacen(legacy_loop, "", 1);
    } else if source.contains("settle_pending_matches(&settlement_state") {
        fail("legacy settlement loop drifted from the reviewed source shape");
    }

    let legacy_signature =
        "pub async fn settle_pending_matches(state: &AppState, limit: i64) -> Result<u64, String> {";
    let fail_closed_signature =
        "pub async fn settle_pending_matches(_state: &AppState, _limit: i64) -> Result<u64, String> {";
    if let Some(start) = source.find(legacy_signature) {
        let end_marker = "\nasync fn persist_campaign(\n";
        let Some(relative_end) = source[start..].find(end_marker) else {
            fail("cannot find reviewed end of legacy settlement function");
        };
        let end = start + relative_end;
        let replacement = r#"/// Compatibility API retained only to fail closed for downstream callers.
///
/// Terminal economic settlement is owned by the independently deployed
/// `trnm-settlement-worker`. The game-server process must never execute signer
/// or CEX I/O, mutate campaign economic queues, or advance the terminal
/// settlement marker itself.
pub async fn settle_pending_matches(_state: &AppState, _limit: i64) -> Result<u64, String> {
    Err(
        "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited"
            .to_string(),
    )
}
"#;
        source.replace_range(start..end, replacement);
    } else if !source.contains(fail_closed_signature) {
        fail("legacy settlement function is neither reviewed legacy nor fail-closed form");
    }

    if source.contains("reconcile_economy(&state.cex") {
        fail("game-server still contains synchronous CEX reconciliation");
    }
    if source.contains("settle_pending_matches(&settlement_state") {
        fail("game-server still schedules in-process settlement");
    }

    let Some(body) = source.strip_prefix(LIB_HEADER) else {
        fail("game-server crate header drifted from the reviewed template");
    };
    let generated = rewrite_migration_includes(body);
    fs::write(out_dir.join("trnm_game_server_lib_generated.rs"), generated)
        .unwrap_or_else(|error| fail(format!("write generated game server: {error}")));
}

fn generate_settlement_worker(out_dir: &Path) {
    let template_path = PathBuf::from("src/settlement_worker.rs.in");
    let mut source = fs::read_to_string(&template_path)
        .unwrap_or_else(|error| fail(format!("read {}: {error}", template_path.display())));

    replace_once(
        &mut source,
        "use serde::Serialize;",
        "use futures_util::FutureExt;\nuse serde::Serialize;",
        "settlement-worker FutureExt import",
    );
    replace_once(
        &mut source,
        "const MIGRATION_V17: &str = include_str!(\"../migrations/0017_online_settlement_worker_runtime_v1.sql\");",
        r#"const MIGRATION_V17: &str = include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_V18: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");
const MIGRATION_V19: &str =
    include_str!("../migrations/0019_online_settlement_runtime_hardening_v1.sql");"#,
        "settlement-worker migration constants",
    );
    replace_once(
        &mut source,
        r#"        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
"#,
        r#"        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18_i32,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
        (
            19_i32,
            "0019_online_settlement_runtime_hardening_v1",
            MIGRATION_V19,
        ),
"#,
        "settlement-worker migration ledger",
    );

    replace_range_once(
        &mut source,
        "pub async fn run(config: WorkerConfig) -> Result<(), String> {",
        "\nasync fn apply_worker_migrations(",
        r###"#[cfg(unix)]
async fn shutdown_signal() -> Result<&'static str, String> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(|error| format!("install settlement SIGTERM handler: {error}"))?;
    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            result.map_err(|error| format!("install settlement SIGINT handler: {error}"))?;
            Ok("SIGINT")
        }
        signal = terminate.recv() => {
            signal.ok_or_else(|| "settlement SIGTERM stream closed".to_string())?;
            Ok("SIGTERM")
        }
    }
}

#[cfg(not(unix))]
async fn shutdown_signal() -> Result<&'static str, String> {
    tokio::signal::ctrl_c()
        .await
        .map_err(|error| format!("install settlement shutdown handler: {error}"))?;
    Ok("CTRL_C")
}

fn finish_shutdown(signal: Result<&'static str, String>) -> Result<(), String> {
    let signal = signal?;
    tracing::info!(signal, "settlement worker stopped after bounded in-flight drain");
    Ok(())
}

pub async fn run(config: WorkerConfig) -> Result<(), String> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(config.pool_max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.database_url)
        .await
        .map_err(|error| format!("connect settlement worker PostgreSQL pool: {error}"))?;
    apply_worker_migrations(&pool).await?;

    let cex = CexClient::new(
        config.cex_url.clone(),
        config.game_authority_token.clone(),
        config.signer_url.clone(),
        config.signer_token.clone(),
    )?;
    cex.readiness().await?;

    tracing::info!(
        worker_id = %config.worker_id,
        batch_size = config.batch_size,
        lease_milliseconds = config.lease_milliseconds,
        "transaction-free settlement worker is ready"
    );

    let mut shutdown = Box::pin(shutdown_signal());
    loop {
        if let Some(signal) = shutdown.as_mut().now_or_never() {
            return finish_shutdown(signal);
        }

        let mut work_count = 0_u64;
        match capture_pending_matches(&pool, config.batch_size).await {
            Ok(captured) => work_count = work_count.saturating_add(captured),
            Err(error) => tracing::error!(%error, "settlement capture scan failed"),
        }

        if let Some(signal) = shutdown.as_mut().now_or_never() {
            return finish_shutdown(signal);
        }

        let mut claimed_jobs = Vec::with_capacity(config.batch_size);
        for _ in 0..config.batch_size {
            match claim_settlement_job(&pool, &config.worker_id, config.lease_milliseconds).await {
                Ok(Some(job)) => {
                    work_count = work_count.saturating_add(1);
                    claimed_jobs.push(job);
                }
                Ok(None) => break,
                Err(error) => {
                    tracing::error!(%error, "settlement claim was isolated; unrelated jobs remain eligible");
                }
            }
        }

        let mut tasks = tokio::task::JoinSet::new();
        for job in claimed_jobs {
            let task_pool = pool.clone();
            let task_cex = cex.clone();
            let job_id = job.job_id.clone();
            tasks.spawn(async move {
                let result = process_claimed_job(&task_pool, &task_cex, job).await;
                (job_id, result)
            });
        }
        while let Some(joined) = tasks.join_next().await {
            match joined {
                Ok((job_id, Ok(()))) => {
                    tracing::debug!(%job_id, "settlement remote phase completed");
                }
                Ok((job_id, Err(error))) => {
                    tracing::error!(%job_id, %error, "claimed settlement job failed in isolation");
                }
                Err(error) => {
                    tracing::error!(%error, "settlement task join failed; durable lease recovery remains active");
                }
            }
        }

        if let Some(signal) = shutdown.as_mut().now_or_never() {
            return finish_shutdown(signal);
        }

        match apply_ready_captures(&pool, config.batch_size).await {
            Ok(applied) => work_count = work_count.saturating_add(applied),
            Err(error) => tracing::error!(%error, "settlement apply scan failed"),
        }

        if work_count == 0 {
            tokio::select! {
                signal = &mut shutdown => return finish_shutdown(signal),
                _ = sleep(config.poll_interval) => {}
            }
        }
    }
}
"###,
        "settlement-worker lifecycle and bounded concurrency",
    );

    replace_range_once(
        &mut source,
        "pub async fn capture_pending_matches(pool: &PgPool, limit: usize) -> Result<u64, String> {",
        "\nasync fn capture_match(",
        r###"async fn record_runtime_failure(
    pool: &PgPool,
    subject_kind: &str,
    subject_id: &str,
    error: &str,
) -> Result<bool, String> {
    if !matches!(subject_kind, "capture" | "apply") || subject_id.trim().is_empty() {
        return Err("invalid settlement runtime failure subject".to_string());
    }
    sqlx::query_scalar::query_scalar::<_, bool>(
        "insert into public.trnm_online_settlement_runtime_failures as failure (
            subject_kind, subject_id, consecutive_failures, last_error,
            next_attempt_at, updated_at
         ) values ($1, $2, 1, left($3, 1024),
                   clock_timestamp() + interval '5 seconds', clock_timestamp())
         on conflict (subject_kind, subject_id) do update
            set consecutive_failures = least(failure.consecutive_failures + 1, 1000000),
                last_error = excluded.last_error,
                next_attempt_at = clock_timestamp() + interval '30 seconds',
                quarantined_at = case
                    when failure.consecutive_failures + 1 >= 8
                    then coalesce(failure.quarantined_at, clock_timestamp())
                    else failure.quarantined_at
                end,
                updated_at = clock_timestamp()
         returning quarantined_at is not null",
    )
    .bind(subject_kind)
    .bind(subject_id)
    .bind(error)
    .fetch_one(pool)
    .await
    .map_err(|db_error| format!("record settlement {subject_kind} failure: {db_error}"))
}

async fn clear_runtime_failure(
    pool: &PgPool,
    subject_kind: &str,
    subject_id: &str,
) -> Result<(), String> {
    sqlx::query::query(
        "delete from public.trnm_online_settlement_runtime_failures
          where subject_kind = $1 and subject_id = $2 and quarantined_at is null",
    )
    .bind(subject_kind)
    .bind(subject_id)
    .execute(pool)
    .await
    .map_err(|error| format!("clear settlement {subject_kind} failure: {error}"))?;
    Ok(())
}

pub async fn capture_pending_matches(pool: &PgPool, limit: usize) -> Result<u64, String> {
    let limit = i64::try_from(limit).map_err(|_| "capture limit is too large".to_string())?;
    let match_ids = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_row.match_id
           from public.trnm_online_matches match_row
          where public.trnm_online_settlement_match_ready_v1(match_row.match_id)
            and not exists (
                select 1
                  from public.trnm_online_settlement_captures capture
                 where capture.match_id = match_row.match_id
                   and capture.state in ('active', 'dead_letter')
            )
            and not exists (
                select 1
                  from public.trnm_online_settlement_runtime_failures failure
                 where failure.subject_kind = 'capture'
                   and failure.subject_id = match_row.match_id::text
                   and (
                       failure.quarantined_at is not null
                       or failure.next_attempt_at > clock_timestamp()
                   )
            )
          order by match_row.updated_at, match_row.match_id
          limit $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|error| format!("scan settlement capture candidates: {error}"))?;

    let mut captured = 0_u64;
    for match_id in match_ids {
        match capture_match(pool, match_id).await {
            Ok(count) => {
                if count > 0 {
                    clear_runtime_failure(pool, "capture", &match_id.to_string()).await?;
                }
                captured = captured.saturating_add(count);
            }
            Err(error) => {
                let quarantined = record_runtime_failure(
                    pool,
                    "capture",
                    &match_id.to_string(),
                    &error,
                )
                .await
                .unwrap_or_else(|record_error| {
                    tracing::error!(%record_error, %match_id, "failed to persist capture isolation evidence");
                    false
                });
                tracing::error!(%match_id, %error, quarantined, "settlement capture candidate failed in isolation");
            }
        }
    }
    Ok(captured)
}
"###,
        "settlement capture poison isolation",
    );

    replace_range_once(
        &mut source,
        "async fn claim_settlement_job(\n",
        "\nfn settlement_job_from_row(",
        r###"async fn claim_settlement_job(
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
        Err(binding_error) => {
            let job_id: String = row
                .try_get("job_id")
                .map_err(|error| format!("read malformed claimed job id: {error}"))?;
            let capture_id: Option<String> = row
                .try_get("capture_id")
                .map_err(|error| format!("read malformed claimed capture id: {error}"))?;
            let lease_generation: i64 = row
                .try_get("lease_generation")
                .map_err(|error| format!("read malformed claimed lease generation: {error}"))?;
            let dead = sqlx::query_scalar::query_scalar::<_, bool>(
                "select public.trnm_online_dead_letter_settlement_job_v1($1, $2, $3, $4)",
            )
            .bind(&job_id)
            .bind(owner)
            .bind(lease_generation)
            .bind(format!("durable claimed-job binding failed: {binding_error}"))
            .fetch_one(pool)
            .await
            .map_err(|error| format!("quarantine malformed claimed job {job_id}: {error}"))?;
            if dead {
                if let Some(capture_id) = capture_id {
                    mark_capture_state(
                        pool,
                        &capture_id,
                        "dead_letter",
                        "durable claimed-job binding failed",
                    )
                    .await?;
                }
            }
            Err(format!(
                "claimed settlement job {job_id} failed durable binding and was quarantined: {binding_error}"
            ))
        }
    }
}
"###,
        "malformed claimed-job quarantine",
    );

    replace_range_once(
        &mut source,
        "async fn handle_external_failure(\n",
        "\nfn retry_delay_milliseconds(",
        r###"fn normalize_external_failure(error: ExternalSettlementError) -> ExternalSettlementError {
    match error {
        ExternalSettlementError::Permanent(message)
            if message.starts_with("decode isolated signer response:")
                || message.starts_with("decode isolated signer receipt lookup:")
                || message.starts_with("decode CEX receipt:")
                || message.starts_with("decode CEX receipt lookup:")
                || message.contains("(409 Conflict)")
                || message.contains("(409):") =>
        {
            ExternalSettlementError::Retryable(format!(
                "ambiguous remote outcome; exact receipt lookup is required before retry: {message}"
            ))
        }
        other => other,
    }
}

async fn handle_external_failure(
    pool: &PgPool,
    job: &SettlementJob,
    remote_attempt: i32,
    error: ExternalSettlementError,
) -> Result<(), String> {
    match normalize_external_failure(error) {
        ExternalSettlementError::Retryable(message) => {
            let delay = retry_delay_milliseconds(&job.job_id, remote_attempt);
            let state = sqlx::query_scalar::query_scalar::<_, Option<String>>(
                "select public.trnm_online_retry_settlement_job_v1($1, $2, $3, $4, $5)",
            )
            .bind(&job.job_id)
            .bind(&job.lease_owner)
            .bind(job.lease_generation)
            .bind(message)
            .bind(delay)
            .fetch_one(pool)
            .await
            .map_err(|error| format!("mark settlement retry: {error}"))?;
            if state.as_deref() == Some("dead_letter") {
                mark_capture_state(
                    pool,
                    &job.capture_id,
                    "dead_letter",
                    "settlement remote retry budget exhausted",
                )
                .await?;
            } else if state.as_deref() != Some("retryable") {
                return Err("settlement retry lost its lease fence".to_string());
            }
        }
        ExternalSettlementError::Permanent(message) => {
            let dead = sqlx::query_scalar::query_scalar::<_, bool>(
                "select public.trnm_online_dead_letter_settlement_job_v1($1, $2, $3, $4)",
            )
            .bind(&job.job_id)
            .bind(&job.lease_owner)
            .bind(job.lease_generation)
            .bind(&message)
            .fetch_one(pool)
            .await
            .map_err(|error| format!("dead-letter settlement job: {error}"))?;
            if !dead {
                return Err("permanent settlement failure lost its lease fence".to_string());
            }
            mark_capture_state(pool, &job.capture_id, "dead_letter", &message).await?;
        }
    }
    Ok(())
}
"###,
        "ambiguous remote outcome normalization",
    );

    replace_range_once(
        &mut source,
        "async fn apply_ready_captures(pool: &PgPool, limit: usize) -> Result<u64, String> {",
        "\nenum ApplyCaptureResult {",
        r###"async fn apply_ready_captures(pool: &PgPool, limit: usize) -> Result<u64, String> {
    let limit = i64::try_from(limit).map_err(|_| "apply limit is too large".to_string())?;
    let capture_ids = sqlx::query_scalar::query_scalar::<_, String>(
        "select capture.capture_id
           from public.trnm_online_settlement_captures capture
          where capture.state = 'active'
            and exists (
                select 1 from public.trnm_online_settlement_jobs job
                 where job.capture_id = capture.capture_id
            )
            and not exists (
                select 1 from public.trnm_online_settlement_jobs job
                 where job.capture_id = capture.capture_id
                   and (job.state <> 'succeeded' or job.campaign_applied_at is not null)
            )
            and not exists (
                select 1
                  from public.trnm_online_settlement_runtime_failures failure
                 where failure.subject_kind = 'apply'
                   and failure.subject_id = capture.capture_id
                   and (
                       failure.quarantined_at is not null
                       or failure.next_attempt_at > clock_timestamp()
                   )
            )
          order by capture.created_at, capture.capture_id
          limit $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|error| format!("scan ready settlement captures: {error}"))?;

    let mut applied = 0_u64;
    for capture_id in capture_ids {
        match apply_capture(pool, &capture_id).await {
            Ok(ApplyCaptureResult::NotReady) => {}
            Ok(ApplyCaptureResult::Applied { finalized }) => {
                clear_runtime_failure(pool, "apply", &capture_id).await?;
                applied = applied.saturating_add(1);
                tracing::info!(%capture_id, finalized, "settlement capture applied");
            }
            Ok(ApplyCaptureResult::Stale(reason)) => {
                mark_capture_state(pool, &capture_id, "stale", &reason).await?;
                tracing::warn!(%capture_id, %reason, "settlement capture became stale");
            }
            Ok(ApplyCaptureResult::DeadLetter(reason)) => {
                mark_capture_state(pool, &capture_id, "dead_letter", &reason).await?;
                tracing::error!(%capture_id, %reason, "settlement capture dead-lettered");
            }
            Err(error) => {
                let quarantined = record_runtime_failure(pool, "apply", &capture_id, &error)
                    .await
                    .unwrap_or_else(|record_error| {
                        tracing::error!(%record_error, %capture_id, "failed to persist apply isolation evidence");
                        false
                    });
                tracing::error!(%capture_id, %error, quarantined, "settlement apply failed in isolation");
            }
        }
    }
    Ok(applied)
}
"###,
        "settlement apply poison isolation",
    );

    replace_once(
        &mut source,
        r#"    let jobs_by_campaign = jobs
        .iter()
        .map(|job| (job.campaign_id.as_str(), job))
        .collect::<BTreeMap<_, _>>();
"#,
        r#"    let jobs_by_campaign = jobs
        .iter()
        .map(|job| (job.campaign_id.as_str(), job))
        .collect::<BTreeMap<_, _>>();
    if jobs_by_campaign.len() != jobs.len() {
        transaction
            .rollback()
            .await
            .map_err(|error| format!("rollback duplicate campaign jobs: {error}"))?;
        return Ok(ApplyCaptureResult::DeadLetter(
            "capture contains multiple settlement jobs for one campaign".to_string(),
        ));
    }
"#,
        "duplicate campaign-job rejection",
    );

    replace_once(
        &mut source,
        "        campaign.campaign.revision = campaign.campaign.revision.saturating_add(1);",
        r#"        let Some(next_revision) = campaign.campaign.revision.checked_add(1) else {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback campaign revision overflow: {error}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "campaign {} revision exhausted",
                campaign.campaign_id
            )));
        };
        campaign.campaign.revision = next_revision;"#,
        "campaign revision overflow rejection",
    );

    replace_once(
        &mut source,
        "#[cfg(test)]\nmod tests {",
        r#"#[cfg(test)]
mod runtime_hardening_tests {
    use super::*;

    #[test]
    fn malformed_success_and_conflict_are_ambiguous_not_permanent() {
        for message in [
            "decode isolated signer response: truncated JSON",
            "decode isolated signer receipt lookup: truncated JSON",
            "decode CEX receipt: truncated JSON",
            "decode CEX receipt lookup: truncated JSON",
            "CEX intent rejected (409 Conflict): duplicate",
        ] {
            assert!(matches!(
                normalize_external_failure(ExternalSettlementError::Permanent(message.to_string())),
                ExternalSettlementError::Retryable(_)
            ));
        }
    }

    #[test]
    fn definite_validation_failure_remains_permanent() {
        assert!(matches!(
            normalize_external_failure(ExternalSettlementError::Permanent(
                "receipt/campaign binding mismatch".to_string()
            )),
            ExternalSettlementError::Permanent(_)
        ));
    }
}

#[cfg(test)]
mod tests {"#,
        "settlement runtime hardening tests",
    );

    let generated = rewrite_migration_includes(&source);
    fs::write(
        out_dir.join("trnm_settlement_worker_generated.rs"),
        generated,
    )
    .unwrap_or_else(|error| fail(format!("write generated settlement worker: {error}")));
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs.in");
    println!("cargo:rerun-if-changed=src/settlement_worker.rs.in");
    println!("cargo:rerun-if-changed=migrations");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"));
    generate_game_server(&out_dir);
    generate_settlement_worker(&out_dir);
}
