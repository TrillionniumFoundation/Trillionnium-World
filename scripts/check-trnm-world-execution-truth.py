#!/usr/bin/env python3
"""Check/render the selected execution snapshot; never certify its evidence."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tempfile
from datetime import datetime, timezone

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
VIEW = "docs/status/CURRENT.md"
SELECTOR = "trnm-current-execution-snapshot"
CLOSURE_KEYS = (
    "world_owned_source_development_closed", "exact_head_ci_closed",
    "independent_review_closed", "server_governance_closed", "all_plan_gaps_closed",
)
EXTERNAL_KEYS = {
    "nakama_canonical_authority_and_cutover", "trusted_cex_custody_and_settlement",
    "deployment_and_public_edge", "cross_host_recovery_and_endurance",
    "human_and_accessibility_validation", "privacy_legal_support_and_commercial",
}
MAX_BYTES = 256 * 1024


class TruthFailure(ValueError):
    pass


def check(condition: bool, message: str) -> None:
    if not condition:
        raise TruthFailure(message)


def inside(root: Path, relative: str, *, missing: bool = False) -> Path:
    path = PurePosixPath(relative)
    check(bool(path.parts) and not path.is_absolute() and path.as_posix() == relative
          and not any(p in {"..", ".git"} for p in path.parts)
          and "\\" not in relative and ":" not in relative and "\0" not in relative,
          "unsafe repository-relative path")
    cursor = root
    for part in path.parts:
        cursor /= part
        check(not cursor.is_symlink(), "symlink is not an execution-truth source")
    check(missing or cursor.is_file(), "execution-truth source is missing")
    return cursor


def read(root: Path, relative: str) -> bytes:
    path = inside(root, relative)
    with path.open("rb") as handle:
        data = handle.read(MAX_BYTES + 1)
    check(0 < len(data) <= MAX_BYTES and bool(data.strip()), "empty/oversized execution-truth source")
    return data


def unique_object(pairs: list[tuple[str, object]]) -> dict:
    result = {}
    for key, value in pairs:
        check(key not in result, "duplicate JSON key")
        result[key] = value
    return result


def reject_constant(_value: str) -> None:
    raise TruthFailure("non-finite JSON number")


def text_code(value: object, label: str) -> str:
    check(isinstance(value, str) and re.fullmatch(r"[a-z][a-z0-9_]{0,127}", value) is not None,
          f"invalid {label}")
    return value


def positive_id(value: object, label: str) -> None:
    check(type(value) is int and 0 < value <= 2**63 - 1, f"invalid {label}")


def validate_snapshot(snapshot: object) -> dict:
    check(isinstance(snapshot, dict), "snapshot must be an object")
    check(snapshot.get("schema") == "trnm_world_plan_v4_execution_truth_v1", "unsupported snapshot schema")
    check(snapshot.get("repository") == REPOSITORY, "crossed repository")
    positive_id(snapshot.get("operative_pull_request"), "operative pull request")
    branch = snapshot.get("operative_branch")
    check(isinstance(branch, str) and re.fullmatch(r"(?:feature|fix|chore|docs|test)/world-[a-z0-9][a-z0-9._-]*", branch) is not None,
          "invalid operative branch")
    recorded = snapshot.get("recorded_at_utc")
    check(isinstance(recorded, str) and re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", recorded) is not None,
          "snapshot time must be explicit UTC")
    datetime.strptime(recorded, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=timezone.utc)
    for key in ("source_qualification", "source_publication", "github_actions", "closure", "external_evidence_gaps"):
        check(isinstance(snapshot.get(key), dict) and bool(snapshot[key]), f"missing {key}")
    qualification = snapshot["source_qualification"]
    for key in ("source_world_head", "source_world_tree", "qualification_control_head", "qualified_source_tree"):
        check(isinstance(qualification.get(key), str) and re.fullmatch(r"[0-9a-f]{40}", qualification[key]) is not None,
              f"invalid {key}")
    for key in ("artifact_zip_sha256", "source_patch_sha256", "candidate_archive_sha256", "manifest_sha256", "identity_sha256"):
        check(isinstance(qualification.get(key), str) and re.fullmatch(r"[0-9a-f]{64}", qualification[key]) is not None,
              f"invalid {key}")
    for key in ("workflow_run_id", "workflow_job_id", "artifact_id"):
        positive_id(qualification.get(key), key)
    check(isinstance(qualification.get("rust_toolchain"), str)
          and re.fullmatch(r"\d+\.\d+\.\d+", qualification["rust_toolchain"]) is not None, "invalid toolchain")
    check(isinstance(qualification.get("qualification_result"), str)
          and qualification["qualification_result"] in {"pass", "fail", "pending"}, "invalid qualification result")
    publication = snapshot["source_publication"]
    text_code(publication.get("state"), "publication state")
    for key in ("qualified_tree_present", "qualified_tree_attached_to_pull_request"):
        check(type(publication.get(key)) is bool, f"missing/invalid {key}")
    check(not publication["qualified_tree_attached_to_pull_request"] or publication["qualified_tree_present"],
          "attached source cannot be absent")
    actions = snapshot["github_actions"]
    count = actions.get("repository_workflow_run_total")
    check(type(count) is int and 0 <= count <= 2**63 - 1, "invalid workflow count")
    text_code(actions.get("state"), "Actions state")
    text_code(actions.get("exact_head_evidence"), "exact-head evidence state")
    closure = snapshot["closure"]
    for key in CLOSURE_KEYS:
        check(type(closure.get(key)) is bool, f"missing/invalid {key}")
    # This v1 renderer is bound to the current no-production candidate profile.
    # A future release authorization requires reviewed schema/gate changes.
    check(closure.get("production_authorization") == "not_granted", "unsupported production authorization")
    external = snapshot["external_evidence_gaps"]
    check(EXTERNAL_KEYS <= external.keys(), "external evidence denominator omitted")
    for key, value in external.items():
        text_code(key, "external evidence key")
        check(isinstance(value, str) and value in {"open", "closed", "blocked", "pending"}, "invalid external evidence state")
    if closure["world_owned_source_development_closed"]:
        check(publication["qualified_tree_attached_to_pull_request"]
              and qualification["qualification_result"] == "pass", "source closure without qualified publication")
    if actions["exact_head_evidence"] in {"pass", "verified"}:
        check(count > 0, "exact-head evidence assertion with zero workflow runs")
    if closure["exact_head_ci_closed"]:
        check(count > 0 and actions["exact_head_evidence"] in {"pass", "verified"}, "CI closure without non-empty evidence")
    if closure["all_plan_gaps_closed"]:
        check(all(closure[k] for k in CLOSURE_KEYS) and all(v == "closed" for v in external.values()),
              "aggregate closure hides an open denominator")
    return snapshot


def select_snapshot(root: Path) -> tuple[str, dict, str]:
    plan = read(root, "CURRENT_PLAN.md").decode("utf-8")
    # Exactly one machine selector and one matching human pointer. Code fences
    # cannot supply a selector. Arbitrary newest-file selection is forbidden.
    check(plan.count(SELECTOR) == 1, "missing/ambiguous execution selector")
    fence = None
    active = []
    for line in plan.splitlines():
        match = re.match(r"^ {0,3}(`{3,}|~{3,})(.*)$", line)
        if fence:
            if match and match[1][0] == fence[0] and len(match[1]) >= fence[1] and not match[2].strip():
                fence = None
            continue
        if match:
            fence = (match[1][0], len(match[1]))
        else:
            active.append(line)
    check(fence is None, "unclosed plan code fence")
    visible = "\n".join(active)
    selected = re.findall(r"(?m)^<!-- " + SELECTOR + r": ([^\n]+) -->$", visible)
    check(len(selected) == 1, "missing/ambiguous active execution selector")
    relative = selected[0]
    check(re.fullmatch(r"docs/status/world-plan-v4-execution-truth-\d{4}-\d{2}-\d{2}\.json", relative) is not None,
          "invalid execution snapshot path")
    human = re.findall(r"The authoritative current execution snapshot is:\s*\n\s*- `([^`]+)`", visible)
    check(human == [relative], "human and machine execution pointers disagree")
    raw = read(root, relative)
    snapshot = validate_snapshot(json.loads(raw.decode("utf-8"), object_pairs_hook=unique_object,
                                            parse_constant=reject_constant))
    prs = re.findall(r"(?m)^- Pull request: `#([0-9]+)`$", visible)
    branches = re.findall(r"(?m)^- Branch: `([^`]+)`$", visible)
    check(prs == [str(snapshot["operative_pull_request"])] and branches == [snapshot["operative_branch"]],
          "root candidate and snapshot disagree")
    return relative, snapshot, hashlib.sha256(raw).hexdigest()


def render(root: Path) -> str:
    relative, snapshot, digest = select_snapshot(root)
    q = snapshot["source_qualification"]
    p = snapshot["source_publication"]
    a = snapshot["github_actions"]
    lines = [
        "# Trillionnium World Current Status", "",
        "> Generated from the snapshot explicitly selected by `CURRENT_PLAN.md`.",
        "> These are recorded assertions, not fresh GitHub queries or independent evidence verification.", "",
        f"- Selected execution snapshot: `{relative}`",
        f"- Snapshot recorded at (UTC): `{snapshot['recorded_at_utc']}`",
        f"- Snapshot SHA-256: `{digest}`",
        f"- Operative pull request: `#{snapshot['operative_pull_request']}`",
        f"- Operative branch: `{snapshot['operative_branch']}`", "",
        "Later explicit live observations in `CURRENT_PLAN.md` govern only their stated scope.",
        "In particular, consult that root pointer for the CEX dependency disposition; an old retained pin is not a newly selected or qualified CEX candidate.",
        "The binding repository boundary and accepted ADRs remain authoritative.", "",
        "## Recorded source and repository posture", "",
        "| Field | Recorded value |", "| --- | --- |",
        f"| Separate artifact qualification | `{q['qualification_result']}` |",
        f"| Qualified source tree | `{q['qualified_source_tree']}` |",
        f"| Qualification control head | `{q['qualification_control_head']}` |",
        f"| Qualification run / artifact | `{q['workflow_run_id']}` / `{q['artifact_id']}` |",
        f"| Qualified artifact ZIP SHA-256 | `{q['artifact_zip_sha256']}` |",
        f"| Rust toolchain | `{q['rust_toolchain']}` |",
        f"| Source publication state | `{p['state']}` |",
        f"| Qualified tree present | `{str(p['qualified_tree_present']).lower()}` |",
        f"| Qualified tree attached to PR | `{str(p['qualified_tree_attached_to_pull_request']).lower()}` |",
        f"| Repository workflow run count at snapshot | `{a['repository_workflow_run_total']}` |",
        f"| Actions state | `{a['state']}` |",
        f"| Exact-head evidence | `{a['exact_head_evidence']}` |", "",
        "A qualified artifact is not publication on the operative branch. Source publication is not compilation, hosted CI, independent review or release eligibility.",
        "Re-query GitHub for the final PR head and prospective merge object before assigning remote verification credit. Empty, skipped, cancelled or stale checks are not a pass.", "",
        "## Recorded closure flags", "", "| Denominator | Recorded value |", "| --- | --- |",
    ]
    lines += [f"| `{key}` | `{str(snapshot['closure'][key]).lower()}` |" for key in CLOSURE_KEYS]
    lines += ["| Production authorization | `not_granted` |", "", "## Recorded external evidence", "",
              "| Denominator | Recorded state |", "| --- | --- |"]
    lines += [f"| `{key}` | `{value}` |" for key, value in sorted(snapshot["external_evidence_gaps"].items())]
    lines += ["", "## Evidence boundary", "",
              "Public online operation, public player markets and commercial release remain **NO-GO / disabled** under the current root plan. This renderer cannot authorize deployment or enablement.",
              "Repository protection, required checks and review enforcement require live server-side read-back. This view does not reproduce superseded governance observations as current facts.",
              "Source, fixture tests and generated status cannot satisfy cross-repository, deployed, cross-host, custody, human, accessibility, privacy, legal, support or commercial evidence requirements.", "",
              "## Maintenance", "", "```bash",
              "python3 scripts/check-trnm-world-execution-truth.py",
              "python3 scripts/test-trnm-world-execution-truth.py", "```", "",
              "The default check is read-only and rejects any stale or manually altered view.",
              "After an authorized snapshot/pointer update, a local operator may regenerate this file with `--write`; that option is rejected in CI and does not modify the snapshot or close any gap.", ""]
    return "\n".join(lines)


def verify(root: Path) -> None:
    check(read(root, VIEW) == render(root).encode("utf-8"), "CURRENT.md is stale or manually altered")


def write_view(root: Path) -> None:
    check(not any(os.environ.get(k, "").lower() not in {"", "0", "false"} for k in ("CI", "GITHUB_ACTIONS")),
          "status generation is forbidden in CI")
    data = render(root).encode("utf-8")
    target = inside(root, VIEW, missing=True)
    check(target.parent.is_dir(), "status directory is missing")
    name = None
    try:
        with tempfile.NamedTemporaryFile(dir=target.parent, prefix=".execution-view-", delete=False) as handle:
            name = handle.name
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
        os.chmod(name, 0o644)
        os.replace(name, target)
    finally:
        if name is not None and os.path.exists(name):
            os.unlink(name)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--write", action="store_true", help="local operator-only regeneration")
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        if args.write:
            write_view(root)
        verify(root)
    except (TruthFailure, OSError, ValueError, UnicodeError, RecursionError) as error:
        print(f"TRNM World execution view: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM World execution view: PASS (selected-snapshot consistency only; no evidence promotion)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
