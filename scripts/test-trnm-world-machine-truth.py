#!/usr/bin/env python3
"""Negative regressions for the World machine-truth hierarchy."""

from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "world_machine_truth",
    Path(__file__).with_name("check-trnm-world-machine-truth.py"),
)
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)
ROOT = Path(__file__).resolve().parents[1]


class MachineTruthTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.documents, cls.current_plan = M.load_sources(ROOT)

    def fixture(self):
        return copy.deepcopy(self.documents), self.current_plan

    def expect_failure(self, documents, current_plan=None) -> None:
        with self.assertRaises(M.MachineTruthFailure):
            M.validate_documents(documents, current_plan or self.current_plan)

    def test_current_repository_truth_is_consistent(self) -> None:
        M.validate_documents(copy.deepcopy(self.documents), self.current_plan)

    def test_old_plan_cannot_become_current_boundary(self) -> None:
        documents, plan = self.fixture()
        documents["PROJECT_BOUNDARY.json"]["documentation"]["current_plan"] = M.HISTORICAL_PLAN
        self.expect_failure(documents, plan)

    def test_old_ledger_cannot_enter_truth_order(self) -> None:
        documents, plan = self.fixture()
        documents["docs/catalog.json"]["truth_order"].insert(3, M.HISTORICAL_LEDGER)
        self.expect_failure(documents, plan)

    def test_operational_candidate_cannot_revert_to_old_branch(self) -> None:
        documents, plan = self.fixture()
        documents[M.CURRENT_SNAPSHOT]["operative_pull_request"] = 46
        documents[M.CURRENT_SNAPSHOT]["operative_branch"] = "fix/world-plan-v4-development-closure-20260831"
        self.expect_failure(documents, plan)

    def test_zero_runs_cannot_be_promoted_to_exact_head_ci_closed(self) -> None:
        documents, plan = self.fixture()
        documents[M.CURRENT_SNAPSHOT]["closure"]["exact_head_ci_closed"] = True
        self.expect_failure(documents, plan)

    def test_external_gap_cannot_be_closed_by_assertion(self) -> None:
        documents, plan = self.fixture()
        documents[M.CURRENT_SNAPSHOT]["external_evidence_gaps"]["deployment_and_public_edge"] = "closed"
        self.expect_failure(documents, plan)

    def test_desired_protection_contract_cannot_claim_observed_enforcement(self) -> None:
        documents, plan = self.fixture()
        documents[M.PROTECTION_CONTRACT]["server_state"] = "enforced"
        self.expect_failure(documents, plan)

    def test_production_authorization_cannot_flip_positive(self) -> None:
        documents, plan = self.fixture()
        documents[M.CUTOVER_CONTRACT]["production_authorization"] = "granted"
        self.expect_failure(documents, plan)

    def test_unimplemented_adapters_cannot_claim_availability(self) -> None:
        documents, plan = self.fixture()
        profile = documents[M.CUTOVER_CONTRACT]["profiles"]["production"]
        profile["implementation_state"] = "implemented"
        profile["exact_head_qualified"] = True
        self.expect_failure(documents, plan)

    def test_provenance_counts_cannot_drift(self) -> None:
        documents, plan = self.fixture()
        documents[M.PROVENANCE_CONTRACT]["counts"]["reviewed_successor"] = 1
        self.expect_failure(documents, plan)

    def test_historical_gate_cannot_enable_public_market(self) -> None:
        documents, plan = self.fixture()
        for gate in documents[M.HISTORICAL_GATES]["gates"]:
            if gate["id"] == "public_player_market":
                gate["status"] = "enabled"
        self.expect_failure(documents, plan)

    def test_current_plan_must_name_pr_104_and_no_credit(self) -> None:
        documents, plan = self.fixture()
        plan = plan.replace("Pull request: `#104`", "Pull request: `#46`")
        self.expect_failure(documents, plan)


if __name__ == "__main__":
    result = unittest.TextTestRunner(verbosity=1).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(MachineTruthTests)
    )
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World machine-truth negative fixtures: PASS")
