#!/usr/bin/env python3
"""Static safety contract for the World repository-control applier.

This version validates f-string endpoint fragments from source text rather than
assuming they appear as complete ast.Constant values.
"""

from __future__ import annotations

import ast
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "admin" / "apply-world-repository-controls.py"
REQUIRED_CONTEXTS = {
    "trnm-world-v5/closure-contract",
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/transition-contract",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/game-workspace-release",
    "trnm-world-v4/supply-chain",
    "trnm-world-v4/qualified-source-exact-head",
    "trnm-world-v4/prospective-merge",
}
FORBIDDEN = (
    "/merges",
    "/statuses/",
    "/check-runs",
    "/git/blobs",
    "/git/commits",
    "/git/refs",
    "/git/tags",
    "/releases",
    "/deployments",
    "force=True",
    '"force": True',
)


def fail(message: str) -> None:
    print(f"repository control applier contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    source = SCRIPT.read_text(encoding="utf-8")
    try:
        ast.parse(source, filename=str(SCRIPT))
    except SyntaxError as error:
        fail(f"invalid Python: {error}")

    required_text = {
        "TrillionniumFoundation/Trillionnium-World",
        'BRANCH = "main"',
        "TRNM_WORLD_ADMIN_TOKEN",
        "/actions/permissions",
        "/actions/permissions/selected-actions",
        "/branches/{BRANCH}/protection",
    } | REQUIRED_CONTEXTS
    missing = sorted(item for item in required_text if item not in source)
    if missing:
        fail(f"missing immutable text: {missing}")

    for fragment in FORBIDDEN:
        if fragment in source:
            fail(f"forbidden capability present: {fragment}")

    required_phrases = (
        '"enabled": True, "allowed_actions": "selected"',
        '"github_owned_allowed": True',
        '"verified_allowed": False',
        '"patterns_allowed": []',
        '"required_approving_review_count": 1',
        '"dismiss_stale_reviews": True',
        '"require_code_owner_reviews": True',
        '"require_last_push_approval": True',
        '"enforce_admins": True',
        '"allow_force_pushes": False',
        '"allow_deletions": False',
        '"required_conversation_resolution": True',
        'branch.get("protected") is not True',
        'observed_contexts != set(CONTEXTS)',
        'verify_actions(token)',
        'verify_protection(token)',
    )
    for phrase in required_phrases:
        if phrase not in source:
            fail(f"missing fail-closed control: {phrase}")

    if source.count('api("PUT"') != 3:
        fail("only the two Actions settings and one protection PUT are permitted")
    if 'api("PATCH"' in source or 'api("POST"' in source or 'api("DELETE"' in source:
        fail("PATCH, POST and DELETE capabilities are forbidden")

    print("TRNM_WORLD_REPOSITORY_CONTROL_APPLIER_V2=PASS")


if __name__ == "__main__":
    main()
