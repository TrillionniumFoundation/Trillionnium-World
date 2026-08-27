use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn required_string<'a>(value: &'a Value, key: &str) -> &'a str {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("registry field {key} must be a string"))
}

fn is_exact_commit(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[test]
fn p0_registry_has_one_owner_per_target_capability_and_every_slice_is_explicit() {
    let root = repository_root();
    let path = root.join("docs/status/p0-execution.json");
    let registry: Value = serde_json::from_str(
        &fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    )
    .expect("P0 registry must be valid JSON");

    assert_eq!(
        required_string(&registry, "contract_version"),
        "trnm_world_p0_execution_registry_v1"
    );
    assert_eq!(required_string(&registry, "project_id"), "trillionnium-world");

    let target = registry
        .get("target_architecture")
        .and_then(Value::as_object)
        .expect("target_architecture must be an object");
    let expected = BTreeMap::from([
        ("world", "deterministic_game_domain"),
        ("nakama", "online_match_authority_and_completion_signing"),
        ("chain", "ingress_finality_and_inclusion_proof"),
        ("cex", "wallet_ledger_and_custody"),
        ("integration", "cross_repository_lock_and_e2e_evidence"),
    ]);
    assert_eq!(target.len(), expected.len());
    for (owner, capability) in expected {
        assert_eq!(
            target.get(owner).and_then(Value::as_str),
            Some(capability),
            "authority owner {owner} drifted"
        );
    }

    let slices = registry
        .get("slices")
        .and_then(Value::as_array)
        .expect("slices must be an array");
    let required_slices = BTreeSet::from(["P0.1", "P0.2", "P0.3", "P0.4"]);
    let mut observed = BTreeSet::new();
    let allowed_status = BTreeSet::from(["planned", "in_review", "blocked", "done"]);
    let allowed_evidence = BTreeSet::from([
        "source_implemented",
        "remote_ci_verified",
        "deployed",
        "operationally_observed",
        "release_approved",
    ]);

    for slice in slices {
        let slice_id = required_string(slice, "slice_id");
        assert!(observed.insert(slice_id), "duplicate registry slice {slice_id}");
        let status = required_string(slice, "status");
        let evidence = required_string(slice, "evidence_level");
        assert!(allowed_status.contains(status), "invalid status for {slice_id}");
        assert!(
            allowed_evidence.contains(evidence),
            "invalid evidence level for {slice_id}"
        );
        assert_eq!(
            required_string(slice, "repository"),
            "TrillionniumFoundation/Trillionnium-World"
        );
        let limitations = slice
            .get("limitations")
            .and_then(Value::as_array)
            .expect("limitations must be an array");
        assert!(
            !limitations.is_empty(),
            "{slice_id} must retain explicit limitations"
        );
        assert!(limitations.iter().all(|item| {
            item.as_str()
                .is_some_and(|text| !text.trim().is_empty())
        }));

        let requires_remote_identity = matches!(
            evidence,
            "remote_ci_verified" | "deployed" | "operationally_observed" | "release_approved"
        );
        let commit = slice.get("commit").and_then(Value::as_str);
        let workflow = slice.get("workflow_run").and_then(Value::as_str);
        if requires_remote_identity {
            assert!(
                commit.is_some_and(is_exact_commit),
                "{slice_id} claims {evidence} without an exact commit"
            );
            assert!(
                workflow.is_some_and(|value| !value.trim().is_empty()),
                "{slice_id} claims {evidence} without a workflow run"
            );
        }
        if status == "done" {
            assert_ne!(
                evidence, "source_implemented",
                "{slice_id} cannot be done on source-only evidence"
            );
        }
        if status == "in_review" {
            assert!(
                slice
                    .get("branch")
                    .and_then(Value::as_str)
                    .is_some_and(|branch| branch.starts_with("feature/world-")),
                "{slice_id} in_review must identify its feature branch"
            );
        }
    }
    assert_eq!(observed, required_slices);
}

#[test]
fn p0_registry_cannot_promote_public_online_or_market_without_a_contract_change() {
    let root = repository_root();
    let registry: Value = serde_json::from_str(
        &fs::read_to_string(root.join("docs/status/p0-execution.json")).unwrap(),
    )
    .unwrap();
    let gates = registry
        .get("release_gates")
        .and_then(Value::as_object)
        .expect("release_gates must be an object");
    assert_eq!(gates.get("public_online").and_then(Value::as_str), Some("no_go"));
    assert_eq!(
        gates.get("public_player_market").and_then(Value::as_str),
        Some("disabled")
    );
    assert_ne!(
        gates.get("trusted_cex_settlement").and_then(Value::as_str),
        Some("release_approved")
    );
}

#[test]
fn p0_registry_references_current_source_documents_that_exist() {
    let root = repository_root();
    for relative in [
        "PROJECT_BOUNDARY.md",
        "PROJECT_BOUNDARY.json",
        "docs/README.md",
        "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md",
        "docs/development/trnm-world-development-plan-v2.md",
        "docs/development/trnm-world-p0-execution-spec-v1.md",
        "docs/protocol/trnm-match-evidence-commitment-v1.md",
    ] {
        let path = root.join(relative);
        assert!(Path::new(&path).is_file(), "missing current document {relative}");
    }
}
