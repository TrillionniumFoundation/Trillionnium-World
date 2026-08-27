#![allow(dead_code)]

#[path = "../cex.rs"]
mod cex;
#[path = "../settlement_worker.rs"]
mod settlement_worker;
#[path = "../signer_protocol.rs"]
mod signer_protocol;

use settlement_worker::WorkerConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trnm_settlement_worker=info".into()),
        )
        .init();

    let result = match WorkerConfig::from_env() {
        Ok(config) => settlement_worker::run(config).await,
        Err(error) => Err(format!("settlement worker configuration: {error}")),
    };
    if let Err(error) = result {
        tracing::error!(%error, "TRNM settlement worker failed closed");
        std::process::exit(1);
    }
}
