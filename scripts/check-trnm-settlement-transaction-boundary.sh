#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="${1:-full}"
SCAN_ROOT="${2:-$ROOT_DIR/trillionnium/crates/trnm-game-server/src}"

python3 - "$MODE" "$SCAN_ROOT" <<'PY'
from __future__ import annotations

import pathlib
import re
import sys

mode = sys.argv[1]
root = pathlib.Path(sys.argv[2]).resolve()
if mode not in {"full", "scan-only"}:
    raise SystemExit("usage: check-trnm-settlement-transaction-boundary.sh [full|scan-only] [root]")
if not root.exists():
    raise SystemExit(f"settlement scan root does not exist: {root}")

fn_start = re.compile(r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?(?:async[ \t]+)?fn[ \t]+([A-Za-z0-9_]+)[^{;]*\{")

def functions(text: str, path: pathlib.Path):
    for match in fn_start.finditer(text):
        depth = 0
        in_string = False
        escaped = False
        index = match.end() - 1
        end = None
        while index < len(text):
            char = text[index]
            if in_string:
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == '"':
                    in_string = False
            else:
                if char == '"':
                    in_string = True
                elif char == "{":
                    depth += 1
                elif char == "}":
                    depth -= 1
                    if depth == 0:
                        end = index + 1
                        break
            index += 1
        if end is not None:
            yield match.group(1), text[match.start():end], path

rust_files = sorted(root.rglob("*.rs"))
if not rust_files:
    raise SystemExit(f"no Rust files found under {root}")

all_functions = []
combined = []
for path in rust_files:
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        continue
    combined.append(text)
    all_functions.extend(functions(text, path))
combined_source = "\n".join(combined)

errors: list[str] = []
reconcile_count = 0
transaction_markers = (".begin().await", "Transaction<'", "Transaction <'", "FOR UPDATE", "for update", "FOR SHARE", "for share")
external_markers = (".reconcile_economy(", ".execute_authoritative(", ".blocking_client")

for name, body, path in all_functions:
    if ".reconcile_economy(" in body:
        reconcile_count += 1
    if any(marker in body for marker in transaction_markers) and any(marker in body for marker in external_markers):
        errors.append(f"{path.relative_to(root)}:{name}: external settlement work appears in transaction-owning code")

if mode == "full":
    if reconcile_count == 0:
        errors.append("no reconcile_economy call found; update ADR-0002 and this reviewed checker with the backend replacement")
    if "spawn_blocking" not in combined_source and "block_in_place" not in combined_source:
        errors.append("synchronous economy reconciliation has no explicit blocking execution boundary")
    settlement_names = {name for name, _, _ in all_functions if "settlement" in name}
    if not any("capture" in name for name in settlement_names):
        errors.append("no settlement capture function found")
    if not any("apply" in name for name in settlement_names):
        errors.append("no settlement apply function found")

if errors:
    print("TRNM settlement transaction-boundary check failed:", file=sys.stderr)
    for error in errors:
        print(f" - {error}", file=sys.stderr)
    raise SystemExit(1)

print("TRNM settlement transaction-boundary check passed")
PY
