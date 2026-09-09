#!/usr/bin/env python3
"""Fail-closed validation for every physical Rust/Node manifest in this repository."""

from __future__ import annotations

import argparse
import datetime as dt
import glob
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import sys
import tomllib
from typing import Any

DEFAULT_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CATALOG = "docs/component-catalog.json"
MAX_BYTES = 1_048_576
MAX_TEXT_BYTES = 2_097_152
MAX_DEPTH = 32
MAX_NODES = 30_000
MAX_COMPONENTS = 256

ROOT_KEYS = {"schema", "as_of", "repository", "production_authorization", "components"}
COMPONENT_KEYS = {
    "id", "path", "manifest", "kind", "lifecycle", "release_denominator",
    "owner", "canonical_docs", "gate",
}
MANIFEST_BY_KIND = {
    "rust-workspace": "Cargo.toml",
    "rust-crate": "Cargo.toml",
    "node-application": "package.json",
    "node-example": "package.json",
}
LIFECYCLE_DENOMINATOR = {
    "active": "game-product",
    "active-compatibility": "game-product",
    "compatibility-laboratory": "game-product",
    "cutover-candidate": "world-authority-candidate",
    "fixture-candidate": "world-authority-candidate",
    "excluded-legacy": "none",
    "mvp-perimeter": "scope-dependent",
    "historical-compatible-subproject": "none",
    "historical-example": "none",
    "protocol-contract": "cross-repository-contract",
    "vendored-dependency": "dependency-only",
}
EXPECTED_GAME = {
    "trnm-economy-protocol", "trnm-rpg-core", "trnm-campaign-core",
    "trnm-rts-protocol", "trnm-rts-sim", "trnm-online-protocol",
    "trnm-game-server", "trnm-first-contact",
}
EXPECTED_AUTHORITY = {
    "trnm-world-domain", "trnm-world-command", "trnm-world-projection",
    "trnm-world-map-provider", "trnm-world-ui-fragments", "trnm-world-api",
    "trnm-world-server",
}
IGNORED_DIRECTORY_NAMES = {
    ".git", ".next", "build", "coverage", "dist", "node_modules", "out",
    "run", "target",
}
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9._-]{1,127}$")


class CatalogError(RuntimeError):
    pass


def _reject_constant(value: str) -> None:
    raise CatalogError(f"non-finite JSON constant is forbidden: {value}")


def _object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise CatalogError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _budget(value: Any, depth: int = 0, count: list[int] | None = None) -> None:
    if count is None:
        count = [0]
    count[0] += 1
    if count[0] > MAX_NODES:
        raise CatalogError(f"JSON node budget exceeded: {MAX_NODES}")
    if depth > MAX_DEPTH:
        raise CatalogError(f"JSON depth exceeded: {MAX_DEPTH}")
    if isinstance(value, dict):
        for key, child in value.items():
            if not isinstance(key, str):
                raise CatalogError("JSON object key is not a string")
            _budget(child, depth + 1, count)
    elif isinstance(value, list):
        for child in value:
            _budget(child, depth + 1, count)


def _strict_json_bytes(data: bytes, label: str, max_bytes: int = MAX_BYTES) -> Any:
    if len(data) > max_bytes:
        raise CatalogError(f"{label} exceeds {max_bytes} bytes")
    try:
        text = data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise CatalogError(f"{label} is not strict UTF-8: {exc}") from exc
    try:
        value = json.loads(text, object_pairs_hook=_object, parse_constant=_reject_constant)
    except (json.JSONDecodeError, CatalogError) as exc:
        raise CatalogError(f"invalid {label} JSON: {exc}") from exc
    _budget(value)
    return value


def load_catalog(path: Path) -> dict[str, Any]:
    value = _strict_json_bytes(path.read_bytes(), "component catalog")
    if not isinstance(value, dict):
        raise CatalogError("catalog root must be an object")
    return value


def canonical_relative_path(raw: Any, label: str) -> str:
    if not isinstance(raw, str) or not raw or raw != raw.strip():
        raise CatalogError(f"{label} must be a nonblank canonical path")
    if len(raw.encode("utf-8")) > 512:
        raise CatalogError(f"{label} exceeds 512 UTF-8 bytes")
    if "\\" in raw:
        raise CatalogError(f"{label} uses a backslash")
    if raw.startswith("/") or raw.startswith("./") or raw.endswith("/"):
        raise CatalogError(f"{label} is not a canonical repository-relative path: {raw}")
    parts = raw.split("/")
    if any(part in {"", ".", ".."} for part in parts):
        raise CatalogError(f"{label} contains an empty/dot/parent segment: {raw}")
    if ":" in parts[0]:
        raise CatalogError(f"{label} resembles an absolute/drive path: {raw}")
    value = PurePosixPath(raw)
    if value.is_absolute() or value.as_posix() != raw:
        raise CatalogError(f"{label} is not canonical POSIX form: {raw}")
    return raw


def require_string(item: dict[str, Any], field: str, component_id: str) -> str:
    value = item.get(field)
    if not isinstance(value, str) or not value or value != value.strip():
        raise CatalogError(f"{component_id}: {field} must be a nonblank canonical string")
    if len(value.encode("utf-8")) > 512:
        raise CatalogError(f"{component_id}: {field} exceeds 512 UTF-8 bytes")
    return value


def checked_path(root: Path, relative: str, expected: str, nonempty: bool = False) -> Path:
    relative = canonical_relative_path(relative, "repository path")
    root_real = root.resolve(strict=True)
    current = root_real
    for part in PurePosixPath(relative).parts:
        current = current / part
        try:
            info = current.lstat()
        except FileNotFoundError as exc:
            raise CatalogError(f"repository path is missing: {relative}") from exc
        if stat.S_ISLNK(info.st_mode):
            raise CatalogError(f"repository path traverses a symlink: {relative}")
    resolved = current.resolve(strict=True)
    try:
        resolved.relative_to(root_real)
    except ValueError as exc:
        raise CatalogError(f"repository path escapes root: {relative}") from exc
    info = resolved.stat()
    if expected == "file" and not stat.S_ISREG(info.st_mode):
        raise CatalogError(f"repository path is not a regular file: {relative}")
    if expected == "dir" and not stat.S_ISDIR(info.st_mode):
        raise CatalogError(f"repository path is not a directory: {relative}")
    if nonempty and expected == "file" and info.st_size == 0:
        raise CatalogError(f"repository file is empty: {relative}")
    return resolved


def read_text(root: Path, relative: str, label: str) -> str:
    path = checked_path(root, relative, "file", nonempty=True)
    data = path.read_bytes()
    if len(data) > MAX_TEXT_BYTES:
        raise CatalogError(f"{label} exceeds {MAX_TEXT_BYTES} bytes: {relative}")
    try:
        return data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise CatalogError(f"{label} is not strict UTF-8: {relative}") from exc


def scan_repository_manifests(root: Path) -> set[str]:
    manifests: set[str] = set()
    root_real = root.resolve(strict=True)
    for current_raw, directories, files in os.walk(root_real, followlinks=False):
        current = Path(current_raw)
        relative_current = current.relative_to(root_real)
        kept: list[str] = []
        for directory in directories:
            if directory in IGNORED_DIRECTORY_NAMES:
                continue
            candidate = current / directory
            if candidate.is_symlink():
                raise CatalogError(
                    "repository contains a symlink directory outside generated roots: "
                    + candidate.relative_to(root_real).as_posix()
                )
            kept.append(directory)
        directories[:] = kept
        for filename in files:
            if filename not in {"Cargo.toml", "package.json"}:
                continue
            candidate = current / filename
            relative = (relative_current / filename).as_posix()
            if candidate.is_symlink():
                raise CatalogError(f"repository manifest is a symlink: {relative}")
            if not candidate.is_file():
                raise CatalogError(f"repository manifest is not a regular file: {relative}")
            manifests.add(relative)
    return manifests


def parse_rust_manifest(root: Path, relative: str) -> dict[str, Any]:
    text = read_text(root, relative, "Cargo manifest")
    try:
        value = tomllib.loads(text)
    except tomllib.TOMLDecodeError as exc:
        raise CatalogError(f"invalid Cargo manifest {relative}: {exc}") from exc
    if not isinstance(value, dict):
        raise CatalogError(f"Cargo manifest is not a TOML table: {relative}")
    return value


def parse_node_manifest(root: Path, relative: str) -> dict[str, Any]:
    value = _strict_json_bytes(
        checked_path(root, relative, "file", nonempty=True).read_bytes(),
        f"Node manifest {relative}",
    )
    if not isinstance(value, dict):
        raise CatalogError(f"Node manifest root must be an object: {relative}")
    return value


def component_package_name(component: dict[str, Any], manifest: dict[str, Any]) -> str | None:
    kind = component["kind"]
    if kind == "rust-workspace":
        return None
    if kind == "rust-crate":
        package = manifest.get("package")
        if not isinstance(package, dict):
            raise CatalogError(f"{component['id']}: rust-crate manifest lacks [package]")
        name = package.get("name")
    else:
        name = manifest.get("name")
    if not isinstance(name, str) or not name.strip():
        raise CatalogError(f"{component['id']}: manifest package name is missing")
    return name


def allowed_package_names(component_id: str) -> set[str]:
    names = {component_id}
    if component_id.startswith("legacy-"):
        names.add(component_id.removeprefix("legacy-"))
    if component_id == "vendor-wayland-scanner":
        names.add("wayland-scanner")
    return names


def rust_source_and_test_evidence(root: Path, component: dict[str, Any]) -> None:
    component_id = component["id"]
    component_path = checked_path(root, component["path"], "dir")
    source_root = component_path / "src"
    if source_root.is_symlink() or not source_root.is_dir():
        raise CatalogError(f"{component_id}: rust crate has no regular src directory")
    source_files = sorted(path for path in source_root.rglob("*.rs") if path.is_file() and not path.is_symlink())
    if not source_files:
        raise CatalogError(f"{component_id}: rust crate has no Rust source")
    if component["lifecycle"] in {"excluded-legacy", "vendored-dependency"}:
        return
    tests_root = component_path / "tests"
    external_tests = bool(
        tests_root.is_dir() and not tests_root.is_symlink()
        and any(path.is_file() and not path.is_symlink() for path in tests_root.rglob("*.rs"))
    )
    inline_tests = False
    for source in source_files:
        try:
            text = source.read_text(encoding="utf-8", errors="strict")
        except UnicodeDecodeError as exc:
            raise CatalogError(f"{component_id}: Rust source is not UTF-8: {source}") from exc
        if "#[cfg(test)]" in text or "mod tests" in text:
            inline_tests = True
            break
    if not (external_tests or inline_tests):
        raise CatalogError(f"{component_id}: no component-specific Rust test evidence")


def node_source_and_script_evidence(root: Path, component: dict[str, Any], manifest: dict[str, Any]) -> None:
    component_id = component["id"]
    component_path = checked_path(root, component["path"], "dir")
    source_files: list[Path] = []
    for suffix in ("*.js", "*.jsx", "*.ts", "*.tsx", "*.mjs", "*.cjs"):
        source_files.extend(
            path for path in component_path.rglob(suffix)
            if path.is_file() and not path.is_symlink()
            and not any(part in IGNORED_DIRECTORY_NAMES for part in path.relative_to(component_path).parts)
        )
    if not source_files:
        raise CatalogError(f"{component_id}: Node component has no source/example file")
    scripts = manifest.get("scripts")
    if not isinstance(scripts, dict):
        raise CatalogError(f"{component_id}: Node manifest lacks scripts")
    if component["kind"] == "node-example":
        if not isinstance(scripts.get("start"), str) or not scripts["start"].strip():
            raise CatalogError(f"{component_id}: historical example requires a bounded start script")
        return
    for required in ("build", "test"):
        if not isinstance(scripts.get(required), str) or not scripts[required].strip():
            raise CatalogError(f"{component_id}: Node application lacks {required!r} script")


def expand_workspace_members(root: Path, component: dict[str, Any], manifest: dict[str, Any], catalog_manifests: set[str]) -> set[str]:
    workspace = manifest.get("workspace")
    if not isinstance(workspace, dict):
        raise CatalogError(f"{component['id']}: rust-workspace manifest lacks [workspace]")
    members = workspace.get("members")
    if not isinstance(members, list) or not members:
        raise CatalogError(f"{component['id']}: workspace members must be nonempty")
    workspace_dir = PurePosixPath(component["manifest"]).parent
    expanded: set[str] = set()
    for raw_member in members:
        member = canonical_relative_path(raw_member, f"{component['id']}: workspace member")
        absolute_pattern = root / workspace_dir.as_posix() / member
        matches = sorted(glob.glob(str(absolute_pattern))) if glob.has_magic(member) else [str(absolute_pattern)]
        if not matches:
            raise CatalogError(f"{component['id']}: workspace member pattern matched nothing: {member}")
        for match in matches:
            path = Path(match)
            manifest_path = path if path.name == "Cargo.toml" else path / "Cargo.toml"
            try:
                relative = manifest_path.resolve(strict=True).relative_to(root.resolve(strict=True)).as_posix()
            except (FileNotFoundError, ValueError) as exc:
                raise CatalogError(f"{component['id']}: invalid workspace member: {member}") from exc
            checked_path(root, relative, "file", nonempty=True)
            if relative not in catalog_manifests:
                raise CatalogError(f"{component['id']}: workspace member is not classified: {relative}")
            expanded.add(relative)
    return expanded


def exact_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    keys = set(value)
    missing = sorted(expected - keys)
    extra = sorted(keys - expected)
    if missing or extra:
        raise CatalogError(f"{label} key mismatch missing={missing} extra={extra}")


def validate(catalog: dict[str, Any], root: Path) -> tuple[int, int]:
    exact_keys(catalog, ROOT_KEYS, "catalog root")
    if catalog["schema"] != "trnm_world_component_catalog_v1":
        raise CatalogError("unexpected component catalog schema")
    if catalog["repository"] != "TrillionniumFoundation/Trillionnium-World":
        raise CatalogError("catalog repository identity mismatch")
    if catalog["production_authorization"] != "not_granted":
        raise CatalogError("component catalogue cannot grant production authorization")
    try:
        dt.date.fromisoformat(catalog["as_of"])
    except (TypeError, ValueError) as exc:
        raise CatalogError("as_of must be an ISO calendar date") from exc

    components = catalog["components"]
    if not isinstance(components, list) or not components:
        raise CatalogError("components must be a nonempty array")
    if len(components) > MAX_COMPONENTS:
        raise CatalogError(f"component count exceeds {MAX_COMPONENTS}")

    ids: set[str] = set()
    paths: set[str] = set()
    manifests: set[str] = set()
    normalized: list[dict[str, Any]] = []
    parsed_manifests: dict[str, dict[str, Any]] = {}

    for index, raw in enumerate(components):
        if not isinstance(raw, dict):
            raise CatalogError(f"component[{index}] must be an object")
        exact_keys(raw, COMPONENT_KEYS, f"component[{index}]")
        component_id = require_string(raw, "id", f"component[{index}]")
        if not ID_PATTERN.fullmatch(component_id):
            raise CatalogError(f"invalid component id: {component_id}")
        component_path = canonical_relative_path(raw["path"], f"{component_id}: path")
        manifest_path = canonical_relative_path(raw["manifest"], f"{component_id}: manifest")
        kind = require_string(raw, "kind", component_id)
        lifecycle = require_string(raw, "lifecycle", component_id)
        denominator = require_string(raw, "release_denominator", component_id)
        require_string(raw, "owner", component_id)
        gate = canonical_relative_path(raw["gate"], f"{component_id}: gate")

        if component_id in ids:
            raise CatalogError(f"duplicate component id: {component_id}")
        if component_path in paths:
            raise CatalogError(f"duplicate component path: {component_path}")
        if manifest_path in manifests:
            raise CatalogError(f"duplicate component manifest: {manifest_path}")
        ids.add(component_id)
        paths.add(component_path)
        manifests.add(manifest_path)

        if kind not in MANIFEST_BY_KIND:
            raise CatalogError(f"{component_id}: unsupported component kind {kind!r}")
        expected_manifest = f"{component_path}/{MANIFEST_BY_KIND[kind]}"
        if manifest_path != expected_manifest:
            raise CatalogError(f"{component_id}: manifest must be exactly {expected_manifest}, got {manifest_path}")
        if lifecycle not in LIFECYCLE_DENOMINATOR:
            raise CatalogError(f"{component_id}: unsupported lifecycle {lifecycle!r}")
        expected_denominator = LIFECYCLE_DENOMINATOR[lifecycle]
        if denominator != expected_denominator:
            raise CatalogError(f"{component_id}: lifecycle {lifecycle!r} requires denominator {expected_denominator!r}")
        if kind == "node-example" and lifecycle != "historical-example":
            raise CatalogError(f"{component_id}: node-example must be historical-example")
        if kind == "node-application" and lifecycle != "historical-compatible-subproject":
            raise CatalogError(f"{component_id}: Node application lifecycle is not fail-closed historical")

        checked_path(root, component_path, "dir")
        checked_path(root, manifest_path, "file", nonempty=True)
        checked_path(root, gate, "file", nonempty=True)
        docs = raw["canonical_docs"]
        if not isinstance(docs, list) or not docs or len(docs) > 16:
            raise CatalogError(f"{component_id}: canonical_docs must contain 1..16 paths")
        seen_docs: set[str] = set()
        normalized_docs: list[str] = []
        for doc_raw in docs:
            doc = canonical_relative_path(doc_raw, f"{component_id}: canonical doc")
            if doc in seen_docs:
                raise CatalogError(f"{component_id}: duplicate canonical doc {doc}")
            seen_docs.add(doc)
            checked_path(root, doc, "file", nonempty=True)
            normalized_docs.append(doc)
        if denominator in {"game-product", "world-authority-candidate"} and len(normalized_docs) < 2:
            raise CatalogError(f"{component_id}: active/candidate component needs contract plus design/index")

        component = dict(raw)
        component.update(path=component_path, manifest=manifest_path, gate=gate, canonical_docs=normalized_docs)
        if kind.startswith("rust-"):
            parsed = parse_rust_manifest(root, manifest_path)
            if kind == "rust-workspace" and not isinstance(parsed.get("workspace"), dict):
                raise CatalogError(f"{component_id}: rust-workspace manifest lacks [workspace]")
            if kind == "rust-crate" and not isinstance(parsed.get("package"), dict):
                raise CatalogError(f"{component_id}: rust-crate manifest lacks [package]")
        else:
            parsed = parse_node_manifest(root, manifest_path)
        package_name = component_package_name(component, parsed)
        if package_name is not None and package_name not in allowed_package_names(component_id):
            raise CatalogError(f"{component_id}: package name {package_name!r} does not match component identity")
        component["_package_name"] = package_name
        parsed_manifests[manifest_path] = parsed
        normalized.append(component)

    discovered = scan_repository_manifests(root)
    missing = sorted(discovered - manifests)
    stale = sorted(manifests - discovered)
    if missing:
        raise CatalogError("unclassified repository manifests: " + ", ".join(missing))
    if stale:
        raise CatalogError("catalog manifests absent from repository: " + ", ".join(stale))

    components_by_manifest = {item["manifest"]: item for item in normalized}
    workspace_membership: dict[str, set[str]] = {}
    for component in normalized:
        if component["kind"] != "rust-workspace":
            continue
        members = expand_workspace_members(root, component, parsed_manifests[component["manifest"]], manifests)
        for member_manifest in members:
            workspace_membership.setdefault(member_manifest, set()).add(component["manifest"])

    for component in normalized:
        component_id = component["id"]
        kind = component["kind"]
        parsed = parsed_manifests[component["manifest"]]
        if kind == "rust-crate":
            rust_source_and_test_evidence(root, component)
        elif kind.startswith("node-"):
            node_source_and_script_evidence(root, component, parsed)

        package_name = component.get("_package_name")
        tokens = {component_id, component["path"], component["manifest"]}
        if isinstance(package_name, str):
            tokens.add(package_name)
        doc_specific = False
        for doc in component["canonical_docs"]:
            if doc.startswith(component["path"] + "/"):
                doc_specific = True
                break
            if any(token in read_text(root, doc, "canonical documentation") for token in tokens):
                doc_specific = True
                break
        if not doc_specific:
            raise CatalogError(f"{component_id}: canonical_docs are generic and not component-specific")

        gate_text = read_text(root, component["gate"], "component gate")
        gate_direct = component["gate"].startswith(component["path"] + "/") or any(token in gate_text for token in tokens)
        if not gate_direct and kind == "rust-crate":
            gate_direct = any(
                components_by_manifest[parent]["gate"] == component["gate"]
                for parent in workspace_membership.get(component["manifest"], set())
            )
        if not gate_direct:
            raise CatalogError(f"{component_id}: gate has no direct or proven workspace traceability")

    actual_game = {
        item["id"] for item in normalized
        if item["kind"] == "rust-crate" and item["release_denominator"] == "game-product"
    }
    if actual_game != EXPECTED_GAME:
        raise CatalogError(f"game-product crate denominator mismatch: {sorted(actual_game)}")
    actual_authority = {
        item["id"] for item in normalized
        if item["kind"] == "rust-crate" and item["release_denominator"] == "world-authority-candidate"
    }
    if actual_authority != EXPECTED_AUTHORITY:
        raise CatalogError(f"World authority crate denominator mismatch: {sorted(actual_authority)}")

    by_id = {item["id"]: item for item in normalized}
    transition = by_id.get("trnm-world-transition-v1")
    if not transition or transition["lifecycle"] != "protocol-contract":
        raise CatalogError("trnm-world-transition-v1 must be classified as a protocol contract")
    vendor = by_id.get("vendor-wayland-scanner")
    if not vendor or vendor["lifecycle"] != "vendored-dependency":
        raise CatalogError("vendor-wayland-scanner must be classified as a vendored dependency")
    game_manifest = parse_rust_manifest(root, "trillionnium/Cargo.toml")
    patch = game_manifest.get("patch", {}).get("crates-io", {}).get("wayland-scanner")
    if not isinstance(patch, dict) or patch.get("path") != "vendor/wayland-scanner":
        raise CatalogError("vendored wayland-scanner is not exactly bound by trillionnium/Cargo.toml")
    sdk = by_id.get("trnm-sdk-js-example")
    if not sdk or sdk["kind"] != "node-example" or sdk["release_denominator"] != "none":
        raise CatalogError("trnm-sdk-js-example must remain a non-release historical example")
    web4 = by_id.get("web4-frontend")
    if not web4 or web4["release_denominator"] != "none":
        raise CatalogError("web4-frontend must remain outside the World release denominator")
    return len(normalized), len(discovered)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("catalog", nargs="?", default=DEFAULT_CATALOG)
    parser.add_argument("--root", default=str(DEFAULT_ROOT))
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.root).resolve()
    catalog_path = Path(args.catalog)
    if not catalog_path.is_absolute():
        catalog_path = root / catalog_path
    try:
        relative_catalog = catalog_path.resolve(strict=True).relative_to(root).as_posix()
        checked_path(root, relative_catalog, "file", nonempty=True)
        component_count, manifest_count = validate(load_catalog(catalog_path), root)
    except (CatalogError, OSError, ValueError) as exc:
        print(f"TRNM_WORLD_COMPONENT_CATALOG=FAIL reason={exc}", file=sys.stderr)
        return 1
    print(
        "TRNM_WORLD_COMPONENT_CATALOG=PASS "
        f"components={component_count} manifests={manifest_count} "
        "production_authorization=not_granted"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
