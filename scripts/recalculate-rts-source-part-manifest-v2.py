#!/usr/bin/env python3
"""Recalculate existing RTS source-part records touched by the module migration."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from typing import Any

MANIFEST_RELATIVE = Path("trillionnium/crates/trnm-rts-sim/src/lib_parts/manifest.json")
CRATE_RELATIVE = Path("trillionnium/crates/trnm-rts-sim")
TARGETS = {
    "src/lib_parts/tests/part_01.rs",
    "src/lib_parts/tests/rts_tests_01.rs",
    "src/lib_parts/tests/rts_tests_02.rs",
    "src/lib_parts/tests/rts_tests_03.rs",
}
REQUIRED_RECORDS = {"src/lib_parts/tests/part_01.rs"}
PATH_KEYS = ("path", "file", "source", "relative_path")
SIZE_KEYS = ("bytes", "byte_length", "size_bytes", "length")
DIGEST_KEYS = ("sha256", "sha256_hex", "digest")


def normalized_target(value: str) -> str | None:
    value = value.replace("\\", "/")
    for target in TARGETS:
        short = target.removeprefix("src/lib_parts/")
        if value in {target, short} or value.endswith("/" + target):
            return target
    return None


def update_record(record: dict[str, Any], crate_root: Path) -> str | None:
    target = None
    for key in PATH_KEYS:
        value = record.get(key)
        if isinstance(value, str):
            target = normalized_target(value)
            if target is not None:
                break
    if target is None:
        return None
    physical = crate_root / target
    if not physical.is_file() or physical.is_symlink():
        raise ValueError(f"target source part is not a regular file: {target}")
    data = physical.read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    fields = 0
    for key in SIZE_KEYS:
        if key in record:
            if not isinstance(record[key], int):
                raise ValueError(f"{target}: {key} is not an integer")
            record[key] = len(data)
            fields += 1
    for key in DIGEST_KEYS:
        if key in record:
            old = record[key]
            if not isinstance(old, str):
                raise ValueError(f"{target}: {key} is not a string")
            if old.startswith("sha256:"):
                record[key] = "sha256:" + digest
            elif len(old) == 64:
                record[key] = digest
            else:
                raise ValueError(f"{target}: unsupported {key} representation")
            fields += 1
    if fields == 0:
        raise ValueError(f"{target}: record contains no recognized integrity field")
    return target


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
        print("usage: recalculate-rts-source-part-manifest-v2.py REPOSITORY_ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve()
    manifest = root / MANIFEST_RELATIVE
    try:
        document = json.loads(manifest.read_text(encoding="utf-8"))
        updated: list[str] = []
        walk(document, root / CRATE_RELATIVE, updated)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
        print(f"RTS_SOURCE_PART_RECALCULATION=FAIL reason={error}", file=sys.stderr)
        return 1
    observed = set(updated)
    missing = sorted(REQUIRED_RECORDS - observed)
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
    print(
        "RTS_SOURCE_PART_RECALCULATION=PASS updated="
        + ",".join(sorted(observed))
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
