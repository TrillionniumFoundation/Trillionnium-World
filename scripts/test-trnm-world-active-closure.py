#!/usr/bin/env python3
"""Negative fixtures for the active World closure truth contract."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "active_closure",
    Path(__file__).with_name("check-trnm-world-active-closure.py"),
)
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

ROOT = Path(__file__).resolve().parents[1]
FILES = (
    M.TOOLCHAIN,
    M.BOUNDARY,
    M.CATALOG,
    M.LEDGER,
    M.CEX_LOCK,
    M.PROTECTION,
    M.RELEASE,
    M.PLAN,
)


class ActiveClosureFixtures(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory(prefix="trnm-world-active-closure-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for relative in FILES:
            source = ROOT / relative
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)

    def expect_failure(self) -> None:
        with self.assertRaises((M.ActiveClosureFailure, OSError, ValueError)):
            M.validate(self.root)

    def load_json(self, relative: str) -> dict:
        return json.loads((self.root / relative).read_text(encoding="utf-8"))

    def write_json(self, relative: str, value: dict) -> None:
        (self.root / relative).write_text(
            json.dumps(value, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )

    def test_current_fixture_passes(self) -> None:
        M.validate(self.root)

    def test_toolchain_downgrade_fails(self) -> None:
        path = self.root / M.TOOLCHAIN
        path.write_text(path.read_text().replace('channel = "1.98.1"', 'channel = "1.98.0"'))
        self.expect_failure()

    def test_catalog_omits_release_truth_fails(self) -> None:
        catalog = self.load_json(M.CATALOG)
        catalog["truth_order"].remove(M.RELEASE)
        self.write_json(M.CATALOG, catalog)
        self.expect_failure()

    def test_duplicate_json_key_fails(self) -> None:
        path = self.root / M.CEX_LOCK
        text = path.read_text(encoding="utf-8")
        path.write_text(text.replace('"schema":', '"schema": "duplicate",\n  "schema":', 1), encoding="utf-8")
        self.expect_failure()

    def test_non_finite_json_fails(self) -> None:
        path = self.root / M.LEDGER
        text = path.read_text(encoding="utf-8")
        path.write_text(text.replace('"generated_utc":', '"invalid": NaN,\n  "generated_utc":', 1), encoding="utf-8")
        self.expect_failure()

    def test_stale_cex_pr_fails(self) -> None:
        lock = self.load_json(M.CEX_LOCK)
        lock["cex"]["pull_request"] = 24
        self.write_json(M.CEX_LOCK, lock)
        self.expect_failure()

    def test_cex_qualification_overclaim_fails(self) -> None:
        lock = self.load_json(M.CEX_LOCK)
        lock["qualified"] = True
        self.write_json(M.CEX_LOCK, lock)
        self.expect_failure()

    def test_repository_ci_overclaim_fails(self) -> None:
        ledger = self.load_json(M.LEDGER)
        ledger["closure"]["exact_head_ci_closed"] = True
        self.write_json(M.LEDGER, ledger)
        self.expect_failure()

    def test_main_context_mismatch_cannot_be_hidden(self) -> None:
        ledger = self.load_json(M.LEDGER)
        ledger["repository_evidence"]["desired_contract_matches_observation"] = True
        self.write_json(M.LEDGER, ledger)
        self.expect_failure()

    def test_release_truth_overclaim_fails(self) -> None:
        path = self.root / M.RELEASE
        path.write_text(path.read_text().replace("production_ready=false", "production_ready=true"))
        self.expect_failure()

    def test_stale_world_pr_fails(self) -> None:
        path = self.root / M.PLAN
        path.write_text(path.read_text() + "\nPull request: `#46`\n")
        self.expect_failure()


if __name__ == "__main__":
    M.validate(ROOT)
    result = unittest.TextTestRunner(verbosity=1).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(ActiveClosureFixtures)
    )
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World active closure negative fixtures: PASS")
