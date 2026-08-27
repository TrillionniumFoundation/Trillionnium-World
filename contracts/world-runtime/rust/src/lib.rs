#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]

//! Unsigned deterministic execution boundary for `trnm_world_runtime_v1`.
//!
//! This crate owns only World game-domain validation, canonicalization,
//! hashing and deterministic ruleset execution. It deliberately has no
//! networking, persistence, participant authority, global ordering, signing,
//! Chain-finality or wallet-custody capability.

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};
use trnm_rts_protocol::RtsFrameOrder;
use trnm_rts_sim::MissionSimV1;
use unicode_normalization::UnicodeNormalization;

pub const CONTRACT_VERSION: &str = "trnm_world_runtime_v1";
pub const EXECUTE_REQUEST: &str = "execute_request";
pub const EXECUTE_RESULT: &str = "execute_result";
pub const MAX_CANONICAL_DEPTH: usize = 64;
pub const MAX_CANONICAL_NODES: usize = 100_000;
pub const MAX_CANONICAL_BYTES: usize = 16 * 1024 * 1024;
pub const DEFAULT_MAX_ADVANCE_STEPS: u64 = 10_000;

pub const INITIAL_STATE_DOMAIN: &str = "trnm.world.runtime.v1.initial_state";
pub const COMMAND_BATCH_DOMAIN: &str = "trnm.world.runtime.v1.command_batch";
pub const FINAL_STATE_DOMAIN: &str = "trnm.world.runtime.v1.final_state";
pub const OUTCOME_DOMAIN: &str = "trnm.world.runtime.v1.outcome";
pub const REPLAY_MATERIAL_DOMAIN: &str = "trnm.world.runtime.v1.replay_material";

const REQUEST_FIELDS: [&str; 6] = [
    "contract_version",
    "message_type",
    "ruleset",
    "content_digest",
    "initial_state",
    "commands",
];
const RULESET_FIELDS: [&str; 3] = ["id", "version", "digest"];
const COMMAND_FIELDS: [&str; 3] = ["batch_ordinal", "kind", "payload"];
const ADVANCE_FIELDS: [&str; 1] = ["target_tick"];
const FORBIDDEN_AUTHORITY_FIELDS: [&str; 11] = [
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
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeError {
    code: &'static str,
    message: String,
}

impl RuntimeError {
    #[must_use]
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn as_json(&self) -> Value {
        json!({
            "contract_version": "trnm_world_runtime_error_v1",
            "error_code": self.code,
            "error": self.message,
            "recoverable": false,
        })
    }
}

impl Display for RuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RuntimeError {}

struct StrictJson(Value);

impl<'de> Deserialize<'de> for StrictJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}

struct StrictJsonVisitor;

impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJson;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("strict canonical JSON without duplicate keys or floats")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictJson(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictJson(Value::Number(Number::from(value))))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = i64::try_from(value)
            .map_err(|_| E::custom("integer is outside signed 64-bit range"))?;
        Ok(StrictJson(Value::Number(Number::from(value))))
    }

    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Err(E::custom("floating-point numbers are forbidden"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(StrictJson(Value::String(value.nfc().collect())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictJson(Value::String(value.nfc().collect())))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJson(Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJson(Value::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        StrictJson::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(StrictJson(value)) = sequence.next_element::<StrictJson>()? {
            values.push(value);
        }
        Ok(StrictJson(Value::Array(values)))
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut raw_keys = BTreeSet::new();
        let mut normalized_keys = BTreeSet::new();
        let mut values = Map::new();
        while let Some(raw_key) = access.next_key::<String>()? {
            if !raw_keys.insert(raw_key.clone()) {
                return Err(de::Error::custom(format!(
                    "duplicate object key: {raw_key}"
                )));
            }
            let normalized_key: String = raw_key.nfc().collect();
            if !normalized_keys.insert(normalized_key.clone()) {
                return Err(de::Error::custom(format!(
                    "normalized object key collision: {normalized_key}"
                )));
            }
            let StrictJson(value) = access.next_value::<StrictJson>()?;
            values.insert(normalized_key, value);
        }
        Ok(StrictJson(Value::Object(values)))
    }
}

pub fn parse_strict_json(input: &str) -> Result<Value, RuntimeError> {
    if input.len() > MAX_CANONICAL_BYTES {
        return Err(RuntimeError::new(
            "resource_limit_exceeded",
            format!("input exceeds {MAX_CANONICAL_BYTES} bytes"),
        ));
    }
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let StrictJson(value) = StrictJson::deserialize(&mut deserializer).map_err(|error| {
        RuntimeError::new("invalid_canonical_json", error.to_string())
    })?;
    deserializer.end().map_err(|error| {
        RuntimeError::new("invalid_canonical_json", error.to_string())
    })?;
    canonical_json_bytes(&value)?;
    Ok(value)
}

#[derive(Default)]
struct CanonicalBudget {
    nodes: usize,
}

impl CanonicalBudget {
    fn visit(&mut self, depth: usize) -> Result<(), RuntimeError> {
        if depth > MAX_CANONICAL_DEPTH {
            return Err(RuntimeError::new(
                "resource_limit_exceeded",
                format!("canonical depth exceeds {MAX_CANONICAL_DEPTH}"),
            ));
        }
        self.nodes = self.nodes.saturating_add(1);
        if self.nodes > MAX_CANONICAL_NODES {
            return Err(RuntimeError::new(
                "resource_limit_exceeded",
                format!("canonical node count exceeds {MAX_CANONICAL_NODES}"),
            ));
        }
        Ok(())
    }
}

pub fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, RuntimeError> {
    let mut output = String::new();
    let mut budget = CanonicalBudget::default();
    write_canonical(value, &mut output, &mut budget, 0)?;
    if output.len() > MAX_CANONICAL_BYTES {
        return Err(RuntimeError::new(
            "resource_limit_exceeded",
            format!("canonical output exceeds {MAX_CANONICAL_BYTES} bytes"),
        ));
    }
    Ok(output.into_bytes())
}

fn write_canonical(
    value: &Value,
    output: &mut String,
    budget: &mut CanonicalBudget,
    depth: usize,
) -> Result<(), RuntimeError> {
    budget.visit(depth)?;
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
        Value::Number(number) => {
            let value = number.as_i64().ok_or_else(|| {
                RuntimeError::new(
                    "invalid_canonical_json",
                    "numbers must be signed 64-bit integers",
                )
            })?;
            output.push_str(&value.to_string());
        }
        Value::String(value) => {
            let normalized: String = value.nfc().collect();
            output.push_str(&serde_json::to_string(&normalized).map_err(|error| {
                RuntimeError::new("invalid_canonical_json", error.to_string())
            })?);
        }
        Value::Array(values) => {
            output.push('[');
            for (index, item) in values.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                write_canonical(item, output, budget, depth.saturating_add(1))?;
            }
            output.push(']');
        }
        Value::Object(values) => {
            let mut normalized = Vec::with_capacity(values.len());
            let mut keys = BTreeSet::new();
            for (raw_key, item) in values {
                let key: String = raw_key.nfc().collect();
                if !keys.insert(key.clone()) {
                    return Err(RuntimeError::new(
                        "invalid_canonical_json",
                        format!("normalized object key collision: {key}"),
                    ));
                }
                normalized.push((key, item));
            }
            normalized.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            output.push('{');
            for (index, (key, item)) in normalized.into_iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                output.push_str(&serde_json::to_string(&key).map_err(|error| {
                    RuntimeError::new("invalid_canonical_json", error.to_string())
                })?);
                output.push(':');
                write_canonical(item, output, budget, depth.saturating_add(1))?;
            }
            output.push('}');
        }
    }
    Ok(())
}

pub fn domain_hash(domain: &str, value: &Value) -> Result<String, RuntimeError> {
    if domain.is_empty() || !domain.is_ascii() || domain.contains('\n') {
        return Err(RuntimeError::new(
            "output_contract_violation",
            "hash domain must be non-empty, single-line ASCII",
        ));
    }
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    hasher.update(b"\n");
    hasher.update(canonical_json_bytes(value)?);
    Ok(format!("{digest:x}", digest = hasher.finalize()))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeSelection {
    ruleset_id: String,
    ruleset_version: String,
    ruleset_digest: String,
    content_digest: String,
}

impl RuntimeSelection {
    pub fn new(
        ruleset_id: impl Into<String>,
        ruleset_version: impl Into<String>,
        ruleset_digest: impl Into<String>,
        content_digest: impl Into<String>,
    ) -> Result<Self, RuntimeError> {
        let selection = Self {
            ruleset_id: ruleset_id.into(),
            ruleset_version: ruleset_version.into(),
            ruleset_digest: ruleset_digest.into(),
            content_digest: content_digest.into(),
        };
        require_identifier(&selection.ruleset_id, "ruleset.id")?;
        require_identifier(&selection.ruleset_version, "ruleset.version")?;
        require_hex64(&selection.ruleset_digest, "ruleset.digest")?;
        require_hex64(&selection.content_digest, "content_digest")?;
        Ok(selection)
    }

    #[must_use]
    pub fn ruleset_id(&self) -> &str {
        &self.ruleset_id
    }

    #[must_use]
    pub fn ruleset_version(&self) -> &str {
        &self.ruleset_version
    }

    #[must_use]
    pub fn ruleset_digest(&self) -> &str {
        &self.ruleset_digest
    }

    #[must_use]
    pub fn content_digest(&self) -> &str {
        &self.content_digest
    }

    #[must_use]
    pub fn ruleset_value(&self) -> Value {
        json!({
            "id": self.ruleset_id,
            "version": self.ruleset_version,
            "digest": self.ruleset_digest,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeCommandV1 {
    batch_ordinal: u64,
    kind: String,
    payload: Value,
    canonical_value: Value,
}

impl RuntimeCommandV1 {
    #[must_use]
    pub const fn batch_ordinal(&self) -> u64 {
        self.batch_ordinal
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    #[must_use]
    pub const fn payload(&self) -> &Value {
        &self.payload
    }

    #[must_use]
    pub const fn canonical_value(&self) -> &Value {
        &self.canonical_value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldExecutionMaterial {
    pub final_state: Value,
    pub outcome: Value,
    pub replay_material: Value,
}

impl WorldExecutionMaterial {
    #[must_use]
    pub const fn new(final_state: Value, outcome: Value, replay_material: Value) -> Self {
        Self {
            final_state,
            outcome,
            replay_material,
        }
    }
}

pub trait WorldRulesetExecutor: Send + Sync {
    fn execute(
        &self,
        initial_state: &Value,
        commands: &[RuntimeCommandV1],
    ) -> Result<WorldExecutionMaterial, RuntimeError>;
}

pub struct WorldRuntimeV1<E> {
    selection: RuntimeSelection,
    executor: E,
}

impl<E> WorldRuntimeV1<E>
where
    E: WorldRulesetExecutor,
{
    #[must_use]
    pub const fn new(selection: RuntimeSelection, executor: E) -> Self {
        Self {
            selection,
            executor,
        }
    }

    pub fn execute_json(&self, input: &str) -> Result<String, RuntimeError> {
        let request = parse_strict_json(input)?;
        let result = self.execute_value(&request)?;
        String::from_utf8(canonical_json_bytes(&result)?).map_err(|error| {
            RuntimeError::new("output_contract_violation", error.to_string())
        })
    }

    pub fn execute_value(&self, request: &Value) -> Result<Value, RuntimeError> {
        let validated = validate_request(request, &self.selection)?;
        let command_values = Value::Array(
            validated
                .commands
                .iter()
                .map(|command| command.canonical_value.clone())
                .collect(),
        );
        let initial_state_hash = domain_hash(INITIAL_STATE_DOMAIN, &validated.initial_state)?;
        let command_batch_hash = domain_hash(COMMAND_BATCH_DOMAIN, &command_values)?;
        let material = self
            .executor
            .execute(&validated.initial_state, &validated.commands)?;
        reject_forbidden_authority_keys(&material.final_state, "final_state")?;
        reject_forbidden_authority_keys(&material.outcome, "outcome")?;
        reject_forbidden_authority_keys(&material.replay_material, "replay_material")?;
        let final_state_hash = domain_hash(FINAL_STATE_DOMAIN, &material.final_state)?;
        let outcome_hash = domain_hash(OUTCOME_DOMAIN, &material.outcome)?;
        let replay_material_hash =
            domain_hash(REPLAY_MATERIAL_DOMAIN, &material.replay_material)?;
        let result = json!({
            "contract_version": CONTRACT_VERSION,
            "message_type": EXECUTE_RESULT,
            "ruleset": self.selection.ruleset_value(),
            "content_digest": self.selection.content_digest,
            "initial_state_hash": initial_state_hash,
            "command_batch_hash": command_batch_hash,
            "final_state": material.final_state,
            "final_state_hash": final_state_hash,
            "outcome": material.outcome,
            "outcome_hash": outcome_hash,
            "replay_material": material.replay_material,
            "replay_material_hash": replay_material_hash,
        });
        canonical_json_bytes(&result)?;
        Ok(result)
    }
}

struct ValidatedRequest {
    initial_state: Value,
    commands: Vec<RuntimeCommandV1>,
}

fn validate_request(
    request: &Value,
    selection: &RuntimeSelection,
) -> Result<ValidatedRequest, RuntimeError> {
    let request = exact_object(request, &REQUEST_FIELDS, "execute request")?;
    if request.get("contract_version").and_then(Value::as_str) != Some(CONTRACT_VERSION) {
        return Err(RuntimeError::new(
            "unsupported_contract",
            "unsupported World runtime contract version",
        ));
    }
    if request.get("message_type").and_then(Value::as_str) != Some(EXECUTE_REQUEST) {
        return Err(RuntimeError::new(
            "invalid_contract",
            "message_type must be execute_request",
        ));
    }
    let ruleset = exact_object(
        request
            .get("ruleset")
            .ok_or_else(|| RuntimeError::new("invalid_contract", "ruleset is missing"))?,
        &RULESET_FIELDS,
        "ruleset",
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
    let content_digest = request
        .get("content_digest")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RuntimeError::new("invalid_contract", "content_digest must be a string")
        })?;
    require_identifier(ruleset_id, "ruleset.id")?;
    require_identifier(ruleset_version, "ruleset.version")?;
    require_hex64(ruleset_digest, "ruleset.digest")?;
    require_hex64(content_digest, "content_digest")?;
    if ruleset_id != selection.ruleset_id
        || ruleset_version != selection.ruleset_version
        || ruleset_digest != selection.ruleset_digest
    {
        return Err(RuntimeError::new(
            "ruleset_unavailable",
            "request ruleset does not match the installed exact selection",
        ));
    }
    if content_digest != selection.content_digest {
        return Err(RuntimeError::new(
            "content_unavailable",
            "request content digest does not match the installed exact content",
        ));
    }

    let initial_state = request
        .get("initial_state")
        .cloned()
        .ok_or_else(|| RuntimeError::new("invalid_contract", "initial_state is missing"))?;
    reject_forbidden_authority_keys(&initial_state, "initial_state")?;
    canonical_json_bytes(&initial_state)?;

    let command_values = request
        .get("commands")
        .and_then(Value::as_array)
        .ok_or_else(|| RuntimeError::new("invalid_contract", "commands must be an array"))?;
    if command_values.len() > MAX_CANONICAL_NODES {
        return Err(RuntimeError::new(
            "resource_limit_exceeded",
            "command count exceeds the canonical node budget",
        ));
    }
    let mut commands = Vec::with_capacity(command_values.len());
    for (expected_ordinal, raw_command) in command_values.iter().enumerate() {
        let command = exact_object(raw_command, &COMMAND_FIELDS, "command")?;
        let ordinal = command
            .get("batch_ordinal")
            .and_then(Value::as_i64)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| {
                RuntimeError::new(
                    "ordinal_discontinuity",
                    "batch_ordinal must be a non-negative signed integer",
                )
            })?;
        let expected_ordinal = u64::try_from(expected_ordinal).map_err(|_| {
            RuntimeError::new(
                "resource_limit_exceeded",
                "command ordinal exceeds u64 range",
            )
        })?;
        if ordinal != expected_ordinal {
            return Err(RuntimeError::new(
                "ordinal_discontinuity",
                "command ordinals must be contiguous from zero",
            ));
        }
        let kind = command
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(|| RuntimeError::new("invalid_game_command", "kind must be a string"))?;
        require_identifier(kind, "command.kind")?;
        let payload = command
            .get("payload")
            .cloned()
            .ok_or_else(|| RuntimeError::new("invalid_game_command", "payload is missing"))?;
        reject_forbidden_authority_keys(&payload, "command payload")?;
        canonical_json_bytes(&payload)?;
        commands.push(RuntimeCommandV1 {
            batch_ordinal: ordinal,
            kind: kind.to_owned(),
            payload,
            canonical_value: raw_command.clone(),
        });
    }
    canonical_json_bytes(&Value::Object(request.clone()))?;
    Ok(ValidatedRequest {
        initial_state,
        commands,
    })
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

#[derive(Clone, Debug)]
pub struct RtsMissionExecutor {
    max_advance_steps: u64,
}

impl Default for RtsMissionExecutor {
    fn default() -> Self {
        Self {
            max_advance_steps: DEFAULT_MAX_ADVANCE_STEPS,
        }
    }
}

impl RtsMissionExecutor {
    pub fn new(max_advance_steps: u64) -> Result<Self, RuntimeError> {
        if max_advance_steps == 0 || max_advance_steps > 1_000_000 {
            return Err(RuntimeError::new(
                "resource_limit_exceeded",
                "max_advance_steps must be in 1..=1,000,000",
            ));
        }
        Ok(Self { max_advance_steps })
    }

    fn advance_to(
        &self,
        simulation: &mut MissionSimV1,
        target_tick: u64,
    ) -> Result<(), RuntimeError> {
        if target_tick < simulation.tick {
            return Err(RuntimeError::new(
                "invalid_game_command",
                "target tick regresses deterministic World state",
            ));
        }
        let steps = target_tick.saturating_sub(simulation.tick);
        if steps > self.max_advance_steps {
            return Err(RuntimeError::new(
                "resource_limit_exceeded",
                format!(
                    "command requires {steps} simulation steps, exceeding {}",
                    self.max_advance_steps
                ),
            ));
        }
        while simulation.tick < target_tick {
            if simulation.terminal() {
                return Err(RuntimeError::new(
                    "deterministic_execution_failed",
                    "simulation became terminal before the requested target tick",
                ));
            }
            simulation.step().map_err(|error| {
                RuntimeError::new(
                    "deterministic_execution_failed",
                    format!("deterministic RTS step failed: {error}"),
                )
            })?;
        }
        Ok(())
    }

    fn advance_payload_tick(payload: &Value) -> Result<u64, RuntimeError> {
        let payload = exact_object(payload, &ADVANCE_FIELDS, "advance payload")?;
        payload
            .get("target_tick")
            .and_then(Value::as_i64)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| {
                RuntimeError::new(
                    "invalid_game_command",
                    "advance target_tick must be a non-negative signed integer",
                )
            })
    }
}

impl WorldRulesetExecutor for RtsMissionExecutor {
    fn execute(
        &self,
        initial_state: &Value,
        commands: &[RuntimeCommandV1],
    ) -> Result<WorldExecutionMaterial, RuntimeError> {
        let mut simulation: MissionSimV1 = serde_json::from_value(initial_state.clone()).map_err(
            |error| {
                RuntimeError::new(
                    "invalid_game_state",
                    format!("decode MissionSimV1 initial state: {error}"),
                )
            },
        )?;
        for command in commands {
            match command.kind() {
                "rts_order_v1" => {
                    let order: RtsFrameOrder = serde_json::from_value(command.payload().clone())
                        .map_err(|error| {
                            RuntimeError::new(
                                "invalid_game_command",
                                format!("decode RtsFrameOrder: {error}"),
                            )
                        })?;
                    self.advance_to(&mut simulation, u64::from(order.frame))?;
                    simulation.issue_order(order).map_err(|error| {
                        RuntimeError::new(
                            "invalid_game_command",
                            format!("apply deterministic RTS order: {error}"),
                        )
                    })?;
                }
                "advance_to_tick_v1" => {
                    let target_tick = Self::advance_payload_tick(command.payload())?;
                    self.advance_to(&mut simulation, target_tick)?;
                }
                kind => {
                    return Err(RuntimeError::new(
                        "invalid_game_command",
                        format!("unsupported World RTS command kind: {kind}"),
                    ));
                }
            }
        }

        let terminal = simulation.terminal();
        let tick = simulation.tick;
        let simulation_snapshot_hash = simulation.snapshot_hash().map_err(|error| {
            RuntimeError::new(
                "output_contract_violation",
                format!("hash final MissionSimV1 state: {error}"),
            )
        })?;
        let outcome = if terminal {
            let battle_result = simulation.clone().into_result().map_err(|error| {
                RuntimeError::new(
                    "output_contract_violation",
                    format!("derive terminal battle result: {error}"),
                )
            })?;
            json!({
                "terminal": true,
                "tick": tick,
                "simulation_snapshot_hash": simulation_snapshot_hash,
                "battle_result": battle_result,
            })
        } else {
            json!({
                "terminal": false,
                "tick": tick,
                "simulation_snapshot_hash": simulation_snapshot_hash,
            })
        };
        let final_state = serde_json::to_value(&simulation).map_err(|error| {
            RuntimeError::new(
                "output_contract_violation",
                format!("encode final MissionSimV1 state: {error}"),
            )
        })?;
        let replay_material = json!({
            "contract_version": "trnm_world_replay_material_v1",
            "applied_command_count": commands.len(),
            "final_tick": tick,
            "terminal": terminal,
            "simulation_snapshot_hash": simulation_snapshot_hash,
        });
        Ok(WorldExecutionMaterial::new(
            final_state,
            outcome,
            replay_material,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    struct CounterExecutor;

    impl WorldRulesetExecutor for CounterExecutor {
        fn execute(
            &self,
            initial_state: &Value,
            commands: &[RuntimeCommandV1],
        ) -> Result<WorldExecutionMaterial, RuntimeError> {
            let mut value = initial_state
                .get("value")
                .and_then(Value::as_i64)
                .ok_or_else(|| {
                    RuntimeError::new("invalid_game_state", "counter value is missing")
                })?;
            for command in commands {
                if command.kind() != "add_v1" {
                    return Err(RuntimeError::new(
                        "invalid_game_command",
                        "counter supports add_v1 only",
                    ));
                }
                let delta = command
                    .payload()
                    .get("delta")
                    .and_then(Value::as_i64)
                    .ok_or_else(|| {
                        RuntimeError::new("invalid_game_command", "delta is missing")
                    })?;
                value = value.checked_add(delta).ok_or_else(|| {
                    RuntimeError::new("deterministic_execution_failed", "counter overflow")
                })?;
            }
            Ok(WorldExecutionMaterial::new(
                json!({"value": value}),
                json!({"terminal": true, "value": value}),
                json!({"applied_command_count": commands.len()}),
            ))
        }
    }

    fn selection() -> RuntimeSelection {
        RuntimeSelection::new("counter", "1", "1".repeat(64), "2".repeat(64)).unwrap()
    }

    fn request(commands: Value) -> Value {
        json!({
            "contract_version": CONTRACT_VERSION,
            "message_type": EXECUTE_REQUEST,
            "ruleset": {
                "id": "counter",
                "version": "1",
                "digest": "1".repeat(64),
            },
            "content_digest": "2".repeat(64),
            "initial_state": {"value": 1},
            "commands": commands,
        })
    }

    #[test]
    fn canonical_object_order_matches_fixed_sha_vector() {
        let value = json!({"b": 2, "a": 1});
        let canonical = canonical_json_bytes(&value).unwrap();
        assert_eq!(canonical, br#"{"a":1,"b":2}"#);
        assert_eq!(
            format!("{:x}", Sha256::digest(&canonical)),
            "43258cff783fe7036d8a43033f830adfc60ec037382473548ac742b888292777"
        );
    }

    #[test]
    fn strict_parser_rejects_duplicate_float_and_normalized_key_collision() {
        for (input, expected) in [
            (r#"{"a":1,"a":2}"#, "duplicate object key"),
            (r#"{"value":1.0}"#, "floating-point numbers are forbidden"),
            (r#"{"é":1,"é":2}"#, "normalized object key collision"),
        ] {
            let error = parse_strict_json(input).unwrap_err();
            assert_eq!(error.code(), "invalid_canonical_json");
            assert!(error.message().contains(expected));
        }
    }

    #[test]
    fn execute_binds_unsigned_game_material_and_preserves_authority_boundary() {
        let runtime = WorldRuntimeV1::new(selection(), CounterExecutor);
        let result = runtime
            .execute_value(&request(json!([
                {"batch_ordinal": 0, "kind": "add_v1", "payload": {"delta": 2}},
                {"batch_ordinal": 1, "kind": "add_v1", "payload": {"delta": 3}}
            ])))
            .unwrap();
        assert_eq!(result["final_state"], json!({"value": 6}));
        assert_eq!(result["outcome"]["value"], 6);
        for field in [
            "initial_state_hash",
            "command_batch_hash",
            "final_state_hash",
            "outcome_hash",
            "replay_material_hash",
        ] {
            let hash = result[field].as_str().unwrap();
            assert_eq!(hash.len(), 64);
            assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
        for forbidden in FORBIDDEN_AUTHORITY_FIELDS {
            assert!(result.get(forbidden).is_none());
        }
    }

    #[test]
    fn command_tampering_changes_the_command_batch_hash() {
        let runtime = WorldRuntimeV1::new(selection(), CounterExecutor);
        let first = runtime
            .execute_value(&request(json!([
                {"batch_ordinal": 0, "kind": "add_v1", "payload": {"delta": 2}}
            ])))
            .unwrap();
        let second = runtime
            .execute_value(&request(json!([
                {"batch_ordinal": 0, "kind": "add_v1", "payload": {"delta": 3}}
            ])))
            .unwrap();
        assert_ne!(first["command_batch_hash"], second["command_batch_hash"]);
        assert_ne!(first["final_state_hash"], second["final_state_hash"]);
    }

    #[test]
    fn boolean_or_gapped_ordinals_fail_closed() {
        let runtime = WorldRuntimeV1::new(selection(), CounterExecutor);
        for commands in [
            json!([{"batch_ordinal": false, "kind": "add_v1", "payload": {"delta": 1}}]),
            json!([
                {"batch_ordinal": 0, "kind": "add_v1", "payload": {"delta": 1}},
                {"batch_ordinal": 2, "kind": "add_v1", "payload": {"delta": 1}}
            ]),
        ] {
            assert_eq!(
                runtime.execute_value(&request(commands)).unwrap_err().code(),
                "ordinal_discontinuity"
            );
        }
    }

    #[test]
    fn installed_ruleset_and_content_are_exact() {
        let runtime = WorldRuntimeV1::new(selection(), CounterExecutor);
        let mut wrong_ruleset = request(json!([]));
        wrong_ruleset["ruleset"]["digest"] = Value::String("3".repeat(64));
        assert_eq!(
            runtime.execute_value(&wrong_ruleset).unwrap_err().code(),
            "ruleset_unavailable"
        );
        let mut wrong_content = request(json!([]));
        wrong_content["content_digest"] = Value::String("4".repeat(64));
        assert_eq!(
            runtime.execute_value(&wrong_content).unwrap_err().code(),
            "content_unavailable"
        );
    }

    #[test]
    fn authority_fields_are_rejected_even_when_nested_in_game_material() {
        let runtime = WorldRuntimeV1::new(selection(), CounterExecutor);
        let error = runtime
            .execute_value(&request(json!([
                {
                    "batch_ordinal": 0,
                    "kind": "add_v1",
                    "payload": {"delta": 1, "completion_signature": "forbidden"}
                }
            ])))
            .unwrap_err();
        assert_eq!(error.code(), "authority_boundary_violation");
    }

    #[test]
    fn production_rts_executor_implements_the_contract_trait() {
        fn assert_executor<T: WorldRulesetExecutor>() {}
        assert_executor::<RtsMissionExecutor>();
        assert!(RtsMissionExecutor::new(0).is_err());
        assert!(RtsMissionExecutor::new(10_000).is_ok());
    }
}
