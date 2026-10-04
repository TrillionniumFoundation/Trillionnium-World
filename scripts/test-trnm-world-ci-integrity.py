#!/usr/bin/env python3
"""Offline negative fixtures for the complete reviewed World workflow inventory."""
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
MODULES = "trnm-world-module-documentation.yml"
NATIVE = "world-pr-native-admission-v1.yml"
V5 = "trnm-world-v5-closure-contract.yml"
EXPECTED_WORKFLOWS = 15
EXPECTED_CONTEXTS = 49


class WorkflowInventoryTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = pathlib.Path(self.temp.name)
        self.folder = self.root / ".github/workflows"
        shutil.copytree(ROOT / ".github/workflows", self.folder)

    def edit(self, name: str, before: str, after: str) -> None:
        path = self.folder / name
        source = path.read_text(encoding="utf-8")
        self.assertIn(before, source)
        path.write_text(source.replace(before, after, 1), encoding="utf-8")

    def reject(self) -> None:
        with self.assertRaises(SystemExit):
            checker.workflow_inventory(self.root)

    def test_complete_inventory_has_unique_static_and_matrix_contexts(self) -> None:
        contexts = checker.workflow_inventory(self.root)
        self.assertEqual(len(checker.WORKFLOW_JOBS) + len(checker.MATRIX_WORKFLOWS), EXPECTED_WORKFLOWS)
        self.assertEqual(len(contexts), EXPECTED_CONTEXTS)
        self.assertEqual(contexts["trnm-game-ci"], NATIVE)
        self.assertEqual(contexts["trnm-world-p0-boundaries"], NATIVE)
        self.assertEqual(contexts["trnm-world-status-evidence"], NATIVE)
        self.assertEqual(contexts["trnm-world-v12/required"], NATIVE)
        self.assertEqual(contexts["trnm-world/docs-complete"], MODULES)

    def test_rust_199_matrix_covers_all_six_roots_and_portable_contract(self) -> None:
        text = (self.folder / "trnm-world-rust-199.yml").read_text()
        for manifest in (
            "contracts/Cargo.toml",
            "trillionnium/Cargo.toml",
            "trillionnium/crates/platform/Cargo.toml",
            "trillionnium/crates/world-authority/Cargo.toml",
            "trillionnium/contracts/trnm-world-transition-v1/Cargo.toml",
            "trillionnium/tools/trnm-settlement-outbox-contract/Cargo.toml",
        ):
            self.assertIn(f"manifest: {manifest}\n", text)
        self.assertIn("commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4", text)
        self.assertIn("cargo test --manifest-path \"$MANIFEST\" --workspace --all-targets --locked", text)
        self.assertIn("cargo clippy --manifest-path \"$MANIFEST\" --workspace --all-targets --locked -- -D warnings", text)
        self.assertIn('test "$(git -C "$source" rev-parse HEAD^2)" = "$HEAD_SHA"', text)
        for target in ("wasm32-unknown-unknown", "aarch64-linux-android",
                       "armv7-linux-androideabi", "x86_64-linux-android", "i686-linux-android"):
            self.assertIn(target, text)

    def test_rust_199_requires_pr_and_full_raw_admission_before_and_after(self) -> None:
        text = (self.folder / "trnm-world-rust-199.yml").read_text()
        triggers = text.split("\npermissions:", 1)[0]
        self.assertIn("  pull_request:", triggers)
        self.assertNotIn("  push:", triggers)
        self.assertNotIn("  workflow_dispatch:", triggers)
        self.assertIn("labels=(head merge)", text)
        admission = 'python3 -B scripts/check-trnm-world-ci-target.py --root "$source" --role "$label"'
        self.assertEqual(text.count(admission), 2)
        self.assertIn("$label-admission-before.json", text)
        self.assertIn("$label-admission-after.json", text)
        self.assertIn("trap verify_after EXIT", text)
        self.assertIn('|| status=1', text)
        required = text.split("\n  required:", 1)[1]
        self.assertIn('test "$GITHUB_EVENT_NAME" = pull_request', required)
        self.assertIn('test "$RESULT" = success', required)

    def test_rust_199_missing_lane_is_rejected(self) -> None:
        path = self.folder / "trnm-world-rust-199.yml"
        path.write_text(path.read_text().replace("context: trnm-world-rust-199/contracts", "context: omitted"))
        with self.assertRaises(SystemExit):
            checker.workflow_inventory(self.root)

    def test_removed_and_extra_workflows_are_rejected(self) -> None:
        (self.folder / V5).unlink()
        self.reject()
        shutil.copy2(ROOT / ".github/workflows" / V5, self.folder / V5)
        (self.folder / "unreviewed.yml").write_text("name: surprise\n", encoding="utf-8")
        self.reject()

    def test_yaml_extension_extra_is_rejected(self) -> None:
        (self.folder / "unreviewed.yaml").write_text("name: surprise\n", encoding="utf-8")
        self.reject()

    def test_linked_file_and_directory_are_rejected(self) -> None:
        source = self.folder / V5
        outside = self.root / "outside.yml"
        source.rename(outside)
        source.symlink_to(outside)
        self.reject()

        source.unlink()
        shutil.copy2(ROOT / ".github/workflows" / V5, source)
        original = self.root / "outside-workflows"
        self.folder.rename(original)
        self.folder.symlink_to(original, target_is_directory=True)
        self.reject()

    def test_empty_oversized_and_non_utf8_workflows_are_rejected(self) -> None:
        path = self.folder / V5
        original = path.read_bytes()
        for content in (b"", b" \n", b" " * (256 * 1024 + 1), b"\xff"):
            path.write_bytes(content)
            with self.subTest(prefix=content[:4]):
                self.reject()
        path.write_bytes(original)

    def test_static_context_and_job_identifier_drift_are_rejected(self) -> None:
        self.edit(
            MODULES,
            "name: trnm-world/docs-complete",
            "name: trnm-world-v5/closure-contract",
        )
        self.reject()

        shutil.rmtree(self.folder)
        shutil.copytree(ROOT / ".github/workflows", self.folder)
        self.edit(MODULES, "  complete-documentation:\n", "  renamed-documentation:\n")
        self.reject()

    def test_ordinary_workflow_dynamic_name_is_rejected(self) -> None:
        self.edit(
            GAP,
            "name: trnm-world-v4/docs-governance",
            "name: ${{ matrix.context }}",
        )
        self.reject()

    def test_matrix_context_name_and_required_dependency_drift_are_rejected(self) -> None:
        self.edit(NATIVE, "context: trnm-game-ci", "context: trnm-game-ci-drift")
        self.reject()

        shutil.rmtree(self.folder)
        shutil.copytree(ROOT / ".github/workflows", self.folder)
        self.edit(NATIVE, "name: ${{ matrix.context }}", "name: trnm-game-ci")
        self.reject()

        shutil.rmtree(self.folder)
        shutil.copytree(ROOT / ".github/workflows", self.folder)
        self.edit(NATIVE, "needs: [qualification]", "needs: []")
        self.reject()

    def test_missing_read_permission_and_any_write_permission_are_rejected(self) -> None:
        self.edit(GAP, "permissions:\n  contents: read\n", "")
        self.reject()

        for value in ("contents: write", "issues: write", "statuses: write"):
            shutil.rmtree(self.folder)
            shutil.copytree(ROOT / ".github/workflows", self.folder)
            self.edit(GAP, "contents: read", value)
            with self.subTest(value=value):
                self.reject()

    def test_job_level_permissions_are_rejected(self) -> None:
        self.edit(
            MODULES,
            "    runs-on: ubuntu-24.04",
            "    permissions:\n      contents: read\n    runs-on: ubuntu-24.04",
        )
        self.reject()

    def test_mutable_action_runner_and_privileged_trigger_are_rejected(self) -> None:
        self.edit(
            NATIVE,
            "actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02",
            "actions/upload-artifact@v4",
        )
        self.reject()

        shutil.rmtree(self.folder)
        shutil.copytree(ROOT / ".github/workflows", self.folder)
        self.edit(NATIVE, "runs-on: ubuntu-24.04", "runs-on: ubuntu-latest")
        self.reject()

        shutil.rmtree(self.folder)
        shutil.copytree(ROOT / ".github/workflows", self.folder)
        self.edit(NATIVE, "  pull_request:", "  pull_request_target:")
        self.reject()

    def test_retained_credentials_and_source_mutation_are_rejected(self) -> None:
        original = (self.folder / NATIVE).read_text(encoding="utf-8")
        for marker in (
            "persist-credentials: true",
            "git push",
            "git commit candidate",
            "git tag release",
            "gh pr merge 1",
            "git update-ref refs/heads/main HEAD",
            "cargo clippy --fix",
        ):
            injected = marker if marker.startswith("persist-credentials:") else "# forbidden: " + marker
            (self.folder / NATIVE).write_text(original + "\n" + injected + "\n", encoding="utf-8")
            with self.subTest(marker=marker):
                self.reject()

    def test_commit_tree_is_allowed_because_it_does_not_move_a_ref(self) -> None:
        path = self.folder / GAP
        source = path.read_text(encoding="utf-8")
        path.write_text(source + "\n# allowed token: git commit-tree\n", encoding="utf-8")
        contexts = checker.workflow_inventory(self.root)
        self.assertEqual(len(contexts), EXPECTED_CONTEXTS)

    def test_cli_workflows_only_is_explicitly_non_hosted(self) -> None:
        result = subprocess.run(
            [sys.executable, str(SCRIPT), "--root", str(self.root), "--workflows-only"],
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(f"{EXPECTED_CONTEXTS} unique contexts", result.stdout)
        self.assertIn("workflow inventory only", result.stdout)
        self.assertIn("no hosted/governance evidence", result.stdout)

    def test_default_mode_rejects_missing_documentation(self) -> None:
        result = subprocess.run(
            [sys.executable, str(SCRIPT), "--root", str(self.root)],
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("required current/historical contract is missing", result.stderr)

    def test_child_failure_and_timeout_are_not_swallowed(self) -> None:
        for relative in checker.REQUIRED_DOCS:
            path = self.root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("fixture input, not real evidence\n", encoding="utf-8")
        with patch.object(sys, "argv", [str(SCRIPT), "--root", str(self.root)]):
            outcomes = (
                subprocess.CompletedProcess([], 1, "", "fixture failed"),
                subprocess.TimeoutExpired("fixture", 180),
            )
            for outcome in outcomes:
                with self.subTest(outcome=type(outcome).__name__):
                    kwargs = {"side_effect": outcome} if isinstance(outcome, Exception) else {"return_value": outcome}
                    with patch.object(checker.subprocess, "run", **kwargs):
                        with self.assertRaises(SystemExit):
                            checker.main()

    def test_inventory_check_does_not_write_source(self) -> None:
        before = {path.name: path.read_bytes() for path in self.folder.iterdir()}
        checker.workflow_inventory(self.root)
        after = {path.name: path.read_bytes() for path in self.folder.iterdir()}
        self.assertEqual(before, after)


if __name__ == "__main__":
    unittest.main(verbosity=2)
