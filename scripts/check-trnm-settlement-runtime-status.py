#!/usr/bin/env python3
"""Fail-closed validator for WORLD-P0-001 settlement runtime status.

The status document is a source/evidence projection, not a place to hand-edit a
release claim. Source integration, local fixtures, exact-commit CI, deployed
fault evidence, and release promotion remain distinct states.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import re
import sys
from typing import Any

SCHEMA_ID = "trnm_world_settlement_runtime_status_v1"
CLAIM_ID = "WORLD-P0-001-settlement-fencing-v1"
WORK_ITEM = "WORLD-P0-001"
OWNER_REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
AUTHORITY_PROFILE = "world_legacy_local_alpha"
EXPECTED_BRANCH = "fix/world-settlement-recovery-v1"
EXPECTED_BASE = "39e223aa93d55e115353972d3175542a202427e8"
SHA1 = re.compile(r"^[0-9a-f]{40}$")
TOKEN = re.compile(r"^[a-z0-9][a-z0-9_]*$")
CHECK = re.compile(r"^trnm-settlement-fencing/[a-z0-9-]+$")

ALLOWED_STATUSES = {
    "planned",
    "implemented_pending_exact_commit_ci",
    "verified_remote",
    "deployed",
    "operational",
}
PROMOTED_STATUSES = {"verified_remote", "deployed", "operational"}

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
}

REQUIRED_OPEN_GATES = {
    "remove_inert_legacy_reconcile_economy_caller",
    "prove_no_external_request_before_capture_commit_in_postgresql_black_box_test",
    "land_cex_receipt_lookup_endpoint_in_owner_repository",
    "obtain_exact_commit_postgresql_recovery_and_serialization_evidence",
    "prove_deployed_signer_and_cex_response_loss_recovery",
    "prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix",
    "add_reviewed_operator_replay_and_retention_controls",
    "obtain_exact_commit_github_actions_evidence",
    "obtain_reviewer_signoff",
}

REQUIRED_CHECKS = {
    "trnm-settlement-fencing/static-contracts",
    "trnm-settlement-fencing/rust-contracts",
}

REQUIRED_LIMITATIONS = {
    "source implementation is not remote verification",
    "CEX receipt lookup remains an owner-repository dependency",
    "no production deployment credit",
    "no trusted CEX settlement promotion",
    "no public online or public market credit",
}


class ValidationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise ValidationError(message)


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing JSON file: {path}")
    except json.JSONDecodeError as error:
        fail(f"invalid JSON in {path}: {error}")
    if not isinstance(value, dict):
        fail(f"{path} must contain one JSON object")
    return value


def require_exact_keys(value: dict[str, Any], expected: set[str], context: str) -> None:
    actual = set(value)
    missing = sorted(expected - actual)
    extra = sorted(actual - expected)
    if missing or extra:
        fail(f"{context} key mismatch; missing={missing}, extra={extra}")


def require_const(value: Any, expected: Any, field: str) -> None:
    if value != expected:
        fail(f"{field} must equal {expected!r}, got {value!r}")


def require_unique_strings(
    value: Any,
    field: str,
    *,
    pattern: re.Pattern[str] | None = None,
    allow_empty: bool = False,
) -> list[str]:
    if not isinstance(value, list) or (not allow_empty and not value):
        qualifier = "possibly empty " if allow_empty else "non-empty "
        fail(f"{field} must be a {qualifier}array")
    if any(not isinstance(item, str) or not item for item in value):
        fail(f"{field} must contain non-empty strings only")
    if len(value) != len(set(value)):
        fail(f"{field} must not contain duplicates")
    if pattern is not None:
        invalid = [item for item in value if pattern.fullmatch(item) is None]
        if invalid:
            fail(f"{field} contains invalid tokens: {invalid}")
    return value


def validate_schema(schema: dict[str, Any]) -> set[str]:
    if schema.get("type") != "object" or schema.get("additionalProperties") is not False:
        fail("status schema must be a closed JSON object")
    required = schema.get("required")
    properties = schema.get("properties")
    if not isinstance(required, list) or not required or not isinstance(properties, dict):
        fail("status schema must publish required fields and properties")
    if set(required) != set(properties):
        fail("status schema required/properties sets must match exactly")
    consts = {
        "schema": SCHEMA_ID,
        "claim_id": CLAIM_ID,
        "work_item": WORK_ITEM,
        "owner_repository": OWNER_REPOSITORY,
        "authority_profile": AUTHORITY_PROFILE,
        "public_online": "no_go",
        "public_player_market": "disabled",
    }
    for field, expected in consts.items():
        if properties.get(field, {}).get("const") != expected:
            fail(f"status schema lost const {field}={expected}")
    return set(required)


def validate_status(status: dict[str, Any], schema_keys: set[str]) -> None:
    require_exact_keys(status, schema_keys, "settlement status")
    require_const(status["schema"], SCHEMA_ID, "schema")
    require_const(status["claim_id"], CLAIM_ID, "claim_id")
    require_const(status["work_item"], WORK_ITEM, "work_item")
    require_const(status["owner_repository"], OWNER_REPOSITORY, "owner_repository")
    require_const(status["authority_profile"], AUTHORITY_PROFILE, "authority_profile")
    require_const(status["branch"], EXPECTED_BRANCH, "branch")
    require_const(status["base_commit"], EXPECTED_BASE, "base_commit")
    require_const(status["public_online"], "no_go", "public_online")
    require_const(status["public_player_market"], "disabled", "public_player_market")

    try:
        dt.date.fromisoformat(status["as_of"])
    except (TypeError, ValueError):
        fail("as_of must be an ISO-8601 date")

    if status["status"] not in ALLOWED_STATUSES:
        fail(f"unknown status: {status['status']!r}")
    if not SHA1.fullmatch(status["base_commit"]):
        fail("base_commit must be a lowercase 40-character Git SHA")
    verified_commit = status["verified_commit"]
    if verified_commit is not None and not (
        isinstance(verified_commit, str) and SHA1.fullmatch(verified_commit)
    ):
        fail("verified_commit must be null or a lowercase 40-character Git SHA")

    controls = set(
        require_unique_strings(
            status["implemented_controls"], "implemented_controls", pattern=TOKEN
        )
    )
    missing_controls = sorted(REQUIRED_CONTROLS - controls)
    if missing_controls:
        fail(f"implemented_controls lost mandatory controls: {missing_controls}")

    open_gates = set(
        require_unique_strings(status["open_gates"], "open_gates", pattern=TOKEN)
    )
    missing_open_gates = sorted(REQUIRED_OPEN_GATES - open_gates)
    if missing_open_gates:
        fail(f"open_gates hides mandatory blockers: {missing_open_gates}")

    checks = set(
        require_unique_strings(status["required_checks"], "required_checks", pattern=CHECK)
    )
    if checks != REQUIRED_CHECKS:
        fail(f"required_checks must equal {sorted(REQUIRED_CHECKS)}, got {sorted(checks)}")

    evidence = status["evidence"]
    if not isinstance(evidence, dict):
        fail("evidence must be an object")
    require_exact_keys(
        evidence,
        {"remote_workflow_runs", "artifacts", "reviewers", "limitations"},
        "evidence",
    )
    runs = evidence["remote_workflow_runs"]
    if not isinstance(runs, list) or any(
        not isinstance(item, int) or isinstance(item, bool) or item < 1 for item in runs
    ):
        fail("remote_workflow_runs must contain positive integer run IDs")
    if len(runs) != len(set(runs)):
        fail("remote_workflow_runs must not contain duplicates")
    artifacts = require_unique_strings(
        evidence["artifacts"], "evidence.artifacts", allow_empty=True
    )
    reviewers = require_unique_strings(
        evidence["reviewers"], "evidence.reviewers", allow_empty=True
    )
    limitations = set(
        require_unique_strings(evidence["limitations"], "evidence.limitations")
    )
    missing_limitations = sorted(REQUIRED_LIMITATIONS - limitations)
    if missing_limitations:
        fail(f"evidence limitations lost honest boundaries: {missing_limitations}")

    if status["status"] == "implemented_pending_exact_commit_ci":
        require_const(status["release_effect"], "none", "release_effect")
        if verified_commit is not None or runs or artifacts or reviewers:
            fail(
                "implemented_pending_exact_commit_ci must not fabricate verified commit, "
                "workflow, artifact, or reviewer evidence"
            )
    elif status["status"] in PROMOTED_STATUSES:
        if verified_commit is None:
            fail("promoted status requires verified_commit")
        if verified_commit == status["base_commit"]:
            fail("promoted status must bind the exact implementation head, not only its base")
        if not runs or not artifacts or not reviewers:
            fail("promoted status requires workflow runs, artifacts, and reviewer signoff")
        if open_gates:
            fail("promoted status cannot retain unresolved P0 gates")
        if status["release_effect"] != "trusted_cex_settlement_candidate":
            fail("promoted settlement status must use the bounded candidate release effect")
    else:
        require_const(status["release_effect"], "none", "release_effect")


def function_body(source: str, name: str, next_marker: str) -> str:
    marker = f"create or replace function public.{name}"
    start = source.find(marker)
    if start < 0:
        return ""
    end_offset = source[start:].find(next_marker)
    if end_offset < 0:
        return ""
    return source[start : start + end_offset]


def read_required(repo: pathlib.Path, relative: str) -> str:
    path = repo / relative
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read required source {relative}: {error}")
    if not text:
        fail(f"required source is empty: {relative}")
    return text


def require_markers(source: str, markers: tuple[str, ...], context: str) -> None:
    for marker in markers:
        if marker not in source:
            fail(f"{context} lost marker: {marker}")


def validate_source(repo: pathlib.Path) -> None:
    outbox = read_required(
        repo, "trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql"
    )
    worker = read_required(
        repo, "trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql"
    )
    cex = read_required(repo, "trillionnium/crates/trnm-game-server/src/cex.rs")
    signer_protocol = read_required(
        repo, "trillionnium/crates/trnm-game-server/src/signer_protocol.rs"
    )
    signer_binary = read_required(
        repo, "trillionnium/crates/trnm-game-server/src/bin/trnm-entitlement-signer.rs"
    )
    database_test = read_required(
        repo, "trillionnium/crates/trnm-game-server/tests/settlement_database_contract.rs"
    )
    serialization_test = read_required(
        repo, "trillionnium/crates/trnm-game-server/tests/settlement_serialization_database.rs"
    )
    workflow = read_required(repo, ".github/workflows/trnm-settlement-fencing.yml")
    recovery_doc = read_required(repo, "docs/protocol/trnm-settlement-receipt-recovery-v1.md")
    operations_runbook = read_required(repo, "docs/runbooks/trnm-settlement-operations-v1.md")
    normalized_worker = " ".join(worker.split())

    require_markers(
        outbox,
        (
            "references public.trnm_online_matches(match_id) on delete restrict",
            "references public.trnm_online_campaigns(campaign_id) on delete restrict",
        ),
        "outbox retention",
    )
    if "on delete cascade" in outbox:
        fail("outbox source reintroduced cascade deletion")

    identity = function_body(
        worker,
        "trnm_online_remote_request_id_v1",
        "create or replace function public.trnm_online_set_remote_request_id_v1",
    )
    if not identity:
        fail("worker migration lost stable remote request identity function")
    require_markers(
        identity,
        (
            "pg_catalog.sha256(",
            "pg_catalog.encode(",
            "pg_catalog.convert_to(",
            "p_match_id::text",
            "p_campaign_id",
            "p_intent_id",
        ),
        "remote request identity",
    )
    if identity.count("pg_catalog.octet_length(") < 4:
        fail("remote request identity is not length-prefixed")
    for forbidden in ("capture_id", "capture_generation", "intent_hash", "md5("):
        if forbidden in identity:
            fail(f"remote request identity incorrectly depends on {forbidden}")

    require_markers(
        normalized_worker,
        (
            "remote_request_id must be an ordinary stored column",
            "settlement match, campaign and intent identity fields are immutable",
            "remote_request_id does not match durable settlement identity",
            "authorization_request_id is null or authorization_request_id = remote_request_id",
            "entitlement_nonce is null or entitlement_nonce = remote_request_id",
            "trnm_online_claim_settlement_job_v1 is retired; use v2",
            "p_authorization_request_id = remote_request_id",
            "create or replace function public.trnm_online_settlement_serialization_key_v1",
            "pg_catalog.pg_try_advisory_xact_lock",
            "pg_catalog.hashtextextended",
            "create or replace view public.trnm_online_settlement_job_status_v1",
            "create or replace view public.trnm_online_settlement_metrics_v1",
            "when job.state = 'succeeded' then 'pending_apply'",
            "oldest_eligible_age",
            "oldest_pending_apply_age",
        ),
        "worker migration",
    )
    if worker.count("lease_expires_at > pg_catalog.clock_timestamp()") < 5:
        fail("worker migration does not fence every remote mutation with a live lease")

    require_markers(
        cex,
        (
            "async fn lookup_signer_receipt",
            "ENTITLEMENT_SIGNER_RECEIPT_PATH",
            "async fn lookup_authorized_settlement_receipt",
            "CEX_SETTLEMENT_RECEIPT_LOOKUP_PATH",
            "CEX_SETTLEMENT_RECEIPT_LOOKUP_CONTRACT",
            "INTENT_HASH_HEADER",
            "signer_response_loss_recovers_by_lookup_without_a_second_sign",
            "cex_response_loss_recovers_by_lookup_without_a_second_submit",
            "cex_lookup_with_a_mismatched_hash_fails_closed",
            "Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())",
        ),
        "CEX/signer client",
    )
    require_markers(
        signer_protocol,
        ('ENTITLEMENT_SIGNER_RECEIPT_PATH: &str = "/v1/signer/receipts"',),
        "signer protocol",
    )
    require_markers(
        signer_binary,
        (
            '"/v1/signer/receipts/:request_id"',
            "get(get_signing_receipt)",
            "entitlement.nonce != request.request_id",
        ),
        "signer service",
    )
    if "entitlement.intent_id != request.request_id" in signer_binary:
        fail("transport request identity is incorrectly aliased to economic intent identity")

    require_markers(
        database_test,
        (
            "settlement_database_identity_lease_and_retention_contract",
            "TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST",
        ),
        "PostgreSQL identity test",
    )
    require_markers(
        serialization_test,
        (
            "account_serialization_does_not_block_unrelated_work",
            "tokio::join!",
            "trnm_online_settlement_metrics_v1",
            "TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST",
        ),
        "PostgreSQL serialization test",
    )
    require_markers(
        workflow,
        (
            "fix/world-settlement-recovery-v1",
            "postgres:16.4-alpine",
            "TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST: '1'",
            "--test settlement_database_contract",
            "--test settlement_serialization_database",
            "--bin trnm-entitlement-signer",
        ),
        "settlement workflow",
    )
    require_markers(
        recovery_doc,
        (
            "lookup before submit",
            "trnm_cex_settlement_receipt_lookup_v1",
            "/v1/signer/receipts/{request_id}",
        ),
        "receipt recovery protocol",
    )
    require_markers(
        operations_runbook,
        (
            "trnm_online_settlement_metrics_v1",
            "pending_apply",
            "dead_letter",
        ),
        "settlement operations runbook",
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--repo", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parents[1]
    )
    parser.add_argument(
        "--status",
        type=pathlib.Path,
        default=pathlib.Path("docs/status/settlement-runtime-v1.json"),
    )
    parser.add_argument(
        "--schema",
        type=pathlib.Path,
        default=pathlib.Path("docs/status/settlement-runtime-v1.schema.json"),
    )
    parser.add_argument("--skip-source", action="store_true")
    return parser.parse_args()


def resolve(repo: pathlib.Path, path: pathlib.Path) -> pathlib.Path:
    return path if path.is_absolute() else repo / path


def main() -> int:
    args = parse_args()
    repo = args.repo.resolve()
    try:
        schema = load_json(resolve(repo, args.schema))
        status = load_json(resolve(repo, args.status))
        schema_keys = validate_schema(schema)
        validate_status(status, schema_keys)
        if not args.skip_source:
            validate_source(repo)
    except (ValidationError, OSError) as error:
        print(f"TRNM settlement runtime status check failed: {error}", file=sys.stderr)
        return 1

    print(
        "TRNM settlement runtime status: "
        f"{status['status']} ({status['claim_id']}; open_gates={len(status['open_gates'])})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
