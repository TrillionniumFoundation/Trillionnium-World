use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::SigningKey;
use serde_json::json;

fn main() -> Result<(), String> {
    let seed = std::env::var("TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64")
        .map_err(|_| "TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_BASE64 is required".to_string())?;
    let seed = STANDARD
        .decode(seed.trim())
        .map_err(|error| format!("decode Ed25519 seed: {error}"))?;
    let seed: [u8; 32] = seed
        .try_into()
        .map_err(|_| "Ed25519 seed must contain exactly 32 bytes".to_string())?;
    let key_id = std::env::var("TRNM_ENTITLEMENT_ED25519_KEY_ID")
        .map_err(|_| "TRNM_ENTITLEMENT_ED25519_KEY_ID is required".to_string())?;
    if key_id.trim().is_empty() {
        return Err("TRNM_ENTITLEMENT_ED25519_KEY_ID cannot be empty".to_string());
    }
    let verifying_key = SigningKey::from_bytes(&seed).verifying_key();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "keys": {
                key_id: {
                    "issuer": "trnm-online-game-server",
                    "public_key_base64": STANDARD.encode(verifying_key.to_bytes()),
                    "status": "active"
                }
            }
        }))
        .map_err(|error| format!("encode issuer registry: {error}"))?
    );
    Ok(())
}
