#!/usr/bin/env python3
from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
import os
import pathlib
import re
import subprocess
import tarfile
from typing import Final

CONTRACT_VERSION: Final = "trnm_world_transition_v1"
BUNDLE_VERSION: Final = "trnm_world_transition_bundle_v1"
ARCHIVE_ROOT: Final = "trnm-world-transition-v1"
SHA40 = re.compile(r"^[0-9a-f]{40}$")

SOURCE_FILES: Final[dict[str, str]] = {
    "Cargo.lock": "trillionnium/contracts/trnm-world-transition-v1/Cargo.lock",
    "Cargo.toml": "trillionnium/contracts/trnm-world-transition-v1/Cargo.toml",
    "README.md": "trillionnium/contracts/trnm-world-transition-v1/README.md",
    "rust-toolchain.toml": "trillionnium/contracts/trnm-world-transition-v1/rust-toolchain.toml",
    "src/canonical_json.rs": "trillionnium/contracts/trnm-world-transition-v1/src/canonical_json.rs",
    "src/contract.rs": "trillionnium/contracts/trnm-world-transition-v1/src/contract.rs",
    "docs/protocol/trnm-world-transition-v1.md": "docs/protocol/trnm-world-transition-v1.md",
    "docs/protocol/schemas/trnm-world-transition-v1.schema.json": "docs/protocol/schemas/trnm-world-transition-v1.schema.json",
    "docs/protocol/vectors/trnm-world-transition-v1.json": "docs/protocol/vectors/trnm-world-transition-v1.json",
    "docs/development/trnm-world-transition-contract-v1.json": "docs/development/trnm-world-transition-contract-v1.json",
}


def sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def git_value(root: pathlib.Path, revision: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", revision],
        text=True,
        stderr=subprocess.DEVNULL,
    ).strip()


def resolve_identity(
    root: pathlib.Path,
    source_commit: str | None,
    source_tree: str | None,
) -> tuple[str, str]:
    commit = source_commit or os.environ.get("TRNM_TRANSITION_SOURCE_COMMIT")
    tree = source_tree or os.environ.get("TRNM_TRANSITION_SOURCE_TREE")
    if not commit:
        commit = git_value(root, "HEAD")
    if not tree:
        tree = git_value(root, f"{commit}^{{tree}}")
    if not SHA40.fullmatch(commit):
        raise SystemExit("source commit must be a lower-case 40-character Git SHA")
    if not SHA40.fullmatch(tree):
        raise SystemExit("source tree must be a lower-case 40-character Git SHA")
    return commit, tree


def canonical_json(value: object) -> bytes:
    return (
        json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
        + "\n"
    ).encode("utf-8")


def load_sources(root: pathlib.Path) -> dict[str, bytes]:
    files: dict[str, bytes] = {}
    for archive_path, source_path in sorted(SOURCE_FILES.items()):
        source = root / source_path
        if not source.is_file():
            raise SystemExit(f"required contract source is missing: {source_path}")
        files[archive_path] = source.read_bytes()
    return files


def build_manifest(
    files: dict[str, bytes],
    source_commit: str,
    source_tree: str,
) -> bytes:
    entries = [
        {
            "path": path,
            "sha256": sha256_bytes(content),
            "size": len(content),
        }
        for path, content in sorted(files.items())
    ]
    manifest = {
        "authority_scope": {
            "canonical_online_authority_claimed": False,
            "match_completed_v1_owned": False,
            "match_evidence_signing_key_owned": False,
        },
        "bundle_version": BUNDLE_VERSION,
        "contract_version": CONTRACT_VERSION,
        "files": entries,
        "next_dependency": "WORLD-P0-003",
        "source_commit": source_commit,
        "source_tree": source_tree,
    }
    return canonical_json(manifest)


def add_regular_file(archive: tarfile.TarFile, name: str, content: bytes) -> None:
    info = tarfile.TarInfo(name=name)
    info.size = len(content)
    info.mode = 0o644
    info.uid = 0
    info.gid = 0
    info.uname = ""
    info.gname = ""
    info.mtime = 0
    archive.addfile(info, io.BytesIO(content))


def build_tar(files: dict[str, bytes], manifest: bytes) -> bytes:
    output = io.BytesIO()
    with tarfile.open(fileobj=output, mode="w", format=tarfile.USTAR_FORMAT) as archive:
        for path, content in sorted(files.items()):
            add_regular_file(archive, f"{ARCHIVE_ROOT}/{path}", content)
        add_regular_file(archive, f"{ARCHIVE_ROOT}/MANIFEST.json", manifest)
    return output.getvalue()


def deterministic_gzip(content: bytes) -> bytes:
    output = io.BytesIO()
    with gzip.GzipFile(
        filename="",
        mode="wb",
        compresslevel=9,
        fileobj=output,
        mtime=0,
    ) as compressed:
        compressed.write(content)
    return output.getvalue()


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Build a deterministic exact-revision World transition contract bundle."
    )
    parser.add_argument(
        "--root",
        type=pathlib.Path,
        default=pathlib.Path(__file__).resolve().parents[1],
    )
    parser.add_argument("--output-dir", type=pathlib.Path, required=True)
    parser.add_argument("--source-commit")
    parser.add_argument("--source-tree")
    args = parser.parse_args()

    root = args.root.resolve()
    source_commit, source_tree = resolve_identity(
        root, args.source_commit, args.source_tree
    )
    files = load_sources(root)
    manifest = build_manifest(files, source_commit, source_tree)
    archive_bytes = deterministic_gzip(build_tar(files, manifest))
    archive_sha256 = sha256_bytes(archive_bytes)

    output_dir = args.output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    archive_path = output_dir / f"{ARCHIVE_ROOT}.tar.gz"
    checksum_path = output_dir / f"{ARCHIVE_ROOT}.tar.gz.sha256"
    manifest_path = output_dir / f"{ARCHIVE_ROOT}.manifest.json"

    archive_path.write_bytes(archive_bytes)
    checksum_path.write_text(
        f"{archive_sha256}  {archive_path.name}\n", encoding="utf-8"
    )
    manifest_path.write_bytes(manifest)

    print(
        json.dumps(
            {
                "archive": str(archive_path),
                "archive_sha256": archive_sha256,
                "contract_version": CONTRACT_VERSION,
                "manifest": str(manifest_path),
                "source_commit": source_commit,
                "source_tree": source_tree,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
