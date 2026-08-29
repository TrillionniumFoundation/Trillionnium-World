#!/usr/bin/env python3
"""Fail-closed source/status gate for the WORLD-P0-001 v4 candidate."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import re
import sys
from typing import Any

SCHEMA = "trnm_world_settlement_runtime_status_v1"
CLAIM = "WORLD-P0-001-settlement-fencing-v1"
OWNER = "TrillionniumFoundation/Trillionnium-World"
BRANCH = "fix/world-plan-gap-closure-v4"
BASE = "1d4dee6d5add45a64f5c138f424e3bdab369ecd4"
SHA = re.compile(r"^[0-9a-f]{40}$")
TOKEN = re.compile(r"^[a-z0-9][a-z0-9_]*$")
CHECK = re.compile(r"^trnm-world-v4/[a-z0-9-]+$")

REQUIRED_KEYS = {
    "schema",
    "claim_id",
    "work_item",
    "status",
    "as_of",
    "owner_repository",
    "branch",
    "base_commit",
    "verified_commit",
    "authority_profile",
    "release_effect",
    "public_online",
    "public_player_market",
    "implemented_controls",
    "open_gates",
    "required_checks",
    "evidence",
}
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
    "game_server_and_worker_register_migrations_16_through_19",
    "capture_commit_precedes_remote_attempt_postgresql_proof",
    "audited_exact_identity_operator_replay",
    "operator_replay_allows_one_additional_remote_attempt",
    "operator_replay_and_policy_evidence_are_append_only",
    "operator_policy_retention_floor_and_alert_projection",
    "generated_runtime_source_fails_closed_on_template_drift",
    "sigint_sigterm_stop_new_admission",
    "bounded_shutdown_drain_with_lease_recovery",
    "bounded_parallel_remote_execution",
    "poison_match_job_capture_quarantine",
    "one_campaign_job_per_capture_database_unique",
    "malformed_success_is_ambiguous_retryable",
    "http_conflict_is_lookup_recoverable",
    "bounded_remote_error_body",
    "read_only_pinned_exact_head_ci",
}
REQUIRED_GATES = {
    "run_exact_head_v4_checks",
    "merge_cex_owner_repository_pull_request",
    "bind_exact_cex_build_and_deployment_artifact",
    "prove_deployed_signer_and_cex_response_loss_recovery",
    "prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix",
    "approve_backup_pitr_restore_and_receipt_retention",
    "obtain_reviewer_signoff",
}
REQUIRED_CHECKS = {
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/supply-chain",
}
REQUIRED_LIMITATIONS = {
    "source implementation is not deployment evidence",
    "CEX owner implementation is an unmerged exact-head candidate",
    "external relay evidence does not replace World repository governance",
    "build-time semantic source generation remains tracked migration debt",
    "no production deployment credit",
    "no trusted CEX settlement promotion",
    "no public online or public market credit",
}


class Invalid(RuntimeError):
    pass


def fail(message: str) -> None:
    raise Invalid(message)


def load(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot load {path}: {error}")
    if not isinstance(value, dict):
        fail(f"{path} must contain one object")
    return value


def exact_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    actual = set(value)
    if actual != expected:
        fail(
            f"{label} key drift: missing={sorted(expected - actual)} "
            f"extra={sorted(actual - expected)}"
        )


def string_set(value: Any, field: str, *, allow_empty: bool = False) -> set[str]:
    if not isinstance(value, list) or (not allow_empty and not value):
        fail(f"{field} must be {'possibly empty' if allow_empty else 'non-empty'} array")
    if any(not isinstance(item, str) or not item for item in value):
        fail(f"{field} must contain non-empty strings")
    if len(value) != len(set(value)):
        fail(f"{field} contains duplicates")
    return set(value)


def require_markers(source: str, markers: tuple[str, ...], label: str) -> None:
    missing = [marker for marker in markers if marker not in source]
    if missing:
        fail(f"{label} lost markers: {missing}")


def validate_status(status: dict[str, Any], schema: dict[str, Any]) -> None:
    exact_keys(status, REQUIRED_KEYS, "status")
    if schema.get("type") != "object" or schema.get("additionalProperties") is not False:
        fail("schema must remain a closed object")
    if set(schema.get("required", [])) != set(schema.get("properties", {})):
        fail("schema required/properties drift")

    constants = {
        "schema": SCHEMA,
        "claim_id": CLAIM,
        "work_item": "WORLD-P0-001",
        "owner_repository": OWNER,
        "branch": BRANCH,
        "base_commit": BASE,
        "authority_profile": "world_legacy_local_alpha",
        "public_online": "no_go",
        "public_player_market": "disabled",
        "status": "implemented_pending_exact_commit_ci",
        "release_effect": "none",
        "verified_commit": None,
    }
    for field, expected in constants.items():
        if status.get(field) != expected:
            fail(f"{field} must be {expected!r}, got {status.get(field)!r}")
    if SHA.fullmatch(status["base_commit"]) is None:
        fail("base_commit is not an exact Git SHA")
    try:
        dt.date.fromisoformat(status["as_of"])
    except (TypeError, ValueError):
        fail("as_of must be an ISO date")

    controls = string_set(status["implemented_controls"], "implemented_controls")
    gates = string_set(status["open_gates"], "open_gates")
    checks = string_set(status["required_checks"], "required_checks")
    if any(TOKEN.fullmatch(value) is None for value in controls | gates):
        fail("implemented controls/open gates contain invalid tokens")
    if controls != REQUIRED_CONTROLS:
        fail(f"implemented controls drift: missing={sorted(REQUIRED_CONTROLS-controls)} extra={sorted(controls-REQUIRED_CONTROLS)}")
    if gates != REQUIRED_GATES:
        fail(f"open gates drift: missing={sorted(REQUIRED_GATES-gates)} extra={sorted(gates-REQUIRED_GATES)}")
    if checks != REQUIRED_CHECKS or any(CHECK.fullmatch(value) is None for value in checks):
        fail(f"required checks drift: {sorted(checks)}")

    evidence = status["evidence"]
    if not isinstance(evidence, dict):
        fail("evidence must be an object")
    exact_keys(
        evidence,
        {"remote_workflow_runs", "artifacts", "reviewers", "limitations"},
        "evidence",
    )
    if evidence["remote_workflow_runs"] or evidence["artifacts"] or evidence["reviewers"]:
        fail("candidate source cannot self-invent future evidence")
    limitations = string_set(evidence["limitations"], "evidence.limitations")
    if limitations != REQUIRED_LIMITATIONS:
        fail("evidence limitations drift")


def read(repo: pathlib.Path, relative: str) -> str:
    path = repo / relative
    try:
        value = path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {relative}: {error}")
    if not value:
        fail(f"empty required source: {relative}")
    return value


def validate_source(repo: pathlib.Path) -> None:
    entry = read(repo, "trillionnium/crates/trnm-game-server/src/lib.rs")
    worker_entry = read(repo, "trillionnium/crates/trnm-game-server/src/settlement_worker.rs")
    runtime_v2 = read(repo, "trillionnium/crates/trnm-game-server/src/settlement_worker_runtime_v2.rs")
    build = read(repo, "trillionnium/crates/trnm-game-server/build.rs")
    cargo = read(repo, "trillionnium/crates/trnm-game-server/Cargo.toml")
    worker_sql = read(repo, "trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql")
    operator_sql = read(repo, "trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql")
    quarantine_sql = read(repo, "trillionnium/crates/trnm-game-server/migrations/0019_online_settlement_quarantine_v1.sql")
    cex_entry = read(repo, "trillionnium/crates/trnm-game-server/src/cex.rs")
    cex_template = read(repo, "trillionnium/crates/trnm-game-server/src/cex.rs.in")
    runtime_test = read(repo, "trillionnium/crates/trnm-game-server/tests/settlement_runtime_v2_contract.rs")
    workflow = read(repo, ".github/workflows/trnm-world-gap-closure-v4.yml")

    require_markers(entry, ("trnm_game_server_lib_generated.rs",), "game-server entrypoint")
    require_markers(
        worker_entry,
        ("trnm_settlement_worker_generated.rs", "settlement_worker_runtime_v2.rs"),
        "worker entrypoint",
    )
    require_markers(
        build,
        (
            "WORLD-P0 source transform failed closed",
            "0019_online_settlement_quarantine_v1",
            "run_legacy_disabled",
            "generate_cex",
            "bounded_error_body",
            "StatusCode::CONFLICT",
            "trnm_cex_generated.rs",
        ),
        "generated runtime transform",
    )
    require_markers(
        runtime_v2,
        (
            "SignalKind::terminate",
            "JoinSet::<Result<(), String>>::new",
            "drain_remote_work_v2",
            "trnm_online_quarantine_claimed_settlement_job_v1",
            "trnm_online_settlement_scope_quarantined_v1",
            "trnm_online_record_settlement_quarantine_v1",
        ),
        "runtime v2",
    )
    require_markers(
        worker_sql.lower(),
        (
            "trnm_online_remote_request_id_v1",
            "pg_try_advisory_xact_lock",
            "lease_expires_at > pg_catalog.clock_timestamp()",
            "pending_apply",
        ),
        "worker migration",
    )
    require_markers(
        operator_sql.lower(),
        (
            "trnm_online_settlement_operator_replay_requests",
            "before update or delete",
            "before truncate",
            "retention",
        ),
        "operator controls",
    )
    require_markers(
        quarantine_sql.lower(),
        (
            "idx_trnm_online_settlement_job_one_campaign_per_capture_v1",
            "trnm_online_settlement_quarantine_v1",
            "trnm_online_record_settlement_quarantine_v1",
            "trnm_online_quarantine_claimed_settlement_job_v1",
            "trnm_online_resolve_settlement_quarantine_v1",
        ),
        "quarantine migration",
    )
    require_markers(cex_entry, ("trnm_cex_generated.rs",), "CEX entrypoint")
    require_markers(
        cex_template,
        ("lookup_signer_receipt", "lookup_authorized_settlement_receipt"),
        "CEX recovery template",
    )
    if '"blocking"' in cargo or "reqwest::blocking" in cex_template:
        fail("blocking HTTP support returned")
    require_markers(
        runtime_test,
        (
            "runtime_v2_owns_interruptible_admission_and_bounded_drain",
            "unrelated_remote_work_can_run_concurrently",
            "poison_work_is_quarantined_without_reusing_a_lost_lease",
        ),
        "runtime v2 contract test",
    )
    require_markers(
        workflow,
        (
            "trnm-world-v4/docs-governance",
            "trnm-world-v4/settlement-postgres",
            "trnm-world-v4/supply-chain",
            "contents: read",
            "cargo test -p trnm-game-server --all-targets --locked",
            "cargo clippy -p trnm-game-server --all-targets --locked -- -D warnings",
        ),
        "v4 workflow",
    )
    for forbidden in ("contents: write", "git push", "git commit", "git tag", "clippy --fix"):
        if forbidden in workflow:
            fail(f"v4 workflow contains forbidden mutation: {forbidden}")


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
    try:
        validate_status(load(status_path), load(schema_path))
        if not args.skip_source:
            validate_source(repo)
    except Invalid as error:
        print(f"TRNM settlement runtime status: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM settlement runtime status: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
