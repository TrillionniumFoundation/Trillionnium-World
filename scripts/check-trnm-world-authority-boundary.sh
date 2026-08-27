#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export ROOT_DIR

python3 - <<'PY'
from __future__ import annotations

import json
import os
import re
import sys
import tomllib
from pathlib import Path

root = Path(os.environ["ROOT_DIR"]).resolve()
failures: list[str] = []


def fail(message: str) -> None:
    failures.append(message)


def load_toml(path: Path) -> dict:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:  # pragma: no cover - shell gate diagnostic
        fail(f"cannot parse {path.relative_to(root)}: {exc}")
        return {}


boundary_path = root / "PROJECT_BOUNDARY.json"
if not boundary_path.is_file():
    fail("PROJECT_BOUNDARY.json is missing")
    boundary = {}
else:
    try:
        boundary = json.loads(boundary_path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"PROJECT_BOUNDARY.json is invalid JSON: {exc}")
        boundary = {}

if boundary.get("project_id") != "trillionnium-world":
    fail("PROJECT_BOUNDARY.json project_id must be trillionnium-world")
if boundary.get("lane") != "game-product":
    fail("PROJECT_BOUNDARY.json lane must be game-product")
if boundary.get("cargo", {}).get("external_path_dependencies") != "forbid":
    fail("external_path_dependencies must remain forbid")
for package in ("trnm-consensus-app", "trnm-runtime"):
    if package not in boundary.get("cargo", {}).get("forbidden_packages", []):
        fail(f"PROJECT_BOUNDARY.json must forbid {package}")

required_current_docs = [
    root / "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md",
    root / "docs/development/trnm-world-development-plan-v2.md",
    root / "docs/protocol/trnm-match-evidence-commitment-v1.md",
]
for path in required_current_docs:
    if not path.is_file():
        fail(f"required current authority document is missing: {path.relative_to(root)}")

workspace = root / "trillionnium"
workspace_manifest = load_toml(workspace / "Cargo.toml")
members = workspace_manifest.get("workspace", {}).get("members", [])
if not isinstance(members, list) or not members:
    fail("trillionnium workspace members are missing")
    members = []

active_manifests: list[Path] = [workspace / "Cargo.toml"]
for member in members:
    if not isinstance(member, str):
        fail(f"workspace member is not a string: {member!r}")
        continue
    if "*" in member:
        active_manifests.extend(sorted(workspace.glob(f"{member}/Cargo.toml")))
    else:
        active_manifests.append(workspace / member / "Cargo.toml")

seen: set[Path] = set()
for manifest in active_manifests:
    manifest = manifest.resolve()
    if manifest in seen:
        continue
    seen.add(manifest)
    if not manifest.is_file():
        fail(f"workspace manifest is missing: {manifest}")
        continue
    parsed = load_toml(manifest)
    tables: list[tuple[str, dict]] = []
    for name in ("dependencies", "dev-dependencies", "build-dependencies"):
        value = parsed.get(name, {})
        if isinstance(value, dict):
            tables.append((name, value))
    target = parsed.get("target", {})
    if isinstance(target, dict):
        for target_name, target_config in target.items():
            if not isinstance(target_config, dict):
                continue
            for name in ("dependencies", "dev-dependencies", "build-dependencies"):
                value = target_config.get(name, {})
                if isinstance(value, dict):
                    tables.append((f"target.{target_name}.{name}", value))
    workspace_table = parsed.get("workspace", {})
    if isinstance(workspace_table, dict):
        value = workspace_table.get("dependencies", {})
        if isinstance(value, dict):
            tables.append(("workspace.dependencies", value))

    for table_name, dependencies in tables:
        for dependency, spec in dependencies.items():
            if not isinstance(spec, dict) or "path" not in spec:
                continue
            path_value = spec.get("path")
            if not isinstance(path_value, str) or not path_value.strip():
                fail(f"{manifest.relative_to(root)} {table_name}.{dependency} has invalid path")
                continue
            resolved = (manifest.parent / path_value).resolve()
            try:
                resolved.relative_to(workspace)
            except ValueError:
                fail(
                    f"external Cargo path dependency is forbidden: "
                    f"{manifest.relative_to(root)} {dependency} -> {resolved}"
                )
                continue
            platform = workspace / "crates/platform"
            if resolved == platform or platform in resolved.parents:
                fail(
                    f"active World dependency enters excluded platform tree: "
                    f"{manifest.relative_to(root)} {dependency} -> {resolved}"
                )

# Encode forbidden implementation markers from fragments so this checker does
# not fail by matching its own policy literals.  These names are deliberately
# specific: ordinary documentation may discuss the target architecture, but
# active World source must not load a Nakama match-authority key or implement a
# competing signed completion surface.
forbidden_markers = [
    "TRNM_NAKAMA_" + "AUTHORITY_PRIVATE_KEY",
    "NAKAMA_MATCH_" + "AUTHORITY_PRIVATE_KEY",
    "sign_match_" + "completed_v1",
    "match_completed_" + "signing_key",
    "world_owned_" + "match_completed_signer",
]
source_roots = [workspace / "crates", root / "deploy", root / "packaging"]
source_suffixes = {".rs", ".toml", ".yaml", ".yml", ".json", ".service"}
for source_root in source_roots:
    if not source_root.exists():
        continue
    for path in source_root.rglob("*"):
        if not path.is_file() or path.suffix not in source_suffixes:
            continue
        if "crates/platform" in path.as_posix() or "/archive/" in path.as_posix():
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for marker in forbidden_markers:
            if marker in text:
                fail(
                    f"World active source contains forbidden target-authority marker "
                    f"{marker!r}: {path.relative_to(root)}"
                )

# Self-test the negative matcher.  A policy gate without a known failing
# fixture can silently become a no-op during refactoring.
negative_fixture = "const KEY: &str = \"" + forbidden_markers[0] + "\";"
if not any(marker in negative_fixture for marker in forbidden_markers):
    fail("authority boundary negative fixture did not fail")
positive_fixture = "pub const WORLD_RUNTIME_CONTRACT: &str = \"trnm_world_runtime_v1\";"
if any(marker in positive_fixture for marker in forbidden_markers):
    fail("authority boundary positive fixture was rejected")

if failures:
    print(json.dumps({"status": "blocked", "failures": failures}, indent=2), file=sys.stderr)
    raise SystemExit(1)

print(
    json.dumps(
        {
            "status": "ok",
            "contract": "trnm_world_authority_boundary_v1",
            "workspace_manifests_checked": len(seen),
            "forbidden_markers_checked": len(forbidden_markers),
            "negative_fixture": "passed",
        },
        sort_keys=True,
    )
)
PY
