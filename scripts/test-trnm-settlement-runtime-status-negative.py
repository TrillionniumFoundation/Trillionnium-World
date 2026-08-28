#!/usr/bin/env python3
"""Negative fixtures for the WORLD-P0-001 machine-readable status gate."""

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


def invoke(status_path: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            sys.executable,
            str(CHECKER),
            "--repo",
            str(ROOT),
            "--status",
            str(status_path),
            "--schema",
            str(SCHEMA),
            "--skip-source",
        ],
        check=False,
        capture_output=True,
        text=True,
    )


def mutate_verified_without_evidence(value: dict[str, Any]) -> None:
    value["status"] = "verified_remote"
    value["verified_commit"] = "1" * 40
    value["release_effect"] = "trusted_cex_settlement_candidate"


def mutate_public_online(value: dict[str, Any]) -> None:
    value["public_online"] = "enabled"


def mutate_hidden_blocker(value: dict[str, Any]) -> None:
    value["open_gates"].remove("obtain_exact_commit_github_actions_evidence")


def mutate_extra_claim(value: dict[str, Any]) -> None:
    value["release_ready"] = True


def mutate_capture_scoped_identity_claim(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove(
        "stable_remote_request_identity_excludes_capture_generation"
    )


def main() -> int:
    baseline = json.loads(STATUS.read_text(encoding="utf-8"))
    with tempfile.TemporaryDirectory(prefix="trnm-settlement-status-negative-") as directory:
        directory_path = pathlib.Path(directory)
        baseline_path = directory_path / "baseline.json"
        baseline_path.write_text(
            json.dumps(baseline, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        baseline_result = invoke(baseline_path)
        if baseline_result.returncode != 0:
            print("baseline status fixture did not pass:", file=sys.stderr)
            print(baseline_result.stdout, file=sys.stderr)
            print(baseline_result.stderr, file=sys.stderr)
            return 1

        cases: list[tuple[str, Callable[[dict[str, Any]], None]]] = [
            ("verified-without-evidence", mutate_verified_without_evidence),
            ("public-online-overclaim", mutate_public_online),
            ("hidden-ci-blocker", mutate_hidden_blocker),
            ("extra-release-claim", mutate_extra_claim),
            ("capture-scoped-identity-regression", mutate_capture_scoped_identity_claim),
        ]
        for name, mutator in cases:
            fixture = copy.deepcopy(baseline)
            mutator(fixture)
            fixture_path = directory_path / f"{name}.json"
            fixture_path.write_text(
                json.dumps(fixture, indent=2, sort_keys=True) + "\n",
                encoding="utf-8",
            )
            result = invoke(fixture_path)
            if result.returncode == 0:
                print(f"negative fixture unexpectedly passed: {name}", file=sys.stderr)
                return 1

    print("TRNM settlement runtime status negative fixtures: passed (5/5 rejected)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
