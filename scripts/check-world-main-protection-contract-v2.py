#!/usr/bin/env python3
"""Validate the declared World main-protection contract without claiming enforcement."""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
PATH = ROOT / "docs" / "governance" / "main-protection-contract-v2.json"
EXPECTED_CONTEXTS = {
    "trnm-world-v5/closure-contract",
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/transition-contract",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/game-workspace-release",
    "trnm-world-v4/supply-chain",
    "trnm-world-v4/qualified-source-exact-head",
    "trnm-world-v4/prospective-merge",
}


def fail(message: str) -> None:
    print(f"main protection contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    data = json.loads(PATH.read_text(encoding="utf-8"))
    if data.get("schema") != "trnm_world_main_protection_contract_v2":
        fail("schema drift")
    if data.get("repository") != "TrillionniumFoundation/Trillionnium-World" or data.get("branch") != "main":
        fail("repository or branch drift")
    if data.get("server_state") != "not_enforced_until_api_readback":
        fail("document must not claim server enforcement")
    reviews = data.get("required_pull_request_reviews", {})
    required_reviews = {
        "required_approving_review_count": 1,
        "dismiss_stale_reviews": True,
        "require_code_owner_reviews": True,
        "require_last_push_approval": True,
        "last_pusher_may_approve": False,
    }
    for key, value in required_reviews.items():
        if reviews.get(key) != value:
            fail(f"review control drift: {key}")
    checks = data.get("required_status_checks", {})
    if checks.get("strict") is not True or checks.get("non_empty_execution_required") is not True:
        fail("status-check strictness weakened")
    if set(checks.get("contexts", [])) != EXPECTED_CONTEXTS:
        fail("required context set drift")
    if checks.get("accepted_conclusions") != ["success"]:
        fail("only success may receive credit")
    for key, value in {
        "require_conversation_resolution": True,
        "require_linear_history": True,
        "allow_force_pushes": False,
        "allow_deletions": False,
        "block_direct_pushes": True,
        "admin_enforcement": True,
    }.items():
        if data.get(key) != value:
            fail(f"branch control drift: {key}")
    bypass = data.get("bypass", {})
    if bypass.get("ordinary_bypass_allowed") is not False or len(bypass.get("break_glass_requires", [])) < 6:
        fail("bypass boundary weakened")
    if data.get("production_authorization") != "not_granted":
        fail("governance document cannot grant production authorization")
    print("TRNM_WORLD_MAIN_PROTECTION_CONTRACT_V2=PASS")


if __name__ == "__main__":
    main()
