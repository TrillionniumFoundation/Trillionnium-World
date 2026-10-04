#!/usr/bin/env python3
"""Negative tests for the shared bounded strict JSON loader."""

from __future__ import annotations

import json
from pathlib import Path
import tempfile
import unittest

from trnm_world_strict_json import StrictJsonError, load_strict_json, loads_strict


class StrictJsonTests(unittest.TestCase):
    def test_valid_document(self) -> None:
        self.assertEqual(loads_strict('{"a":[1,true,null,"x"]}'), {"a": [1, True, None, "x"]})

    def test_duplicate_keys_at_all_depths_are_rejected(self) -> None:
        for raw in ('{"a":1,"a":2}', '{"outer":{"a":1,"a":2}}'):
            with self.subTest(raw=raw), self.assertRaises(StrictJsonError):
                loads_strict(raw)

    def test_non_finite_tokens_are_rejected(self) -> None:
        for token in ("NaN", "Infinity", "-Infinity"):
            with self.subTest(token=token), self.assertRaises(StrictJsonError):
                loads_strict('{"value":' + token + "}")

    def test_depth_and_node_budgets_are_rejected(self) -> None:
        with self.assertRaises(StrictJsonError):
            loads_strict("[[[[0]]]]", max_depth=3)
        with self.assertRaises(StrictJsonError):
            loads_strict(json.dumps(list(range(20))), max_nodes=10)

    def test_invalid_utf8_size_empty_and_symlink_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory(prefix="strict-json-") as directory:
            root = Path(directory)
            bad = root / "bad.json"
            bad.write_bytes(b'{"x":"\xff"}')
            with self.assertRaises(StrictJsonError):
                load_strict_json(bad)
            big = root / "big.json"
            big.write_text('{"x":"' + "a" * 32 + '"}', encoding="utf-8")
            with self.assertRaises(StrictJsonError):
                load_strict_json(big, max_bytes=8)
            empty = root / "empty.json"
            empty.write_bytes(b"")
            with self.assertRaises(StrictJsonError):
                load_strict_json(empty)
            valid = root / "valid.json"
            valid.write_text("{}", encoding="utf-8")
            link = root / "link.json"
            link.symlink_to(valid)
            with self.assertRaises(StrictJsonError):
                load_strict_json(link)


if __name__ == "__main__":
    result = unittest.TextTestRunner(verbosity=1).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(StrictJsonTests)
    )
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World shared strict JSON negative fixtures: PASS")
