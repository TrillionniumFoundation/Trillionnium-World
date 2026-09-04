#!/usr/bin/env python3
"""Publish the exact qualified World Plan V4 v13k bytes to a review branch.

Dry-run is the default. Publication requires --publish, --expected-head and the
TRNM_WORLD_IMPORT_TOKEN environment variable. The tool refuses main/master,
never force-updates a ref, and never writes statuses, tags, releases or
deployments. It verifies the immutable artifact, reconstructs its archived Git
tree exactly, expands every directory marker through the qualified patch, and
checks every file's byte/Git identity before any GitHub write.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.error
import urllib.parse
import urllib.request
import zipfile

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
DEFAULT_BRANCH = "fix/world-plan-v4-development-closure-20260831"
WRITE_COUNT = 73
DELETION_COUNT = 2
EXPECTED = {
    "artifact_zip_sha256": "456a181bdc8f8aa248229b044db9eec4f52572ea3b20bca6492907db58d64ef5",
    "candidate_archive_sha256": "4c703f428f9a54262a6c0c1340028d08d7883f25ef437c1d7e221a280f53f071",
    "identity_sha256": "d05b375af8d0d8317a2e6e58b75a594949729203499f6677b43f1ea36ff31110",
    "manifest_sha256": "44cf59478b28e8fde793ca0d705ba3634216a5d388ce7264e74cd0319c41ff6f",
    "patch_sha256": "ba49dba1e7fbf842f146ac399647e188faafcfbd5ce3ad17425ef88850e0199f",
    "qualified_tree": "5e613185f5a2abda42df371f3755e73667717309",
    "source_head": "5605cfb8861aa923f69ff032ddbff7d035bccb0c",
    "source_tree": "928f43b328e5347b07357e41481df1c7e097adca",
    "control_head": "68e9631b3fc3f75f332497f8d0551608bf0e1413",
    "qualification_run": 33452853784,
    "qualification_artifact": 9780499701,
}


class ImportFailure(RuntimeError):
    pass


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def command(*args: str, cwd: Path | None = None, capture: bool = False) -> str:
    result = subprocess.run(
        list(args),
        cwd=cwd,
        check=False,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.PIPE if capture else None,
    )
    if result.returncode:
        detail = ""
        if capture:
            detail = f"\nstdout: {result.stdout[-3000:]}\nstderr: {result.stderr[-3000:]}"
        raise ImportFailure(f"command failed ({result.returncode}): {' '.join(args)}{detail}")
    return result.stdout.strip() if capture else ""


def checked_relative(value: str, label: str) -> str:
    normalized = value.replace("\\", "/")
    path = PurePosixPath(normalized)
    if not normalized or path.is_absolute() or ".." in path.parts or normalized.startswith("./"):
        raise ImportFailure(f"unsafe {label}: {value}")
    return normalized


def safe_extract_zip(archive: Path, destination: Path) -> None:
    with zipfile.ZipFile(archive) as bundle:
        for info in bundle.infolist():
            name = checked_relative(info.filename, "artifact member")
            unix_mode = (info.external_attr >> 16) & 0o170000
            if unix_mode == 0o120000:
                raise ImportFailure(f"artifact symlink is forbidden: {name}")
            if info.file_size > 256 * 1024 * 1024:
                raise ImportFailure(f"oversized artifact member: {name}")
        bundle.extractall(destination)


def safe_extract_tar(archive: Path, destination: Path) -> None:
    with tarfile.open(archive, "r:gz") as bundle:
        members = bundle.getmembers()
        for member in members:
            name = checked_relative(member.name, "candidate archive member")
            if member.issym() or member.islnk() or member.isdev() or member.isfifo():
                raise ImportFailure(f"unsupported candidate archive member: {name}")
            if member.isfile() and member.size > 256 * 1024 * 1024:
                raise ImportFailure(f"oversized candidate archive member: {name}")
        for member in members:
            target = destination / member.name
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            if not member.isfile():
                raise ImportFailure(f"unsupported candidate archive entry: {member.name}")
            target.parent.mkdir(parents=True, exist_ok=True)
            source = bundle.extractfile(member)
            if source is None:
                raise ImportFailure(f"cannot read candidate archive entry: {member.name}")
            with source, target.open("wb") as output:
                shutil.copyfileobj(source, output)
            target.chmod(0o755 if member.mode & 0o111 else 0o644)


def find_one(root: Path, name: str) -> Path:
    matches = list(root.rglob(name))
    if len(matches) != 1:
        raise ImportFailure(f"expected one {name}, found {len(matches)}")
    return matches[0]


def parse_patch_paths(patch: Path) -> list[str]:
    paths: list[str] = []
    seen: set[str] = set()
    prefix = "diff --git a/"
    for line in patch.read_text(encoding="utf-8").splitlines():
        if not line.startswith(prefix):
            continue
        remainder = line[len(prefix) :]
        try:
            before, after = remainder.split(" b/", 1)
        except ValueError as error:
            raise ImportFailure(f"malformed patch path line: {line}") from error
        before = checked_relative(before, "patch source path")
        after = checked_relative(after, "patch target path")
        if before != after:
            raise ImportFailure(f"rename/copy is not allowed in qualified patch: {before} -> {after}")
        if after in seen:
            raise ImportFailure(f"duplicate qualified patch path: {after}")
        seen.add(after)
        paths.append(after)
    if len(paths) != WRITE_COUNT:
        raise ImportFailure(f"qualified patch write count mismatch: {len(paths)}")
    return paths


def classify_manifest(manifest: dict, patch_paths: list[str]) -> tuple[dict[str, dict], set[str]]:
    records = manifest.get("files")
    if not isinstance(records, list):
        raise ImportFailure("artifact manifest files must be a list")

    declared_files: dict[str, dict] = {}
    directory_markers: set[str] = set()
    deletions: set[str] = set()
    for item in records:
        if not isinstance(item, dict):
            raise ImportFailure("artifact manifest entry must be an object")
        path = checked_relative(str(item.get("path", "")), "manifest path")
        status = str(item.get("status", "")).strip()
        if status == "D":
            deletions.add(path)
        elif status == "??" and path.endswith("/"):
            directory_markers.add(path)
        elif status in {"M", "??"}:
            required = {"bytes", "git_blob_sha1", "sha256"}
            if not required.issubset(item):
                raise ImportFailure(f"manifest file metadata is incomplete: {path}")
            declared_files[path] = item
        else:
            raise ImportFailure(f"unsupported manifest status {status!r}: {path}")

    patch_set = set(patch_paths)
    if not set(declared_files).issubset(patch_set):
        raise ImportFailure("manifest file entry is absent from qualified patch")
    for marker in directory_markers:
        if not any(path.startswith(marker) for path in patch_paths):
            raise ImportFailure(f"empty manifest directory marker: {marker}")
    for path in patch_paths:
        if path in declared_files:
            continue
        if not any(path.startswith(marker) for marker in directory_markers):
            raise ImportFailure(f"patch path is outside manifest coverage: {path}")

    if len(deletions) != DELETION_COUNT:
        raise ImportFailure(f"qualified deletion count mismatch: {len(deletions)}")
    if patch_set & deletions:
        raise ImportFailure("qualified write/delete path sets overlap")
    return declared_files, deletions


def github(method: str, path: str, token: str, payload: dict | None = None) -> dict:
    body = None if payload is None else json.dumps(payload, separators=(",", ":")).encode()
    request = urllib.request.Request(
        "https://api.github.com" + path,
        data=body,
        method=method,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "trnm-world-v13k-importer/v2",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            raw = response.read()
            return json.loads(raw) if raw else {}
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", "replace")
        raise ImportFailure(f"GitHub {method} {path} failed {error.code}: {detail[:4000]}") from error


def prepare(artifact_zip: Path, workspace: Path) -> tuple[Path, list[dict]]:
    if digest(artifact_zip) != EXPECTED["artifact_zip_sha256"]:
        raise ImportFailure("artifact ZIP SHA-256 mismatch")

    artifact_root = workspace / "artifact"
    artifact_root.mkdir()
    safe_extract_zip(artifact_zip, artifact_root)
    patch = find_one(artifact_root, "world-v13-source.patch")
    manifest_path = find_one(artifact_root, "manifest.json")
    identity = find_one(artifact_root, "identity.txt")
    candidate_archive = find_one(artifact_root, "world-v13k-candidate-tree.tar.gz")
    checks = {
        patch: EXPECTED["patch_sha256"],
        manifest_path: EXPECTED["manifest_sha256"],
        identity: EXPECTED["identity_sha256"],
        candidate_archive: EXPECTED["candidate_archive_sha256"],
    }
    for path, expected_digest in checks.items():
        if digest(path) != expected_digest:
            raise ImportFailure(f"artifact member SHA-256 mismatch: {path.name}")

    candidate = workspace / "candidate"
    candidate.mkdir()
    safe_extract_tar(candidate_archive, candidate)
    command("git", "init", "-q", str(candidate))
    command("git", "-C", str(candidate), "add", "-f", "-A")
    archived_tree = command("git", "-C", str(candidate), "write-tree", capture=True)
    if archived_tree != EXPECTED["qualified_tree"]:
        raise ImportFailure(f"candidate archive tree mismatch: {archived_tree}")

    patch_paths = parse_patch_paths(patch)
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    declared_files, deletions = classify_manifest(manifest, patch_paths)

    records: list[dict] = []
    for relative in patch_paths:
        path = candidate / relative
        if not path.is_file() or path.is_symlink():
            raise ImportFailure(f"qualified write is not a regular file: {relative}")
        content = path.read_bytes()
        blob = command("git", "-C", str(candidate), "hash-object", "--", relative, capture=True)
        index = command("git", "-C", str(candidate), "ls-files", "-s", "--", relative, capture=True)
        fields = index.split()
        if len(fields) < 4:
            raise ImportFailure(f"candidate index entry is missing: {relative}")
        mode = fields[0]
        if mode not in {"100644", "100755"}:
            raise ImportFailure(f"unsupported mode {mode}: {relative}")
        declared = declared_files.get(relative)
        if declared is not None:
            if blob != declared["git_blob_sha1"]:
                raise ImportFailure(f"manifest Git blob mismatch: {relative}")
            if len(content) != declared["bytes"]:
                raise ImportFailure(f"manifest byte count mismatch: {relative}")
            if hashlib.sha256(content).hexdigest() != declared["sha256"]:
                raise ImportFailure(f"manifest byte mismatch: {relative}")
        records.append({"path": relative, "content": content, "mode": mode, "sha": blob})

    for relative in sorted(deletions):
        if (candidate / relative).exists():
            raise ImportFailure(f"qualified deletion still exists in archive: {relative}")
        records.append({"path": relative, "content": None, "mode": "100644", "sha": None})

    if sum(record["sha"] is not None for record in records) != WRITE_COUNT:
        raise ImportFailure("expanded qualified write count mismatch")
    if sum(record["sha"] is None for record in records) != DELETION_COUNT:
        raise ImportFailure("expanded qualified deletion count mismatch")
    return candidate, records


def publish(repository: str, branch: str, expected_head: str, token: str, records: list[dict]) -> dict:
    encoded = urllib.parse.quote(branch, safe="")
    observed = github("GET", f"/repos/{repository}/branches/{encoded}", token)["commit"]["sha"]
    if observed != expected_head:
        raise ImportFailure(f"target branch moved: expected={expected_head} observed={observed}")

    entries = []
    for record in records:
        if record["sha"] is None:
            entries.append({"path": record["path"], "mode": "100644", "type": "blob", "sha": None})
            continue
        blob = github(
            "POST",
            f"/repos/{repository}/git/blobs",
            token,
            {"content": base64.b64encode(record["content"]).decode("ascii"), "encoding": "base64"},
        )
        if blob["sha"] != record["sha"]:
            raise ImportFailure(f"server blob mismatch: {record['path']}")
        entries.append({"path": record["path"], "mode": record["mode"], "type": "blob", "sha": record["sha"]})

    exact_tree = github(
        "POST",
        f"/repos/{repository}/git/trees",
        token,
        {"base_tree": EXPECTED["source_tree"], "tree": entries},
    )["sha"]
    if exact_tree != EXPECTED["qualified_tree"]:
        raise ImportFailure(f"server qualified tree mismatch: {exact_tree}")

    head_commit = github("GET", f"/repos/{repository}/git/commits/{observed}", token)
    overlay_tree = github(
        "POST",
        f"/repos/{repository}/git/trees",
        token,
        {"base_tree": head_commit["tree"]["sha"], "tree": entries},
    )["sha"]
    if github("GET", f"/repos/{repository}/branches/{encoded}", token)["commit"]["sha"] != observed:
        raise ImportFailure("target branch moved during object import")

    message = f"""fix(world): publish exact qualified Plan V4 direct-source bytes

Source-World-Head: {EXPECTED['source_head']}
Source-World-Tree: {EXPECTED['source_tree']}
Qualification-Control-Head: {EXPECTED['control_head']}
Qualification-Run: {EXPECTED['qualification_run']}
Qualification-Artifact: {EXPECTED['qualification_artifact']}
Artifact-SHA256: {EXPECTED['artifact_zip_sha256']}
Patch-SHA256: {EXPECTED['patch_sha256']}
Qualified-Tree: {EXPECTED['qualified_tree']}

This commit overlays exact qualified source bytes onto the existing review branch.
It grants no deployment, custody, public-online, legal or commercial authorization.
"""
    commit = github(
        "POST",
        f"/repos/{repository}/git/commits",
        token,
        {"message": message, "tree": overlay_tree, "parents": [observed]},
    )["sha"]
    github("PATCH", f"/repos/{repository}/git/refs/heads/{branch}", token, {"sha": commit, "force": False})
    read_back = github("GET", f"/repos/{repository}/branches/{encoded}", token)["commit"]["sha"]
    if read_back != commit:
        raise ImportFailure("final ref read-back mismatch")
    return {"old_head": observed, "new_head": commit, "qualified_tree": exact_tree, "overlay_tree": overlay_tree}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact-zip", required=True, type=Path)
    parser.add_argument("--repository", default=REPOSITORY)
    parser.add_argument("--target-branch", default=DEFAULT_BRANCH)
    parser.add_argument("--expected-head")
    parser.add_argument("--publish", action="store_true")
    parser.add_argument("--result", type=Path, default=Path("world-v13k-import-result.json"))
    args = parser.parse_args()

    if args.repository != REPOSITORY:
        raise ImportFailure("repository must be the canonical World repository")
    if args.target_branch in {"main", "master"} or args.target_branch.startswith("tags/"):
        raise ImportFailure("target must be an approved review branch")

    with tempfile.TemporaryDirectory(prefix="trnm-world-v13k-") as temporary:
        _, records = prepare(args.artifact_zip, Path(temporary))
        result = {
            "schema": "trnm_world_v13k_import_result_v2",
            "repository": args.repository,
            "target_branch": args.target_branch,
            "qualified_tree": EXPECTED["qualified_tree"],
            "file_count": sum(record["sha"] is not None for record in records),
            "deletion_count": sum(record["sha"] is None for record in records),
            "published": False,
        }
        if args.publish:
            if not args.expected_head:
                raise ImportFailure("--expected-head is required with --publish")
            token = os.environ.get("TRNM_WORLD_IMPORT_TOKEN", "")
            if not token:
                raise ImportFailure("TRNM_WORLD_IMPORT_TOKEN is required with --publish")
            result.update(publish(args.repository, args.target_branch, args.expected_head, token, records))
            result["published"] = True
        args.result.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print("TRNM_WORLD_V13K_IMPORT_PUBLISH=PASS" if args.publish else "TRNM_WORLD_V13K_IMPORT_DRY_RUN=PASS")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ImportFailure, OSError, ValueError, json.JSONDecodeError, tarfile.TarError) as error:
        print(f"import failed closed: {error}", file=sys.stderr)
        raise SystemExit(1)
