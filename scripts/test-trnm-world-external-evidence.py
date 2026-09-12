#!/usr/bin/env python3
"""Hostile fixtures for check-trnm-world-external-evidence.py."""
from __future__ import annotations

import copy
import datetime as dt
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("check-trnm-world-external-evidence.py")
SPEC = importlib.util.spec_from_file_location("trnm_world_external_evidence", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot import {SCRIPT}")
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)

NOW = dt.datetime(2026, 9, 9, 6, 0, 0, tzinfo=dt.timezone.utc)
HEX40_A = "1" * 40
HEX40_B = "2" * 40
HEX64_A = "a" * 64
HEX64_B = "b" * 64


def record() -> dict[str, object]:
    return {
        "schema": "trnm_world_evidence_record_v1",
        "record_id": "WORLD-V6-015-cross-host-20260909-a",
        "claim_id": "WORLD-V6-015",
        "evidence_class": "cross_host",
        "decision": "pass",
        "scope": {
            "capability": "PostgreSQL failover, PITR and old-primary isolation",
            "explicit_non_claims": ["public network", "custody", "commercial authorization"],
            "limitations": ["two declared failure domains only"],
        },
        "source_identity": {
            "repository": "TrillionniumFoundation/Trillionnium-World",
            "commit": HEX40_A,
            "tree": HEX40_B,
            "branch_or_tag_observed": "fix/world-v6-ci-finalize-20260909",
            "dirty": False,
        },
        "build_identity": {
            "toolchain": "rustc 1.98.1",
            "lockfile_sha256": HEX64_A,
            "binary_or_package_sha256": [HEX64_B],
            "sbom_sha256": HEX64_A,
            "provenance_sha256": HEX64_B,
        },
        "component_lock": {
            "world": {"commit": HEX40_A, "tree": HEX40_B},
            "nakama": None,
            "cex": None,
            "chain": None,
            "integration": None,
        },
        "environment": {
            "class": "cross_host",
            "topology": "host-a primary, host-b promoted target, isolated witness storage",
            "operating_systems": ["Linux"],
            "hardware": ["two independently identified hosts"],
            "database": "PostgreSQL 16 exact image digest",
            "network": "private routed network with injected partitions",
            "regions": ["test-region-a", "test-region-b"],
            "configuration_sha256": [HEX64_A],
            "secret_values_recorded": False,
        },
        "execution": {
            "started_at_utc": "2026-09-09T01:00:00Z",
            "ended_at_utc": "2026-09-09T03:00:00Z",
            "commands": ["run-failover-matrix", "run-pitr-matrix", "probe-old-primary"],
            "exit_codes": [0, 0, 0],
            "thresholds_declared_before_run": ["zero duplicate value", "zero stale publication"],
            "fault_injections": ["primary kill", "network isolation", "old primary return"],
            "cleanup_and_rollback": ["quarantine test cluster", "revoke test credentials"],
            "terminal_summary": "All declared cross-host criteria passed with no duplicate value or stale publication.",
        },
        "artifacts": [
            {
                "name": "cross-host-raw-evidence.tar.zst",
                "sha256": HEX64_A,
                "size_bytes": 4096,
                "retention_location": "immutable://evidence/world-v6/015/a",
                "retention_until": "2026-12-31T00:00:00Z",
                "contains_personal_or_secret_data": False,
            }
        ],
        "results": {
            "criteria": ["fence rejects old primary", "PITR reconciles exact receipts"],
            "observations": ["old primary returned read-only", "receipt recovery remained exact-once"],
            "failures": [],
            "unexplained_divergence_count": 0,
            "all_applicable_criteria_passed": True,
        },
        "review": {
            "executor": "world-sre-executor-a",
            "independent_reviewer": "world-sre-reviewer-b",
            "reviewed_at_utc": "2026-09-09T04:00:00Z",
            "review_comment_or_decision_uri": "immutable://reviews/world-v6/015/a",
            "reviewer_is_independent": True,
            "conflicts_disclosed": [],
        },
        "validity": {
            "expires_at_utc": "2026-10-09T04:00:00Z",
            "invalidated_by": [
                "source_commit_or_tree_change",
                "binary_or_package_change",
                "component_lock_change",
                "environment_or_topology_change_outside_declared_scope",
            ],
        },
        "authorization": {
            "production_authorization": "not_granted",
            "authorized_scope": None,
            "accountable_human_decision": None,
        },
    }


def write(value: object, *, raw: str | None = None) -> tuple[tempfile.TemporaryDirectory[str], Path]:
    temporary = tempfile.TemporaryDirectory()
    path = Path(temporary.name) / "record.json"
    path.write_text(raw if raw is not None else json.dumps(value, indent=2), encoding="utf-8")
    return temporary, path


class ExternalEvidenceTests(unittest.TestCase):
    def validate(self, value: object) -> dict[str, object]:
        temporary, path = write(value)
        with temporary:
            return CHECKER.validate(path, now=NOW)

    def reject(self, value: object, pattern: str) -> None:
        temporary, path = write(value)
        with temporary:
            with self.assertRaisesRegex(CHECKER.EvidenceError, pattern):
                CHECKER.validate(path, now=NOW)

    def test_valid_cross_host_record_passes(self) -> None:
        result = self.validate(record())
        self.assertEqual(result["status"], "pass")
        self.assertEqual(result["claim_id"], "WORLD-V6-015")
        self.assertEqual(result["evidence_class"], "cross_host")

    def test_placeholder_rejected(self) -> None:
        value = record()
        value["scope"]["capability"] = "REPLACE_WITH_CAPABILITY"  # type: ignore[index]
        self.reject(value, "placeholder")

    def test_duplicate_key_rejected(self) -> None:
        temporary, path = write({}, raw='{"schema":"trnm_world_evidence_record_v1","schema":"duplicate"}')
        with temporary:
            with self.assertRaisesRegex(CHECKER.EvidenceError, "duplicate JSON key"):
                CHECKER.validate(path, now=NOW)

    def test_short_endurance_rejected(self) -> None:
        value = record()
        value["claim_id"] = "WORLD-V6-016"
        value["execution"]["ended_at_utc"] = "2026-09-09T23:00:00Z"  # type: ignore[index]
        self.reject(value, "shorter than 24 hours")

    def test_same_executor_and_reviewer_rejected(self) -> None:
        value = record()
        value["review"]["independent_reviewer"] = value["review"]["executor"]  # type: ignore[index]
        self.reject(value, "independently identified")

    def test_expired_record_rejected(self) -> None:
        value = record()
        value["validity"]["expires_at_utc"] = "2026-09-09T05:00:00Z"  # type: ignore[index]
        self.reject(value, "expired")

    def test_nonfinal_evidence_cannot_grant_production(self) -> None:
        value = record()
        value["authorization"]["production_authorization"] = "granted_by_explicit_human_decision"  # type: ignore[index]
        self.reject(value, "cannot grant production")

    def test_pass_with_nonzero_exit_rejected(self) -> None:
        value = record()
        value["execution"]["exit_codes"] = [0, 1, 0]  # type: ignore[index]
        self.reject(value, "nonzero command exit")

    def test_pass_with_unexplained_divergence_rejected(self) -> None:
        value = record()
        value["results"]["unexplained_divergence_count"] = 1  # type: ignore[index]
        self.reject(value, "unexplained divergence")

    def test_pending_record_rejected(self) -> None:
        value = record()
        value["decision"] = "pending"
        self.reject(value, "must be pass or fail")

    def test_environment_class_mismatch_rejected(self) -> None:
        value = record()
        value["environment"]["class"] = "single_host_runtime"  # type: ignore[index]
        self.reject(value, "requires environment.class=cross_host")


if __name__ == "__main__":
    unittest.main(verbosity=2)
