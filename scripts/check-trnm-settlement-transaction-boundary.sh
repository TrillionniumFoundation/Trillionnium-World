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

# The scanner extracts Rust function bodies by brace matching. It is not a
# general Rust parser; it is deliberately narrow and fails conservatively for
# the reviewed settlement transport markers.
fn_start = re.compile(
    r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?(?:async[ \t]+)?fn[ \t]+([A-Za-z0-9_]+)[^{;]*\{"
)

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
        if end is None:
            continue
        yield match.group(1), text[match.start():end], path

rust_files = sorted(root.rglob("*.rs"))
if not rust_files:
    raise SystemExit(f"no Rust files found under {root}")

all_functions = []
for path in rust_files:
    try:
        all_functions.extend(functions(path.read_text(encoding="utf-8"), path))
    except UnicodeDecodeError:
        pass

errors: list[str] = []
reconcile_functions = []

transaction_markers = (
    ".begin().await",
    ".begin()\n",
    "Transaction<'",
    "Transaction <'",
    "for update",
    "FOR UPDATE",
    "for share",
    "FOR SHARE",
)
external_markers = (
    ".reconcile_economy(",
    ".execute_authoritative(",
    ".blocking_client",
    ".signer_url",
)

for name, body, path in all_functions:
    has_reconcile = ".reconcile_economy(" in body
    if has_reconcile:
        reconcile_functions.append((name, body, path))
    owns_or_accepts_transaction = any(marker in body for marker in transaction_markers)
    invokes_external = any(marker in body for marker in external_markers)
    if owns_or_accepts_transaction and invokes_external:
        relative = path.relative_to(root)
        errors.append(
            f"{relative}:{name}: external settlement transport/reconciliation appears in transaction-owning code"
        )

if mode == "full":
    if not reconcile_functions:
        # A future fully asynchronous/backend-specific implementation may remove
        # this marker. That change must update this checker and ADR in the same
        # reviewed PR rather than silently bypassing the boundary.
        errors.append("no reconcile_economy call found; update the reviewed boundary checker with the backend migration")
    else:
        for name, body, path in reconcile_functions:
            if "spawn_blocking" not in body and "block_in_place" not in body:
                relative = path.relative_to(root)
                errors.append(
                    f"{relative}:{name}: synchronous economy reconciliation is not on an explicit bounded blocking boundary"
                )

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
