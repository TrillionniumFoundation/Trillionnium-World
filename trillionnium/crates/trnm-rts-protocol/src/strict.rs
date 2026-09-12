//! Versioned JSON intake for untrusted order bytes. Not player authorization.
//!
//! Legacy `Deserialize` stays compatible with saved replay material. New callers
//! must opt into this profile explicitly; it never silently downgrades to legacy.

use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile, RTS_ORDER_PROTOCOL};

pub const INTAKE_CONTRACT: &str = "trnm_rts_order_intake_v1";
pub const MAX_INPUT_BYTES: usize = 128 * 1024;
pub const MAX_ID_BYTES: usize = 160;
pub const MAX_SUBJECTS: usize = 256;
pub const MAX_LABEL_BYTES: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntakeError {
    ResourceBudgetExceeded,
    InvalidEncoding,
    UnsupportedIntakeContract,
    UnsupportedOrderContract,
    InvalidIdentifier,
    DuplicateSubject,
    InvalidShape,
}

impl IntakeError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::ResourceBudgetExceeded => "resource_budget_exceeded",
            Self::InvalidEncoding => "invalid_encoding",
            Self::UnsupportedIntakeContract => "unsupported_intake_contract",
            Self::UnsupportedOrderContract => "unsupported_order_contract",
            Self::InvalidIdentifier => "invalid_identifier",
            Self::DuplicateSubject => "duplicate_subject",
            Self::InvalidShape => "invalid_shape",
        }
    }
}

impl std::fmt::Display for IntakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for IntakeError {}

/// Constructible only by successful versioned intake. Contains no authority proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedOrder(RtsFrameOrder);

impl ValidatedOrder {
    pub fn as_order(&self) -> &RtsFrameOrder {
        &self.0
    }

    pub fn into_order(self) -> RtsFrameOrder {
        self.0
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    intake_contract: String,
    #[serde(deserialize_with = "object_only")]
    order: WireOrder,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireTile {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireOrder {
    contract: String,
    frame: u32,
    player_id: String,
    subject_actor_ids: Vec<String>,
    #[serde(deserialize_with = "string_enum_only")]
    kind: RtsOrderKind,
    #[serde(default)]
    queued: bool,
    #[serde(default, deserialize_with = "optional_object_only")]
    target_tile: Option<WireTile>,
    #[serde(default)]
    target_actor_id: Option<String>,
    #[serde(default)]
    target_rule_id: Option<String>,
    #[serde(default)]
    queue_id: Option<String>,
    #[serde(default)]
    formation_id: Option<String>,
    #[serde(deserialize_with = "string_enum_only")]
    source: RtsOrderSource,
    #[serde(default)]
    raw_command_label: Option<String>,
}

// A derived externally tagged unit enum accepts both "hold" and {"hold":null}.
// This wire contract permits strings only, independent of the legacy reader.
fn string_enum_only<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let spelling = String::deserialize(deserializer)?;
    T::deserialize(serde::de::value::StringDeserializer::<D::Error>::new(
        spelling,
    ))
}

// Serde structs may otherwise accept positional JSON arrays. Require maps
// without an intermediate Value, preserving duplicate-field rejection.
fn object_only<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct ObjectVisitor<T>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for ObjectVisitor<T> {
        type Value = T;

        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("a JSON object")
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<T, A::Error> {
            T::deserialize(serde::de::value::MapAccessDeserializer::new(map))
        }
    }
    deserializer.deserialize_map(ObjectVisitor::<T>(std::marker::PhantomData))
}

fn optional_object_only<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct OptionalObjectVisitor<T>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for OptionalObjectVisitor<T> {
        type Value = Option<T>;

        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("a JSON object or null")
        }

        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D2: serde::Deserializer<'de>>(self, d: D2) -> Result<Self::Value, D2::Error> {
            object_only(d).map(Some)
        }
    }
    deserializer.deserialize_option(OptionalObjectVisitor::<T>(std::marker::PhantomData))
}

fn identifier(value: &str) -> Result<(), IntakeError> {
    if value.len() > MAX_ID_BYTES {
        return Err(IntakeError::ResourceBudgetExceeded);
    }
    // Exact Unicode is preserved, not normalized. Restrict only ASCII control
    // and space bytes so independent implementations use identical predicates.
    if value.is_empty() || value.bytes().any(|byte| byte <= 0x20 || byte == 0x7f) {
        return Err(IntakeError::InvalidIdentifier);
    }
    Ok(())
}

/// Parse one envelope. Size is checked before parsing/allocation amplification.
///
/// All errors are bounded stable codes with no reflected untrusted payload.
/// No simulation, filesystem, clock, credential, or network is accessed.
pub fn decode(input: &[u8]) -> Result<ValidatedOrder, IntakeError> {
    if input.len() > MAX_INPUT_BYTES {
        return Err(IntakeError::ResourceBudgetExceeded);
    }
    // Direct typed deserialization rejects duplicate known fields, unknown
    // fields at every object boundary, invalid UTF-8, numeric widths and tails.
    // Never deserialize through Value: that could erase duplicate keys.
    let mut decoder = serde_json::Deserializer::from_slice(input);
    let envelope: Envelope = object_only(&mut decoder).map_err(|_| IntakeError::InvalidEncoding)?;
    decoder.end().map_err(|_| IntakeError::InvalidEncoding)?;
    if envelope.intake_contract != INTAKE_CONTRACT {
        return Err(IntakeError::UnsupportedIntakeContract);
    }
    let wire = envelope.order;
    if wire.contract != RTS_ORDER_PROTOCOL {
        return Err(IntakeError::UnsupportedOrderContract);
    }
    if wire.subject_actor_ids.len() > MAX_SUBJECTS {
        return Err(IntakeError::ResourceBudgetExceeded);
    }
    identifier(&wire.player_id)?;
    let mut subjects = BTreeSet::new();
    for subject in &wire.subject_actor_ids {
        identifier(subject)?;
        if !subjects.insert(subject.as_str()) {
            return Err(IntakeError::DuplicateSubject);
        }
    }
    for value in [
        &wire.target_actor_id,
        &wire.target_rule_id,
        &wire.queue_id,
        &wire.formation_id,
    ]
    .into_iter()
    .flatten()
    {
        identifier(value)?;
    }
    if wire
        .raw_command_label
        .as_ref()
        .is_some_and(|label| label.len() > MAX_LABEL_BYTES)
    {
        return Err(IntakeError::ResourceBudgetExceeded);
    }
    let order = RtsFrameOrder {
        contract: wire.contract,
        frame: wire.frame,
        player_id: wire.player_id,
        subject_actor_ids: wire.subject_actor_ids,
        kind: wire.kind,
        queued: wire.queued,
        target_tile: wire.target_tile.map(|tile| RtsTile::new(tile.x, tile.y)),
        target_actor_id: wire.target_actor_id,
        target_rule_id: wire.target_rule_id,
        queue_id: wire.queue_id,
        formation_id: wire.formation_id,
        source: wire.source,
        raw_command_label: wire.raw_command_label,
    };
    order.validate().map_err(|_| IntakeError::InvalidShape)?;
    Ok(ValidatedOrder(order))
}
