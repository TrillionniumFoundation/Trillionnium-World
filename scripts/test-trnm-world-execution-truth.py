#!/usr/bin/env python3
"""Offline negative tests for selected-snapshot rendering, not evidence approval."""
from __future__ import annotations

import copy
import importlib.util
import json
import os
import sys
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

# Keep validation read-only even when Python bytecode caching is enabled.
sys.dont_write_bytecode = True

SCRIPT = Path(__file__).with_name("check-trnm-world-execution-truth.py")
spec = importlib.util.spec_from_file_location("execution_truth", SCRIPT)
assert spec is not None and spec.loader is not None
truth = importlib.util.module_from_spec(spec)
spec.loader.exec_module(truth)
SNAPSHOT = "docs/status/world-plan-v4-execution-truth-2026-09-02.json"


def fixture() -> dict:
    q = {k: "a" * 40 for k in ("source_world_head", "source_world_tree", "qualification_control_head", "qualified_source_tree")}
    q.update({k: "b" * 64 for k in ("artifact_zip_sha256", "source_patch_sha256", "candidate_archive_sha256", "manifest_sha256", "identity_sha256")})
    q.update(workflow_run_id=1, workflow_job_id=2, artifact_id=3, rust_toolchain="1.98.0", qualification_result="pass")
    return {
        "schema": "trnm_world_plan_v4_execution_truth_v1", "repository": truth.REPOSITORY,
        "recorded_at_utc": "2026-09-02T08:30:00Z", "operative_pull_request": 46,
        "operative_branch": "fix/world-plan-v4-development-closure-20260831",
        "source_qualification": q,
        "source_publication": {"state": "publication_blocked", "qualified_tree_present": False,
                               "qualified_tree_attached_to_pull_request": False},
        "github_actions": {"repository_workflow_run_total": 0, "state": "blocked", "exact_head_evidence": "absent"},
        "closure": {**dict.fromkeys(truth.CLOSURE_KEYS, False), "production_authorization": "not_granted"},
        "external_evidence_gaps": dict.fromkeys(sorted(truth.EXTERNAL_KEYS), "open"),
    }


class ExecutionTruthTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "docs/status").mkdir(parents=True)
        self.snapshot = fixture()
        self.plan = (
            "# Current Plan\n\nThe authoritative current execution snapshot is:\n\n"
            f"- `{SNAPSHOT}`\n\n<!-- {truth.SELECTOR}: {SNAPSHOT} -->\n\n"
            "- Pull request: `#46`\n- Branch: `fix/world-plan-v4-development-closure-20260831`\n"
        )
        self.save()
        (self.root / truth.VIEW).write_text(truth.render(self.root), encoding="utf-8")

    def save(self) -> None:
        (self.root / "CURRENT_PLAN.md").write_text(self.plan, encoding="utf-8")
        (self.root / SNAPSHOT).write_text(json.dumps(self.snapshot, indent=2) + "\n", encoding="utf-8")

    def rejected(self) -> None:
        self.save()
        with self.assertRaises((truth.TruthFailure, ValueError)):
            truth.render(self.root)

    def test_selected_snapshot_passes_without_promoting_gaps(self):
        before = copy.deepcopy(self.snapshot)
        truth.verify(self.root)
        rendered = truth.render(self.root)
        self.assertIn("not fresh GitHub queries", rendered)
        self.assertIn("Production authorization | `not_granted`", rendered)
        self.assertEqual(before, self.snapshot)
        self.assertNotIn("world-v4-convergence-state-2026-08-30", rendered)

    def test_unselected_newer_snapshot_cannot_override(self):
        (self.root / "docs/status/world-plan-v4-execution-truth-2099-01-01.json").write_text("{}")
        truth.verify(self.root)

    def test_snapshot_change_makes_view_stale(self):
        self.snapshot["github_actions"]["repository_workflow_run_total"] = 1
        self.save()
        with self.assertRaises(truth.TruthFailure):
            truth.verify(self.root)

    def test_manual_view_change_rejected(self):
        with (self.root / truth.VIEW).open("a") as handle:
            handle.write("All gaps are closed.\n")
        with self.assertRaises(truth.TruthFailure):
            truth.verify(self.root)

    def test_missing_selector(self):
        self.plan = self.plan.replace(truth.SELECTOR, "other")
        self.rejected()

    def test_duplicate_selector(self):
        self.plan += f"<!-- {truth.SELECTOR}: {SNAPSHOT} -->\n"
        self.rejected()

    def test_fenced_selector(self):
        marker = f"<!-- {truth.SELECTOR}: {SNAPSHOT} -->"
        self.plan = self.plan.replace(marker, f"```\n{marker}\n```")
        self.rejected()

    def test_unclosed_code_fence(self):
        self.plan += "```\n"
        self.rejected()

    def test_crossed_human_pointer(self):
        self.plan = self.plan.replace(f"- `{SNAPSHOT}`", "- `docs/status/old.json`")
        self.rejected()

    def test_unsafe_selector_variants(self):
        for path in ("../outside.json", "/tmp/outside.json", "docs/status/../bad.json", "C:\\bad.json"):
            with self.subTest(path=path):
                (self.root / "CURRENT_PLAN.md").write_text(self.plan.replace(SNAPSHOT, path))
                with self.assertRaises(truth.TruthFailure):
                    truth.render(self.root)

    def test_duplicate_json_key(self):
        p = self.root / SNAPSHOT
        p.write_text(p.read_text().replace('"schema":', '"schema": "crossed", "schema":', 1))
        with self.assertRaises(truth.TruthFailure):
            truth.render(self.root)

    def test_nonfinite_json_rejected(self):
        p = self.root / SNAPSHOT
        p.write_text(p.read_text().replace('"repository_workflow_run_total": 0', '"repository_workflow_run_total": NaN'))
        with self.assertRaises(truth.TruthFailure):
            truth.render(self.root)

    def test_crossed_repository(self):
        self.snapshot["repository"] = "other/repo"
        self.rejected()

    def test_crossed_schema(self):
        self.snapshot["schema"] = "future_v99"
        self.rejected()

    def test_crossed_candidate(self):
        self.snapshot["operative_pull_request"] = 39
        self.rejected()

    def test_crossed_branch(self):
        self.snapshot["operative_branch"] = "fix/world-other"
        self.rejected()

    def test_boolean_is_not_workflow_count(self):
        self.snapshot["github_actions"]["repository_workflow_run_total"] = False
        self.rejected()

    def test_string_is_not_closure_flag(self):
        self.snapshot["closure"]["all_plan_gaps_closed"] = "false"
        self.rejected()

    def test_empty_denominator_objects(self):
        for key in ("source_qualification", "source_publication", "github_actions", "closure", "external_evidence_gaps"):
            snapshot = fixture()
            snapshot[key] = {}
            with self.subTest(key=key), self.assertRaises(truth.TruthFailure):
                truth.validate_snapshot(snapshot)

    def test_missing_external_denominator(self):
        del self.snapshot["external_evidence_gaps"]["human_and_accessibility_validation"]
        self.rejected()

    def test_bad_field_shapes(self):
        for field, key in (("source_qualification", "qualification_result"), ("external_evidence_gaps", "human_and_accessibility_validation")):
            for value in ([], {}, None, True, 1):
                snapshot = fixture()
                snapshot[field][key] = value
                with self.subTest(field=field, value=value), self.assertRaises(truth.TruthFailure):
                    truth.validate_snapshot(snapshot)

    def test_attached_but_absent(self):
        self.snapshot["source_publication"]["qualified_tree_attached_to_pull_request"] = True
        self.rejected()

    def test_source_closure_without_publication(self):
        self.snapshot["closure"]["world_owned_source_development_closed"] = True
        self.rejected()

    def test_ci_closed_with_zero_runs(self):
        self.snapshot["closure"]["exact_head_ci_closed"] = True
        self.rejected()

    def test_failed_artifact_cannot_support_source_closure(self):
        self.snapshot["source_publication"].update(qualified_tree_present=True, qualified_tree_attached_to_pull_request=True)
        self.snapshot["source_qualification"]["qualification_result"] = "fail"
        self.snapshot["closure"]["world_owned_source_development_closed"] = True
        self.rejected()

    def test_verified_exact_head_with_zero_runs(self):
        self.snapshot["github_actions"]["exact_head_evidence"] = "verified"
        self.rejected()

    def test_aggregate_false_closure(self):
        self.snapshot["closure"]["all_plan_gaps_closed"] = True
        self.rejected()

    def test_production_promotion_rejected(self):
        self.snapshot["closure"]["production_authorization"] = "granted"
        self.rejected()

    def test_explicit_utc_and_calendar_validation(self):
        for value in ("2026-09-02", "2026-09-02T08:30:00+08:00", "2026-02-30T00:00:00Z"):
            snapshot = fixture()
            snapshot["recorded_at_utc"] = value
            with self.subTest(value=value), self.assertRaises((truth.TruthFailure, ValueError)):
                truth.validate_snapshot(snapshot)

    def test_hash_and_id_validation(self):
        for key, value in (("source_world_head", "x" * 40), ("artifact_zip_sha256", "a" * 63), ("artifact_id", True)):
            snapshot = fixture()
            snapshot["source_qualification"][key] = value
            with self.subTest(key=key), self.assertRaises(truth.TruthFailure):
                truth.validate_snapshot(snapshot)

    def test_markdown_state_injection_rejected(self):
        self.snapshot["source_publication"]["state"] = "pass` | enabled"
        self.rejected()

    def test_symlink_sources_rejected(self):
        for relative in ("CURRENT_PLAN.md", SNAPSHOT, truth.VIEW):
            p = self.root / relative
            data = p.read_bytes()
            other = self.root / "other"
            other.write_bytes(data)
            p.unlink()
            p.symlink_to(other)
            with self.subTest(path=relative), self.assertRaises(truth.TruthFailure):
                truth.verify(self.root)
            p.unlink()
            p.write_bytes(data)

    def test_oversized_snapshot(self):
        (self.root / SNAPSHOT).write_bytes(b" " * (truth.MAX_BYTES + 1))
        with self.assertRaises(truth.TruthFailure):
            truth.render(self.root)

    def test_write_requires_local_operator(self):
        before = (self.root / truth.VIEW).read_bytes()
        for key in ("CI", "GITHUB_ACTIONS"):
            with patch.dict(os.environ, {key: "true"}), self.subTest(key=key):
                with self.assertRaises(truth.TruthFailure):
                    truth.write_view(self.root)
                self.assertEqual(before, (self.root / truth.VIEW).read_bytes())

    def test_local_generation_is_idempotent_and_preserves_snapshot(self):
        raw = (self.root / SNAPSHOT).read_bytes()
        with patch.dict(os.environ, {"CI": "false", "GITHUB_ACTIONS": "false"}):
            truth.write_view(self.root)
            first = (self.root / truth.VIEW).read_bytes()
            truth.write_view(self.root)
        self.assertEqual(first, (self.root / truth.VIEW).read_bytes())
        self.assertEqual(raw, (self.root / SNAPSHOT).read_bytes())
        truth.verify(self.root)
        self.assertFalse(list((self.root / "docs/status").glob(".execution-view-*")))


if __name__ == "__main__":
    unittest.main(verbosity=2)
