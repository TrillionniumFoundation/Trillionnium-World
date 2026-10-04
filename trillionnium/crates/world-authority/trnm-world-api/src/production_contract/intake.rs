//! Opt-in strict structural intake for the production-adapter value boundary.
//!
//! This profile adds bounded raw JSON, duplicate/unknown-field rejection and
//! explicit scalar limits without changing legacy DTO deserialization or wire
//! serialization. It does NOT authenticate callers, recompute command/state
//! hashes, implement a repository, or grant production readiness. Runtime routes
//! must explicitly adopt it under the separate intake contract.

use super::base::*;
use serde::de::{self, DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use std::collections::BTreeSet;
use std::fmt;

pub const WORLD_PRODUCTION_INTAKE_CONTRACT: &str = "trillionnium_world_production_intake_v1";
pub const MAX_PRODUCTION_INPUT_BYTES: usize = 128 * 1024;
pub const MAX_PRODUCTION_ID_BYTES: usize = 160;
pub const MAX_PRODUCTION_LABELS: usize = 16;

mod sealed {
    pub trait Sealed {}
}

/// Closed set of data transfer values covered by the frozen intake profile.
/// A validated value is still untrusted for all identity and authority purposes.
pub trait ProductionIntakeValue: sealed::Sealed + DeserializeOwned {
    const FIELDS: &'static [&'static str];
}

macro_rules! intake_value {
    ($ty:ty, [$($field:literal),+ $(,)?]) => {
        impl sealed::Sealed for $ty {}
        impl ProductionIntakeValue for $ty {
            const FIELDS: &'static [&'static str] = &[$($field),+];
        }
    };
}

intake_value!(WorldCredentialReference, [
    "adapter_contract",
    "credential_id",
    "issuer",
    "audience",
]);

intake_value!(WorldIdentityResolutionRequest, [
    "adapter_contract",
    "request_id",
    "credential",
]);

intake_value!(WorldSessionAuthorizationRequest, [
    "adapter_contract",
    "request_id",
    "actor_id",
    "account_id",
    "session_id",
    "expected_generation",
    "expected_audience",
]);

intake_value!(WorldAccountOperationRequest, [
    "adapter_contract",
    "request_id",
    "actor_id",
    "account_id",
    "session_id",
    "session_generation",
    "operation",
]);

intake_value!(WorldMutationContext, [
    "adapter_contract",
    "request_id",
    "actor_id",
    "account_id",
    "session_id",
    "session_generation",
    "aggregate_id",
    "expected_revision",
    "writer_epoch",
    "api_contract",
    "domain_contract",
    "command_contract",
    "command_sha256",
]);

intake_value!(WorldAggregateLoadRequest, [
    "adapter_contract",
    "aggregate_id",
    "expected_writer_epoch",
]);

intake_value!(WorldAggregateRequestLookup, [
    "adapter_contract",
    "aggregate_id",
    "request_id",
    "request_sha256",
]);

intake_value!(WorldLedgerLookupRequest, [
    "adapter_contract",
    "request_id",
    "intent_id",
    "intent_sha256",
    "account_id",
]);

intake_value!(WorldLedgerSubmitRequest, [
    "adapter_contract",
    "request_id",
    "intent_id",
    "intent_sha256",
    "account_id",
    "amount_units",
    "currency",
]);

intake_value!(WorldEvidenceAppendRequest, [
    "adapter_contract",
    "request_id",
    "evidence_kind",
    "subject_id",
    "payload_sha256",
]);

intake_value!(WorldMetricRecordRequest, [
    "adapter_contract",
    "metric_name",
    "value",
    "bounded_labels",
]);

intake_value!(WorldRoutingAuthorizationRequest, [
    "adapter_contract",
    "request_id",
    "caller_identity",
    "caller_audience",
    "component_lock_id",
    "route",
]);

intake_value!(WorldRoutingDecision, [
    "adapter_contract",
    "accepted",
    "caller_identity",
    "component_lock_id",
    "reason_code",
]);

intake_value!(WorldDeploymentIdentity, [
    "adapter_contract",
    "deployment_id",
    "environment",
    "service_identity",
    "component_lock_id",
    "source_commit",
    "source_tree",
    "binary_sha256",
]);

intake_value!(WorldProductionAdapterError, [
    "adapter_contract",
    "code",
    "retryable",
    "safe_message",
]);

fn failure(code: WorldProductionAdapterErrorCode) -> WorldProductionAdapterError {
    // Never reflect an attacker-controlled field, payload, token or value.
    WorldProductionAdapterError::new(code, false, "production intake rejected")
}

pub(super) struct StrictValue(pub(super) Value);

impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StrictVisitor;
        impl<'de> Visitor<'de> for StrictVisitor {
            type Value = StrictValue;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("bounded JSON with unique object keys and integer numbers")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<StrictValue, E> {
                Ok(StrictValue(Value::Bool(v)))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<StrictValue, E> {
                Ok(StrictValue(Value::Number(v.into())))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<StrictValue, E> {
                Ok(StrictValue(Value::Number(v.into())))
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> Result<StrictValue, E> {
                Err(E::custom("floating point numbers are forbidden"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<StrictValue, E> {
                Ok(StrictValue(Value::String(v.to_owned())))
            }
            fn visit_string<E: de::Error>(self, v: String) -> Result<StrictValue, E> {
                Ok(StrictValue(Value::String(v)))
            }
            fn visit_unit<E: de::Error>(self) -> Result<StrictValue, E> {
                Ok(StrictValue(Value::Null))
            }
            fn visit_none<E: de::Error>(self) -> Result<StrictValue, E> {
                self.visit_unit()
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<StrictValue, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element::<StrictValue>()? {
                    values.push(value.0);
                }
                Ok(StrictValue(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<StrictValue, A::Error> {
                let mut values = Map::new();
                while let Some((key, value)) = map.next_entry::<String, StrictValue>()? {
                    if values.insert(key, value.0).is_some() {
                        return Err(de::Error::custom("duplicate JSON key"));
                    }
                }
                Ok(StrictValue(Value::Object(values)))
            }
        }
        deserializer.deserialize_any(StrictVisitor)
    }
}

fn shape<'a>(value: &'a Value, fields: &[&str]) -> WorldProductionAdapterResult<&'a Map<String, Value>> {
    let object = value.as_object().ok_or_else(|| failure(WorldProductionAdapterErrorCode::InvalidInput))?;
    if object.len() != fields.len() || fields.iter().any(|field| !object.contains_key(*field)) {
        return Err(failure(WorldProductionAdapterErrorCode::InvalidInput));
    }
    Ok(object)
}

fn bounded_text(value: &str, cap: usize) -> bool {
    !value.is_empty() && value.trim() == value && value.len() <= cap && !value.chars().any(char::is_control)
}

fn lower_hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate_fields(object: &Map<String, Value>) -> WorldProductionAdapterResult<()> {
    if object.get("adapter_contract").and_then(Value::as_str) != Some(WORLD_PRODUCTION_ADAPTER_CONTRACT) {
        return Err(failure(WorldProductionAdapterErrorCode::UnsupportedContract));
    }
    for (key, value) in object {
        match value {
            Value::String(text) => {
                let valid = if key.ends_with("_sha256") {
                    lower_hex(text, 64)
                } else if matches!(key.as_str(), "source_commit" | "source_tree") {
                    lower_hex(text, 40)
                } else if key == "safe_message" {
                    bounded_text(text, 256)
                } else if key == "route" {
                    bounded_text(text, 512) && text.starts_with('/')
                        && !text.starts_with("//") && !text.chars().any(|c| matches!(c, '?' | '#' | '\\'))
                } else {
                    bounded_text(text, MAX_PRODUCTION_ID_BYTES)
                };
                if !valid {
                    return Err(failure(WorldProductionAdapterErrorCode::InvalidInput));
                }
            }
            Value::Number(number) => {
                // All unsigned counters fit PostgreSQL BIGINT under this profile.
                // This validates representation, not freshness or authorization.
                if number.as_i64().is_none() {
                    return Err(failure(WorldProductionAdapterErrorCode::InvalidInput));
                }
            }
            Value::Object(nested) if key == "credential" => validate_fields(nested)?,
            Value::Array(labels) if key == "bounded_labels" => {
                if labels.len() > MAX_PRODUCTION_LABELS {
                    return Err(failure(WorldProductionAdapterErrorCode::InvalidInput));
                }
                let mut seen = BTreeSet::new();
                for label in labels {
                    let pair = label.as_array().ok_or_else(|| failure(WorldProductionAdapterErrorCode::InvalidInput))?;
                    if pair.len() != 2 {
                        return Err(failure(WorldProductionAdapterErrorCode::InvalidInput));
                    }
                    let name = pair[0].as_str().ok_or_else(|| failure(WorldProductionAdapterErrorCode::InvalidInput))?;
                    let value = pair[1].as_str().ok_or_else(|| failure(WorldProductionAdapterErrorCode::InvalidInput))?;
                    if !bounded_text(name, 64) || !bounded_text(value, MAX_PRODUCTION_ID_BYTES) || !seen.insert(name) {
                        return Err(failure(WorldProductionAdapterErrorCode::InvalidInput));
                    }
                }
            }
            Value::Bool(_) => {}
            _ => return Err(failure(WorldProductionAdapterErrorCode::InvalidInput)),
        }
    }
    Ok(())
}

/// Decode a covered DTO using the opt-in strict profile. Transport code must
/// enforce the same byte ceiling before buffering. This never performs I/O.
pub fn decode_production_input<T: ProductionIntakeValue>(bytes: &[u8]) -> WorldProductionAdapterResult<T> {
    if bytes.len() > MAX_PRODUCTION_INPUT_BYTES {
        return Err(failure(WorldProductionAdapterErrorCode::CapacityExceeded));
    }
    let strict: StrictValue = serde_json::from_slice(bytes)
        .map_err(|_| failure(WorldProductionAdapterErrorCode::InvalidInput))?;
    let object = shape(&strict.0, T::FIELDS)?;
    if let Some(credential) = object.get("credential") {
        shape(credential, WorldCredentialReference::FIELDS)?;
    }
    // Typed deserialization checks booleans, enums, integer ranges and nested
    // tuple shapes before semantic scalar validation. No coercions are allowed.
    let typed: T = serde_json::from_value(strict.0.clone())
        .map_err(|_| failure(WorldProductionAdapterErrorCode::InvalidInput))?;
    validate_fields(object)?;
    Ok(typed)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn credential() -> &'static [u8] {
        br#"{"adapter_contract":"trillionnium_world_production_adapter_v1","credential_id":"cred-1","issuer":"nakama","audience":"world"}"#
    }

    #[test]
    fn accepts_bounded_exact_shape() {
        let got: WorldCredentialReference = decode_production_input(credential()).unwrap();
        assert_eq!(got.issuer, "nakama");
    }

    #[test]
    fn rejects_duplicate_unknown_and_wrong_contract() {
        for raw in [
            br#"{"adapter_contract":"trillionnium_world_production_adapter_v1","credential_id":"a","credential_id":"b","issuer":"n","audience":"w"}"#.as_slice(),
            br#"{"adapter_contract":"trillionnium_world_production_adapter_v1","credential_id":"a","issuer":"n","audience":"w","extra":1}"#.as_slice(),
            br#"{"adapter_contract":"wrong","credential_id":"a","issuer":"n","audience":"w"}"#.as_slice(),
        ] {
            assert!(decode_production_input::<WorldCredentialReference>(raw).is_err());
        }
    }

    #[test]
    fn rejects_oversize_noninteger_and_unsafe_route() {
        assert_eq!(decode_production_input::<WorldCredentialReference>(&vec![b' '; MAX_PRODUCTION_INPUT_BYTES + 1]).unwrap_err().code,
            WorldProductionAdapterErrorCode::CapacityExceeded);
        let metric = br#"{"adapter_contract":"trillionnium_world_production_adapter_v1","metric_name":"m","value":1.5,"bounded_labels":[]}"#;
        assert!(decode_production_input::<WorldMetricRecordRequest>(metric).is_err());
        let route = br#"{"adapter_contract":"trillionnium_world_production_adapter_v1","request_id":"r","caller_identity":"i","caller_audience":"a","component_lock_id":"c","route":"//evil"}"#;
        assert!(decode_production_input::<WorldRoutingAuthorizationRequest>(route).is_err());
    }

    #[test]
    fn credential_reference_has_no_secret_value_surface() {
        let encoded = serde_json::to_string(&decode_production_input::<WorldCredentialReference>(credential()).unwrap()).unwrap();
        for forbidden in ["token_value", "password", "private_key", "cookie_value"] {
            assert!(!encoded.contains(forbidden));
        }
    }

    #[test]
    fn structural_intake_never_grants_production() {
        let readiness = world_production_adapter_readiness();
        assert!(!readiness.production_ready);
        assert!(!readiness.fixture_adapter_credit_allowed);
        assert_eq!(readiness.production_authorization, "not_granted");
    }
}
