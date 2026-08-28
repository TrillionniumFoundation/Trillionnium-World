#!/usr/bin/env python3
"""Fail-closed source/server projection for WORLD-P0-005."""

from __future__ import annotations

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
STATUS = ROOT / "docs/status/world-main-governance-v1.json"
CODEOWNERS = ROOT / ".github/CODEOWNERS"
WORKFLOW = ROOT / ".github/workflows/trnm-world-governance.yml"
TOKEN = re.compile(r"^[A-Za-z0-9](?:[A-Za-z0-9-]{0,38})$")

REQUIRED_GATES = {
    "create_or_verify_main_ruleset",
    "require_current_exact_named_checks",
    "require_code_owner_review",
    "enforce_rules_for_administrators",
    "disable_force_push_and_branch_deletion",
    "obtain_server_side_governance_snapshot",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World main governance: FAIL: {message}")


value = json.loads(STATUS.read_text(encoding="utf-8"))
if value.get("schema") != "trnm_world_main_governance_status_v1":
    fail("wrong schema")
if value.get("repository") != "TrillionniumFoundation/Trillionnium-World":
    fail("wrong repository")
if value.get("default_branch") != "main":
    fail("wrong default branch")
if value.get("status") != "source_ready_server_controls_unverified":
    fail("source cannot self-assert server governance")
if value.get("release_effect") != "none" or value.get("public_online") is not False:
    fail("governance source cannot grant release or public-online credit")
if set(value.get("open_gates", [])) != REQUIRED_GATES:
    fail("server governance blockers were hidden")
server = value.get("server_controls", {})
if server != {
    "ruleset_observed": False,
    "branch_protection_observed": False,
    "required_checks_observed": [],
    "enforce_admins_observed": False,
    "code_owner_review_observed": False,
}:
    fail("unverified source invented server-side controls")
owners = value.get("source_controls", {}).get("codeowner_principals", [])
if not owners or any(TOKEN.fullmatch(owner) is None for owner in owners):
    fail("invalid CODEOWNERS principals")
codeowners = CODEOWNERS.read_text(encoding="utf-8")
for owner in owners:
    if f"@{owner}" not in codeowners:
        fail(f"CODEOWNERS missing @{owner}")
workflow = WORKFLOW.read_text(encoding="utf-8")
for marker in (
    "trnm-world-governance/source-contract",
    "check-world-main-governance.py",
    "world-main-governance-v1.json",
):
    if marker not in workflow:
        fail(f"governance workflow missing {marker}")
print("TRNM World main governance source contract: PASS")
