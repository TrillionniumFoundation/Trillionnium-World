#!/usr/bin/env python3
"""Negative fixtures for the WORLD-P0-001 v4 status contract."""

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
        timeout=15,
    )


def remove_control(name: str) -> Callable[[dict[str, Any]], None]:
    return lambda value: value["implemented_controls"].remove(name)


def remove_gate(name: str) -> Callable[[dict[str, Any]], None]:
    return lambda value: value["open_gates"].remove(name)


def main() -> int:
    baseline = json.loads(STATUS.read_text(encoding="utf-8"))
    cases: list[tuple[str, Callable[[dict[str, Any]], None]]] = [
        ("unpublished-source-claimed-implemented", lambda value: value.__setitem__("status", "implemented_pending_exact_commit_ci")),
        ("hidden-source-publication-gate", remove_gate("publish_reviewed_direct_source_and_successor_manifest")),
        ("verified-without-evidence", lambda value: value.__setitem__("status", "verified_remote")),
        ("verified-commit-invented", lambda value: value.__setitem__("verified_commit", "1" * 40)),
        ("public-online-overclaim", lambda value: value.__setitem__("public_online", "enabled")),
        ("market-overclaim", lambda value: value.__setitem__("public_player_market", "enabled")),
        (
            "release-overclaim",
            lambda value: value.__setitem__("release_effect", "trusted_cex_settlement_candidate"),
        ),
        ("wrong-branch", lambda value: value.__setitem__("branch", "main")),
        ("wrong-base", lambda value: value.__setitem__("base_commit", "0" * 40)),
        ("hidden-ci-gate", remove_gate("run_exact_head_v4_checks")),
        ("hidden-cex-merge-gate", remove_gate("merge_cex_owner_repository_pull_request")),
        ("hidden-deployment-gate", remove_gate("bind_exact_cex_build_and_deployment_artifact")),
        ("hidden-review-gate", remove_gate("obtain_reviewer_signoff")),
        ("shutdown-control-removed", remove_control("sigint_sigterm_stop_new_admission")),
        ("quarantine-control-removed", remove_control("poison_match_job_capture_quarantine")),
        ("ambiguity-control-removed", remove_control("malformed_success_is_ambiguous_retryable")),
        ("migration-chain-control-removed", remove_control("game_server_and_worker_register_migrations_16_through_19")),
        (
            "required-context-replaced",
            lambda value: value["required_checks"].__setitem__(0, "trnm-world-v4/fake"),
        ),
        ("future-run-invented", lambda value: value["evidence"]["remote_workflow_runs"].append(1)),
        ("review-invented", lambda value: value["evidence"]["reviewers"].append("reviewer")),
        ("extra-claim", lambda value: value.__setitem__("release_ready", True)),
        ("legacy-source-control", lambda value: value["implemented_controls"].__setitem__(
            value["implemented_controls"].index("ordinary_compiled_source_excludes_semantic_generation"),
            "generated_runtime_source_fails_closed_on_template_drift")),
        ("compact-date", lambda value: value.__setitem__("as_of", "20260905")),
        ("invalid-calendar-date", lambda value: value.__setitem__("as_of", "2026-02-30")),
        ("missing-limitation", lambda value: value["evidence"]["limitations"].pop()),
        ("null-run-list", lambda value: value["evidence"].__setitem__("remote_workflow_runs", None)),
        ("object-artifact-list", lambda value: value["evidence"].__setitem__("artifacts", {})),
        ("false-review-list", lambda value: value["evidence"].__setitem__("reviewers", False)),
        ("string-run-list", lambda value: value["evidence"].__setitem__("remote_workflow_runs", "")),
    ]

    raw_case_count = 0
    with tempfile.TemporaryDirectory(prefix="trnm-settlement-negative-") as directory:
        directory_path = pathlib.Path(directory)
        baseline_path = directory_path / "baseline.json"
        baseline_path.write_text(json.dumps(baseline, indent=2) + "\n", encoding="utf-8")
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

        raw_cases = [
            ("duplicate-key", json.dumps(baseline).replace('"schema":', '"schema": "crossed", "schema":', 1)),
            ("nonfinite", json.dumps(baseline).replace('"verified_commit": null', '"verified_commit": NaN')),
            ("oversize", " " * (256 * 1024 + 1)),
            ("array-root", "[]"),
        ]
        for name, raw in raw_cases:
            path = directory_path / f"{name}.json"
            path.write_text(raw, encoding="utf-8")
            if invoke(path).returncode == 0:
                print(f"raw negative unexpectedly passed: {name}", file=sys.stderr)
                return 1
            raw_case_count += 1
        linked = directory_path / "linked.json"
        linked.symlink_to(baseline_path)
        if invoke(linked).returncode == 0:
            print("linked status source unexpectedly passed", file=sys.stderr)
            return 1
        raw_case_count += 1

    print(
        "TRNM settlement runtime status negative fixtures: "
        f"PASS ({len(cases) + raw_case_count}/{len(cases) + raw_case_count} rejected)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
