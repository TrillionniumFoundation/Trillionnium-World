#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="${1:-full}"
SCAN_ROOT="${2:-$ROOT_DIR/trillionnium/crates/trnm-game-server/src}"

python3 - "$MODE" "$SCAN_ROOT" "$ROOT_DIR" <<'PY'
from __future__ import annotations

import pathlib
import re
import sys

mode = sys.argv[1]
root = pathlib.Path(sys.argv[2]).resolve()
repo = pathlib.Path(sys.argv[3]).resolve()
if mode not in {"full", "scan-only"}:
    raise SystemExit(
        "usage: check-trnm-settlement-transaction-boundary.sh "
        "[full|scan-only] [root]"
    )
if not root.exists():
    raise SystemExit(f"settlement scan root does not exist: {root}")

fn_start = re.compile(
    r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?"
    r"(?:async[ \t]+)?fn[ \t]+([A-Za-z0-9_]+)[^{;]*\{"
)


def functions(text: str, path: pathlib.Path):
    for match in fn_start.finditer(text):
        depth = 0
        in_string = False
        in_char = False
        escaped = False
        index = match.end() - 1
        end = None
        while index < len(text):
            char = text[index]
            if in_string or in_char:
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif in_string and char == '"':
                    in_string = False
                elif in_char and char == "'":
                    in_char = False
            else:
                if char == '"':
                    in_string = True
                elif char == "'":
                    in_char = True
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

transaction_pattern = re.compile(
    r"\.begin\s*\(\s*\)\s*\.await"
    r"|Transaction\s*<"
    r"|for\s+(?:no\s+key\s+)?update"
    r"|for\s+share",
    re.IGNORECASE | re.DOTALL,
)
remote_pattern = re.compile(
    r"\.authorize_settlement_intent\s*\("
    r"|\.submit_authorized_settlement_intent\s*\("
    r"|\.execute_authoritative\s*\("
    r"|\.send\s*\(\s*\)\s*\.await"
    r"|reqwest::blocking"
    r"|blocking_client",
    re.DOTALL,
)

errors: list[str] = []
function_map: dict[str, list[tuple[str, pathlib.Path]]] = {}
for name, body, path in all_functions:
    function_map.setdefault(name, []).append((body, path))
    if transaction_pattern.search(body) and remote_pattern.search(body):
        errors.append(
            f"{path.relative_to(root)}:{name}: signer/CEX transport appears in "
            "transaction-owning code"
        )

if mode == "full":
    required_paths = [
        repo / "trillionnium/crates/trnm-game-server/src/settlement_worker.rs",
        repo / "trillionnium/crates/trnm-game-server/src/bin/trnm-settlement-worker.rs",
        repo / "trillionnium/crates/trnm-game-server/src/cex.rs",
        repo / "trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql",
        repo / "trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql",
    ]
    for path in required_paths:
        if not path.is_file() or path.stat().st_size == 0:
            errors.append(f"missing settlement runtime component: {path.relative_to(repo)}")

    for required_name in ("capture_match", "apply_capture", "process_claimed_job"):
        if required_name not in function_map:
            errors.append(f"missing settlement phase function {required_name}")

    for phase_name in ("capture_match", "apply_capture"):
        for body, path in function_map.get(phase_name, []):
            if not transaction_pattern.search(body):
                errors.append(
                    f"{path.relative_to(root)}:{phase_name}: phase does not own its "
                    "short PostgreSQL transaction"
                )
            if remote_pattern.search(body):
                errors.append(
                    f"{path.relative_to(root)}:{phase_name}: phase performs remote I/O"
                )

    execute_bodies = function_map.get("process_claimed_job", [])
    if execute_bodies:
        for body, path in execute_bodies:
            if transaction_pattern.search(body):
                errors.append(
                    f"{path.relative_to(root)}:process_claimed_job: execute phase owns "
                    "a PostgreSQL transaction"
                )
            for marker in (
                ".authorize_settlement_intent(",
                ".submit_authorized_settlement_intent(",
            ):
                if marker not in body:
                    errors.append(
                        f"{path.relative_to(root)}:process_claimed_job: missing {marker}"
                    )

    cex_source = (repo / "trillionnium/crates/trnm-game-server/src/cex.rs").read_text(
        encoding="utf-8"
    )
    if "Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())" not in cex_source:
        errors.append("synchronous CexClient EconomyBackend no longer fails closed")
    if "blocking_client" in cex_source or "reqwest::blocking" in cex_source:
        errors.append("CEX client reintroduced blocking HTTP transport")

    worker_source = (
        repo / "trillionnium/crates/trnm-game-server/src/settlement_worker.rs"
    ).read_text(encoding="utf-8")
    for required in (
        "expected_campaign_state_hash",
        "authorization_request_id",
        "entitlement_issued_at_epoch",
        "entitlement_expires_at_epoch",
        "entitlement_nonce",
        "campaign_applied_at",
        "ReceiptProgressionClass::RecoverableHold",
    ):
        if required not in worker_source:
            errors.append(f"settlement worker lost durable invariant {required}")

    migration_source = "\n".join(
        path.read_text(encoding="utf-8") for path in required_paths[-2:]
    )
    for required in (
        "trnm_online_settlement_captures",
        "trnm_online_claim_settlement_job_v2",
        "trnm_online_store_settlement_authorization_v1",
        "trnm_online_begin_settlement_remote_attempt_v1",
        "trnm_online_complete_settlement_job_v1",
        "trnm_online_retry_settlement_job_v1",
        "trnm_online_dead_letter_settlement_job_v1",
    ):
        if required not in migration_source:
            errors.append(f"settlement migration lost database contract {required}")

    legacy_calls = combined_source.count("reconcile_economy(&state.cex")
    if legacy_calls > 1:
        errors.append(
            f"legacy compatibility settlement caller expanded from one to {legacy_calls}"
        )

if errors:
    print("TRNM settlement transaction-boundary check failed:", file=sys.stderr)
    for error in errors:
        print(f" - {error}", file=sys.stderr)
    raise SystemExit(1)

legacy_calls = combined_source.count("reconcile_economy(&state.cex")
if mode == "full" and legacy_calls == 1:
    print(
        "TRNM settlement transaction-boundary check passed: runtime worker is split; "
        "one inert compatibility reconciliation caller remains registered for deletion"
    )
else:
    print("TRNM settlement transaction-boundary check passed")
PY
