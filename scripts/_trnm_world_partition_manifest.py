#!/usr/bin/env python3
"""Bind textual Rust source partitions to reviewed manifests.

The include-boundary inventory classifies whether a textual include is allowed.
This module adds the missing integrity layer: the entrypoint order, file bytes,
SHA-256 digests, section order, and nested test partitions must match the
checked-in manifest exactly.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
from typing import Any, Iterable

EXPECTED_MANIFESTS = (
    "trillionnium/crates/trnm-game-server/src/lib_parts/manifest.json",
)
MAX_MANIFEST_BYTES = 512 * 1024
HEX64 = re.compile(r"^[0-9a-f]{64}$")
ROOT_KEYS = {
    "crate",
    "max_impl_bytes",
    "max_part_bytes",
    "nested_test_parts",
    "parts",
    "schema",
    "sections",
    "semantic_generation",
    "textual_include_reason",
}
PART_KEYS = {"bytes", "path", "section", "sha256"}
ENTRYPOINT_EDGE = re.compile(
    r'include!\(\s*"(?P<include>[^"]+)"\s*\)'
    r'|#\s*\[\s*path\s*=\s*"(?P<path>[^"]+)"\s*\]',
    re.MULTILINE,
)
NESTED_TEST_EDGE = re.compile(r'"/src/(?P<path>lib_parts/[^"]+\.rs)"')


class PartitionManifestFailure(ValueError):
    """Raised when a source-partition manifest is incomplete or drifts."""


def _strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise PartitionManifestFailure(
                f"duplicate partition-manifest JSON key: {key}"
            )
        result[key] = value
    return result


def _reject_constant(value: str) -> None:
    raise PartitionManifestFailure(
        f"non-finite partition-manifest JSON constant is forbidden: {value}"
    )


def _inside(root: Path, relative: str, label: str) -> Path:
    if not isinstance(relative, str) or not relative or relative != relative.strip():
        raise PartitionManifestFailure(f"{label} must be a nonempty trimmed path")
    candidate = Path(relative)
    if candidate.is_absolute() or ".." in candidate.parts:
        raise PartitionManifestFailure(f"{label} must remain repository-relative")
    target = (root / candidate).resolve(strict=False)
    try:
        target.relative_to(root)
    except ValueError as error:
        raise PartitionManifestFailure(f"{label} escapes the repository") from error
    return target


def _read_utf8(path: Path, label: str, maximum: int | None = None) -> str:
    try:
        data = path.read_bytes()
    except OSError as error:
        raise PartitionManifestFailure(f"{label} cannot be read: {path}") from error
    if maximum is not None and len(data) > maximum:
        raise PartitionManifestFailure(f"{label} exceeds the byte budget: {path}")
    try:
        return data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise PartitionManifestFailure(f"{label} is not strict UTF-8: {path}") from error


def _load_manifest(path: Path) -> dict[str, Any]:
    text = _read_utf8(path, "partition manifest", MAX_MANIFEST_BYTES)
    try:
        value = json.loads(
            text,
            object_pairs_hook=_strict_object,
            parse_constant=_reject_constant,
        )
    except json.JSONDecodeError as error:
        raise PartitionManifestFailure(
            f"invalid strict partition-manifest JSON: {path}: {error}"
        ) from error
    if not isinstance(value, dict) or set(value) != ROOT_KEYS:
        raise PartitionManifestFailure(
            f"partition manifest root key set drift: {path}"
        )
    return value


def _positive_int(value: Any, label: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        raise PartitionManifestFailure(f"{label} must be a positive integer")
    return value


def _validate_entry(
    source_root: Path,
    raw: Any,
    sections: set[str],
    maximum_bytes: int,
    label: str,
) -> tuple[str, str]:
    if not isinstance(raw, dict) or set(raw) != PART_KEYS:
        raise PartitionManifestFailure(f"{label} key set drift")
    relative = raw["path"]
    section = raw["section"]
    expected_bytes = _positive_int(raw["bytes"], f"{label}.bytes")
    digest = raw["sha256"]
    if not isinstance(section, str) or section not in sections:
        raise PartitionManifestFailure(f"{label}.section is not declared")
    if not isinstance(digest, str) or HEX64.fullmatch(digest) is None:
        raise PartitionManifestFailure(f"{label}.sha256 must be lowercase hex")
    if (
        not isinstance(relative, str)
        or not relative.startswith("lib_parts/")
        or relative != relative.strip()
    ):
        raise PartitionManifestFailure(
            f"{label}.path must be a trimmed lib_parts/ path"
        )

    target = _inside(source_root, relative, f"{label}.path")
    if not target.is_file() or target.is_symlink():
        raise PartitionManifestFailure(
            f"{label}.path must name an ordinary file: {relative}"
        )
    data = target.read_bytes()
    if len(data) != expected_bytes:
        raise PartitionManifestFailure(
            f"{label}.bytes drift for {relative}: "
            f"expected={expected_bytes}, actual={len(data)}"
        )
    if len(data) > maximum_bytes:
        raise PartitionManifestFailure(
            f"{label}.bytes exceeds max_part_bytes for {relative}"
        )
    actual_digest = hashlib.sha256(data).hexdigest()
    if actual_digest != digest:
        raise PartitionManifestFailure(
            f"{label}.sha256 drift for {relative}: "
            f"expected={digest}, actual={actual_digest}"
        )
    return relative, section


def _entrypoint_paths(entrypoint: Path) -> list[str]:
    text = _read_utf8(entrypoint, "partition entrypoint")
    paths: list[str] = []
    for match in ENTRYPOINT_EDGE.finditer(text):
        relative = match.group("include") or match.group("path")
        if relative.startswith("lib_parts/"):
            paths.append(relative)
    return paths


def _nested_test_paths(test_module: Path) -> list[str]:
    text = _read_utf8(test_module, "nested test partition entrypoint")
    return [match.group("path") for match in NESTED_TEST_EDGE.finditer(text)]


def validate_manifest(repo_root: Path, manifest_relative: str) -> None:
    root = repo_root.resolve(strict=True)
    manifest_path = _inside(root, manifest_relative, "partition manifest path")
    if not manifest_path.is_file() or manifest_path.is_symlink():
        raise PartitionManifestFailure(
            f"required partition manifest is missing: {manifest_relative}"
        )
    manifest = _load_manifest(manifest_path)
    if manifest["schema"] != "trnm_semantic_direct_source_partition_v1":
        raise PartitionManifestFailure(
            f"partition manifest schema drift: {manifest_relative}"
        )
    if manifest["semantic_generation"] is not False:
        raise PartitionManifestFailure(
            f"semantic source generation is forbidden: {manifest_relative}"
        )
    reason = manifest["textual_include_reason"]
    if not isinstance(reason, str) or not reason.strip() or reason != reason.strip():
        raise PartitionManifestFailure(
            f"textual_include_reason must be nonempty and trimmed: {manifest_relative}"
        )
    max_impl_bytes = _positive_int(
        manifest["max_impl_bytes"], "partition manifest max_impl_bytes"
    )
    max_part_bytes = _positive_int(
        manifest["max_part_bytes"], "partition manifest max_part_bytes"
    )
    if max_impl_bytes > max_part_bytes:
        raise PartitionManifestFailure(
            f"max_impl_bytes exceeds max_part_bytes: {manifest_relative}"
        )

    sections_raw = manifest["sections"]
    if (
        not isinstance(sections_raw, list)
        or not sections_raw
        or any(
            not isinstance(section, str)
            or not section
            or section != section.strip()
            for section in sections_raw
        )
        or len(set(sections_raw)) != len(sections_raw)
    ):
        raise PartitionManifestFailure(
            f"sections must be unique nonempty strings: {manifest_relative}"
        )
    sections = set(sections_raw)

    parts_raw = manifest["parts"]
    nested_raw = manifest["nested_test_parts"]
    if not isinstance(parts_raw, list) or not parts_raw:
        raise PartitionManifestFailure(f"parts must be nonempty: {manifest_relative}")
    if not isinstance(nested_raw, list):
        raise PartitionManifestFailure(
            f"nested_test_parts must be a list: {manifest_relative}"
        )

    part_records = [
        _validate_entry(
            manifest_path.parent.parent,
            raw,
            sections,
            max_part_bytes,
            f"parts[{index}]",
        )
        for index, raw in enumerate(parts_raw)
    ]
    part_paths = [path for path, _section in part_records]
    if len(set(part_paths)) != len(part_paths):
        raise PartitionManifestFailure(
            f"duplicate source partition path: {manifest_relative}"
        )

    entrypoint = manifest_path.parent.parent / "lib.rs"
    observed_paths = _entrypoint_paths(entrypoint)
    if observed_paths != part_paths:
        raise PartitionManifestFailure(
            f"entrypoint partition order drift: expected={part_paths}, "
            f"actual={observed_paths}"
        )

    observed_sections: list[str] = []
    for _path, section in part_records:
        if not observed_sections or observed_sections[-1] != section:
            observed_sections.append(section)
    if observed_sections != sections_raw:
        raise PartitionManifestFailure(
            f"section order drift: expected={sections_raw}, "
            f"actual={observed_sections}"
        )

    nested_records = [
        _validate_entry(
            manifest_path.parent.parent,
            raw,
            sections,
            max_part_bytes,
            f"nested_test_parts[{index}]",
        )
        for index, raw in enumerate(nested_raw)
    ]
    nested_paths = [path for path, _section in nested_records]
    if len(set(nested_paths)) != len(nested_paths):
        raise PartitionManifestFailure(
            f"duplicate nested test partition path: {manifest_relative}"
        )
    if any(section != "tests" for _path, section in nested_records):
        raise PartitionManifestFailure(
            f"nested test partitions must use section=tests: {manifest_relative}"
        )

    test_entrypoints = [
        manifest_path.parent.parent / relative
        for relative, section in part_records
        if section == "tests"
    ]
    if len(test_entrypoints) != 1:
        raise PartitionManifestFailure(
            f"exactly one tests entrypoint is required: {manifest_relative}"
        )
    observed_nested = _nested_test_paths(test_entrypoints[0])
    if observed_nested != nested_paths:
        raise PartitionManifestFailure(
            f"nested test partition order drift: expected={nested_paths}, "
            f"actual={observed_nested}"
        )

    crate_name = manifest["crate"]
    expected_crate = manifest_path.parent.parent.parent.name
    if (
        not isinstance(crate_name, str)
        or crate_name != expected_crate
        or not crate_name.strip()
    ):
        raise PartitionManifestFailure(
            f"crate identity drift: expected={expected_crate}, actual={crate_name!r}"
        )


def validate_all(
    repo_root: Path,
    manifests: Iterable[str] = EXPECTED_MANIFESTS,
) -> int:
    root = repo_root.resolve(strict=True)
    manifest_list = tuple(manifests)
    if not manifest_list:
        raise PartitionManifestFailure("partition manifest inventory is empty")
    seen: set[str] = set()
    for relative in manifest_list:
        if relative in seen:
            raise PartitionManifestFailure(
                f"duplicate partition manifest inventory entry: {relative}"
            )
        seen.add(relative)
        validate_manifest(root, relative)
    return len(manifest_list)
