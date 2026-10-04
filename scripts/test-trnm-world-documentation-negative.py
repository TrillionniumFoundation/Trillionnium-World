#!/usr/bin/env python3
"""Prove the World documentation checker rejects governance and status overclaims."""

from __future__ import annotations

import json
import pathlib
import shutil
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-documentation.py"


def run(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["python3", str(CHECKER), str(root)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )


def copy_fixture(destination: pathlib.Path) -> None:
    required = [
        "README.md",
        "CURRENT_PLAN.md",
        "PROJECT_BOUNDARY.md",
        "PROJECT_BOUNDARY.json",
        "AGENTS.md",
        "docs/README.md",
        "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md",
        "docs/development/trillionnium-world-development-plan-2026-08-29.json",
        "docs/development/trnm-world-gap-closure-ledger-v4.json",
        "docs/status/CURRENT.md",
        "docs/status/world-gates-v1.json",
        "docs/architecture/trnm-world-system-context-v1.md",
        "docs/architecture/trnm-world-authority-state-ownership-v1.md",
        "docs/architecture/trnm-settlement-lifecycle-v2.md",
        "docs/architecture/trnm-determinism-and-canonical-json-v1.md",
        "docs/security/trnm-world-threat-model-v1.md",
        "docs/database/trnm-world-postgres-contract-v1.md",
        "docs/release/trnm-world-release-gate-matrix-v2.md",
        "docs/release/trnm-world-evidence-record-v1.md",
        "docs/runbooks/trnm-settlement-shutdown-and-quarantine-v1.md",
        "docs/runbooks/trnm-authority-cutover-rollback-v1.md",
    ]
    for relative in required:
        source = ROOT / relative
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, target)
    shutil.copytree(ROOT / ".github/workflows", destination / ".github/workflows")


baseline = run(ROOT)
if baseline.returncode != 0:
    raise SystemExit(f"baseline documentation check failed:\n{baseline.stdout}")

mutations = []


def public_online_overclaim(root: pathlib.Path) -> None:
    path = root / "PROJECT_BOUNDARY.json"
    value = json.loads(path.read_text(encoding="utf-8"))
    value["release"]["public_online"] = "ready"
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


mutations.append(("public online overclaim", public_online_overclaim))


def write_workflow(root: pathlib.Path) -> None:
    (root / ".github/workflows/evil.yml").write_text(
        "name: evil\non: workflow_dispatch\npermissions:\n  contents: write\njobs: {}\n",
        encoding="utf-8",
    )


mutations.append(("write-enabled workflow", write_workflow))


def duplicate_gap(root: pathlib.Path) -> None:
    path = root / "docs/development/trnm-world-gap-closure-ledger-v4.json"
    value = json.loads(path.read_text(encoding="utf-8"))
    value["entries"].append(value["entries"][0])
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


mutations.append(("duplicate gap", duplicate_gap))


def stale_plan(root: pathlib.Path) -> None:
    path = root / "CURRENT_PLAN.md"
    text = path.read_text(encoding="utf-8").replace(
        "TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md",
        "TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md",
    )
    path.write_text(text, encoding="utf-8")


mutations.append(("stale plan pointer", stale_plan))

for name, mutation in mutations:
    with tempfile.TemporaryDirectory(prefix="trnm-world-doc-negative-") as temporary:
        fixture = pathlib.Path(temporary)
        copy_fixture(fixture)
        mutation(fixture)
        result = run(fixture)
        if result.returncode == 0:
            raise SystemExit(f"negative fixture unexpectedly passed: {name}")

print("TRNM World documentation negative fixtures: PASS")
