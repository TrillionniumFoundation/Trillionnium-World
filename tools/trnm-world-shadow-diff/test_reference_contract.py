#!/usr/bin/env python3
from __future__ import annotations

import copy
import pathlib
import sys
import unittest

TOOL_ROOT = pathlib.Path(__file__).resolve().parent
REPOSITORY_ROOT = TOOL_ROOT.parents[1]
sys.path.insert(0, str(TOOL_ROOT))

from reference_contract import (  # noqa: E402
    CONTRACT_VERSION,
    ZERO_HASH,
    AppliedOutput,
    ContractError,
    applied_receipt,
    canonical_request,
    load_vector,
    rejected_receipt,
)

VECTOR = (
    REPOSITORY_ROOT
    / "trillionnium/contracts/trnm-world-rules-contract-v1/vectors/first-contact-vector-0001.json"
)


class ReferenceContractTests(unittest.TestCase):
    def test_committed_vector_round_trips(self) -> None:
        vector, receipt = load_vector(VECTOR)
        self.assertEqual(vector["request"]["contract_version"], CONTRACT_VERSION)
        self.assertTrue(receipt.record["transition_hash"])
        self.assertTrue(receipt.canonical.endswith(b"\n"))
        self.assertEqual(receipt.record["disposition"], "applied")

    def test_rejected_receipt_has_no_game_output_commitment(self) -> None:
        vector, _ = load_vector(VECTOR)
        receipt = rejected_receipt(vector["request"], "domain_rejected")
        self.assertEqual(receipt.record["state_after_hash"], ZERO_HASH)
        self.assertEqual(receipt.record["outcome_hash"], ZERO_HASH)
        self.assertEqual(receipt.record["replay_hash"], ZERO_HASH)
        self.assertEqual(receipt.record["steps_used"], 0)

    def test_unknown_contract_version_is_rejected(self) -> None:
        vector, _ = load_vector(VECTOR)
        request = copy.deepcopy(vector["request"])
        request["contract_version"] = "trnm_world_rules_v2"
        with self.assertRaises(ContractError):
            canonical_request(request)

    def test_new_online_field_is_rejected_not_ignored(self) -> None:
        vector, _ = load_vector(VECTOR)
        request = copy.deepcopy(vector["request"])
        request["player_session"] = "forbidden"
        with self.assertRaises(ContractError):
            canonical_request(request)

    def test_budget_breach_is_rejected_before_receipt(self) -> None:
        vector, _ = load_vector(VECTOR)
        request = vector["request"]
        with self.assertRaises(ContractError):
            applied_receipt(
                request,
                AppliedOutput(
                    state_after=b"x" * 4097,
                    outcome=b"",
                    replay=b"",
                    steps_used=1,
                ),
            )


if __name__ == "__main__":
    unittest.main()
