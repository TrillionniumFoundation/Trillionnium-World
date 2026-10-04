#!/usr/bin/env python3
"""Validate pending document-catalog additions and fail until folded.

This temporary guard prevents newly added current documentation from becoming an
untracked side catalogue. It should be deleted in the same commit that folds all
rows into docs/catalog.json.
"""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
ADDITIONS = ROOT / "docs/catalog-additions-2026-09-09.json"
CATALOG = ROOT / "docs/catalog.json"


def main() -> int:
    additions = json.loads(ADDITIONS.read_text(encoding="utf-8"))
    catalog = json.loads(CATALOG.read_text(encoding="utf-8"))
    current = {row.get("path") for row in catalog.get("documents", []) if isinstance(row, dict)}
    required = {row.get("path") for row in additions.get("documents", []) if isinstance(row, dict)}
    missing = sorted(path for path in required - current if isinstance(path, str))
    if missing:
        print("TRNM_WORLD_CATALOG_ADDITIONS=FAIL missing=" + ",".join(missing), file=sys.stderr)
        return 1
    print(f"TRNM_WORLD_CATALOG_ADDITIONS=PASS rows={len(required)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
