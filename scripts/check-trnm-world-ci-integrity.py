#!/usr/bin/env python3
"""Reject CI workflow drift, mutation capability, context collisions, and weak admission wiring."""
from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

WORKFLOW_JOBS = {
    "trnm-world-cex-sequence-50-qualification.yml": {
        "static-lock-contract": "trnm-world-v5/cex-lock-contract",
        "live-upstream-qualification": "trnm-world-v5/cex-sequence-50-live",
    },
    "trnm-world-gap-closure-v4.yml": {
        "docs-governance": "trnm-world-v4/docs-governance",
        "transition-contract": "trnm-world-v4/transition-contract",
        "settlement-postgres": "trnm-world-v4/settlement-postgres",
        "game-workspace-release": "trnm-world-v4/game-workspace-release",
        "supply-chain": "trnm-world-v4/supply-chain",
    },
    "trnm-world-module-documentation.yml": {
        "complete-documentation": "trnm-world/docs-complete",
    },
    "trnm-world-postgres-cutover.yml": {
        "postgres-cutover": "trnm-world-postgres-cutover",
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
    "trnm-world-v4-final-gates.yml": {
        "docs-governance": "trnm-world-v4-supplemental/docs-governance",
        "transition-contract": "trnm-world-v4-supplemental/transition-contract",
        "settlement-postgres": "trnm-world-v4-supplemental/settlement-postgres",
        "game-workspace-release": "trnm-world-v4-supplemental/game-workspace-release",
        "supply-chain": "trnm-world-v4-supplemental/supply-chain",
        "qualified-source-exact-head": "trnm-world-v4/qualified-source-exact-head",
        "closure-contract": "trnm-world-v5-supplemental/closure-contract",
        "prospective-merge": "trnm-world-v4/prospective-merge",
    },
    "trnm-world-v5-closure-contract.yml": {
        "closure-contract": "trnm-world-v5/closure-contract",
    },
    "trnm-world-v6-admission-v2.yml": {
        "truth": "trnm-world-v6/truth-head-and-merge",
        "source": "trnm-world-v6/source-head-and-merge",
        "postgres": "trnm-world-v6/postgres-head-and-merge",
        "package": "trnm-world-v6/package-head-and-merge",
        "required": "trnm-world-v6/required-v2",
    },
    "trnm-world-v6-admission.yml": {
        "truth-head": "trnm-world-v6/docs-governance-exact-head",
        "truth-merge": "trnm-world-v6/docs-governance-prospective-merge",
        "source-head": "trnm-world-v6/source-exact-head",
        "source-merge": "trnm-world-v6/source-prospective-merge",
        "postgres-head": "trnm-world-v6/postgres-exact-head",
        "postgres-merge": "trnm-world-v6/postgres-prospective-merge",
        "package-head": "trnm-world-v6/supply-chain-package-exact-head",
        "package-merge": "trnm-world-v6/supply-chain-package-prospective-merge",
        "required": "trnm-world-v6/required",
    },
    "trnm-world-v6-external-evidence-contract.yml": {
        "external-evidence-contract": "trnm-world-v6/external-evidence-contract",
    },
}

MATRIX_WORKFLOWS = {
    "world-pr-native-admission-v1.yml": {
        "job_id": "qualification",
        "name_expression": "${{ matrix.context }}",
        "pairs": {
            ("source", "trnm-game-ci"),
            ("postgres", "trnm-world-p0-boundaries"),
            ("package", "trnm-world-status-evidence"),
        },
        "required_job_id": "required",
        "required_context": "trnm-world-v12/required",
    },
}

REQUIRED_DOCS = {
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md",
    "docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md",
    "docs/development/TRILLIONNIUM_WORLD_PLAN_V4_EXECUTION_UPDATE_2026-08-30.md",
    "docs/development/trillionnium-world-development-plan-2026-08-29.json",
    "docs/development/trnm-world-gap-closure-ledger-v4.json",
    "docs/development/trnm-world-gap-closure-ledger-v5.json",
    "docs/development/trnm-world-gap-closure-ledger-v6.json",
    "docs/status/world-v4-convergence-state-2026-08-30.json",
    "docs/status/world-v4-convergence-state-2026-08-30.schema.json",
    "docs/integration/trnm-world-cex-current-pending-lock-v2.json",
}
WRITE_PERMISSION_RE = re.compile(
    r"(?im)^[ \t]*(?:actions|checks|contents|deployments|discussions|id-token|issues|packages|pages|pull-requests|repository-projects|security-events|statuses):[ \t]*write[ \t]*(?:#.*)?$"
)
PERSIST_CREDENTIALS_RE = re.compile(r"(?im)^[ \t]*persist-credentials:[ \t]*(?:true|yes|on|1)[ \t]*(?:#.*)?$")
MUTATING_GIT_RE = re.compile(
    r"\bgit\s+(?:commit|push|tag|merge|rebase|cherry-pick|am|apply|reset|checkout\s+-B|switch\s+-C|update-ref)\b",
    re.I,
)
MUTATING_GH_RE = re.compile(r"\bgh\s+(?:pr\s+merge|release\s+create|api\s+.*-(?:X|f)\s*(?:POST|PATCH|PUT|DELETE))\b", re.I)
FIX_RE = re.compile(r"(?<![A-Za-z0-9_-])(?:cargo\s+(?:clippy|fmt)|ruff|eslint|prettier)\b[^\n]*(?:--fix|--write)(?![A-Za-z0-9_-])", re.I)
ACTION_RE = re.compile(r"(?m)^[ \t]*uses:[ \t]*([^\s#]+)")
RUNNER_RE = re.compile(r"(?im)^[ \t]*runs-on:[ \t]*([^\s#]+)")
STATIC_JOB_NAME_RE = re.compile(r"(?m)^[ ]{4}name:[ \t]*([^\s#][^#]*?)[ \t]*$")
JOB_ID_RE = re.compile(r"^[A-Za-z0-9_-]+$")
DEFAULT_RUNNER = "ubuntu-24.04"
MAX_WORKFLOW_BYTES = 256 * 1024


def die(message: str) -> None:
    raise SystemExit(f"TRNM World CI integrity: FAIL: {message}")


def read_workflow(path: pathlib.Path) -> str:
    try:
        if path.is_symlink() or not path.is_file():
            die(f"workflow is not a regular unlinked file: {path}")
        data = path.read_bytes()
        if not data or not data.strip():
            die(f"workflow is empty: {path}")
        if len(data) > MAX_WORKFLOW_BYTES:
            die(f"workflow exceeds {MAX_WORKFLOW_BYTES} bytes: {path}")
        return data.decode("utf-8", errors="strict")
    except (OSError, UnicodeError) as exc:
        die(f"cannot inspect workflow {path}: {exc}")


def jobs_section(source: str, workflow: pathlib.Path) -> str:
    matches = list(re.finditer(r"(?m)^jobs:\s*(?:#.*)?$", source))
    if len(matches) != 1:
        die(f"{workflow}: expected exactly one top-level jobs mapping")
    return source[matches[0].end():]


def job_blocks(source: str, workflow: pathlib.Path) -> dict[str, str]:
    section = jobs_section(source, workflow)
    matches = list(re.finditer(r"(?m)^  ([A-Za-z0-9_-]+):\s*(?:#.*)?$", section))
    if not matches:
        die(f"{workflow}: no top-level jobs found")
    result: dict[str, str] = {}
    for index, match in enumerate(matches):
        job_id = match.group(1)
        if not JOB_ID_RE.fullmatch(job_id) or job_id in result:
            die(f"{workflow}: invalid or duplicate top-level job id {job_id!r}")
        end = matches[index + 1].start() if index + 1 < len(matches) else len(section)
        result[job_id] = section[match.end():end]
    return result


def static_job_names(source: str, workflow: pathlib.Path) -> dict[str, str]:
    result: dict[str, str] = {}
    for job_id, body in job_blocks(source, workflow).items():
        matches = STATIC_JOB_NAME_RE.findall(body)
        if len(matches) != 1:
            die(f"{workflow}: top-level job {job_id!r} must have exactly one static name")
        value = matches[0].strip().strip("'\"")
        if not value or "${{" in value or "}}" in value:
            die(f"{workflow}: top-level job {job_id!r} has a dynamic or empty name")
        result[job_id] = value
    return result


def matrix_contexts(source: str, workflow: pathlib.Path, contract: dict[str, object]) -> dict[str, str]:
    blocks = job_blocks(source, workflow)
    expected_ids = {str(contract["job_id"]), str(contract["required_job_id"])}
    if set(blocks) != expected_ids:
        die(f"{workflow}: matrix admission job ids drift: expected={sorted(expected_ids)} actual={sorted(blocks)}")

    qualification = blocks[str(contract["job_id"])]
    name_matches = re.findall(r"(?m)^[ ]{4}name:[ \t]*([^#]+?)[ \t]*$", qualification)
    if len(name_matches) != 1 or name_matches[0].strip().strip("'\"") != contract["name_expression"]:
        die(f"{workflow}: matrix qualification name expression drift")

    pairs = set(
        re.findall(
            r"(?m)^[ ]{10}- lane:[ \t]*([A-Za-z0-9_-]+)[ \t]*$\n^[ ]{12}context:[ \t]*([A-Za-z0-9_./-]+)[ \t]*$",
            qualification,
        )
    )
    expected_pairs = set(contract["pairs"])
    if pairs != expected_pairs or len(pairs) != len(expected_pairs):
        die(f"{workflow}: matrix lane/context set drift: expected={sorted(expected_pairs)} actual={sorted(pairs)}")

    required_body = blocks[str(contract["required_job_id"])]
    required_names = STATIC_JOB_NAME_RE.findall(required_body)
    if len(required_names) != 1:
        die(f"{workflow}: required aggregate must have exactly one static name")
    required_context = required_names[0].strip().strip("'\"")
    if required_context != contract["required_context"]:
        die(f"{workflow}: required aggregate context drift")
    if not re.search(r"(?m)^[ ]{4}needs:[ \t]*\[qualification\][ \t]*$", required_body):
        die(f"{workflow}: required aggregate must depend on the complete matrix job")

    result = {context: workflow.name for _, context in expected_pairs}
    result[required_context] = workflow.name
    return result


def workflow_inventory(root: pathlib.Path) -> dict[str, str]:
    folder = root / ".github/workflows"
    try:
        if folder.is_symlink() or not folder.is_dir():
            die(f"workflow directory is not a regular directory: {folder}")
        actual_paths = sorted(folder.iterdir())
    except OSError as exc:
        die(f"cannot inspect workflow directory: {exc}")
    if any(path.is_symlink() or not path.is_file() for path in actual_paths):
        die("workflow directory contains a linked or non-file entry")
    actual = {path.name for path in actual_paths}
    expected = set(WORKFLOW_JOBS) | set(MATRIX_WORKFLOWS)
    if actual != expected:
        die(f"workflow inventory drift: expected={sorted(expected)} actual={sorted(actual)}")

    contexts: dict[str, str] = {}
    for name in sorted(actual):
        path = folder / name
        source = read_workflow(path)
        lowered = source.lower()
        if "permissions:\n  contents: read" not in source:
            die(f"{name}: missing top-level contents: read")
        if WRITE_PERMISSION_RE.search(source) or re.search(r"(?m)^[ ]{4}permissions:[ \t]*", source):
            die(f"{name}: validation workflow requests write or job-level permissions")
        if "pull_request_target:" in lowered or "workflow_run:" in lowered:
            die(f"{name}: privileged trigger forbidden")
        if PERSIST_CREDENTIALS_RE.search(source):
            die(f"{name}: retained checkout credentials forbidden")
        sanitized = source.replace("git commit-tree", "git_commit_tree")
        if MUTATING_GIT_RE.search(sanitized) or MUTATING_GH_RE.search(source) or FIX_RE.search(source):
            die(f"{name}: source-mutating validation command")
        for action in ACTION_RE.findall(source):
            if not re.fullmatch(r"[^@\s]+@[0-9a-f]{40,64}", action):
                die(f"{name}: action is not immutable: {action}")
        for runner in RUNNER_RE.findall(source):
            runner = runner.strip().strip("'\"")
            if runner != DEFAULT_RUNNER:
                die(f"{name}: mutable or unapproved runner: {runner}")

        if name in MATRIX_WORKFLOWS:
            actual_contexts = matrix_contexts(source, path, MATRIX_WORKFLOWS[name])
        else:
            actual_jobs = static_job_names(source, path)
            expected_jobs = WORKFLOW_JOBS[name]
            if actual_jobs != expected_jobs:
                die(f"{name}: job/context drift: expected={expected_jobs} actual={actual_jobs}")
            actual_contexts = {context: name for context in actual_jobs.values()}

        for context, owner in actual_contexts.items():
            if context in contexts:
                die(f"duplicate check context {context}: {contexts[context]} and {owner}")
            contexts[context] = owner
    return contexts


def run_required_documentation(root: pathlib.Path) -> None:
    missing = [item for item in REQUIRED_DOCS if not (root / item).is_file()]
    if missing:
        die(f"required current/historical contract is missing: {missing}")
    commands = [
        [sys.executable, str(root / "scripts/check-trnm-world-documentation.py")],
        [sys.executable, str(root / "scripts/check-trnm-world-component-catalog.py")],
        [sys.executable, str(root / "scripts/test-trnm-world-component-catalog.py")],
        [sys.executable, str(root / "scripts/check-trnm-world-detailed-documentation.py")],
        [sys.executable, str(root / "scripts/test-trnm-world-detailed-documentation.py")],
        [sys.executable, str(root / "scripts/check-trnm-world-authority-documentation.py")],
        [sys.executable, str(root / "scripts/test-trnm-world-authority-documentation.py")],
        [sys.executable, str(root / "scripts/check-trnm-world-contract-module-documentation.py")],
        [sys.executable, str(root / "scripts/test-trnm-world-contract-module-documentation.py")],
        [sys.executable, str(root / "scripts/check-trnm-world-v4-candidate.py")],
        [sys.executable, str(root / "scripts/check-trnm-world-active-closure.py")],
    ]
    for command in commands:
        try:
            completed = subprocess.run(command, cwd=root, text=True, capture_output=True, timeout=180)
        except (OSError, subprocess.TimeoutExpired) as exc:
            die(f"required documentation command failed to execute: {command}: {exc}")
        if completed.returncode != 0:
            sys.stdout.write(completed.stdout)
            sys.stderr.write(completed.stderr)
            die(f"required documentation command failed: {command}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=str(pathlib.Path(__file__).resolve().parents[1]))
    parser.add_argument("--workflows-only", action="store_true")
    args = parser.parse_args()
    root = pathlib.Path(args.root).resolve()
    contexts = workflow_inventory(root)
    if args.workflows_only:
        print(f"TRNM World CI integrity: PASS ({len(contexts)} unique contexts; workflow inventory only, no hosted/governance evidence)")
        return
    run_required_documentation(root)
    print(f"TRNM World CI integrity: PASS ({len(contexts)} unique contexts; current documentation commands also passed)")


if __name__ == "__main__":
    main()
