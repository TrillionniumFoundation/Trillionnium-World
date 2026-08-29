//! Final build-stage wrapper for the legacy generated-source compatibility enclave.
//!
//! `build_generated_base.rs` contains the reviewed migration/source transforms inherited
//! from the stacked settlement candidate. This wrapper calls that generator, then applies
//! only bounded lifecycle/deadline hardening to the generated settlement worker. The
//! generated-source architecture remains transitional and is tracked by WORLD-P0-009.

mod generated_base {
    include!("build_generated_base.rs");

    pub fn run() {
        main();
    }
}

use std::{env, fs, path::PathBuf};

fn fail(message: impl AsRef<str>) -> ! {
    panic!(
        "WORLD-P0 settlement deadline transform failed closed: {}",
        message.as_ref()
    );
}

fn replace_once(source: &mut String, old: &str, new: &str, label: &str) {
    let count = source.matches(old).count();
    match count {
        1 => *source = source.replacen(old, new, 1),
        0 if source.contains(new) => {}
        _ => fail(format!(
            "{label}: expected one reviewed generated-source shape, found {count}"
        )),
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build_generated_base.rs");
    generated_base::run();

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"));
    let worker_path = out_dir.join("trnm_settlement_worker_generated.rs");
    let mut source = fs::read_to_string(&worker_path)
        .unwrap_or_else(|error| fail(format!("read {}: {error}", worker_path.display())));

    replace_once(
        &mut source,
        "    let mut shutdown = Box::pin(shutdown_signal());\n    loop {",
        r#"    let mut shutdown = Box::pin(shutdown_signal());
    let operation_timeout_millis = u64::try_from(config.lease_milliseconds.min(30_000))
        .map_err(|_| "settlement operation timeout is outside u64 range".to_string())?;
    let operation_timeout = Duration::from_millis(operation_timeout_millis);
    loop {"#,
        "derive bounded operation deadline from the live lease",
    );

    replace_once(
        &mut source,
        r#"        match capture_pending_matches(&pool, config.batch_size).await {
            Ok(captured) => work_count = work_count.saturating_add(captured),
            Err(error) => tracing::error!(%error, "settlement capture scan failed"),
        }
"#,
        r#"        match tokio::time::timeout(
            operation_timeout,
            capture_pending_matches(&pool, config.batch_size),
        )
        .await
        {
            Ok(Ok(captured)) => work_count = work_count.saturating_add(captured),
            Ok(Err(error)) => tracing::error!(%error, "settlement capture scan failed"),
            Err(_) => tracing::error!(
                timeout_milliseconds = operation_timeout_millis,
                "settlement capture scan timed out"
            ),
        }
"#,
        "bound capture scan",
    );

    replace_once(
        &mut source,
        r#"        let mut claimed_jobs = Vec::with_capacity(config.batch_size);
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
"#,
        r#"        let mut claimed_jobs = Vec::with_capacity(config.batch_size);
        for _ in 0..config.batch_size {
            if let Some(signal) = shutdown.as_mut().now_or_never() {
                return finish_shutdown(signal);
            }
            match tokio::time::timeout(
                operation_timeout,
                claim_settlement_job(&pool, &config.worker_id, config.lease_milliseconds),
            )
            .await
            {
                Ok(Ok(Some(job))) => {
                    work_count = work_count.saturating_add(1);
                    claimed_jobs.push(job);
                }
                Ok(Ok(None)) => break,
                Ok(Err(error)) => {
                    tracing::error!(%error, "settlement claim was isolated; unrelated jobs remain eligible");
                }
                Err(_) => {
                    tracing::error!(
                        timeout_milliseconds = operation_timeout_millis,
                        "settlement claim timed out"
                    );
                }
            }
        }
"#,
        "bound claim loop and observe shutdown between claims",
    );

    replace_once(
        &mut source,
        r#"            tasks.spawn(async move {
                let result = process_claimed_job(&task_pool, &task_cex, job).await;
                (job_id, result)
            });
"#,
        r#"            let task_timeout = operation_timeout;
            tasks.spawn(async move {
                let result = match tokio::time::timeout(
                    task_timeout,
                    process_claimed_job(&task_pool, &task_cex, job),
                )
                .await
                {
                    Ok(result) => result,
                    Err(_) => Err(format!(
                        "settlement remote phase exceeded {} ms; durable lease recovery will retry",
                        task_timeout.as_millis()
                    )),
                };
                (job_id, result)
            });
"#,
        "bound each remote settlement task",
    );

    replace_once(
        &mut source,
        r#"        match apply_ready_captures(&pool, config.batch_size).await {
            Ok(applied) => work_count = work_count.saturating_add(applied),
            Err(error) => tracing::error!(%error, "settlement apply scan failed"),
        }
"#,
        r#"        match tokio::time::timeout(
            operation_timeout,
            apply_ready_captures(&pool, config.batch_size),
        )
        .await
        {
            Ok(Ok(applied)) => work_count = work_count.saturating_add(applied),
            Ok(Err(error)) => tracing::error!(%error, "settlement apply scan failed"),
            Err(_) => tracing::error!(
                timeout_milliseconds = operation_timeout_millis,
                "settlement apply scan timed out"
            ),
        }
"#,
        "bound apply scan",
    );

    for required in [
        "operation_timeout_millis",
        "settlement capture scan timed out",
        "settlement claim timed out",
        "settlement remote phase exceeded",
        "settlement apply scan timed out",
    ] {
        if !source.contains(required) {
            fail(format!("generated settlement worker is missing {required}"));
        }
    }

    fs::write(&worker_path, source)
        .unwrap_or_else(|error| fail(format!("write {}: {error}", worker_path.display())));
}
