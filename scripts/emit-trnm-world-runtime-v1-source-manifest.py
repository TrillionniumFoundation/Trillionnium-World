#!/usr/bin/env python3
"""Emit an exact source manifest for the World runtime v1 contract.

The manifest is generated from the checked-out Git commit and contract blobs.
It is source provenance only. Integration must independently vendor/verify the
blobs and bind an exact Nakama consumer before cross-repository credit exists.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

CONTRACT_FILES = [
    "contracts/world-runtime/v1/README.md",
    "contracts/world-runtime/v1/trnm-world-runtime-v1.schema.json",
    "contracts/world-runtime/v1/golden-vectors.json",
    "docs/protocol/trnm-world-runtime-v1.md",
    "scripts/verify-trnm-world-runtime-v1.py",
]


def git(root: Path, *args: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(root), *args],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    return completed.stdout.strip()


def exact_sha(value: str) -> bool:
    return len(value) == 40 and all(ch in "0123456789abcdef" for ch in value)


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    try:
        commit = git(root, "rev-parse", "HEAD")
        tree = git(root, "rev-parse", "HEAD^{tree}")
        if not exact_sha(commit) or not exact_sha(tree):
            raise ValueError("Git commit/tree is not exact lowercase 40-hex")
        dirty = git(root, "status", "--porcelain", "--untracked-files=no")
        if dirty:
            raise ValueError("tracked working tree is dirty; source manifest would be ambiguous")

        blobs = []
        for relative in CONTRACT_FILES:
            path = root / relative
            payload = path.read_bytes()
            git_blob = git(root, "rev-parse", f"HEAD:{relative}")
            if not exact_sha(git_blob):
                raise ValueError(f"Git blob for {relative} is not exact")
            blobs.append(
                {
                    "path": relative,
                    "bytes": len(payload),
                    "git_blob_sha1": git_blob,
                    "sha256": hashlib.sha256(payload).hexdigest(),
                }
            )

        report = {
            "contract_version": "trnm_world_runtime_source_manifest_v1",
            "status": "source_exact",
            "repository": "TrillionniumFoundation/Trillionnium-World",
            "commit": commit,
            "tree": tree,
            "runtime_contract": "trnm_world_runtime_v1",
            "blobs": blobs,
            "authority": {
                "world_deterministic_game_domain": True,
                "nakama_global_ordering": False,
                "nakama_completion_signing": False,
                "chain_finality": False,
                "cex_custody": False,
            },
            "limitations": [
                "Source provenance is not deployment or release evidence.",
                "Integration must independently verify vendored blobs and bind an exact Nakama consumer.",
                "Public online and public player market remain blocked."
            ],
        }
        print(json.dumps(report, sort_keys=True))
        return 0
    except (OSError, subprocess.CalledProcessError, ValueError) as error:
        print(json.dumps({"status": "blocked", "error": str(error)}), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
