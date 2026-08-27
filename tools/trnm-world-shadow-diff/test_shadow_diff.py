#!/usr/bin/env python3
from __future__ import annotations

import copy
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
    compare,
    load_jsonl,
)

VECTOR = (
    REPOSITORY_ROOT
    / "trillionnium/contracts/trnm-world-rules-contract-v1/vectors/first-contact-vector-0001.json"
)


class ShadowDiffTests(unittest.TestCase):
    def setUp(self) -> None:
        vector, receipt = load_vector(VECTOR)
        self.fixture_id = vector["fixture_id"]
        self.record = {"fixture_id": self.fixture_id, **receipt.record}

    def write_jsonl(self, directory: pathlib.Path, name: str, record: dict) -> pathlib.Path:
        path = directory / name
        path.write_text(json.dumps(record, sort_keys=True) + "\n")
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

    def test_one_hash_difference_fails_closed_and_names_field(self) -> None:
        divergent = copy.deepcopy(self.record)
        divergent["outcome_hash"] = "f" * 64
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            world = load_jsonl(self.write_jsonl(root, "world.jsonl", self.record))
            candidate = load_jsonl(self.write_jsonl(root, "candidate.jsonl", divergent))
            summary = compare(world, candidate)
            self.assertEqual(summary["comparison_status"], "failed")
            self.assertEqual(summary["mismatches"][0]["fields"], ["outcome_hash"])

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
                + "\n"
            )
            candidate_path = self.write_jsonl(root, "candidate.jsonl", self.record)
            summary = compare(load_jsonl(world_path), load_jsonl(candidate_path))
            self.assertEqual(summary["comparison_status"], "failed")
            self.assertEqual(summary["missing_fixture_ids"], ["first-contact-vector-0002"])

    def test_rejected_record_cannot_commit_game_output(self) -> None:
        invalid = copy.deepcopy(self.record)
        invalid["disposition"] = "rejected"
        invalid["error_code"] = "domain_rejected"
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


if __name__ == "__main__":
    unittest.main()
