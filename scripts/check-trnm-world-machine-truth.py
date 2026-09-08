#!/usr/bin/env python3
"""Validate the hierarchy and no-credit semantics of current World machine truth."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
from typing import Any

from trnm_world_strict_json import StrictJsonError, load_strict_json

CURRENT_SNAPSHOT = "docs/status/world-plan-v4-execution-truth-2026-09-08.json"
HISTORICAL_SNAPSHOT = "docs/status/world-plan-v4-execution-truth-2026-09-02.json"
HISTORICAL_PLAN = "docs/development/trillionnium-world-development-plan-2026-08-29.json"
HISTORICAL_LEDGER = "docs/development/trnm-world-gap-closure-ledger-v4.json"
HISTORICAL_GATES = "docs/status/world-gates-v1.json"
PROTECTION_CONTRACT = "docs/governance/main-protection-contract-v2.json"
CUTOVER_CONTRACT = "docs/contracts/trillionnium-world-authority-cutover-v1.json"
PROVENANCE_CONTRACT = "docs/contracts/trillionnium-world-authority-provenance-v1.json"
OPERATIVE_REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
OPERATIVE_PULL_REQUEST = 104
OPERATIVE_BRANCH = "fix/world-convergence-v6-20260908"
PRODUCTION_ADAPTER_CONTRACT = "trillionnium_world_production_adapter_v1"
EXPECTED_PRODUCTION_ADAPTERS = {
    "identity", "session_guard", "account", "durable_repository", "ledger",
    "evidence_sink", "metrics_sink", "internal_routing", "deployment_identity",
}
TRUTH_PREFIX = (
    "PROJECT_BOUNDARY.md", "CURRENT_PLAN.md", "docs/catalog.json",
    CURRENT_SNAPSHOT, "docs/status/CURRENT.md",
)
SHA1_RE = re.compile(r"^[0-9a-f]{40}$")


class MachineTruthFailure(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise MachineTruthFailure(message)


def obj(value: Any, label: str) -> dict[str, Any]:
    require(isinstance(value, dict), f"{label} must be an object")
    return value


def array(value: Any, label: str) -> list[Any]:
    require(isinstance(value, list), f"{label} must be a list")
    return value


def sha1(value: Any, label: str) -> str:
    require(isinstance(value, str) and SHA1_RE.fullmatch(value) is not None, f"{label} is not a Git SHA-1")
    return value


def read_text(root: Path, relative: str, maximum_bytes: int = 256 * 1024) -> str:
    path = root / relative
    require(path.is_file() and not path.is_symlink(), f"missing or symlinked text source: {relative}")
    size = path.stat().st_size
    require(0 < size <= maximum_bytes, f"empty or oversized text source: {relative}")
    try:
        return path.read_bytes().decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise MachineTruthFailure(f"invalid UTF-8 in {relative}: {error}") from error


def load_sources(root: Path) -> tuple[dict[str, Any], str]:
    paths = (
        "PROJECT_BOUNDARY.json", "docs/catalog.json", CURRENT_SNAPSHOT,
        HISTORICAL_SNAPSHOT, HISTORICAL_PLAN, HISTORICAL_LEDGER,
        HISTORICAL_GATES, PROTECTION_CONTRACT, CUTOVER_CONTRACT,
        PROVENANCE_CONTRACT,
    )
    documents: dict[str, Any] = {}
    for relative in paths:
        try:
            documents[relative] = load_strict_json(root / relative)
        except (OSError, StrictJsonError) as error:
            raise MachineTruthFailure(f"cannot load {relative}: {error}") from error
        require(isinstance(documents[relative], dict), f"{relative} root must be an object")
    return documents, read_text(root, "CURRENT_PLAN.md")


def catalog_entry(catalog: dict[str, Any], path: str) -> dict[str, Any]:
    matches = [entry for entry in array(catalog.get("documents"), "catalog documents")
               if isinstance(entry, dict) and entry.get("path") == path]
    require(len(matches) == 1, f"catalog must contain exactly one entry for {path}")
    return matches[0]


def gate_by_id(gates: dict[str, Any], gate_id: str) -> dict[str, Any]:
    matches = [entry for entry in array(gates.get("gates"), "gate list")
               if isinstance(entry, dict) and entry.get("id") == gate_id]
    require(len(matches) == 1, f"gate list must contain exactly one {gate_id}")
    return matches[0]


def authorization_values(value: Any, path: str = "$") -> list[tuple[str, Any]]:
    result: list[tuple[str, Any]] = []
    if isinstance(value, dict):
        for key, item in value.items():
            child = f"{path}.{key}"
            if key == "production_authorization":
                result.append((child, item))
            result.extend(authorization_values(item, child))
    elif isinstance(value, list):
        for index, item in enumerate(value):
            result.extend(authorization_values(item, f"{path}[{index}]"))
    return result


def validate_documents(documents: dict[str, Any], current_plan: str) -> None:
    boundary = obj(documents.get("PROJECT_BOUNDARY.json"), "boundary")
    catalog = obj(documents.get("docs/catalog.json"), "catalog")
    snapshot = obj(documents.get(CURRENT_SNAPSHOT), "current snapshot")
    old_snapshot = obj(documents.get(HISTORICAL_SNAPSHOT), "historical snapshot")
    old_plan = obj(documents.get(HISTORICAL_PLAN), "historical plan")
    old_ledger = obj(documents.get(HISTORICAL_LEDGER), "historical ledger")
    old_gates = obj(documents.get(HISTORICAL_GATES), "historical gates")
    protection = obj(documents.get(PROTECTION_CONTRACT), "protection contract")
    cutover = obj(documents.get(CUTOVER_CONTRACT), "cutover contract")
    provenance = obj(documents.get(PROVENANCE_CONTRACT), "provenance contract")

    require(boundary.get("schema") == 3, "boundary schema must remain 3")
    require(boundary.get("project_id") == "trillionnium-world" and boundary.get("lane") == "game-product", "project identity/lane drift")
    require(obj(boundary.get("remote"), "boundary remote").get("canonical_slug") == OPERATIVE_REPOSITORY, "canonical repository drift")
    docs = obj(boundary.get("documentation"), "boundary documentation")
    require(docs.get("current_plan") == "CURRENT_PLAN.md", "CURRENT_PLAN.md must be binding")
    require(docs.get("current_execution_snapshot") == CURRENT_SNAPSHOT, "current snapshot pointer drift")
    require(docs.get("historical_plan_manifest") == HISTORICAL_PLAN, "historical plan pointer drift")
    require(docs.get("historical_gap_ledger") == HISTORICAL_LEDGER, "historical ledger pointer drift")
    require(docs.get("historical_machine_inputs_are_execution_authority") is False, "historical machine inputs became execution authority")
    authority = obj(boundary.get("authority"), "boundary authority")
    require(authority.get("world_domain_service_workspace") == "trillionnium/crates/world-authority/Cargo.toml", "domain-service workspace drift")
    require(authority.get("world_production_adapter_contract") == PRODUCTION_ADAPTER_CONTRACT, "production adapter contract drift")
    require(authority.get("world_production_authorization") == "not_granted", "boundary production authorization drift")
    require(obj(boundary.get("runtime"), "boundary runtime").get("fixture_listener") == "explicit-opt-in-loopback-only", "fixture listener policy drift")
    release = obj(boundary.get("release"), "boundary release")
    require(release.get("public_online") == "no_go" and release.get("public_player_market") == "disabled", "public release posture weakened")
    require(release.get("production_authorization") == "not_granted", "release production authorization drift")

    require(catalog.get("schema") == "trnm_world_document_catalog_v1", "catalog schema drift")
    truth_order = array(catalog.get("truth_order"), "catalog truth_order")
    require(tuple(truth_order[: len(TRUTH_PREFIX)]) == TRUTH_PREFIX, "catalog truth hierarchy drift")
    require(len(truth_order) == len(set(truth_order)), "catalog truth_order duplicate")
    require(not ({HISTORICAL_PLAN, HISTORICAL_LEDGER, HISTORICAL_GATES} & set(truth_order)), "historical input entered current truth_order")
    current_entry = catalog_entry(catalog, CURRENT_SNAPSHOT)
    require(current_entry.get("class") == "execution-snapshot" and current_entry.get("status") == "current", "current snapshot catalog classification drift")
    require(catalog_entry(catalog, HISTORICAL_SNAPSHOT).get("class") == "historical-execution-snapshot", "old snapshot not classified historical")

    for token in (
        "Pull request: `#104`", f"Branch: `{OPERATIVE_BRANCH}`", CURRENT_SNAPSHOT,
        HISTORICAL_PLAN, HISTORICAL_LEDGER, "Production authorization remains **not granted**",
    ):
        require(token in current_plan, f"CURRENT_PLAN.md missing: {token}")

    require(snapshot.get("schema") == "trnm_world_plan_v4_execution_truth_v1", "snapshot schema drift")
    require(snapshot.get("repository") == OPERATIVE_REPOSITORY, "snapshot repository drift")
    require(snapshot.get("operative_pull_request") == OPERATIVE_PULL_REQUEST and snapshot.get("operative_branch") == OPERATIVE_BRANCH, "operative candidate drift")
    observed_head = sha1(snapshot.get("observed_head_before_truth_update"), "observed head")
    sha1(snapshot.get("observed_tree_before_truth_update"), "observed tree")
    actions = obj(snapshot.get("github_actions"), "github_actions")
    require(actions.get("repository_workflow_run_total") == 0, "unverified hosted run count promoted")
    require(actions.get("state") == "scheduler_no_exact_head_runs" and actions.get("exact_head_evidence") == "absent", "hosted evidence state promoted")
    require(actions.get("exact_head") == observed_head, "exact-head observation mismatch")
    sha1(actions.get("prospective_merge"), "prospective merge")
    closure = obj(snapshot.get("closure"), "closure")
    for key in ("world_owned_source_development_closed", "exact_head_ci_closed", "independent_review_closed", "server_governance_closed", "all_plan_gaps_closed"):
        require(closure.get(key) is False, f"unproved closure promoted: {key}")
    require(closure.get("production_authorization") == "not_granted", "snapshot production authorization drift")
    external = obj(snapshot.get("external_evidence_gaps"), "external gaps")
    require(external and all(value == "open" for value in external.values()), "external gap promoted without evidence")

    require(old_snapshot.get("schema") == "trnm_world_plan_v4_execution_truth_v1" and old_snapshot.get("operative_pull_request") != OPERATIVE_PULL_REQUEST, "old snapshot was rewritten as current")
    require(old_plan.get("plan_id") == "trillionnium-world-development-2026-08-29-v4" and old_plan.get("as_of") == "2026-08-29", "historical plan identity/date drift")
    require(old_plan.get("working_branch") != OPERATIVE_BRANCH and old_plan.get("verified_commit") is None and old_plan.get("release_effect") == "none", "historical plan was silently promoted")
    require(old_ledger.get("plan_id") == "trillionnium-world-development-2026-08-29-v4" and old_ledger.get("as_of") == "2026-08-29", "historical ledger identity/date drift")
    require(old_ledger.get("candidate_branch") != OPERATIVE_BRANCH and old_ledger.get("verified_commit") is None and old_ledger.get("release_effect") == "none", "historical ledger was silently promoted")
    require(old_gates.get("as_of") == "2026-08-29", "historical gates date drift")
    require(gate_by_id(old_gates, "public_online").get("status") == "no_go", "historical public-online no-go weakened")
    require(gate_by_id(old_gates, "public_player_market").get("status") == "disabled", "historical public-market disablement weakened")
    for gate in array(old_gates.get("gates"), "historical gates"):
        require(isinstance(gate, dict) and gate.get("evidence") == [], f"historical gate gained unbound evidence: {gate.get('id') if isinstance(gate, dict) else '?'}")

    require(protection.get("schema") == "trnm_world_main_protection_contract_v2", "protection schema drift")
    require(protection.get("repository") == OPERATIVE_REPOSITORY and protection.get("branch") == "main", "protection target drift")
    require(protection.get("server_state") == "not_enforced_until_api_readback", "desired protection misrepresented as enforced")
    checks = obj(protection.get("required_status_checks"), "required checks")
    contexts = array(checks.get("contexts"), "required contexts")
    require(checks.get("strict") is True and checks.get("non_empty_execution_required") is True, "required checks weakened")
    require(contexts and len(contexts) == len(set(contexts)), "required contexts empty or duplicated")
    require(checks.get("accepted_conclusions") == ["success"], "accepted check conclusions drift")

    require(cutover.get("schema") == "trillionnium.world.authority.cutover.v1", "cutover schema drift")
    production = obj(obj(cutover.get("profiles"), "cutover profiles").get("production"), "production profile")
    require(production.get("adapter_contract") == PRODUCTION_ADAPTER_CONTRACT, "cutover adapter contract drift")
    require(production.get("fixture_adapters_allowed") is False and production.get("file_repository_allowed") is False and production.get("fail_closed") is True, "production fixture/fail-close policy drift")
    require(production.get("implementation_state") == "absent" and production.get("exact_head_qualified") is False, "unimplemented adapters promoted")
    adapters = array(production.get("required_adapters"), "required adapters")
    require(set(adapters) == EXPECTED_PRODUCTION_ADAPTERS and len(adapters) == len(EXPECTED_PRODUCTION_ADAPTERS), "production adapter inventory drift")

    require(provenance.get("schema") == "trillionnium.world.authority.provenance.v2", "provenance schema drift")
    counts = obj(provenance.get("counts"), "provenance counts")
    require(counts.get("baseline_files") == 15 and counts.get("current_files") == 27, "provenance inventory count drift")
    require(counts.get("exact_historical") == 13 and counts.get("reviewed_successor") == 2 and counts.get("new_qualification") == 12, "provenance classification drift")

    for path in ("PROJECT_BOUNDARY.json", CURRENT_SNAPSHOT, PROTECTION_CONTRACT, CUTOVER_CONTRACT, PROVENANCE_CONTRACT):
        values = authorization_values(documents[path])
        require(values, f"{path} lacks explicit production_authorization")
        for location, value in values:
            require(value == "not_granted", f"positive or malformed production authorization at {location}: {value!r}")


def validate(root: Path) -> None:
    documents, plan = load_sources(root.resolve())
    validate_documents(documents, plan)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        validate(args.root)
    except (MachineTruthFailure, OSError, UnicodeError) as error:
        print(f"TRNM World machine truth: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM World machine truth: PASS (current hierarchy, historical isolation and no-credit semantics)")
    print("Hosted CI, server governance, external systems and production authorization remain evidence-bound.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
