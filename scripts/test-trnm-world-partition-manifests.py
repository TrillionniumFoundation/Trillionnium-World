#!/usr/bin/env python3
"""Negative and positive tests for source-partition manifest integrity."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

MODULE_PATH = Path(__file__).with_name("_trnm_world_partition_manifest.py")
SPEC = importlib.util.spec_from_file_location(
    "trnm_world_partition_manifest", MODULE_PATH
)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load partition-manifest validator: {MODULE_PATH}")
VALIDATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VALIDATOR)

MANIFEST = Path(
    "trillionnium/crates/trnm-game-server/src/lib_parts/manifest.json"
)


class PartitionManifestTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.src = self.root / "trillionnium/crates/trnm-game-server/src"
        (self.src / "lib_parts/alpha").mkdir(parents=True)
        (self.src / "lib_parts/tests").mkdir(parents=True)
        (self.src / "lib_parts/alpha/part_01.rs").write_text(
            "pub fn alpha() -> u8 { 1 }\n", encoding="utf-8"
        )
        (self.src / "lib_parts/tests/part_01.rs").write_text(
            'use super::*;\n'
            'include!(concat!(env!("CARGO_MANIFEST_DIR"), '
            '"/src/lib_parts/tests/nested_01.rs"));\n',
            encoding="utf-8",
        )
        (self.src / "lib_parts/tests/nested_01.rs").write_text(
            "#[test]\nfn nested() {}\n", encoding="utf-8"
        )
        (self.src / "lib.rs").write_text(
            'include!("lib_parts/alpha/part_01.rs");\n'
            '#[cfg(test)]\n'
            '#[path = "lib_parts/tests/part_01.rs"]\n'
            'mod tests;\n',
            encoding="utf-8",
        )
        self.write_manifest()

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def entry(self, relative: str, section: str) -> dict[str, object]:
        data = (self.src / relative).read_bytes()
        return {
            "bytes": len(data),
            "path": relative,
            "section": section,
            "sha256": hashlib.sha256(data).hexdigest(),
        }

    def manifest_data(self) -> dict[str, object]:
        return {
            "crate": "trnm-game-server",
            "max_impl_bytes": 32_000,
            "max_part_bytes": 60_000,
            "nested_test_parts": [
                self.entry("lib_parts/tests/nested_01.rs", "tests")
            ],
            "parts": [
                self.entry("lib_parts/alpha/part_01.rs", "alpha"),
                self.entry("lib_parts/tests/part_01.rs", "tests"),
            ],
            "schema": "trnm_semantic_direct_source_partition_v1",
            "sections": ["alpha", "tests"],
            "semantic_generation": False,
            "textual_include_reason": "preserve one reviewed crate-root API",
        }

    def write_manifest(self, mutate=None) -> None:
        data = self.manifest_data()
        if mutate is not None:
            mutate(data)
        target = self.root / MANIFEST
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(
            json.dumps(data, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )

    def assert_rejected(self, mutate) -> None:
        self.write_manifest(mutate)
        with self.assertRaises(VALIDATOR.PartitionManifestFailure):
            VALIDATOR.validate_all(self.root)

    def test_valid_manifest_binds_entrypoint_and_nested_order(self) -> None:
        self.assertEqual(VALIDATOR.validate_all(self.root), 1)

    def test_missing_required_manifest_fails_closed(self) -> None:
        (self.root / MANIFEST).unlink()
        with self.assertRaises(VALIDATOR.PartitionManifestFailure):
            VALIDATOR.validate_all(self.root)

    def test_digest_drift_fails_closed(self) -> None:
        self.assert_rejected(
            lambda data: data["parts"][0].__setitem__("sha256", "0" * 64)
        )

    def test_byte_count_drift_fails_closed(self) -> None:
        self.assert_rejected(
            lambda data: data["parts"][0].__setitem__(
                "bytes", data["parts"][0]["bytes"] + 1
            )
        )

    def test_entrypoint_order_drift_fails_closed(self) -> None:
        self.assert_rejected(lambda data: data["parts"].reverse())

    def test_nested_partition_order_drift_fails_closed(self) -> None:
        (self.src / "lib_parts/tests/nested_02.rs").write_text(
            "#[test]\nfn nested_two() {}\n", encoding="utf-8"
        )

        def mutate(data):
            data["nested_test_parts"].append(
                self.entry("lib_parts/tests/nested_02.rs", "tests")
            )

        self.assert_rejected(mutate)

    def test_semantic_generation_fails_closed(self) -> None:
        self.assert_rejected(
            lambda data: data.__setitem__("semantic_generation", True)
        )


if __name__ == "__main__":
    unittest.main()
