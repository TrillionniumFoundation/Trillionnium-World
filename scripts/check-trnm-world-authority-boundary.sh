#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="${1:-full}"
SCAN_ROOT="${2:-$ROOT_DIR}"

python3 - "$ROOT_DIR" "$MODE" "$SCAN_ROOT" <<'PY'
from __future__ import annotations

import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1]).resolve()
mode = sys.argv[2]
scan_root = pathlib.Path(sys.argv[3]).resolve()

if mode not in {"full", "scan-only"}:
    raise SystemExit("usage: check-trnm-world-authority-boundary.sh [full|scan-only] [root]")

errors: list[str] = []

if mode == "full":
    required = {
        root / "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md": [
            "World-local authority is `legacy_local_alpha`",
            "Nakama",
            "Exactly one component is externally authoritative",
        ],
        root / "docs/development/trnm-world-development-plan-v3.md": [
            "current executable plan",
            "External DNS, signer, CEX or HTTP work never executes",
            "public online remains **NO-GO**",
        ],
        root / "PROJECT_BOUNDARY.json": [
            '"project_id": "trillionnium-world"',
            '"external_path_dependencies": "forbid"',
        ],
    }
    for path, markers in required.items():
        if not path.is_file():
            errors.append(f"missing required boundary document: {path.relative_to(root)}")
            continue
        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                errors.append(f"{path.relative_to(root)} is missing marker: {marker}")

forbidden = {
    r"TRNM_NAKAMA_AUTHORITY_PRIVATE_KEY": "Nakama authority private-key custody in World",
    r"NAKAMA_AUTHORITY_PRIVATE_KEY": "Nakama authority private-key custody in World",
    r"WORLD_MATCH_COMPLETED_SIGNING_KEY": "World-owned target completion signing key",
    r"\bsign_match_completed_v1\b": "World-owned MatchCompletedV1 signer",
    r"\bWorldMatchCompletedSigner\b": "World-owned MatchCompletedV1 signer type",
    r"\bworld_canonical_roster_root\b": "competing canonical roster root",
    r"\bworld_canonical_event_root\b": "competing canonical event root",
    r"\bworld_canonical_archive_root\b": "competing canonical archive root",
    r"\bworld_chain_finality_proof\b": "World-local Chain finality claim",
    r"\bworld_chain_inclusion_proof\b": "World-local Chain inclusion claim",
}
compiled = [(re.compile(pattern), reason) for pattern, reason in forbidden.items()]

allowed_suffixes = {".rs", ".toml", ".sh", ".py", ".json", ".yaml", ".yml", ".service"}
skip_parts = {".git", "target", "run", "vendor", "archive", "node_modules", "__pycache__"}

if not scan_root.exists():
    errors.append(f"scan root does not exist: {scan_root}")
else:
    for path in scan_root.rglob("*"):
        if not path.is_file() or path.suffix not in allowed_suffixes:
            continue
        relative = path.relative_to(scan_root)
        if any(part in skip_parts for part in relative.parts):
            continue
        if "docs" in relative.parts:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for pattern, reason in compiled:
            if pattern.search(text):
                errors.append(f"{relative}: forbidden authority boundary: {reason}")
        if path.name == "Cargo.toml":
            for line_no, line in enumerate(text.splitlines(), 1):
                if "path" not in line or "=" not in line:
                    continue
                if re.search(r"(?i)Trillionnium[-_/](Chain|Nakama)|\.\./(?:\.\./)*(?:Trillionnium-)?(?:Chain|Nakama)(?:/|\")", line):
                    errors.append(f"{relative}:{line_no}: sibling Chain/Nakama filesystem dependency is forbidden")

if errors:
    print("TRNM World authority-boundary check failed:", file=sys.stderr)
    for error in errors:
        print(f" - {error}", file=sys.stderr)
    raise SystemExit(1)

print("TRNM World authority-boundary check passed")
PY
