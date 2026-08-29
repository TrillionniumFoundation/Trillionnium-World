#!/usr/bin/env python3
"""Fail-closed consistency checks for Trillionnium World current documentation."""

from __future__ import annotations

import json
import pathlib
import re
import sys
from typing import Any

ROOT = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else pathlib.Path(__file__).resolve().parents[1]
PLAN_ID = "trillionnium-world-development-2026-08-29-v4"
PLAN_MD = "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md"
PLAN_JSON = "docs/development/trillionnium-world-development-plan-2026-08-29.json"
LEDGER_JSON = "docs/development/trnm-world-gap-closure-ledger-v4.json"
EVIDENCE_CLASSES = {
    "source_static",
    "unit",
    "database_black_box",
    "single_host_runtime",
    "cross_repository_integration",
    "cross_host",
    "public_network",
    "human",
    "custody_security",
    "commercial_legal",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World documentation: FAIL: {message}")


def read(path: str) -> str:
    target = ROOT / path
    if not target.is_file():
        fail(f"missing required file: {path}")
    text = target.read_text(encoding="utf-8")
    if not text.strip():
        fail(f"required file is empty: {path}")
    return text


def load(path: str) -> Any:
    try:
        return json.loads(read(path))
    except json.JSONDecodeError as error:
        fail(f"invalid JSON {path}: {error}")


required_paths = [
    "README.md",
    "CURRENT_PLAN.md",
    "PROJECT_BOUNDARY.md",
    "PROJECT_BOUNDARY.json",
    "AGENTS.md",
    "docs/README.md",
    PLAN_MD,
    PLAN_JSON,
    LEDGER_JSON,
    "docs/status/CURRENT.md",
    "docs/status/world-gates-v1.json",
    "docs/architecture/trnm-world-system-context-v1.md",
    "docs/architecture/trnm-world-authority-state-ownership-v1.md",
    "docs/architecture/trnm-settlement-lifecycle-v2.md",
    "docs/architecture/trnm-determinism-and-canonical-json-v1.md",
    "docs/security/trnm-world-threat-model-v1.md",
    "docs/database/trnm-world-postgres-contract-v1.md",
    "docs/release/trnm-world-release-gate-matrix-v2.md",
    "docs/release/trnm-world-evidence-record-v1.md",
    "docs/runbooks/trnm-settlement-shutdown-and-quarantine-v1.md",
    "docs/runbooks/trnm-authority-cutover-rollback-v1.md",
]
for path in required_paths:
    read(path)

current = read("CURRENT_PLAN.md")
for path in (PLAN_MD, PLAN_JSON, LEDGER_JSON):
    if path not in current:
        fail(f"CURRENT_PLAN.md does not point to {path}")

boundary = load("PROJECT_BOUNDARY.json")
if boundary.get("schema") != 3:
    fail("PROJECT_BOUNDARY.json schema must be 3")
if boundary.get("project_id") != "trillionnium-world" or boundary.get("lane") != "game-product":
    fail("project identity/lane drift")
if boundary.get("remote", {}).get("visibility") != "public":
    fail("repository visibility projection must be public")
if boundary.get("documentation", {}).get("current_plan") != PLAN_MD:
    fail("boundary current-plan pointer drift")
if boundary.get("ci", {}).get("validation_repository_permissions") != "read_only":
    fail("validation CI must be read-only")
if boundary.get("ci", {}).get("self_modifying_candidate_source") != "forbid":
    fail("self-modifying candidate source must be forbidden")
if boundary.get("release", {}).get("public_online") != "no_go":
    fail("public online overclaim")
if boundary.get("release", {}).get("public_player_market") != "disabled":
    fail("public player market overclaim")

plan = load(PLAN_JSON)
if plan.get("schema") != "trnm_world_development_plan_v4" or plan.get("plan_id") != PLAN_ID:
    fail("machine plan identity drift")
if plan.get("base_commit") != "1d4dee6d5add45a64f5c138f424e3bdab369ecd4":
    fail("machine plan base commit drift")
if plan.get("public_online") != "no_go" or plan.get("public_player_market") != "disabled":
    fail("machine plan release overclaim")
if set(plan.get("evidence_classes", [])) != EVIDENCE_CLASSES:
    fail("machine plan evidence classes drift")

ledger = load(LEDGER_JSON)
if ledger.get("schema") != "trnm_world_gap_closure_ledger_v4" or ledger.get("plan_id") != PLAN_ID:
    fail("gap ledger identity drift")
entries = ledger.get("entries")
if not isinstance(entries, list) or not entries:
    fail("gap ledger entries are missing")
ids: set[str] = set()
for entry in entries:
    if not isinstance(entry, dict):
        fail("gap ledger entry is not an object")
    item_id = entry.get("id")
    if not isinstance(item_id, str) or not re.fullmatch(r"WORLD-P[0-2]-[0-9]{3}[A-Z]?", item_id):
        fail(f"invalid gap id: {item_id!r}")
    if item_id in ids:
        fail(f"duplicate gap id: {item_id}")
    ids.add(item_id)
    if entry.get("evidence_class") not in EVIDENCE_CLASSES:
        fail(f"invalid evidence class for {item_id}")
    if not isinstance(entry.get("acceptance"), list) or not entry["acceptance"]:
        fail(f"missing acceptance criteria for {item_id}")
    if not isinstance(entry.get("depends_on"), list):
        fail(f"invalid dependency list for {item_id}")

expected_ids = {
    "WORLD-P0-001",
    "WORLD-P0-001A",
    "WORLD-P0-001B",
    "WORLD-P0-001C",
    "WORLD-P0-002",
    "WORLD-P0-002A",
    "WORLD-P0-003",
    "WORLD-P0-004",
    "WORLD-P0-005",
    "WORLD-P0-006",
}
if not expected_ids.issubset(ids):
    fail(f"ledger hides immediate P0 rows: {sorted(expected_ids - ids)}")

world_gates = load("docs/status/world-gates-v1.json")
by_id = {item.get("id"): item for item in world_gates.get("gates", []) if isinstance(item, dict)}
if by_id.get("public_online", {}).get("status") != "no_go":
    fail("world gate public_online must remain no_go")
if by_id.get("public_player_market", {}).get("status") != "disabled":
    fail("world gate public_player_market must remain disabled")

for path in ("README.md", "docs/README.md", "PROJECT_BOUNDARY.md", "AGENTS.md", PLAN_MD):
    text = read(path)
    if "TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md" in text and path != PLAN_MD:
        fail(f"stale 2026-08-27 plan pointer remains in current file {path}")

workflow_dir = ROOT / ".github/workflows"
if not workflow_dir.is_dir():
    fail("workflow directory is missing")
for workflow in sorted(workflow_dir.glob("*.yml")) + sorted(workflow_dir.glob("*.yaml")):
    text = workflow.read_text(encoding="utf-8")
    normalized = text.lower()
    if re.search(r"(?m)^\s*contents:\s*write\s*$", normalized):
        fail(f"write-enabled validation workflow: {workflow.relative_to(ROOT)}")
    for forbidden in ("git push", "git commit", "clippy --fix"):
        if forbidden in normalized:
            fail(f"self-modifying workflow marker {forbidden!r}: {workflow.relative_to(ROOT)}")

for removed in (
    ".github/workflows/apply-world-settlement-gap-closure-v1.yml",
    ".github/workflows/trnm-world-settlement-self-heal.yml",
    ".github/workflows/world-settlement-converge.yml",
):
    if (ROOT / removed).exists():
        fail(f"retired self-modifying workflow still exists: {removed}")

print("TRNM World documentation and CI integrity: PASS")
