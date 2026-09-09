#!/usr/bin/env python3
"""Fail-closed source/status gate for the WORLD-P0-001 v4 candidate."""

from __future__ import annotations

import argparse
import importlib.util
import datetime as dt
import json
import pathlib
import re
import sys
import tomllib
from typing import Any

SCHEMA = "trnm_world_settlement_runtime_status_v1"
CLAIM = "WORLD-P0-001-settlement-fencing-v1"
OWNER = "TrillionniumFoundation/Trillionnium-World"
BRANCH = "fix/world-plan-v4-development-closure-20260831"
BASE = "c93dad9ff07e5f26c059fb36abdf7095055388e1"
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
    "ordinary_compiled_source_excludes_semantic_generation",
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
    "publish_reviewed_direct_source_and_successor_manifest",
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
    "planned integration inventory is not implemented or verified on the live branch",
    "source implementation is not deployment evidence",
    "retained CEX dependency is unqualified; consult CURRENT_PLAN.md for owner-selected identity",
    "external relay evidence does not replace World repository governance",
    "local direct-source checks do not prove publication on the operative branch",
    "no production deployment credit",
    "no trusted CEX settlement promotion",
    "no public online or public market credit",
}


class Invalid(RuntimeError):
    pass


def fail(message: str) -> None:
    raise Invalid(message)


def load(path: pathlib.Path) -> dict[str, Any]:
    def unique(pairs):
        value = {}
        for key, item in pairs:
            if key in value:
                fail("duplicate JSON key")
            value[key] = item
        return value

    def constant(_value):
        fail("non-finite JSON number")

    try:
        if path.is_symlink() or not path.is_file():
            fail("status/schema source is missing or linked")
        with path.open("rb") as handle:
            data = handle.read(256 * 1024 + 1)
        if not data.strip() or len(data) > 256 * 1024:
            fail("status/schema source is empty or oversized")
        value = json.loads(data.decode("utf-8"), object_pairs_hook=unique, parse_constant=constant)
    except (OSError, UnicodeError, json.JSONDecodeError, RecursionError) as error:
        fail(f"cannot load {path.name}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.name} must contain one object")
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
    if set(schema.get("required", [])) != REQUIRED_KEYS or set(schema.get("properties", {})) != REQUIRED_KEYS:
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
        "status": "planned",
        "release_effect": "none",
        "verified_commit": None,
    }
    for field, expected in constants.items():
        if status.get(field) != expected:
            fail(f"{field} must be {expected!r}, got {status.get(field)!r}")
    if SHA.fullmatch(status["base_commit"]) is None:
        fail("base_commit is not an exact Git SHA")
    try:
        if not isinstance(status["as_of"], str) or re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", status["as_of"]) is None:
            fail("as_of must be YYYY-MM-DD")
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
    for field in ("remote_workflow_runs", "artifacts", "reviewers"):
        if not isinstance(evidence[field], list) or evidence[field]:
            fail("candidate evidence fields must remain empty arrays, never invented evidence")
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
    # Reuse the independently tested direct-source reader; no build template or
    # dynamic generated source is authoritative. Importing writes no bytecode.
    sys.dont_write_bytecode = True
    path = pathlib.Path(__file__).with_name("check-trnm-settlement-transaction-boundary.py")
    if path.is_symlink() or not path.is_file():
        fail("direct-source boundary checker is missing or linked")
    spec = importlib.util.spec_from_file_location("trnm_runtime_boundary", path)
    if spec is None or spec.loader is None:
        fail("cannot load direct-source boundary checker")
    boundary = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = boundary
    spec.loader.exec_module(boundary)
    try:
        boundary.check_repository(repo, full=True)
        reader = boundary.SourceBundle(repo / "trillionnium/crates/trnm-game-server")
        runtime_v2 = reader.read("src/settlement_worker_runtime_v2.rs")
        worker_sql = reader.read("migrations/0017_online_settlement_worker_runtime_v1.sql")
        operator_sql = reader.read("migrations/0018_online_settlement_operator_controls_v1.sql")
        quarantine_sql = reader.read("migrations/0019_online_settlement_quarantine_v1.sql")
        cex = reader.read("src/cex.rs")
        runtime_test = reader.read("tests/settlement_runtime_v2_contract.rs")
        cargo = reader.read("Cargo.toml")
    except (boundary.BoundaryFailure, OSError, UnicodeError, ValueError) as error:
        fail(f"direct-source boundary: {error}")
    require_markers(runtime_v2, (
        "SignalKind::terminate", "JoinSet::<Result<(), String>>::new", "drain_remote_work_v2",
        "trnm_online_quarantine_claimed_settlement_job_v1", "trnm_online_settlement_scope_quarantined_v1",
        "trnm_online_record_settlement_quarantine_v1"), "runtime v2")
    require_markers(worker_sql.lower(), (
        "trnm_online_remote_request_id_v1", "pg_try_advisory_xact_lock",
        "lease_expires_at > pg_catalog.clock_timestamp()", "pending_apply"), "worker migration")
    require_markers(operator_sql.lower(), (
        "trnm_online_settlement_operator_replay_requests", "before update or delete",
        "before truncate", "retention"), "operator controls")
    require_markers(quarantine_sql.lower(), (
        "idx_trnm_online_settlement_job_one_campaign_per_capture_v1", "trnm_online_settlement_quarantine_v1",
        "trnm_online_record_settlement_quarantine_v1", "trnm_online_quarantine_claimed_settlement_job_v1",
        "trnm_online_resolve_settlement_quarantine_v1"), "quarantine migration")
    require_markers(cex, ("bounded_error_body", "StatusCode::CONFLICT",
                         "lookup_signer_receipt", "lookup_authorized_settlement_receipt"), "direct CEX recovery")
    manifest = tomllib.loads(cargo)
    reqwest = manifest.get("dependencies", {}).get("reqwest", {})
    if not isinstance(reqwest, dict):
        fail("reqwest dependency policy must be explicit")
    features = reqwest.get("features", [])
    if not isinstance(features, list) or "blocking" in features:
        fail("blocking HTTP support returned")
    try:
        stream = boundary.tokens(runtime_test)
        for name in ("runtime_v2_owns_interruptible_admission_and_bounded_drain",
                     "unrelated_remote_work_can_run_concurrently",
                     "poison_work_is_quarantined_without_reusing_a_lost_lease"):
            boundary.function(stream, name)
    except boundary.BoundaryFailure as error:
        fail(f"runtime test inventory: {error}")
    workflow = read(repo, ".github/workflows/trnm-world-gap-closure-v4.yml")
    require_markers(workflow, (
        "trnm-world-v4/docs-governance", "trnm-world-v4/settlement-postgres", "trnm-world-v4/supply-chain",
        "contents: read", "cargo test -p trnm-game-server --all-targets --locked",
        "cargo clippy -p trnm-game-server --all-targets --locked -- -D warnings"), "v4 workflow")
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
    except (Invalid, OSError, UnicodeError, ValueError, TypeError, RecursionError) as error:
        print(f"TRNM settlement runtime status: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM settlement runtime status: PASS (source/status only; publication, CI, deployment and review remain unverified)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
