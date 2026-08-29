#!/usr/bin/env python3
"""Fail-closed validator for the current World V4 plan and gap registry."""

from __future__ import annotations

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
PLAN = ROOT / "docs/development/trillionnium-world-development-plan-v4-2026-08-29.json"
GAPS = ROOT / "docs/status/world-gap-registry-v2.json"
BOUNDARY = ROOT / "PROJECT_BOUNDARY.json"
HEX40 = re.compile(r"^[0-9a-f]{40}$")


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World plan V4: FAIL: {message}")


def load(path: pathlib.Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain an object")
    return value


plan = load(PLAN)
gaps = load(GAPS)
boundary = load(BOUNDARY)

if plan.get("schema") != 1 or plan.get("status") != "current":
    fail("plan schema/status is not current")
if plan.get("plan_id") != "trillionnium-world-development-v4-2026-08-29":
    fail("unexpected plan id")
if not HEX40.fullmatch(str(plan.get("source_base_commit", ""))):
    fail("plan source base commit is not exact")
if plan.get("verified_commit") is not None:
    fail("source may not invent a verified commit before exact-head evidence")

decision = plan.get("decision", {})
expected_decision = {
    "world_role": "deterministic-game-domain",
    "online_match_authority": "Trillionnium-Nakama",
    "chain_finality_authority": "Trillionnium-Chain",
    "wallet_settlement_authority": "CEX",
    "cross_repository_evidence_authority": "Trillionnium-Integration",
    "world_local_game_server_status": "compatibility-authority-enclave",
    "public_online": "no_go",
    "public_player_market": "disabled",
}
if decision != expected_decision:
    fail("authority/release decision drifted")

if boundary.get("remote", {}).get("visibility") != "public":
    fail("repository visibility metadata does not match GitHub")
if boundary.get("documentation", {}).get("current_plan") != (
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md"
):
    fail("project boundary does not point at V4")
if boundary.get("authority", {}).get("online_match_authority") != "Trillionnium-Nakama":
    fail("project boundary reintroduced World canonical online authority")

work_items = plan.get("work_items")
if not isinstance(work_items, list) or not work_items:
    fail("plan has no work items")
plan_ids = [item.get("id") for item in work_items if isinstance(item, dict)]
if len(plan_ids) != len(set(plan_ids)) or any(not isinstance(item, str) for item in plan_ids):
    fail("plan work item ids are missing or duplicated")

if gaps.get("schema") != "trnm_world_gap_registry_v2":
    fail("wrong gap registry schema")
if gaps.get("repository") != "TrillionniumFoundation/Trillionnium-World":
    fail("wrong gap registry repository")
if gaps.get("source_base_commit") != plan.get("source_base_commit"):
    fail("plan and gap registry base commits disagree")
posture = gaps.get("release_posture", {})
if posture.get("public_online") != "no_go" or posture.get("public_player_market") != "disabled":
    fail("gap registry overclaims public release")

gap_rows = gaps.get("gaps")
if not isinstance(gap_rows, list) or not gap_rows:
    fail("gap registry has no rows")
gap_ids = [row.get("id") for row in gap_rows if isinstance(row, dict)]
if set(gap_ids) != set(plan_ids):
    fail("plan and gap registry work item sets disagree")
if len(gap_ids) != len(set(gap_ids)):
    fail("gap registry contains duplicate ids")

classes = [row.get("class") for row in gap_rows]
computed = {
    "closed": classes.count("closed"),
    "implemented_unverified": classes.count("implemented_unverified"),
    "source_gap": classes.count("source_gap"),
    "planned_or_partial": classes.count("planned_or_partial"),
    "blocked_upstream": classes.count("blocked_upstream"),
    "server_configuration_required": classes.count("server_configuration_required"),
    "external_evidence_required_or_blocked": classes.count(
        "external_evidence_required_or_blocked"
    ),
}
if gaps.get("counts") != computed:
    fail(f"gap counts drifted: expected {computed}, found {gaps.get('counts')}")

required_files = [
    "README.md",
    "CURRENT_PLAN.md",
    "PROJECT_BOUNDARY.md",
    "docs/README.md",
    "docs/status/CURRENT.md",
    "docs/architecture/trnm-world-system-architecture-v1.md",
    "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md",
    "docs/adr/0002-transaction-free-external-settlement.md",
    "docs/adr/0003-reviewable-source-and-non-self-modifying-ci.md",
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md",
    "docs/development/trnm-world-module-decomposition-v1.md",
    "docs/development/trnm-world-protocol-and-database-contract-plan-v1.md",
    "docs/security/trnm-world-threat-model-v1.md",
    "docs/release/trnm-world-release-evidence-contract-v1.md",
    "docs/runbooks/trnm-world-gap-closure-operations-v1.md",
]
for relative in required_files:
    if not (ROOT / relative).is_file():
        fail(f"missing current truth file {relative}")

for relative in (
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md",
    "docs/development/trnm-world-development-plan-v3.md",
    "docs/adr/0002-settlement-external-io-boundary.md",
):
    text = (ROOT / relative).read_text(encoding="utf-8").lower()
    if "superseded" not in text:
        fail(f"historical truth source is not marked superseded: {relative}")

readme = (ROOT / "README.md").read_text(encoding="utf-8")
for forbidden in (
    "TRNM is a Rust L1 protocol",
    "TrillionniumChain/",
    "Current native snapshot (2026-07-23)",
):
    if forbidden in readme:
        fail(f"root README retains stale product claim: {forbidden}")

print("TRNM World plan V4 and gap registry: PASS")
