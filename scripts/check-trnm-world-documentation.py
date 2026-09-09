#!/usr/bin/env python3
"""Fail-closed consistency checks for current Trillionnium World documentation."""

from __future__ import annotations

import pathlib
import re
import subprocess
import sys
import tomllib
from typing import Any

from trnm_world_strict_json import StrictJsonError, load_strict_json

ROOT = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else pathlib.Path(__file__).resolve().parents[1]
PLAN_ID = "trillionnium-world-development-2026-08-29-v4"
HISTORICAL_PLAN = "docs/development/trillionnium-world-development-plan-2026-08-29.json"
HISTORICAL_LEDGER = "docs/development/trnm-world-gap-closure-ledger-v4.json"
CURRENT_LEDGER = "docs/development/trnm-world-gap-closure-ledger-v6.json"
CURRENT_CEX_LOCK = "docs/integration/trnm-world-cex-current-pending-lock-v2.json"
CURRENT_SNAPSHOT = "docs/status/world-plan-v4-execution-truth-2026-09-08.json"
EVIDENCE_CLASSES = {
    "source_static", "unit", "database_black_box", "single_host_runtime",
    "cross_repository_integration", "cross_host", "public_network", "human",
    "custody_security", "commercial_legal",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World documentation: FAIL: {message}")


def read(path: str) -> str:
    target = ROOT / path
    if not target.is_file() or target.is_symlink():
        fail(f"missing or symlinked required file: {path}")
    try:
        text = target.read_bytes().decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        fail(f"invalid UTF-8 {path}: {error}")
    if not text.strip():
        fail(f"required file is empty: {path}")
    return text


def load(path: str) -> dict[str, Any]:
    try:
        value = load_strict_json(ROOT / path)
    except (OSError, StrictJsonError) as error:
        fail(f"invalid strict JSON {path}: {error}")
    if not isinstance(value, dict):
        fail(f"JSON root must be an object: {path}")
    return value


required_paths = [
    "README.md", "CURRENT_PLAN.md", "RELEASE_READINESS.md", "PROJECT_BOUNDARY.md",
    "PROJECT_BOUNDARY.json", "AGENTS.md", "OPERATIONS.md", "rust-toolchain.toml",
    "docs/README.md", "docs/catalog.json",
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md",
    HISTORICAL_PLAN, HISTORICAL_LEDGER, CURRENT_LEDGER, CURRENT_CEX_LOCK, CURRENT_SNAPSHOT,
    "docs/status/CURRENT.md", "docs/status/world-gates-v1.json",
    "docs/architecture/trnm-world-system-context-v1.md",
    "docs/architecture/trnm-world-authority-state-ownership-v1.md",
    "docs/architecture/trnm-settlement-lifecycle-v2.md",
    "docs/architecture/trnm-determinism-and-canonical-json-v1.md",
    "docs/architecture/cex-world-authority-cutover-v1.md",
    "docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md",
    "docs/security/trnm-world-threat-model-v1.md",
    "docs/database/trnm-world-postgres-contract-v1.md",
    "docs/release/trnm-world-release-gate-matrix-v2.md",
    "docs/release/trnm-world-evidence-record-v1.md",
    "docs/runbooks/trnm-settlement-shutdown-and-quarantine-v1.md",
    "docs/runbooks/trnm-authority-cutover-rollback-v1.md",
    "docs/contracts/trillionnium-world-authority-cutover-v1.json",
    "docs/contracts/trillionnium-world-authority-provenance-v1.json",
    "docs/modules/README.md", "docs/modules/world-authority/README.md",
    "trillionnium/crates/world-authority/Cargo.toml",
    "trillionnium/crates/world-authority/README.md",
    "scripts/check-trnm-world-active-closure.py",
    "scripts/test-trnm-world-active-closure.py",
    "scripts/check-trnm-world-detailed-documentation.py",
    "scripts/test-trnm-world-detailed-documentation.py",
    "scripts/check-trnm-world-authority-documentation.py",
    "scripts/test-trnm-world-authority-documentation.py",
    "scripts/check-trnm-world-cex-pending-lock.py",
]
for path in required_paths:
    read(path)

toolchain = tomllib.loads(read("rust-toolchain.toml")).get("toolchain", {})
if toolchain.get("channel") != "1.98.1":
    fail("current candidate toolchain must be Rust 1.98.1")

current_plan = read("CURRENT_PLAN.md")
for path in (CURRENT_LEDGER, CURRENT_CEX_LOCK, CURRENT_SNAPSHOT, "RELEASE_READINESS.md"):
    if path not in current_plan:
        fail(f"CURRENT_PLAN.md does not point to {path}")

boundary = load("PROJECT_BOUNDARY.json")
if boundary.get("schema") != 3:
    fail("PROJECT_BOUNDARY.json schema must be 3")
if boundary.get("project_id") != "trillionnium-world" or boundary.get("lane") != "game-product":
    fail("project identity/lane drift")
documentation = boundary.get("documentation", {})
if not isinstance(documentation, dict):
    fail("boundary documentation section missing")
expected_pointers = {
    "current_plan": "CURRENT_PLAN.md",
    "current_execution_snapshot": CURRENT_SNAPSHOT,
    "current_gap_ledger": CURRENT_LEDGER,
    "current_cex_pending_lock": CURRENT_CEX_LOCK,
    "release_readiness": "RELEASE_READINESS.md",
}
for key, value in expected_pointers.items():
    if documentation.get(key) != value:
        fail(f"boundary documentation pointer drift: {key}")
if boundary.get("ci", {}).get("validation_repository_permissions") != "read_only":
    fail("validation CI must be read-only")
if boundary.get("ci", {}).get("self_modifying_candidate_source") != "forbid":
    fail("self-modifying candidate source must be forbidden")
if boundary.get("release", {}).get("public_online") != "no_go":
    fail("public online overclaim")
if boundary.get("release", {}).get("public_player_market") != "disabled":
    fail("public player market overclaim")
if boundary.get("release", {}).get("production_authorization") != "not_granted":
    fail("production authorization overclaim")

catalog = load("docs/catalog.json")
if catalog.get("schema") != "trnm_world_document_catalog_v1":
    fail("catalog schema drift")
truth_order = catalog.get("truth_order")
if not isinstance(truth_order, list) or len(truth_order) != len(set(truth_order)):
    fail("catalog truth order invalid")
required_truth = [
    "PROJECT_BOUNDARY.md", "CURRENT_PLAN.md", "docs/catalog.json", CURRENT_SNAPSHOT,
    "docs/status/CURRENT.md", "RELEASE_READINESS.md", CURRENT_LEDGER, CURRENT_CEX_LOCK,
]
if truth_order[:5] != required_truth[:5] or not set(required_truth).issubset(set(truth_order)):
    fail("current truth hierarchy drift")
documents = catalog.get("documents")
if not isinstance(documents, list):
    fail("catalog documents missing")
catalog_paths = [item.get("path") for item in documents if isinstance(item, dict)]
if len(catalog_paths) != len(set(catalog_paths)):
    fail("duplicate catalog paths")
for path in required_truth:
    if path not in catalog_paths:
        fail(f"current truth file not registered in catalog: {path}")

historical_plan = load(HISTORICAL_PLAN)
if historical_plan.get("schema") != "trnm_world_development_plan_v4" or historical_plan.get("plan_id") != PLAN_ID:
    fail("historical machine plan identity drift")
if historical_plan.get("public_online") != "no_go" or historical_plan.get("public_player_market") != "disabled":
    fail("historical plan release overclaim")
if set(historical_plan.get("evidence_classes", [])) != EVIDENCE_CLASSES:
    fail("historical plan evidence classes drift")

historical_ledger = load(HISTORICAL_LEDGER)
if historical_ledger.get("schema") != "trnm_world_gap_closure_ledger_v4" or historical_ledger.get("plan_id") != PLAN_ID:
    fail("historical gap ledger identity drift")
entries = historical_ledger.get("entries")
if not isinstance(entries, list) or not entries:
    fail("historical gap ledger entries missing")
ids: set[str] = set()
for entry in entries:
    if not isinstance(entry, dict):
        fail("historical gap ledger entry is not an object")
    item_id = entry.get("id")
    if not isinstance(item_id, str) or not re.fullmatch(r"WORLD-P[0-2]-[0-9]{3}[A-Z]?", item_id):
        fail(f"invalid historical gap id: {item_id!r}")
    if item_id in ids:
        fail(f"duplicate historical gap id: {item_id}")
    ids.add(item_id)
    if entry.get("evidence_class") not in EVIDENCE_CLASSES:
        fail(f"invalid evidence class for {item_id}")
    if not isinstance(entry.get("acceptance"), list) or not entry["acceptance"]:
        fail(f"missing acceptance criteria for {item_id}")
    if not isinstance(entry.get("depends_on"), list):
        fail(f"invalid dependency list for {item_id}")

current_ledger = load(CURRENT_LEDGER)
if current_ledger.get("schema") != "trnm_world_gap_closure_ledger_v6":
    fail("current V6 ledger identity drift")
if current_ledger.get("closure", {}).get("all_plan_gaps_closed") is not False:
    fail("current V6 ledger overclaims complete closure")
if current_ledger.get("closure", {}).get("production_authorization") != "not_granted":
    fail("current V6 ledger production authorization drift")

current_lock = load(CURRENT_CEX_LOCK)
if current_lock.get("schema") != "trnm_world_cex_pending_component_lock_v2":
    fail("current CEX lock identity drift")
if current_lock.get("qualified") is not False or current_lock.get("world_adoption") != "disabled":
    fail("current CEX lock overclaims qualification/adoption")

world_gates = load("docs/status/world-gates-v1.json")
by_id = {item.get("id"): item for item in world_gates.get("gates", []) if isinstance(item, dict)}
if by_id.get("public_online", {}).get("status") != "no_go":
    fail("world gate public_online must remain no_go")
if by_id.get("public_player_market", {}).get("status") != "disabled":
    fail("world gate public_player_market must remain disabled")

for path in ("README.md", "CURRENT_PLAN.md", "RELEASE_READINESS.md", "docs/README.md", "PROJECT_BOUNDARY.md", "AGENTS.md"):
    text = read(path)
    if "/Users/" in text or ".openclaw/workspace" in text:
        fail(f"personal path remains in current file {path}")

workflow_dir = ROOT / ".github/workflows"
if not workflow_dir.is_dir():
    fail("workflow directory is missing")
for workflow in sorted(workflow_dir.glob("*.yml")) + sorted(workflow_dir.glob("*.yaml")):
    text = workflow.read_text(encoding="utf-8")
    normalized = text.lower()
    if re.search(r"(?m)^\s*contents:\s*write\s*$", normalized):
        fail(f"write-enabled validation workflow: {workflow.relative_to(ROOT)}")
    for pattern, label in ((r"\bgit\s+push(?:\s|$)", "git push"), (r"\bgit\s+commit(?:\s|$)", "git commit"), (r"clippy\s+--fix", "clippy --fix")):
        if re.search(pattern, normalized):
            fail(f"self-modifying workflow marker {label!r}: {workflow.relative_to(ROOT)}")

for removed in (
    ".github/workflows/apply-world-settlement-gap-closure-v1.yml",
    ".github/workflows/trnm-world-settlement-self-heal.yml",
    ".github/workflows/world-settlement-converge.yml",
):
    if (ROOT / removed).exists():
        fail(f"retired self-modifying workflow still exists: {removed}")

for relative, arguments in (
    ("scripts/check-trnm-world-execution-truth.py", [str(ROOT)]),
    ("scripts/test-trnm-world-execution-truth.py", []),
    ("scripts/test-trnm-world-qualified-checkout.py", []),
    ("scripts/check-trnm-world-machine-truth.py", [str(ROOT)]),
    ("scripts/test-trnm-world-machine-truth.py", []),
    ("scripts/check-trnm-world-active-closure.py", [str(ROOT)]),
    ("scripts/test-trnm-world-active-closure.py", []),
    ("scripts/check-trnm-world-detailed-documentation.py", [str(ROOT)]),
    ("scripts/test-trnm-world-detailed-documentation.py", []),
    ("scripts/check-trnm-world-authority-documentation.py", [str(ROOT)]),
    ("scripts/test-trnm-world-authority-documentation.py", []),
    ("scripts/check-trnm-world-cex-pending-lock.py", [str(ROOT)]),
):
    path = ROOT / relative
    if not path.is_file() or path.is_symlink():
        fail(f"missing or linked documentation/truth gate: {relative}")
    try:
        result = subprocess.run([sys.executable, str(path), *arguments], check=False, timeout=240)
    except (OSError, subprocess.TimeoutExpired):
        fail(f"documentation/truth gate unavailable: {relative}")
    if result.returncode != 0:
        fail(f"documentation/truth gate failed: {relative}")

print("TRNM World documentation and CI integrity: PASS")
