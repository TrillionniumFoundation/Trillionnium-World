#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]

//! Host-side execution observations and deterministic shadow comparison for
//! `trnm_world_runtime_v1`.
//!
//! This crate is deliberately Bevy-free and has no networking, persistence,
//! signing, participant-admission, global-ordering, Chain-finality or CEX
//! custody capability. It compares already produced World-owned deterministic
//! material and resource observations only.

use serde_json::{json, Map, Value};
use trnm_world_runtime_adapter::{
    canonical_json_bytes, domain_hash, parse_strict_json, RuntimeError, CONTRACT_VERSION,
    EXECUTE_RESULT, FINAL_STATE_DOMAIN, OUTCOME_DOMAIN,
    REPLAY_MATERIAL_DOMAIN,
};

pub const RUNTIME_ERROR_VERSION: &str = "trnm_world_runtime_error_v1";
pub const RUNTIME_OBSERVATION_VERSION: &str = "trnm_world_runtime_observation_v1";
pub const SHADOW_INPUT_VERSION: &str = "trnm_world_shadow_input_v1";
pub const SHADOW_REPORT_VERSION: &str = "trnm_world_shadow_report_v1";
pub const SHADOW_REQUEST_DOMAIN: &str = "trnm.world.shadow.v1.request";
pub const SHADOW_RESPONSE_DOMAIN: &str = "trnm.world.shadow.v1.response";

const OBSERVATION_FIELDS: [&str; 7] = [
    "contract_version",
    "implementation_id",
    "implementation_revision",
    "request_hash",
    "response",
    "duration_micros",
    "response_bytes",
];
const SHADOW_INPUT_FIELDS: [&str; 4] = ["contract_version", "world", "candidate", "budgets"];
const BUDGET_FIELDS: [&str; 2] = [
    "max_candidate_duration_micros",
    "max_candidate_response_bytes",
];
const EXECUTE_RESULT_FIELDS: [&str; 12] = [
    "contract_version",
    "message_type",
    "ruleset",
    "content_digest",
    "initial_state_hash",
    "command_batch_hash",
    "final_state",
    "final_state_hash",
    "outcome",
    "outcome_hash",
    "replay_material",
    "replay_material_hash",
];
const RULESET_FIELDS: [&str; 3] = ["id", "version", "digest"];
const ERROR_FIELDS: [&str; 4] = ["contract_version", "error_code", "error", "recoverable"];
const FORBIDDEN_AUTHORITY_FIELDS: [&str; 13] = [
    "participant_roster",
    "participant_roles",
    "global_sequence",
    "event_root",
    "roster_root",
    "archive_root",
    "completion_signature",
    "authority_key_id",
    "chain_finality",
    "inclusion_proof",
    "wallet_balance",
    "session_token",
    "idempotency_receipt",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResponseKind {
    Success,
    Error,
}

impl ResponseKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy)]
struct ValidatedObservation<'a> {
    implementation_id: &'a str,
    implementation_revision: &'a str,
    request_hash: &'a str,
    response: &'a Value,
    duration_micros: u64,
    response_bytes: u64,
    response_kind: ResponseKind,
}

#[derive(Clone, Copy)]
struct ResourceBudgets {
    max_candidate_duration_micros: u64,
    max_candidate_response_bytes: u64,
}

pub fn runtime_observation(
    implementation_id: &str,
    implementation_revision: &str,
    request: &Value,
    response: Value,
    duration_micros: u64,
) -> Result<Value, RuntimeError> {
    require_identifier(implementation_id, "implementation_id")?;
    require_commit(implementation_revision, "implementation_revision")?;
    reject_forbidden_authority_keys(request, "runtime request")?;
    let request_hash = domain_hash(SHADOW_REQUEST_DOMAIN, request)?;
    validate_runtime_response(&response)?;
    let response_bytes = canonical_json_bytes(&response)?.len();
    let duration_micros = signed_json_integer(duration_micros, "duration_micros")?;
    let response_bytes = signed_json_integer(
        u64::try_from(response_bytes).map_err(|_| {
            RuntimeError::new(
                "resource_limit_exceeded",
                "response byte count exceeds u64 range",
            )
        })?,
        "response_bytes",
    )?;
    Ok(json!({
        "contract_version": RUNTIME_OBSERVATION_VERSION,
        "implementation_id": implementation_id,
        "implementation_revision": implementation_revision,
        "request_hash": request_hash,
        "response": response,
        "duration_micros": duration_micros,
        "response_bytes": response_bytes,
    }))
}

pub fn compare_shadow_json(input: &str) -> Result<String, RuntimeError> {
    let input = parse_strict_json(input)?;
    let report = compare_shadow_value(&input)?;
    String::from_utf8(canonical_json_bytes(&report)?).map_err(|error| {
        RuntimeError::new("output_contract_violation", error.to_string())
    })
}

pub fn compare_shadow_value(input: &Value) -> Result<Value, RuntimeError> {
    let input = exact_object(input, &SHADOW_INPUT_FIELDS, "shadow input")?;
    if input.get("contract_version").and_then(Value::as_str) != Some(SHADOW_INPUT_VERSION) {
        return Err(RuntimeError::new(
            "unsupported_contract",
            "unsupported World shadow input contract version",
        ));
    }
    let world = validate_observation(
        input
            .get("world")
            .ok_or_else(|| RuntimeError::new("invalid_contract", "world observation is missing"))?,
        "world",
    )?;
    let candidate = validate_observation(
        input.get("candidate").ok_or_else(|| {
            RuntimeError::new("invalid_contract", "candidate observation is missing")
        })?,
        "candidate",
    )?;
    let budgets = validate_budgets(
        input
            .get("budgets")
            .ok_or_else(|| RuntimeError::new("invalid_contract", "budgets are missing"))?,
    )?;

    let mut divergences = Vec::new();
    let duration_budget_ok = candidate.duration_micros <= budgets.max_candidate_duration_micros;
    if !duration_budget_ok {
        divergences.push(divergence(
            "candidate_duration_budget_exceeded",
            "candidate.duration_micros",
            Value::from(signed_json_integer(
                budgets.max_candidate_duration_micros,
                "max_candidate_duration_micros",
            )?),
            Value::from(signed_json_integer(
                candidate.duration_micros,
                "candidate.duration_micros",
            )?),
        ));
    }
    let response_budget_ok = candidate.response_bytes <= budgets.max_candidate_response_bytes;
    if !response_budget_ok {
        divergences.push(divergence(
            "candidate_response_budget_exceeded",
            "candidate.response_bytes",
            Value::from(signed_json_integer(
                budgets.max_candidate_response_bytes,
                "max_candidate_response_bytes",
            )?),
            Value::from(signed_json_integer(
                candidate.response_bytes,
                "candidate.response_bytes",
            )?),
        ));
    }

    let mut input_binding_equal = world.request_hash == candidate.request_hash;
    if !input_binding_equal {
        divergences.push(divergence(
            "request_hash_mismatch",
            "request_hash",
            Value::String(world.request_hash.to_owned()),
            Value::String(candidate.request_hash.to_owned()),
        ));
    }
    let mut final_state_equal = Value::Null;
    let mut outcome_equal = Value::Null;
    let mut replay_equal = Value::Null;
    let mut error_equal = Value::Null;

    match (world.response_kind, candidate.response_kind) {
        (ResponseKind::Success, ResponseKind::Success) => {
            let ruleset_equal = compare_path(
                world.response,
                candidate.response,
                "ruleset",
                "ruleset_mismatch",
                &mut divergences,
            );
            let content_equal = compare_path(
                world.response,
                candidate.response,
                "content_digest",
                "content_digest_mismatch",
                &mut divergences,
            );
            let initial_state_equal = compare_path(
                world.response,
                candidate.response,
                "initial_state_hash",
                "initial_state_hash_mismatch",
                &mut divergences,
            );
            let command_batch_equal = compare_path(
                world.response,
                candidate.response,
                "command_batch_hash",
                "command_batch_hash_mismatch",
                &mut divergences,
            );
            input_binding_equal = input_binding_equal
                && ruleset_equal
                && content_equal
                && initial_state_equal
                && command_batch_equal;

            let final_hash_equal = compare_path(
                world.response,
                candidate.response,
                "final_state_hash",
                "final_state_hash_mismatch",
                &mut divergences,
            );
            let final_material_equal = compare_path(
                world.response,
                candidate.response,
                "final_state",
                "final_state_material_mismatch",
                &mut divergences,
            );
            final_state_equal = Value::Bool(final_hash_equal && final_material_equal);

            let outcome_hash_equal = compare_path(
                world.response,
                candidate.response,
                "outcome_hash",
                "outcome_hash_mismatch",
                &mut divergences,
            );
            let outcome_material_equal = compare_path(
                world.response,
                candidate.response,
                "outcome",
                "outcome_material_mismatch",
                &mut divergences,
            );
            outcome_equal = Value::Bool(outcome_hash_equal && outcome_material_equal);

            let replay_hash_equal = compare_path(
                world.response,
                candidate.response,
                "replay_material_hash",
                "replay_material_hash_mismatch",
                &mut divergences,
            );
            let replay_material_equal = compare_path(
                world.response,
                candidate.response,
                "replay_material",
                "replay_material_mismatch",
                &mut divergences,
            );
            replay_equal = Value::Bool(replay_hash_equal && replay_material_equal);
        }
        (ResponseKind::Error, ResponseKind::Error) => {
            let code_equal = compare_path(
                world.response,
                candidate.response,
                "error_code",
                "error_code_mismatch",
                &mut divergences,
            );
            let recoverable_equal = compare_path(
                world.response,
                candidate.response,
                "recoverable",
                "error_recoverability_mismatch",
                &mut divergences,
            );
            error_equal = Value::Bool(code_equal && recoverable_equal);
        }
        _ => {
            divergences.push(divergence(
                "execution_kind_mismatch",
                "response",
                Value::String(world.response_kind.label().to_owned()),
                Value::String(candidate.response_kind.label().to_owned()),
            ));
        }
    }

    let world_response_hash = domain_hash(SHADOW_RESPONSE_DOMAIN, world.response)?;
    let candidate_response_hash = domain_hash(SHADOW_RESPONSE_DOMAIN, candidate.response)?;
    let resource_budget_ok = duration_budget_ok && response_budget_ok;
    let equivalent = divergences.is_empty();

    let report = json!({
        "contract_version": SHADOW_REPORT_VERSION,
        "equivalent": equivalent,
        "world_implementation": {
            "id": world.implementation_id,
            "revision": world.implementation_revision,
            "request_hash": world.request_hash,
            "response_kind": world.response_kind.label(),
            "response_hash": world_response_hash,
            "duration_micros": signed_json_integer(world.duration_micros, "world.duration_micros")?,
            "response_bytes": signed_json_integer(world.response_bytes, "world.response_bytes")?,
        },
        "candidate_implementation": {
            "id": candidate.implementation_id,
            "revision": candidate.implementation_revision,
            "request_hash": candidate.request_hash,
            "response_kind": candidate.response_kind.label(),
            "response_hash": candidate_response_hash,
            "duration_micros": signed_json_integer(candidate.duration_micros, "candidate.duration_micros")?,
            "response_bytes": signed_json_integer(candidate.response_bytes, "candidate.response_bytes")?,
        },
        "input_binding_equal": input_binding_equal,
        "final_state_equal": final_state_equal,
        "outcome_equal": outcome_equal,
        "replay_equal": replay_equal,
        "error_equal": error_equal,
        "resource_budget_ok": resource_budget_ok,
        "divergences": divergences,
        "authority_claims": {
            "participant_admission": false,
            "global_ordering": false,
            "canonical_roots": false,
            "completion_signing": false,
            "chain_finality": false,
            "cex_custody": false,
        },
        "limitations": [
            "Shadow equality is unsigned deterministic game-domain evidence only.",
            "It is not participant, ordering, archive, signature, finality or settlement evidence.",
            "An independent Nakama consumer and Integration component lock remain required."
        ],
    });
    canonical_json_bytes(&report)?;
    Ok(report)
}

fn validate_observation<'a>(
    value: &'a Value,
    label: &str,
) -> Result<ValidatedObservation<'a>, RuntimeError> {
    let observation = exact_object(value, &OBSERVATION_FIELDS, label)?;
    if observation
        .get("contract_version")
        .and_then(Value::as_str)
        != Some(RUNTIME_OBSERVATION_VERSION)
    {
        return Err(RuntimeError::new(
            "unsupported_contract",
            format!("unsupported {label} observation contract version"),
        ));
    }
    let implementation_id = observation
        .get("implementation_id")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RuntimeError::new(
                "invalid_contract",
                format!("{label}.implementation_id must be a string"),
            )
        })?;
    require_identifier(implementation_id, &format!("{label}.implementation_id"))?;
    let implementation_revision = observation
        .get("implementation_revision")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RuntimeError::new(
                "invalid_contract",
                format!("{label}.implementation_revision must be a string"),
            )
        })?;
    require_commit(
        implementation_revision,
        &format!("{label}.implementation_revision"),
    )?;
    let request_hash = observation
        .get("request_hash")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RuntimeError::new(
                "invalid_contract",
                format!("{label}.request_hash must be a string"),
            )
        })?;
    require_hex64(request_hash, &format!("{label}.request_hash"))?;
    let response = observation
        .get("response")
        .ok_or_else(|| RuntimeError::new("invalid_contract", format!("{label}.response missing")))?;
    let response_kind = validate_runtime_response(response)?;
    let duration_micros = non_negative_u64(
        observation.get("duration_micros"),
        &format!("{label}.duration_micros"),
    )?;
    let response_bytes = non_negative_u64(
        observation.get("response_bytes"),
        &format!("{label}.response_bytes"),
    )?;
    let actual_response_bytes = u64::try_from(canonical_json_bytes(response)?.len()).map_err(|_| {
        RuntimeError::new(
            "resource_limit_exceeded",
            format!("{label}.response byte count exceeds u64 range"),
        )
    })?;
    if response_bytes != actual_response_bytes {
        return Err(RuntimeError::new(
            "invalid_contract",
            format!(
                "{label}.response_bytes does not bind canonical response bytes"
            ),
        ));
    }
    Ok(ValidatedObservation {
        implementation_id,
        implementation_revision,
        request_hash,
        response,
        duration_micros,
        response_bytes,
        response_kind,
    })
}

fn validate_budgets(value: &Value) -> Result<ResourceBudgets, RuntimeError> {
    let budgets = exact_object(value, &BUDGET_FIELDS, "shadow budgets")?;
    let max_candidate_duration_micros = non_negative_u64(
        budgets.get("max_candidate_duration_micros"),
        "budgets.max_candidate_duration_micros",
    )?;
    let max_candidate_response_bytes = non_negative_u64(
        budgets.get("max_candidate_response_bytes"),
        "budgets.max_candidate_response_bytes",
    )?;
    if max_candidate_duration_micros == 0 || max_candidate_response_bytes == 0 {
        return Err(RuntimeError::new(
            "invalid_contract",
            "shadow budgets must be greater than zero",
        ));
    }
    Ok(ResourceBudgets {
        max_candidate_duration_micros,
        max_candidate_response_bytes,
    })
}

fn validate_runtime_response(value: &Value) -> Result<ResponseKind, RuntimeError> {
    reject_forbidden_authority_keys(value, "runtime response")?;
    canonical_json_bytes(value)?;
    let object = value.as_object().ok_or_else(|| {
        RuntimeError::new("invalid_contract", "runtime response must be an object")
    })?;
    match object.get("contract_version").and_then(Value::as_str) {
        Some(CONTRACT_VERSION) => {
            validate_execute_result(value)?;
            Ok(ResponseKind::Success)
        }
        Some(RUNTIME_ERROR_VERSION) => {
            validate_error(value)?;
            Ok(ResponseKind::Error)
        }
        _ => Err(RuntimeError::new(
            "unsupported_contract",
            "runtime response has an unsupported contract version",
        )),
    }
}

fn validate_execute_result(value: &Value) -> Result<(), RuntimeError> {
    let result = exact_object(value, &EXECUTE_RESULT_FIELDS, "execute result")?;
    if result.get("message_type").and_then(Value::as_str) != Some(EXECUTE_RESULT) {
        return Err(RuntimeError::new(
            "invalid_contract",
            "execute result message_type must be execute_result",
        ));
    }
    let ruleset = exact_object(
        result
            .get("ruleset")
            .ok_or_else(|| RuntimeError::new("invalid_contract", "ruleset is missing"))?,
        &RULESET_FIELDS,
        "result ruleset",
    )?;
    let ruleset_id = ruleset.get("id").and_then(Value::as_str).ok_or_else(|| {
        RuntimeError::new("invalid_contract", "ruleset.id must be a string")
    })?;
    let ruleset_version = ruleset
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RuntimeError::new("invalid_contract", "ruleset.version must be a string")
        })?;
    let ruleset_digest = ruleset
        .get("digest")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RuntimeError::new("invalid_contract", "ruleset.digest must be a string")
        })?;
    require_identifier(ruleset_id, "ruleset.id")?;
    require_identifier(ruleset_version, "ruleset.version")?;
    require_hex64(ruleset_digest, "ruleset.digest")?;
    for field in [
        "content_digest",
        "initial_state_hash",
        "command_batch_hash",
        "final_state_hash",
        "outcome_hash",
        "replay_material_hash",
    ] {
        let hash = result.get(field).and_then(Value::as_str).ok_or_else(|| {
            RuntimeError::new("invalid_contract", format!("{field} must be a string"))
        })?;
        require_hex64(hash, field)?;
    }
    verify_material_hash(result, "final_state", "final_state_hash", FINAL_STATE_DOMAIN)?;
    verify_material_hash(result, "outcome", "outcome_hash", OUTCOME_DOMAIN)?;
    verify_material_hash(
        result,
        "replay_material",
        "replay_material_hash",
        REPLAY_MATERIAL_DOMAIN,
    )?;
    Ok(())
}

fn validate_error(value: &Value) -> Result<(), RuntimeError> {
    let error = exact_object(value, &ERROR_FIELDS, "runtime error")?;
    let code = error
        .get("error_code")
        .and_then(Value::as_str)
        .ok_or_else(|| RuntimeError::new("invalid_contract", "error_code must be a string"))?;
    require_identifier(code, "error_code")?;
    let message = error.get("error").and_then(Value::as_str).ok_or_else(|| {
        RuntimeError::new("invalid_contract", "error must be a string")
    })?;
    if message.is_empty() || message.len() > 4096 {
        return Err(RuntimeError::new(
            "invalid_contract",
            "error message must contain 1..=4096 bytes",
        ));
    }
    if !error.get("recoverable").is_some_and(Value::is_boolean) {
        return Err(RuntimeError::new(
            "invalid_contract",
            "recoverable must be a boolean",
        ));
    }
    Ok(())
}

fn verify_material_hash(
    result: &Map<String, Value>,
    value_field: &str,
    hash_field: &str,
    domain: &str,
) -> Result<(), RuntimeError> {
    let material = result.get(value_field).ok_or_else(|| {
        RuntimeError::new("invalid_contract", format!("{value_field} is missing"))
    })?;
    let claimed = result.get(hash_field).and_then(Value::as_str).ok_or_else(|| {
        RuntimeError::new("invalid_contract", format!("{hash_field} is missing"))
    })?;
    let actual = domain_hash(domain, material)?;
    if actual != claimed {
        return Err(RuntimeError::new(
            "output_contract_violation",
            format!("{hash_field} does not bind {value_field}"),
        ));
    }
    Ok(())
}

fn compare_path(
    world: &Value,
    candidate: &Value,
    field: &str,
    code: &str,
    divergences: &mut Vec<Value>,
) -> bool {
    let world_value = world.get(field).cloned().unwrap_or(Value::Null);
    let candidate_value = candidate.get(field).cloned().unwrap_or(Value::Null);
    if world_value == candidate_value {
        true
    } else {
        divergences.push(divergence(code, field, world_value, candidate_value));
        false
    }
}

fn divergence(code: &str, path: &str, expected: Value, actual: Value) -> Value {
    json!({
        "code": code,
        "path": path,
        "expected": summarize_value(expected),
        "actual": summarize_value(actual),
    })
}

fn summarize_value(value: Value) -> Value {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => value,
        compound => {
            let hash = domain_hash("trnm.world.shadow.v1.divergence_value", &compound)
                .unwrap_or_else(|_| "unavailable".to_owned());
            json!({"canonical_hash": hash})
        }
    }
}

fn exact_object<'a>(
    value: &'a Value,
    expected: &[&str],
    label: &str,
) -> Result<&'a Map<String, Value>, RuntimeError> {
    let object = value.as_object().ok_or_else(|| {
        RuntimeError::new("invalid_contract", format!("{label} must be an object"))
    })?;
    for key in object.keys() {
        if !expected.contains(&key.as_str()) {
            let code = if is_forbidden_authority_field(key) {
                "authority_boundary_violation"
            } else {
                "invalid_contract"
            };
            return Err(RuntimeError::new(
                code,
                format!("unknown field in {label}: {key}"),
            ));
        }
    }
    for field in expected {
        if !object.contains_key(*field) {
            return Err(RuntimeError::new(
                "invalid_contract",
                format!("missing field in {label}: {field}"),
            ));
        }
    }
    Ok(object)
}

fn require_identifier(value: &str, label: &str) -> Result<(), RuntimeError> {
    let valid = (1..=128).contains(&value.len())
        && value.bytes().next().is_some_and(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit()
        })
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'_' | b':' | b'-')
        });
    if valid {
        Ok(())
    } else {
        Err(RuntimeError::new(
            "invalid_contract",
            format!("{label} is not a portable identifier"),
        ))
    }
}

fn require_commit(value: &str, label: &str) -> Result<(), RuntimeError> {
    if value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        Ok(())
    } else {
        Err(RuntimeError::new(
            "invalid_contract",
            format!("{label} must be lowercase 40-hex"),
        ))
    }
}

fn require_hex64(value: &str, label: &str) -> Result<(), RuntimeError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        Ok(())
    } else {
        Err(RuntimeError::new(
            "invalid_contract",
            format!("{label} must be lowercase 64-hex"),
        ))
    }
}

fn non_negative_u64(value: Option<&Value>, label: &str) -> Result<u64, RuntimeError> {
    value
        .and_then(Value::as_i64)
        .and_then(|value| u64::try_from(value).ok())
        .ok_or_else(|| {
            RuntimeError::new(
                "invalid_contract",
                format!("{label} must be a non-negative signed integer"),
            )
        })
}

fn signed_json_integer(value: u64, label: &str) -> Result<i64, RuntimeError> {
    i64::try_from(value).map_err(|_| {
        RuntimeError::new(
            "resource_limit_exceeded",
            format!("{label} exceeds signed 64-bit JSON range"),
        )
    })
}

fn is_forbidden_authority_field(field: &str) -> bool {
    FORBIDDEN_AUTHORITY_FIELDS.contains(&field)
}

fn reject_forbidden_authority_keys(value: &Value, label: &str) -> Result<(), RuntimeError> {
    match value {
        Value::Object(object) => {
            for (key, item) in object {
                if is_forbidden_authority_field(key) {
                    return Err(RuntimeError::new(
                        "authority_boundary_violation",
                        format!("{label} contains forbidden authority field {key}"),
                    ));
                }
                reject_forbidden_authority_keys(item, label)?;
            }
        }
        Value::Array(values) => {
            for item in values {
                reject_forbidden_authority_keys(item, label)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(domain: &str, value: &Value) -> String {
        domain_hash(domain, value).unwrap()
    }

    fn success_response(outcome_value: i64) -> Value {
        let final_state = json!({"tick": 2, "value": outcome_value});
        let outcome = json!({"terminal": true, "value": outcome_value});
        let replay = json!({"commands": 1, "final_tick": 2});
        json!({
            "contract_version": CONTRACT_VERSION,
            "message_type": EXECUTE_RESULT,
            "ruleset": {
                "id": "fixture",
                "version": "1",
                "digest": "1".repeat(64),
            },
            "content_digest": "2".repeat(64),
            "initial_state_hash": "3".repeat(64),
            "command_batch_hash": "4".repeat(64),
            "final_state": final_state,
            "final_state_hash": hash(FINAL_STATE_DOMAIN, &final_state),
            "outcome": outcome,
            "outcome_hash": hash(OUTCOME_DOMAIN, &outcome),
            "replay_material": replay,
            "replay_material_hash": hash(REPLAY_MATERIAL_DOMAIN, &replay),
        })
    }

    fn error_response(code: &str, message: &str) -> Value {
        json!({
            "contract_version": RUNTIME_ERROR_VERSION,
            "error_code": code,
            "error": message,
            "recoverable": false,
        })
    }

    fn request() -> Value {
        json!({
            "contract_version": CONTRACT_VERSION,
            "message_type": "execute_request",
            "ruleset": {
                "id": "fixture",
                "version": "1",
                "digest": "1".repeat(64),
            },
            "content_digest": "2".repeat(64),
            "initial_state": {"tick": 0, "value": 0},
            "commands": []
        })
    }

    fn observation(id: &str, revision: char, response: Value, duration: u64) -> Value {
        runtime_observation(
            id,
            &revision.to_string().repeat(40),
            &request(),
            response,
            duration,
        )
        .unwrap()
    }

    fn shadow_input(world: Value, candidate: Value) -> Value {
        json!({
            "contract_version": SHADOW_INPUT_VERSION,
            "world": world,
            "candidate": candidate,
            "budgets": {
                "max_candidate_duration_micros": 1000,
                "max_candidate_response_bytes": 100000,
            }
        })
    }

    #[test]
    fn identical_success_is_equivalent() {
        let response = success_response(7);
        let report = compare_shadow_value(&shadow_input(
            observation("world-rust", 'a', response.clone(), 10),
            observation("nakama-consumer", 'b', response, 11),
        ))
        .unwrap();
        assert_eq!(report["equivalent"], true);
        assert_eq!(report["input_binding_equal"], true);
        assert_eq!(report["final_state_equal"], true);
        assert_eq!(report["outcome_equal"], true);
        assert_eq!(report["replay_equal"], true);
        assert_eq!(report["resource_budget_ok"], true);
        assert_eq!(report["divergences"], json!([]));
    }

    #[test]
    fn outcome_divergence_is_typed_and_fail_closed() {
        let report = compare_shadow_value(&shadow_input(
            observation("world-rust", 'a', success_response(7), 10),
            observation("nakama-consumer", 'b', success_response(8), 11),
        ))
        .unwrap();
        assert_eq!(report["equivalent"], false);
        let codes = report["divergences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value["code"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert!(codes.contains(&"final_state_hash_mismatch"));
        assert!(codes.contains(&"outcome_hash_mismatch"));
    }

    #[test]
    fn copied_hash_over_tampered_material_is_rejected_before_comparison() {
        let world = success_response(7);
        let mut candidate = world.clone();
        candidate["outcome"]["value"] = Value::from(8);
        let input = shadow_input(
            observation("world-rust", 'a', world, 10),
            json!({
                "contract_version": RUNTIME_OBSERVATION_VERSION,
                "implementation_id": "nakama-consumer",
                "implementation_revision": "b".repeat(40),
                "request_hash": domain_hash(SHADOW_REQUEST_DOMAIN, &request()).unwrap(),
                "response_bytes": canonical_json_bytes(&candidate).unwrap().len(),
                "duration_micros": 11,
                "response": candidate,
            }),
        );
        let error = compare_shadow_value(&input).unwrap_err();
        assert_eq!(error.code(), "output_contract_violation");
        assert!(error.message().contains("outcome_hash"));
    }

    #[test]
    fn equivalent_errors_ignore_message_wording_but_bind_code_and_recoverability() {
        let report = compare_shadow_value(&shadow_input(
            observation(
                "world-rust",
                'a',
                error_response("invalid_game_command", "World wording"),
                10,
            ),
            observation(
                "nakama-consumer",
                'b',
                error_response("invalid_game_command", "Consumer wording"),
                11,
            ),
        ))
        .unwrap();
        assert_eq!(report["equivalent"], true);
        assert_eq!(report["error_equal"], true);
    }

    #[test]
    fn success_error_and_resource_budget_mismatches_fail_closed() {
        let mut input = shadow_input(
            observation("world-rust", 'a', success_response(7), 10),
            observation(
                "nakama-consumer",
                'b',
                error_response("invalid_game_command", "rejected"),
                2000,
            ),
        );
        input["budgets"]["max_candidate_duration_micros"] = Value::from(1000);
        let report = compare_shadow_value(&input).unwrap();
        assert_eq!(report["equivalent"], false);
        let codes = report["divergences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value["code"].as_str().unwrap())
            .collect::<Vec<_>>();
        assert!(codes.contains(&"execution_kind_mismatch"));
        assert!(codes.contains(&"candidate_duration_budget_exceeded"));
    }

    #[test]
    fn authority_material_is_rejected_from_shadow_responses() {
        let mut response = success_response(7);
        response["outcome"]["completion_signature"] = Value::String("forbidden".to_owned());
        let error = runtime_observation("world-rust", &"a".repeat(40), &request(), response, 10).unwrap_err();
        assert_eq!(error.code(), "authority_boundary_violation");
    }

    #[test]
    fn response_bytes_are_exactly_bound() {
        let response = success_response(7);
        let mut observation = runtime_observation(
            "world-rust",
            &"a".repeat(40),
            &request(),
            response,
            10,
        )
        .unwrap();
        observation["response_bytes"] = Value::from(1);
        let error = compare_shadow_value(&shadow_input(
            observation,
            runtime_observation(
                "nakama-consumer",
                &"b".repeat(40),
                &request(),
                success_response(7),
                11,
            )
            .unwrap(),
        ))
        .unwrap_err();
        assert_eq!(error.code(), "invalid_contract");
        assert!(error.message().contains("response_bytes"));
    }

    #[test]
    fn command_batch_domain_remains_owned_by_runtime_contract() {
        let value = json!([]);
        assert_eq!(
            domain_hash(trnm_world_runtime_adapter::COMMAND_BATCH_DOMAIN, &value).unwrap().len(),
            64
        );
    }
}
