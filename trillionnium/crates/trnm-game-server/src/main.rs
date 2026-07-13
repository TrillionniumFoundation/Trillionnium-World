use std::{net::SocketAddr, path::PathBuf};
use trnm_game_server::{
    build_router, resolve_authority_tick_interval, run_authority_loop,
    validate_operations_bind_addr, AppState, AppStateConfig,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trnm_game_server=info,tower_http=info".into()),
        )
        .init();
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

    let state = AppState::connect(AppStateConfig {
        database_url,
        cex_base_url,
        game_authority_token,
        entitlement_signer_url,
        entitlement_signer_token,
        asset_root,
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
    .unwrap_or_else(|error| panic!("Online Authority startup failed closed: {error}"));
    let loop_state = state.clone();
    tokio::spawn(async move {
        run_authority_loop(loop_state, tick_interval).await;
    });
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap_or_else(|error| panic!("bind {bind_addr}: {error}"));
    tracing::info!(%bind_addr, "TRNM Online Production v2 ready");
    let shutdown_state = state.clone();
    axum::serve(listener, build_router(state))
        .with_graceful_shutdown(async move {
            shutdown_signal().await;
            if let Err(error) = shutdown_state.graceful_shutdown().await {
                tracing::error!(%error, "Online Authority graceful shutdown failed closed");
            }
        })
        .await
        .expect("serve Online Authority");
}

async fn shutdown_signal() {
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

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
