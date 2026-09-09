#!/usr/bin/env python3
"""Verify immutable historical origin and the exact reviewed World-authority successor."""

from __future__ import annotations

import argparse
from collections import Counter
import copy
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, Callable

from trnm_world_strict_json import StrictJsonError, load_strict_json

EXPECTED_SCHEMA = "trillionnium.world.authority.provenance.v2"
EXPECTED_SOURCE_COMMIT = "d44d8930c917b55da7b23eb19e9645feb8f4ee59"
EXPECTED_SOURCE_TREE = "c2b588961b36c549b70910e822f3771a9e4504d9"
EXPECTED_TARGET_WORKSPACE = "trillionnium/crates/world-authority"
EXPECTED_TREE_NAMES = {
    "trnm-world-domain",
    "trnm-world-command",
    "trnm-world-projection",
    "trnm-world-map-provider",
    "trnm-world-ui-fragments",
    "trnm-world-api",
    "trnm-world-server",
}
EXPECTED_ROOT_ENTRIES = {"Cargo.lock", "Cargo.toml", "README.md", *EXPECTED_TREE_NAMES}
EXPECTED_COUNTS = {
    "baseline_trees": 7,
    "baseline_files": 15,
    "current_trees": 7,
    "current_files": 27,
    "exact_historical": 13,
    "reviewed_successor": 2,
    "new_qualification": 12,
}
CLASS_TO_COUNT = {
    "exact_historical_restoration": "exact_historical",
    "reviewed_successor": "reviewed_successor",
    "new_qualification_material": "new_qualification",
}
SHA1_RE = re.compile(r"^[0-9a-f]{40}$")


class ProvenanceError(RuntimeError):
    pass


def run_git(repo: Path, *args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=repo,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise ProvenanceError(
            f"git {' '.join(args)} failed: "
            f"{completed.stderr.strip() or completed.stdout.strip()}"
        )
    return completed.stdout.rstrip("\n")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ProvenanceError(message)


def require_sha(value: Any, label: str) -> str:
    require(
        isinstance(value, str) and SHA1_RE.fullmatch(value) is not None,
        f"{label} is not a Git SHA-1",
    )
    return value


def require_string(value: Any, label: str) -> str:
    require(isinstance(value, str) and bool(value.strip()), f"{label} is missing")
    return value


def require_mapping(value: Any, label: str, expected_count: int) -> dict[str, Any]:
    require(isinstance(value, dict), f"{label} must be an object")
    require(len(value) == expected_count, f"{label} count drift")
    return value


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        document = load_strict_json(path, max_bytes=256 * 1024)
    except (OSError, StrictJsonError) as error:
        raise ProvenanceError(f"cannot read provenance manifest {path}: {error}") from error
    require(isinstance(document, dict), "provenance manifest must be a JSON object")
    return document


def validate_shape(document: dict[str, Any]) -> None:
    require(document.get("schema") == EXPECTED_SCHEMA, "provenance schema mismatch")
    require(document.get("status") == "active", "provenance manifest must be active")
    require(
        document.get("source_repository") == "TrillionniumFoundation/Trillionnium-World",
        "source repository mismatch",
    )
    require(document.get("source_commit") == EXPECTED_SOURCE_COMMIT, "source commit mismatch")
    require(document.get("source_commit_tree") == EXPECTED_SOURCE_TREE, "source tree mismatch")
    require(document.get("target_workspace") == EXPECTED_TARGET_WORKSPACE, "target workspace mismatch")
    require_sha(document.get("target_workspace_tree"), "target workspace tree")
    require(
        document.get("production_authorization") == "not_granted",
        "provenance must not grant production authorization",
    )

    counts = require_mapping(document.get("counts"), "counts", len(EXPECTED_COUNTS))
    require(counts == EXPECTED_COUNTS, f"count contract drift: {counts}")

    baseline_trees = require_mapping(document.get("baseline_trees"), "baseline_trees", 7)
    current_trees = require_mapping(document.get("current_trees"), "current_trees", 7)
    baseline_files = require_mapping(document.get("baseline_files"), "baseline_files", 15)
    current_files = require_mapping(document.get("current_files"), "current_files", 27)
    require(set(baseline_trees) == EXPECTED_TREE_NAMES, "baseline tree name set drift")
    require(set(current_trees) == EXPECTED_TREE_NAMES, "current tree name set drift")

    seen_source_trees: set[str] = set()
    for name, entry in baseline_trees.items():
        require(isinstance(entry, dict), f"baseline tree entry must be an object: {name}")
        source_path = require_string(entry.get("source_path"), f"source tree path: {name}")
        target_path = require_string(entry.get("target_path"), f"target tree path: {name}")
        require_sha(entry.get("tree"), f"baseline tree: {name}")
        require(source_path.endswith(f"/{name}"), f"source tree/name mismatch: {name}")
        require(target_path == f"{EXPECTED_TARGET_WORKSPACE}/{name}", f"target tree/name mismatch: {name}")
        require(source_path not in seen_source_trees, f"duplicate source tree: {source_path}")
        seen_source_trees.add(source_path)
        require_sha(current_trees[name], f"current tree: {name}")

    seen_source_files: set[str] = set()
    for target_path, entry in baseline_files.items():
        require(isinstance(entry, dict), f"baseline file entry must be an object: {target_path}")
        source_path = require_string(entry.get("source_path"), f"baseline source: {target_path}")
        require_sha(entry.get("blob"), f"baseline blob: {target_path}")
        require(target_path.startswith(f"{EXPECTED_TARGET_WORKSPACE}/"), f"baseline target escapes workspace: {target_path}")
        require(source_path not in seen_source_files, f"duplicate source file: {source_path}")
        seen_source_files.add(source_path)

    classes: Counter[str] = Counter()
    for target_path, entry in current_files.items():
        require(isinstance(entry, dict), f"current file entry must be an object: {target_path}")
        require(target_path.startswith(f"{EXPECTED_TARGET_WORKSPACE}/"), f"current target escapes workspace: {target_path}")
        current_blob = require_sha(entry.get("blob"), f"current blob: {target_path}")
        classification = require_string(entry.get("class"), f"classification: {target_path}")
        reason = require_string(entry.get("reason"), f"reason: {target_path}")
        require(len(reason) <= 512, f"reason is unbounded: {target_path}")
        require(classification in CLASS_TO_COUNT, f"unsupported classification: {classification}")
        classes[classification] += 1
        baseline = baseline_files.get(target_path)
        if classification == "exact_historical_restoration":
            require(baseline is not None, f"exact restoration lacks baseline: {target_path}")
            require(current_blob == baseline["blob"], f"exact restoration bytes differ: {target_path}")
        elif classification == "reviewed_successor":
            require(baseline is not None, f"reviewed successor lacks baseline: {target_path}")
            require(current_blob != baseline["blob"], f"successor did not change bytes: {target_path}")
        else:
            require(baseline is None, f"new material shadows baseline target: {target_path}")

    expected_classes = {
        classification: EXPECTED_COUNTS[count_name]
        for classification, count_name in CLASS_TO_COUNT.items()
    }
    require(dict(classes) == expected_classes, f"classification count drift: {dict(classes)}")
    require(set(baseline_files).issubset(current_files), "historical target missing from current inventory")

    root_entries = document.get("target_root_entries")
    require(isinstance(root_entries, list), "target_root_entries must be a list")
    require(len(root_entries) == len(set(root_entries)), "duplicate target root entry")
    require(set(root_entries) == EXPECTED_ROOT_ENTRIES, "target root inventory drift")
    require_string(document.get("claim_boundary"), "claim_boundary")


def git_file_inventory(repo: Path, revision: str, workspace: str) -> dict[str, tuple[str, str, str]]:
    output = run_git(repo, "ls-tree", "-r", revision, "--", workspace)
    result: dict[str, tuple[str, str, str]] = {}
    for line in output.splitlines():
        if not line:
            continue
        metadata, path = line.split("\t", 1)
        mode, object_type, object_id = metadata.split(" ", 2)
        require(path not in result, f"duplicate Git path: {path}")
        result[path] = (mode, object_type, object_id)
    return result


def validate_git_objects(document: dict[str, Any], repo: Path) -> None:
    source_commit = document["source_commit"]
    run_git(repo, "cat-file", "-e", f"{source_commit}^{{commit}}")
    require(
        run_git(repo, "rev-parse", f"{source_commit}^{{tree}}") == document["source_commit_tree"],
        "historical source commit tree drift",
    )
    require(run_git(repo, "rev-parse", "HEAD") != source_commit, "wrapper must be a distinct commit")

    target_workspace = document["target_workspace"]
    actual_workspace_tree = run_git(repo, "rev-parse", f"HEAD:{target_workspace}")
    require(
        actual_workspace_tree == document["target_workspace_tree"],
        f"current workspace tree drift: expected={document['target_workspace_tree']} actual={actual_workspace_tree}",
    )
    actual_root = {
        line for line in run_git(repo, "ls-tree", "--name-only", f"HEAD:{target_workspace}").splitlines() if line
    }
    require(actual_root == set(document["target_root_entries"]), "target root inventory mismatch")

    for name, baseline in document["baseline_trees"].items():
        source_object = run_git(repo, "rev-parse", f"{source_commit}:{baseline['source_path']}")
        target_object = run_git(repo, "rev-parse", f"HEAD:{baseline['target_path']}")
        require(source_object == baseline["tree"], f"historical source tree drift: {name}")
        require(target_object == document["current_trees"][name], f"current successor tree drift: {name}")

    for target_path, baseline in document["baseline_files"].items():
        source_object = run_git(repo, "rev-parse", f"{source_commit}:{baseline['source_path']}")
        require(source_object == baseline["blob"], f"historical source blob drift: {baseline['source_path']}")

    actual_files = git_file_inventory(repo, "HEAD", target_workspace)
    expected_files = document["current_files"]
    require(
        set(actual_files) == set(expected_files),
        f"current file inventory mismatch: missing={sorted(set(expected_files)-set(actual_files))} "
        f"extra={sorted(set(actual_files)-set(expected_files))}",
    )
    for path, entry in expected_files.items():
        mode, object_type, object_id = actual_files[path]
        require(mode == "100644", f"unexpected file mode at {path}: {mode}")
        require(object_type == "blob", f"unexpected object type at {path}: {object_type}")
        require(object_id == entry["blob"], f"current blob drift: {path}")


def validate(document: dict[str, Any], repo: Path) -> None:
    validate_shape(document)
    validate_git_objects(document, repo)


def expect_failure(label: str, operation: Callable[[], None]) -> None:
    try:
        operation()
    except ProvenanceError:
        print(f"hostile provenance fixture rejected: {label}")
        return
    raise ProvenanceError(f"hostile provenance fixture was accepted: {label}")


def run_hostile_tests(document: dict[str, Any], repo: Path) -> None:
    omitted = copy.deepcopy(document)
    omitted["current_files"].pop(next(iter(omitted["current_files"])))
    expect_failure("omitted-current-file", lambda: validate_shape(omitted))

    substituted = copy.deepcopy(document)
    first = next(iter(substituted["current_files"].values()))
    first["blob"] = "0" * 40
    expect_failure("substituted-current-blob", lambda: validate(substituted, repo))

    fake_exact = copy.deepcopy(document)
    successor = next(
        entry for entry in fake_exact["current_files"].values()
        if entry["class"] == "reviewed_successor"
    )
    successor["class"] = "exact_historical_restoration"
    expect_failure("changed-bytes-claimed-exact", lambda: validate_shape(fake_exact))

    fake_new = copy.deepcopy(document)
    new_path, new_entry = next(
        (path, entry) for path, entry in fake_new["current_files"].items()
        if entry["class"] == "new_qualification_material"
    )
    fake_new["baseline_files"][new_path] = {
        "source_path": "trillionnium/crates/legacy-game/fabricated",
        "blob": new_entry["blob"],
    }
    expect_failure("new-file-with-fabricated-baseline", lambda: validate_shape(fake_new))

    wrong_tree = copy.deepcopy(document)
    wrong_tree["target_workspace_tree"] = "2" * 40
    expect_failure("substituted-current-workspace-tree", lambda: validate(wrong_tree, repo))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--repo-root", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    repo = (args.repo_root or Path(__file__).resolve().parents[1]).resolve()
    manifest = args.manifest
    if not manifest.is_absolute():
        manifest = repo / manifest
    document = load_manifest(manifest)
    validate(document, repo)
    if args.self_test:
        run_hostile_tests(document, repo)
    print(
        "world authority provenance: immutable history and exact reviewed successor verified; "
        "omission, substitution, false-exactness, fabricated-baseline and tree-drift fixtures rejected"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProvenanceError as error:
        print(f"world authority provenance failure: {error}", file=sys.stderr)
        raise SystemExit(1)
