#!/usr/bin/env python3
"""Fail-closed source/server projection for WORLD-P0-005."""

from __future__ import annotations

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
STATUS = ROOT / "docs/status/world-main-governance-v1.json"
CODEOWNERS = ROOT / ".github/CODEOWNERS"
WORKFLOW = ROOT / ".github/workflows/trnm-world-gap-closure-v4.yml"
TOKEN = re.compile(r"^[A-Za-z0-9](?:[A-Za-z0-9-]{0,38})$")

REQUIRED_GATES = {
    "run_candidate_checks_at_least_once",
    "create_or_verify_main_ruleset",
    "require_current_exact_named_checks",
    "require_code_owner_review",
    "enforce_rules_for_administrators",
    "disable_force_push_and_branch_deletion",
    "obtain_server_side_governance_snapshot",
}
REQUIRED_CONTEXTS = {
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/transition-contract",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/game-workspace-release",
    "trnm-world-v4/supply-chain",
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
source = value.get("source_controls", {})
if source.get("workflow") != ".github/workflows/trnm-world-gap-closure-v4.yml":
    fail("wrong current governance workflow")
if source.get("workflow_contents_permission") != "read":
    fail("governance workflow is not read-only")
if source.get("self_mutating_workflows_present") is not False:
    fail("source claims self-mutating validation is still present")
if set(source.get("required_check_contexts_candidate", [])) != REQUIRED_CONTEXTS:
    fail("candidate required-check registry drifted")
owners = source.get("codeowner_principals", [])
if not owners or any(TOKEN.fullmatch(owner) is None for owner in owners):
    fail("invalid CODEOWNERS principals")
codeowners = CODEOWNERS.read_text(encoding="utf-8")
for owner in owners:
    if f"@{owner}" not in codeowners:
        fail(f"CODEOWNERS missing @{owner}")
workflow = WORKFLOW.read_text(encoding="utf-8")
for marker in REQUIRED_CONTEXTS | {
    "permissions:",
    "contents: read",
    "check-world-main-governance.py",
    "world-main-governance-v1.json",
}:
    if marker not in workflow:
        fail(f"governance workflow missing {marker}")
for forbidden in (
    "contents: write",
    "persist-credentials: true",
    "git push",
    "git commit",
    "git tag",
    "clippy --fix",
):
    if forbidden in workflow:
        fail(f"governance workflow contains forbidden mutation: {forbidden}")
print("TRNM World main governance source contract: PASS")
