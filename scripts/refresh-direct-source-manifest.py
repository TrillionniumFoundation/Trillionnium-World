#!/usr/bin/env python3
"""Refresh exact byte length and SHA-256 fields for named direct-source entries."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path, PurePosixPath
import sys
from typing import Any

PATH_KEYS = {"path", "file", "source", "relative_path", "source_path"}


def canonical(value: str) -> str:
    path = PurePosixPath(value)
    if path.is_absolute() or path.as_posix() != value or ".." in path.parts:
        raise ValueError(f"non-canonical manifest path: {value!r}")
    return value


def candidates(root: Path, manifest: Path, source: Path) -> set[str]:
    values = {
        source.relative_to(root).as_posix(),
        source.relative_to(manifest.parent).as_posix(),
        source.name,
    }
    return {canonical(value) for value in values}


def objects(value: Any):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from objects(child)
    elif isinstance(value, list):
        for child in value:
            yield from objects(child)


def path_value(item: dict[str, Any]) -> str | None:
    explicit = [item[key] for key in PATH_KEYS if isinstance(item.get(key), str)]
    if len(explicit) == 1:
        return explicit[0]
    pathish = [
        value
        for key, value in item.items()
        if isinstance(value, str)
        and ("path" in key.lower() or key.lower().endswith("file"))
        and value.endswith(".rs")
    ]
    return pathish[0] if len(pathish) == 1 else None


def update_entry(item: dict[str, Any], source: Path) -> tuple[int, int]:
    data = source.read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    length_updates = 0
    digest_updates = 0
    for key, value in list(item.items()):
        lower = key.lower()
        if isinstance(value, int) and any(token in lower for token in ("byte", "length", "size")):
            item[key] = len(data)
            length_updates += 1
        elif isinstance(value, str) and "sha256" in lower:
            item[key] = ("sha256:" if value.startswith("sha256:") else "") + digest
            digest_updates += 1
    return length_updates, digest_updates


def main() -> int:
    if len(sys.argv) < 4:
        print("usage: refresh-direct-source-manifest.py ROOT MANIFEST SOURCE...", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve(strict=True)
    manifest = Path(sys.argv[2]).resolve(strict=True)
    try:
        manifest.relative_to(root)
    except ValueError:
        print("manifest escapes repository root", file=sys.stderr)
        return 1
    document = json.loads(manifest.read_text(encoding="utf-8"))
    for raw_source in sys.argv[3:]:
        source = Path(raw_source).resolve(strict=True)
        try:
            source.relative_to(root)
        except ValueError:
            print(f"source escapes repository root: {source}", file=sys.stderr)
            return 1
        names = candidates(root, manifest, source)
        matches = [item for item in objects(document) if path_value(item) in names]
        if len(matches) != 1:
            print(
                f"{source.relative_to(root)}: expected one manifest entry, found {len(matches)}; candidates={sorted(names)}",
                file=sys.stderr,
            )
            return 1
        lengths, digests = update_entry(matches[0], source)
        if lengths < 1 or digests < 1:
            print(
                f"{source.relative_to(root)}: manifest entry lacks explicit byte-length/SHA-256 fields",
                file=sys.stderr,
            )
            return 1
        print(
            f"DIRECT_SOURCE_REFRESHED={source.relative_to(root).as_posix()} length_fields={lengths} digest_fields={digests}"
        )
    manifest.write_text(
        json.dumps(document, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
