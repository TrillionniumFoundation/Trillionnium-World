use axum::{
    extract::{DefaultBodyLimit, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::row::Row;
use sqlx_postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, path::Path, sync::Arc, time::Duration};
use trnm_economy_protocol::{
    ServerSignedValueEntitlementV2, ValueEntitlementSource,
    SERVER_SIGNED_VALUE_ENTITLEMENT_V2_CONTRACT,
};
use trnm_game_server::signer_protocol::{
    EntitlementSignRequest, EntitlementSignResponse, EntitlementSignerAttestationRequest,
    EntitlementSignerAttestationResponse, EntitlementSignerReadiness, ENTITLEMENT_SIGNER_CONTRACT,
    ENTITLEMENT_SIGNER_ISSUER, ENTITLEMENT_SIGNER_RECEIPT_PATH, SIGNER_AUTH_HEADER,
};

const SIGNER_DATABASE_MAX_CONNECTIONS: u32 = 4;
const SIGNER_MIGRATION: &str = r#"
create table if not exists trnm_entitlement_signing_receipts (
    request_id text primary key,
    request_hash text not null check (length(request_hash) = 64),
    signing_receipt_hash text not null check (length(signing_receipt_hash) = 64),
    key_id text not null,
    issuer text not null,
    signature text not null,
    entitlement_json jsonb not null,
    created_at timestamptz not null default now()
);
"#;

#[derive(Clone)]
struct SignerState {
    pool: PgPool,
    auth_token: Arc<String>,
    signing_key: Arc<SigningKey>,
    key_id: Arc<String>,
}

type ApiError = (StatusCode, Json<Value>);

fn error(status: StatusCode, message: impl Into<String>) -> ApiError {
    (status, Json(json!({"error": message.into()})))
}

fn require_auth(state: &SignerState, headers: &HeaderMap) -> Result<(), ApiError> {
    let supplied = headers
        .get(SIGNER_AUTH_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if supplied.is_empty() || supplied != state.auth_token.as_str() {
        return Err(error(
            StatusCode::UNAUTHORIZED,
            "isolated signer credential is required",
        ));
    }
    Ok(())
}

async fn health() -> &'static str {
    "trnm-entitlement-signer ok"
}

fn database_pool_is_operational(
    pool_size: u32,
    idle_connections: usize,
    max_connections: u32,
) -> bool {
    idle_connections > 0 || pool_size < max_connections
}

async fn readiness(State(state): State<SignerState>) -> impl IntoResponse {
    let postgres = sqlx::query_scalar::query_scalar::<_, i32>("select 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    let database_pool_idle_connections = state.pool.num_idle();
    let database_pool_size = state.pool.size();
    let database_pool_saturation_healthy = database_pool_is_operational(
        database_pool_size,
        database_pool_idle_connections,
        SIGNER_DATABASE_MAX_CONNECTIONS,
    );
    let ready = postgres && database_pool_saturation_healthy;
    (
        if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(EntitlementSignerReadiness {
            status: if ready { "ok" } else { "blocked" }.to_string(),
            contract_version: ENTITLEMENT_SIGNER_CONTRACT.to_string(),
            key_id: state.key_id.as_ref().clone(),
            issuer: ENTITLEMENT_SIGNER_ISSUER.to_string(),
            custody: "isolated_process_mode_600_seed_not_kms_hsm".to_string(),
            postgres_receipts: postgres,
            database_pool_saturation_healthy,
            database_pool_max_connections: SIGNER_DATABASE_MAX_CONNECTIONS,
            database_pool_size,
            database_pool_idle_connections,
            private_key_exported_to_game_server: false,
            provider_kind: "file_seed".to_string(),
            public_key_base64: STANDARD.encode(state.signing_key.verifying_key().to_bytes()),
            public_key_sha256: format!(
                "{:x}",
                Sha256::digest(state.signing_key.verifying_key().to_bytes())
            ),
            key_non_exportable: false,
            external_provider_attested: false,
        }),
    )
}

async fn attest_signer(
    State(state): State<SignerState>,
    headers: HeaderMap,
    Json(request): Json<EntitlementSignerAttestationRequest>,
) -> Result<Json<EntitlementSignerAttestationResponse>, ApiError> {
    require_auth(&state, &headers)?;
    if request.contract_version != ENTITLEMENT_SIGNER_CONTRACT
        || !(32..=128).contains(&request.challenge.len())
        || !request
            .challenge
            .bytes()
            .all(|byte| byte.is_ascii_graphic())
    {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "signer attestation challenge is invalid",
        ));
    }
    let observed_at_epoch = Utc::now().timestamp();
    let public_key = state.signing_key.verifying_key().to_bytes();
    let mut response = EntitlementSignerAttestationResponse {
        contract_version: ENTITLEMENT_SIGNER_CONTRACT.to_string(),
        challenge: request.challenge,
        key_id: state.key_id.as_ref().clone(),
        issuer: ENTITLEMENT_SIGNER_ISSUER.to_string(),
        provider_kind: "file_seed".to_string(),
        public_key_base64: STANDARD.encode(public_key),
        public_key_sha256: format!("{:x}", Sha256::digest(public_key)),
        observed_at_epoch,
        expires_at_epoch: observed_at_epoch.saturating_add(30),
        signature: String::new(),
    };
    let payload = response
        .signing_payload()
        .map_err(|message| error(StatusCode::INTERNAL_SERVER_ERROR, message))?;
    response.signature = STANDARD.encode(state.signing_key.sign(&payload).to_bytes());
    Ok(Json(response))
}

fn validate_request_id(request_id: &str) -> Result<(), ApiError> {
    if request_id.is_empty()
        || request_id.len() > 200
        || !request_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'-' | b'_' | b'.'))
    {
        return Err(error(StatusCode::BAD_REQUEST, "invalid signer request id"));
    }
    Ok(())
}

fn validate_request_identity(request: &EntitlementSignRequest) -> Result<(), ApiError> {
    if request.contract_version != ENTITLEMENT_SIGNER_CONTRACT {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "invalid signer request contract",
        ));
    }
    validate_request_id(&request.request_id)
}

fn validate_request(request: &EntitlementSignRequest) -> Result<(), ApiError> {
    validate_request_identity(request)?;
    let entitlement = &request.entitlement;
    let now = Utc::now().timestamp();
    if entitlement.contract_version != SERVER_SIGNED_VALUE_ENTITLEMENT_V2_CONTRACT
        || entitlement.signature_algorithm != "ed25519"
        || !entitlement.signature.is_empty()
        || !entitlement.key_id.is_empty()
        || entitlement.issuer != ENTITLEMENT_SIGNER_ISSUER
        || entitlement.source != ValueEntitlementSource::Battle
        || !(1..=100).contains(&entitlement.amount_credits)
        || entitlement.currency != "wallet_credits"
        || entitlement.issued_at_epoch < now.saturating_sub(60)
        || entitlement.issued_at_epoch > now.saturating_add(15)
        || entitlement.expires_at_epoch <= entitlement.issued_at_epoch
        || entitlement.expires_at_epoch > entitlement.issued_at_epoch.saturating_add(600)
        || entitlement.match_id.is_empty()
        || entitlement.result_hash.len() != 64
        || entitlement.participants_hash.len() != 64
        || entitlement.nonce.is_empty()
        || entitlement.intent_id != request.request_id
    {
        return Err(error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "signer rejected entitlement policy or authoritative binding",
        ));
    }
    Ok(())
}

fn response_from_row(
    request_id: &str,
    row: &sqlx_postgres::PgRow,
    duplicate: bool,
) -> Result<EntitlementSignResponse, ApiError> {
    Ok(EntitlementSignResponse {
        contract_version: ENTITLEMENT_SIGNER_CONTRACT.to_string(),
        request_id: request_id.to_string(),
        request_hash: row
            .try_get("request_hash")
            .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?,
        signing_receipt_hash: row
            .try_get("signing_receipt_hash")
            .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?,
        key_id: row
            .try_get("key_id")
            .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?,
        issuer: row
            .try_get("issuer")
            .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?,
        signature: row
            .try_get("signature")
            .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?,
        duplicate,
    })
}

async fn get_signing_receipt(
    State(state): State<SignerState>,
    headers: HeaderMap,
    Path(request_id): Path<String>,
) -> Result<Json<EntitlementSignResponse>, ApiError> {
    require_auth(&state, &headers)?;
    validate_request_id(&request_id)?;
    let row = sqlx::query::query(
        "select request_hash, signing_receipt_hash, key_id, issuer, signature
           from trnm_entitlement_signing_receipts
          where request_id = $1",
    )
    .bind(&request_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?
    .ok_or_else(|| error(StatusCode::NOT_FOUND, "signer receipt not found"))?;
    Ok(Json(response_from_row(&request_id, &row, true)?))
}

async fn sign_entitlement(
    State(state): State<SignerState>,
    headers: HeaderMap,
    Json(mut request): Json<EntitlementSignRequest>,
) -> Result<Json<EntitlementSignResponse>, ApiError> {
    require_auth(&state, &headers)?;
    validate_request_identity(&request)?;
    if let Some(row) = sqlx::query::query(
        "select request_hash, signing_receipt_hash, key_id, issuer, signature, entitlement_json
         from trnm_entitlement_signing_receipts where request_id = $1",
    )
    .bind(&request.request_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?
    {
        let mut stored: ServerSignedValueEntitlementV2 = serde_json::from_value(
            row.try_get("entitlement_json")
                .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?,
        )
        .map_err(|decode| error(StatusCode::INTERNAL_SERVER_ERROR, decode.to_string()))?;
        stored.key_id.clear();
        stored.signature.clear();
        if serde_json::to_value(stored)
            .map_err(|encode| error(StatusCode::INTERNAL_SERVER_ERROR, encode.to_string()))?
            != serde_json::to_value(&request.entitlement)
                .map_err(|encode| error(StatusCode::INTERNAL_SERVER_ERROR, encode.to_string()))?
        {
            return Err(error(
                StatusCode::CONFLICT,
                "signer request id was replayed with a different payload",
            ));
        }
        return Ok(Json(response_from_row(&request.request_id, &row, true)?));
    }
    validate_request(&request)?;
    request.entitlement.key_id = state.key_id.as_ref().clone();
    let payload = request
        .entitlement
        .signing_payload()
        .map_err(|message| error(StatusCode::UNPROCESSABLE_ENTITY, message))?;
    let request_hash = format!("{:x}", Sha256::digest(&payload));
    let signature = STANDARD.encode(state.signing_key.sign(&payload).to_bytes());
    let signing_receipt_hash = format!(
        "{:x}",
        Sha256::digest(
            format!("{}:{}:{}", request_hash, state.key_id.as_str(), signature).as_bytes()
        )
    );
    request.entitlement.signature = signature.clone();
    let inserted = sqlx::query::query(
        "insert into trnm_entitlement_signing_receipts (
            request_id, request_hash, signing_receipt_hash, key_id, issuer,
            signature, entitlement_json
         ) values ($1, $2, $3, $4, $5, $6, $7)
         on conflict (request_id) do nothing",
    )
    .bind(&request.request_id)
    .bind(&request_hash)
    .bind(&signing_receipt_hash)
    .bind(state.key_id.as_str())
    .bind(ENTITLEMENT_SIGNER_ISSUER)
    .bind(&signature)
    .bind(
        serde_json::to_value(&request.entitlement)
            .map_err(|encode| error(StatusCode::INTERNAL_SERVER_ERROR, encode.to_string()))?,
    )
    .execute(&state.pool)
    .await
    .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?;
    if inserted.rows_affected() != 1 {
        let row = sqlx::query::query(
            "select request_hash, signing_receipt_hash, key_id, issuer, signature
             from trnm_entitlement_signing_receipts where request_id = $1",
        )
        .bind(&request.request_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?;
        let stored_hash: String = row
            .try_get("request_hash")
            .map_err(|db| error(StatusCode::INTERNAL_SERVER_ERROR, db.to_string()))?;
        if stored_hash != request_hash {
            return Err(error(
                StatusCode::CONFLICT,
                "concurrent signer request id collision",
            ));
        }
        return Ok(Json(response_from_row(&request.request_id, &row, true)?));
    }
    Ok(Json(EntitlementSignResponse {
        contract_version: ENTITLEMENT_SIGNER_CONTRACT.to_string(),
        request_id: request.request_id,
        request_hash,
        signing_receipt_hash,
        key_id: state.key_id.as_ref().clone(),
        issuer: ENTITLEMENT_SIGNER_ISSUER.to_string(),
        signature,
        duplicate: false,
    }))
}

#[cfg(unix)]
fn verify_private_key_permissions(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let metadata =
        std::fs::metadata(path).map_err(|error| format!("inspect signer private seed: {error}"))?;
    if !metadata.is_file() || metadata.permissions().mode() & 0o077 != 0 {
        return Err("signer private seed must be a regular mode-600 file".to_string());
    }
    Ok(())
}

#[cfg(not(unix))]
fn verify_private_key_permissions(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err("signer private seed must be a regular file".to_string());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trnm_entitlement_signer=info".into()),
        )
        .init();
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL is required for durable signer receipts".to_string())?;
    let key_id = std::env::var("TRNM_ENTITLEMENT_ED25519_KEY_ID")
        .map_err(|_| "TRNM_ENTITLEMENT_ED25519_KEY_ID is required".to_string())?;
    if key_id.is_empty()
        || key_id.len() > 100
        || !key_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err("signer key id is invalid".to_string());
    }
    let key_path = std::env::var("TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE")
        .map_err(|_| "TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE is required".to_string())?;
    verify_private_key_permissions(Path::new(&key_path))?;
    let seed = STANDARD
        .decode(
            std::fs::read_to_string(&key_path)
                .map_err(|error| format!("read signer private seed: {error}"))?
                .trim(),
        )
        .map_err(|error| format!("decode signer private seed: {error}"))?;
    let seed: [u8; 32] = seed
        .try_into()
        .map_err(|_| "signer private seed must contain exactly 32 bytes".to_string())?;
    let auth_token = std::env::var("TRNM_ENTITLEMENT_SIGNER_TOKEN")
        .map_err(|_| "TRNM_ENTITLEMENT_SIGNER_TOKEN is required".to_string())?;
    if auth_token.len() < 32 {
        return Err("TRNM_ENTITLEMENT_SIGNER_TOKEN must be at least 32 characters".to_string());
    }
    let bind_addr = std::env::var("TRNM_ENTITLEMENT_SIGNER_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:7010".to_string())
        .parse::<SocketAddr>()
        .map_err(|_| "TRNM_ENTITLEMENT_SIGNER_BIND_ADDR must be a socket address".to_string())?;
    if !bind_addr.ip().is_loopback() {
        return Err("isolated signer only permits loopback bind".to_string());
    }
    let pool = PgPoolOptions::new()
        .max_connections(SIGNER_DATABASE_MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .map_err(|error| format!("connect signer PostgreSQL: {error}"))?;
    sqlx::raw_sql::raw_sql(SIGNER_MIGRATION)
        .execute(&pool)
        .await
        .map_err(|error| format!("migrate signer receipts: {error}"))?;
    let state = SignerState {
        pool,
        auth_token: Arc::new(auth_token),
        signing_key: Arc::new(SigningKey::from_bytes(&seed)),
        key_id: Arc::new(key_id),
    };
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|error| format!("bind isolated signer {bind_addr}: {error}"))?;
    tracing::info!(%bind_addr, key_id = %state.key_id, "isolated entitlement signer ready");
    debug_assert_eq!(ENTITLEMENT_SIGNER_RECEIPT_PATH, "/v1/signer/receipts");
    axum::serve(
        listener,
        Router::new()
            .route("/health", get(health))
            .route("/v1/signer/readiness", get(readiness))
            .route("/v1/signer/attest", post(attest_signer))
            .route("/v1/signer/sign", post(sign_entitlement))
            .route(
                "/v1/signer/receipts/:request_id",
                get(get_signing_receipt),
            )
            .layer(DefaultBodyLimit::max(64 * 1024))
            .with_state(state),
    )
    .await
    .map_err(|error| format!("serve isolated signer: {error}"))
}

#[cfg(test)]
mod tests {
    use super::{database_pool_is_operational, validate_request_id};

    #[test]
    fn readiness_only_reports_saturation_at_the_pool_limit() {
        assert!(database_pool_is_operational(1, 0, 4));
        assert!(database_pool_is_operational(4, 1, 4));
        assert!(!database_pool_is_operational(4, 0, 4));
    }

    #[test]
    fn signer_receipt_lookup_accepts_only_stable_request_ids() {
        assert!(validate_request_id(
            "trnm-settlement-remote-v1:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        )
        .is_ok());
        assert!(validate_request_id("bad/request").is_err());
        assert!(validate_request_id("").is_err());
    }
}
