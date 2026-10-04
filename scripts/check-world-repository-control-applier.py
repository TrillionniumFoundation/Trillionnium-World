#!/usr/bin/env python3
"""Static safety contract for the World repository-control applier."""

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
        tree = ast.parse(source, filename=str(SCRIPT))
    except SyntaxError as error:
        fail(f"invalid Python: {error}")
    literals = {
        node.value
        for node in ast.walk(tree)
        if isinstance(node, ast.Constant) and isinstance(node.value, str)
    }
    for literal in {
        "TrillionniumFoundation/Trillionnium-World",
        "main",
        "TRNM_WORLD_ADMIN_TOKEN",
        "/actions/permissions",
        "/actions/permissions/selected-actions",
        "/branches/{BRANCH}/protection",
    }:
        if literal not in literals:
            fail(f"missing immutable literal: {literal}")
    missing_contexts = sorted(REQUIRED_CONTEXTS - literals)
    if missing_contexts:
        fail(f"missing required contexts: {missing_contexts}")
    for fragment in FORBIDDEN:
        if fragment in source:
            fail(f"forbidden capability present: {fragment}")
    required_phrases = (
        '"enabled": True, "allowed_actions": "selected"',
        '"github_owned_allowed": True',
        '"verified_allowed": False',
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
    )
    for phrase in required_phrases:
        if phrase not in source:
            fail(f"missing fail-closed control: {phrase}")
    print("TRNM_WORLD_REPOSITORY_CONTROL_APPLIER=PASS")


if __name__ == "__main__":
    main()
