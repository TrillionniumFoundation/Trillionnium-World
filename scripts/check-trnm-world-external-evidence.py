#!/usr/bin/env python3
"""Validate immutable external evidence without promoting a template into proof."""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import sys
from typing import Any

MAX_BYTES = 2 * 1024 * 1024
MAX_DEPTH = 64
MAX_NODES = 50_000
HEX40 = re.compile(r"[0-9a-f]{40}")
HEX64 = re.compile(r"[0-9a-f]{64}")
CLAIM = re.compile(r"WORLD-(?:V[0-9]+|EXT)-[0-9]{3}")
EVIDENCE_CLASSES = {
    "source_static",
    "unit",
    "database_black_box",
    "single_host_runtime",
    "cross_repository_integration",
    "cross_host",
    "public_network",
    "human",
    "custody_security",
    "commercial_legal",
}
PLACEHOLDERS = (
    "REPLACE_WITH",
    "TBD",
    "TODO",
    "placeholder",
    "example.invalid",
)


class EvidenceError(ValueError):
    pass


def fail(message: str) -> None:
    raise EvidenceError(message)


def duplicate_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def reject_constant(token: str) -> object:
    fail(f"non-finite JSON value: {token}")


def enforce_budget(value: Any) -> None:
    stack: list[tuple[Any, int]] = [(value, 1)]
    nodes = 0
    while stack:
        current, depth = stack.pop()
        nodes += 1
        if nodes > MAX_NODES:
            fail("JSON node budget exceeded")
        if depth > MAX_DEPTH:
            fail("JSON depth budget exceeded")
        if isinstance(current, dict):
            stack.extend((item, depth + 1) for item in current.values())
        elif isinstance(current, list):
            stack.extend((item, depth + 1) for item in current)


def load(path: Path) -> dict[str, Any]:
    if path.is_symlink() or not path.is_file():
        fail(f"record is missing, linked or not regular: {path}")
    raw = path.read_bytes()
    if not raw or len(raw) > MAX_BYTES:
        fail(f"record is empty or oversized: {path}")
    try:
        text = raw.decode("utf-8", errors="strict")
        value = json.loads(text, object_pairs_hook=duplicate_object, parse_constant=reject_constant)
    except (UnicodeError, json.JSONDecodeError) as error:
        fail(f"invalid strict JSON: {error}")
    enforce_budget(value)
    if not isinstance(value, dict):
        fail("record root must be an object")
    if any(marker.lower() in text.lower() for marker in PLACEHOLDERS):
        fail("record still contains placeholder text")
    return value


def text(value: Any, label: str, minimum: int = 1) -> str:
    if not isinstance(value, str) or len(value.strip()) < minimum:
        fail(f"missing or weak {label}")
    return value.strip()


def exact_hex(value: Any, pattern: re.Pattern[str], label: str) -> str:
    value = text(value, label)
    if pattern.fullmatch(value) is None or set(value) == {"0"}:
        fail(f"invalid {label}")
    return value


def timestamp(value: Any, label: str) -> dt.datetime:
    raw = text(value, label)
    try:
        parsed = dt.datetime.fromisoformat(raw.replace("Z", "+00:00"))
    except ValueError as error:
        fail(f"invalid {label}: {raw}")
    if parsed.tzinfo is None:
        fail(f"{label} must include an offset")
    return parsed.astimezone(dt.timezone.utc)


def list_of_text(value: Any, label: str, *, minimum: int = 1) -> list[str]:
    if not isinstance(value, list) or len(value) < minimum:
        fail(f"{label} must contain at least {minimum} entries")
    result = [text(item, f"{label} item", 2) for item in value]
    if len(result) != len(set(result)):
        fail(f"{label} contains duplicates")
    return result


def validate(path: Path, *, now: dt.datetime | None = None) -> dict[str, Any]:
    record = load(path)
    now = now or dt.datetime.now(dt.timezone.utc)
    if record.get("schema") != "trnm_world_evidence_record_v1":
        fail("unexpected evidence schema")
    claim_id = text(record.get("claim_id"), "claim_id")
    if CLAIM.fullmatch(claim_id) is None:
        fail("invalid claim_id")
    evidence_class = text(record.get("evidence_class"), "evidence_class")
    if evidence_class not in EVIDENCE_CLASSES:
        fail("unsupported evidence_class")
    if record.get("decision") not in {"pass", "fail"}:
        fail("decision must be pass or fail; pending records are not evidence")

    source = record.get("source_identity")
    if not isinstance(source, dict):
        fail("source_identity must be an object")
    repository = text(source.get("repository"), "source repository", 5)
    commit = exact_hex(source.get("commit"), HEX40, "source commit")
    tree = exact_hex(source.get("tree"), HEX40, "source tree")
    if source.get("dirty") is not False:
        fail("evidence source must be a clean immutable checkout")

    build = record.get("build_identity")
    if not isinstance(build, dict):
        fail("build_identity must be an object")
    text(build.get("toolchain"), "toolchain", 3)
    exact_hex(build.get("lockfile_sha256"), HEX64, "lockfile_sha256")
    binaries = build.get("binary_or_package_sha256")
    if not isinstance(binaries, list) or not binaries:
        fail("at least one binary/package digest is required")
    for index, digest in enumerate(binaries):
        exact_hex(digest, HEX64, f"binary/package digest {index}")

    environment = record.get("environment")
    if not isinstance(environment, dict):
        fail("environment must be an object")
    environment_class = text(environment.get("class"), "environment class")
    class_map = {
        "cross_host": "cross_host",
        "public_network": "public_network",
        "human": "human",
        "custody_security": "custody_security",
        "commercial_legal": "commercial_legal",
    }
    required_environment = class_map.get(evidence_class)
    if required_environment and environment_class != required_environment:
        fail(f"{evidence_class} evidence requires environment.class={required_environment}")
    if environment.get("secret_values_recorded") is not False:
        fail("secret_values_recorded must be false")
    text(environment.get("topology"), "topology", 8)

    execution = record.get("execution")
    if not isinstance(execution, dict):
        fail("execution must be an object")
    started = timestamp(execution.get("started_at_utc"), "started_at_utc")
    ended = timestamp(execution.get("ended_at_utc"), "ended_at_utc")
    if ended <= started or ended > now + dt.timedelta(minutes=5):
        fail("execution interval is invalid or in the future")
    duration = ended - started
    commands = list_of_text(execution.get("commands"), "commands")
    exit_codes = execution.get("exit_codes")
    if not isinstance(exit_codes, list) or len(exit_codes) != len(commands) or not all(type(code) is int for code in exit_codes):
        fail("exit_codes must be integer-aligned with commands")
    if record.get("decision") == "pass" and any(code != 0 for code in exit_codes):
        fail("passing evidence contains a nonzero command exit")
    if claim_id.endswith("-016") and duration < dt.timedelta(hours=24):
        fail("24-hour endurance evidence is shorter than 24 hours")
    list_of_text(execution.get("thresholds_declared_before_run"), "predeclared thresholds")
    text(execution.get("terminal_summary"), "terminal_summary", 20)

    artifacts = record.get("artifacts")
    if not isinstance(artifacts, list) or not artifacts:
        fail("artifacts must be nonempty")
    for index, artifact in enumerate(artifacts):
        if not isinstance(artifact, dict):
            fail(f"artifact {index} is not an object")
        text(artifact.get("name"), f"artifact {index} name", 3)
        exact_hex(artifact.get("sha256"), HEX64, f"artifact {index} sha256")
        size = artifact.get("size_bytes")
        if type(size) is not int or size <= 0:
            fail(f"artifact {index} size_bytes must be positive")
        text(artifact.get("retention_location"), f"artifact {index} retention location", 8)
        if artifact.get("contains_personal_or_secret_data") not in {True, False}:
            fail(f"artifact {index} data-classification flag is required")

    results = record.get("results")
    if not isinstance(results, dict):
        fail("results must be an object")
    list_of_text(results.get("criteria"), "criteria")
    list_of_text(results.get("observations"), "observations")
    if results.get("all_applicable_criteria_passed") is not (record.get("decision") == "pass"):
        fail("decision disagrees with all_applicable_criteria_passed")
    divergence = results.get("unexplained_divergence_count")
    if divergence is not None and (type(divergence) is not int or divergence < 0):
        fail("unexplained_divergence_count must be null or nonnegative integer")
    if record.get("decision") == "pass" and divergence not in {None, 0}:
        fail("passing evidence contains unexplained divergence")

    review = record.get("review")
    if not isinstance(review, dict):
        fail("review must be an object")
    executor = text(review.get("executor"), "executor", 3)
    reviewer = text(review.get("independent_reviewer"), "independent_reviewer", 3)
    if executor.casefold() == reviewer.casefold() or review.get("reviewer_is_independent") is not True:
        fail("reviewer must be independently identified")
    reviewed_at = timestamp(review.get("reviewed_at_utc"), "reviewed_at_utc")
    if reviewed_at < ended or reviewed_at > now + dt.timedelta(minutes=5):
        fail("review time must follow execution and not be in the future")
    text(review.get("review_comment_or_decision_uri"), "review decision reference", 8)

    validity = record.get("validity")
    if not isinstance(validity, dict):
        fail("validity must be an object")
    expires = timestamp(validity.get("expires_at_utc"), "expires_at_utc")
    if expires <= reviewed_at or expires <= now:
        fail("evidence is expired or has invalid validity interval")
    list_of_text(validity.get("invalidated_by"), "invalidated_by", minimum=3)

    authorization = record.get("authorization")
    if not isinstance(authorization, dict):
        fail("authorization must be an object")
    production = authorization.get("production_authorization")
    if evidence_class == "commercial_legal" and production == "granted_by_explicit_human_decision":
        text(authorization.get("authorized_scope"), "authorized_scope", 8)
        text(authorization.get("accountable_human_decision"), "accountable_human_decision", 8)
    elif production != "not_granted":
        fail("non-final evidence cannot grant production authorization")

    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    return {
        "schema": "trnm_world_external_evidence_validation_v1",
        "status": "pass",
        "record_sha256": digest,
        "claim_id": claim_id,
        "evidence_class": evidence_class,
        "repository": repository,
        "commit": commit,
        "tree": tree,
        "duration_seconds": int(duration.total_seconds()),
        "expires_at_utc": expires.isoformat(),
        "decision": record["decision"],
        "production_authorization": production,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("record", type=Path)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    try:
        result = validate(args.record)
    except (EvidenceError, OSError) as error:
        if args.json:
            print(json.dumps({"schema": "trnm_world_external_evidence_validation_v1", "status": "fail", "error": str(error)}, sort_keys=True))
        else:
            print(f"TRNM World external evidence: FAIL: {error}", file=sys.stderr)
        return 1
    if args.json:
        print(json.dumps(result, sort_keys=True))
    else:
        print(f"TRNM World external evidence: PASS ({result['claim_id']}; {result['record_sha256']})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
