#!/usr/bin/env python3
"""Bind a local CI checkout to its GitHub event; never certify test/release results."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import stat
import sys
import tempfile
from typing import Any, Mapping

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
REPOSITORY_ID = 1323087277
BRANCH = "fix/world-plan-v4-development-closure-20260831"
MAX_EVENT_BYTES = 2 * 1024 * 1024
MAX_GIT_BYTES = 1024 * 1024
MAX_TRACKED_FILES = 50_000
MAX_TRACKED_FILE_BYTES = 64 * 1024 * 1024
MAX_TRACKED_BYTES = 512 * 1024 * 1024
HEX = re.compile(r"[0-9a-f]{40}")
LANE = re.compile(r"(?:feature|fix|chore|docs|test)/world-[a-z0-9][a-z0-9._-]*")


class TargetFailure(ValueError):
    pass


def require(ok: bool, message: str) -> None:
    if not ok:
        raise TargetFailure(message)


def object_value(value: Any, label: str) -> dict:
    require(isinstance(value, dict), f"{label} must be an object")
    return value


def oid(value: Any, label: str) -> str:
    require(isinstance(value, str) and HEX.fullmatch(value) is not None
            and value != "0" * 40, f"invalid {label}")
    return value


def positive(value: Any, label: str) -> int:
    require(type(value) is int and 0 < value < 2**63, f"invalid {label}")
    return value


def repository(value: Any) -> None:
    data = object_value(value, "repository")
    require(data.get("full_name") == REPOSITORY, "crossed repository")
    require(positive(data.get("id"), "repository id") == REPOSITORY_ID, "crossed repository id")


def read_event(path: Path) -> tuple[dict, str]:
    require(path.is_file() and not path.is_symlink(), "event file missing or linked")
    with path.open("rb") as handle:
        raw = handle.read(MAX_EVENT_BYTES + 1)
    require(0 < len(raw) <= MAX_EVENT_BYTES, "event file empty or oversized")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "duplicate event JSON key")
            result[key] = value
        return result

    def nonfinite(_value):
        raise TargetFailure("non-finite event JSON value")

    event = json.loads(raw.decode("utf-8"), object_pairs_hook=unique, parse_constant=nonfinite)
    return object_value(event, "event"), hashlib.sha256(raw).hexdigest()


def git(root: Path, *arguments: str) -> str:
    # No network commands, credential reads, hooks, replacement objects or index refresh.
    environment = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    environment.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull,
                       GIT_NO_REPLACE_OBJECTS="1", GIT_OPTIONAL_LOCKS="0",
                       GIT_TERMINAL_PROMPT="0")
    with tempfile.TemporaryFile() as output, tempfile.TemporaryFile() as errors:
        result = subprocess.run(
            ["git", "--no-replace-objects", "-c", "core.fsmonitor=false",
             "-c", "core.untrackedCache=false", "-C", str(root), *arguments],
            env=environment, stdout=output, stderr=errors, check=False, timeout=30,
        )
        require(result.returncode == 0, "Git identity inspection failed")
        require(output.tell() <= MAX_GIT_BYTES, "Git identity output exceeds budget")
        output.seek(0)
        return output.read().decode("utf-8").strip()


def commit_headers(raw: str) -> tuple[str, list[str]]:
    headers = raw.split("\n\n", 1)[0].splitlines()
    trees = [line[5:] for line in headers if line.startswith("tree ")]
    parents = [line[7:] for line in headers if line.startswith("parent ")]
    require(len(trees) == 1, "commit must contain one tree")
    tree = oid(trees[0], "commit tree")
    for parent in parents:
        oid(parent, "commit parent")
    require(len(parents) == len(set(parents)), "duplicate commit parent")
    return tree, parents


def tracked_entries(raw: str, *, index: bool = False) -> dict[str, tuple[str, str]]:
    """Read NUL-delimited Git inventories without filename quoting or filters."""
    require(bool(raw) and raw.endswith("\0"), "empty or truncated tracked inventory")
    result: dict[str, tuple[str, str]] = {}
    for row in raw.split("\0")[:-1]:
        require(len(result) < MAX_TRACKED_FILES, "tracked file count exceeds budget")
        metadata, separator, relative = row.partition("\t")
        fields = metadata.split(" ")
        require(bool(separator) and len(fields) == 3, "malformed tracked inventory")
        mode, kind, digest = fields if not index else (fields[0], fields[2], fields[1])
        require(mode in {"100644", "100755"} and kind == ("0" if index else "blob"),
                "tracked inventory contains a link, submodule, sparse entry or conflict")
        oid(digest, "tracked blob")
        path = PurePosixPath(relative)
        require(bool(path.parts) and not path.is_absolute() and path.as_posix() == relative
                and not any(part in {"..", ".git"} for part in path.parts)
                and "\\" not in relative and ":" not in relative,
                "unsafe tracked path")
        require(relative not in result, "duplicate tracked path")
        result[relative] = (mode, digest)
    return result


def index_flags(raw: str, expected: set[str]) -> None:
    require(bool(raw) and raw.endswith("\0"), "empty or truncated index flags")
    paths: set[str] = set()
    for row in raw.split("\0")[:-1]:
        # -v lowercases assume-unchanged; S denotes skip-worktree. A complete
        # validation checkout must use ordinary cached entries, not sparse or
        # assumed-clean entries. Never clear flags or refresh the index here.
        require(row.startswith("H "), "index hides tracked worktree state")
        relative = row[2:]
        require(relative in expected and relative not in paths, "index flag path mismatch")
        paths.add(relative)
    require(paths == expected, "index flag inventory is incomplete")


def file_identity(value: os.stat_result) -> tuple:
    return (value.st_dev, value.st_ino, value.st_mode, value.st_size,
            value.st_mtime_ns, value.st_ctime_ns, value.st_nlink)


def worktree_blob(root_fd: int, relative: str, remaining: int) -> tuple[str, str, int]:
    """Hash actual regular-file bytes via no-follow directory-relative opens.

    Ignore Git stat shortcuts, clean/smudge filters and core.filemode. The CI
    contract is raw byte identity, not a normalized or assumed-clean projection.
    """
    directory = os.dup(root_fd)
    try:
        parts = PurePosixPath(relative).parts
        for part in parts[:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                            dir_fd=directory)
            os.close(directory)
            directory = child
        fd = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
                     dir_fd=directory)
        try:
            before = os.fstat(fd)
            require(stat.S_ISREG(before.st_mode), "tracked path is not a regular file")
            require(before.st_size <= min(MAX_TRACKED_FILE_BYTES, remaining),
                    "tracked file or aggregate bytes exceed budget")
            digest = hashlib.sha1(b"blob " + str(before.st_size).encode("ascii") + b"\0",
                                  usedforsecurity=False)
            size = 0
            while True:
                chunk = os.read(fd, 64 * 1024)
                if not chunk:
                    break
                size += len(chunk)
                require(size <= min(MAX_TRACKED_FILE_BYTES, remaining, before.st_size),
                        "tracked file grew or exceeded budget during inspection")
                digest.update(chunk)
            require(size == before.st_size and file_identity(before) == file_identity(os.fstat(fd)),
                    "tracked file changed during inspection")
            mode = "100755" if before.st_mode & 0o111 else "100644"
            return mode, digest.hexdigest(), size
        finally:
            os.close(fd)
    except OSError as error:
        raise TargetFailure("tracked file is missing, linked or unreadable") from error
    finally:
        os.close(directory)


def verify_tracked_checkout(root: Path, commit: str) -> dict:
    """Bind HEAD, every stage-zero index entry, and raw tracked worktree bytes."""
    require(os.name == "posix" and hasattr(os, "O_NOFOLLOW")
            and hasattr(os, "O_DIRECTORY") and os.open in os.supports_dir_fd,
            "raw checkout verification requires POSIX no-follow opens")
    tree = tracked_entries(git(root, "ls-tree", "-r", "--full-tree", "-z", commit))
    index = git(root, "ls-files", "--stage", "-z")
    flags = git(root, "ls-files", "-v", "-z")
    require(tracked_entries(index, index=True) == tree, "index does not exactly match HEAD")
    index_flags(flags, set(tree))
    root_fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    size = 0
    try:
        for relative, expected in tree.items():
            mode, digest, count = worktree_blob(root_fd, relative, MAX_TRACKED_BYTES - size)
            require((mode, digest) == expected, "raw tracked worktree mode/blob differs from HEAD")
            size += count
    finally:
        os.close(root_fd)
    require(git(root, "ls-files", "--stage", "-z") == index
            and git(root, "ls-files", "-v", "-z") == flags,
            "index changed during raw checkout inspection")
    return {"tracked_files_hashed": len(tree), "tracked_bytes_hashed": size,
            "index_matches_head": True, "tracked_blob_bytes_match_head": True}


def verify(root: Path, event: dict, env: Mapping[str, str], requested_role: str) -> dict:
    require(requested_role in {"event", "head", "merge"}, "unsupported requested role")
    event = object_value(event, "event")
    repository(event.get("repository"))
    require(env.get("GITHUB_REPOSITORY") == REPOSITORY, "crossed environment repository")
    require(env.get("GITHUB_REPOSITORY_ID") == str(REPOSITORY_ID), "crossed environment repository id")
    event_name = env.get("GITHUB_EVENT_NAME")
    require(event_name in {"pull_request", "push", "workflow_dispatch"}, "unsupported event; privileged PR-target execution is forbidden")
    event_sha = oid(env.get("GITHUB_SHA"), "event SHA")
    ref = env.get("GITHUB_REF")
    require(isinstance(ref, str), "event ref missing")
    head = base = None
    number = None

    if event_name == "pull_request":
        pr = object_value(event.get("pull_request"), "pull request")
        number = positive(event.get("number"), "pull request number")
        require(positive(pr.get("number"), "nested pull request number") == number, "crossed pull request number")
        require(pr.get("state") == "open", "pull request is not open")
        head_data = object_value(pr.get("head"), "pull request head")
        base_data = object_value(pr.get("base"), "pull request base")
        repository(head_data.get("repo"))
        repository(base_data.get("repo"))
        head = oid(head_data.get("sha"), "pull request head")
        base = oid(base_data.get("sha"), "pull request base")
        head_ref = head_data.get("ref")
        require(isinstance(head_ref, str) and LANE.fullmatch(head_ref) is not None, "head branch violates World lane")
        require(base_data.get("ref") == "main", "unexpected pull request base branch")
        require(env.get("GITHUB_HEAD_REF") == head_ref and env.get("GITHUB_BASE_REF") == "main", "event branch metadata disagrees")
        require(ref == f"refs/pull/{number}/merge", "not the pull request merge ref")
        require(event_sha not in {head, base}, "PR event SHA is not a prospective merge")
        role = "head" if requested_role == "head" else "prospective_merge"
        expected = head if role == "head" else event_sha
    else:
        require(requested_role != "merge", "a non-PR event cannot establish prospective-merge evidence")
        require(ref in {"refs/heads/main", f"refs/heads/{BRANCH}"}, "unapproved push/dispatch branch or tag")
        require(not env.get("GITHUB_HEAD_REF") and not env.get("GITHUB_BASE_REF"), "unexpected PR branch metadata")
        if event_name == "push":
            require(event.get("deleted") is False, "deleted or malformed push event")
            require(event.get("ref") == ref and oid(event.get("after"), "push after") == event_sha, "push ref/SHA disagreement")
            role = "head"
        else:
            # Dispatch inputs cannot override the runner-provided revision.
            if "ref" in event:
                require(event["ref"] in {ref, ref.removeprefix("refs/heads/")}, "dispatch ref disagreement")
            role = "dispatched_head"
        expected = event_sha

    require(root.is_dir() and not root.is_symlink(), "checkout root missing or linked")
    root = root.resolve()
    require(Path(git(root, "rev-parse", "--show-toplevel")).resolve() == root, "not the checkout root")
    origin = git(root, "config", "--get", "remote.origin.url")
    require(origin in {f"https://github.com/{REPOSITORY}", f"https://github.com/{REPOSITORY}.git",
                       f"git@github.com:{REPOSITORY}.git"}, "crossed checkout origin")
    actual = oid(git(root, "rev-parse", "--verify", "HEAD^{commit}"), "checkout commit")
    require(actual == expected, "checkout does not match the requested event role")
    tree, parents = commit_headers(git(root, "cat-file", "-p", actual))
    if role == "prospective_merge":
        require(parents == [base, head], "prospective merge parents do not exactly equal event base/head")
    require(git(root, "show", f"{actual}:PROJECT_ID") == "trillionnium-world", "crossed committed project identity")
    tracked = verify_tracked_checkout(root, actual)
    require(not git(root, "ls-files", "--others", "--exclude-standard", "-z"), "checkout has untracked changes")
    require(git(root, "rev-parse", "--verify", "HEAD^{commit}") == actual, "HEAD moved during inspection")

    run = env.get("GITHUB_RUN_ID", "")
    attempt = env.get("GITHUB_RUN_ATTEMPT", "")
    job = env.get("GITHUB_JOB", "")
    require(re.fullmatch(r"[1-9][0-9]{0,18}", run) is not None, "invalid workflow run id")
    require(re.fullmatch(r"[1-9][0-9]{0,8}", attempt) is not None, "invalid workflow run attempt")
    require(re.fullmatch(r"[A-Za-z_][A-Za-z0-9_-]{0,99}", job) is not None, "invalid workflow job id")
    return {
        **tracked,
        "schema": "trnm_world_ci_target_identity_v1", "repository": REPOSITORY,
        "event": event_name, "requested_role": requested_role, "role": role,
        "event_sha": event_sha, "checkout_commit": actual, "checkout_tree": tree,
        "commit_parents": parents, "pull_request": number, "event_base": base, "event_head": head,
        "workflow_run_id": run, "workflow_run_attempt": attempt, "workflow_job": job,
        "identity_matches_event": True, "tests_verified": False,
        "remote_evidence_verified": False, "production_authorization": "not_granted",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--role", choices=("event", "head", "merge"), default="event")
    args = parser.parse_args()
    try:
        path = os.environ.get("GITHUB_EVENT_PATH")
        require(bool(path), "GITHUB_EVENT_PATH missing")
        event, digest = read_event(Path(path))
        result = verify(args.root, event, os.environ, args.role)
        result["event_payload_sha256"] = digest
    except (TargetFailure, OSError, ValueError, TypeError, RecursionError, subprocess.TimeoutExpired) as error:
        print(f"TRNM CI target identity: FAIL: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
