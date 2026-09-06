#!/usr/bin/env python3
"""Offline regressions of the reviewed workflow inventory, not hosted validation."""
from __future__ import annotations

import importlib.util
import pathlib
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
SCRIPT = pathlib.Path(__file__).with_name("check-trnm-world-ci-integrity.py")
ROOT = SCRIPT.resolve().parents[1]
spec = importlib.util.spec_from_file_location("ci_integrity", SCRIPT)
assert spec is not None and spec.loader is not None
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)
GAP = "trnm-world-gap-closure-v4.yml"
FINAL = "trnm-world-v4-final-gates.yml"


class WorkflowInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = pathlib.Path(self.temp.name)
        self.folder = self.root / ".github/workflows"
        shutil.copytree(ROOT / ".github/workflows", self.folder)

    def edit(self, name, before, after):
        path = self.folder / name
        source = path.read_text()
        self.assertIn(before, source)
        path.write_text(source.replace(before, after, 1))

    def reject(self):
        with self.assertRaises(SystemExit):
            checker.workflow_inventory(self.root)

    def test_reviewed_eight_workflows_have_twenty_unique_contexts(self):
        contexts = checker.workflow_inventory(self.root)
        self.assertEqual(len(contexts), 20)
        for job in ("docs-governance", "transition-contract", "settlement-postgres", "game-workspace-release", "supply-chain"):
            self.assertEqual(contexts["trnm-world-v4/" + job], GAP)
        self.assertEqual(contexts["trnm-world-v5/closure-contract"], "trnm-world-v5-closure-contract.yml")

    def test_removed_workflow_rejected(self):
        (self.folder / FINAL).unlink()
        self.reject()

    def test_extra_workflow_rejected(self):
        (self.folder / "unreviewed.yml").write_text("name: surprise\n")
        self.reject()

    def test_extra_yaml_extension_rejected(self):
        (self.folder / "unreviewed.yaml").write_text("name: surprise\n")
        self.reject()

    def test_linked_workflow_rejected(self):
        path = self.folder / FINAL
        copy = self.root / "outside.yml"
        path.rename(copy)
        path.symlink_to(copy)
        self.reject()

    def test_linked_workflow_directory_rejected(self):
        original = self.root / "outside"
        self.folder.rename(original)
        self.folder.symlink_to(original, target_is_directory=True)
        self.reject()

    def test_empty_oversized_and_non_utf8_workflow_rejected(self):
        path = self.folder / FINAL
        for content in (b"", b" \n", b" " * (256 * 1024 + 1), b"\xff"):
            path.write_bytes(content)
            with self.subTest(content=content[:4]):
                with self.assertRaises((SystemExit, UnicodeError)):
                    checker.workflow_inventory(self.root)

    def test_primary_context_cannot_move_to_narrower_job(self):
        self.edit(GAP, "name: trnm-world-v4/supply-chain", "name: trnm-world-v4-supplemental/supply-chain")
        self.edit(FINAL, "name: trnm-world-v4-supplemental/supply-chain", "name: trnm-world-v4/supply-chain")
        self.reject()

    def test_duplicate_primary_context_rejected(self):
        self.edit(FINAL, "name: trnm-world-v4-supplemental/supply-chain", "name: trnm-world-v4/supply-chain")
        self.reject()

    def test_duplicate_closure_context_rejected(self):
        self.edit(FINAL, "name: trnm-world-v5-supplemental/closure-contract", "name: trnm-world-v5/closure-contract")
        self.reject()

    def test_missing_job_rejected(self):
        self.edit(FINAL, "  supply-chain:\n", "  renamed-supply:\n")
        self.reject()

    def test_duplicate_job_identifier_rejected(self):
        path = self.folder / GAP
        path.write_text(path.read_text() + "\n  supply-chain:\n    name: trnm-world-v4/other\n")
        self.reject()

    def test_duplicate_jobs_mapping_rejected(self):
        path = self.folder / GAP
        path.write_text(path.read_text() + "\njobs:\n  other:\n    name: trnm-world-v4/other\n")
        self.reject()

    def test_missing_static_job_name_rejected(self):
        self.edit(GAP, "    name: trnm-world-v4/docs-governance\n", "")
        self.reject()

    def test_dynamic_name_rejected(self):
        self.edit(GAP, "name: trnm-world-v4/docs-governance", "name: ${{ matrix.context }}")
        self.reject()

    def test_quoted_or_commented_marker_cannot_supply_job_name(self):
        self.edit(GAP, "    name: trnm-world-v4/docs-governance", "    # name: trnm-world-v4/docs-governance")
        self.reject()

    def test_missing_read_permission_rejected(self):
        self.edit(GAP, "permissions:\n  contents: read\n", "")
        self.reject()

    def test_write_permissions_rejected(self):
        original = (self.folder / GAP).read_text()
        for value in ("contents: write", "contents:  write # unauthorized", "issues: write"):
            (self.folder / GAP).write_text(original.replace("contents: read", value))
            with self.subTest(value=value): self.reject()

    def test_job_permission_override_rejected(self):
        self.edit(GAP, "    runs-on: ubuntu-24.04", "    permissions:\n      contents: read\n    runs-on: ubuntu-24.04")
        self.reject()

    def test_mutable_action_rejected(self):
        self.edit(GAP, "actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683", "actions/checkout@v4")
        self.reject()

    def test_mutable_runner_rejected(self):
        self.edit(GAP, "runs-on: ubuntu-24.04", "runs-on: ubuntu-latest")
        self.reject()

    def test_privileged_trigger_rejected(self):
        self.edit(GAP, "  pull_request:", "  pull_request_target:")
        self.reject()

    def test_source_mutation_and_retained_credentials_rejected(self):
        original = (self.folder / GAP).read_text()
        for marker in ("git push", "git commit", "git tag", "gh pr merge", "update-ref", "clippy --fix", "persist-credentials: true"):
            (self.folder / GAP).write_text(original + "\n# forbidden: " + marker + "\n")
            with self.subTest(marker=marker): self.reject()

    def test_six_unmodified_workflow_names_are_required(self):
        for name in sorted(set(checker.WORKFLOW_JOBS) - {GAP, FINAL}):
            with self.subTest(name=name):
                data = (self.folder / name).read_bytes()
                (self.folder / name).unlink()
                self.reject()
                (self.folder / name).write_bytes(data)

    def test_cli_narrow_mode_does_not_claim_full_validation(self):
        result = subprocess.run([sys.executable, str(SCRIPT), "--root", str(self.root), "--workflows-only"],
                                capture_output=True, text=True, timeout=15)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("workflow inventory only", result.stdout)
        self.assertIn("no hosted/governance evidence", result.stdout)

    def test_default_mode_does_not_skip_missing_documentation(self):
        result = subprocess.run([sys.executable, str(SCRIPT), "--root", str(self.root)],
                                capture_output=True, text=True, timeout=15)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("required current/historical contract is missing", result.stderr)

    def test_child_failure_and_timeout_are_not_swallowed(self):
        for relative in checker.REQUIRED_DOCS:
            path = self.root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("fixture input, not real evidence\n")
        with patch.object(sys, "argv", [str(SCRIPT), "--root", str(self.root)]):
            for outcome in (subprocess.CompletedProcess([], 1, "", "fixture failed"), subprocess.TimeoutExpired("fixture", 180)):
                with self.subTest(outcome=type(outcome).__name__):
                    kwargs = {"side_effect": outcome} if isinstance(outcome, Exception) else {"return_value": outcome}
                    with patch.object(checker.subprocess, "run", **kwargs):
                        with self.assertRaises(SystemExit): checker.main()

    def test_inventory_check_does_not_write_source(self):
        before = {p.name: p.read_bytes() for p in self.folder.iterdir()}
        checker.workflow_inventory(self.root)
        self.assertEqual(before, {p.name: p.read_bytes() for p in self.folder.iterdir()})


if __name__ == "__main__":
    unittest.main(verbosity=2)
