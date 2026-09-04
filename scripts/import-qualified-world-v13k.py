#!/usr/bin/env python3
"""Verify/import the immutable v13k source artifact without overwriting new work.

Dry-run is the default. --publish is an operator-only action, never a CI step.
Only the existing Plan V4 review branch is permitted. No status, merge, tag,
release, deployment, force update or production authorization is produced.
"""
from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
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
DELETIONS = {
    "trillionnium/crates/trnm-game-server/build.rs",
    "trillionnium/crates/trnm-game-server/src/lib.rs.in",
}
MAX_MEMBER_BYTES = 256 * 1024 * 1024
MAX_ARCHIVE_BYTES = 512 * 1024 * 1024
MAX_API_BYTES = 16 * 1024 * 1024


class ImportFailure(RuntimeError):
    """A checked boundary failed; no promotion may be inferred."""


def digest(path: Path) -> str:
    with path.open("rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()


def blob_sha(content: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(content)).encode() + b"\0" + content).hexdigest()


def checked_relative(value: str, label: str) -> str:
    path = PurePosixPath(value)
    if (not value or not path.parts or ":" in path.parts[0] or "\\" in value or path.is_absolute() or
            any(part in {"..", ".git"} for part in path.parts) or
            path.as_posix() != value.rstrip("/") or "\0" in value):
        raise ImportFailure(f"unsafe {label}: {value!r}")
    return value


def check_members(names_and_sizes: list[tuple[str, int]]) -> None:
    seen: set[str] = set()
    total = 0
    for name, size in names_and_sizes:
        normalized = checked_relative(name, "archive member").rstrip("/")
        if normalized in seen or size < 0 or size > MAX_MEMBER_BYTES:
            raise ImportFailure(f"duplicate or oversized archive member: {name}")
        seen.add(normalized)
        total += size
    if total > MAX_ARCHIVE_BYTES or len(seen) > 100_000:
        raise ImportFailure("archive resource budget exceeded")


def safe_extract_zip(archive: Path, destination: Path) -> None:
    with zipfile.ZipFile(archive) as bundle:
        infos = bundle.infolist()
        check_members([(item.filename, item.file_size) for item in infos])
        for item in infos:
            kind = (item.external_attr >> 16) & 0o170000
            if kind not in {0, 0o040000, 0o100000}:
                raise ImportFailure("artifact special file is forbidden")
        bundle.extractall(destination)


def safe_extract_tar(archive: Path, destination: Path) -> None:
    with tarfile.open(archive, "r:gz") as bundle:
        members = bundle.getmembers()
        check_members([(member.name, member.size) for member in members])
        if any(not (member.isfile() or member.isdir()) for member in members):
            raise ImportFailure("candidate special file/link is forbidden")
        for member in members:
            target = destination / member.name
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            source = bundle.extractfile(member)
            if source is None:
                raise ImportFailure(f"cannot read candidate member: {member.name}")
            with source, target.open("xb") as output:
                while block := source.read(1024 * 1024):
                    output.write(block)
            target.chmod(0o755 if member.mode & 0o111 else 0o644)


def command(*args: str, cwd: Path | None = None, capture: bool = False) -> str:
    environment = {**os.environ, "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": os.devnull}
    result = subprocess.run(args, cwd=cwd, env=environment, check=False, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=120)
    if result.returncode:
        raise ImportFailure(f"command failed: {args[0]}: {result.stderr[-2000:]}")
    return result.stdout.strip() if capture else ""


def parse_patch_paths(patch: Path) -> list[str]:
    paths = []
    for line in patch.read_text(encoding="utf-8").splitlines():
        if not line.startswith("diff --git "):
            continue
        match = re.fullmatch(r"diff --git a/(\S+) b/(\S+)", line)
        if match is None or match[1] != match[2]:
            raise ImportFailure("rename or malformed qualified patch header")
        paths.append(checked_relative(match[1], "patch path"))
    if len(paths) != WRITE_COUNT or len(set(paths)) != WRITE_COUNT:
        raise ImportFailure("qualified patch write count/uniqueness mismatch")
    return paths


def classify_manifest(manifest: dict, patch_paths: list[str]) -> tuple[dict, set[str]]:
    records = manifest.get("files")
    if not isinstance(records, list):
        raise ImportFailure("artifact manifest files must be a list")
    declared, directories, deletions, seen = {}, set(), set(), set()
    for item in records:
        if not isinstance(item, dict) or not isinstance(item.get("path"), str):
            raise ImportFailure("invalid manifest record")
        path = checked_relative(item["path"], "manifest path")
        if path.rstrip("/") in seen:
            raise ImportFailure("duplicate manifest path")
        seen.add(path.rstrip("/"))
        status = str(item.get("status", "")).strip()
        if status == "D":
            deletions.add(path)
        elif status == "??" and path.endswith("/"):
            directories.add(path)
        elif status in {"M", "??"} and {"bytes", "git_blob_sha1", "sha256"} <= item.keys():
            declared[path] = item
        else:
            raise ImportFailure("invalid manifest status or file metadata")
    paths = set(patch_paths)
    if (not set(declared) <= paths or paths & deletions or len(deletions) != DELETION_COUNT or
            any(not any(path.startswith(d) for path in paths) for d in directories)):
        raise ImportFailure("manifest write/delete/directory mismatch")
    if any(path not in declared and not any(path.startswith(d) for d in directories) for path in paths):
        raise ImportFailure("patch path is outside manifest coverage")
    return declared, deletions


def prepare(artifact_zip: Path, workspace: Path) -> tuple[Path, list[dict]]:
    if digest(artifact_zip) != EXPECTED["artifact_zip_sha256"]:
        raise ImportFailure("artifact ZIP SHA-256 mismatch")
    artifact = workspace / "artifact"
    artifact.mkdir()
    safe_extract_zip(artifact_zip, artifact)
    members = {
        "world-v13-source.patch": "patch_sha256", "manifest.json": "manifest_sha256",
        "identity.txt": "identity_sha256", "world-v13k-candidate-tree.tar.gz": "candidate_archive_sha256",
    }
    if {p.name for p in artifact.iterdir()} != set(members) | {"SHA256SUMS"}:
        raise ImportFailure("unexpected artifact layout")
    for name, key in members.items():
        if digest(artifact / name) != EXPECTED[key]:
            raise ImportFailure(f"artifact member SHA-256 mismatch: {name}")
    # SHA256SUMS historically includes its own empty pre-write hash. The fixed
    # independent pins above, not that self-referential line, authorize bytes.
    candidate = workspace / "candidate"
    candidate.mkdir()
    safe_extract_tar(artifact / "world-v13k-candidate-tree.tar.gz", candidate)
    command("git", "init", "-q", "--object-format=sha1", str(candidate))
    command("git", "-C", str(candidate), "-c", "core.autocrlf=false", "add", "-f", "-A")
    tree = command("git", "-C", str(candidate), "write-tree", capture=True)
    if tree != EXPECTED["qualified_tree"]:
        raise ImportFailure(f"candidate archive tree mismatch: {tree}")
    paths = parse_patch_paths(artifact / "world-v13-source.patch")
    declared, deletions = classify_manifest(json.loads((artifact / "manifest.json").read_text()), paths)
    if deletions != DELETIONS:
        raise ImportFailure("qualified deletion identities mismatch")
    records = []
    for relative in paths:
        path = candidate / relative
        data = path.read_bytes()
        sha = blob_sha(data)
        mode = "100755" if path.stat().st_mode & 0o111 else "100644"
        item = declared.get(relative)
        if item and (sha != item["git_blob_sha1"] or len(data) != item["bytes"] or
                     hashlib.sha256(data).hexdigest() != item["sha256"]):
            raise ImportFailure(f"manifest byte mismatch: {relative}")
        records.append({"path": relative, "content": data, "mode": mode, "sha": sha})
    for relative in sorted(deletions):
        if (candidate / relative).exists():
            raise ImportFailure("qualified deletion remains in archive")
        records.append({"path": relative, "content": None, "mode": "100644", "sha": None})
    return candidate, records


def github(method: str, path: str, token: str, payload: dict | None = None) -> dict:
    prefix = f"/repos/{REPOSITORY}/"
    suffix = path.removeprefix(prefix)
    read_paths = {
        "branches/" + urllib.parse.quote(DEFAULT_BRANCH, safe=""),
    }
    allowed = (
        method == "GET" and (suffix in read_paths or re.fullmatch(
            r"git/(commits/[0-9a-f]{40}|trees/[0-9a-f]{40}\?recursive=1)", suffix))
        or method == "POST" and suffix in {"git/blobs", "git/trees", "git/commits"}
        or method == "PATCH" and suffix == "git/refs/heads/" + DEFAULT_BRANCH
    )
    if not path.startswith(prefix) or not allowed:
        raise ImportFailure("unsupported API capability")
    data = None if payload is None else json.dumps(payload, separators=(",", ":")).encode()
    request = urllib.request.Request("https://api.github.com" + path, data=data, method=method,
                                    headers={"Accept": "application/vnd.github+json",
                                             "Authorization": f"Bearer {token}",
                                             "X-GitHub-Api-Version": "2022-11-28",
                                             "User-Agent": "trnm-world-v13k-importer/v3"})
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            raw = response.read(MAX_API_BYTES + 1)
            if len(raw) > MAX_API_BYTES:
                raise ImportFailure("API response resource budget exceeded")
            return json.loads(raw) if raw else {}
    except urllib.error.HTTPError as error:
        # Do not echo server-controlled bodies or headers that could reflect secrets.
        raise ImportFailure(f"GitHub {method} failed HTTP {error.code}") from error
    except urllib.error.URLError as error:
        raise ImportFailure("GitHub transport unavailable; publication not proven") from error


def read_tree(repository: str, tree: str, token: str) -> dict[str, tuple[str, str, str]]:
    result = github("GET", f"/repos/{repository}/git/trees/{tree}?recursive=1", token)
    if result.get("sha") != tree or result.get("truncated") is not False or not isinstance(result.get("tree"), list):
        raise ImportFailure("missing, crossed or truncated Git tree")
    leaves = {}
    seen = set()
    for entry in result["tree"]:
        path = checked_relative(entry["path"], "remote tree path")
        if path in seen:
            raise ImportFailure("duplicate remote tree path")
        seen.add(path)
        if entry["type"] == "tree":
            continue
        if not re.fullmatch(r"[0-9a-f]{40}", entry["sha"]):
            raise ImportFailure("invalid remote tree object identity")
        leaves[path] = (entry["mode"], entry["type"], entry["sha"])
    return leaves


def overlay_expected(base: dict, current: dict, records: list[dict]) -> dict:
    result = dict(current)
    seen = set()
    for record in records:
        path = checked_relative(record["path"], "qualified record path")
        if path in seen:
            raise ImportFailure("duplicate qualified record")
        seen.add(path)
        desired = None if record["sha"] is None else (record["mode"], "blob", record["sha"])
        if current.get(path) not in (base.get(path), desired):
            raise ImportFailure(f"qualified source overlaps changed path: {path}")
        if desired is None:
            result.pop(path, None)
        else:
            result[path] = desired
    return result


def validate_context(repository: str, branch: str, head: str) -> None:
    if repository != REPOSITORY or branch != DEFAULT_BRANCH:
        raise ImportFailure("publication is restricted to the operative World review branch")
    if not re.fullmatch(r"[0-9a-f]{40}", head):
        raise ImportFailure("--expected-head must be an exact commit SHA")
    if any(os.environ.get(key, "").lower() not in {"", "0", "false"} for key in ("GITHUB_ACTIONS", "CI")):
        raise ImportFailure("source publication is forbidden in CI")


def validate_records(records: list[dict]) -> None:
    if (len(records) != WRITE_COUNT + DELETION_COUNT or
            sum(r.get("sha") is not None for r in records) != WRITE_COUNT or
            {r.get("path") for r in records if r.get("sha") is None} != DELETIONS):
        raise ImportFailure("qualified record counts/deletions mismatch")
    paths = set()
    for record in records:
        path = checked_relative(record["path"], "qualified record path")
        if path in paths or record["mode"] not in {"100644", "100755"}:
            raise ImportFailure("duplicate qualified record or invalid mode")
        paths.add(path)
        if record["sha"] is None:
            if record["content"] is not None:
                raise ImportFailure("deletion contains bytes")
        elif not isinstance(record["content"], bytes) or blob_sha(record["content"]) != record["sha"]:
            raise ImportFailure("local record byte identity mismatch")


def publish(repository: str, branch: str, expected_head: str, token: str, records: list[dict]) -> dict:
    validate_context(repository, branch, expected_head)
    validate_records(records)
    if not token:
        raise ImportFailure("publication token is required")
    encoded = urllib.parse.quote(branch, safe="")
    branch_path = f"/repos/{repository}/branches/{encoded}"
    observed = github("GET", branch_path, token)["commit"]["sha"]
    if observed != expected_head:
        raise ImportFailure("target branch moved before object import")
    head_commit = github("GET", f"/repos/{repository}/git/commits/{observed}", token)
    current_tree = head_commit["tree"]["sha"]
    base = read_tree(repository, EXPECTED["source_tree"], token)
    current = read_tree(repository, current_tree, token)
    wanted = overlay_expected(base, current, records)  # Before ANY write.
    if current == wanted:
        return {"old_head": observed, "new_head": observed, "overlay_tree": current_tree,
                "qualified_tree": EXPECTED["qualified_tree"], "disposition": "already_present"}
    entries = []
    for record in records:
        if record["sha"] is not None:
            if blob_sha(record["content"]) != record["sha"]:
                raise ImportFailure("local record byte identity mismatch")
            result = github("POST", f"/repos/{repository}/git/blobs", token,
                            {"content": base64.b64encode(record["content"]).decode("ascii"), "encoding": "base64"})
            if result["sha"] != record["sha"]:
                raise ImportFailure("server blob mismatch")
        entries.append({"path": record["path"], "mode": record["mode"], "type": "blob", "sha": record["sha"]})
    qualified = github("POST", f"/repos/{repository}/git/trees", token,
                       {"base_tree": EXPECTED["source_tree"], "tree": entries})["sha"]
    if qualified != EXPECTED["qualified_tree"]:
        raise ImportFailure("server qualified tree mismatch")
    # GitHub rejects deleting an already absent path. Preserve idempotent retries.
    overlay_entries = [e for e in entries if e["sha"] is not None or e["path"] in current]
    overlay = github("POST", f"/repos/{repository}/git/trees", token,
                     {"base_tree": current_tree, "tree": overlay_entries})["sha"]
    if read_tree(repository, overlay, token) != wanted:
        raise ImportFailure("overlay read-back changed qualified or unrelated files")
    if github("GET", branch_path, token)["commit"]["sha"] != observed:
        raise ImportFailure("target branch moved during object import")
    message = ("fix(world): publish exact qualified Plan V4 direct-source bytes\n\n"
               f"Qualified-Tree: {qualified}\nArtifact-SHA256: {EXPECTED['artifact_zip_sha256']}\n"
               f"Patch-SHA256: {EXPECTED['patch_sha256']}\n"
               "Preserves the existing governance overlay. No CI/release/production credit.\n")
    commit = github("POST", f"/repos/{repository}/git/commits", token,
                    {"message": message, "tree": overlay, "parents": [observed]})["sha"]
    check = github("GET", f"/repos/{repository}/git/commits/{commit}", token)
    if check["tree"]["sha"] != overlay or [p["sha"] for p in check["parents"]] != [observed]:
        raise ImportFailure("created commit tree/parent read-back mismatch")
    github("PATCH", f"/repos/{repository}/git/refs/heads/{branch}", token, {"sha": commit, "force": False})
    if github("GET", branch_path, token)["commit"]["sha"] != commit:
        raise ImportFailure("final ref read-back mismatch; inspect remote before retry")
    return {"old_head": observed, "new_head": commit, "qualified_tree": qualified,
            "overlay_tree": overlay, "disposition": "published"}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact-zip", required=True, type=Path)
    parser.add_argument("--repository", default=REPOSITORY)
    parser.add_argument("--target-branch", default=DEFAULT_BRANCH)
    parser.add_argument("--expected-head")
    parser.add_argument("--publish", action="store_true")
    parser.add_argument("--result", type=Path, default=Path("world-v13k-import-result.json"))
    args = parser.parse_args()
    if args.repository != REPOSITORY or args.target_branch != DEFAULT_BRANCH:
        raise ImportFailure("repository and target must match the current Plan V4 candidate")
    if args.publish:
        validate_context(args.repository, args.target_branch, args.expected_head or "")
        if not os.environ.get("TRNM_WORLD_IMPORT_TOKEN"):
            raise ImportFailure("TRNM_WORLD_IMPORT_TOKEN is required with --publish")
    with tempfile.TemporaryDirectory(prefix="trnm-world-v13k-") as temporary:
        _, records = prepare(args.artifact_zip, Path(temporary))
        result = {"schema": "trnm_world_v13k_import_result_v3", "repository": args.repository,
                  "target_branch": args.target_branch, "qualified_tree": EXPECTED["qualified_tree"],
                  "file_count": WRITE_COUNT, "deletion_count": DELETION_COUNT, "published": False,
                  "exact_head_ci": "not_proven", "production_authorization": "not_granted"}
        if args.publish:
            result.update(publish(args.repository, args.target_branch, args.expected_head,
                                  os.environ["TRNM_WORLD_IMPORT_TOKEN"], records))
            result["published"] = True
        args.result.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print("TRNM_WORLD_V13K_IMPORT_PUBLISH=PASS" if args.publish else "TRNM_WORLD_V13K_IMPORT_DRY_RUN=PASS")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ImportFailure, OSError, ValueError, KeyError, tarfile.TarError,
            zipfile.BadZipFile, subprocess.TimeoutExpired) as error:
        print(f"import failed closed: {error}", file=sys.stderr)
        raise SystemExit(1)
