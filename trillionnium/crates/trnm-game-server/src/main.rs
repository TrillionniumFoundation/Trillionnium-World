use std::{ffi::OsString, net::SocketAddr, path::PathBuf};
use trnm_game_server::{
    build_router, resolve_authority_tick_interval, run_authority_loop, run_fail_close_maintenance,
    validate_operations_bind_addr, AppState, AppStateConfig, FailCloseMaintenanceConfig,
};
use uuid::Uuid;

const JOURNAL_FATAL_EXIT_CODE: i32 = 70;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ShutdownCause {
    Operator,
    JournalFatal,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trnm_game_server=info,tower_http=info".into()),
        )
        .init();
    match parse_maintenance_args(std::env::args().skip(1)) {
        Ok(Some((match_id, adopt_legacy_pre_v13))) => {
            let required = |name: &str| {
                std::env::var(name).map_err(|_| format!("{name} is required for maintenance"))
            };
            let result = async {
                run_fail_close_maintenance(FailCloseMaintenanceConfig {
                    database_url: required("DATABASE_URL")?,
                    published_tick_journal_dir: canonical_maintenance_journal_dir(
                        std::env::var_os("TRNM_PUBLISHED_TICK_JOURNAL_DIR"),
                    )?,
                    instance_id: required("TRNM_FLEET_INSTANCE_ID")?,
                    physical_host_id: required("TRNM_FLEET_PHYSICAL_HOST_ID")?,
                    match_id,
                    failure_reason: required("TRNM_MAINTENANCE_FAILURE_REASON")?,
                    adopt_legacy_pre_v13,
                })
                .await
            }
            .await;
            match result {
                Ok(report) => {
                    println!(
                        "{}",
                        serde_json::to_string(&report).expect("serialize maintenance report")
                    );
                    return;
                }
                Err(error) => {
                    tracing::error!(%error, %match_id, "Online Authority maintenance failed closed");
                    std::process::exit(JOURNAL_FATAL_EXIT_CODE);
                }
            }
        }
        Ok(None) => {}
        Err(error) => {
            tracing::error!(%error, "invalid Online Authority maintenance command");
            std::process::exit(64);
        }
    }
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is required; Online Authority never falls back to memory");
    let cex_base_url = std::env::var("TRNM_CEX_LEDGER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:7002".to_string());
    let game_authority_token =
        std::env::var("TRNM_GAME_AUTHORITY_TOKEN").expect("TRNM_GAME_AUTHORITY_TOKEN is required");
    let entitlement_signer_url = std::env::var("TRNM_ENTITLEMENT_SIGNER_URL")
        .expect("TRNM_ENTITLEMENT_SIGNER_URL is required");
    let entitlement_signer_token = std::env::var("TRNM_ENTITLEMENT_SIGNER_TOKEN")
        .expect("TRNM_ENTITLEMENT_SIGNER_TOKEN is required");
    let moderator_token =
        std::env::var("TRNM_MODERATOR_TOKEN").expect("TRNM_MODERATOR_TOKEN is required");
    let instance_id =
        std::env::var("TRNM_FLEET_INSTANCE_ID").expect("TRNM_FLEET_INSTANCE_ID is required");
    let region = std::env::var("TRNM_FLEET_REGION").expect("TRNM_FLEET_REGION is required");
    let public_endpoint = std::env::var("TRNM_FLEET_PUBLIC_ENDPOINT")
        .expect("TRNM_FLEET_PUBLIC_ENDPOINT is required");
    let physical_host_id = std::env::var("TRNM_FLEET_PHYSICAL_HOST_ID")
        .expect("TRNM_FLEET_PHYSICAL_HOST_ID is required");
    let capacity = std::env::var("TRNM_FLEET_CAPACITY")
        .expect("TRNM_FLEET_CAPACITY is required")
        .parse::<i32>()
        .expect("TRNM_FLEET_CAPACITY must be an integer");
    let rate_limit_per_minute = std::env::var("TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE")
        .unwrap_or_else(|_| "600".to_string())
        .parse::<u32>()
        .expect("TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE must be an integer");
    let request_body_limit_bytes = std::env::var("TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES")
        .unwrap_or_else(|_| "262144".to_string())
        .parse::<u32>()
        .expect("TRNM_PRODUCTION_REQUEST_BODY_LIMIT_BYTES must be an integer");
    let asset_root = std::env::var_os("TRNM_ASSET_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets"));
    let published_tick_journal_dir = std::env::var_os("TRNM_PUBLISHED_TICK_JOURNAL_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("run/trnm-game-server/published-ticks"));
    let bind_addr = std::env::var("TRNM_GAME_SERVER_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:7005".to_string())
        .parse::<SocketAddr>()
        .expect("TRNM_GAME_SERVER_BIND_ADDR must be a socket address");
    let requested_tick_ms = std::env::var("TRNM_GAME_SERVER_TICK_MS").ok().map(|value| {
        value
            .parse::<u64>()
            .expect("TRNM_GAME_SERVER_TICK_MS must be an integer")
    });
    let allow_accelerated_test_clock =
        std::env::var("TRNM_ALLOW_ACCELERATED_TEST_CLOCK").is_ok_and(|value| value == "1");
    validate_operations_bind_addr(bind_addr)
        .unwrap_or_else(|error| panic!("Online Operations public bind failed closed: {error}"));
    let tick_interval = resolve_authority_tick_interval(
        requested_tick_ms,
        allow_accelerated_test_clock && bind_addr.ip().is_loopback(),
    )
    .unwrap_or_else(|error| panic!("Online Authority clock failed closed: {error}"));
    // Bind before claiming a new fleet epoch. A local port conflict must not
    // fence a healthy authority process and then fail startup itself.
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap_or_else(|error| panic!("bind {bind_addr}: {error}"));

    let state = match AppState::connect(AppStateConfig {
        database_url,
        cex_base_url,
        game_authority_token,
        entitlement_signer_url,
        entitlement_signer_token,
        asset_root,
        published_tick_journal_dir,
        moderator_token,
        instance_id,
        region,
        public_endpoint,
        physical_host_id,
        capacity,
        rate_limit_per_minute,
        request_body_limit_bytes,
        tick_interval,
        accelerated_test_clock: allow_accelerated_test_clock,
    })
    .await
    {
        Ok(state) => state,
        Err(error) => {
            // A startup journal fsync may be stuck in spawn_blocking. Exiting
            // directly avoids Tokio runtime teardown waiting forever for that
            // uninterruptible operation; the supervisor restarts from the
            // durable high-water evidence.
            tracing::error!(%error, "Online Authority startup failed closed");
            std::process::exit(JOURNAL_FATAL_EXIT_CODE);
        }
    };
    let mut journal_fatal = state.journal_fatal_shutdown();
    let loop_state = state.clone();
    tokio::spawn(async move {
        run_authority_loop(loop_state, tick_interval).await;
    });
    tracing::info!(%bind_addr, "TRNM Online Authority v3 / Production v2 ready");
    let (http_shutdown_tx, http_shutdown_rx) = tokio::sync::oneshot::channel();
    let server_state = state.clone();
    let mut server = tokio::spawn(async move {
        axum::serve(listener, build_router(server_state))
            .with_graceful_shutdown(async move {
                let _ = http_shutdown_rx.await;
            })
            .await
    });
    let shutdown_cause = shutdown_signal(&mut journal_fatal).await;
    // First make every command path return a recoverable 503, then trigger
    // Axum's accept/connection drain, and only then ask match actors to flush.
    // Requests that were already accepted during SIGTERM cannot dead-letter a
    // real client intent as a non-recoverable conflict.
    state.begin_draining().await;
    let _ = http_shutdown_tx.send(());
    tokio::task::yield_now().await;
    let mut service_failed = false;
    if let Err(error) = state.graceful_shutdown().await {
        tracing::error!(%error, "Online Authority graceful shutdown failed closed");
        service_failed = true;
    }
    match tokio::time::timeout(std::time::Duration::from_secs(12), &mut server).await {
        Ok(Ok(Ok(()))) => {}
        Ok(Ok(Err(error))) => {
            tracing::error!(%error, "serve Online Authority failed");
            service_failed = true;
        }
        Ok(Err(error)) => {
            tracing::error!(%error, "Online Authority server task failed");
            service_failed = true;
        }
        Err(_) => {
            tracing::error!("HTTP graceful drain exceeded its hard timeout; cancelling server");
            server.abort();
            let _ = tokio::time::timeout(std::time::Duration::from_secs(1), server).await;
            service_failed = true;
        }
    }
    let journal_failed_closed = *journal_fatal.borrow();
    if let Some(exit_code) =
        shutdown_exit_code(shutdown_cause, journal_failed_closed, service_failed)
    {
        tracing::error!(
            exit_code,
            "Online Authority is exiting without Tokio runtime teardown"
        );
        std::process::exit(exit_code);
    }
}

fn canonical_maintenance_journal_dir(value: Option<OsString>) -> Result<PathBuf, String> {
    let path = value
        .map(PathBuf::from)
        .ok_or_else(|| "TRNM_PUBLISHED_TICK_JOURNAL_DIR is required for maintenance".to_string())?;
    if !path.is_absolute() {
        return Err(
            "TRNM_PUBLISHED_TICK_JOURNAL_DIR must be a canonical absolute path for maintenance"
                .to_string(),
        );
    }
    let canonical = std::fs::canonicalize(&path).map_err(|error| {
        format!(
            "canonicalize maintenance journal directory {}: {error}",
            path.display()
        )
    })?;
    if canonical != path || !canonical.is_dir() {
        return Err(
            "TRNM_PUBLISHED_TICK_JOURNAL_DIR must name an existing canonical directory".to_string(),
        );
    }
    Ok(canonical)
}

fn parse_maintenance_args<I>(args: I) -> Result<Option<(Uuid, bool)>, String>
where
    I: IntoIterator<Item = String>,
{
    let args = args.into_iter().collect::<Vec<_>>();
    if args.is_empty() {
        return Ok(None);
    }
    if args.first().map(String::as_str) != Some("--maintenance-fail-close") {
        return Err("unsupported command-line arguments".to_string());
    }
    if !matches!(args.len(), 2 | 3) || (args.len() == 3 && args[2] != "--adopt-legacy-pre-v13") {
        return Err(
            "usage: trnm-game-server --maintenance-fail-close <uuid> [--adopt-legacy-pre-v13]"
                .to_string(),
        );
    }
    let match_id = Uuid::parse_str(&args[1])
        .map_err(|_| "maintenance selector must be one exact match UUID".to_string())?;
    if match_id.is_nil() {
        return Err("maintenance selector must not be the nil UUID".to_string());
    }
    Ok(Some((match_id, args.len() == 3)))
}

fn shutdown_exit_code(
    cause: ShutdownCause,
    journal_failed_closed: bool,
    service_failed: bool,
) -> Option<i32> {
    if cause == ShutdownCause::JournalFatal || journal_failed_closed {
        Some(JOURNAL_FATAL_EXIT_CODE)
    } else if service_failed {
        Some(1)
    } else {
        None
    }
}

async fn shutdown_signal(journal_fatal: &mut tokio::sync::watch::Receiver<bool>) -> ShutdownCause {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install Ctrl-C shutdown handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM shutdown handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let journal_failed = async {
        if !*journal_fatal.borrow() {
            let _ = journal_fatal.changed().await;
        }
    };

    tokio::select! {
        _ = ctrl_c => ShutdownCause::Operator,
        _ = terminate => ShutdownCause::Operator,
        _ = journal_failed => ShutdownCause::JournalFatal,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_maintenance_journal_dir, parse_maintenance_args, shutdown_exit_code,
        ShutdownCause, JOURNAL_FATAL_EXIT_CODE,
    };

    #[test]
    fn journal_poison_always_selects_direct_nonzero_exit() {
        assert_eq!(
            shutdown_exit_code(ShutdownCause::JournalFatal, false, false),
            Some(JOURNAL_FATAL_EXIT_CODE)
        );
        assert_eq!(
            shutdown_exit_code(ShutdownCause::Operator, true, false),
            Some(JOURNAL_FATAL_EXIT_CODE)
        );
        assert_eq!(
            shutdown_exit_code(ShutdownCause::Operator, false, false),
            None
        );
        assert_eq!(
            shutdown_exit_code(ShutdownCause::Operator, false, true),
            Some(1)
        );
    }

    #[test]
    fn maintenance_cli_requires_exact_uuid_and_explicit_legacy_flag() {
        let match_id = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
        assert_eq!(parse_maintenance_args(Vec::<String>::new()), Ok(None));
        assert_eq!(
            parse_maintenance_args(vec![
                "--maintenance-fail-close".to_string(),
                match_id.to_string(),
            ]),
            Ok(Some((uuid::Uuid::parse_str(match_id).unwrap(), false)))
        );
        assert_eq!(
            parse_maintenance_args(vec![
                "--maintenance-fail-close".to_string(),
                match_id.to_string(),
                "--adopt-legacy-pre-v13".to_string(),
            ]),
            Ok(Some((uuid::Uuid::parse_str(match_id).unwrap(), true)))
        );
        for invalid in [
            vec!["--maintenance-fail-close".to_string()],
            vec![
                "--maintenance-fail-close".to_string(),
                "not-a-uuid".to_string(),
            ],
            vec!["--unknown".to_string()],
        ] {
            assert!(parse_maintenance_args(invalid).is_err());
        }
    }

    #[test]
    fn maintenance_journal_path_has_no_relative_or_default_fallback() {
        assert!(canonical_maintenance_journal_dir(None).is_err());
        assert!(canonical_maintenance_journal_dir(Some("relative/journal".into())).is_err());
    }
}
