use crate::signer_protocol::{
    EntitlementIssuerKeyStatusRequest, EntitlementIssuerKeyStatusResponse,
    EntitlementSignerAttestationRequest, EntitlementSignerAttestationResponse,
    EntitlementSignerReadiness, ENTITLEMENT_SIGNER_CONTRACT, ENTITLEMENT_SIGNER_ISSUER,
    SIGNER_AUTH_HEADER,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{sync::Arc, time::Duration};
use trnm_campaign_core::EconomyBackend;
use trnm_economy_protocol::{
    EconomicIntent, EconomicReceipt, EconomyAccountBinding, WalletSnapshot,
};

const PLAYER_SESSION_HEADER: &str = "x-trnm-player-session";
const GAME_AUTHORITY_HEADER: &str = "x-trnm-game-authority";
const SETTLEMENT_OUTBOX_REQUIRED: &str =
    "external economy settlement is owned by trnm-settlement-worker; synchronous EconomyBackend I/O is prohibited";

#[derive(Debug, Clone, Serialize)]
struct SessionVerifyRequest<'a> {
    player_id: &'a str,
    account_id: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionVerifyResponse {
    pub verified: bool,
    pub session_id: String,
    pub player_id: String,
    pub account_id: String,
    pub device_id: String,
    pub recovery_generation: i64,
    pub expires_at_epoch: i64,
}

#[derive(Clone)]
pub struct CexClient {
    base_url: Arc<String>,
    game_authority_token: Arc<String>,
    signer_url: Arc<String>,
    signer_token: Arc<String>,
    async_client: reqwest::Client,
}

impl CexClient {
    pub fn new(
        base_url: String,
        game_authority_token: String,
        signer_url: String,
        signer_token: String,
    ) -> Result<Self, String> {
        if game_authority_token.len() < 24 {
            return Err("TRNM_GAME_AUTHORITY_TOKEN must be at least 24 characters".to_string());
        }
        if signer_token.len() < 32 {
            return Err("TRNM_ENTITLEMENT_SIGNER_TOKEN must be at least 32 characters".to_string());
        }
        if signer_url.trim().is_empty() {
            return Err("TRNM_ENTITLEMENT_SIGNER_URL is required".to_string());
        }
        let async_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|error| format!("build asynchronous CEX/signer client: {error}"))?;
        Ok(Self {
            base_url: Arc::new(base_url.trim_end_matches('/').to_string()),
            game_authority_token: Arc::new(game_authority_token),
            signer_url: Arc::new(signer_url.trim_end_matches('/').to_string()),
            signer_token: Arc::new(signer_token),
            async_client,
        })
    }

    pub async fn readiness(&self) -> Result<(), String> {
        let response = self
            .async_client
            .get(format!("{}/v1/trnm/economy/readiness", self.base_url))
            .send()
            .await
            .map_err(|error| format!("CEX readiness transport: {error}"))?;
        if !response.status().is_success() {
            return Err(format!("CEX readiness returned {}", response.status()));
        }
        self.signer_attestation().await.map(|_| ())
    }

    pub async fn signer_readiness(&self) -> Result<EntitlementSignerReadiness, String> {
        let response = self
            .async_client
            .get(format!("{}/v1/signer/readiness", self.signer_url))
            .send()
            .await
            .map_err(|error| format!("isolated signer readiness transport: {error}"))?;
        if !response.status().is_success() {
            return Err(format!(
                "isolated signer readiness returned {}",
                response.status()
            ));
        }
        let readiness = response
            .json::<EntitlementSignerReadiness>()
            .await
            .map_err(|error| format!("decode isolated signer readiness: {error}"))?;
        if readiness.status != "ok"
            || readiness.contract_version != ENTITLEMENT_SIGNER_CONTRACT
            || readiness.private_key_exported_to_game_server
            || !readiness.database_pool_saturation_healthy
        {
            return Err("isolated signer readiness failed custody contract".to_string());
        }
        Ok(readiness)
    }

    pub async fn signer_attestation(&self) -> Result<EntitlementSignerAttestationResponse, String> {
        let challenge = format!("trnm-signer-registry-check:{}", uuid::Uuid::new_v4());
        let response = self
            .async_client
            .post(format!("{}/v1/signer/attest", self.signer_url))
            .header(SIGNER_AUTH_HEADER, self.signer_token.as_str())
            .json(&EntitlementSignerAttestationRequest {
                contract_version: ENTITLEMENT_SIGNER_CONTRACT.to_string(),
                challenge: challenge.clone(),
            })
            .send()
            .await
            .map_err(|error| format!("isolated signer attestation transport: {error}"))?;
        if !response.status().is_success() {
            return Err(format!(
                "isolated signer attestation returned {}",
                response.status()
            ));
        }
        let attestation = response
            .json::<EntitlementSignerAttestationResponse>()
            .await
            .map_err(|error| format!("decode isolated signer attestation: {error}"))?;
        let now = Utc::now().timestamp();
        if attestation.contract_version != ENTITLEMENT_SIGNER_CONTRACT
            || attestation.challenge != challenge
            || attestation.issuer != ENTITLEMENT_SIGNER_ISSUER
            || attestation.observed_at_epoch > now.saturating_add(5)
            || attestation.observed_at_epoch < now.saturating_sub(15)
            || attestation.expires_at_epoch <= now
            || attestation.expires_at_epoch > attestation.observed_at_epoch.saturating_add(30)
        {
            return Err("isolated signer attestation binding is invalid".to_string());
        }
        let public_key = STANDARD
            .decode(&attestation.public_key_base64)
            .map_err(|error| format!("decode signer attestation public key: {error}"))?;
        let public_key: [u8; 32] = public_key
            .try_into()
            .map_err(|_| "signer attestation public key must contain 32 bytes".to_string())?;
        if format!("{:x}", Sha256::digest(public_key)) != attestation.public_key_sha256 {
            return Err("signer attestation public-key fingerprint mismatch".to_string());
        }
        let signature = STANDARD
            .decode(&attestation.signature)
            .map_err(|error| format!("decode signer attestation signature: {error}"))?;
        let signature = Signature::from_slice(&signature)
            .map_err(|error| format!("decode signer Ed25519 signature: {error}"))?;
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|error| format!("decode signer Ed25519 public key: {error}"))?;
        let payload = attestation.signing_payload()?;
        verifying_key
            .verify(&payload, &signature)
            .map_err(|_| "signer key-possession attestation signature failed".to_string())?;

        let registry = self
            .async_client
            .post(format!(
                "{}/v1/trnm/economy/issuer-keys/status",
                self.base_url
            ))
            .header(GAME_AUTHORITY_HEADER, self.game_authority_token.as_str())
            .json(&EntitlementIssuerKeyStatusRequest {
                key_id: attestation.key_id.clone(),
            })
            .send()
            .await
            .map_err(|error| format!("CEX issuer registry status transport: {error}"))?;
        if !registry.status().is_success() {
            return Err(format!(
                "CEX issuer registry rejected signer key ({})",
                registry.status()
            ));
        }
        let registry = registry
            .json::<EntitlementIssuerKeyStatusResponse>()
            .await
            .map_err(|error| format!("decode CEX issuer registry status: {error}"))?;
        if registry.key_id != attestation.key_id
            || registry.issuer != attestation.issuer
            || registry.status != "active"
            || registry.signature_algorithm != "ed25519"
            || registry.public_key_sha256 != attestation.public_key_sha256
        {
            return Err("signer key is not the active CEX registry key".to_string());
        }
        Ok(attestation)
    }

    pub async fn verify_session(
        &self,
        player_session: &str,
        player_id: &str,
        account_id: &str,
    ) -> Result<SessionVerifyResponse, String> {
        let response = self
            .async_client
            .post(format!("{}/v1/trnm/identity/session/verify", self.base_url))
            .header(PLAYER_SESSION_HEADER, player_session)
            .json(&SessionVerifyRequest {
                player_id,
                account_id,
            })
            .send()
            .await
            .map_err(|error| format!("CEX session verification transport: {error}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("CEX rejected player session ({status}): {body}"));
        }
        let verified = response
            .json::<SessionVerifyResponse>()
            .await
            .map_err(|error| format!("decode CEX session verification: {error}"))?;
        if !verified.verified {
            return Err("CEX player session is not verified".to_string());
        }
        Ok(verified)
    }
}

impl EconomyBackend for CexClient {
    fn backend_id(&self) -> &str {
        "cex-trnm-settlement-outbox-v1"
    }

    fn execute(&self, _intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
        Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())
    }

    fn wallet_snapshot(
        &self,
        _binding: &EconomyAccountBinding,
        _cursor: u64,
    ) -> Result<Option<WalletSnapshot>, String> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{CexClient, SETTLEMENT_OUTBOX_REQUIRED};
    use trnm_campaign_core::EconomyBackend;
    use trnm_economy_protocol::{
        ActorRef, EconomicIntent, EconomicIntentKind, IdempotencyKey,
        TERM_EXCHANGE_PROTOCOL_VERSION,
    };

    #[test]
    fn synchronous_backend_never_performs_external_settlement() {
        let client = CexClient::new(
            "http://127.0.0.1:1".to_string(),
            "g".repeat(24),
            "http://127.0.0.1:2".to_string(),
            "s".repeat(32),
        )
        .unwrap();
        let intent = EconomicIntent {
            protocol_version: TERM_EXCHANGE_PROTOCOL_VERSION.to_string(),
            intent_id: "intent-a".to_string(),
            term_id: "term-a".to_string(),
            term_version: "1".to_string(),
            domain: "trnm_game".to_string(),
            kind: EconomicIntentKind::CompleteContract,
            idempotency_key: IdempotencyKey {
                scope: "test".to_string(),
                key: "intent-a".to_string(),
            },
            actors: vec![ActorRef {
                actor_id: "actor-a".to_string(),
                actor_kind: "player".to_string(),
                account_id: None,
            }],
            assets: Vec::new(),
            amount_credits: Some(0),
            currency: None,
            metadata: serde_json::json!({}),
            created_at_epoch: 0,
        };
        assert_eq!(client.execute(&intent), Err(SETTLEMENT_OUTBOX_REQUIRED.to_string()));
        assert_eq!(client.wallet_snapshot(&trnm_economy_protocol::EconomyAccountBinding {
            actor_id: "actor-a".to_string(),
            account_id: "account-a".to_string(),
            binding_revision: 1,
        }, 0), Ok(None));
    }
}
