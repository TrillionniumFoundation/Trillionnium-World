#!/usr/bin/env python3
"""Fail-closed validator for WORLD-P0-001 settlement runtime status.

The status document is a source/evidence projection, not a place to hand-edit a
release claim. This checker deliberately validates promotion prerequisites in
addition to basic JSON shape so an empty CI/evidence collection cannot be
reported as verified.
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
EXPECTED_BRANCH = "fix/world-settlement-fencing-v1"
EXPECTED_BASE = "6d9546beed9b075d625849d7f371b9b88ea20f96"
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
    "authorization_and_entitlement_nonce_bind_remote_request_identity",
    "authorization_attempt_completion_retry_and_dead_letter_require_live_lease",
    "legacy_claim_v1_fails_closed",
    "remote_success_is_distinct_from_campaign_application",
    "settlement_evidence_uses_restrictive_foreign_keys",
    "synchronous_cex_economy_backend_fails_closed",
    "static_positive_and_negative_contract_checks",
}

REQUIRED_OPEN_GATES = {
    "remove_inert_legacy_reconcile_economy_caller",
    "prove_no_external_request_before_capture_commit_in_postgresql_black_box_test",
    "prove_stale_lease_and_two_worker_contention_in_postgresql",
    "prove_signer_response_loss_and_cex_ambiguous_commit_recovery",
    "add_exact_remote_receipt_lookup_and_recovery",
    "prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix",
    "add_operator_metrics_dashboards_replay_and_retention_runbooks",
    "obtain_exact_commit_github_actions_evidence",
    "obtain_reviewer_signoff",
}

REQUIRED_CHECKS = {
    "trnm-settlement-fencing/static-contracts",
    "trnm-settlement-fencing/rust-contracts",
}

REQUIRED_LIMITATIONS = {
    "source implementation is not remote verification",
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
    if not isinstance(required, list) or not required:
        fail("status schema must publish required fields")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("status schema must publish properties")
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
            status["implemented_controls"],
            "implemented_controls",
            pattern=TOKEN,
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


def validate_source(repo: pathlib.Path) -> None:
    outbox = (
        repo
        / "trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql"
    ).read_text(encoding="utf-8")
    worker = (
        repo
        / "trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql"
    ).read_text(encoding="utf-8")
    cex = (repo / "trillionnium/crates/trnm-game-server/src/cex.rs").read_text(
        encoding="utf-8"
    )
    normalized_worker = " ".join(worker.split())

    required_outbox = {
        "references public.trnm_online_matches(match_id) on delete restrict",
        "references public.trnm_online_campaigns(campaign_id) on delete restrict",
    }
    for marker in required_outbox:
        if marker not in outbox:
            fail(f"outbox source lost retention marker: {marker}")
    if "on delete cascade" in outbox:
        fail("outbox source reintroduced cascade deletion")

    identity = function_body(
        worker,
        "trnm_online_remote_request_id_v1",
        "create or replace function public.trnm_online_set_remote_request_id_v1",
    )
    if not identity:
        fail("worker migration lost stable remote request identity function")
    for marker in (
        "pg_catalog.sha256(",
        "pg_catalog.encode(",
        "pg_catalog.convert_to(",
        "p_match_id::text",
        "p_campaign_id",
        "p_intent_id",
    ):
        if marker not in identity:
            fail(f"remote request identity lost {marker}")
    if identity.count("pg_catalog.octet_length(") < 4:
        fail("remote request identity is not length-prefixed")
    for forbidden in ("capture_id", "capture_generation", "intent_hash", "md5("):
        if forbidden in identity:
            fail(f"remote request identity incorrectly depends on {forbidden}")

    for marker in (
        "add column if not exists remote_request_id text",
        "remote_request_id must be an ordinary stored column",
        "set remote_request_id = public.trnm_online_remote_request_id_v1(",
        "alter column remote_request_id set not null",
        "message = 'remote_request_id does not match durable settlement identity'",
        "create trigger trnm_online_settlement_remote_id_insert_v1 before insert",
        "create trigger trnm_online_settlement_remote_id_update_v1 before update of match_id, campaign_id, intent_id, remote_request_id",
        "trnm_online_claim_settlement_job_v1 is retired; use v2",
        "p_authorization_request_id = remote_request_id",
        "create or replace view public.trnm_online_settlement_job_status_v1",
        "when job.state = 'succeeded' then 'pending_apply'",
    ):
        if marker not in normalized_worker:
            fail(f"worker migration lost P0 marker: {marker}")
    if worker.count("lease_expires_at > pg_catalog.clock_timestamp()") < 5:
        fail("worker migration does not fence every remote mutation with a live lease")
    if "Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())" not in cex:
        fail("synchronous CEX EconomyBackend no longer fails closed")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--repo",
        type=pathlib.Path,
        default=pathlib.Path(__file__).resolve().parents[1],
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
