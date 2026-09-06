#!/usr/bin/env python3
"""Verify the exact historical provenance of the bounded World authority workspace."""

from __future__ import annotations

import argparse
import copy
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any, Callable

EXPECTED_SCHEMA = "trillionnium.world.authority.provenance.v1"
EXPECTED_SOURCE_COMMIT = "d44d8930c917b55da7b23eb19e9645feb8f4ee59"
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
EXPECTED_ROOT_ENTRIES = {
    "Cargo.lock",
    "Cargo.toml",
    "README.md",
    *EXPECTED_TREE_NAMES,
}
SHA1_RE = re.compile(r"^[0-9a-f]{40}$")


class ProvenanceError(RuntimeError):
    """Raised when a provenance claim cannot be proved exactly."""


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
            f"git {' '.join(args)} failed: {completed.stderr.strip() or completed.stdout.strip()}"
        )
    return completed.stdout.strip()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ProvenanceError(message)


def require_sha(value: Any, label: str) -> str:
    require(isinstance(value, str) and SHA1_RE.fullmatch(value) is not None, f"{label} is not a Git SHA-1")
    return value


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
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
    require(document.get("target_workspace") == EXPECTED_TARGET_WORKSPACE, "target workspace mismatch")
    require(
        document.get("production_authorization") == "not_granted",
        "source provenance must not grant production authorization",
    )

    trees = document.get("restored_trees")
    files = document.get("restored_files")
    root_entries = document.get("target_root_entries")
    require(isinstance(trees, list), "restored_trees must be a list")
    require(isinstance(files, list), "restored_files must be a list")
    require(isinstance(root_entries, list), "target_root_entries must be a list")
    require(len(trees) == 7, "exactly seven restored trees are required")
    require(len(files) == 15, "exactly fifteen restored files are required")
    require(document.get("restored_tree_count") == len(trees), "restored_tree_count drift")
    require(document.get("restored_file_count") == len(files), "restored_file_count drift")
    require(set(root_entries) == EXPECTED_ROOT_ENTRIES, "target root inventory drift")
    require(len(root_entries) == len(set(root_entries)), "duplicate target root entry")

    tree_names: set[str] = set()
    tree_targets: set[str] = set()
    tree_sources: set[str] = set()
    for entry in trees:
        require(isinstance(entry, dict), "restored tree entry must be an object")
        name = entry.get("name")
        source_path = entry.get("source_path")
        target_path = entry.get("target_path")
        require(isinstance(name, str), "restored tree name missing")
        require(isinstance(source_path, str), f"restored tree source path missing: {name}")
        require(isinstance(target_path, str), f"restored tree target path missing: {name}")
        require_sha(entry.get("git_object"), f"restored tree object: {name}")
        require(source_path.endswith(f"/{name}"), f"source tree/name mismatch: {name}")
        require(target_path == f"{EXPECTED_TARGET_WORKSPACE}/{name}", f"target tree/name mismatch: {name}")
        tree_names.add(name)
        tree_sources.add(source_path)
        tree_targets.add(target_path)
    require(tree_names == EXPECTED_TREE_NAMES, "restored tree name set drift")
    require(len(tree_sources) == len(trees), "duplicate restored source tree")
    require(len(tree_targets) == len(trees), "duplicate restored target tree")

    file_sources: set[str] = set()
    file_targets: set[str] = set()
    for entry in files:
        require(isinstance(entry, dict), "restored file entry must be an object")
        source_path = entry.get("source_path")
        target_path = entry.get("target_path")
        require(isinstance(source_path, str), "restored file source path missing")
        require(isinstance(target_path, str), "restored file target path missing")
        require_sha(entry.get("git_blob"), f"restored file blob: {target_path}")
        require(
            any(target_path.startswith(f"{tree}/") for tree in tree_targets),
            f"restored file escapes listed trees: {target_path}",
        )
        file_sources.add(source_path)
        file_targets.add(target_path)
    require(len(file_sources) == len(files), "duplicate restored source file")
    require(len(file_targets) == len(files), "duplicate restored target file")


def validate_git_objects(document: dict[str, Any], repo: Path) -> None:
    source_commit = document["source_commit"]
    run_git(repo, "cat-file", "-e", f"{source_commit}^{{commit}}")
    require(run_git(repo, "rev-parse", "HEAD") != source_commit, "qualification wrapper must be a distinct commit")

    target_workspace = document["target_workspace"]
    actual_root = set(run_git(repo, "ls-tree", "--name-only", f"HEAD:{target_workspace}").splitlines())
    expected_root = set(document["target_root_entries"])
    require(actual_root == expected_root, f"target root inventory mismatch: expected={sorted(expected_root)} actual={sorted(actual_root)}")

    restored_target_files = {entry["target_path"] for entry in document["restored_files"]}
    observed_target_files: set[str] = set()

    for tree in document["restored_trees"]:
        expected = tree["git_object"]
        source_object = run_git(repo, "rev-parse", f"{source_commit}:{tree['source_path']}")
        target_object = run_git(repo, "rev-parse", f"HEAD:{tree['target_path']}")
        require(source_object == expected, f"historical source tree drift: {tree['name']} expected={expected} actual={source_object}")
        require(target_object == expected, f"restored target tree drift: {tree['name']} expected={expected} actual={target_object}")
        files_in_tree = set(
            line
            for line in run_git(repo, "ls-tree", "-r", "--name-only", "HEAD", "--", tree["target_path"]).splitlines()
            if line
        )
        expected_files_in_tree = {
            path for path in restored_target_files if path.startswith(f"{tree['target_path']}/")
        }
        require(
            files_in_tree == expected_files_in_tree,
            f"restored tree file inventory mismatch: {tree['name']} expected={sorted(expected_files_in_tree)} actual={sorted(files_in_tree)}",
        )
        observed_target_files.update(files_in_tree)

    require(observed_target_files == restored_target_files, "restored file coverage is incomplete")

    for entry in document["restored_files"]:
        expected = entry["git_blob"]
        source_object = run_git(repo, "rev-parse", f"{source_commit}:{entry['source_path']}")
        target_object = run_git(repo, "rev-parse", f"HEAD:{entry['target_path']}")
        require(source_object == expected, f"historical source blob drift: {entry['source_path']}")
        require(target_object == expected, f"restored target blob drift: {entry['target_path']}")


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
    omitted["restored_files"].pop()
    omitted["restored_file_count"] = len(omitted["restored_files"])
    expect_failure("omitted-restored-file", lambda: validate_shape(omitted))

    substituted = copy.deepcopy(document)
    substituted["restored_files"][0]["git_blob"] = "0" * 40
    expect_failure("substituted-restored-blob", lambda: validate(substituted, repo))

    extra = copy.deepcopy(document)
    extra["restored_files"].append(
        {
            "source_path": "trillionnium/crates/legacy-game/trnm-world-domain/src/hostile.rs",
            "target_path": f"{EXPECTED_TARGET_WORKSPACE}/trnm-world-domain/src/hostile.rs",
            "git_blob": "1" * 40,
        }
    )
    extra["restored_file_count"] = len(extra["restored_files"])
    expect_failure("extra-restored-file", lambda: validate_shape(extra))

    omitted_tree = copy.deepcopy(document)
    omitted_tree["restored_trees"].pop()
    omitted_tree["restored_tree_count"] = len(omitted_tree["restored_trees"])
    expect_failure("omitted-restored-tree", lambda: validate_shape(omitted_tree))


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
        "world authority provenance: exact historical trees/files verified; "
        "omission, substitution and extra-file fixtures rejected"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProvenanceError as error:
        print(f"world authority provenance failure: {error}", file=sys.stderr)
        raise SystemExit(1)
