#!/usr/bin/env python3
"""Offline fault tests for source publication. These are not hosted CI evidence."""
from __future__ import annotations
import base64
import copy
import importlib.util
import io
import json
import os
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import urllib.error
import zipfile

SPEC = importlib.util.spec_from_file_location("world_importer", Path(__file__).with_name("import-qualified-world-v13k.py"))
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)
H, C, T, N = "a" * 40, "b" * 40, "c" * 40, "d" * 40
OLD, DOC = ("100644", "blob", "e" * 40), ("100644", "blob", "f" * 40)
RECORDS = [{"path": "src/a.rs", "content": b"new\n", "mode": "100644", "sha": M.blob_sha(b"new\n")},
           {"path": "legacy.in", "content": None, "mode": "100644", "sha": None}]
BASE = {"src/a.rs": OLD, "legacy.in": OLD}
CURRENT = {**BASE, "docs/README.md": DOC}


class FakeGitHub:
    def __init__(self):
        self.calls = []
        self.current = dict(CURRENT)
        self.head = H
        self.branch_reads = 0
        self.race = False
        self.bad_blob = False
        self.bad_qualified = False
        self.bad_overlay = False
        self.bad_parent = False
        self.bad_final_ref = False
        self.overlay_entries = None

    def tree(self, sha, leaves):
        return {"sha": sha, "truncated": False, "tree": [
            {"path": p, "mode": v[0], "type": v[1], "sha": v[2]} for p, v in leaves.items()]}

    def __call__(self, method, path, token, payload=None):
        self.calls.append((method, path, copy.deepcopy(payload)))
        if method == "GET" and "/branches/" in path:
            self.branch_reads += 1
            if self.race and self.branch_reads == 2:
                return {"commit": {"sha": "1" * 40}}
            return {"commit": {"sha": self.head}}
        if method == "GET" and path.endswith("/git/commits/" + H):
            return {"tree": {"sha": C}}
        if method == "GET" and path.endswith("/git/commits/" + N):
            return {"tree": {"sha": T}, "parents": [{"sha": "2" * 40 if self.bad_parent else H}]}
        if method == "GET" and "/git/trees/" in path:
            sha = path.split("/git/trees/")[1].split("?")[0]
            if sha == M.EXPECTED["source_tree"]:
                return self.tree(sha, BASE)
            if sha == C:
                return self.tree(sha, self.current)
            if sha == T:
                wanted = M.overlay_expected(BASE, self.current, RECORDS)
                if self.bad_overlay:
                    wanted.pop("docs/README.md")
                return self.tree(sha, wanted)
        if method == "POST" and path.endswith("/git/blobs"):
            return {"sha": "0" * 40 if self.bad_blob else M.blob_sha(base64.b64decode(payload["content"]))}
        if method == "POST" and path.endswith("/git/trees"):
            if payload["base_tree"] == M.EXPECTED["source_tree"]:
                return {"sha": "0" * 40 if self.bad_qualified else M.EXPECTED["qualified_tree"]}
            self.overlay_entries = payload["tree"]
            return {"sha": T}
        if method == "POST" and path.endswith("/git/commits"):
            assert payload["parents"] == [H]
            assert payload["tree"] == T
            return {"sha": N}
        if method == "PATCH":
            assert path.endswith("/git/refs/heads/" + M.DEFAULT_BRANCH)
            assert payload == {"sha": N, "force": False}
            self.head = H if self.bad_final_ref else N
            return {}
        raise AssertionError((method, path, payload))


class ImportTests(unittest.TestCase):
    def setUp(self):
        self.env = patch.dict(os.environ, {"CI": "", "GITHUB_ACTIONS": ""})
        self.env.start()
        self.addCleanup(self.env.stop)

    def run_publication(self, fake):
        with patch.object(M, "github", fake), patch.object(M, "WRITE_COUNT", 1), \
                patch.object(M, "DELETION_COUNT", 1), patch.object(M, "DELETIONS", {"legacy.in"}):
            return M.publish(M.REPOSITORY, M.DEFAULT_BRANCH, H, "unit-fake-token", RECORDS)

    def rejected_before_ref(self, flag):
        fake = FakeGitHub()
        setattr(fake, flag, True)
        with self.assertRaises(M.ImportFailure):
            self.run_publication(fake)
        self.assertFalse(any(call[0] == "PATCH" for call in fake.calls))

    def test_immutable_identity_and_counts(self):
        self.assertEqual(M.WRITE_COUNT, 73)
        self.assertEqual(M.DELETION_COUNT, 2)
        self.assertEqual(M.EXPECTED["qualified_tree"], "5e613185f5a2abda42df371f3755e73667717309")
        self.assertEqual(M.EXPECTED["artifact_zip_sha256"], "456a181bdc8f8aa248229b044db9eec4f52572ea3b20bca6492907db58d64ef5")

    def test_empty_records_cannot_claim_already_present(self):
        with patch.object(M, "github") as network, self.assertRaises(M.ImportFailure):
            M.publish(M.REPOSITORY, M.DEFAULT_BRANCH, H, "unit-fake-token", [])
        network.assert_not_called()

    def test_altered_bytes_rejected_before_network(self):
        records = copy.deepcopy(RECORDS)
        records[0]["content"] = b"altered"
        with patch.object(M, "WRITE_COUNT", 1), patch.object(M, "DELETION_COUNT", 1), \
                patch.object(M, "DELETIONS", {"legacy.in"}), patch.object(M, "github") as network, \
                self.assertRaises(M.ImportFailure):
            M.publish(M.REPOSITORY, M.DEFAULT_BRANCH, H, "unit-fake-token", records)
        network.assert_not_called()

    def test_safe_paths(self):
        for value in ("../escape", "/abs", "./x", "x//y", "x/../y", "x\\y", ".git/config", ".", "./", "C:/x", ""):
            with self.subTest(value=value), self.assertRaises(M.ImportFailure):
                M.checked_relative(value, "test")
        self.assertEqual(M.checked_relative("src/parts/", "test"), "src/parts/")

    def test_duplicate_archive_members(self):
        with self.assertRaises(M.ImportFailure):
            M.check_members([("same", 1), ("same", 1)])

    def test_archive_budget(self):
        with self.assertRaises(M.ImportFailure):
            M.check_members([("large", M.MAX_MEMBER_BYTES + 1)])

    def test_zip_path_traversal(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            with zipfile.ZipFile(root / "bad.zip", "w") as z:
                z.writestr("../escape", b"x")
            with self.assertRaises(M.ImportFailure):
                M.safe_extract_zip(root / "bad.zip", root / "out")
            self.assertFalse((root / "escape").exists())

    def test_tar_symlink(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            with tarfile.open(root / "bad.tgz", "w:gz") as t:
                item = tarfile.TarInfo("link")
                item.type, item.linkname = tarfile.SYMTYPE, "/tmp/elsewhere"
                t.addfile(item)
            with self.assertRaises(M.ImportFailure):
                M.safe_extract_tar(root / "bad.tgz", root / "out")

    def test_directory_markers_expand_not_delete(self):
        manifest = {"files": [{"path": "parts/", "status": "??"},
                              {"path": "old-a", "status": " D"}, {"path": "old-b", "status": " D"}]}
        declared, deleted = M.classify_manifest(manifest, ["parts/a.rs"])
        self.assertEqual(declared, {})
        self.assertEqual(deleted, {"old-a", "old-b"})

    def test_directory_as_deletion_rejected(self):
        with self.assertRaises(M.ImportFailure):
            M.classify_manifest({"files": [{"path": "parts/", "status": " D"},
                                            {"path": "old-b", "status": " D"}]}, ["parts/a.rs"])

    def test_duplicate_manifest_path(self):
        with self.assertRaises(M.ImportFailure):
            M.classify_manifest({"files": [{"path": "parts/", "status": "??"}] * 2}, ["parts/a.rs"])

    def test_corrupt_artifact_before_extraction(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "bad.zip").write_bytes(b"not the fixed artifact")
            with patch.object(M, "safe_extract_zip") as extract, self.assertRaises(M.ImportFailure):
                M.prepare(root / "bad.zip", root)
            extract.assert_not_called()

    def test_context_rejects_other_repository(self):
        with self.assertRaises(M.ImportFailure):
            M.validate_context("other/repo", M.DEFAULT_BRANCH, H)

    def test_context_rejects_arbitrary_branch(self):
        for branch in ("main", "master", "fix/world-other", "tags/v1", "../main"):
            with self.subTest(branch=branch), self.assertRaises(M.ImportFailure):
                M.validate_context(M.REPOSITORY, branch, H)

    def test_context_requires_full_sha(self):
        with self.assertRaises(M.ImportFailure):
            M.validate_context(M.REPOSITORY, M.DEFAULT_BRANCH, "main")

    def test_ci_cannot_publish(self):
        for key in ("CI", "GITHUB_ACTIONS"):
            with patch.dict(os.environ, {key: "true"}), self.assertRaises(M.ImportFailure):
                M.validate_context(M.REPOSITORY, M.DEFAULT_BRANCH, H)

    def test_unrelated_overlay_preserved(self):
        self.assertEqual(M.overlay_expected(BASE, CURRENT, RECORDS)["docs/README.md"], DOC)

    def test_source_drift_blocks_before_any_write(self):
        fake = FakeGitHub()
        fake.current["src/a.rs"] = ("100644", "blob", "3" * 40)
        with self.assertRaises(M.ImportFailure):
            self.run_publication(fake)
        self.assertTrue(all(c[0] == "GET" for c in fake.calls))

    def test_mode_drift_blocks_before_any_write(self):
        fake = FakeGitHub()
        fake.current["src/a.rs"] = ("100755", "blob", OLD[2])
        with self.assertRaises(M.ImportFailure):
            self.run_publication(fake)
        self.assertTrue(all(c[0] == "GET" for c in fake.calls))

    def test_deletion_drift_blocks_before_any_write(self):
        fake = FakeGitHub()
        fake.current["legacy.in"] = ("100644", "blob", "3" * 40)
        with self.assertRaises(M.ImportFailure):
            self.run_publication(fake)
        self.assertTrue(all(c[0] == "GET" for c in fake.calls))

    def test_success_exact_parent_and_nonforce_ref(self):
        fake = FakeGitHub()
        result = self.run_publication(fake)
        self.assertEqual(result["new_head"], N)
        self.assertEqual(result["disposition"], "published")
        self.assertEqual(sum(c[0] == "PATCH" for c in fake.calls), 1)

    def test_already_published_is_read_only(self):
        fake = FakeGitHub()
        fake.current = M.overlay_expected(BASE, CURRENT, RECORDS)
        result = self.run_publication(fake)
        self.assertEqual(result["disposition"], "already_present")
        self.assertTrue(all(c[0] == "GET" for c in fake.calls))

    def test_absent_deletion_not_sent_twice(self):
        fake = FakeGitHub()
        fake.current.pop("legacy.in")
        self.run_publication(fake)
        self.assertTrue(all(e["sha"] is not None for e in fake.overlay_entries))

    def test_branch_moves_during_upload(self):
        self.rejected_before_ref("race")

    def test_wrong_blob_rejected(self):
        self.rejected_before_ref("bad_blob")

    def test_wrong_qualified_tree_rejected(self):
        self.rejected_before_ref("bad_qualified")

    def test_unrelated_file_loss_rejected(self):
        self.rejected_before_ref("bad_overlay")

    def test_wrong_commit_parent_rejected(self):
        self.rejected_before_ref("bad_parent")

    def test_ref_readback_failure_is_not_success(self):
        fake = FakeGitHub()
        fake.bad_final_ref = True
        with self.assertRaisesRegex(M.ImportFailure, "final ref read-back"):
            self.run_publication(fake)

    def test_truncated_tree_rejected(self):
        with patch.object(M, "github", return_value={"sha": C, "truncated": True, "tree": []}), self.assertRaises(M.ImportFailure):
            M.read_tree(M.REPOSITORY, C, "unit-fake-token")

    def test_crossed_tree_rejected(self):
        with patch.object(M, "github", return_value={"sha": T, "truncated": False, "tree": []}), self.assertRaises(M.ImportFailure):
            M.read_tree(M.REPOSITORY, C, "unit-fake-token")

    def test_duplicate_tree_path_rejected(self):
        result = FakeGitHub().tree(C, CURRENT)
        result["tree"].append(result["tree"][0])
        with patch.object(M, "github", return_value=result), self.assertRaises(M.ImportFailure):
            M.read_tree(M.REPOSITORY, C, "unit-fake-token")

    def test_api_forbids_status_release_and_main(self):
        for suffix in ("statuses/" + H, "releases", "deployments", "git/refs/heads/main", "merges"):
            with self.subTest(suffix=suffix), patch.object(M.urllib.request, "urlopen") as network, self.assertRaises(M.ImportFailure):
                M.github("POST", f"/repos/{M.REPOSITORY}/{suffix}", "unit-fake-token", {})
            network.assert_not_called()

    def test_http_error_redacts_body(self):
        error = urllib.error.HTTPError("https://api.github.com", 403, "denied", {}, io.BytesIO(b"secret-reflection"))
        with patch.object(M.urllib.request, "urlopen", side_effect=error), self.assertRaises(M.ImportFailure) as raised:
            M.github("POST", f"/repos/{M.REPOSITORY}/git/blobs", "unit-fake-token", {})
        self.assertNotIn("secret", str(raised.exception))


if __name__ == "__main__":
    unittest.main()
