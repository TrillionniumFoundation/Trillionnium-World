//! Opt-in structural World state / command intake. No authority or persistence.
//!
//! Versioned separately from legacy Serde readers. Arrays retain their order;
//! this is NOT a canonical encoder, hash verifier, gameplay command executor,
//! session guard, ledger verifier or production repository.
use super::intake::StrictValue;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;
use trnm_world_command::WorldCommand;
use trnm_world_domain::{WorldState, WORLD_DOMAIN_CONTRACT};

pub const WORLD_STATE_INTAKE_CONTRACT: &str = "trillionnium_world_state_intake_v1";
pub const MAX_WORLD_STATE_INPUT_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_WORLD_COMMAND_INPUT_BYTES: usize = 128 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorldStateIntakeError {
    InvalidJson,
    InvalidShape,
    ResourceLimit,
    UnsupportedContract,
    DuplicateIdentity,
    MissingReference,
}
impl std::fmt::Display for WorldStateIntakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("World intake rejected")
    }
}
impl std::error::Error for WorldStateIntakeError {}
type IntakeResult<T> = Result<T, WorldStateIntakeError>;

#[derive(Clone, Copy)]
enum Rule {
    Text(usize, bool),
    Integer(i64, i64),
    Boolean,
    List(&'static str, usize),
}
// (field name, required, validation rule). Optional fields retain Serde defaults.
type Field = (&'static str, bool, Rule);
const ID: Rule = Rule::Text(160, false);
const NODE: &[Field] = &[
    ("id", true, ID), ("name", true, Rule::Text(512, false)),
    ("region", true, ID), ("location_id", false, Rule::Text(160, true)),
    ("node_kind", false, Rule::Text(160, true)),
    ("description", false, Rule::Text(4096, true)),
    ("status", false, Rule::Text(160, true)),
    ("lat_e7", true, Rule::Integer(-900_000_000, 900_000_000)),
    ("lng_e7", true, Rule::Integer(-1_800_000_000, 1_800_000_000)),
    ("tags", false, Rule::List("text", 32)),
];
const EDGE: &[Field] = &[("from", true, ID), ("to", true, ID), ("direction", true, ID)];
const POSITION: &[Field] = &[
    ("actor_id", true, ID), ("node_id", true, ID), ("source_of_truth", true, ID),
];
const NPC: &[Field] = &[
    ("id", true, ID), ("name", true, Rule::Text(512, false)), ("node_id", true, ID),
    ("teaches_skills", false, Rule::List("text", 64)),
];
const TASK: &[Field] = &[
    ("id", true, ID), ("title", true, Rule::Text(512, false)), ("node_id", true, ID),
    ("reward_units", true, Rule::Integer(0, i64::MAX)),
    ("ledger_settlement_required", true, Rule::Boolean),
];
const RECEIPT: &[Field] = &[
    ("id", true, ID), ("progression_class", true, ID), ("status", true, ID),
];
const STATE: &[Field] = &[
    ("contract_version", true, ID), ("source", true, ID),
    ("nodes", true, Rule::List("node", 4096)),
    ("edges", true, Rule::List("edge", 16384)),
    ("positions", true, Rule::List("position", 4096)),
    ("npcs", true, Rule::List("npc", 4096)),
    ("tasks", true, Rule::List("task", 8192)),
    ("receipts", false, Rule::List("receipt", 8192)),
];

fn bounded_text(value: &Value, cap: usize, empty: bool) -> IntakeResult<()> {
    let text = value.as_str().ok_or(WorldStateIntakeError::InvalidShape)?;
    if text.len() > cap { return Err(WorldStateIntakeError::ResourceLimit); }
    if (!empty && text.is_empty()) || text.trim() != text || text.chars().any(char::is_control) {
        return Err(WorldStateIntakeError::InvalidShape);
    }
    Ok(())
}

fn record(value: &Value, fields: &[Field]) -> IntakeResult<()> {
    let obj = value.as_object().ok_or(WorldStateIntakeError::InvalidShape)?;
    if obj.keys().any(|k| !fields.iter().any(|(name, _, _)| k.as_str() == *name))
        || fields.iter().any(|(name, required, _)| *required && !obj.contains_key(*name)) {
        return Err(WorldStateIntakeError::InvalidShape);
    }
    for (name, _, rule) in fields {
        let Some(value) = obj.get(*name) else { continue; };
        match rule {
            Rule::Text(cap, empty) => bounded_text(value, *cap, *empty)?,
            Rule::Boolean => {
                if !value.is_boolean() { return Err(WorldStateIntakeError::InvalidShape); }
            }
            Rule::Integer(low, high) => {
                let n = value.as_number().ok_or(WorldStateIntakeError::InvalidShape)?;
                let number = n.as_i64().ok_or(WorldStateIntakeError::ResourceLimit)?;
                if number < *low || number > *high { return Err(WorldStateIntakeError::ResourceLimit); }
            }
            Rule::List(kind, cap) => {
                let list = value.as_array().ok_or(WorldStateIntakeError::InvalidShape)?;
                if list.len() > *cap { return Err(WorldStateIntakeError::ResourceLimit); }
                for item in list {
                    match *kind {
                        "text" => bounded_text(item, 160, false)?,
                        "node" => record(item, NODE)?, "edge" => record(item, EDGE)?,
                        "position" => record(item, POSITION)?, "npc" => record(item, NPC)?,
                        "task" => record(item, TASK)?, "receipt" => record(item, RECEIPT)?,
                        _ => return Err(WorldStateIntakeError::InvalidShape),
                    }
                }
            }
        }
    }
    Ok(())
}

fn raw_value(bytes: &[u8], limit: usize) -> IntakeResult<Value> {
    if bytes.len() > limit { return Err(WorldStateIntakeError::ResourceLimit); }
    // A quote/escape-aware depth bound is only a budget check. Serde still
    // validates the full grammar, UTF-8, escapes, unique keys and integer lexemes.
    let mut depth = 0_i32;
    let mut quoted = false;
    let mut escaped = false;
    for byte in bytes {
        if quoted {
            if escaped { escaped = false; }
            else if *byte == b'\\' { escaped = true; }
            else if *byte == b'"' { quoted = false; }
        } else if *byte == b'"' { quoted = true; }
        else if matches!(byte, b'[' | b'{') {
            depth += 1;
            if depth > 32 { return Err(WorldStateIntakeError::InvalidJson); }
        } else if matches!(byte, b']' | b'}') { depth -= 1; }
    }
    let value: StrictValue = serde_json::from_slice(bytes).map_err(|_| WorldStateIntakeError::InvalidJson)?;
    Ok(value.0)
}

fn list<'a>(value: &'a Value, name: &str) -> &'a [Value] {
    value.get(name).and_then(Value::as_array).map_or(&[], Vec::as_slice)
}
fn string<'a>(value: &'a Value, name: &str) -> IntakeResult<&'a str> {
    value.get(name).and_then(Value::as_str).ok_or(WorldStateIntakeError::InvalidShape)
}
fn unique<'a>(values: impl IntoIterator<Item = &'a str>) -> IntakeResult<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) { return Err(WorldStateIntakeError::DuplicateIdentity); }
    }
    Ok(())
}

fn state_rules(value: &Value) -> IntakeResult<()> {
    record(value, STATE)?;
    if string(value, "contract_version")? != WORLD_DOMAIN_CONTRACT {
        return Err(WorldStateIntakeError::UnsupportedContract);
    }
    for name in ["nodes", "npcs", "tasks", "receipts"] {
        let ids: IntakeResult<Vec<&str>> = list(value, name).iter().map(|v| string(v, "id")).collect();
        unique(ids?)?;
    }
    let actors: IntakeResult<Vec<&str>> = list(value, "positions").iter().map(|v| string(v, "actor_id")).collect();
    unique(actors?)?;
    let mut directions = BTreeSet::new();
    for edge in list(value, "edges") {
        if !directions.insert((string(edge, "from")?, string(edge, "direction")?)) {
            return Err(WorldStateIntakeError::DuplicateIdentity);
        }
    }
    for (collection, field) in [("nodes", "tags"), ("npcs", "teaches_skills")] {
        for item in list(value, collection) {
            let texts: IntakeResult<Vec<&str>> = list(item, field).iter()
                .map(|v| v.as_str().ok_or(WorldStateIntakeError::InvalidShape)).collect();
            unique(texts?)?;
        }
    }
    let nodes: IntakeResult<BTreeSet<&str>> = list(value, "nodes").iter().map(|v| string(v, "id")).collect();
    let nodes = nodes?;
    for edge in list(value, "edges") {
        if !nodes.contains(string(edge, "from")?) || !nodes.contains(string(edge, "to")?) {
            return Err(WorldStateIntakeError::MissingReference);
        }
    }
    for name in ["positions", "npcs", "tasks"] {
        for item in list(value, name) {
            if !nodes.contains(string(item, "node_id")?) { return Err(WorldStateIntakeError::MissingReference); }
        }
    }
    Ok(())
}

/// Decode one complete state under the explicit bounded profile. Missing legacy
/// optional fields retain their Serde defaults. Receipts remain UNVERIFIED data.
pub fn decode_bounded_world_state(bytes: &[u8]) -> IntakeResult<WorldState> {
    let value = raw_value(bytes, MAX_WORLD_STATE_INPUT_BYTES)?;
    state_rules(&value)?;
    serde_json::from_value(value).map_err(|_| WorldStateIntakeError::InvalidShape)
}

/// Validate a typed state before handing it to a repository. Does not verify its
/// hash, revision, writer epoch, provenance, authentication or storage outcome.
pub fn validate_bounded_world_state(state: &WorldState) -> IntakeResult<()> {
    struct ByteBudget(usize);
    impl std::io::Write for ByteBudget {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if bytes.len() > self.0 {
                return Err(std::io::Error::other("World state byte budget exceeded"));
            }
            self.0 -= bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }
    // Count serialized bytes without allocating a second unbounded payload.
    serde_json::to_writer(ByteBudget(MAX_WORLD_STATE_INPUT_BYTES), state)
        .map_err(|_| WorldStateIntakeError::ResourceLimit)?;
    let value = serde_json::to_value(state).map_err(|_| WorldStateIntakeError::InvalidShape)?;
    state_rules(&value)
}

/// Decode command shape only. The caller must still invoke authoritative domain
/// admission/command logic. Unknown variants and enum object forms are rejected.
pub fn decode_bounded_world_command(bytes: &[u8]) -> IntakeResult<WorldCommand> {
    let value = raw_value(bytes, MAX_WORLD_COMMAND_INPUT_BYTES)?;
    let fields: &[&str] = match value.get("kind").and_then(Value::as_str) {
        Some("move") => &["kind", "actor_id", "direction"],
        Some("talk_npc") => &["kind", "actor_id", "npc_id"],
        Some("train_skill") => &["kind", "actor_id", "npc_id", "skill_id"],
        Some("complete_task") => &["kind", "actor_id", "task_id"],
        _ => return Err(WorldStateIntakeError::InvalidShape),
    };
    let obj = value.as_object().ok_or(WorldStateIntakeError::InvalidShape)?;
    if obj.len() != fields.len() || fields.iter().any(|f| !obj.contains_key(*f)) {
        return Err(WorldStateIntakeError::InvalidShape);
    }
    for field in fields.iter().filter(|f| **f != "kind") { bounded_text(&value[*field], 160, false)?; }
    serde_json::from_value(value).map_err(|_| WorldStateIntakeError::InvalidShape)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> Vec<u8> {
        serde_json::to_vec(&WorldState::fixture()).unwrap()
    }

    #[test]
    fn fixture_state_passes_structural_validation() {
        let decoded = decode_bounded_world_state(&state()).unwrap();
        validate_bounded_world_state(&decoded).unwrap();
    }

    #[test]
    fn command_shapes_are_explicit_and_unknown_fields_fail() {
        assert!(decode_bounded_world_command(br#"{"kind":"move","actor_id":"a","direction":"east"}"#).is_ok());
        assert!(decode_bounded_world_command(br#"{"kind":"move","actor_id":"a","direction":"east","extra":1}"#).is_err());
        assert!(decode_bounded_world_command(br#"{"kind":"unknown","actor_id":"a"}"#).is_err());
    }

    #[test]
    fn duplicate_keys_and_oversize_input_fail_closed() {
        assert!(decode_bounded_world_command(br#"{"kind":"move","kind":"move","actor_id":"a","direction":"east"}"#).is_err());
        assert_eq!(decode_bounded_world_state(&vec![b' '; MAX_WORLD_STATE_INPUT_BYTES + 1]).unwrap_err(), WorldStateIntakeError::ResourceLimit);
    }

    #[test]
    fn missing_node_reference_is_rejected() {
        let mut value: Value = serde_json::from_slice(&state()).unwrap();
        value["positions"][0]["node_id"] = Value::String("missing".into());
        let bytes = serde_json::to_vec(&value).unwrap();
        assert_eq!(decode_bounded_world_state(&bytes).unwrap_err(), WorldStateIntakeError::MissingReference);
    }

    #[test]
    fn structural_validation_does_not_verify_receipts_or_authorize_production() {
        let mut value: Value = serde_json::from_slice(&state()).unwrap();
        value["receipts"] = serde_json::json!([{"id":"r","progression_class":"fixture","status":"unverified"}]);
        assert!(decode_bounded_world_state(&serde_json::to_vec(&value).unwrap()).is_ok());
        assert!(!super::super::world_production_adapter_readiness().production_ready);
    }
}
