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
    pub provider_kind: String,
    pub public_key_base64: String,
    pub public_key_sha256: String,
    pub key_non_exportable: bool,
    pub external_provider_attested: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementSignerAttestationRequest {
    pub contract_version: String,
    pub challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementSignerAttestationResponse {
    pub contract_version: String,
    pub challenge: String,
    pub key_id: String,
    pub issuer: String,
    pub provider_kind: String,
    pub public_key_base64: String,
    pub public_key_sha256: String,
    pub observed_at_epoch: i64,
    pub expires_at_epoch: i64,
    pub signature: String,
}

impl EntitlementSignerAttestationResponse {
    pub fn signing_payload(&self) -> Result<Vec<u8>, String> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_json::to_vec(&unsigned).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementIssuerKeyStatusRequest {
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementIssuerKeyStatusResponse {
    pub key_id: String,
    pub issuer: String,
    pub status: String,
    pub signature_algorithm: String,
    pub public_key_sha256: String,
}
