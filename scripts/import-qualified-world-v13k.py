#!/usr/bin/env python3
"""Publish the exact qualified World Plan V4 v13k bytes to a review branch.

Dry-run is the default. Publication requires --publish, --expected-head and the
TRNM_WORLD_IMPORT_TOKEN environment variable. The tool refuses main/master,
never force-updates a ref, and never writes statuses, tags, releases or
 deployments. It first reproduces the qualified tree locally, then verifies
every manifest blob before any GitHub write.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys
import tempfile
import urllib.error
import urllib.parse
import urllib.request
import zipfile

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
DEFAULT_BRANCH = "fix/world-plan-v4-development-closure-20260831"
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
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


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


def safe_extract(archive: Path, destination: Path) -> None:
    with zipfile.ZipFile(archive) as bundle:
        for info in bundle.infolist():
            name = info.filename.replace("\\", "/")
            member = PurePosixPath(name)
            if member.is_absolute() or ".." in member.parts:
                raise ImportFailure(f"unsafe artifact member: {name}")
            if info.file_size > 256 * 1024 * 1024:
                raise ImportFailure(f"oversized artifact member: {name}")
        bundle.extractall(destination)


def find_one(root: Path, name: str) -> Path:
    matches = list(root.rglob(name))
    if len(matches) != 1:
        raise ImportFailure(f"expected one {name}, found {len(matches)}")
    return matches[0]


def github(method: str, path: str, token: str, payload: dict | None = None) -> dict:
    body = None if payload is None else json.dumps(payload, separators=(",", ":")).encode()
    request = urllib.request.Request(
        "https://api.github.com" + path,
        data=body,
        method=method,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "trnm-world-v13k-importer/v1",
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
    safe_extract(artifact_zip, artifact_root)
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
    command("git", "init", "-q", str(candidate))
    command("git", "-C", str(candidate), "remote", "add", "origin", f"https://github.com/{REPOSITORY}.git")
    command("git", "-C", str(candidate), "fetch", "--no-tags", "--depth=1", "origin", EXPECTED["source_head"])
    command("git", "-C", str(candidate), "checkout", "--detach", "FETCH_HEAD")
    if command("git", "-C", str(candidate), "rev-parse", "HEAD", capture=True) != EXPECTED["source_head"]:
        raise ImportFailure("source commit read-back mismatch")
    if command("git", "-C", str(candidate), "rev-parse", "HEAD^{tree}", capture=True) != EXPECTED["source_tree"]:
        raise ImportFailure("source tree read-back mismatch")
    command("git", "-C", str(candidate), "apply", "--binary", "--check", str(patch))
    command("git", "-C", str(candidate), "apply", "--binary", str(patch))
    command("git", "-C", str(candidate), "add", "-A")
    if command("git", "-C", str(candidate), "write-tree", capture=True) != EXPECTED["qualified_tree"]:
        raise ImportFailure("qualified tree mismatch")

    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    records: list[dict] = []
    for item in manifest["files"]:
        relative = item["path"]
        path = candidate / relative
        record = {"path": relative, "status": item["status"]}
        if path.is_file():
            content = path.read_bytes()
            blob = command("git", "-C", str(candidate), "hash-object", "--", relative, capture=True)
            if blob != item["git_blob_sha1"]:
                raise ImportFailure(f"Git blob mismatch: {relative}")
            if len(content) != item["bytes"] or hashlib.sha256(content).hexdigest() != item["sha256"]:
                raise ImportFailure(f"manifest byte mismatch: {relative}")
            mode = command("git", "-C", str(candidate), "ls-files", "-s", "--", relative, capture=True).split()[0]
            if mode not in {"100644", "100755"}:
                raise ImportFailure(f"unsupported mode {mode}: {relative}")
            record.update({"content": content, "mode": mode, "sha": blob})
        else:
            record.update({"content": None, "mode": "100644", "sha": None})
        records.append(record)
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
            "schema": "trnm_world_v13k_import_result_v1",
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


try:
    raise SystemExit(main())
except (ImportFailure, OSError, ValueError, json.JSONDecodeError) as error:
    print(f"import failed closed: {error}", file=sys.stderr)
    raise SystemExit(1)
