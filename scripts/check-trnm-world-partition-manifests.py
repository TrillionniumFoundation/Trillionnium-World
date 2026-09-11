#!/usr/bin/env python3
"""Fail closed when reviewed textual Rust partitions drift from their manifests."""

from __future__ import annotations

from pathlib import Path
import sys

from _trnm_world_partition_manifest import (
    PartitionManifestFailure,
    validate_all,
)


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    try:
        count = validate_all(root)
    except (PartitionManifestFailure, OSError, ValueError) as error:
        print(f"TRNM World partition manifests: FAIL: {error}", file=sys.stderr)
        return 1
    print(
        "TRNM World partition manifests: PASS "
        f"(manifests={count}, semantic_generation=false, bytes_and_order_bound=true)"
    )
    print(
        "Manifest integrity freezes the reviewed textual seam; "
        "it does not claim compiler-module migration or production authorization."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
