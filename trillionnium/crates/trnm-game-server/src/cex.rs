use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use trnm_campaign_core::EconomyBackend;
use trnm_economy_protocol::{
    EconomicIntent, EconomicIntentKind, EconomicReceipt, EconomyAccountBinding,
    ServerSignedValueEntitlementV1, ValueEntitlementSource, WalletSnapshot,
    SERVER_SIGNED_VALUE_ENTITLEMENT_METADATA_KEY,
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
    async_client: reqwest::Client,
    blocking_client: reqwest::blocking::Client,
}

impl CexClient {
    pub fn new(base_url: String, game_authority_token: String) -> Result<Self, String> {
        if game_authority_token.len() < 24 {
            return Err("TRNM_GAME_AUTHORITY_TOKEN must be at least 24 characters".to_string());
        }
        Ok(Self {
            base_url: Arc::new(base_url.trim_end_matches('/').to_string()),
            game_authority_token: Arc::new(game_authority_token),
            async_client: reqwest::Client::new(),
            blocking_client: reqwest::blocking::Client::new(),
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
        Ok(())
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
            let entitlement = self
                .blocking_client
                .post(format!("{}/v1/trnm/economy/entitlements", self.base_url))
                .headers(self.authority_headers()?)
                .json(&json!({
                    "actor_id": actor.actor_id,
                    "account_id": account_id,
                    "source": ValueEntitlementSource::Battle,
                    "source_id": authorized.metadata.get("value_event_id")
                        .and_then(|value| value.as_str())
                        .unwrap_or(&authorized.intent_id),
                    "intent_id": authorized.intent_id,
                    "amount_credits": authorized.amount_credits.unwrap_or_default(),
                    "lifetime_seconds": 600
                }))
                .send()
                .map_err(|error| format!("CEX entitlement transport: {error}"))?;
            if !entitlement.status().is_success() {
                let status = entitlement.status();
                let body = entitlement.text().unwrap_or_default();
                return Err(format!("CEX entitlement rejected ({status}): {body}"));
            }
            let entitlement = entitlement
                .json::<ServerSignedValueEntitlementV1>()
                .map_err(|error| format!("decode CEX entitlement: {error}"))?;
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
        "cex-trnm-online-authority-v1"
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
