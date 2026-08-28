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


def sql_function(source: str, name: str, next_marker: str) -> str:
    marker = f"create or replace function public.{name}"
    start = source.find(marker)
    if start < 0:
        return ""
    end_offset = source[start:].find(next_marker)
    if end_offset < 0:
        return ""
    return source[start : start + end_offset]


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
        repo / "trillionnium/crates/trnm-game-server/tests/settlement_remote_identity_contract.rs",
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

    outbox_migration = required_paths[3].read_text(encoding="utf-8")
    worker_migration = required_paths[4].read_text(encoding="utf-8")
    normalized_worker_migration = " ".join(worker_migration.split())
    migration_source = f"{outbox_migration}\n{worker_migration}"
    for required in (
        "trnm_online_settlement_captures",
        "trnm_online_claim_settlement_job_v2",
        "trnm_online_store_settlement_authorization_v1",
        "trnm_online_begin_settlement_remote_attempt_v1",
        "trnm_online_complete_settlement_job_v1",
        "trnm_online_retry_settlement_job_v1",
        "trnm_online_dead_letter_settlement_job_v1",
        "remote_request_id",
        "trnm_online_settlement_job_status_v1",
    ):
        if required not in migration_source:
            errors.append(f"settlement migration lost database contract {required}")

    for forbidden in (
        "references public.trnm_online_matches(match_id) on delete cascade",
        "references public.trnm_online_campaigns(campaign_id) on delete cascade",
    ):
        if forbidden in outbox_migration:
            errors.append(
                "settlement economic evidence may not be removed by upstream cascade"
            )

    identity = sql_function(
        worker_migration,
        "trnm_online_remote_request_id_v1",
        "create or replace function public.trnm_online_set_remote_request_id_v1",
    )
    if not identity:
        errors.append("settlement migration has no stable remote request identity function")
    else:
        for required in (
            "pg_catalog.sha256(",
            "pg_catalog.encode(",
            "pg_catalog.convert_to(",
            "p_match_id::text",
            "p_campaign_id",
            "p_intent_id",
        ):
            if required not in identity:
                errors.append(f"remote request identity lost binding {required}")
        if identity.count("pg_catalog.octet_length(") < 4:
            errors.append("remote request identity is not length-prefixed")
        for forbidden in ("capture_id", "capture_generation", "intent_hash", "md5("):
            if forbidden in identity:
                errors.append(
                    f"remote request identity incorrectly depends on {forbidden}"
                )

    for required in (
        "add column if not exists remote_request_id text",
        "remote_request_id must be an ordinary stored column",
        "set remote_request_id = public.trnm_online_remote_request_id_v1(",
        "alter column remote_request_id set not null",
        "message = 'remote_request_id does not match durable settlement identity'",
        "create trigger trnm_online_settlement_remote_id_insert_v1 before insert",
        "create trigger trnm_online_settlement_remote_id_update_v1 before update of match_id, campaign_id, intent_id, remote_request_id",
        "entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id)",
        "authorization_request_id = coalesce( job.authorization_request_id, job.remote_request_id )",
        "p_authorization_request_id = remote_request_id",
    ):
        if required not in normalized_worker_migration:
            errors.append(f"stable remote retry contract lost marker: {required}")
    if "coalesce(job.authorization_request_id, job.job_id)" in normalized_worker_migration:
        errors.append("capture-scoped job_id is being reused as remote request identity")

    legacy_claim = sql_function(
        worker_migration,
        "trnm_online_claim_settlement_job_v1",
        "create or replace function public.trnm_online_claim_settlement_job_v2",
    )
    normalized_legacy_claim = " ".join(legacy_claim.split())
    for required in (
        "errcode = '0A000'",
        "trnm_online_claim_settlement_job_v1 is retired; use v2",
    ):
        if required not in normalized_legacy_claim:
            errors.append(f"legacy v1 settlement claim is not fail-closed: {required}")
    if "for update skip locked" in normalized_legacy_claim:
        errors.append("legacy v1 settlement claim still leases work")

    lease_functions = (
        (
            "trnm_online_store_settlement_authorization_v1",
            "create or replace function public.trnm_online_begin_settlement_remote_attempt_v1",
        ),
        (
            "trnm_online_begin_settlement_remote_attempt_v1",
            "create or replace function public.trnm_online_complete_settlement_job_v1",
        ),
        (
            "trnm_online_complete_settlement_job_v1",
            "create or replace function public.trnm_online_retry_settlement_job_v1",
        ),
        (
            "trnm_online_retry_settlement_job_v1",
            "create or replace function public.trnm_online_dead_letter_settlement_job_v1",
        ),
        (
            "trnm_online_dead_letter_settlement_job_v1",
            "create or replace view public.trnm_online_settlement_job_status_v1",
        ),
    )
    for function_name, next_marker in lease_functions:
        body = sql_function(worker_migration, function_name, next_marker)
        if not body:
            errors.append(f"cannot inspect settlement SQL function {function_name}")
            continue
        for required in (
            "state = 'leased'",
            "lease_owner = p_owner",
            "lease_generation = p_lease_generation",
            "lease_expires_at > pg_catalog.clock_timestamp()",
        ):
            if required not in body:
                errors.append(f"{function_name} lost live-lease fence {required}")

    for required in (
        "when 'succeeded' then 'remote_succeeded'",
        "when job.state = 'succeeded' then 'pending_apply'",
        "when job.campaign_applied_at is not null then 'applied'",
    ):
        if required not in worker_migration:
            errors.append(f"settlement status projection lost semantic marker: {required}")

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
        "TRNM settlement transaction-boundary check passed: capture/execute/apply, "
        "database-derived SHA-256 remote identity, v1 claim retirement, live-lease "
        "fencing and durable evidence retention are enforced; one inert compatibility "
        "caller remains registered for deletion"
    )
else:
    print("TRNM settlement transaction-boundary check passed")
PY
