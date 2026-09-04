#!/usr/bin/env python3
"""Fail-closed static and semantic contract for the qualified-source importer."""

from __future__ import annotations

import ast
import importlib.util
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
IMPORTER = ROOT / "scripts" / "import-qualified-world-v13k.py"
EXPECTED_LITERALS = {
    "TrillionniumFoundation/Trillionnium-World",
    "fix/world-plan-v4-development-closure-20260831",
    "5605cfb8861aa923f69ff032ddbff7d035bccb0c",
    "928f43b328e5347b07357e41481df1c7e097adca",
    "68e9631b3fc3f75f332497f8d0551608bf0e1413",
    "456a181bdc8f8aa248229b044db9eec4f52572ea3b20bca6492907db58d64ef5",
    "4c703f428f9a54262a6c0c1340028d08d7883f25ef437c1d7e221a280f53f071",
    "44cf59478b28e8fde793ca0d705ba3634216a5d388ce7264e74cd0319c41ff6f",
    "d05b375af8d0d8317a2e6e58b75a594949729203499f6677b43f1ea36ff31110",
    "ba49dba1e7fbf842f146ac399647e188faafcfbd5ce3ad17425ef88850e0199f",
    "5e613185f5a2abda42df371f3755e73667717309",
    "TRNM_WORLD_IMPORT_TOKEN",
}
FORBIDDEN_ENDPOINT_FRAGMENTS = (
    "/statuses/",
    "/check-runs",
    "/merges",
    "/releases",
    "/deployments",
    "/git/tags",
    "/git/refs/tags",
    "/branches/main",
    "/branches/master",
)


def fail(message: str) -> None:
    print(f"v13k import contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_importer():
    spec = importlib.util.spec_from_file_location("trnm_world_v13k_importer", IMPORTER)
    if spec is None or spec.loader is None:
        fail("cannot load importer module")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def semantic_directory_marker_test(module) -> None:
    patch_paths = ["root.txt"] + [f"split/part_{index:02d}.rs" for index in range(72)]
    manifest = {
        "files": [
            {
                "path": "root.txt",
                "status": " M",
                "bytes": 1,
                "git_blob_sha1": "0" * 40,
                "sha256": "0" * 64,
            },
            {"path": "split/", "status": "??"},
            {"path": "obsolete-a", "status": " D"},
            {"path": "obsolete-b", "status": " D"},
        ]
    }
    declared, deletions = module.classify_manifest(manifest, patch_paths)
    if set(declared) != {"root.txt"}:
        fail("declared file classification drifted")
    if deletions != {"obsolete-a", "obsolete-b"}:
        fail("only explicit D entries may become deletions")
    if "split/" in deletions:
        fail("manifest directory marker was treated as deletion")

    broken = {"files": [dict(item) for item in manifest["files"]]}
    broken["files"][1]["status"] = " D"
    try:
        module.classify_manifest(broken, patch_paths)
    except module.ImportFailure:
        pass
    else:
        fail("directory-as-deletion regression was not rejected")


def main() -> None:
    source = IMPORTER.read_text(encoding="utf-8")
    try:
        tree = ast.parse(source, filename=str(IMPORTER))
    except SyntaxError as error:
        fail(f"importer is not valid Python: {error}")

    literals = {
        node.value
        for node in ast.walk(tree)
        if isinstance(node, ast.Constant) and isinstance(node.value, str)
    }
    missing = sorted(EXPECTED_LITERALS - literals)
    if missing:
        fail(f"missing immutable literals: {missing}")

    required_phrases = (
        "WRITE_COUNT = 73",
        "DELETION_COUNT = 2",
        'parser.add_argument("--publish", action="store_true")',
        'parser.add_argument("--expected-head")',
        'if args.target_branch in {"main", "master"}',
        'command("git", "-C", str(candidate), "add", "-f", "-A")',
        'if status == "D":',
        'elif status == "??" and path.endswith("/"):',
        "candidate archive tree mismatch",
        "patch path is outside manifest coverage",
        '"force": False',
        "target branch moved during object import",
        "server qualified tree mismatch",
        "final ref read-back mismatch",
        "artifact ZIP SHA-256 mismatch",
        "manifest byte mismatch",
        "server blob mismatch",
    )
    for phrase in required_phrases:
        if phrase not in source:
            fail(f"missing safety phrase: {phrase}")

    for fragment in FORBIDDEN_ENDPOINT_FRAGMENTS:
        if fragment in source:
            fail(f"forbidden endpoint capability present: {fragment}")

    if "force=True" in source or '"force": True' in source:
        fail("force ref update is forbidden")
    if source.count('github("PATCH"') != 1:
        fail("exactly one PATCH operation is allowed")
    if "/git/refs/heads/{branch}" not in source:
        fail("the only PATCH must target the selected review-branch ref")

    module = load_importer()
    if module.WRITE_COUNT != 73 or module.DELETION_COUNT != 2:
        fail("runtime operation counts drifted")
    semantic_directory_marker_test(module)
    print("TRNM_WORLD_V13K_IMPORT_CONTRACT=PASS")


if __name__ == "__main__":
    main()
