#!/usr/bin/env python3
"""Local Git fault tests; no hosted execution or publication is simulated as real."""
from __future__ import annotations

import hashlib
import importlib.util
import os
import sys
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

# Keep validation read-only even when Python bytecode caching is enabled.
sys.dont_write_bytecode = True

spec = importlib.util.spec_from_file_location("qualified_checkout", Path(__file__).with_name("check-trnm-world-qualified-checkout.py"))
assert spec is not None and spec.loader is not None
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


def blob(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()


class QualifiedCheckoutTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.environment = {**os.environ, "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": os.devnull,
                            "GIT_AUTHOR_NAME": "Checkout fixture", "GIT_AUTHOR_EMAIL": "fixture@invalid",
                            "GIT_COMMITTER_NAME": "Checkout fixture", "GIT_COMMITTER_EMAIL": "fixture@invalid"}
        for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_OBJECT_DIRECTORY",
                    "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_CONFIG_PARAMETERS", "GIT_CONFIG_COUNT"):
            self.environment.pop(key, None)
        self.run_git("init", "-q", "--object-format=sha1")
        self.run_git("config", "core.hooksPath", os.devnull)
        self.run_git("config", "commit.gpgSign", "false")
        self.run_git("remote", "add", "origin", f"https://github.com/{checker.REPOSITORY}.git")
        (self.root / "PROJECT_ID").write_text("trillionnium-world\n")
        (self.root / "src").mkdir()
        self.data = b"pub fn exact_source() {}\n"
        (self.root / "src/lib.rs").write_bytes(self.data)
        self.records = [{"path": "src/lib.rs", "sha": blob(self.data), "mode": "100644"},
                        {"path": "build.rs", "sha": None, "mode": "100644"}]
        self.commit()

    def run_git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.root), *args], env=self.environment, stderr=subprocess.PIPE)

    def commit(self):
        self.run_git("add", "-A")
        self.run_git("commit", "-q", "-m", "fixture only")
        self.head = self.run_git("rev-parse", "HEAD").decode().strip()

    def verify(self, expected=None):
        return checker.verify_checkout(self.root, self.records, expected)

    def test_matching_commit_and_worktree(self):
        result = self.verify(self.head)
        self.assertEqual(result["qualified_writes"], 1)
        self.assertEqual(result["qualified_deletions"], 1)
        self.assertEqual(result["checkout_commit"], self.head)
        for key in ("remote_branch_publication", "exact_head_ci", "independent_review"):
            self.assertEqual(result[key], "not_proven")
        self.assertEqual(result["production_authorization"], "not_granted")

    def test_wrong_expected_head(self):
        with self.assertRaises(checker.CheckoutFailure):
            self.verify("0" * 40)

    def test_uncommitted_correct_bytes_do_not_prove_committed_bytes(self):
        (self.root / "src/lib.rs").write_text("wrong committed implementation\n")
        self.commit()
        (self.root / "src/lib.rs").write_bytes(self.data)
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_dirty_worktree_is_rejected(self):
        (self.root / "src/lib.rs").write_bytes(self.data + b"// changed\n")
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_required_deletion_in_worktree(self):
        (self.root / "build.rs").write_text("fn main() {}\n")
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_required_deletion_still_committed(self):
        (self.root / "build.rs").write_text("fn main() {}\n")
        self.commit()
        (self.root / "build.rs").unlink()
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_missing_worktree_file(self):
        (self.root / "src/lib.rs").unlink()
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_unrelated_governance_overlay_is_preserved(self):
        (self.root / "review-policy.md").write_text("independent review required\n")
        self.commit()
        before = self.run_git("status", "--porcelain")
        self.verify()
        self.assertEqual(before, self.run_git("status", "--porcelain"))
        self.assertTrue((self.root / "review-policy.md").exists())

    def test_uncommitted_unrelated_overlay_is_not_promoted(self):
        (self.root / "review-policy.md").write_text("uncommitted\n")
        result = self.verify()
        self.assertEqual(result["remote_branch_publication"], "not_proven")
        self.assertIn(b"review-policy.md", self.run_git("status", "--porcelain"))

    def test_crossed_origin(self):
        self.run_git("remote", "set-url", "origin", "https://github.com/other/repository.git")
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_crossed_project(self):
        (self.root / "PROJECT_ID").write_text("trillionnium-chain\n")
        self.commit()
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_nested_directory_not_checkout_root(self):
        with self.assertRaises(checker.CheckoutFailure):
            checker.verify_checkout(self.root / "src", self.records)

    def test_duplicate_record(self):
        self.records.append(dict(self.records[0]))
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_empty_records(self):
        self.records.clear()
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_unsafe_record(self):
        self.records[0]["path"] = "../outside"
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_symlink_source(self):
        p = self.root / "src/lib.rs"
        p.unlink()
        other = self.root / "outside"
        other.write_bytes(self.data)
        p.symlink_to(other)
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_dangling_symlink_deletion(self):
        (self.root / "build.rs").symlink_to(self.root / "absent")
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_executable_mode_drift(self):
        (self.root / "src/lib.rs").chmod(0o755)
        with self.assertRaises(checker.CheckoutFailure):
            self.verify()

    def test_oversized_worktree(self):
        with patch.object(checker, "MAX_FILE_BYTES", 4):
            with self.assertRaises(checker.CheckoutFailure):
                self.verify()

    def test_head_moves_during_inspection(self):
        original = checker.git
        reads = 0
        def moving(root, *args):
            nonlocal reads
            if args == ("rev-parse", "--verify", "HEAD^{commit}"):
                reads += 1
                if reads == 2:
                    return b"0" * 40 + b"\n"
            return original(root, *args)
        with patch.object(checker, "git", side_effect=moving):
            with self.assertRaises(checker.CheckoutFailure):
                self.verify()

    def test_malformed_tree_record(self):
        for raw in (b"missing tab\0", b"100644 blob bad\tx\0", b"truncated", b"\xff\tx\0"):
            with self.subTest(raw=raw), self.assertRaises(checker.CheckoutFailure):
                checker.parse_tree(raw)

    def test_duplicate_tree_path(self):
        row = b"100644 blob " + b"a" * 40 + b"\tsrc/lib.rs\0"
        with self.assertRaises(checker.CheckoutFailure):
            checker.parse_tree(row + row)

    def test_environment_cannot_redirect_git_checkout(self):
        with patch.dict(os.environ, {"GIT_DIR": "/missing", "GIT_WORK_TREE": "/missing",
                                     "GIT_CONFIG_COUNT": "1", "GIT_CONFIG_KEY_0": "remote.origin.url",
                                     "GIT_CONFIG_VALUE_0": "https://github.com/other/repo.git"}):
            self.verify()


if __name__ == "__main__":
    unittest.main(verbosity=2)
