#!/usr/bin/env python3
"""Reject mutable, self-verifying, unpinned, or documentation-drifted World CI."""

from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
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
    "docs/status/v4-candidate-v1.json",
    "docs/status/v4-candidate-v1.schema.json",
    "scripts/check-trnm-world-documentation.py",
    "scripts/check-trnm-world-transition-conformance.py",
    "scripts/check-trnm-world-v4-candidate.py",
    "scripts/test-trnm-world-v4-candidate-negative.py",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World CI integrity: FAIL: {message}")


# Explicit reviewed inventory. Adding a workflow or moving a required context
# requires updating this mapping and negative tests, not weakening cardinality.
WORKFLOW_JOBS = {
    "trnm-world-gap-closure-v4.yml": {
        name: "trnm-world-v4/" + name for name in (
            "docs-governance", "transition-contract", "settlement-postgres",
            "game-workspace-release", "supply-chain")
    },
    "trnm-world-v4-final-gates.yml": {
        **{name: "trnm-world-v4-supplemental/" + name for name in (
            "docs-governance", "transition-contract", "settlement-postgres",
            "game-workspace-release", "supply-chain")},
        "qualified-source-exact-head": "trnm-world-v4/qualified-source-exact-head",
        "closure-contract": "trnm-world-v5-supplemental/closure-contract",
        "prospective-merge": "trnm-world-v4/prospective-merge",
    },
    "trnm-world-cex-sequence-50-qualification.yml": {
        "static-lock-contract": "trnm-world-v5/cex-lock-contract",
        "live-upstream-qualification": "trnm-world-v5/cex-sequence-50-live",
    },
    "trnm-world-module-documentation.yml": {
        "module-documentation": "trnm-world/module-documentation",
    },
    "trnm-world-repository-contract-v2.yml": {
        "repository-contract": "trnm-world-v5/repository-contract",
    },
    "trnm-world-repository-contract-v3.yml": {
        "repository-contract": "trnm-world-v5/repository-contract-v3",
    },
    "trnm-world-rts-intake-contract.yml": {
        "intake-contract": "trnm-world-v4/rts-intake-contract",
    },
    "trnm-world-v5-closure-contract.yml": {
        "closure-contract": "trnm-world-v5/closure-contract",
    },
}


def workflow_inventory(root: pathlib.Path) -> dict[str, str]:
    """Check the bounded current layout, not general YAML/shell semantics.

    Static indented job identifiers and names are required. The workflow
    service remains the authority for parsing, scheduling and actual execution.
    """
    folder = root / ".github/workflows"
    if folder.is_symlink() or not folder.is_dir():
        fail("workflow directory is missing or linked")
    files = sorted([*folder.glob("*.yml"), *folder.glob("*.yaml")])
    if {path.name for path in files} != set(WORKFLOW_JOBS):
        fail("active workflow inventory differs from the reviewed eight files")
    contexts: dict[str, str] = {}
    for path in files:
        if path.is_symlink() or not path.is_file():
            fail("workflow is missing or linked")
        with path.open("rb") as handle:
            data = handle.read(256 * 1024 + 1)
        if not data.strip() or len(data) > 256 * 1024:
            fail("workflow is empty or exceeds the 256 KiB budget")
        text = data.decode("utf-8")
        if not re.search(r"(?m)^permissions:\n  contents: read\s*$", text):
            fail(f"{path.name} must declare global read-only contents permission")
        if len(re.findall(r"(?m)^\s*permissions:", text)) != 1:
            fail(f"{path.name} has a missing/ambiguous permissions block")
        if re.search(r"(?m)^\s*[a-z_-]+:\s*(?:write|write-all)\s*(?:#.*)?$", text):
            fail(f"{path.name} declares write permission")
        if "runs-on: ubuntu-latest" in text:
            fail(f"{path.name} uses mutable ubuntu-latest")
        for needle, reason in FORBIDDEN.items():
            if needle in text:
                fail(f"{path.name}: {reason} ({needle})")
        for line_number, line in enumerate(text.splitlines(), 1):
            if "uses:" in line and PINNED_USE.match(line) is None:
                fail(f"{path.name}:{line_number} action is not pinned to 40 hex characters")
        if text.count("\njobs:\n") != 1:
            fail(f"{path.name} must have one ordinary jobs mapping")
        parts = re.split(r"(?m)^  ([A-Za-z_][A-Za-z_0-9-]*):\n", text.split("\njobs:\n", 1)[1])
        identifiers = parts[1::2]
        if len(identifiers) != len(set(identifiers)):
            fail(f"{path.name} has duplicate job identifiers")
        actual = {}
        for name, body in zip(identifiers, parts[2::2]):
            names = re.findall(r"(?m)^    name: ([a-z0-9][a-z0-9/-]*)\s*$", body)
            if len(names) != 1:
                fail(f"{path.name}/{name} must have one static job name")
            context = names[0]
            if context in contexts:
                fail(f"duplicate check context {context}: {contexts[context]} and {path.name}")
            contexts[context] = path.name
            actual[name] = context
        if actual != WORKFLOW_JOBS[path.name]:
            fail(f"{path.name} job/context ownership differs from reviewed mapping")
    return contexts


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=pathlib.Path, default=ROOT)
    parser.add_argument("--workflows-only", action="store_true",
                        help="static inventory only; no document, candidate or execution credit")
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        contexts = workflow_inventory(root)
        if not args.workflows_only:
            for relative in sorted(REQUIRED_DOCS):
                path = root / relative
                if path.is_symlink() or not path.is_file() or not path.read_text(encoding="utf-8").strip():
                    fail(f"required current/historical contract is missing or empty: {relative}")
            # The selected current snapshot is validated by documentation.py.
            # The older v4-candidate record is still checked as a historical
            # schema/input only; it does not select the current PR or its head.
            for relative, arguments in (
                ("scripts/check-trnm-world-documentation.py", [str(root)]),
                ("scripts/check-trnm-world-v4-candidate.py", []),
                ("scripts/test-trnm-world-v4-candidate-negative.py", []),
            ):
                result = subprocess.run([sys.executable, str(root / relative), *arguments],
                                        check=False, capture_output=True, text=True, cwd=root,
                                        timeout=180)
                if result.returncode != 0:
                    fail(relative + " failed: " + (result.stderr.strip() or result.stdout.strip()))
    except (OSError, UnicodeError, ValueError, subprocess.TimeoutExpired) as error:
        fail(str(error))
    scope = "workflow inventory only" if args.workflows_only else "workflow inventory, selected documentation and historical schema"
    print(f"TRNM World CI integrity: PASS ({len(WORKFLOW_JOBS)} workflows, {len(contexts)} unique contexts; {scope}; no hosted/governance evidence)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
