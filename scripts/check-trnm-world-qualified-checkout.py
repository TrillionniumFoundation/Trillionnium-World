#!/usr/bin/env python3
"""Verify qualified paths in a committed checkout, without remote/CI promotion.

The pinned artifact is verified by the existing v13k importer before any path
comparison. This command never imports objects, changes source, updates refs,
contacts GitHub, or treats an uncommitted worktree as published source.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
import tarfile
import zipfile
import tempfile

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
MAX_GIT_BYTES = 16 * 1024 * 1024
MAX_FILE_BYTES = 16 * 1024 * 1024


class CheckoutFailure(ValueError):
    pass


def check(condition: bool, message: str) -> None:
    if not condition:
        raise CheckoutFailure(message)


def safe_path(root: Path, relative: str) -> Path:
    p = PurePosixPath(relative)
    check(bool(p.parts) and p.as_posix() == relative and not p.is_absolute()
          and not any(x in {"..", ".git"} for x in p.parts)
          and "\\" not in relative and ":" not in relative and "\0" not in relative,
          "unsafe qualified path")
    cursor = root
    for part in p.parts:
        cursor /= part
        check(not cursor.is_symlink(), "qualified path traverses a symlink")
    return cursor


def git(root: Path, *args: str) -> bytes:
    # Inspection only. Do not read user-level command aliases or external diff
    # configuration, and do not permit replacement objects to redefine HEAD.
    environment = {**os.environ, "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": os.devnull,
                   "GIT_NO_REPLACE_OBJECTS": "1", "GIT_TERMINAL_PROMPT": "0"}
    for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_OBJECT_DIRECTORY",
                "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_CONFIG_PARAMETERS", "GIT_CONFIG_COUNT"):
        environment.pop(key, None)
    try:
        result = subprocess.run(["git", "-C", str(root), *args], env=environment, check=False,
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=60)
    except (OSError, subprocess.TimeoutExpired) as error:
        raise CheckoutFailure("Git checkout inspection unavailable") from error
    check(result.returncode == 0 and len(result.stdout) <= MAX_GIT_BYTES,
          "Git checkout inspection failed or exceeded budget")
    return result.stdout


def parse_tree(raw: bytes) -> dict[str, tuple[str, str, str]]:
    check(len(raw) <= MAX_GIT_BYTES, "Git tree resource budget exceeded")
    entries = {}
    check(not raw or raw.endswith(b"\0"), "truncated Git tree output")
    for record in raw.split(b"\0")[:-1]:
        try:
            metadata, path = record.split(b"\t", 1)
            mode, kind, sha = metadata.decode("ascii").split(" ")
            relative = path.decode("utf-8")
        except (ValueError, UnicodeError) as error:
            raise CheckoutFailure("malformed Git tree record") from error
        check(relative not in entries and re.fullmatch(r"[0-9a-f]{40}", sha) is not None,
              "duplicate/invalid Git tree record")
        entries[relative] = (mode, kind, sha)
    return entries


def verify_checkout(root: Path, records: list[dict], expected_head: str | None = None) -> dict:
    root = root.resolve()
    check(Path(git(root, "rev-parse", "--show-toplevel").decode().strip()).resolve() == root,
          "root is not the Git checkout root")
    origin = git(root, "config", "--get", "remote.origin.url").decode().strip()
    check(origin in {f"https://github.com/{REPOSITORY}.git", f"https://github.com/{REPOSITORY}",
                     f"git@github.com:{REPOSITORY}.git"}, "crossed repository origin")
    head = git(root, "rev-parse", "--verify", "HEAD^{commit}").decode().strip()
    check(re.fullmatch(r"[0-9a-f]{40}", head) is not None, "invalid checkout commit")
    if expected_head is not None:
        check(re.fullmatch(r"[0-9a-f]{40}", expected_head) is not None and head == expected_head,
              "checkout does not match expected exact head")
    check(git(root, "show", f"{head}:PROJECT_ID").decode().strip() == "trillionnium-world", "crossed project")
    tree = git(root, "rev-parse", f"{head}^{{tree}}").decode().strip()
    entries = parse_tree(git(root, "ls-tree", "-r", "-z", head))
    check(bool(records), "empty qualified record set")
    seen = set()
    failures = []
    writes = deletions = 0
    for record in records:
        relative = record["path"]
        path = safe_path(root, relative)
        check(relative not in seen, "duplicate qualified path")
        seen.add(relative)
        sha = record["sha"]
        if sha is None:
            deletions += 1
            if relative in entries or path.exists():
                failures.append(relative + ": required deletion remains")
            continue
        writes += 1
        check(re.fullmatch(r"[0-9a-f]{40}", sha) is not None and record["mode"] in {"100644", "100755"},
              "invalid qualified record identity")
        if entries.get(relative) != (record["mode"], "blob", sha):
            failures.append(relative + ": committed mode/blob mismatch")
        if not path.is_file():
            failures.append(relative + ": worktree file missing")
            continue
        check(path.stat().st_size <= MAX_FILE_BYTES, "qualified worktree file exceeds budget")
        with path.open("rb") as handle:
            data = handle.read(MAX_FILE_BYTES + 1)
        check(len(data) <= MAX_FILE_BYTES, "qualified worktree file exceeds budget")
        actual = hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()
        mode = "100755" if path.stat().st_mode & 0o111 else "100644"
        if actual != sha or mode != record["mode"]:
            failures.append(relative + ": worktree mode/blob mismatch")
    check(git(root, "rev-parse", "--verify", "HEAD^{commit}").decode().strip() == head,
          "HEAD moved during checkout verification")
    check(not failures, "qualified checkout mismatch: " + "; ".join(failures[:12]))
    return {"schema": "trnm_world_qualified_checkout_verification_v1", "repository": REPOSITORY,
            "checkout_commit": head, "checkout_tree": tree, "qualified_writes": writes,
            "qualified_deletions": deletions, "qualified_paths_match_committed_and_worktree_bytes": True,
            "remote_branch_publication": "not_proven", "exact_head_ci": "not_proven",
            "independent_review": "not_proven", "production_authorization": "not_granted"}


def load_importer():
    path = Path(__file__).with_name("import-qualified-world-v13k.py")
    check(path.is_file() and not path.is_symlink(), "verified-artifact importer is missing or linked")
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("world_qualified_importer", path)
    check(spec is not None and spec.loader is not None, "cannot load verified-artifact importer")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--artifact-zip", required=True, type=Path)
    parser.add_argument("--expected-head", help="exact locally checked commit; not a remote observation")
    args = parser.parse_args()
    try:
        importer = load_importer()
        with tempfile.TemporaryDirectory(prefix="trnm-qualified-checkout-") as temporary:
            _, records = importer.prepare(args.artifact_zip, Path(temporary))
            importer.validate_records(records)
            result = verify_checkout(args.root, records, args.expected_head)
            result["qualified_artifact_zip_sha256"] = importer.EXPECTED["artifact_zip_sha256"]
            result["qualified_artifact_tree"] = importer.EXPECTED["qualified_tree"]
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0
    except (CheckoutFailure, OSError, ValueError, KeyError, RuntimeError,
            subprocess.TimeoutExpired, tarfile.TarError, zipfile.BadZipFile) as error:
        print(f"TRNM World qualified checkout: FAIL: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
