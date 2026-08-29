#!/usr/bin/env python3
from __future__ import annotations

import copy
import hashlib
import json
import pathlib
import sys
import tempfile
import unittest

TOOL_ROOT = pathlib.Path(__file__).resolve().parent
REPOSITORY_ROOT = TOOL_ROOT.parents[1]
sys.path.insert(0, str(TOOL_ROOT))

from reference_contract import load_vector  # noqa: E402
from trnm_world_shadow_diff import (  # noqa: E402
    ShadowDiffError,
    canonical_receipt_without_transition_hash,
    compare,
    load_jsonl,
)

VECTOR = (
    REPOSITORY_ROOT
    / "trillionnium/contracts/trnm-world-rules-contract-v1/vectors/first-contact-vector-0001.json"
)
ZERO_HASH = "0" * 64


class ShadowDiffTests(unittest.TestCase):
    def setUp(self) -> None:
        vector, receipt = load_vector(VECTOR)
        self.fixture_id = vector["fixture_id"]
        self.record = {"fixture_id": self.fixture_id, **receipt.record}

    @staticmethod
    def refresh_transition_hash(record: dict) -> None:
        record["transition_hash"] = hashlib.sha256(
            canonical_receipt_without_transition_hash(record)
        ).hexdigest()

    @staticmethod
    def write_jsonl(directory: pathlib.Path, name: str, record: dict) -> pathlib.Path:
        path = directory / name
        path.write_text(json.dumps(record, sort_keys=True) + "\n", encoding="utf-8")
        return path

    def test_matching_world_and_candidate_pass(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            world = load_jsonl(self.write_jsonl(root, "world.jsonl", self.record))
            candidate = load_jsonl(self.write_jsonl(root, "candidate.jsonl", self.record))
            summary = compare(world, candidate)
            self.assertEqual(summary["comparison_status"], "passed")
            self.assertEqual(summary["matched_records"], 1)
            self.assertEqual(summary["mismatches"], [])

    def test_one_hash_difference_fails_closed_and_names_committed_fields(self) -> None:
        divergent = copy.deepcopy(self.record)
        divergent["outcome_hash"] = "f" * 64
        self.refresh_transition_hash(divergent)
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            world = load_jsonl(self.write_jsonl(root, "world.jsonl", self.record))
            candidate = load_jsonl(self.write_jsonl(root, "candidate.jsonl", divergent))
            summary = compare(world, candidate)
            self.assertEqual(summary["comparison_status"], "failed")
            self.assertEqual(
                summary["mismatches"][0]["fields"],
                ["outcome_hash", "transition_hash"],
            )

    def test_missing_candidate_fixture_fails_closed(self) -> None:
        second = copy.deepcopy(self.record)
        second["fixture_id"] = "first-contact-vector-0002"
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            world_path = root / "world.jsonl"
            world_path.write_text(
                json.dumps(self.record, sort_keys=True)
                + "\n"
                + json.dumps(second, sort_keys=True)
                + "\n",
                encoding="utf-8",
            )
            candidate_path = self.write_jsonl(root, "candidate.jsonl", self.record)
            summary = compare(load_jsonl(world_path), load_jsonl(candidate_path))
            self.assertEqual(summary["comparison_status"], "failed")
            self.assertEqual(summary["missing_fixture_ids"], ["first-contact-vector-0002"])

    def test_rejected_record_cannot_commit_game_output(self) -> None:
        invalid = copy.deepcopy(self.record)
        invalid["disposition"] = "rejected"
        invalid["error_code"] = "domain_rejected"
        self.refresh_transition_hash(invalid)
        with tempfile.TemporaryDirectory() as temporary:
            path = self.write_jsonl(pathlib.Path(temporary), "invalid.jsonl", invalid)
            with self.assertRaises(ShadowDiffError):
                load_jsonl(path)

    def test_extra_online_authority_field_is_rejected(self) -> None:
        invalid = copy.deepcopy(self.record)
        invalid["player_session"] = "forbidden"
        with tempfile.TemporaryDirectory() as temporary:
            path = self.write_jsonl(pathlib.Path(temporary), "invalid.jsonl", invalid)
            with self.assertRaises(ShadowDiffError):
                load_jsonl(path)

    def test_duplicate_json_key_is_rejected_before_comparison(self) -> None:
        encoded = json.dumps(self.record, sort_keys=True)
        duplicate = encoded[:-1] + ',"fixture_id":"duplicate"}\n'
        with tempfile.TemporaryDirectory() as temporary:
            path = pathlib.Path(temporary) / "duplicate.jsonl"
            path.write_text(duplicate, encoding="utf-8")
            with self.assertRaises(ShadowDiffError):
                load_jsonl(path)

    def test_unknown_rejection_code_is_rejected_even_when_both_sides_match(self) -> None:
        invalid = copy.deepcopy(self.record)
        invalid.update(
            {
                "disposition": "rejected",
                "error_code": "future_unregistered_error",
                "state_after_hash": ZERO_HASH,
                "outcome_hash": ZERO_HASH,
                "replay_hash": ZERO_HASH,
                "steps_used": 0,
                "output_bytes": 0,
                "replay_bytes": 0,
            }
        )
        self.refresh_transition_hash(invalid)
        with tempfile.TemporaryDirectory() as temporary:
            path = self.write_jsonl(pathlib.Path(temporary), "unknown-error.jsonl", invalid)
            with self.assertRaises(ShadowDiffError):
                load_jsonl(path)

    def test_transition_hash_must_bind_the_supplied_record(self) -> None:
        invalid = copy.deepcopy(self.record)
        invalid["transition_hash"] = "f" * 64
        with tempfile.TemporaryDirectory() as temporary:
            path = self.write_jsonl(pathlib.Path(temporary), "bad-hash.jsonl", invalid)
            with self.assertRaises(ShadowDiffError):
                load_jsonl(path)

    def test_nonfinite_json_number_is_rejected(self) -> None:
        encoded = json.dumps(self.record, sort_keys=True)
        encoded = encoded.replace(
            f'"steps_used": {self.record["steps_used"]}',
            '"steps_used": NaN',
            1,
        )
        with tempfile.TemporaryDirectory() as temporary:
            path = pathlib.Path(temporary) / "nan.jsonl"
            path.write_text(encoded + "\n", encoding="utf-8")
            with self.assertRaises(ShadowDiffError):
                load_jsonl(path)


if __name__ == "__main__":
    unittest.main()
