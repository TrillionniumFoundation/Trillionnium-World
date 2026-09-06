#!/usr/bin/env python3
"""Apply the reviewed WORLD-P0-001 mechanical gap-closure patch.

The script is intentionally exact and idempotent. It refuses to continue if the
reviewed legacy source shapes have drifted, so a CI writer cannot silently patch
an unrelated game-server revision.
"""

from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]


def fail(message: str) -> None:
    raise SystemExit(message)


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count == 1:
        return text.replace(old, new)
    if count == 0 and new in text:
        return text
    fail(f"{label}: expected one reviewed source shape, found {count}")


def write_if_changed(path: pathlib.Path, content: str) -> None:
    if not content.endswith("\n"):
        content += "\n"
    if path.read_text(encoding="utf-8") != content:
        path.write_text(content, encoding="utf-8")


lib_path = ROOT / "trillionnium/crates/trnm-game-server/src/lib.rs"
lib = lib_path.read_text(encoding="utf-8")
lib = replace_once(
    lib,
    'const MIGRATION_V15: &str = include_str!("../migrations/0015_online_realtime_hot_path_v1.sql");',
    '''const MIGRATION_V15: &str = include_str!("../migrations/0015_online_realtime_hot_path_v1.sql");
const MIGRATION_V16: &str = include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const MIGRATION_V17: &str =
    include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_V18: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");''',
    "game-server migration constants",
)
lib = replace_once(
    lib,
    '        (15, "0015_online_realtime_hot_path_v1", MIGRATION_V15),\n',
    '''        (15, "0015_online_realtime_hot_path_v1", MIGRATION_V15),
        (16, "0016_online_settlement_outbox_v1", MIGRATION_V16),
        (
            17,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
''',
    "game-server migration ledger",
)
legacy_spawn = '''    let settlement_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = settle_pending_matches(&settlement_state, 2).await {
                tracing::error!(%error, "online authority settlement remains pending");
            }
        }
    });

'''
if legacy_spawn in lib:
    lib = lib.replace(legacy_spawn, "", 1)
elif "settle_pending_matches(&settlement_state" in lib:
    fail("legacy settlement loop drifted from the reviewed source shape")

legacy_signature = (
    "pub async fn settle_pending_matches(state: &AppState, limit: i64) "
    "-> Result<u64, String> {"
)
fail_closed_signature = (
    "pub async fn settle_pending_matches(_state: &AppState, _limit: i64) "
    "-> Result<u64, String> {"
)
if legacy_signature in lib:
    start = lib.index(legacy_signature)
    end_marker = "\nasync fn persist_campaign(\n"
    end = lib.find(end_marker, start)
    if end < 0:
        fail("cannot find reviewed end of legacy settlement function")
    replacement = '''/// Compatibility API retained only to fail closed for downstream callers.
///
/// Terminal economic settlement is owned by the independently deployed
/// `trnm-settlement-worker`. The game-server process must never execute signer
/// or CEX I/O, mutate campaign economic queues, or advance the terminal
/// settlement marker itself.
pub async fn settle_pending_matches(_state: &AppState, _limit: i64) -> Result<u64, String> {
    Err(
        "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited"
            .to_string(),
    )
}
'''
    lib = lib[:start] + replacement + lib[end:]
elif fail_closed_signature not in lib:
    fail("legacy settlement function is neither reviewed legacy nor fail-closed form")
if "reconcile_economy(&state.cex" in lib:
    fail("game-server still contains synchronous CEX reconciliation")
if "settle_pending_matches(&settlement_state" in lib:
    fail("game-server still schedules in-process settlement")
write_if_changed(lib_path, lib)

worker_path = ROOT / "trillionnium/crates/trnm-game-server/src/settlement_worker.rs"
worker = worker_path.read_text(encoding="utf-8")
worker = replace_once(
    worker,
    'const MIGRATION_V17: &str = include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");',
    '''const MIGRATION_V17: &str = include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_V18: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");''',
    "settlement-worker migration constant",
)
worker = replace_once(
    worker,
    '''        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
''',
    '''        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18_i32,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
''',
    "settlement-worker migration ledger",
)
write_if_changed(worker_path, worker)

boundary_path = ROOT / "scripts/check-trnm-settlement-transaction-boundary.sh"
boundary = boundary_path.read_text(encoding="utf-8")
boundary = boundary.replace("if legacy_calls > 1:", "if legacy_calls != 0:")
boundary = boundary.replace(
    "legacy compatibility settlement caller expanded from one to {legacy_calls}",
    "game-server settlement caller must be absent; found {legacy_calls}",
)
old_tail = '''legacy_calls = combined_source.count("reconcile_economy(&state.cex")
if mode == "full" and legacy_calls == 1:
    print(
        "TRNM settlement transaction-boundary check passed: capture/execute/apply, "
        "immutable trigger-derived SHA-256 remote identity, v1 claim retirement, "
        "live-lease fencing and durable evidence retention are enforced; one inert "
        "compatibility caller remains registered for deletion"
    )
else:
    print("TRNM settlement transaction-boundary check passed")
PY
'''
new_tail = '''print(
    "TRNM settlement transaction-boundary check passed: the game-server has no "
    "synchronous CEX caller; capture/execute/apply, stable identity, live-lease "
    "fencing and durable evidence retention are enforced"
)
PY
'''
if old_tail in boundary:
    boundary = boundary.replace(old_tail, new_tail, 1)
elif new_tail not in boundary:
    fail("transaction-boundary checker tail drifted")
write_if_changed(boundary_path, boundary)

migration_debt_path = ROOT / "scripts/check_trnm_settlement_transaction_boundary.sh"
migration_debt = migration_debt_path.read_text(encoding="utf-8")
old_debt = '''# One compatibility call site remains in lib.rs, but the synchronous CexClient
# implementation is fail-closed and cannot perform transport. No second caller
# may appear, and this final caller remains an explicit P0 deletion gate.
mapfile -t legacy_calls < <(
  grep -RInF --include='*.rs' 'reconcile_economy(&state.cex' \\
    trillionnium/crates 2>/dev/null || true
)
[[ "${#legacy_calls[@]}" == "1" ]] || {
  printf '%s\\n' "${legacy_calls[@]:-}" >&2
  fail "expected exactly one inert compatibility settlement call; found ${#legacy_calls[@]}"
}
[[ "${legacy_calls[0]}" == "$legacy_file:"* ]] \\
  || fail "registered compatibility settlement debt moved outside its reviewed file"
'''
new_debt = '''# The game-server process no longer owns terminal economy settlement. Any direct
# synchronous CEX reconciliation is a P0 regression.
mapfile -t legacy_calls < <(
  grep -RInF --include='*.rs' 'reconcile_economy(&state.cex' \\
    trillionnium/crates 2>/dev/null || true
)
[[ "${#legacy_calls[@]}" == "0" ]] || {
  printf '%s\\n' "${legacy_calls[@]:-}" >&2
  fail "game-server synchronous settlement caller returned; found ${#legacy_calls[@]}"
}
'''
if old_debt in migration_debt:
    migration_debt = migration_debt.replace(old_debt, new_debt, 1)
elif new_debt not in migration_debt:
    fail("migration-debt checker legacy block drifted")
migration_debt = migration_debt.replace(
    "TRNM settlement transaction boundary: amber (runtime split, stable identity, lookup-before-submit, account serialization, PostgreSQL tests and operator metrics implemented; legacy caller, CEX owner endpoint and deployed fault evidence remain open)",
    "TRNM settlement transaction boundary: green-source (legacy caller removed; complete migration chain, lookup-before-submit, serialization and audited operator controls enforced; deployment evidence remains separate)",
)
write_if_changed(migration_debt_path, migration_debt)

status = {
    "schema": "trnm_world_settlement_runtime_status_v1",
    "claim_id": "WORLD-P0-001-settlement-fencing-v1",
    "work_item": "WORLD-P0-001",
    "status": "implemented_pending_exact_commit_ci",
    "as_of": "2026-08-28",
    "owner_repository": "TrillionniumFoundation/Trillionnium-World",
    "branch": "fix/world-settlement-gap-closure-v1",
    "base_commit": "ee881a0fec0f40091eaeba67c667ea82ff9d440a",
    "verified_commit": None,
    "authority_profile": "world_legacy_local_alpha",
    "release_effect": "none",
    "public_online": "no_go",
    "public_player_market": "disabled",
    "implemented_controls": [
        "capture_execute_apply_transaction_split",
        "trigger_enforced_sha256_remote_request_identity",
        "stable_remote_request_identity_excludes_capture_generation",
        "settlement_identity_fields_and_aliases_are_immutable",
        "authorization_and_entitlement_nonce_bind_remote_request_identity",
        "authorization_attempt_completion_retry_and_dead_letter_require_live_lease",
        "legacy_claim_v1_fails_closed",
        "signer_receipt_lookup_precedes_sign",
        "cex_receipt_lookup_precedes_submit",
        "cex_receipt_lookup_binds_intent_id_and_hash",
        "ambiguous_remote_commit_fixtures_reuse_one_submit",
        "account_or_campaign_work_is_serialized_without_global_fifo",
        "postgresql_serialization_and_lease_tests_are_required",
        "remote_success_is_distinct_from_campaign_application",
        "operator_backlog_and_age_projection",
        "settlement_evidence_uses_restrictive_foreign_keys",
        "synchronous_cex_economy_backend_fails_closed",
        "static_positive_and_negative_contract_checks",
        "legacy_in_process_settlement_caller_removed",
        "game_server_and_worker_register_migrations_16_through_18",
        "capture_commit_precedes_remote_attempt_postgresql_proof",
        "audited_exact_identity_operator_replay",
        "operator_replay_allows_one_additional_remote_attempt",
        "operator_replay_and_policy_evidence_are_append_only",
        "operator_policy_retention_floor_and_alert_projection",
    ],
    "open_gates": [
        "merge_cex_owner_repository_pull_request",
        "bind_exact_cex_build_and_deployment_artifact",
        "prove_deployed_signer_and_cex_response_loss_recovery",
        "prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix",
        "approve_backup_pitr_restore_and_receipt_retention",
        "obtain_exact_commit_github_actions_evidence",
        "obtain_reviewer_signoff",
    ],
    "required_checks": [
        "trnm-settlement-fencing/static-contracts",
        "trnm-settlement-fencing/rust-contracts",
    ],
    "evidence": {
        "remote_workflow_runs": [],
        "artifacts": [],
        "reviewers": [],
        "limitations": [
            "source implementation is not deployment evidence",
            "CEX owner component remains an unmerged deployment candidate",
            "no production deployment credit",
            "no trusted CEX settlement promotion",
            "no public online or public market credit",
        ],
    },
}
write_if_changed(
    ROOT / "docs/status/settlement-runtime-v1.json",
    json.dumps(status, indent=2) + "\n",
)

checker = r'''#!/usr/bin/env python3
"""Fail-closed validator for the current WORLD-P0-001 source/evidence posture."""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys
from typing import Any

SCHEMA_ID = "trnm_world_settlement_runtime_status_v1"
CLAIM_ID = "WORLD-P0-001-settlement-fencing-v1"
WORK_ITEM = "WORLD-P0-001"
OWNER = "TrillionniumFoundation/Trillionnium-World"
BRANCH = "fix/world-settlement-gap-closure-v1"
BASE = "ee881a0fec0f40091eaeba67c667ea82ff9d440a"
SHA = re.compile(r"^[0-9a-f]{40}$")
TOKEN = re.compile(r"^[a-z0-9][a-z0-9_]*$")
CHECK = re.compile(r"^trnm-settlement-fencing/[a-z0-9-]+$")

REQUIRED_CONTROLS = {
    "capture_execute_apply_transaction_split",
    "trigger_enforced_sha256_remote_request_identity",
    "stable_remote_request_identity_excludes_capture_generation",
    "settlement_identity_fields_and_aliases_are_immutable",
    "authorization_and_entitlement_nonce_bind_remote_request_identity",
    "authorization_attempt_completion_retry_and_dead_letter_require_live_lease",
    "legacy_claim_v1_fails_closed",
    "signer_receipt_lookup_precedes_sign",
    "cex_receipt_lookup_precedes_submit",
    "cex_receipt_lookup_binds_intent_id_and_hash",
    "ambiguous_remote_commit_fixtures_reuse_one_submit",
    "account_or_campaign_work_is_serialized_without_global_fifo",
    "postgresql_serialization_and_lease_tests_are_required",
    "remote_success_is_distinct_from_campaign_application",
    "operator_backlog_and_age_projection",
    "settlement_evidence_uses_restrictive_foreign_keys",
    "synchronous_cex_economy_backend_fails_closed",
    "static_positive_and_negative_contract_checks",
    "legacy_in_process_settlement_caller_removed",
    "game_server_and_worker_register_migrations_16_through_18",
    "capture_commit_precedes_remote_attempt_postgresql_proof",
    "audited_exact_identity_operator_replay",
    "operator_replay_allows_one_additional_remote_attempt",
    "operator_replay_and_policy_evidence_are_append_only",
    "operator_policy_retention_floor_and_alert_projection",
}
REQUIRED_GATES = {
    "merge_cex_owner_repository_pull_request",
    "bind_exact_cex_build_and_deployment_artifact",
    "prove_deployed_signer_and_cex_response_loss_recovery",
    "prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix",
    "approve_backup_pitr_restore_and_receipt_retention",
    "obtain_exact_commit_github_actions_evidence",
    "obtain_reviewer_signoff",
}
REQUIRED_LIMITATIONS = {
    "source implementation is not deployment evidence",
    "CEX owner component remains an unmerged deployment candidate",
    "no production deployment credit",
    "no trusted CEX settlement promotion",
    "no public online or public market credit",
}
REQUIRED_CHECKS = {
    "trnm-settlement-fencing/static-contracts",
    "trnm-settlement-fencing/rust-contracts",
}


def fail(message: str) -> None:
    raise SystemExit(message)


def load(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot load {path}: {error}")
    if not isinstance(value, dict):
        fail(f"{path} must contain one JSON object")
    return value


def string_set(value: Any, field: str, pattern: re.Pattern[str] | None = None) -> set[str]:
    if not isinstance(value, list) or any(not isinstance(item, str) or not item for item in value):
        fail(f"{field} must contain non-empty strings")
    if len(value) != len(set(value)):
        fail(f"{field} contains duplicates")
    if pattern and any(pattern.fullmatch(item) is None for item in value):
        fail(f"{field} contains invalid tokens")
    return set(value)


def require_markers(source: str, markers: tuple[str, ...], context: str) -> None:
    for marker in markers:
        if marker not in source:
            fail(f"{context} lost marker: {marker}")


def read(repo: pathlib.Path, relative: str) -> str:
    path = repo / relative
    try:
        value = path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {relative}: {error}")
    if not value:
        fail(f"required source is empty: {relative}")
    return value


def validate_status(status: dict[str, Any], schema: dict[str, Any]) -> None:
    required = schema.get("required")
    properties = schema.get("properties")
    if schema.get("type") != "object" or schema.get("additionalProperties") is not False:
        fail("status schema must be a closed object")
    if not isinstance(required, list) or not isinstance(properties, dict):
        fail("status schema is incomplete")
    if set(required) != set(properties) or set(status) != set(required):
        fail("status keys do not exactly match the closed schema")
    constants = {
        "schema": SCHEMA_ID,
        "claim_id": CLAIM_ID,
        "work_item": WORK_ITEM,
        "owner_repository": OWNER,
        "branch": BRANCH,
        "base_commit": BASE,
        "authority_profile": "world_legacy_local_alpha",
        "public_online": "no_go",
        "public_player_market": "disabled",
        "status": "implemented_pending_exact_commit_ci",
        "release_effect": "none",
    }
    for field, expected in constants.items():
        if status.get(field) != expected:
            fail(f"{field} must equal {expected!r}")
    if not SHA.fullmatch(status["base_commit"]):
        fail("base_commit is not a lowercase Git SHA")
    if status["verified_commit"] is not None:
        fail("source candidate must not self-claim a verified commit")
    controls = string_set(status["implemented_controls"], "implemented_controls", TOKEN)
    missing = REQUIRED_CONTROLS - controls
    if missing:
        fail(f"implemented controls missing: {sorted(missing)}")
    gates = string_set(status["open_gates"], "open_gates", TOKEN)
    missing = REQUIRED_GATES - gates
    if missing:
        fail(f"open gates hide blockers: {sorted(missing)}")
    checks = string_set(status["required_checks"], "required_checks", CHECK)
    if checks != REQUIRED_CHECKS:
        fail(f"required checks drifted: {sorted(checks)}")
    evidence = status.get("evidence")
    if not isinstance(evidence, dict) or set(evidence) != {
        "remote_workflow_runs", "artifacts", "reviewers", "limitations"
    }:
        fail("evidence object is not closed")
    if evidence["remote_workflow_runs"] or evidence["artifacts"] or evidence["reviewers"]:
        fail("source candidate cannot embed future CI, artifact or reviewer evidence")
    limitations = string_set(evidence["limitations"], "evidence.limitations")
    missing = REQUIRED_LIMITATIONS - limitations
    if missing:
        fail(f"honest limitations missing: {sorted(missing)}")


def validate_source(repo: pathlib.Path) -> None:
    lib = read(repo, "trillionnium/crates/trnm-game-server/src/lib.rs")
    worker = read(repo, "trillionnium/crates/trnm-game-server/src/settlement_worker.rs")
    outbox = read(repo, "trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql")
    runtime = read(repo, "trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql")
    operator = read(repo, "trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql")
    cex = read(repo, "trillionnium/crates/trnm-game-server/src/cex.rs")
    operator_test = read(repo, "trillionnium/crates/trnm-game-server/tests/settlement_operator_controls_database.rs")
    boundary_test = read(repo, "trillionnium/crates/trnm-game-server/tests/settlement_game_server_boundary.rs")
    capture_test = read(repo, "trillionnium/crates/trnm-game-server/tests/settlement_capture_commit_boundary.rs")
    runbook = read(repo, "docs/runbooks/trnm-settlement-operations-v1.md")
    workflow = read(repo, ".github/workflows/trnm-settlement-fencing.yml")

    if "reconcile_economy(&state.cex" in lib:
        fail("game-server reintroduced synchronous CEX settlement")
    if "settle_pending_matches(&settlement_state" in lib:
        fail("game-server reintroduced the in-process settlement loop")
    require_markers(
        lib,
        (
            "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited",
            "0016_online_settlement_outbox_v1",
            "0017_online_settlement_worker_runtime_v1",
            "0018_online_settlement_operator_controls_v1",
        ),
        "game-server",
    )
    require_markers(
        worker,
        (
            "0016_online_settlement_outbox_v1",
            "0017_online_settlement_worker_runtime_v1",
            "0018_online_settlement_operator_controls_v1",
            ".authorize_settlement_intent(",
            ".submit_authorized_settlement_intent(",
        ),
        "settlement worker",
    )
    if "on delete cascade" in outbox:
        fail("settlement evidence uses cascade deletion")
    require_markers(
        outbox,
        (
            "on delete restrict",
            "trnm_online_settlement_jobs",
            "trnm_online_settlement_captures",
        ),
        "outbox migration",
    )
    require_markers(
        runtime,
        (
            "trnm_online_claim_settlement_job_v1 is retired; use v2",
            "lease_expires_at > pg_catalog.clock_timestamp()",
            "trnm_online_settlement_metrics_v1",
            "pending_apply",
            "pg_catalog.pg_try_advisory_xact_lock",
        ),
        "worker migration",
    )
    if runtime.count("lease_expires_at > pg_catalog.clock_timestamp()") < 5:
        fail("worker mutation functions are not all live-lease fenced")
    require_markers(
        operator,
        (
            "trnm_online_authorize_settlement_replay_v1",
            "trnm_online_append_settlement_operator_policy_v1",
            "remote_attempts = 15",
            "receipt-free dead-letter work",
            "settlement operator evidence is append-only",
            "before update or delete",
            "before truncate",
            "retention_days between 365 and 36500",
            "trnm_online_settlement_operator_alerts_v1",
            "revoke all on function",
        ),
        "operator migration",
    )
    require_markers(
        cex,
        (
            "async fn lookup_signer_receipt",
            "async fn lookup_authorized_settlement_receipt",
            "Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())",
        ),
        "World CEX client",
    )
    require_markers(
        operator_test,
        (
            "settlement_operator_replay_is_exact_audited_one_attempt_and_append_only",
            "remote_attempts\"), 15",
            "append-only receipt",
        ),
        "operator PostgreSQL test",
    )
    require_markers(
        boundary_test,
        (
            "game_server_does_not_execute_terminal_economy_settlement",
            "both_runtime_entrypoints_register_the_complete_settlement_migration_chain",
        ),
        "game-server boundary test",
    )
    require_markers(
        capture_test,
        ("uncommitted", "remote"),
        "capture-before-I/O test",
    )
    require_markers(
        runbook,
        (
            "trnm_online_authorize_settlement_replay_v1",
            "exactly one more",
            "minimum retention of 365 days",
            "Direct SQL",
        ),
        "operator runbook",
    )
    require_markers(
        workflow,
        (
            "github.event.pull_request.head.sha || github.sha",
            "settlement_operator_controls_database",
            "settlement_game_server_boundary",
            "actions/upload-artifact@",
            "git rev-parse 'HEAD^{tree}'",
        ),
        "settlement workflow",
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parents[1])
    parser.add_argument("--status", type=pathlib.Path)
    parser.add_argument("--schema", type=pathlib.Path)
    parser.add_argument("--skip-source", action="store_true")
    args = parser.parse_args()
    repo = args.repo.resolve()
    status_path = args.status or repo / "docs/status/settlement-runtime-v1.json"
    schema_path = args.schema or repo / "docs/status/settlement-runtime-v1.schema.json"
    validate_status(load(status_path), load(schema_path))
    if not args.skip_source:
        validate_source(repo)
    print("TRNM WORLD-P0-001 settlement status/source contract: PASS")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as error:  # defensive fail-closed wrapper
        print(f"unexpected settlement status checker failure: {error}", file=sys.stderr)
        raise SystemExit(1)
'''
write_if_changed(ROOT / "scripts/check-trnm-settlement-runtime-status.py", checker)

negative = r'''#!/usr/bin/env python3
"""Negative fixtures for the current WORLD-P0-001 status gate."""

from __future__ import annotations

import copy
import json
import pathlib
import subprocess
import sys
import tempfile
from typing import Any, Callable

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-settlement-runtime-status.py"
STATUS = ROOT / "docs/status/settlement-runtime-v1.json"
SCHEMA = ROOT / "docs/status/settlement-runtime-v1.schema.json"


def invoke(path: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(CHECKER), "--repo", str(ROOT), "--status", str(path),
         "--schema", str(SCHEMA), "--skip-source"],
        check=False,
        capture_output=True,
        text=True,
    )


def verified(value: dict[str, Any]) -> None:
    value["status"] = "verified_remote"
    value["verified_commit"] = "1" * 40


def public_online(value: dict[str, Any]) -> None:
    value["public_online"] = "enabled"


def missing_ci(value: dict[str, Any]) -> None:
    value["open_gates"].remove("obtain_exact_commit_github_actions_evidence")


def missing_cex_merge(value: dict[str, Any]) -> None:
    value["open_gates"].remove("merge_cex_owner_repository_pull_request")


def missing_artifact(value: dict[str, Any]) -> None:
    value["open_gates"].remove("bind_exact_cex_build_and_deployment_artifact")


def missing_legacy_removal(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove("legacy_in_process_settlement_caller_removed")


def missing_capture_proof(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove("capture_commit_precedes_remote_attempt_postgresql_proof")


def missing_operator(value: dict[str, Any]) -> None:
    value["implemented_controls"].remove("audited_exact_identity_operator_replay")


def branch_drift(value: dict[str, Any]) -> None:
    value["branch"] = "main"


def fabricated_evidence(value: dict[str, Any]) -> None:
    value["evidence"]["remote_workflow_runs"] = [1]


def extra_claim(value: dict[str, Any]) -> None:
    value["release_ready"] = True


def release_effect(value: dict[str, Any]) -> None:
    value["release_effect"] = "trusted_cex_settlement_candidate"


def main() -> int:
    baseline = json.loads(STATUS.read_text(encoding="utf-8"))
    cases: list[tuple[str, Callable[[dict[str, Any]], None]]] = [
        ("verified-without-evidence", verified),
        ("public-online-overclaim", public_online),
        ("hidden-ci-gate", missing_ci),
        ("hidden-cex-merge-gate", missing_cex_merge),
        ("hidden-artifact-gate", missing_artifact),
        ("legacy-removal-regression", missing_legacy_removal),
        ("capture-proof-regression", missing_capture_proof),
        ("operator-control-regression", missing_operator),
        ("branch-drift", branch_drift),
        ("fabricated-evidence", fabricated_evidence),
        ("extra-claim", extra_claim),
        ("release-effect-overclaim", release_effect),
    ]
    with tempfile.TemporaryDirectory(prefix="trnm-settlement-status-negative-") as directory:
        root = pathlib.Path(directory)
        baseline_path = root / "baseline.json"
        baseline_path.write_text(json.dumps(baseline, indent=2) + "\n", encoding="utf-8")
        result = invoke(baseline_path)
        if result.returncode != 0:
            print(result.stdout, result.stderr, file=sys.stderr)
            raise SystemExit("baseline settlement status failed")
        for name, mutate in cases:
            fixture = copy.deepcopy(baseline)
            mutate(fixture)
            path = root / f"{name}.json"
            path.write_text(json.dumps(fixture, indent=2) + "\n", encoding="utf-8")
            if invoke(path).returncode == 0:
                raise SystemExit(f"negative fixture unexpectedly passed: {name}")
    print(f"TRNM settlement runtime status negative fixtures: passed ({len(cases)}/{len(cases)} rejected)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
'''
write_if_changed(ROOT / "scripts/test-trnm-settlement-runtime-status-negative.py", negative)

workflow = r'''name: trnm-settlement-fencing

on:
  workflow_dispatch:
  pull_request:
    branches:
      - feature/world-p0-execution-v3
      - fix/world-settlement-fencing-v1
      - fix/world-settlement-recovery-v1
      - fix/world-settlement-cex-component-lock-v1
    paths:
      - '.github/workflows/trnm-settlement-fencing.yml'
      - 'docs/development/trnm-settlement-outbox-v1.md'
      - 'docs/protocol/trnm-settlement-receipt-recovery-v1.md'
      - 'docs/runbooks/trnm-settlement-operations-v1.md'
      - 'docs/status/settlement-runtime-v1.json'
      - 'docs/status/settlement-runtime-v1.schema.json'
      - 'docs/status/settlement-cex-component-lock-v1.json'
      - 'docs/status/settlement-cex-component-lock-v1.schema.json'
      - 'scripts/check-trnm-settlement-runtime-status.py'
      - 'scripts/test-trnm-settlement-runtime-status-negative.py'
      - 'scripts/check-trnm-settlement-cex-component-lock.py'
      - 'scripts/test-trnm-settlement-cex-component-lock-negative.py'
      - 'scripts/check-trnm-settlement-transaction-boundary.sh'
      - 'scripts/check_trnm_settlement_transaction_boundary.sh'
      - 'trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql'
      - 'trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql'
      - 'trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql'
      - 'trillionnium/crates/trnm-game-server/src/lib.rs'
      - 'trillionnium/crates/trnm-game-server/src/bin/trnm-entitlement-signer.rs'
      - 'trillionnium/crates/trnm-game-server/src/bin/trnm-settlement-worker.rs'
      - 'trillionnium/crates/trnm-game-server/src/cex.rs'
      - 'trillionnium/crates/trnm-game-server/src/settlement_worker.rs'
      - 'trillionnium/crates/trnm-game-server/src/signer_protocol.rs'
      - 'trillionnium/crates/trnm-game-server/tests/settlement_*.rs'
  push:
    branches:
      - fix/world-settlement-fencing-v1
      - fix/world-settlement-recovery-v1
      - fix/world-settlement-cex-component-lock-v1
      - fix/world-settlement-gap-closure-v1
    paths:
      - '.github/workflows/trnm-settlement-fencing.yml'
      - 'docs/development/trnm-settlement-outbox-v1.md'
      - 'docs/protocol/trnm-settlement-receipt-recovery-v1.md'
      - 'docs/runbooks/trnm-settlement-operations-v1.md'
      - 'docs/status/settlement-runtime-v1.json'
      - 'docs/status/settlement-runtime-v1.schema.json'
      - 'docs/status/settlement-cex-component-lock-v1.json'
      - 'docs/status/settlement-cex-component-lock-v1.schema.json'
      - 'scripts/check-trnm-settlement-runtime-status.py'
      - 'scripts/test-trnm-settlement-runtime-status-negative.py'
      - 'scripts/check-trnm-settlement-cex-component-lock.py'
      - 'scripts/test-trnm-settlement-cex-component-lock-negative.py'
      - 'scripts/check-trnm-settlement-transaction-boundary.sh'
      - 'scripts/check_trnm_settlement_transaction_boundary.sh'
      - 'trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql'
      - 'trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql'
      - 'trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql'
      - 'trillionnium/crates/trnm-game-server/src/lib.rs'
      - 'trillionnium/crates/trnm-game-server/src/bin/trnm-entitlement-signer.rs'
      - 'trillionnium/crates/trnm-game-server/src/bin/trnm-settlement-worker.rs'
      - 'trillionnium/crates/trnm-game-server/src/cex.rs'
      - 'trillionnium/crates/trnm-game-server/src/settlement_worker.rs'
      - 'trillionnium/crates/trnm-game-server/src/signer_protocol.rs'
      - 'trillionnium/crates/trnm-game-server/tests/settlement_*.rs'

permissions:
  contents: read

concurrency:
  group: trnm-settlement-fencing-${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: never
  CARGO_INCREMENTAL: '0'
  RUST_BACKTRACE: '1'
  RUST_TEST_THREADS: '1'
  EXPECTED_HEAD_SHA: ${{ github.event.pull_request.head.sha || github.sha }}

jobs:
  static-contracts:
    name: trnm-settlement-fencing/static-contracts
    runs-on: ubuntu-24.04
    timeout-minutes: 15
    steps:
      - name: Checkout exact source head
        uses: actions/checkout@fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09
        with:
          ref: ${{ env.EXPECTED_HEAD_SHA }}
          fetch-depth: 1
          persist-credentials: false
      - name: Verify exact commit and tree
        run: |
          test "$(git rev-parse HEAD)" = "${EXPECTED_HEAD_SHA}"
          git rev-parse HEAD
          git rev-parse 'HEAD^{tree}'
      - name: Parse Python and JSON artifacts
        run: |
          python3 -m py_compile \
            scripts/check-trnm-settlement-runtime-status.py \
            scripts/test-trnm-settlement-runtime-status-negative.py \
            scripts/check-trnm-settlement-cex-component-lock.py \
            scripts/test-trnm-settlement-cex-component-lock-negative.py
          python3 - <<'PY'
          import json
          from pathlib import Path
          for path in (
              Path('docs/status/settlement-runtime-v1.json'),
              Path('docs/status/settlement-runtime-v1.schema.json'),
              Path('docs/status/settlement-cex-component-lock-v1.json'),
              Path('docs/status/settlement-cex-component-lock-v1.schema.json'),
          ):
              json.loads(path.read_text(encoding='utf-8'))
          PY
      - name: Validate machine-readable settlement posture
        run: |
          python3 scripts/check-trnm-settlement-runtime-status.py
          python3 scripts/check-trnm-settlement-cex-component-lock.py
      - name: Reject status and lock overclaims
        run: |
          python3 scripts/test-trnm-settlement-runtime-status-negative.py
          python3 scripts/test-trnm-settlement-cex-component-lock-negative.py
      - name: Validate settlement shell syntax
        run: |
          bash -n scripts/check-trnm-settlement-transaction-boundary.sh
          bash -n scripts/check_trnm_settlement_transaction_boundary.sh
      - name: Enforce transaction, identity, recovery and operator boundaries
        run: |
          ./scripts/check-trnm-settlement-transaction-boundary.sh
          ./scripts/check_trnm_settlement_transaction_boundary.sh

  rust-contracts:
    name: trnm-settlement-fencing/rust-contracts
    runs-on: ubuntu-24.04
    timeout-minutes: 75
    services:
      postgres:
        image: postgres:16.4-alpine
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: postgres
        ports:
          - 5432:5432
        options: >-
          --health-cmd "pg_isready -U postgres -d postgres"
          --health-interval 5s
          --health-timeout 5s
          --health-retries 20
    env:
      TRNM_SETTLEMENT_TEST_DATABASE_URL: postgres://postgres:postgres@127.0.0.1:5432/postgres
      TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST: '1'
    defaults:
      run:
        working-directory: trillionnium
    steps:
      - name: Checkout exact source head
        uses: actions/checkout@fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09
        with:
          ref: ${{ env.EXPECTED_HEAD_SHA }}
          fetch-depth: 1
          persist-credentials: false
      - name: Verify exact commit and tree
        working-directory: .
        run: |
          test "$(git rev-parse HEAD)" = "${EXPECTED_HEAD_SHA}"
          git rev-parse HEAD
          git rev-parse 'HEAD^{tree}'
      - name: Install pinned Rust toolchain
        run: |
          rustup toolchain install 1.98.0 --profile minimal --component rustfmt,clippy
          rustup default 1.98.0
          rustc --version --verbose
          cargo --version --verbose
      - name: Resolve locked workspace metadata
        run: cargo metadata --locked --no-deps --format-version 1 > /dev/null
      - name: Format
        run: cargo fmt --all -- --check
      - name: Game-server library and signer contracts
        run: |
          cargo test -p trnm-game-server --lib --locked
          cargo test -p trnm-game-server --bin trnm-entitlement-signer --locked
      - name: Settlement source and worker contracts
        run: |
          cargo test -p trnm-game-server --test settlement_game_server_boundary --locked
          cargo test -p trnm-game-server --test settlement_remote_identity_contract --locked
          cargo test -p trnm-game-server --test settlement_worker_contract --locked
          cargo test -p trnm-game-server --test settlement_fault_model --locked
      - name: Mandatory PostgreSQL contracts
        run: |
          cargo test -p trnm-game-server --test settlement_capture_commit_boundary --locked
          cargo test -p trnm-game-server --test settlement_database_contract --locked
          cargo test -p trnm-game-server --test settlement_serialization_database --locked
          cargo test -p trnm-game-server --test settlement_operator_controls_database --locked
      - name: Strict game-server lint
        run: cargo clippy -p trnm-game-server --all-targets --locked -- -D warnings
      - name: Build settlement executables
        run: |
          cargo build -p trnm-game-server --release --locked \
            --bin trnm-settlement-worker \
            --bin trnm-entitlement-signer
      - name: Assemble exact-head evidence
        working-directory: .
        env:
          WORKFLOW_RUN_ID: ${{ github.run_id }}
          WORKFLOW_RUN_ATTEMPT: ${{ github.run_attempt }}
        run: |
          mkdir -p evidence
          cp trillionnium/target/release/trnm-settlement-worker evidence/
          cp trillionnium/target/release/trnm-entitlement-signer evidence/
          cp trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql evidence/
          cp trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql evidence/
          cp trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql evidence/
          cp docs/status/settlement-runtime-v1.json evidence/
          cp docs/status/settlement-cex-component-lock-v1.json evidence/
          python3 - <<'PY'
          import hashlib, json, os, pathlib, subprocess
          root = pathlib.Path('evidence')
          hashes = {
              path.name: hashlib.sha256(path.read_bytes()).hexdigest()
              for path in sorted(root.iterdir())
          }
          payload = {
              'schema': 'trnm_world_settlement_build_evidence_v1',
              'repository': 'TrillionniumFoundation/Trillionnium-World',
              'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
              'tree': subprocess.check_output(['git', 'rev-parse', 'HEAD^{tree}'], text=True).strip(),
              'workflow_run_id': int(os.environ['WORKFLOW_RUN_ID']),
              'workflow_run_attempt': int(os.environ['WORKFLOW_RUN_ATTEMPT']),
              'rust_toolchain': '1.98.0',
              'postgres_image': 'postgres:16.4-alpine',
              'sha256': hashes,
              'limitations': [
                  'source and build evidence is not deployment evidence',
                  'CEX owner merge and cross-repository deployment lock remain required',
                  'no trusted-settlement, public-online or public-market promotion',
              ],
          }
          canonical = json.dumps(payload, sort_keys=True, separators=(',', ':')).encode()
          payload['payload_sha256'] = hashlib.sha256(canonical).hexdigest()
          (root / 'manifest.json').write_text(json.dumps(payload, indent=2, sort_keys=True) + '\n')
          PY
          sha256sum evidence/* | sort > evidence/SHA256SUMS
      - name: Upload exact-head settlement evidence
        uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        with:
          name: trnm-world-settlement-${{ env.EXPECTED_HEAD_SHA }}
          path: evidence/
          if-no-files-found: error
          retention-days: 30
          compression-level: 9
'''
write_if_changed(ROOT / ".github/workflows/trnm-settlement-fencing.yml", workflow)

print("WORLD-P0-001 mechanical gap-closure patch: APPLIED")
