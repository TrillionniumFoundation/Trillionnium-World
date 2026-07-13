use std::{net::SocketAddr, path::PathBuf, time::Duration};
use trnm_game_server::{
    build_router, run_authority_loop, validate_operations_bind_addr, AppState, AppStateConfig,
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
    let entitlement_signing_seed_base64 =
        std::env::var("TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64")
            .expect("TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64 is required");
    let entitlement_key_id = std::env::var("TRNM_ENTITLEMENT_ED25519_KEY_ID")
        .expect("TRNM_ENTITLEMENT_ED25519_KEY_ID is required");
    let moderator_token =
        std::env::var("TRNM_MODERATOR_TOKEN").expect("TRNM_MODERATOR_TOKEN is required");
    let instance_id =
        std::env::var("TRNM_FLEET_INSTANCE_ID").expect("TRNM_FLEET_INSTANCE_ID is required");
    let region = std::env::var("TRNM_FLEET_REGION").expect("TRNM_FLEET_REGION is required");
    let public_endpoint = std::env::var("TRNM_FLEET_PUBLIC_ENDPOINT")
        .expect("TRNM_FLEET_PUBLIC_ENDPOINT is required");
    let capacity = std::env::var("TRNM_FLEET_CAPACITY")
        .expect("TRNM_FLEET_CAPACITY is required")
        .parse::<i32>()
        .expect("TRNM_FLEET_CAPACITY must be an integer");
    let asset_root = std::env::var_os("TRNM_ASSET_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets"));
    let bind_addr = std::env::var("TRNM_GAME_SERVER_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:7005".to_string())
        .parse::<SocketAddr>()
        .expect("TRNM_GAME_SERVER_BIND_ADDR must be a socket address");
    let tick_ms = std::env::var("TRNM_GAME_SERVER_TICK_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(50)
        .clamp(5, 1_000);
    validate_operations_bind_addr(bind_addr)
        .unwrap_or_else(|error| panic!("Online Operations public bind failed closed: {error}"));

    let state = AppState::connect(AppStateConfig {
        database_url,
        cex_base_url,
        game_authority_token,
        entitlement_signing_seed_base64,
        entitlement_key_id,
        asset_root,
        moderator_token,
        instance_id,
        region,
        public_endpoint,
        capacity,
    })
    .await
    .unwrap_or_else(|error| panic!("Online Authority startup failed closed: {error}"));
    let loop_state = state.clone();
    tokio::spawn(async move {
        run_authority_loop(loop_state, Duration::from_millis(tick_ms)).await;
    });
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap_or_else(|error| panic!("bind {bind_addr}: {error}"));
    tracing::info!(%bind_addr, "TRNM Online Operations v2 ready");
    axum::serve(listener, build_router(state))
        .await
        .expect("serve Online Authority");
}
