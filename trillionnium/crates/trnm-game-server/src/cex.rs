use crate::signer_protocol::{
    EntitlementIssuerKeyStatusRequest, EntitlementIssuerKeyStatusResponse, EntitlementSignRequest,
    EntitlementSignResponse, EntitlementSignerAttestationRequest,
    EntitlementSignerAttestationResponse, EntitlementSignerReadiness, ENTITLEMENT_SIGNER_CONTRACT,
    ENTITLEMENT_SIGNER_ISSUER, SIGNER_AUTH_HEADER,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{Datelike, Utc};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{sync::Arc, time::Duration};
use trnm_campaign_core::EconomyBackend;
use trnm_economy_protocol::{
    EconomicIntent, EconomicIntentKind, EconomicReceipt, EconomyAccountBinding,
    ServerSignedValueEntitlementV2, ValueEntitlementSource, WalletSnapshot,
    SERVER_SIGNED_VALUE_ENTITLEMENT_METADATA_KEY, SERVER_SIGNED_VALUE_ENTITLEMENT_V2_CONTRACT,
};

const PLAYER_SESSION_HEADER: &str = "x-trnm-player-session";
const GAME_AUTHORITY_HEADER: &str = "x-trnm-game-authority";

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
    blocking_client: reqwest::blocking::Client,
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
        let blocking_client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|error| format!("build blocking CEX/signer client: {error}"))?;
        Ok(Self {
            base_url: Arc::new(base_url.trim_end_matches('/').to_string()),
            game_authority_token: Arc::new(game_authority_token),
            signer_url: Arc::new(signer_url.trim_end_matches('/').to_string()),
            signer_token: Arc::new(signer_token),
            async_client,
            blocking_client,
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
            .header("x-trnm-game-authority", self.game_authority_token.as_str())
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

    fn authority_headers(&self) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();
        headers.insert(
            GAME_AUTHORITY_HEADER,
            self.game_authority_token
                .parse()
                .map_err(|_| "game authority token is not a valid header".to_string())?,
        );
        Ok(headers)
    }

    fn execute_authoritative(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
        let mut authorized = intent.clone();
        if matches!(authorized.kind, EconomicIntentKind::ReleaseReward)
            && authorized.amount_credits.unwrap_or_default() > 0
        {
            let actor = authorized
                .actors
                .first()
                .ok_or_else(|| "reward intent has no primary actor".to_string())?;
            let account_id = actor
                .account_id
                .clone()
                .ok_or_else(|| "reward intent has no account".to_string())?;
            let metadata_string = |key: &str| -> Result<String, String> {
                authorized
                    .metadata
                    .get(key)
                    .and_then(|value| value.as_str())
                    .filter(|value| !value.is_empty())
                    .map(str::to_string)
                    .ok_or_else(|| format!("online reward is missing authoritative {key}"))
            };
            let now = Utc::now();
            let mut entitlement = ServerSignedValueEntitlementV2 {
                contract_version: SERVER_SIGNED_VALUE_ENTITLEMENT_V2_CONTRACT.to_string(),
                entitlement_id: format!("trnm-online-entitlement:{}", uuid::Uuid::new_v4()),
                issuer: ENTITLEMENT_SIGNER_ISSUER.to_string(),
                key_id: String::new(),
                signature_algorithm: "ed25519".to_string(),
                actor_id: actor.actor_id.clone(),
                account_id,
                source: ValueEntitlementSource::Battle,
                source_id: authorized
                    .metadata
                    .get("value_event_id")
                    .and_then(|value| value.as_str())
                    .unwrap_or(&authorized.intent_id)
                    .to_string(),
                intent_id: authorized.intent_id.clone(),
                amount_credits: authorized.amount_credits.unwrap_or_default(),
                currency: "wallet_credits".to_string(),
                budget_day: (now.year() as u32) * 10_000 + now.month() * 100 + now.day(),
                issued_at_epoch: now.timestamp(),
                expires_at_epoch: now.timestamp().saturating_add(600),
                match_id: metadata_string("online_match_id")?,
                rules_version: metadata_string("online_rules_version")?,
                build_id: metadata_string("online_build_id")?,
                result_hash: metadata_string("online_result_hash")?,
                participants_hash: metadata_string("online_participants_hash")?,
                nonce: uuid::Uuid::new_v4().to_string(),
                signature: String::new(),
            };
            let response = self
                .blocking_client
                .post(format!("{}/v1/signer/sign", self.signer_url))
                .header(SIGNER_AUTH_HEADER, self.signer_token.as_str())
                .json(&EntitlementSignRequest {
                    contract_version: ENTITLEMENT_SIGNER_CONTRACT.to_string(),
                    request_id: authorized.intent_id.clone(),
                    entitlement: entitlement.clone(),
                })
                .send()
                .map_err(|error| format!("isolated signer transport: {error}"))?;
            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().unwrap_or_default();
                return Err(format!(
                    "isolated signer rejected entitlement ({status}): {body}"
                ));
            }
            let signed = response
                .json::<EntitlementSignResponse>()
                .map_err(|error| format!("decode isolated signer response: {error}"))?;
            if signed.contract_version != ENTITLEMENT_SIGNER_CONTRACT
                || signed.request_id != authorized.intent_id
                || signed.issuer != ENTITLEMENT_SIGNER_ISSUER
                || signed.key_id.is_empty()
                || signed.signature.is_empty()
            {
                return Err("isolated signer response failed binding validation".to_string());
            }
            entitlement.key_id = signed.key_id;
            entitlement.signature = signed.signature;
            let payload = entitlement.signing_payload()?;
            let request_hash = format!("{:x}", Sha256::digest(&payload));
            if request_hash != signed.request_hash {
                return Err("isolated signer response request hash mismatch".to_string());
            }
            authorized.metadata[SERVER_SIGNED_VALUE_ENTITLEMENT_METADATA_KEY] =
                serde_json::to_value(entitlement).map_err(|error| error.to_string())?;
        }
        let response = self
            .blocking_client
            .post(format!("{}/v1/trnm/economy/intents", self.base_url))
            .headers(self.authority_headers()?)
            .json(&json!({"intent": authorized}))
            .send()
            .map_err(|error| format!("CEX intent transport: {error}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(format!("CEX intent rejected ({status}): {body}"));
        }
        response
            .json::<EconomicReceipt>()
            .map_err(|error| format!("decode CEX receipt: {error}"))
    }
}

impl EconomyBackend for CexClient {
    fn backend_id(&self) -> &str {
        "cex-trnm-online-authority-v2"
    }

    fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
        self.execute_authoritative(intent)
    }

    fn wallet_snapshot(
        &self,
        binding: &EconomyAccountBinding,
        cursor: u64,
    ) -> Result<Option<WalletSnapshot>, String> {
        let response = self
            .blocking_client
            .post(format!("{}/v1/trnm/economy/wallet", self.base_url))
            .headers(self.authority_headers()?)
            .json(&json!({
                "actor_id": binding.actor_id,
                "account_id": binding.account_id,
                "reconciliation_cursor": cursor
            }))
            .send()
            .map_err(|error| format!("CEX wallet transport: {error}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(format!("CEX wallet rejected ({status}): {body}"));
        }
        response
            .json::<WalletSnapshot>()
            .map(Some)
            .map_err(|error| format!("decode CEX wallet: {error}"))
    }
}
