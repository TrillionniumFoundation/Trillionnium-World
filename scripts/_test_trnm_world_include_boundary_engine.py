#!/usr/bin/env python3
"""Hostile fixtures for the World include-boundary classifier."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

CHECKER = Path(__file__).with_name("check-trnm-world-include-boundaries.py")
SPEC = importlib.util.spec_from_file_location("trnm_world_include_boundaries", CHECKER)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def inventory(entries: list[dict[str, str]]) -> dict[str, object]:
    return {
        "schema": "trnm_world_include_boundary_inventory_v1",
        "as_of": "2026-09-09",
        "baseline_commit": "0" * 40,
        "policy": {
            "new_unclassified_include": "reject",
            "handwritten_runtime": "migrate_to_explicit_modules",
            "test_only": "migrate_to_nested_test_modules",
            "excluded_legacy": "frozen_no_growth",
            "generated_vendored_assets": "classify_before_admission",
        },
        "entries": entries,
    }


def entry(
    path: str = "crate/src/lib.rs",
    expression: str = '"body.rs"',
    include_class: str = "handwritten-runtime",
    state: str = "migrate",
) -> dict[str, str]:
    return {
        "path": path,
        "expression": expression,
        "class": include_class,
        "migration_state": state,
        "owner": "fixture-owner",
        "reason": "hostile fixture classification",
    }


class IncludeBoundaryTests(unittest.TestCase):
    def root(self) -> Path:
        temp = Path(tempfile.mkdtemp(prefix="trnm-include-boundary-"))
        (temp / "crate/src").mkdir(parents=True)
        (temp / "crate/Cargo.toml").write_text(
            '[package]\nname="fixture"\nversion="0.0.0"\nedition="2021"\n',
            encoding="utf-8",
        )
        (temp / "crate/src/body.rs").write_text("pub const VALUE: u8 = 1;\n", encoding="utf-8")
        (temp / "scripts/contracts").mkdir(parents=True)
        return temp

    def write_inventory(self, root: Path, value: dict[str, object]) -> None:
        (root / MODULE.INVENTORY).write_text(
            json.dumps(value, sort_keys=True), encoding="utf-8"
        )

    def test_exact_classified_literal_include_passes(self) -> None:
        root = self.root()
        (root / "crate/src/lib.rs").write_text('include!("body.rs");\n', encoding="utf-8")
        self.write_inventory(root, inventory([entry()]))
        self.assertEqual(MODULE.validate(root), (1, 0))

    def test_unclassified_include_fails_closed(self) -> None:
        root = self.root()
        (root / "crate/src/lib.rs").write_text(
            'include!("body.rs");\ninclude!("other.rs");\n', encoding="utf-8"
        )
        (root / "crate/src/other.rs").write_text("", encoding="utf-8")
        self.write_inventory(root, inventory([entry()]))
        with self.assertRaisesRegex(MODULE.IncludeBoundaryFailure, "unclassified"):
            MODULE.validate(root)

    def test_stale_inventory_entry_fails(self) -> None:
        root = self.root()
        (root / "crate/src/lib.rs").write_text("pub const VALUE: u8 = 1;\n", encoding="utf-8")
        self.write_inventory(root, inventory([entry()]))
        with self.assertRaisesRegex(MODULE.IncludeBoundaryFailure, "missing"):
            MODULE.validate(root)

    def test_include_inside_string_or_comment_is_not_code(self) -> None:
        root = self.root()
        (root / "crate/src/lib.rs").write_text(
            'const TEXT: &str = "include!(\\"body.rs\\");";\n'
            '// include!("body.rs");\n'
            '/* include!("body.rs"); */\n',
            encoding="utf-8",
        )
        self.write_inventory(root, inventory([]))
        self.assertEqual(MODULE.validate(root), (0, 0))

    def test_multiline_or_embedded_include_is_rejected(self) -> None:
        for source in (
            'include!(\n    "body.rs"\n);\n',
            'const A: u8 = 1; include!("body.rs");\n',
        ):
            with self.subTest(source=source):
                root = self.root()
                (root / "crate/src/lib.rs").write_text(source, encoding="utf-8")
                self.write_inventory(root, inventory([entry()]))
                with self.assertRaisesRegex(
                    MODULE.IncludeBoundaryFailure, "one-line standalone"
                ):
                    MODULE.validate(root)

    def test_traversal_and_missing_targets_are_rejected(self) -> None:
        for expression in ('"../outside.rs"', '"missing.rs"'):
            with self.subTest(expression=expression):
                root = self.root()
                (root / "crate/src/lib.rs").write_text(
                    f"include!({expression});\n", encoding="utf-8"
                )
                self.write_inventory(root, inventory([entry(expression=expression)]))
                with self.assertRaises(MODULE.IncludeBoundaryFailure):
                    MODULE.validate(root)

    def test_manifest_relative_target_is_resolved_from_nearest_crate(self) -> None:
        root = self.root()
        (root / "crate/src/lib.rs").write_text(
            'include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/body.rs"));\n',
            encoding="utf-8",
        )
        expression = 'concat!(env!("CARGO_MANIFEST_DIR"),"/src/body.rs")'
        self.write_inventory(root, inventory([entry(expression=expression)]))
        self.assertEqual(MODULE.validate(root), (1, 0))

    def test_duplicate_keys_and_duplicate_entries_are_rejected(self) -> None:
        root = self.root()
        (root / "crate/src/lib.rs").write_text('include!("body.rs");\n', encoding="utf-8")
        duplicate_json = (
            '{"schema":"trnm_world_include_boundary_inventory_v1",'
            '"schema":"duplicate","as_of":"2026-09-09",'
            '"baseline_commit":"' + "0" * 40 + '","policy":{}, "entries":[]}'
        )
        (root / MODULE.INVENTORY).write_text(duplicate_json, encoding="utf-8")
        with self.assertRaisesRegex(MODULE.IncludeBoundaryFailure, "duplicate JSON key"):
            MODULE.validate(root)

        self.write_inventory(root, inventory([entry(), entry()]))
        with self.assertRaisesRegex(MODULE.IncludeBoundaryFailure, "duplicate include"):
            MODULE.validate(root)

    def test_legacy_and_test_classification_constraints_fail_closed(self) -> None:
        cases = [
            entry(include_class="excluded-legacy-runtime", state="frozen"),
            entry(include_class="handwritten-runtime", state="frozen"),
            entry(include_class="test-only", state="migrate"),
        ]
        for bad in cases:
            with self.subTest(bad=bad):
                root = self.root()
                (root / "crate/src/lib.rs").write_text('include!("body.rs");\n', encoding="utf-8")
                self.write_inventory(root, inventory([bad]))
                with self.assertRaises(MODULE.IncludeBoundaryFailure):
                    MODULE.validate(root)


if __name__ == "__main__":
    unittest.main()
