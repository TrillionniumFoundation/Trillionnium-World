#!/usr/bin/env python3
"""Fail-closed catalog for operational surfaces without Cargo or Node manifests."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import sys
from typing import Any

DEFAULT_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CATALOG = "scripts/contracts/trnm-world-operational-surfaces-v1.json"
MAX_BYTES = 262_144
MAX_TEXT_BYTES = 2_097_152
MAX_FILES_PER_SURFACE = 30_000
MAX_TOTAL_BYTES_PER_SURFACE = 2_147_483_648

ROOT_KEYS = {
    "schema",
    "as_of",
    "repository",
    "production_authorization",
    "surfaces",
}
SURFACE_KEYS = {
    "id",
    "path",
    "kind",
    "lifecycle",
    "release_denominator",
    "owner",
    "authority",
    "security_class",
    "data_class",
    "canonical_docs",
    "gate",
    "sentinels",
}
EXPECTED_KIND_BY_PATH = {
    ".cargo": "developer-toolchain",
    ".githooks": "developer-hooks",
    ".github": "repository-governance-ci",
    "assets": "content-asset-pack",
    "config": "runtime-configuration",
    "deploy/postgres": "database-deployment",
    "deploy/systemd": "service-deployment",
    "packaging": "release-packaging",
    "playtests": "human-evidence-specification",
    "scripts": "qualification-and-operations-tooling",
}
LIFECYCLE_DENOMINATOR = {
    "active-build-control": "build-control",
    "active-governance-control": "governance-control",
    "active-game-product": "game-product",
    "compatibility-laboratory": "game-product",
    "cutover-candidate": "world-authority-candidate",
    "evidence-protocol": "human-evidence",
    "qualification-control": "release-evidence",
}
SECURITY_CLASSES = {"public", "internal", "restricted"}
DATA_CLASSES = {
    "no-user-data",
    "licensed-content",
    "configuration-no-secrets",
    "persistence-schema",
    "human-evidence-template",
}
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9._-]{1,127}$")


class SurfaceCatalogError(RuntimeError):
    pass


def _reject_constant(value: str) -> None:
    raise SurfaceCatalogError(f"non-finite JSON constant is forbidden: {value}")


def _strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise SurfaceCatalogError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _exact_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    missing = sorted(expected - set(value))
    extra = sorted(set(value) - expected)
    if missing or extra:
        raise SurfaceCatalogError(
            f"{label} key mismatch missing={missing} extra={extra}"
        )


def _canonical_relative(raw: Any, label: str) -> str:
    if not isinstance(raw, str) or not raw or raw != raw.strip():
        raise SurfaceCatalogError(f"{label} must be a nonblank canonical path")
    if len(raw.encode("utf-8")) > 512:
        raise SurfaceCatalogError(f"{label} exceeds 512 UTF-8 bytes")
    if "\\" in raw or raw.startswith("/") or raw.startswith("./") or raw.endswith("/"):
        raise SurfaceCatalogError(f"{label} is not repository-relative POSIX form: {raw}")
    parts = raw.split("/")
    if any(part in {"", ".", ".."} for part in parts) or ":" in parts[0]:
        raise SurfaceCatalogError(f"{label} contains an unsafe path segment: {raw}")
    if PurePosixPath(raw).as_posix() != raw:
        raise SurfaceCatalogError(f"{label} is not canonical POSIX form: {raw}")
    return raw


def _checked_path(
    root: Path,
    relative: str,
    expected: str,
    *,
    nonempty: bool = False,
) -> Path:
    relative = _canonical_relative(relative, "repository path")
    root_real = root.resolve(strict=True)
    current = root_real
    for part in PurePosixPath(relative).parts:
        current = current / part
        try:
            info = current.lstat()
        except FileNotFoundError as error:
            raise SurfaceCatalogError(f"repository path is missing: {relative}") from error
        if stat.S_ISLNK(info.st_mode):
            raise SurfaceCatalogError(f"repository path traverses a symlink: {relative}")
    resolved = current.resolve(strict=True)
    try:
        resolved.relative_to(root_real)
    except ValueError as error:
        raise SurfaceCatalogError(f"repository path escapes root: {relative}") from error
    info = resolved.stat()
    if expected == "file" and not stat.S_ISREG(info.st_mode):
        raise SurfaceCatalogError(f"repository path is not a regular file: {relative}")
    if expected == "dir" and not stat.S_ISDIR(info.st_mode):
        raise SurfaceCatalogError(f"repository path is not a directory: {relative}")
    if nonempty and expected == "file" and info.st_size == 0:
        raise SurfaceCatalogError(f"repository file is empty: {relative}")
    return resolved


def _require_string(item: dict[str, Any], field: str, label: str) -> str:
    value = item.get(field)
    if not isinstance(value, str) or not value or value != value.strip():
        raise SurfaceCatalogError(f"{label}: {field} must be a nonblank string")
    if len(value.encode("utf-8")) > 512:
        raise SurfaceCatalogError(f"{label}: {field} exceeds 512 UTF-8 bytes")
    return value


def _read_text(root: Path, relative: str, label: str) -> str:
    data = _checked_path(root, relative, "file", nonempty=True).read_bytes()
    if len(data) > MAX_TEXT_BYTES:
        raise SurfaceCatalogError(f"{label} exceeds {MAX_TEXT_BYTES} bytes: {relative}")
    try:
        return data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise SurfaceCatalogError(f"{label} is not strict UTF-8: {relative}") from error


def _load(path: Path) -> dict[str, Any]:
    data = path.read_bytes()
    if len(data) > MAX_BYTES:
        raise SurfaceCatalogError(f"operational surface catalog exceeds {MAX_BYTES} bytes")
    try:
        value = json.loads(
            data.decode("utf-8", errors="strict"),
            object_pairs_hook=_strict_object,
            parse_constant=_reject_constant,
        )
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise SurfaceCatalogError(f"invalid strict catalog JSON: {error}") from error
    if not isinstance(value, dict):
        raise SurfaceCatalogError("catalog root must be an object")
    return value


def _scan_surface(root: Path, relative: str) -> tuple[int, int]:
    directory = _checked_path(root, relative, "dir")
    files = 0
    total_bytes = 0
    for current_raw, directories, names in os.walk(directory, followlinks=False):
        current = Path(current_raw)
        kept: list[str] = []
        for name in directories:
            candidate = current / name
            if candidate.is_symlink():
                raise SurfaceCatalogError(
                    f"surface contains symlink directory: {candidate.relative_to(root).as_posix()}"
                )
            kept.append(name)
        directories[:] = kept
        for name in names:
            candidate = current / name
            relative_file = candidate.relative_to(root).as_posix()
            info = candidate.lstat()
            if stat.S_ISLNK(info.st_mode):
                raise SurfaceCatalogError(f"surface contains symlink file: {relative_file}")
            if not stat.S_ISREG(info.st_mode):
                raise SurfaceCatalogError(f"surface contains non-regular file: {relative_file}")
            files += 1
            total_bytes += info.st_size
            if files > MAX_FILES_PER_SURFACE:
                raise SurfaceCatalogError(
                    f"{relative}: file count exceeds {MAX_FILES_PER_SURFACE}"
                )
            if total_bytes > MAX_TOTAL_BYTES_PER_SURFACE:
                raise SurfaceCatalogError(
                    f"{relative}: byte count exceeds {MAX_TOTAL_BYTES_PER_SURFACE}"
                )
    if files == 0:
        raise SurfaceCatalogError(f"operational surface has no regular files: {relative}")
    return files, total_bytes


def validate(catalog: dict[str, Any], root: Path) -> tuple[int, int, int]:
    root = root.resolve(strict=True)
    _exact_keys(catalog, ROOT_KEYS, "catalog root")
    if catalog["schema"] != "trnm_world_operational_surface_catalog_v1":
        raise SurfaceCatalogError("unexpected operational surface catalog schema")
    if catalog["repository"] != "TrillionniumFoundation/Trillionnium-World":
        raise SurfaceCatalogError("repository identity mismatch")
    if catalog["production_authorization"] != "not_granted":
        raise SurfaceCatalogError("operational catalog cannot grant production authorization")
    try:
        dt.date.fromisoformat(catalog["as_of"])
    except (TypeError, ValueError) as error:
        raise SurfaceCatalogError("as_of must be an ISO calendar date") from error

    raw_surfaces = catalog["surfaces"]
    if not isinstance(raw_surfaces, list) or not raw_surfaces:
        raise SurfaceCatalogError("surfaces must be a nonempty array")
    if len(raw_surfaces) != len(EXPECTED_KIND_BY_PATH):
        raise SurfaceCatalogError(
            f"surface count drift expected={len(EXPECTED_KIND_BY_PATH)} actual={len(raw_surfaces)}"
        )

    ids: set[str] = set()
    paths: set[str] = set()
    sentinel_owners: dict[str, str] = {}
    total_files = 0
    total_bytes = 0

    for index, raw in enumerate(raw_surfaces):
        if not isinstance(raw, dict):
            raise SurfaceCatalogError(f"surface[{index}] must be an object")
        _exact_keys(raw, SURFACE_KEYS, f"surface[{index}]")
        surface_id = _require_string(raw, "id", f"surface[{index}]")
        if not ID_PATTERN.fullmatch(surface_id):
            raise SurfaceCatalogError(f"invalid surface id: {surface_id}")
        path = _canonical_relative(raw["path"], f"{surface_id}: path")
        kind = _require_string(raw, "kind", surface_id)
        lifecycle = _require_string(raw, "lifecycle", surface_id)
        denominator = _require_string(raw, "release_denominator", surface_id)
        _require_string(raw, "owner", surface_id)
        _require_string(raw, "authority", surface_id)
        security_class = _require_string(raw, "security_class", surface_id)
        data_class = _require_string(raw, "data_class", surface_id)
        gate = _canonical_relative(raw["gate"], f"{surface_id}: gate")

        if surface_id in ids:
            raise SurfaceCatalogError(f"duplicate surface id: {surface_id}")
        if path in paths:
            raise SurfaceCatalogError(f"duplicate surface path: {path}")
        ids.add(surface_id)
        paths.add(path)

        expected_kind = EXPECTED_KIND_BY_PATH.get(path)
        if expected_kind is None:
            raise SurfaceCatalogError(f"unrecognized operational surface path: {path}")
        if kind != expected_kind:
            raise SurfaceCatalogError(
                f"{surface_id}: path {path} requires kind {expected_kind!r}, got {kind!r}"
            )
        expected_denominator = LIFECYCLE_DENOMINATOR.get(lifecycle)
        if expected_denominator is None:
            raise SurfaceCatalogError(f"{surface_id}: unsupported lifecycle {lifecycle!r}")
        if denominator != expected_denominator:
            raise SurfaceCatalogError(
                f"{surface_id}: lifecycle {lifecycle!r} requires denominator "
                f"{expected_denominator!r}"
            )
        if security_class not in SECURITY_CLASSES:
            raise SurfaceCatalogError(
                f"{surface_id}: unsupported security_class {security_class!r}"
            )
        if data_class not in DATA_CLASSES:
            raise SurfaceCatalogError(
                f"{surface_id}: unsupported data_class {data_class!r}"
            )

        files, size = _scan_surface(root, path)
        total_files += files
        total_bytes += size
        _read_text(root, gate, f"{surface_id} gate")

        docs = raw["canonical_docs"]
        if not isinstance(docs, list) or not (1 <= len(docs) <= 8):
            raise SurfaceCatalogError(
                f"{surface_id}: canonical_docs must contain 1..8 paths"
            )
        seen_docs: set[str] = set()
        for value in docs:
            doc = _canonical_relative(value, f"{surface_id}: canonical doc")
            if doc in seen_docs:
                raise SurfaceCatalogError(f"{surface_id}: duplicate canonical doc {doc}")
            seen_docs.add(doc)
            _read_text(root, doc, f"{surface_id} canonical documentation")

        sentinels = raw["sentinels"]
        if not isinstance(sentinels, list) or not (1 <= len(sentinels) <= 16):
            raise SurfaceCatalogError(f"{surface_id}: sentinels must contain 1..16 paths")
        seen_sentinels: set[str] = set()
        prefix = path + "/"
        for value in sentinels:
            sentinel = _canonical_relative(value, f"{surface_id}: sentinel")
            if not sentinel.startswith(prefix):
                raise SurfaceCatalogError(
                    f"{surface_id}: sentinel is outside its surface: {sentinel}"
                )
            if sentinel in seen_sentinels:
                raise SurfaceCatalogError(
                    f"{surface_id}: duplicate sentinel {sentinel}"
                )
            if sentinel in sentinel_owners:
                raise SurfaceCatalogError(
                    f"sentinel {sentinel} is owned by both "
                    f"{sentinel_owners[sentinel]} and {surface_id}"
                )
            seen_sentinels.add(sentinel)
            sentinel_owners[sentinel] = surface_id
            _checked_path(root, sentinel, "file", nonempty=True)

    expected_paths = set(EXPECTED_KIND_BY_PATH)
    if paths != expected_paths:
        raise SurfaceCatalogError(
            f"operational surface path denominator mismatch "
            f"missing={sorted(expected_paths - paths)} extra={sorted(paths - expected_paths)}"
        )

    deploy_children = {
        path.relative_to(root).as_posix()
        for path in (root / "deploy").iterdir()
        if path.is_dir() and not path.is_symlink()
    }
    if deploy_children != {"deploy/postgres", "deploy/systemd"}:
        raise SurfaceCatalogError(
            f"deploy child surface drift: {sorted(deploy_children)}"
        )
    asset_children = {
        path.name
        for path in (root / "assets").iterdir()
        if path.is_dir() and not path.is_symlink()
    }
    if asset_children != {"first_contact", "trnm-world"}:
        raise SurfaceCatalogError(
            f"asset child surface drift: {sorted(asset_children)}"
        )
    return len(raw_surfaces), total_files, total_bytes


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("catalog", nargs="?", default=DEFAULT_CATALOG)
    parser.add_argument("--root", default=str(DEFAULT_ROOT))
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.root).resolve()
    catalog = Path(args.catalog)
    if not catalog.is_absolute():
        catalog = root / catalog
    try:
        relative = catalog.resolve(strict=True).relative_to(root).as_posix()
        _checked_path(root, relative, "file", nonempty=True)
        surface_count, file_count, byte_count = validate(_load(catalog), root)
    except (OSError, ValueError, SurfaceCatalogError) as error:
        print(f"TRNM_WORLD_OPERATIONAL_SURFACES=FAIL reason={error}", file=sys.stderr)
        return 1
    print(
        "TRNM_WORLD_OPERATIONAL_SURFACES=PASS "
        f"surfaces={surface_count} files={file_count} bytes={byte_count} "
        "production_authorization=not_granted"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
