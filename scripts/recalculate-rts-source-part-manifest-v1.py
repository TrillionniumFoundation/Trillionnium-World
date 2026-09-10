#!/usr/bin/env python3
"""Recalculate only RTS source-part records whose physical files changed."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from typing import Any

MANIFEST_RELATIVE = Path(
    "trillionnium/crates/trnm-rts-sim/src/lib_parts/manifest.json"
)
CRATE_RELATIVE = Path("trillionnium/crates/trnm-rts-sim")
TARGETS = {
    "src/lib_parts/tests/part_01.rs",
    "src/lib_parts/tests/rts_tests_01.rs",
    "src/lib_parts/tests/rts_tests_02.rs",
    "src/lib_parts/tests/rts_tests_03.rs",
}
PATH_KEYS = ("path", "file", "source", "relative_path")
SIZE_KEYS = ("bytes", "byte_length", "size_bytes", "length")
DIGEST_KEYS = ("sha256", "sha256_hex", "digest")


def normalized_target(value: str) -> str | None:
    value = value.replace("\\", "/")
    for target in TARGETS:
        if value == target or value.endswith("/" + target) or value == target.removeprefix("src/lib_parts/"):
            return target
    return None


def update_record(record: dict[str, Any], crate_root: Path) -> str | None:
    path_value = None
    for key in PATH_KEYS:
        value = record.get(key)
        if isinstance(value, str):
            target = normalized_target(value)
            if target is not None:
                path_value = target
                break
    if path_value is None:
        return None
    physical = crate_root / path_value
    if not physical.is_file() or physical.is_symlink():
        raise ValueError(f"target source part is not a regular file: {path_value}")
    data = physical.read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    touched = False
    for key in SIZE_KEYS:
        if key in record:
            if not isinstance(record[key], int):
                raise ValueError(f"{path_value}: {key} is not an integer")
            record[key] = len(data)
            touched = True
    for key in DIGEST_KEYS:
        if key in record:
            old = record[key]
            if not isinstance(old, str):
                raise ValueError(f"{path_value}: {key} is not a string")
            if old.startswith("sha256:"):
                record[key] = "sha256:" + digest
            elif len(old) == 64:
                record[key] = digest
            else:
                raise ValueError(f"{path_value}: unsupported {key} representation")
            touched = True
    if not touched:
        raise ValueError(f"{path_value}: source-part record has no size or digest field")
    return path_value


def walk(value: Any, crate_root: Path, updated: list[str]) -> None:
    if isinstance(value, dict):
        target = update_record(value, crate_root)
        if target is not None:
            updated.append(target)
        for child in value.values():
            walk(child, crate_root, updated)
    elif isinstance(value, list):
        for child in value:
            walk(child, crate_root, updated)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: recalculate-rts-source-part-manifest-v1.py REPOSITORY_ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve()
    manifest = root / MANIFEST_RELATIVE
    crate_root = root / CRATE_RELATIVE
    try:
        document = json.loads(manifest.read_text(encoding="utf-8"))
        updated: list[str] = []
        walk(document, crate_root, updated)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
        print(f"RTS_SOURCE_PART_RECALCULATION=FAIL reason={error}", file=sys.stderr)
        return 1
    observed = set(updated)
    missing = sorted(TARGETS - observed)
    duplicates = sorted(target for target in observed if updated.count(target) != 1)
    if missing or duplicates:
        print(
            f"RTS_SOURCE_PART_RECALCULATION=FAIL missing={missing} duplicates={duplicates}",
            file=sys.stderr,
        )
        return 1
    manifest.write_text(
        json.dumps(document, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    print(f"RTS_SOURCE_PART_RECALCULATION=PASS records={len(updated)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
