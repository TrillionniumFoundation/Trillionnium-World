use serde::{Deserialize, Serialize};
use trnm_economy_protocol::ServerSignedValueEntitlementV2;

pub const ENTITLEMENT_SIGNER_CONTRACT: &str = "trnm_entitlement_signer_v1";
pub const ENTITLEMENT_SIGNER_ISSUER: &str = "trnm-online-game-server";
pub const SIGNER_AUTH_HEADER: &str = "x-trnm-signer-auth";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementSignRequest {
    pub contract_version: String,
    pub request_id: String,
    pub entitlement: ServerSignedValueEntitlementV2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementSignResponse {
    pub contract_version: String,
    pub request_id: String,
    pub request_hash: String,
    pub signing_receipt_hash: String,
    pub key_id: String,
    pub issuer: String,
    pub signature: String,
    pub duplicate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementSignerReadiness {
    pub status: String,
    pub contract_version: String,
    pub key_id: String,
    pub issuer: String,
    pub custody: String,
    pub postgres_receipts: bool,
    pub private_key_exported_to_game_server: bool,
}
