#!/usr/bin/env python3
"""Reject mutable, self-verifying, unpinned, or documentation-drifted World CI."""

from __future__ import annotations

import pathlib
import re
import subprocess
import sys

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
REQUIRED_DOCS = {
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md",
    "docs/development/trillionnium-world-development-plan-2026-08-29.json",
    "docs/development/trnm-world-gap-closure-ledger-v4.json",
    "docs/development/trnm-world-module-decomposition-v1.md",
    "docs/development/trnm-world-testing-strategy-v2.md",
    "docs/protocol/trnm-world-transition-v1.md",
    "docs/protocol/schemas/trnm-world-transition-v1.schema.json",
    "docs/protocol/vectors/trnm-world-transition-v1.json",
    "docs/protocol/vectors/trnm-world-transition-negative-v1.json",
    "scripts/check-trnm-world-documentation.py",
    "scripts/check-trnm-world-transition-conformance.py",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World CI integrity: FAIL: {message}")


files = sorted([*WORKFLOWS.glob("*.yml"), *WORKFLOWS.glob("*.yaml")])
if not files:
    fail("no active workflows")
if len(files) != 1 or files[0].name != "trnm-world-gap-closure-v4.yml":
    fail(f"active workflow set is not the one v4 gate: {[path.name for path in files]}")

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

for relative in sorted(REQUIRED_DOCS):
    path = ROOT / relative
    if not path.is_file() or not path.read_text(encoding="utf-8").strip():
        fail(f"required current documentation is missing or empty: {relative}")

documentation = subprocess.run(
    [sys.executable, str(ROOT / "scripts/check-trnm-world-documentation.py"), str(ROOT)],
    check=False,
    capture_output=True,
    text=True,
)
if documentation.returncode != 0:
    fail(
        "documentation consistency failed: "
        + (documentation.stderr.strip() or documentation.stdout.strip())
    )

print(
    "TRNM World CI integrity: PASS "
    f"({len(files)} workflow file, {len(required_contexts)} required contexts, "
    f"{len(REQUIRED_DOCS)} current docs)"
)
