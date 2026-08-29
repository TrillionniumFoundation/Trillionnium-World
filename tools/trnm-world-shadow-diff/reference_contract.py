#!/usr/bin/env python3
"""Dependency-free reference implementation of trnm_world_rules_v1 commitments."""

from __future__ import annotations

import hashlib
import json
import pathlib
from dataclasses import dataclass
from typing import Any

CONTRACT_VERSION = "trnm_world_rules_v1"
CONTRACT_RELEASE = "trnm_world_rules_v1@1.0.0-alpha.1"
REQUEST_DOMAIN = "TRNM-WORLD-RULES-REQUEST/1"
RESULT_DOMAIN = "TRNM-WORLD-RULES-RESULT/1"
ZERO_HASH = "0" * 64


class ContractError(ValueError):
    pass


def _token(value: Any, maximum: int, field: str) -> str:
    if not isinstance(value, str) or not value or len(value) > maximum:
        raise ContractError(f"{field} is empty or too long")
    allowed = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._:@-")
    if any(character not in allowed for character in value):
        raise ContractError(f"{field} is not a canonical token")
    return value


def _lower_hex(value: Any, maximum_bytes: int, field: str) -> str:
    if not isinstance(value, str) or not value or len(value) % 2:
        raise ContractError(f"{field} must be non-empty even-length lowercase hex")
    if len(value) > maximum_bytes * 2 or any(character not in "0123456789abcdef" for character in value):
        raise ContractError(f"{field} exceeds its ceiling or is not lowercase hex")
    return value


def _positive_int(value: Any, maximum: int, field: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or not 1 <= value <= maximum:
        raise ContractError(f"{field} is outside the contract range")
    return value


def sha256_hex(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def canonical_request(request: dict[str, Any]) -> bytes:
    expected = {
        "contract_version",
        "ruleset_revision",
        "content_revision",
        "transition_id",
        "state_canonical_hex",
        "command_canonical_hex",
        "resource_budget",
    }
    if set(request) != expected:
        raise ContractError(f"request fields differ: expected={sorted(expected)} actual={sorted(request)}")
    if request["contract_version"] != CONTRACT_VERSION:
        raise ContractError("unsupported contract version")
    ruleset = _token(request["ruleset_revision"], 128, "ruleset_revision")
    content = _token(request["content_revision"], 128, "content_revision")
    transition = _token(request["transition_id"], 192, "transition_id")
    state_hex = _lower_hex(request["state_canonical_hex"], 4 * 1024 * 1024, "state_canonical_hex")
    command_hex = _lower_hex(request["command_canonical_hex"], 256 * 1024, "command_canonical_hex")
    budget = request["resource_budget"]
    if not isinstance(budget, dict) or set(budget) != {
        "max_steps",
        "max_output_bytes",
        "max_replay_bytes",
    }:
        raise ContractError("resource_budget fields differ")
    max_steps = _positive_int(budget["max_steps"], 10_000_000, "max_steps")
    max_output = _positive_int(budget["max_output_bytes"], 5 * 1024 * 1024, "max_output_bytes")
    max_replay = _positive_int(budget["max_replay_bytes"], 16 * 1024 * 1024, "max_replay_bytes")
    return (
        f"{REQUEST_DOMAIN}\n"
        f"contract={CONTRACT_VERSION}\n"
        f"ruleset={ruleset}\n"
        f"content={content}\n"
        f"transition={transition}\n"
        f"max_steps={max_steps}\n"
        f"max_output_bytes={max_output}\n"
        f"max_replay_bytes={max_replay}\n"
        f"state={state_hex}\n"
        f"command={command_hex}\n"
    ).encode("ascii")


@dataclass(frozen=True)
class AppliedOutput:
    state_after: bytes
    outcome: bytes
    replay: bytes
    steps_used: int


@dataclass(frozen=True)
class Receipt:
    record: dict[str, Any]
    canonical_without_transition_hash: bytes
    canonical: bytes


def applied_receipt(request: dict[str, Any], output: AppliedOutput) -> Receipt:
    request_bytes = canonical_request(request)
    budget = request["resource_budget"]
    if not output.state_after or len(output.state_after) > 4 * 1024 * 1024:
        raise ContractError("state_after violates contract ceiling")
    if len(output.outcome) > 1024 * 1024 or len(output.replay) > 16 * 1024 * 1024:
        raise ContractError("outcome or replay violates contract ceiling")
    output_bytes = len(output.state_after) + len(output.outcome)
    replay_bytes = len(output.replay)
    if output.steps_used > budget["max_steps"] or output_bytes > budget["max_output_bytes"] or replay_bytes > budget["max_replay_bytes"]:
        raise ContractError("reference output exceeds request budget")

    record: dict[str, Any] = {
        "contract_version": CONTRACT_VERSION,
        "ruleset_revision": request["ruleset_revision"],
        "content_revision": request["content_revision"],
        "transition_id": request["transition_id"],
        "request_hash": sha256_hex(request_bytes),
        "state_before_hash": sha256_hex(bytes.fromhex(request["state_canonical_hex"])),
        "command_hash": sha256_hex(bytes.fromhex(request["command_canonical_hex"])),
        "disposition": "applied",
        "error_code": "none",
        "state_after_hash": sha256_hex(output.state_after),
        "outcome_hash": sha256_hex(output.outcome),
        "replay_hash": sha256_hex(output.replay),
        "steps_used": output.steps_used,
        "output_bytes": output_bytes,
        "replay_bytes": replay_bytes,
    }
    canonical_without = canonical_receipt_without_transition_hash(record)
    record["transition_hash"] = sha256_hex(canonical_without)
    canonical = canonical_without + f"transition_hash={record['transition_hash']}\n".encode("ascii")
    return Receipt(record=record, canonical_without_transition_hash=canonical_without, canonical=canonical)


def rejected_receipt(request: dict[str, Any], error_code: str) -> Receipt:
    request_bytes = canonical_request(request)
    record: dict[str, Any] = {
        "contract_version": CONTRACT_VERSION,
        "ruleset_revision": request["ruleset_revision"],
        "content_revision": request["content_revision"],
        "transition_id": request["transition_id"],
        "request_hash": sha256_hex(request_bytes),
        "state_before_hash": sha256_hex(bytes.fromhex(request["state_canonical_hex"])),
        "command_hash": sha256_hex(bytes.fromhex(request["command_canonical_hex"])),
        "disposition": "rejected",
        "error_code": error_code,
        "state_after_hash": ZERO_HASH,
        "outcome_hash": ZERO_HASH,
        "replay_hash": ZERO_HASH,
        "steps_used": 0,
        "output_bytes": 0,
        "replay_bytes": 0,
    }
    canonical_without = canonical_receipt_without_transition_hash(record)
    record["transition_hash"] = sha256_hex(canonical_without)
    canonical = canonical_without + f"transition_hash={record['transition_hash']}\n".encode("ascii")
    return Receipt(record=record, canonical_without_transition_hash=canonical_without, canonical=canonical)


def canonical_receipt_without_transition_hash(record: dict[str, Any]) -> bytes:
    order = [
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
    ]
    expected = set(order) | ({"transition_hash"} if "transition_hash" in record else set())
    if set(record) != expected:
        raise ContractError("receipt fields differ from canonical catalogue")
    lines = [RESULT_DOMAIN]
    names = {
        "contract_version": "contract",
        "ruleset_revision": "ruleset",
        "content_revision": "content",
        "transition_id": "transition",
    }
    for field in order:
        lines.append(f"{names.get(field, field)}={record[field]}")
    return ("\n".join(lines) + "\n").encode("ascii")


def load_vector(path: pathlib.Path) -> tuple[dict[str, Any], Receipt]:
    vector = json.loads(path.read_text())
    if vector.get("contract_release") != CONTRACT_RELEASE:
        raise ContractError("vector contract release differs")
    request = vector["request"]
    actual_request = canonical_request(request).decode("ascii")
    if actual_request != vector["expected_canonical_request_utf8"]:
        raise ContractError("canonical request differs from committed vector")
    hash_vector = vector["standard_hash_vector"]
    if sha256_hex(hash_vector["input_utf8"].encode("utf-8")) != hash_vector["sha256"]:
        raise ContractError("standard SHA-256 vector differs")
    engine = vector["reference_engine"]
    receipt = applied_receipt(
        request,
        AppliedOutput(
            state_after=bytes.fromhex(engine["state_after_canonical_hex"]),
            outcome=bytes.fromhex(engine["outcome_canonical_hex"]),
            replay=bytes.fromhex(engine["replay_canonical_hex"]),
            steps_used=engine["steps_used"],
        ),
    )
    return vector, receipt
