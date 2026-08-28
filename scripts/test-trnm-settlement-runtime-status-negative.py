#!/usr/bin/env python3
"""Negative fixtures for the final WORLD-P0-001 status contract."""

from __future__ import annotations

import copy
import json
import pathlib
import subprocess
import sys
import tempfile
from typing import Any, Callable

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-settlement-runtime-status.py"
STATUS = ROOT / "docs/status/settlement-runtime-v1.json"
SCHEMA = ROOT / "docs/status/settlement-runtime-v1.schema.json"


def invoke(path: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            sys.executable,
            str(CHECKER),
            "--repo",
            str(ROOT),
            "--status",
            str(path),
            "--schema",
            str(SCHEMA),
            "--skip-source",
        ],
        check=False,
        capture_output=True,
        text=True,
    )


def remove_control(name: str) -> Callable[[dict[str, Any]], None]:
    return lambda value: value["implemented_controls"].remove(name)


def remove_gate(name: str) -> Callable[[dict[str, Any]], None]:
    return lambda value: value["open_gates"].remove(name)


def main() -> int:
    baseline = json.loads(STATUS.read_text(encoding="utf-8"))
    cases: list[tuple[str, Callable[[dict[str, Any]], None]]] = [
        (
            "verified-without-evidence",
            lambda value: value.__setitem__("status", "verified_remote"),
        ),
        (
            "verified-commit-invented",
            lambda value: value.__setitem__("verified_commit", "1" * 40),
        ),
        (
            "public-online-overclaim",
            lambda value: value.__setitem__("public_online", "enabled"),
        ),
        (
            "market-overclaim",
            lambda value: value.__setitem__("public_player_market", "enabled"),
        ),
        (
            "release-overclaim",
            lambda value: value.__setitem__(
                "release_effect", "trusted_cex_settlement_candidate"
            ),
        ),
        ("wrong-branch", lambda value: value.__setitem__("branch", "main")),
        (
            "hidden-ci-gate",
            remove_gate("obtain_exact_commit_github_actions_evidence"),
        ),
        (
            "hidden-cex-merge-gate",
            remove_gate("merge_cex_owner_repository_pull_request"),
        ),
        (
            "hidden-deployment-gate",
            remove_gate("bind_exact_cex_build_and_deployment_artifact"),
        ),
        ("hidden-review-gate", remove_gate("obtain_reviewer_signoff")),
        (
            "legacy-caller-control-removed",
            remove_control("legacy_in_process_settlement_caller_removed"),
        ),
        (
            "migration-chain-control-removed",
            remove_control("game_server_and_worker_register_migrations_16_through_18"),
        ),
        (
            "operator-replay-control-removed",
            remove_control("audited_exact_identity_operator_replay"),
        ),
        (
            "generated-source-control-removed",
            remove_control("generated_runtime_source_fails_closed_on_template_drift"),
        ),
        (
            "future-run-invented",
            lambda value: value["evidence"]["remote_workflow_runs"].append(1),
        ),
        ("extra-claim", lambda value: value.__setitem__("release_ready", True)),
    ]
    with tempfile.TemporaryDirectory(prefix="trnm-settlement-negative-") as directory:
        directory_path = pathlib.Path(directory)
        baseline_path = directory_path / "baseline.json"
        baseline_path.write_text(
            json.dumps(baseline, indent=2) + "\n", encoding="utf-8"
        )
        result = invoke(baseline_path)
        if result.returncode != 0:
            print(result.stdout, result.stderr, file=sys.stderr)
            return 1
        for name, mutate in cases:
            fixture = copy.deepcopy(baseline)
            mutate(fixture)
            path = directory_path / f"{name}.json"
            path.write_text(json.dumps(fixture, indent=2) + "\n", encoding="utf-8")
            if invoke(path).returncode == 0:
                print(f"negative fixture unexpectedly passed: {name}", file=sys.stderr)
                return 1
    print(
        "TRNM settlement runtime status negative fixtures: "
        f"PASS ({len(cases)}/{len(cases)} rejected)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
