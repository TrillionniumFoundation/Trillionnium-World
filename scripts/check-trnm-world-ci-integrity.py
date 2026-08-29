#!/usr/bin/env python3
"""Reject mutable, self-verifying, or unpinned World workflows."""

from __future__ import annotations

import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKFLOWS = ROOT / ".github/workflows"
PINNED_USE = re.compile(r"^\s*-?\s*uses:\s*[^@\s]+@([0-9a-f]{40})\s*(?:#.*)?$")
FORBIDDEN = {
    "contents: write": "workflows must not write repository contents",
    "persist-credentials: true": "checkout credentials must not persist",
    "pull_request_target:": "untrusted PR code must not run with target privileges",
    "clippy --fix": "CI must not rewrite Rust source",
    "git push": "CI must not push source or tags",
    "git commit": "CI must not create source commits",
    "git tag": "CI must not mint verified tags",
    "gh pr merge": "CI must not merge pull requests",
    "update-ref": "CI must not move Git refs",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World CI integrity: FAIL: {message}")


files = sorted([*WORKFLOWS.glob("*.yml"), *WORKFLOWS.glob("*.yaml")])
if not files:
    fail("no active workflows")

contexts: set[str] = set()
for path in files:
    text = path.read_text(encoding="utf-8")
    relative = path.relative_to(ROOT)
    if "permissions:" not in text or "contents: read" not in text:
        fail(f"{relative} does not declare read-only contents permission")
    if "runs-on: ubuntu-latest" in text:
        fail(f"{relative} uses mutable ubuntu-latest")
    for needle, reason in FORBIDDEN.items():
        if needle in text:
            fail(f"{relative}: {reason} ({needle})")
    for line_number, line in enumerate(text.splitlines(), start=1):
        if "uses:" in line:
            match = PINNED_USE.match(line)
            if match is None:
                fail(f"{relative}:{line_number} action is not pinned to 40 hex characters")
        stripped = line.strip()
        if stripped.startswith("name:") and "/" in stripped:
            contexts.add(stripped.removeprefix("name:").strip())

required_contexts = {
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/transition-contract",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/game-workspace-release",
    "trnm-world-v4/supply-chain",
}
missing = required_contexts - contexts
if missing:
    fail(f"required exact job contexts are missing: {sorted(missing)}")

print(
    "TRNM World CI integrity: PASS "
    f"({len(files)} workflow file(s), {len(required_contexts)} required contexts)"
)
