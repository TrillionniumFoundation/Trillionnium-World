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


def mutate_hidden_ci_blocker(value: dict[str, Any]) -> None:
    value["open_gates"].remove("obtain_exact_commit_github_actions_evidence")


def mutate_hidden_cex_owner_blocker(value: dict[str, Any]) -> None:
    value["open_gates"].remove("land_cex_receipt_lookup_endpoint_in_owner_repository")


def mutate_extra_claim(value: dict[str, Any]) -> None:
    value["release_ready"] = True


def mutate_capture_scoped_identity_claim(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove(
        "stable_remote_request_identity_excludes_capture_generation"
    )


def mutate_mutable_identity_alias_claim(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove(
        "settlement_identity_fields_and_aliases_are_immutable"
    )


def mutate_missing_signer_recovery(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove("signer_receipt_lookup_precedes_sign")


def mutate_missing_cex_hash_binding(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove("cex_receipt_lookup_binds_intent_id_and_hash")


def mutate_missing_serialization(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove(
        "account_or_campaign_work_is_serialized_without_global_fifo"
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
            ("hidden-ci-blocker", mutate_hidden_ci_blocker),
            ("hidden-cex-owner-blocker", mutate_hidden_cex_owner_blocker),
            ("extra-release-claim", mutate_extra_claim),
            ("capture-scoped-identity-regression", mutate_capture_scoped_identity_claim),
            ("mutable-identity-alias-regression", mutate_mutable_identity_alias_claim),
            ("missing-signer-recovery", mutate_missing_signer_recovery),
            ("missing-cex-hash-binding", mutate_missing_cex_hash_binding),
            ("missing-account-serialization", mutate_missing_serialization),
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

    print("TRNM settlement runtime status negative fixtures: passed (10/10 rejected)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
