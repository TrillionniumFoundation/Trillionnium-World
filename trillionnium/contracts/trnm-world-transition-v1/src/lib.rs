#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt::{self, Write as _};

pub const CONTRACT_VERSION: &str = "trnm_world_transition_v1";
pub const REQUEST_HASH_DOMAIN: &str = "trnm.world.transition.request.v1";
pub const TRANSITION_HASH_DOMAIN: &str = "trnm.world.transition.accepted.v1";
pub const OUTCOME_HASH_DOMAIN: &str = "trnm.world.outcome.v1";

pub const MAX_STATE_PAYLOAD_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_COMMAND_PAYLOAD_BYTES: usize = 128 * 1024;
pub const MAX_REPLAY_PAYLOAD_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_OUTCOME_PAYLOAD_BYTES: usize = 512 * 1024;
pub const MAX_TICK: u64 = i64::MAX as u64;

const FORBIDDEN_AUTHORITY_KEYS: &[&str] = &[
    "nakama_session_token",
    "nakama_private_key",
    "match_authority_private_key",
    "canonical_archive_root",
    "chain_finality",
    "chain_app_hash",
    "match_completed_v1",
    "participant_admission_receipt",
    "global_event_cursor",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractValidationError {
    InvalidContractVersion,
    InvalidIdentifier { field: &'static str },
    InvalidSha256 { field: &'static str },
    PayloadHashMismatch { field: &'static str },
    InvalidCanonicalJson { field: &'static str },
    ForbiddenAuthoritySurface { field: &'static str },
    PayloadTooLarge {
        field: &'static str,
        actual: usize,
        maximum: usize,
    },
    InvalidTick,
    AdapterInvariant { detail: &'static str },
}

impl fmt::Display for ContractValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContractVersion => formatter.write_str("invalid contract version"),
            Self::InvalidIdentifier { field } => write!(formatter, "invalid identifier: {field}"),
            Self::InvalidSha256 { field } => write!(formatter, "invalid sha256: {field}"),
            Self::PayloadHashMismatch { field } => {
                write!(formatter, "payload hash mismatch: {field}")
            }
            Self::InvalidCanonicalJson { field } => {
                write!(formatter, "invalid canonical json: {field}")
            }
            Self::ForbiddenAuthoritySurface { field } => {
                write!(formatter, "forbidden authority surface: {field}")
            }
            Self::PayloadTooLarge {
                field,
                actual,
                maximum,
            } => write!(
                formatter,
                "payload too large: {field} ({actual} > {maximum})"
            ),
            Self::InvalidTick => formatter.write_str("invalid deterministic tick"),
            Self::AdapterInvariant { detail } => {
                write!(formatter, "adapter invariant failed: {detail}")
            }
        }
    }
}

impl Error for ContractValidationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransitionErrorCodeV1 {
    InvalidContractVersion,
    InvalidRequest,
    UnknownRulesetRevision,
    UnknownContentRevision,
    PayloadHashMismatch,
    InvalidCanonicalPayload,
    ForbiddenAuthoritySurface,
    ResourceBudgetExceeded,
    InvalidCommand,
    DomainRejected,
    NondeterministicOutput,
    InternalUnavailable,
}

impl TransitionErrorCodeV1 {
    pub const ALL: [Self; 12] = [
        Self::InvalidContractVersion,
        Self::InvalidRequest,
        Self::UnknownRulesetRevision,
        Self::UnknownContentRevision,
        Self::PayloadHashMismatch,
        Self::InvalidCanonicalPayload,
        Self::ForbiddenAuthoritySurface,
        Self::ResourceBudgetExceeded,
        Self::InvalidCommand,
        Self::DomainRejected,
        Self::NondeterministicOutput,
        Self::InternalUnavailable,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidContractVersion => "invalid_contract_version",
            Self::InvalidRequest => "invalid_request",
            Self::UnknownRulesetRevision => "unknown_ruleset_revision",
            Self::UnknownContentRevision => "unknown_content_revision",
            Self::PayloadHashMismatch => "payload_hash_mismatch",
            Self::InvalidCanonicalPayload => "invalid_canonical_payload",
            Self::ForbiddenAuthoritySurface => "forbidden_authority_surface",
            Self::ResourceBudgetExceeded => "resource_budget_exceeded",
            Self::InvalidCommand => "invalid_command",
            Self::DomainRejected => "domain_rejected",
            Self::NondeterministicOutput => "nondeterministic_output",
            Self::InternalUnavailable => "internal_unavailable",
        }
    }

    pub const fn recommended_retryable(self) -> bool {
        matches!(self, Self::InternalUnavailable)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalPayloadV1 {
    pub schema_id: String,
    pub canonical_json: String,
    pub sha256: String,
}

impl CanonicalPayloadV1 {
    pub fn new(
        schema_id: impl Into<String>,
        canonical_json: impl Into<String>,
    ) -> Result<Self, ContractValidationError> {
        let schema_id = schema_id.into();
        let canonical_json = canonical_json.into();
        validate_identifier("payload.schema_id", &schema_id, 160)?;
        validate_minified_json("payload.canonical_json", &canonical_json)?;
        reject_forbidden_authority_surface("payload.canonical_json", &canonical_json)?;
        let sha256 = sha256_hex(canonical_json.as_bytes());
        Ok(Self {
            schema_id,
            canonical_json,
            sha256,
        })
    }

    pub fn from_parts(
        schema_id: impl Into<String>,
        canonical_json: impl Into<String>,
        sha256: impl Into<String>,
    ) -> Result<Self, ContractValidationError> {
        let payload = Self {
            schema_id: schema_id.into(),
            canonical_json: canonical_json.into(),
            sha256: sha256.into(),
        };
        payload.validate_with_limit("payload", usize::MAX)?;
        Ok(payload)
    }

    pub fn validate_with_limit(
        &self,
        field: &'static str,
        maximum_bytes: usize,
    ) -> Result<(), ContractValidationError> {
        validate_identifier("payload.schema_id", &self.schema_id, 160)?;
        if self.canonical_json.len() > maximum_bytes {
            return Err(ContractValidationError::PayloadTooLarge {
                field,
                actual: self.canonical_json.len(),
                maximum: maximum_bytes,
            });
        }
        validate_minified_json(field, &self.canonical_json)?;
        reject_forbidden_authority_surface(field, &self.canonical_json)?;
        validate_sha256(field, &self.sha256)?;
        if sha256_hex(self.canonical_json.as_bytes()) != self.sha256 {
            return Err(ContractValidationError::PayloadHashMismatch { field });
        }
        Ok(())
    }

    pub fn to_canonical_json(&self) -> String {
        let mut output = String::with_capacity(self.canonical_json.len() + 240);
        output.push_str("{\"canonical_json\":");
        output.push_str(&self.canonical_json);
        output.push_str(",\"schema_id\":");
        push_json_string(&mut output, &self.schema_id);
        output.push_str(",\"sha256\":");
        push_json_string(&mut output, &self.sha256);
        output.push('}');
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldCommandV1 {
    pub command_id: String,
    pub payload: CanonicalPayloadV1,
}

impl WorldCommandV1 {
    pub fn new(
        command_id: impl Into<String>,
        payload: CanonicalPayloadV1,
    ) -> Result<Self, ContractValidationError> {
        let command = Self {
            command_id: command_id.into(),
            payload,
        };
        command.validate()?;
        Ok(command)
    }

    pub fn validate(&self) -> Result<(), ContractValidationError> {
        validate_identifier("command.command_id", &self.command_id, 160)?;
        self.payload
            .validate_with_limit("command.payload", MAX_COMMAND_PAYLOAD_BYTES)
    }

    pub fn to_canonical_json(&self) -> String {
        let mut output = String::with_capacity(self.payload.canonical_json.len() + 320);
        output.push_str("{\"command_id\":");
        push_json_string(&mut output, &self.command_id);
        output.push_str(",\"payload\":");
        output.push_str(&self.payload.to_canonical_json());
        output.push('}');
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldTransitionRequestV1 {
    pub contract_version: String,
    pub transition_id: String,
    pub ruleset_revision: String,
    pub content_revision: String,
    pub expected_tick: u64,
    pub previous_state: CanonicalPayloadV1,
    pub command: WorldCommandV1,
}

impl WorldTransitionRequestV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transition_id: impl Into<String>,
        ruleset_revision: impl Into<String>,
        content_revision: impl Into<String>,
        expected_tick: u64,
        previous_state: CanonicalPayloadV1,
        command: WorldCommandV1,
    ) -> Result<Self, ContractValidationError> {
        let request = Self {
            contract_version: CONTRACT_VERSION.to_string(),
            transition_id: transition_id.into(),
            ruleset_revision: ruleset_revision.into(),
            content_revision: content_revision.into(),
            expected_tick,
            previous_state,
            command,
        };
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), ContractValidationError> {
        if self.contract_version != CONTRACT_VERSION {
            return Err(ContractValidationError::InvalidContractVersion);
        }
        validate_identifier("transition_id", &self.transition_id, 160)?;
        validate_identifier("ruleset_revision", &self.ruleset_revision, 160)?;
        validate_identifier("content_revision", &self.content_revision, 160)?;
        if self.expected_tick > MAX_TICK {
            return Err(ContractValidationError::InvalidTick);
        }
        self.previous_state
            .validate_with_limit("previous_state", MAX_STATE_PAYLOAD_BYTES)?;
        self.command.validate()
    }

    pub fn to_canonical_json(&self) -> Result<String, ContractValidationError> {
        self.validate()?;
        let mut output = String::with_capacity(
            self.previous_state.canonical_json.len() + self.command.payload.canonical_json.len() + 768,
        );
        output.push_str("{\"command\":");
        output.push_str(&self.command.to_canonical_json());
        output.push_str(",\"content_revision\":");
        push_json_string(&mut output, &self.content_revision);
        output.push_str(",\"contract_version\":");
        push_json_string(&mut output, &self.contract_version);
        let _ = write!(output, ",\"expected_tick\":{}", self.expected_tick);
        output.push_str(",\"previous_state\":");
        output.push_str(&self.previous_state.to_canonical_json());
        output.push_str(",\"ruleset_revision\":");
        push_json_string(&mut output, &self.ruleset_revision);
        output.push_str(",\"transition_id\":");
        push_json_string(&mut output, &self.transition_id);
        output.push('}');
        Ok(output)
    }

    pub fn request_hash(&self) -> Result<String, ContractValidationError> {
        let canonical = self.to_canonical_json()?;
        Ok(domain_hash(REQUEST_HASH_DOMAIN, canonical.as_bytes()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionOutputV1 {
    pub next_tick: u64,
    pub next_state: CanonicalPayloadV1,
    pub replay_material: CanonicalPayloadV1,
    pub outcome_material: Option<CanonicalPayloadV1>,
}

impl TransitionOutputV1 {
    pub fn validate_against(
        &self,
        request: &WorldTransitionRequestV1,
    ) -> Result<(), ContractValidationError> {
        if self.next_tick < request.expected_tick || self.next_tick > MAX_TICK {
            return Err(ContractValidationError::InvalidTick);
        }
        self.next_state
            .validate_with_limit("next_state", MAX_STATE_PAYLOAD_BYTES)?;
        self.replay_material
            .validate_with_limit("replay_material", MAX_REPLAY_PAYLOAD_BYTES)?;
        if let Some(outcome) = self.outcome_material.as_ref() {
            outcome.validate_with_limit("outcome_material", MAX_OUTCOME_PAYLOAD_BYTES)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainRejectionV1 {
    pub code: TransitionErrorCodeV1,
    pub detail: String,
}

impl DomainRejectionV1 {
    pub fn new(code: TransitionErrorCodeV1, detail: impl AsRef<str>) -> Self {
        let code = match code {
            TransitionErrorCodeV1::InvalidCommand
            | TransitionErrorCodeV1::DomainRejected
            | TransitionErrorCodeV1::ResourceBudgetExceeded
            | TransitionErrorCodeV1::InternalUnavailable => code,
            _ => TransitionErrorCodeV1::DomainRejected,
        };
        Self {
            code,
            detail: sanitize_detail(detail.as_ref()),
        }
    }
}

pub trait WorldRulesAdapterV1 {
    fn supports_ruleset(&self, ruleset_revision: &str) -> bool;
    fn supports_content(&self, content_revision: &str) -> bool;
    fn transition(
        &self,
        request: &WorldTransitionRequestV1,
    ) -> Result<TransitionOutputV1, DomainRejectionV1>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldTransitionAcceptedV1 {
    pub contract_version: String,
    pub transition_id: String,
    pub ruleset_revision: String,
    pub content_revision: String,
    pub request_hash: String,
    pub previous_state_hash: String,
    pub next_tick: u64,
    pub next_state: CanonicalPayloadV1,
    pub replay_material: CanonicalPayloadV1,
    pub outcome_material: Option<CanonicalPayloadV1>,
    pub world_outcome_hash: Option<String>,
    pub world_transition_hash: String,
}

impl WorldTransitionAcceptedV1 {
    fn from_output(
        request: &WorldTransitionRequestV1,
        output: TransitionOutputV1,
    ) -> Result<Self, ContractValidationError> {
        request.validate()?;
        output.validate_against(request)?;
        let request_hash = request.request_hash()?;
        let world_outcome_hash = output.outcome_material.as_ref().map(|outcome| {
            outcome_hash(
                &request.ruleset_revision,
                &request.content_revision,
                outcome,
            )
        });
        let mut accepted = Self {
            contract_version: CONTRACT_VERSION.to_string(),
            transition_id: request.transition_id.clone(),
            ruleset_revision: request.ruleset_revision.clone(),
            content_revision: request.content_revision.clone(),
            request_hash,
            previous_state_hash: request.previous_state.sha256.clone(),
            next_tick: output.next_tick,
            next_state: output.next_state,
            replay_material: output.replay_material,
            outcome_material: output.outcome_material,
            world_outcome_hash,
            world_transition_hash: String::new(),
        };
        accepted.world_transition_hash = accepted.compute_transition_hash()?;
        accepted.validate()?;
        Ok(accepted)
    }

    pub fn validate(&self) -> Result<(), ContractValidationError> {
        if self.contract_version != CONTRACT_VERSION {
            return Err(ContractValidationError::InvalidContractVersion);
        }
        validate_identifier("transition_id", &self.transition_id, 160)?;
        validate_identifier("ruleset_revision", &self.ruleset_revision, 160)?;
        validate_identifier("content_revision", &self.content_revision, 160)?;
        validate_sha256("request_hash", &self.request_hash)?;
        validate_sha256("previous_state_hash", &self.previous_state_hash)?;
        validate_sha256("world_transition_hash", &self.world_transition_hash)?;
        if self.next_tick > MAX_TICK {
            return Err(ContractValidationError::InvalidTick);
        }
        self.next_state
            .validate_with_limit("next_state", MAX_STATE_PAYLOAD_BYTES)?;
        self.replay_material
            .validate_with_limit("replay_material", MAX_REPLAY_PAYLOAD_BYTES)?;
        match (&self.outcome_material, &self.world_outcome_hash) {
            (Some(outcome), Some(hash)) => {
                outcome.validate_with_limit("outcome_material", MAX_OUTCOME_PAYLOAD_BYTES)?;
                validate_sha256("world_outcome_hash", hash)?;
                if outcome_hash(&self.ruleset_revision, &self.content_revision, outcome) != *hash {
                    return Err(ContractValidationError::PayloadHashMismatch {
                        field: "world_outcome_hash",
                    });
                }
            }
            (None, None) => {}
            _ => {
                return Err(ContractValidationError::AdapterInvariant {
                    detail: "outcome material/hash presence mismatch",
                });
            }
        }
        if self.compute_transition_hash()? != self.world_transition_hash {
            return Err(ContractValidationError::PayloadHashMismatch {
                field: "world_transition_hash",
            });
        }
        Ok(())
    }

    pub fn canonical_facts_json(&self) -> Result<String, ContractValidationError> {
        self.next_state
            .validate_with_limit("next_state", MAX_STATE_PAYLOAD_BYTES)?;
        self.replay_material
            .validate_with_limit("replay_material", MAX_REPLAY_PAYLOAD_BYTES)?;
        let mut output = String::with_capacity(
            self.next_state.canonical_json.len() + self.replay_material.canonical_json.len() + 1400,
        );
        output.push_str("{\"content_revision\":");
        push_json_string(&mut output, &self.content_revision);
        output.push_str(",\"contract_version\":");
        push_json_string(&mut output, &self.contract_version);
        output.push_str(",\"next_state\":");
        output.push_str(&self.next_state.to_canonical_json());
        let _ = write!(output, ",\"next_tick\":{}", self.next_tick);
        output.push_str(",\"outcome_material\":");
        if let Some(outcome) = self.outcome_material.as_ref() {
            output.push_str(&outcome.to_canonical_json());
        } else {
            output.push_str("null");
        }
        output.push_str(",\"previous_state_hash\":");
        push_json_string(&mut output, &self.previous_state_hash);
        output.push_str(",\"replay_material\":");
        output.push_str(&self.replay_material.to_canonical_json());
        output.push_str(",\"request_hash\":");
        push_json_string(&mut output, &self.request_hash);
        output.push_str(",\"ruleset_revision\":");
        push_json_string(&mut output, &self.ruleset_revision);
        output.push_str(",\"transition_id\":");
        push_json_string(&mut output, &self.transition_id);
        output.push_str(",\"world_outcome_hash\":");
        if let Some(hash) = self.world_outcome_hash.as_ref() {
            push_json_string(&mut output, hash);
        } else {
            output.push_str("null");
        }
        output.push('}');
        Ok(output)
    }

    pub fn compute_transition_hash(&self) -> Result<String, ContractValidationError> {
        Ok(domain_hash(
            TRANSITION_HASH_DOMAIN,
            self.canonical_facts_json()?.as_bytes(),
        ))
    }

    pub fn to_canonical_json(&self) -> Result<String, ContractValidationError> {
        self.validate()?;
        let facts = self.canonical_facts_json()?;
        let mut output = String::with_capacity(facts.len() + 100);
        output.push_str(&facts[..facts.len() - 1]);
        output.push_str(",\"world_transition_hash\":");
        push_json_string(&mut output, &self.world_transition_hash);
        output.push('}');
        Ok(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldTransitionRejectedV1 {
    pub contract_version: String,
    pub transition_id: String,
    pub request_hash: Option<String>,
    pub code: TransitionErrorCodeV1,
    pub retryable: bool,
    pub detail: String,
}

impl WorldTransitionRejectedV1 {
    fn from_request(
        request: &WorldTransitionRequestV1,
        code: TransitionErrorCodeV1,
        detail: impl AsRef<str>,
    ) -> Self {
        let transition_id = if validate_identifier("transition_id", &request.transition_id, 160)
            .is_ok()
        {
            request.transition_id.clone()
        } else {
            "invalid-transition".to_string()
        };
        Self {
            contract_version: CONTRACT_VERSION.to_string(),
            transition_id,
            request_hash: request.request_hash().ok(),
            code,
            retryable: code.recommended_retryable(),
            detail: sanitize_detail(detail.as_ref()),
        }
    }

    pub fn to_canonical_json(&self) -> String {
        let mut output = String::with_capacity(512);
        output.push_str("{\"code\":");
        push_json_string(&mut output, self.code.as_str());
        output.push_str(",\"contract_version\":");
        push_json_string(&mut output, &self.contract_version);
        output.push_str(",\"detail\":");
        push_json_string(&mut output, &self.detail);
        output.push_str(",\"request_hash\":");
        if let Some(hash) = self.request_hash.as_ref() {
            push_json_string(&mut output, hash);
        } else {
            output.push_str("null");
        }
        let _ = write!(
            output,
            ",\"retryable\":{}",
            if self.retryable { "true" } else { "false" }
        );
        output.push_str(",\"transition_id\":");
        push_json_string(&mut output, &self.transition_id);
        output.push('}');
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldTransitionResultV1 {
    Accepted(WorldTransitionAcceptedV1),
    Rejected(WorldTransitionRejectedV1),
}

impl WorldTransitionResultV1 {
    pub fn to_canonical_json(&self) -> Result<String, ContractValidationError> {
        match self {
            Self::Accepted(accepted) => accepted.to_canonical_json(),
            Self::Rejected(rejected) => Ok(rejected.to_canonical_json()),
        }
    }
}

pub fn execute_transition<A: WorldRulesAdapterV1>(
    adapter: &A,
    request: &WorldTransitionRequestV1,
) -> WorldTransitionResultV1 {
    if let Err(error) = request.validate() {
        return WorldTransitionResultV1::Rejected(WorldTransitionRejectedV1::from_request(
            request,
            validation_error_code(&error),
            error.to_string(),
        ));
    }
    if !adapter.supports_ruleset(&request.ruleset_revision) {
        return WorldTransitionResultV1::Rejected(WorldTransitionRejectedV1::from_request(
            request,
            TransitionErrorCodeV1::UnknownRulesetRevision,
            "ruleset revision is not supported",
        ));
    }
    if !adapter.supports_content(&request.content_revision) {
        return WorldTransitionResultV1::Rejected(WorldTransitionRejectedV1::from_request(
            request,
            TransitionErrorCodeV1::UnknownContentRevision,
            "content revision is not supported",
        ));
    }
    let output = match adapter.transition(request) {
        Ok(output) => output,
        Err(rejection) => {
            return WorldTransitionResultV1::Rejected(WorldTransitionRejectedV1::from_request(
                request,
                rejection.code,
                rejection.detail,
            ));
        }
    };
    match WorldTransitionAcceptedV1::from_output(request, output) {
        Ok(accepted) => WorldTransitionResultV1::Accepted(accepted),
        Err(error) => WorldTransitionResultV1::Rejected(
            WorldTransitionRejectedV1::from_request(
                request,
                validation_error_code(&error),
                error.to_string(),
            ),
        ),
    }
}

fn validation_error_code(error: &ContractValidationError) -> TransitionErrorCodeV1 {
    match error {
        ContractValidationError::InvalidContractVersion => {
            TransitionErrorCodeV1::InvalidContractVersion
        }
        ContractValidationError::PayloadHashMismatch { .. } => {
            TransitionErrorCodeV1::PayloadHashMismatch
        }
        ContractValidationError::InvalidCanonicalJson { .. } => {
            TransitionErrorCodeV1::InvalidCanonicalPayload
        }
        ContractValidationError::ForbiddenAuthoritySurface { .. } => {
            TransitionErrorCodeV1::ForbiddenAuthoritySurface
        }
        ContractValidationError::PayloadTooLarge { .. } => {
            TransitionErrorCodeV1::ResourceBudgetExceeded
        }
        ContractValidationError::AdapterInvariant { .. } => {
            TransitionErrorCodeV1::NondeterministicOutput
        }
        ContractValidationError::InvalidIdentifier { .. }
        | ContractValidationError::InvalidSha256 { .. }
        | ContractValidationError::InvalidTick => TransitionErrorCodeV1::InvalidRequest,
    }
}

fn outcome_hash(
    ruleset_revision: &str,
    content_revision: &str,
    outcome: &CanonicalPayloadV1,
) -> String {
    let mut material = String::with_capacity(512);
    material.push_str("{\"content_revision\":");
    push_json_string(&mut material, content_revision);
    material.push_str(",\"outcome_payload_hash\":");
    push_json_string(&mut material, &outcome.sha256);
    material.push_str(",\"outcome_schema_id\":");
    push_json_string(&mut material, &outcome.schema_id);
    material.push_str(",\"ruleset_revision\":");
    push_json_string(&mut material, ruleset_revision);
    material.push('}');
    domain_hash(OUTCOME_HASH_DOMAIN, material.as_bytes())
}

fn domain_hash(domain: &str, material: &[u8]) -> String {
    let mut preimage = Vec::with_capacity(domain.len() + material.len() + 1);
    preimage.extend_from_slice(domain.as_bytes());
    preimage.push(b'\n');
    preimage.extend_from_slice(material);
    sha256_hex(&preimage)
}

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), ContractValidationError> {
    if value.is_empty()
        || value.len() > maximum
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b':' | b'/' | b'-' | b'+' | b'@')
        })
    {
        return Err(ContractValidationError::InvalidIdentifier { field });
    }
    Ok(())
}

fn validate_sha256(
    field: &'static str,
    value: &str,
) -> Result<(), ContractValidationError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ContractValidationError::InvalidSha256 { field });
    }
    Ok(())
}

fn reject_forbidden_authority_surface(
    field: &'static str,
    canonical_json: &str,
) -> Result<(), ContractValidationError> {
    let lowered = canonical_json.to_ascii_lowercase();
    if FORBIDDEN_AUTHORITY_KEYS
        .iter()
        .any(|key| lowered.contains(&format!("\"{key}\"")))
    {
        return Err(ContractValidationError::ForbiddenAuthoritySurface { field });
    }
    Ok(())
}

fn validate_minified_json(
    field: &'static str,
    canonical_json: &str,
) -> Result<(), ContractValidationError> {
    let bytes = canonical_json.as_bytes();
    if bytes.len() < 2 || !matches!(bytes[0], b'{' | b'[') {
        return Err(ContractValidationError::InvalidCanonicalJson { field });
    }
    let mut stack = Vec::with_capacity(16);
    let mut in_string = false;
    let mut escaped = false;
    for (index, byte) in bytes.iter().copied().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match byte {
                b'\\' => escaped = true,
                b'"' => in_string = false,
                0x00..=0x1f => {
                    return Err(ContractValidationError::InvalidCanonicalJson { field });
                }
                _ => {}
            }
            continue;
        }
        if byte.is_ascii_whitespace() {
            return Err(ContractValidationError::InvalidCanonicalJson { field });
        }
        match byte {
            b'"' => in_string = true,
            b'{' => stack.push(b'}'),
            b'[' => stack.push(b']'),
            b'}' | b']' => {
                if stack.pop() != Some(byte) {
                    return Err(ContractValidationError::InvalidCanonicalJson { field });
                }
                if stack.is_empty() && index + 1 != bytes.len() {
                    return Err(ContractValidationError::InvalidCanonicalJson { field });
                }
            }
            _ => {}
        }
    }
    if in_string || escaped || !stack.is_empty() {
        return Err(ContractValidationError::InvalidCanonicalJson { field });
    }
    Ok(())
}

fn sanitize_detail(detail: &str) -> String {
    let mut sanitized = String::with_capacity(detail.len().min(256));
    for character in detail.chars() {
        if sanitized.len() >= 256 {
            break;
        }
        if character.is_control() {
            sanitized.push(' ');
        } else {
            sanitized.push(character);
        }
    }
    let sanitized = sanitized.trim();
    if sanitized.is_empty() {
        "request rejected".to_string()
    } else {
        sanitized.to_string()
    }
}

fn push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character <= '\u{1f}' => {
                let _ = write!(output, "\\u{:04x}", character as u32);
            }
            character => output.push(character),
        }
    }
    output.push('"');
}

pub fn sha256_hex(input: &[u8]) -> String {
    let digest = sha256(input);
    let mut output = String::with_capacity(64);
    for byte in digest {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn sha256(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
        0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut state = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_length = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_length.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, bytes) in chunk.chunks_exact(4).enumerate() {
            words[index] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];

        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temporary1 = h
                .wrapping_add(sum1)
                .wrapping_add(choice)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temporary2 = sum0.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temporary1);
            d = c;
            c = b;
            b = a;
            a = temporary1.wrapping_add(temporary2);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut digest = [0u8; 32];
    for (index, word) in state.into_iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const EMPTY_OBJECT_HASH: &str =
        "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a";

    fn empty_payload(schema_id: &str) -> CanonicalPayloadV1 {
        CanonicalPayloadV1::new(schema_id, "{}").expect("valid canonical payload")
    }

    fn request(command_json: &str) -> WorldTransitionRequestV1 {
        WorldTransitionRequestV1::new(
            "transition-0001",
            "trnm-rts-rules-v1",
            "first-contact-content-v1",
            120,
            empty_payload("trnm.rts.state.v1"),
            WorldCommandV1::new(
                "command-0001",
                CanonicalPayloadV1::new("trnm.rts.order.v1", command_json)
                    .expect("valid command payload"),
            )
            .expect("valid command"),
        )
        .expect("valid request")
    }

    struct EchoAdapter;

    impl WorldRulesAdapterV1 for EchoAdapter {
        fn supports_ruleset(&self, ruleset_revision: &str) -> bool {
            ruleset_revision == "trnm-rts-rules-v1"
        }

        fn supports_content(&self, content_revision: &str) -> bool {
            content_revision == "first-contact-content-v1"
        }

        fn transition(
            &self,
            request: &WorldTransitionRequestV1,
        ) -> Result<TransitionOutputV1, DomainRejectionV1> {
            Ok(TransitionOutputV1 {
                next_tick: request.expected_tick + 1,
                next_state: empty_payload("trnm.rts.state.v1"),
                replay_material: empty_payload("trnm.rts.replay.v1"),
                outcome_material: None,
            })
        }
    }

    #[test]
    fn sha256_implements_published_core_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn payload_hash_and_canonical_form_are_stable() {
        let payload = empty_payload("trnm.rts.state.v1");
        assert_eq!(payload.sha256, EMPTY_OBJECT_HASH);
        assert_eq!(
            payload.to_canonical_json(),
            format!(
                "{{\"canonical_json\":{{}},\"schema_id\":\"trnm.rts.state.v1\",\"sha256\":\"{EMPTY_OBJECT_HASH}\"}}"
            )
        );
    }

    #[test]
    fn request_canonicalization_is_stable_and_sorted() {
        let request = request("{}");
        assert_eq!(
            request.to_canonical_json().expect("canonical request"),
            format!(
                "{{\"command\":{{\"command_id\":\"command-0001\",\"payload\":{{\"canonical_json\":{{}},\"schema_id\":\"trnm.rts.order.v1\",\"sha256\":\"{EMPTY_OBJECT_HASH}\"}}}},\"content_revision\":\"first-contact-content-v1\",\"contract_version\":\"trnm_world_transition_v1\",\"expected_tick\":120,\"previous_state\":{{\"canonical_json\":{{}},\"schema_id\":\"trnm.rts.state.v1\",\"sha256\":\"{EMPTY_OBJECT_HASH}\"}},\"ruleset_revision\":\"trnm-rts-rules-v1\",\"transition_id\":\"transition-0001\"}}"
            )
        );
    }

    #[test]
    fn identical_inputs_produce_identical_transition_hashes() {
        let first = execute_transition(&EchoAdapter, &request("{}"));
        let second = execute_transition(&EchoAdapter, &request("{}"));
        assert_eq!(first, second);
        let WorldTransitionResultV1::Accepted(accepted) = first else {
            panic!("transition must be accepted");
        };
        assert_eq!(accepted.world_transition_hash.len(), 64);
        accepted.validate().expect("accepted response validates");
    }

    #[test]
    fn changed_command_changes_request_and_transition_hashes() {
        let first_request = request("{}");
        let second_request = request("{\"order\":\"hold\"}");
        assert_ne!(
            first_request.request_hash().expect("first hash"),
            second_request.request_hash().expect("second hash")
        );
        let WorldTransitionResultV1::Accepted(first) =
            execute_transition(&EchoAdapter, &first_request)
        else {
            panic!("first transition must be accepted");
        };
        let WorldTransitionResultV1::Accepted(second) =
            execute_transition(&EchoAdapter, &second_request)
        else {
            panic!("second transition must be accepted");
        };
        assert_ne!(first.world_transition_hash, second.world_transition_hash);
    }

    #[test]
    fn authority_private_material_is_rejected_from_opaque_payloads() {
        let error = CanonicalPayloadV1::new(
            "trnm.rts.order.v1",
            "{\"nakama_private_key\":\"forbidden\"}",
        )
        .expect_err("authority material must fail closed");
        assert!(matches!(
            error,
            ContractValidationError::ForbiddenAuthoritySurface { .. }
        ));
    }

    #[test]
    fn non_minified_payloads_fail_closed() {
        let error = CanonicalPayloadV1::new("trnm.rts.order.v1", "{ \"order\":1}")
            .expect_err("whitespace outside strings is noncanonical");
        assert!(matches!(
            error,
            ContractValidationError::InvalidCanonicalJson { .. }
        ));
    }

    #[test]
    fn unknown_ruleset_and_content_revisions_are_typed_rejections() {
        let mut unknown_ruleset = request("{}");
        unknown_ruleset.ruleset_revision = "unknown-rules-v9".to_string();
        let WorldTransitionResultV1::Rejected(rejection) =
            execute_transition(&EchoAdapter, &unknown_ruleset)
        else {
            panic!("unknown ruleset must be rejected");
        };
        assert_eq!(
            rejection.code,
            TransitionErrorCodeV1::UnknownRulesetRevision
        );
        assert!(!rejection.retryable);

        let mut unknown_content = request("{}");
        unknown_content.content_revision = "unknown-content-v9".to_string();
        let WorldTransitionResultV1::Rejected(rejection) =
            execute_transition(&EchoAdapter, &unknown_content)
        else {
            panic!("unknown content must be rejected");
        };
        assert_eq!(
            rejection.code,
            TransitionErrorCodeV1::UnknownContentRevision
        );
    }

    #[test]
    fn error_catalogue_is_unique_and_stable() {
        let values = TransitionErrorCodeV1::ALL
            .into_iter()
            .map(TransitionErrorCodeV1::as_str)
            .collect::<BTreeSet<_>>();
        assert_eq!(values.len(), TransitionErrorCodeV1::ALL.len());
        assert!(values.contains("nondeterministic_output"));
        assert!(values.contains("forbidden_authority_surface"));
    }

    #[test]
    fn rejection_detail_is_bounded_and_control_free() {
        let detail = format!("secret\n{}", "x".repeat(400));
        let rejection = WorldTransitionRejectedV1::from_request(
            &request("{}"),
            TransitionErrorCodeV1::DomainRejected,
            detail,
        );
        assert!(rejection.detail.len() <= 256);
        assert!(!rejection.detail.chars().any(char::is_control));
    }
}
