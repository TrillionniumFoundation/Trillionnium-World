#!/usr/bin/env python3
"""Offline Git/event fault tests; no fixture is GitHub execution or release evidence."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("check-trnm-world-ci-target.py")
ROOT = SCRIPT.resolve().parents[1]
spec = importlib.util.spec_from_file_location("ci_target", SCRIPT)
assert spec is not None and spec.loader is not None
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)
REPO = {"id": checker.REPOSITORY_ID, "full_name": checker.REPOSITORY}


def environment() -> dict:
    result = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    result.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull,
                  GIT_AUTHOR_NAME="Synthetic target fixture", GIT_AUTHOR_EMAIL="fixture@invalid",
                  GIT_COMMITTER_NAME="Synthetic target fixture", GIT_COMMITTER_EMAIL="fixture@invalid")
    return result


def git(root: Path, *args: str, data: str | None = None) -> str:
    return subprocess.check_output(["git", "-C", str(root), *args], env=environment(),
                                   input=data, text=True, stderr=subprocess.PIPE, timeout=15).strip()


class CheckoutTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.store_temp = tempfile.TemporaryDirectory(prefix="trnm-ci-event-store-")
        cls.store = Path(cls.store_temp.name)
        git(cls.store, "init", "-q", "--object-format=sha1")
        git(cls.store, "config", "core.hooksPath", os.devnull)
        (cls.store / "PROJECT_ID").write_text("trillionnium-world\n")
        git(cls.store, "add", "PROJECT_ID")
        cls.base_tree = git(cls.store, "write-tree")
        cls.base = git(cls.store, "commit-tree", cls.base_tree, data="Synthetic base\n")
        (cls.store / "implementation.txt").write_text("Synthetic head\n")
        git(cls.store, "add", "implementation.txt")
        cls.head_tree = git(cls.store, "write-tree")
        cls.head = git(cls.store, "commit-tree", cls.head_tree, "-p", cls.base, data="Synthetic head\n")
        cls.merge = git(cls.store, "commit-tree", cls.head_tree, "-p", cls.base, "-p", cls.head,
                        data="Synthetic prospective merge, never a real PR\n")
        git(cls.store, "update-ref", "HEAD", cls.merge)

    @classmethod
    def tearDownClass(cls):
        cls.store_temp.cleanup()

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="trnm-ci-event-test-")
        self.addCleanup(self.temp.cleanup)
        self.top = Path(self.temp.name)
        self.root = self.top / "checkout"
        git(self.top, "clone", "--shared", "--no-checkout", "-q", str(self.store), str(self.root))
        git(self.root, "remote", "set-url", "origin", f"https://github.com/{checker.REPOSITORY}.git")
        git(self.root, "checkout", "--detach", "-q", self.merge)
        self.event = {
            "repository": copy.deepcopy(REPO), "number": 46, "action": "synchronize",
            "pull_request": {
                "number": 46, "state": "open", "draft": True,
                "head": {"sha": self.head, "ref": checker.BRANCH, "repo": copy.deepcopy(REPO)},
                "base": {"sha": self.base, "ref": "main", "repo": copy.deepcopy(REPO)},
            },
        }
        self.env = {
            "GITHUB_REPOSITORY": checker.REPOSITORY, "GITHUB_REPOSITORY_ID": str(checker.REPOSITORY_ID),
            "GITHUB_SHA": self.merge, "GITHUB_EVENT_NAME": "pull_request",
            "GITHUB_REF": "refs/pull/46/merge", "GITHUB_HEAD_REF": checker.BRANCH,
            "GITHUB_BASE_REF": "main", "GITHUB_RUN_ID": "123", "GITHUB_RUN_ATTEMPT": "1", "GITHUB_JOB": "fixture",
        }

    def verify(self, role="event"):
        return checker.verify(self.root, self.event, self.env, role)

    def reject(self, role="event"):
        with self.assertRaises(checker.TargetFailure):
            self.verify(role)

    def select_commit(self, *parents: str, message="Synthetic wrong merge\n"):
        commit = git(self.root, "commit-tree", self.head_tree,
                     *[arg for parent in parents for arg in ("-p", parent)], data=message)
        git(self.root, "checkout", "--detach", "-q", commit)
        self.env["GITHUB_SHA"] = commit

    def push(self):
        self.event = {"repository": copy.deepcopy(REPO), "ref": f"refs/heads/{checker.BRANCH}",
                      "after": self.head, "deleted": False}
        self.env.update(GITHUB_EVENT_NAME="push", GITHUB_SHA=self.head, GITHUB_REF=self.event["ref"],
                        GITHUB_HEAD_REF="", GITHUB_BASE_REF="")
        git(self.root, "checkout", "--detach", "-q", self.head)

    def test_pr_event_checks_exact_merge(self):
        result = self.verify()
        self.assertEqual(result["role"], "prospective_merge")
        self.assertEqual(result["commit_parents"], [self.base, self.head])
        self.assertEqual(result["checkout_tree"], self.head_tree)
        self.assertFalse(result["tests_verified"])
        self.assertFalse(result["remote_evidence_verified"])
        self.assertEqual(result["production_authorization"], "not_granted")

    def test_pr_head_is_a_separate_explicit_role(self):
        git(self.root, "checkout", "--detach", "-q", self.head)
        self.assertEqual(self.verify("head")["role"], "head")
        self.reject("merge")
        self.reject("event")

    def test_merge_cannot_impersonate_head(self):
        self.reject("head")

    def test_explicit_merge_role_passes_only_correct_parents(self):
        self.assertEqual(self.verify("merge")["checkout_commit"], self.merge)

    def test_squash_is_not_prospective_two_parent_merge(self):
        self.select_commit(self.base)
        self.reject()

    def test_reversed_parents_are_rejected(self):
        self.select_commit(self.head, self.base)
        self.reject()

    def test_stale_head_parent_is_rejected(self):
        other = git(self.root, "commit-tree", self.head_tree, "-p", self.base, data="Other head\n")
        self.select_commit(self.base, other)
        self.reject()

    def test_extra_parent_is_rejected(self):
        other = git(self.root, "commit-tree", self.head_tree, "-p", self.base, data="Third parent\n")
        self.select_commit(self.base, self.head, other)
        self.reject()

    def test_message_mentions_cannot_supply_missing_parents(self):
        self.select_commit(self.base, message=f"parent {self.head}\nbase {self.base}\n")
        self.reject()

    def test_event_sha_cannot_equal_head_or_base(self):
        for value in (self.base, self.head):
            with self.subTest(value=value):
                self.env["GITHUB_SHA"] = value
                self.reject()

    def test_wrong_actual_checkout_rejected(self):
        git(self.root, "checkout", "--detach", "-q", self.base)
        self.reject()

    def test_ref_number_mismatch_rejected(self):
        self.env["GITHUB_REF"] = "refs/pull/45/merge"
        self.reject()

    def test_boolean_number_is_not_id(self):
        self.event["number"] = True
        self.reject()

    def test_nested_number_mismatch(self):
        self.event["pull_request"]["number"] = 47
        self.reject()

    def test_closed_pr_rejected(self):
        self.event["pull_request"]["state"] = "closed"
        self.reject()

    def test_repository_identity_in_all_three_locations(self):
        for location in ("root", "head", "base"):
            event = copy.deepcopy(self.event)
            target = self.event["repository"] if location == "root" else self.event["pull_request"][location]["repo"]
            target["full_name"] = "other/repository"
            with self.subTest(location=location): self.reject()
            self.event = event

    def test_numeric_repository_id_is_bound(self):
        for value in (True, 1, "1323087277", None):
            self.event["repository"]["id"] = value
            with self.subTest(value=value): self.reject()

    def test_environment_repository_mismatch(self):
        self.env["GITHUB_REPOSITORY"] = "other/repository"
        self.reject()

    def test_environment_repository_id_mismatch(self):
        self.env["GITHUB_REPOSITORY_ID"] = "1"
        self.reject()

    def test_branch_metadata_mismatch(self):
        self.env["GITHUB_HEAD_REF"] = "fix/world-other"
        self.reject()

    def test_lane_and_base_branch_validation(self):
        self.event["pull_request"]["head"]["ref"] = "main"
        self.env["GITHUB_HEAD_REF"] = "main"
        self.reject()
        self.event["pull_request"]["head"]["ref"] = checker.BRANCH
        self.env["GITHUB_HEAD_REF"] = checker.BRANCH
        self.event["pull_request"]["base"]["ref"] = "other"
        self.reject()

    def test_invalid_sha_values(self):
        for value in ("", "0" * 40, "A" * 40, "x" * 40, True, None, self.head[:8], self.head + "\n"):
            with self.subTest(value=value):
                self.env["GITHUB_SHA"] = value
                self.reject()

    def test_missing_or_malformed_pr_shape(self):
        for value in (None, [], "bad"):
            self.event["pull_request"] = value
            with self.subTest(value=value): self.reject()

    def test_privileged_and_unknown_events_are_rejected(self):
        for name in ("pull_request_target", "issue_comment", "merge_group", "schedule", ""):
            with self.subTest(name=name):
                self.env["GITHUB_EVENT_NAME"] = name
                self.reject()

    def test_push_binds_after_and_returns_head_role(self):
        self.push()
        self.assertEqual(self.verify()["role"], "head")
        self.assertEqual(self.verify("head")["checkout_commit"], self.head)

    def test_push_cannot_be_labelled_prospective_merge(self):
        self.push()
        self.reject("merge")

    def test_push_after_disagreement(self):
        self.push()
        self.event["after"] = self.base
        self.reject()

    def test_push_ref_disagreement(self):
        self.push()
        self.event["ref"] = "refs/heads/main"
        self.reject()

    def test_push_deletion_type_is_strict(self):
        self.push()
        for value in (True, None, 0, "false"):
            self.event["deleted"] = value
            with self.subTest(value=value): self.reject()

    def test_tags_and_unapproved_push_branches_rejected(self):
        self.push()
        for ref in ("refs/tags/v1", "refs/heads/other", "refs/pull/46/merge"):
            self.env["GITHUB_REF"] = self.event["ref"] = ref
            with self.subTest(ref=ref): self.reject()

    def test_dispatch_inputs_cannot_override_runner_sha(self):
        self.push()
        self.env["GITHUB_EVENT_NAME"] = "workflow_dispatch"
        self.event = {"repository": copy.deepcopy(REPO), "inputs": {"sha": self.base, "role": "merge"}}
        self.assertEqual(self.verify()["role"], "dispatched_head")
        self.assertEqual(self.verify()["checkout_commit"], self.head)
        self.reject("merge")

    def test_dispatch_ref_disagreement(self):
        self.push()
        self.env["GITHUB_EVENT_NAME"] = "workflow_dispatch"
        self.event = {"repository": copy.deepcopy(REPO), "ref": "main"}
        self.reject()

    def test_crossed_origin_rejected(self):
        git(self.root, "remote", "set-url", "origin", "https://github.com/other/repo.git")
        self.reject()

    def test_changed_tracked_file_rejected(self):
        (self.root / "PROJECT_ID").write_text("wrong\n")
        self.reject()

    def test_untracked_file_rejected(self):
        (self.root / "extra.py").write_text("not committed\n")
        self.reject()

    def test_committed_wrong_project_rejected(self):
        self.push()
        (self.root / "PROJECT_ID").write_text("other-project\n")
        git(self.root, "add", "PROJECT_ID")
        tree = git(self.root, "write-tree")
        commit = git(self.root, "commit-tree", tree, "-p", self.head, data="Wrong project\n")
        git(self.root, "checkout", "--detach", "-q", commit)
        self.event["after"] = self.env["GITHUB_SHA"] = commit
        self.reject()

    def test_nested_and_linked_root_rejected(self):
        nested = self.root / "nested"
        nested.mkdir()
        with self.assertRaises(checker.TargetFailure): checker.verify(nested, self.event, self.env, "event")
        nested.rmdir()
        linked = self.top / "link"
        linked.symlink_to(self.root)
        with self.assertRaises(checker.TargetFailure): checker.verify(linked, self.event, self.env, "event")

    def test_no_index_head_or_source_mutation(self):
        paths = [self.root / ".git/index", self.root / ".git/HEAD", self.root / "PROJECT_ID"]
        before = [p.read_bytes() for p in paths]
        self.verify()
        self.assertEqual(before, [p.read_bytes() for p in paths])

    def test_git_environment_cannot_redirect_inspection(self):
        with patch.dict(os.environ, {"GIT_DIR": "/missing", "GIT_WORK_TREE": "/missing", "GIT_CONFIG_COUNT": "1",
                                     "GIT_CONFIG_KEY_0": "remote.origin.url", "GIT_CONFIG_VALUE_0": "other"}):
            self.verify()

    def test_git_budget_exhaustion_fails_closed(self):
        with patch.object(checker, "MAX_GIT_BYTES", 1): self.reject()

    def test_head_race_rejected(self):
        original = checker.git
        reads = 0
        def racing(root, *args):
            nonlocal reads
            if args == ("rev-parse", "--verify", "HEAD^{commit}"):
                reads += 1
                if reads == 2: return self.head
            return original(root, *args)
        with patch.object(checker, "git", side_effect=racing): self.reject()

    def test_run_attempt_and_job_identity_are_required(self):
        original = self.env.copy()
        for name, value in (("GITHUB_RUN_ID", "0"), ("GITHUB_RUN_ATTEMPT", "-1"), ("GITHUB_JOB", "bad/job")):
            self.env = {**original, name: value}
            with self.subTest(name=name): self.reject()

    def test_cli_success_emits_identity_not_test_pass(self):
        path = self.top / "event.json"
        raw = json.dumps(self.event).encode()
        path.write_bytes(raw)
        result = subprocess.run([sys.executable, str(SCRIPT), "--root", str(self.root)],
                                env={**environment(), **self.env, "GITHUB_EVENT_PATH": str(path)},
                                capture_output=True, text=True, timeout=15)
        self.assertEqual(result.returncode, 0, result.stderr)
        data = json.loads(result.stdout)
        self.assertEqual(data["event_payload_sha256"], hashlib.sha256(raw).hexdigest())
        self.assertFalse(data["tests_verified"])

    def test_cli_failure_has_no_success_json(self):
        path = self.top / "event.json"
        path.write_text(json.dumps(self.event))
        result = subprocess.run([sys.executable, str(SCRIPT), "--root", str(self.root), "--role", "head"],
                                env={**environment(), **self.env, "GITHUB_EVENT_PATH": str(path)},
                                capture_output=True, text=True, timeout=15)
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout, "")
        self.assertIn("FAIL", result.stderr)


class ParserTests(unittest.TestCase):
    def test_event_input_budgets_types_and_unique_keys(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "event.json"
            for raw in (b"", b"[]", b"{", b"\xff", b'{"a":1,"a":2}', b'{"a":NaN}', b" " * (checker.MAX_EVENT_BYTES + 1)):
                path.write_bytes(raw)
                with self.subTest(raw=raw[:30]), self.assertRaises((checker.TargetFailure, ValueError)):
                    checker.read_event(path)

    def test_event_symlink_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "event.json"
            path.write_text("{}")
            link = Path(temp) / "linked.json"
            link.symlink_to(path)
            with self.assertRaises(checker.TargetFailure): checker.read_event(link)

    def test_commit_message_and_signature_cannot_inject_parents(self):
        tree, parents = checker.commit_headers("tree " + "a" * 40 + "\ngpgsig fake\n parent " + "b" * 40 + "\n\nparent " + "c" * 40)
        self.assertEqual(tree, "a" * 40)
        self.assertEqual(parents, [])

    def test_malformed_or_duplicate_commit_headers_rejected(self):
        for raw in ("parent " + "a" * 40, "tree bad", "tree " + "a" * 40 + "\ntree " + "a" * 40,
                    "tree " + "a" * 40 + ("\nparent " + "b" * 40) * 2):
            with self.subTest(raw=raw), self.assertRaises(checker.TargetFailure): checker.commit_headers(raw)


class WorkflowWiringTests(unittest.TestCase):
    """Narrow regression checks for the two known workflow layouts, not a YAML interpreter."""
    def setUp(self):
        self.gap = (ROOT / ".github/workflows/trnm-world-gap-closure-v4.yml").read_text()
        self.final = (ROOT / ".github/workflows/trnm-world-v4-final-gates.yml").read_text()

    def jobs(self, text):
        parts = re.split(r"(?m)^  ([A-Za-z][A-Za-z0-9_-]*):\n", text.split("\njobs:\n", 1)[1])
        return dict(zip(parts[1::2], parts[2::2]))

    def test_required_job_names_are_unique_between_workflows(self):
        names = re.findall(r"(?m)^    name: (.+)$", self.gap + "\n" + self.final)
        self.assertEqual(len(names), len(set(names)))
        for job in ("docs-governance", "transition-contract", "settlement-postgres", "game-workspace-release", "supply-chain"):
            self.assertIn(f"    name: trnm-world-v4/{job}\n", self.gap)
            self.assertIn(f"    name: trnm-world-v4-supplemental/{job}\n", self.final)

    def test_complete_workflow_receives_current_branch_pushes(self):
        triggers = self.gap.split("\npermissions:", 1)[0]
        self.assertIn("      - " + checker.BRANCH, triggers)
        self.assertNotIn("fix/world-plan-gap-closure-v4", triggers)

    def test_each_checkout_has_correct_identity_role_and_artifact(self):
        for name, body in self.jobs(self.gap).items():
            with self.subTest(workflow="gap", job=name):
                self.assertIn("check-trnm-world-ci-target.py --role event", body)
                self.assertIn("ref: ${{ github.sha }}", body)
                self.assertIn("Upload event-bound target identity", body)
                self.assertIn("if-no-files-found: error", body)
        for name, body in self.jobs(self.final).items():
            with self.subTest(workflow="final", job=name):
                role = "merge" if name == "prospective-merge" else "head"
                self.assertIn("check-trnm-world-ci-target.py --role " + role, body)
                self.assertIn("Upload event-bound target identity", body)

    def test_prospective_job_is_not_run_as_merge_on_push(self):
        body = self.jobs(self.final)["prospective-merge"]
        self.assertIn("    if: github.event_name == 'pull_request'", body)
        self.assertNotIn("grep -F", body)

    def test_database_env_matches_real_test_consumer(self):
        body = self.jobs(self.final)["settlement-postgres"]
        self.assertIn("TRNM_SETTLEMENT_TEST_DATABASE_URL:", body)
        self.assertNotIn("\n      DATABASE_URL:", body)
        self.assertIn('TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST: "1"', body)

    def test_toolchain_override_is_explicit_and_original_pins_retained(self):
        for text in (self.gap, self.final):
            self.assertIn('RUSTUP_TOOLCHAIN: "1.98.0"', text)
        self.assertIn("EXPECTED_QUALIFIED_TREE: 5e613185f5a2abda42df371f3755e73667717309", self.final)
        self.assertIn("EXPECTED_PATCH_SHA256: ba49dba1e7fbf842f146ac399647e188faafcfbd5ce3ad17425ef88850e0199f", self.final)

    def test_stronger_required_tests_are_not_replaced_by_metadata_only(self):
        for command in ("cargo test --workspace --all-targets --locked", "cargo build --release --locked -p trnm-first-contact -p trnm-game-server --bins",
                        "cargo audit --deny warnings", "cargo deny check advisories bans licenses sources",
                        "cargo test -p trnm-game-server --all-targets --locked", "package-trnm-game-release.sh --require-clean"):
            self.assertIn(command, self.gap)

    def test_new_regressions_run_in_existing_docs_gate(self):
        self.assertIn("python3 scripts/test-trnm-world-ci-target.py", self.jobs(self.gap)["docs-governance"])


if __name__ == "__main__":
    unittest.main(verbosity=2)
