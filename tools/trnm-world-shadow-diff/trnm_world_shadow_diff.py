#!/usr/bin/env python3
"""Fail-closed World/Nakama deterministic transition shadow comparison."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import sys
from dataclasses import dataclass
from typing import Any, Iterable

CONTRACT_VERSION = "trnm_world_rules_v1"
REQUIRED_FIELDS = {
    "fixture_id",
    "contract_version",
    "ruleset_revision",
    "content_revision",
    "transition_id",
    "request_hash",
    "state_before_hash",
    "command_hash",
    "disposition",
    "error_code",
    "state_after_hash",
    "outcome_hash",
    "replay_hash",
    "steps_used",
    "output_bytes",
    "replay_bytes",
    "transition_hash",
}
COMPARISON_FIELDS = tuple(sorted(REQUIRED_FIELDS - {"fixture_id"}))
HASH_FIELDS = {
    "request_hash",
    "state_before_hash",
    "command_hash",
    "state_after_hash",
    "outcome_hash",
    "replay_hash",
    "transition_hash",
}
TOKEN_FIELDS = {"fixture_id", "ruleset_revision", "content_revision", "transition_id"}


class ShadowDiffError(ValueError):
    pass


@dataclass(frozen=True)
class LoadedRecords:
    source: pathlib.Path
    sha256: str
    records: dict[str, dict[str, Any]]


def file_sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_jsonl(path: pathlib.Path) -> LoadedRecords:
    records: dict[str, dict[str, Any]] = {}
    for line_number, raw_line in enumerate(path.read_text().splitlines(), start=1):
        if not raw_line.strip():
            continue
        try:
            record = json.loads(raw_line)
        except json.JSONDecodeError as error:
            raise ShadowDiffError(f"{path}:{line_number}: invalid JSON: {error}") from error
        validate_record(record, f"{path}:{line_number}")
        fixture_id = record["fixture_id"]
        if fixture_id in records:
            raise ShadowDiffError(f"{path}:{line_number}: duplicate fixture_id {fixture_id}")
        records[fixture_id] = record
    if not records:
        raise ShadowDiffError(f"{path}: no shadow records")
    return LoadedRecords(path, file_sha256(path), records)


def validate_record(record: Any, context: str) -> None:
    if not isinstance(record, dict) or set(record) != REQUIRED_FIELDS:
        actual = sorted(record) if isinstance(record, dict) else type(record).__name__
        raise ShadowDiffError(f"{context}: record fields differ: {actual}")
    if record["contract_version"] != CONTRACT_VERSION:
        raise ShadowDiffError(f"{context}: unsupported contract version")
    for field in TOKEN_FIELDS:
        value = record[field]
        if not isinstance(value, str) or not value or len(value) > 192:
            raise ShadowDiffError(f"{context}: invalid {field}")
        if any(character not in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._:@-" for character in value):
            raise ShadowDiffError(f"{context}: noncanonical {field}")
    for field in HASH_FIELDS:
        value = record[field]
        if not isinstance(value, str) or len(value) != 64 or any(character not in "0123456789abcdef" for character in value):
            raise ShadowDiffError(f"{context}: invalid {field}")
    if record["disposition"] not in {"applied", "rejected"}:
        raise ShadowDiffError(f"{context}: invalid disposition")
    if not isinstance(record["error_code"], str) or not record["error_code"]:
        raise ShadowDiffError(f"{context}: invalid error_code")
    for field in ("steps_used", "output_bytes", "replay_bytes"):
        value = record[field]
        if isinstance(value, bool) or not isinstance(value, int) or value < 0:
            raise ShadowDiffError(f"{context}: invalid {field}")
    if record["disposition"] == "applied" and record["error_code"] != "none":
        raise ShadowDiffError(f"{context}: applied record has an error")
    if record["disposition"] == "rejected":
        if record["error_code"] == "none":
            raise ShadowDiffError(f"{context}: rejected record lacks an error")
        if any(record[field] != "0" * 64 for field in ("state_after_hash", "outcome_hash", "replay_hash")):
            raise ShadowDiffError(f"{context}: rejected record commits game output")
        if any(record[field] != 0 for field in ("steps_used", "output_bytes", "replay_bytes")):
            raise ShadowDiffError(f"{context}: rejected record commits resource usage")


def compare(world: LoadedRecords, candidate: LoadedRecords) -> dict[str, Any]:
    world_ids = set(world.records)
    candidate_ids = set(candidate.records)
    missing = sorted(world_ids - candidate_ids)
    unexpected = sorted(candidate_ids - world_ids)
    mismatches: list[dict[str, Any]] = []
    for fixture_id in sorted(world_ids & candidate_ids):
        world_record = world.records[fixture_id]
        candidate_record = candidate.records[fixture_id]
        fields = [
            field
            for field in COMPARISON_FIELDS
            if world_record[field] != candidate_record[field]
        ]
        if fields:
            mismatches.append(
                {
                    "fixture_id": fixture_id,
                    "fields": fields,
                    "world": {field: world_record[field] for field in fields},
                    "candidate": {field: candidate_record[field] for field in fields},
                }
            )
    passed = not missing and not unexpected and not mismatches
    return {
        "contract_version": CONTRACT_VERSION,
        "comparison_status": "passed" if passed else "failed",
        "world_input": {
            "path": str(world.source),
            "sha256": world.sha256,
            "records": len(world.records),
        },
        "candidate_input": {
            "path": str(candidate.source),
            "sha256": candidate.sha256,
            "records": len(candidate.records),
        },
        "matched_records": len(world_ids & candidate_ids) - len(mismatches),
        "missing_fixture_ids": missing,
        "unexpected_fixture_ids": unexpected,
        "mismatches": mismatches,
        "limitations": [
            "This proves deterministic record equality only for the supplied fixtures.",
            "It does not prove Nakama admission, canonical ordering, signing, Chain finality, or public readiness.",
        ],
    }


def write_summary(summary: dict[str, Any], destination: pathlib.Path | None) -> None:
    encoded = json.dumps(summary, indent=2, sort_keys=True) + "\n"
    if destination is None:
        sys.stdout.write(encoded)
    else:
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(encoded)


def parse_args(argv: Iterable[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--world", required=True, type=pathlib.Path)
    parser.add_argument("--candidate", required=True, type=pathlib.Path)
    parser.add_argument("--summary", type=pathlib.Path)
    return parser.parse_args(argv)


def main(argv: Iterable[str] | None = None) -> int:
    arguments = parse_args(argv)
    try:
        world = load_jsonl(arguments.world)
        candidate = load_jsonl(arguments.candidate)
        summary = compare(world, candidate)
        write_summary(summary, arguments.summary)
        return 0 if summary["comparison_status"] == "passed" else 1
    except (OSError, ShadowDiffError) as error:
        sys.stderr.write(f"trnm-world-shadow-diff: {error}\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
