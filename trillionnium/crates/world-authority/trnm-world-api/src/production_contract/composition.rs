use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::*;

pub const WORLD_PRODUCTION_COMPOSITION_CONTRACT: &str =
    "trillionnium_world_production_composition_v1";
pub const WORLD_PRODUCTION_REQUIRED_ROLES: [&str; 9] = [
    "identity",
    "session_guard",
    "account",
    "repository",
    "ledger",
    "evidence_sink",
    "metrics_sink",
    "internal_routing",
    "deployment_identity",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldProductionAdapterQualification {
    pub role: String,
    pub implementation_id: String,
    pub implementation_owner_id: String,
    pub adapter_contract: String,
    pub evidence_id: String,
    pub evidence_sha256: String,
    pub exact_head_qualified: bool,
    pub external_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldProductionQualificationEvidence {
    pub composition_contract: String,
    pub adapter_contract: String,
    pub source_commit: String,
    pub source_tree: String,
    pub binary_sha256: String,
    pub component_lock_id: String,
    pub deployment_id: String,
    pub qualification_run_id: String,
    pub independent_approver_id: String,
    pub issued_at_utc: String,
    pub adapters: Vec<WorldProductionAdapterQualification>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldProductionCompositionReceipt {
    pub composition_contract: String,
    pub adapter_contract: String,
    pub deployment_id: String,
    pub component_lock_id: String,
    pub source_commit: String,
    pub source_tree: String,
    pub binary_sha256: String,
    pub qualification_run_id: String,
    pub independent_approver_id: String,
    pub assembled_roles: Vec<String>,
    pub assembly_valid: bool,
    pub production_authorization: String,
}

pub struct WorldProductionAdapterSet<'a> {
    pub identity: &'a dyn WorldProductionIdentityAdapter,
    pub session_guard: &'a dyn WorldProductionSessionGuard,
    pub account: &'a dyn WorldProductionAccountAdapter,
    pub repository: &'a dyn WorldProductionRepository,
    pub ledger: &'a dyn WorldProductionLedgerAdapter,
    pub evidence_sink: &'a dyn WorldProductionEvidenceSink,
    pub metrics_sink: &'a dyn WorldProductionMetricsSink,
    pub internal_routing: &'a dyn WorldProductionRoutingAdapter,
    pub deployment_identity: &'a dyn WorldProductionDeploymentAdapter,
}

pub struct WorldProductionRuntimeAssembly<'a> {
    adapters: WorldProductionAdapterSet<'a>,
    receipt: WorldProductionCompositionReceipt,
}

impl<'a> WorldProductionRuntimeAssembly<'a> {
    pub fn adapters(&self) -> &WorldProductionAdapterSet<'a> {
        &self.adapters
    }

    pub fn receipt(&self) -> &WorldProductionCompositionReceipt {
        &self.receipt
    }
}

fn invalid(message: impl Into<String>) -> WorldProductionAdapterError {
    WorldProductionAdapterError::new(
        WorldProductionAdapterErrorCode::InvalidInput,
        false,
        message,
    )
}

fn integrity(message: impl Into<String>) -> WorldProductionAdapterError {
    WorldProductionAdapterError::new(
        WorldProductionAdapterErrorCode::InternalIntegrity,
        false,
        message,
    )
}

fn require_text(value: &str, field: &str) -> WorldProductionAdapterResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > 512
        || value.bytes().any(|byte| byte.is_ascii_control())
    {
        return Err(invalid(format!("invalid composition field: {field}")));
    }
    Ok(())
}

fn require_lower_hex(value: &str, bytes: usize, field: &str) -> WorldProductionAdapterResult<()> {
    let expected = bytes * 2;
    if value.len() != expected
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!(
            "{field} must be exactly {expected} lowercase hexadecimal characters"
        )));
    }
    Ok(())
}

fn require_contract(value: &str, field: &str) -> WorldProductionAdapterResult<()> {
    if value != WORLD_PRODUCTION_ADAPTER_CONTRACT {
        return Err(WorldProductionAdapterError::new(
            WorldProductionAdapterErrorCode::UnsupportedContract,
            false,
            format!("{field} does not match the production adapter contract"),
        ));
    }
    Ok(())
}

fn rejects_fixture_credit(implementation_id: &str) -> bool {
    let value = implementation_id.to_ascii_lowercase();
    [
        "fixture",
        "mock",
        "stub",
        "in-memory",
        "in_memory",
        "dev-only",
        "development-only",
        "file-repository",
        "file_repository",
    ]
    .iter()
    .any(|marker| value.contains(marker))
}

fn validate_deployment_identity(
    identity: &WorldDeploymentIdentity,
) -> WorldProductionAdapterResult<()> {
    require_contract(&identity.adapter_contract, "deployment.adapter_contract")?;
    require_text(&identity.deployment_id, "deployment.deployment_id")?;
    require_text(&identity.environment, "deployment.environment")?;
    require_text(&identity.service_identity, "deployment.service_identity")?;
    require_text(&identity.component_lock_id, "deployment.component_lock_id")?;
    require_lower_hex(&identity.source_commit, 20, "deployment.source_commit")?;
    require_lower_hex(&identity.source_tree, 20, "deployment.source_tree")?;
    require_lower_hex(&identity.binary_sha256, 32, "deployment.binary_sha256")?;
    Ok(())
}

fn validate_evidence(
    evidence: &WorldProductionQualificationEvidence,
    deployment: &WorldDeploymentIdentity,
) -> WorldProductionAdapterResult<Vec<String>> {
    if evidence.composition_contract != WORLD_PRODUCTION_COMPOSITION_CONTRACT {
        return Err(WorldProductionAdapterError::new(
            WorldProductionAdapterErrorCode::UnsupportedContract,
            false,
            "composition contract mismatch",
        ));
    }
    require_contract(&evidence.adapter_contract, "evidence.adapter_contract")?;
    require_lower_hex(&evidence.source_commit, 20, "evidence.source_commit")?;
    require_lower_hex(&evidence.source_tree, 20, "evidence.source_tree")?;
    require_lower_hex(&evidence.binary_sha256, 32, "evidence.binary_sha256")?;
    require_text(&evidence.component_lock_id, "evidence.component_lock_id")?;
    require_text(&evidence.deployment_id, "evidence.deployment_id")?;
    require_text(
        &evidence.qualification_run_id,
        "evidence.qualification_run_id",
    )?;
    require_text(
        &evidence.independent_approver_id,
        "evidence.independent_approver_id",
    )?;
    require_text(&evidence.issued_at_utc, "evidence.issued_at_utc")?;
    if !evidence.issued_at_utc.ends_with('Z') || !evidence.issued_at_utc.contains('T') {
        return Err(invalid("evidence.issued_at_utc must be an explicit UTC timestamp"));
    }

    let exact_binding = evidence.source_commit == deployment.source_commit
        && evidence.source_tree == deployment.source_tree
        && evidence.binary_sha256 == deployment.binary_sha256
        && evidence.component_lock_id == deployment.component_lock_id
        && evidence.deployment_id == deployment.deployment_id;
    if !exact_binding {
        return Err(integrity(
            "qualification evidence is not bound to the deployment identity",
        ));
    }

    if evidence.adapters.len() != WORLD_PRODUCTION_REQUIRED_ROLES.len() {
        return Err(invalid(format!(
            "adapter evidence count mismatch expected={} actual={}",
            WORLD_PRODUCTION_REQUIRED_ROLES.len(),
            evidence.adapters.len()
        )));
    }

    let mut by_role: BTreeMap<&str, &WorldProductionAdapterQualification> = BTreeMap::new();
    for item in &evidence.adapters {
        require_text(&item.role, "adapter.role")?;
        require_text(&item.implementation_id, "adapter.implementation_id")?;
        require_text(
            &item.implementation_owner_id,
            "adapter.implementation_owner_id",
        )?;
        require_contract(&item.adapter_contract, "adapter.adapter_contract")?;
        require_text(&item.evidence_id, "adapter.evidence_id")?;
        require_lower_hex(&item.evidence_sha256, 32, "adapter.evidence_sha256")?;
        if by_role.insert(item.role.as_str(), item).is_some() {
            return Err(invalid(format!(
                "duplicate production adapter role: {}",
                item.role
            )));
        }
        if !WORLD_PRODUCTION_REQUIRED_ROLES.contains(&item.role.as_str()) {
            return Err(invalid(format!(
                "unknown production adapter role: {}",
                item.role
            )));
        }
        if !item.exact_head_qualified || !item.external_evidence {
            return Err(integrity(format!(
                "production adapter role lacks exact-head external evidence: {}",
                item.role
            )));
        }
        if rejects_fixture_credit(&item.implementation_id) {
            return Err(integrity(format!(
                "fixture or development implementation cannot receive production credit: {}",
                item.role
            )));
        }
        if item.implementation_owner_id == evidence.independent_approver_id {
            return Err(integrity(format!(
                "adapter implementer cannot be the independent approver: {}",
                item.role
            )));
        }
    }

    for role in WORLD_PRODUCTION_REQUIRED_ROLES {
        if !by_role.contains_key(role) {
            return Err(invalid(format!(
                "missing required production adapter role: {role}"
            )));
        }
    }
    Ok(WORLD_PRODUCTION_REQUIRED_ROLES
        .iter()
        .map(|role| (*role).to_string())
        .collect())
}

pub fn assemble_world_production_runtime<'a>(
    adapters: WorldProductionAdapterSet<'a>,
    evidence: &WorldProductionQualificationEvidence,
) -> WorldProductionAdapterResult<WorldProductionRuntimeAssembly<'a>> {
    let deployment = adapters.deployment_identity.deployment_identity()?;
    validate_deployment_identity(&deployment)?;
    let assembled_roles = validate_evidence(evidence, &deployment)?;
    let receipt = WorldProductionCompositionReceipt {
        composition_contract: WORLD_PRODUCTION_COMPOSITION_CONTRACT.to_string(),
        adapter_contract: WORLD_PRODUCTION_ADAPTER_CONTRACT.to_string(),
        deployment_id: deployment.deployment_id,
        component_lock_id: deployment.component_lock_id,
        source_commit: deployment.source_commit,
        source_tree: deployment.source_tree,
        binary_sha256: deployment.binary_sha256,
        qualification_run_id: evidence.qualification_run_id.clone(),
        independent_approver_id: evidence.independent_approver_id.clone(),
        assembled_roles,
        assembly_valid: true,
        production_authorization: WORLD_PRODUCTION_AUTHORIZATION.to_string(),
    };
    Ok(WorldProductionRuntimeAssembly { adapters, receipt })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RejectingAdapterSet {
        deployment: WorldDeploymentIdentity,
    }

    fn rejected() -> WorldProductionAdapterError {
        WorldProductionAdapterError::new(
            WorldProductionAdapterErrorCode::RepositoryUnavailable,
            true,
            "test adapter intentionally rejects calls",
        )
    }

    impl WorldProductionIdentityAdapter for RejectingAdapterSet {
        fn resolve_actor(
            &self,
            _request: &WorldIdentityResolutionRequest,
        ) -> WorldProductionAdapterResult<WorldActorIdentity> {
            Err(rejected())
        }
    }
    impl WorldProductionSessionGuard for RejectingAdapterSet {
        fn authorize_session(
            &self,
            _request: &WorldSessionAuthorizationRequest,
        ) -> WorldProductionAdapterResult<WorldSessionDecision> {
            Err(rejected())
        }
    }
    impl WorldProductionAccountAdapter for RejectingAdapterSet {
        fn apply_account_operation(
            &self,
            _request: &WorldAccountOperationRequest,
        ) -> WorldProductionAdapterResult<WorldAccountAuthDecision> {
            Err(rejected())
        }
    }
    impl WorldProductionRepository for RejectingAdapterSet {
        fn load_snapshot(
            &self,
            _request: &WorldAggregateLoadRequest,
        ) -> WorldProductionAdapterResult<WorldAggregateSnapshot> {
            Err(rejected())
        }
        fn lookup_request(
            &self,
            _request: &WorldAggregateRequestLookup,
        ) -> WorldProductionAdapterResult<Option<WorldAggregateCommitReceipt>> {
            Err(rejected())
        }
        fn commit(
            &self,
            _request: &WorldAggregateCommitRequest,
        ) -> WorldProductionAdapterResult<WorldAggregateCommitReceipt> {
            Err(rejected())
        }
    }
    impl WorldProductionLedgerAdapter for RejectingAdapterSet {
        fn lookup_reward(
            &self,
            _request: &WorldLedgerLookupRequest,
        ) -> WorldProductionAdapterResult<Option<WorldLedgerReceipt>> {
            Err(rejected())
        }
        fn submit_reward(
            &self,
            _request: &WorldLedgerSubmitRequest,
        ) -> WorldProductionAdapterResult<WorldLedgerReceipt> {
            Err(rejected())
        }
    }
    impl WorldProductionEvidenceSink for RejectingAdapterSet {
        fn append_evidence(
            &self,
            _request: &WorldEvidenceAppendRequest,
        ) -> WorldProductionAdapterResult<WorldEvidenceReceipt> {
            Err(rejected())
        }
    }
    impl WorldProductionMetricsSink for RejectingAdapterSet {
        fn record_metric(
            &self,
            _request: &WorldMetricRecordRequest,
        ) -> WorldProductionAdapterResult<WorldMetricReceipt> {
            Err(rejected())
        }
    }
    impl WorldProductionRoutingAdapter for RejectingAdapterSet {
        fn authorize_internal_route(
            &self,
            _request: &WorldRoutingAuthorizationRequest,
        ) -> WorldProductionAdapterResult<WorldRoutingDecision> {
            Err(rejected())
        }
    }
    impl WorldProductionDeploymentAdapter for RejectingAdapterSet {
        fn deployment_identity(&self) -> WorldProductionAdapterResult<WorldDeploymentIdentity> {
            Ok(self.deployment.clone())
        }
    }

    fn deployment() -> WorldDeploymentIdentity {
        WorldDeploymentIdentity {
            adapter_contract: WORLD_PRODUCTION_ADAPTER_CONTRACT.to_string(),
            deployment_id: "world-production-deployment-1".to_string(),
            environment: "production-candidate".to_string(),
            service_identity: "spiffe://trillionnium/world".to_string(),
            component_lock_id: "integration-lock-1".to_string(),
            source_commit: "a".repeat(40),
            source_tree: "b".repeat(40),
            binary_sha256: "c".repeat(64),
        }
    }

    fn evidence() -> WorldProductionQualificationEvidence {
        let deployment = deployment();
        WorldProductionQualificationEvidence {
            composition_contract: WORLD_PRODUCTION_COMPOSITION_CONTRACT.to_string(),
            adapter_contract: WORLD_PRODUCTION_ADAPTER_CONTRACT.to_string(),
            source_commit: deployment.source_commit,
            source_tree: deployment.source_tree,
            binary_sha256: deployment.binary_sha256,
            component_lock_id: deployment.component_lock_id,
            deployment_id: deployment.deployment_id,
            qualification_run_id: "external-run-1".to_string(),
            independent_approver_id: "independent-security-reviewer".to_string(),
            issued_at_utc: "2026-09-10T03:00:00Z".to_string(),
            adapters: WORLD_PRODUCTION_REQUIRED_ROLES
                .iter()
                .enumerate()
                .map(|(index, role)| WorldProductionAdapterQualification {
                    role: (*role).to_string(),
                    implementation_id: format!("prod-{role}-v1"),
                    implementation_owner_id: format!("owner-{index}"),
                    adapter_contract: WORLD_PRODUCTION_ADAPTER_CONTRACT.to_string(),
                    evidence_id: format!("evidence-{index}"),
                    evidence_sha256: format!("{:064x}", index + 1),
                    exact_head_qualified: true,
                    external_evidence: true,
                })
                .collect(),
        }
    }

    fn adapters(value: &RejectingAdapterSet) -> WorldProductionAdapterSet<'_> {
        WorldProductionAdapterSet {
            identity: value,
            session_guard: value,
            account: value,
            repository: value,
            ledger: value,
            evidence_sink: value,
            metrics_sink: value,
            internal_routing: value,
            deployment_identity: value,
        }
    }

    #[test]
    fn complete_exact_object_composition_is_constructible_but_not_authorized() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let assembly = assemble_world_production_runtime(adapters(&value), &evidence()).unwrap();
        assert!(assembly.receipt().assembly_valid);
        assert_eq!(assembly.receipt().assembled_roles.len(), 9);
        assert_eq!(
            assembly.receipt().production_authorization,
            WORLD_PRODUCTION_AUTHORIZATION
        );
        assert_eq!(assembly.receipt().production_authorization, "not_granted");
        let _ = assembly.adapters().identity;
    }

    #[test]
    fn missing_role_fails_closed() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let mut proof = evidence();
        proof.adapters.pop();
        assert!(assemble_world_production_runtime(adapters(&value), &proof).is_err());
    }

    #[test]
    fn duplicate_role_fails_closed() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let mut proof = evidence();
        proof.adapters[1].role = proof.adapters[0].role.clone();
        assert!(assemble_world_production_runtime(adapters(&value), &proof).is_err());
    }

    #[test]
    fn fixture_credit_fails_closed() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let mut proof = evidence();
        proof.adapters[0].implementation_id = "fixture-identity".to_string();
        assert!(assemble_world_production_runtime(adapters(&value), &proof).is_err());
    }

    #[test]
    fn non_independent_approval_fails_closed() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let mut proof = evidence();
        proof.adapters[0].implementation_owner_id = proof.independent_approver_id.clone();
        assert!(assemble_world_production_runtime(adapters(&value), &proof).is_err());
    }

    #[test]
    fn exact_head_mismatch_fails_closed() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let mut proof = evidence();
        proof.source_tree = "d".repeat(40);
        assert!(assemble_world_production_runtime(adapters(&value), &proof).is_err());
    }

    #[test]
    fn role_without_external_qualification_fails_closed() {
        let value = RejectingAdapterSet {
            deployment: deployment(),
        };
        let mut proof = evidence();
        proof.adapters[0].external_evidence = false;
        assert!(assemble_world_production_runtime(adapters(&value), &proof).is_err());
    }
}
