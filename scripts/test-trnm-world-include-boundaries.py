#!/usr/bin/env python3
"""Hostile fixtures for the World include-boundary engine and migration ledger."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SCRIPT_DIR = Path(__file__).resolve().parent
CHECKER = SCRIPT_DIR / "check-trnm-world-include-boundaries.py"
ENGINE_TESTS_PATH = SCRIPT_DIR / "_test_trnm_world_include_boundary_engine.py"

CHECKER_SPEC = importlib.util.spec_from_file_location(
    "trnm_world_include_boundaries", CHECKER
)
assert CHECKER_SPEC is not None and CHECKER_SPEC.loader is not None
CHECKER_MODULE = importlib.util.module_from_spec(CHECKER_SPEC)
CHECKER_SPEC.loader.exec_module(CHECKER_MODULE)

ENGINE_TESTS_SPEC = importlib.util.spec_from_file_location(
    "trnm_world_include_boundary_engine_tests", ENGINE_TESTS_PATH
)
assert ENGINE_TESTS_SPEC is not None and ENGINE_TESTS_SPEC.loader is not None
ENGINE_TESTS = importlib.util.module_from_spec(ENGINE_TESTS_SPEC)
ENGINE_TESTS_SPEC.loader.exec_module(ENGINE_TESTS)


def inventory(entries: list[dict[str, str]]) -> dict[str, object]:
    return {
        "schema": "trnm_world_include_boundary_inventory_v1",
        "as_of": "2026-09-10",
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
) -> dict[str, str]:
    return {
        "path": path,
        "expression": expression,
        "class": "handwritten-runtime",
        "migration_state": "migrate",
        "owner": "fixture-owner",
        "reason": "fixture ownership seam",
    }


def ledger(
    completed: list[dict[str, object]],
    baseline: str = "0" * 40,
) -> dict[str, object]:
    return {
        "schema": "trnm_world_include_migration_ledger_v1",
        "as_of": "2026-09-10",
        "inventory_baseline_commit": baseline,
        "completed": completed,
    }


def completed_entry(
    path: str = "crate/src/lib.rs",
    expression: str = '"body.rs"',
    fragments: list[str] | None = None,
) -> dict[str, object]:
    return {
        "path": path,
        "expression": expression,
        "replacement_module": "body",
        "required_fragments": fragments
        or ['#[path = "body.rs"]', "mod body;", "pub use body::*;"],
        "completed_commit": "1" * 40,
    }


class IncludeMigrationLedgerTests(unittest.TestCase):
    def root(self) -> Path:
        root = Path(tempfile.mkdtemp(prefix="trnm-include-migration-"))
        (root / "crate/src").mkdir(parents=True)
        (root / "crate/Cargo.toml").write_text(
            '[package]\nname="fixture"\nversion="0.0.0"\nedition="2021"\n',
            encoding="utf-8",
        )
        (root / "crate/src/body.rs").write_text(
            "pub const VALUE: u8 = 1;\n", encoding="utf-8"
        )
        (root / "scripts/contracts").mkdir(parents=True)
        return root

    def write_json(self, root: Path, relative: str, value: object) -> None:
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(
            json.dumps(value, ensure_ascii=False, sort_keys=True) + "\n",
            encoding="utf-8",
        )

    def prepare_completed(self) -> Path:
        root = self.root()
        (root / "crate/src/lib.rs").write_text(
            '#[path = "body.rs"]\nmod body;\npub use body::*;\n',
            encoding="utf-8",
        )
        self.write_json(root, CHECKER_MODULE.INVENTORY, inventory([entry()]))
        self.write_json(
            root,
            CHECKER_MODULE.MIGRATIONS,
            ledger([completed_entry()]),
        )
        return root

    def test_completed_migration_passes_and_is_removed_from_active_denominator(self) -> None:
        root = self.prepare_completed()
        self.assertEqual(CHECKER_MODULE.validate(root), (0, 0))
        self.assertEqual(
            CHECKER_MODULE._validate_with_progress(root),
            (0, 0, 1),
        )

    def test_completed_include_may_not_reappear(self) -> None:
        root = self.prepare_completed()
        (root / "crate/src/lib.rs").write_text(
            'include!("body.rs");\n'
            '#[path = "body.rs"]\nmod body;\npub use body::*;\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure, "reappeared"
        ):
            CHECKER_MODULE.validate(root)

    def test_replacement_fragments_are_required(self) -> None:
        root = self.prepare_completed()
        (root / "crate/src/lib.rs").write_text(
            '#[path = "body.rs"]\nmod body;\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure, "replacement fragment missing"
        ):
            CHECKER_MODULE.validate(root)

    def test_completed_entry_must_exist_in_bound_inventory(self) -> None:
        root = self.prepare_completed()
        self.write_json(root, CHECKER_MODULE.INVENTORY, inventory([]))
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure, "absent from baseline inventory"
        ):
            CHECKER_MODULE.validate(root)

    def test_inventory_baseline_binding_is_exact(self) -> None:
        root = self.prepare_completed()
        self.write_json(
            root,
            CHECKER_MODULE.MIGRATIONS,
            ledger([completed_entry()], baseline="2" * 40),
        )
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure, "not bound"
        ):
            CHECKER_MODULE.validate(root)

    def test_duplicate_completed_entries_are_rejected(self) -> None:
        root = self.prepare_completed()
        item = completed_entry()
        self.write_json(
            root,
            CHECKER_MODULE.MIGRATIONS,
            ledger([item, dict(item)]),
        )
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure, "duplicate completed"
        ):
            CHECKER_MODULE.validate(root)

    def test_duplicate_ledger_json_keys_are_rejected(self) -> None:
        root = self.prepare_completed()
        target = root / CHECKER_MODULE.MIGRATIONS
        target.write_text(
            '{"schema":"trnm_world_include_migration_ledger_v1",'
            '"schema":"duplicate","as_of":"2026-09-10",'
            '"inventory_baseline_commit":"' + "0" * 40 + '","completed":[]}\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure,
            "duplicate migration-ledger JSON key",
        ):
            CHECKER_MODULE.validate(root)

    def test_unclassified_noncompleted_include_still_fails_closed(self) -> None:
        root = self.prepare_completed()
        (root / "crate/src/other.rs").write_text("", encoding="utf-8")
        (root / "crate/src/lib.rs").write_text(
            '#[path = "body.rs"]\nmod body;\npub use body::*;\n'
            'include!("other.rs");\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(
            CHECKER_MODULE.IncludeBoundaryFailure, "unclassified"
        ):
            CHECKER_MODULE.validate(root)


def main() -> int:
    suite = unittest.TestSuite()
    suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(ENGINE_TESTS))
    suite.addTests(
        unittest.defaultTestLoader.loadTestsFromTestCase(
            IncludeMigrationLedgerTests
        )
    )
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
