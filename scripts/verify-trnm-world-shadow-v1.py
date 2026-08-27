#!/usr/bin/env python3
"""Independent standard-library verifier for World/Nakama shadow vectors.

This verifier compares unsigned deterministic game-domain observations only. It
does not authenticate participants, establish global order, sign completion,
prove Chain finality, or settle CEX balances.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import unicodedata
from pathlib import Path
from typing import Any

RUNTIME_VERSION = "trnm_world_runtime_v1"
RUNTIME_ERROR_VERSION = "trnm_world_runtime_error_v1"
OBSERVATION_VERSION = "trnm_world_runtime_observation_v1"
SHADOW_INPUT_VERSION = "trnm_world_shadow_input_v1"
SHADOW_REQUEST_DOMAIN = "trnm.world.shadow.v1.request"
FINAL_STATE_DOMAIN = "trnm.world.runtime.v1.final_state"
OUTCOME_DOMAIN = "trnm.world.runtime.v1.outcome"
REPLAY_DOMAIN = "trnm.world.runtime.v1.replay_material"
I64_MIN = -(2**63)
I64_MAX = 2**63 - 1
FORBIDDEN_AUTHORITY_FIELDS = {
    "participant_roster",
    "participant_roles",
    "global_sequence",
    "event_root",
    "roster_root",
    "archive_root",
    "completion_signature",
    "authority_key_id",
    "chain_finality",
    "inclusion_proof",
    "wallet_balance",
    "session_token",
    "idempotency_receipt",
}


class ContractError(ValueError):
    pass


def canonicalize(value: Any) -> Any:
    if value is None or isinstance(value, bool):
        return value
    if isinstance(value, int):
        if not I64_MIN <= value <= I64_MAX:
            raise ContractError("integer is outside signed 64-bit range")
        return value
    if isinstance(value, float):
        raise ContractError("floating-point numbers are forbidden")
    if isinstance(value, str):
        return unicodedata.normalize("NFC", value)
    if isinstance(value, list):
        return [canonicalize(item) for item in value]
    if isinstance(value, dict):
        normalized: dict[str, Any] = {}
        for raw_key, item in value.items():
            if not isinstance(raw_key, str):
                raise ContractError("object keys must be strings")
            key = unicodedata.normalize("NFC", raw_key)
            if key in normalized:
                raise ContractError(f"normalized object key collision: {key}")
            normalized[key] = canonicalize(item)
        return normalized
    raise ContractError(f"unsupported canonical value: {type(value).__name__}")


def canonical_bytes(value: Any) -> bytes:
    normalized = canonicalize(value)
    return json.dumps(
        normalized,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def domain_hash(domain: str, value: Any) -> str:
    return hashlib.sha256(domain.encode("ascii") + b"\n" + canonical_bytes(value)).hexdigest()


def exact_object(value: Any, fields: set[str], label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ContractError(f"{label} must be an object")
    unknown = set(value) - fields
    if unknown:
        field = sorted(unknown)[0]
        if field in FORBIDDEN_AUTHORITY_FIELDS:
            raise ContractError(f"{label} contains forbidden authority field {field}")
        raise ContractError(f"unknown field in {label}: {field}")
    missing = fields - set(value)
    if missing:
        raise ContractError(f"missing field in {label}: {sorted(missing)[0]}")
    return value


def require_hex(value: Any, length: int, label: str) -> str:
    if not isinstance(value, str) or len(value) != length or any(
        char not in "0123456789abcdef" for char in value
    ):
        raise ContractError(f"{label} must be lowercase {length}-hex")
    return value


def require_identifier(value: Any, label: str) -> str:
    if not isinstance(value, str) or not (1 <= len(value) <= 128):
        raise ContractError(f"{label} is not a portable identifier")
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789._:-")
    if value[0] not in set("abcdefghijklmnopqrstuvwxyz0123456789") or any(
        char not in allowed for char in value
    ):
        raise ContractError(f"{label} is not a portable identifier")
    return value


def reject_authority(value: Any, label: str) -> None:
    if isinstance(value, dict):
        for key, item in value.items():
            if key in FORBIDDEN_AUTHORITY_FIELDS:
                raise ContractError(f"{label} contains forbidden authority field {key}")
            reject_authority(item, label)
    elif isinstance(value, list):
        for item in value:
            reject_authority(item, label)


def validate_response(value: Any) -> str:
    reject_authority(value, "runtime response")
    canonical_bytes(value)
    if not isinstance(value, dict):
        raise ContractError("runtime response must be an object")
    version = value.get("contract_version")
    if version == RUNTIME_VERSION:
        fields = {
            "contract_version",
            "message_type",
            "ruleset",
            "content_digest",
            "initial_state_hash",
            "command_batch_hash",
            "final_state",
            "final_state_hash",
            "outcome",
            "outcome_hash",
            "replay_material",
            "replay_material_hash",
        }
        result = exact_object(value, fields, "execute result")
        if result["message_type"] != "execute_result":
            raise ContractError("execute result message_type must be execute_result")
        ruleset = exact_object(result["ruleset"], {"id", "version", "digest"}, "result ruleset")
        require_identifier(ruleset["id"], "ruleset.id")
        require_identifier(ruleset["version"], "ruleset.version")
        require_hex(ruleset["digest"], 64, "ruleset.digest")
        for field in (
            "content_digest",
            "initial_state_hash",
            "command_batch_hash",
            "final_state_hash",
            "outcome_hash",
            "replay_material_hash",
        ):
            require_hex(result[field], 64, field)
        for value_field, hash_field, domain in (
            ("final_state", "final_state_hash", FINAL_STATE_DOMAIN),
            ("outcome", "outcome_hash", OUTCOME_DOMAIN),
            ("replay_material", "replay_material_hash", REPLAY_DOMAIN),
        ):
            if domain_hash(domain, result[value_field]) != result[hash_field]:
                raise ContractError(f"{hash_field} does not bind {value_field}")
        return "success"
    if version == RUNTIME_ERROR_VERSION:
        error = exact_object(
            value,
            {"contract_version", "error_code", "error", "recoverable"},
            "runtime error",
        )
        require_identifier(error["error_code"], "error_code")
        if not isinstance(error["error"], str) or not (1 <= len(error["error"]) <= 4096):
            raise ContractError("error message must contain 1..=4096 bytes")
        if not isinstance(error["recoverable"], bool):
            raise ContractError("recoverable must be a boolean")
        return "error"
    raise ContractError("runtime response has an unsupported contract version")


def validate_observation(value: Any, label: str) -> dict[str, Any]:
    observation = exact_object(
        value,
        {
            "contract_version",
            "implementation_id",
            "implementation_revision",
            "request_hash",
            "response",
            "duration_micros",
            "response_bytes",
        },
        label,
    )
    if observation["contract_version"] != OBSERVATION_VERSION:
        raise ContractError(f"unsupported {label} observation contract version")
    require_identifier(observation["implementation_id"], f"{label}.implementation_id")
    require_hex(observation["implementation_revision"], 40, f"{label}.implementation_revision")
    require_hex(observation["request_hash"], 64, f"{label}.request_hash")
    kind = validate_response(observation["response"])
    for field in ("duration_micros", "response_bytes"):
        field_value = observation[field]
        if (
            not isinstance(field_value, int)
            or isinstance(field_value, bool)
            or field_value < 0
            or field_value > I64_MAX
        ):
            raise ContractError(f"{label}.{field} must be a non-negative signed integer")
    actual_bytes = len(canonical_bytes(observation["response"]))
    if observation["response_bytes"] != actual_bytes:
        raise ContractError(f"{label}.response_bytes does not bind canonical response bytes")
    return {**observation, "response_kind": kind}


def compare(input_value: Any) -> dict[str, Any]:
    shadow = exact_object(
        input_value,
        {"contract_version", "world", "candidate", "budgets"},
        "shadow input",
    )
    if shadow["contract_version"] != SHADOW_INPUT_VERSION:
        raise ContractError("unsupported World shadow input contract version")
    world = validate_observation(shadow["world"], "world")
    candidate = validate_observation(shadow["candidate"], "candidate")
    budgets = exact_object(
        shadow["budgets"],
        {"max_candidate_duration_micros", "max_candidate_response_bytes"},
        "shadow budgets",
    )
    for field in ("max_candidate_duration_micros", "max_candidate_response_bytes"):
        field_value = budgets[field]
        if (
            not isinstance(field_value, int)
            or isinstance(field_value, bool)
            or not (1 <= field_value <= I64_MAX)
        ):
            raise ContractError(f"budgets.{field} must be a positive signed integer")

    codes: list[str] = []
    if candidate["duration_micros"] > budgets["max_candidate_duration_micros"]:
        codes.append("candidate_duration_budget_exceeded")
    if candidate["response_bytes"] > budgets["max_candidate_response_bytes"]:
        codes.append("candidate_response_budget_exceeded")
    if world["request_hash"] != candidate["request_hash"]:
        codes.append("request_hash_mismatch")

    if world["response_kind"] != candidate["response_kind"]:
        codes.append("execution_kind_mismatch")
    elif world["response_kind"] == "success":
        for field, code in (
            ("ruleset", "ruleset_mismatch"),
            ("content_digest", "content_digest_mismatch"),
            ("initial_state_hash", "initial_state_hash_mismatch"),
            ("command_batch_hash", "command_batch_hash_mismatch"),
            ("final_state_hash", "final_state_hash_mismatch"),
            ("final_state", "final_state_material_mismatch"),
            ("outcome_hash", "outcome_hash_mismatch"),
            ("outcome", "outcome_material_mismatch"),
            ("replay_material_hash", "replay_material_hash_mismatch"),
            ("replay_material", "replay_material_mismatch"),
        ):
            if world["response"][field] != candidate["response"][field]:
                codes.append(code)
    else:
        if world["response"]["error_code"] != candidate["response"]["error_code"]:
            codes.append("error_code_mismatch")
        if world["response"]["recoverable"] != candidate["response"]["recoverable"]:
            codes.append("error_recoverability_mismatch")

    return {"equivalent": not codes, "divergence_codes": codes}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--vectors",
        default="contracts/world-runtime/v1/shadow-vectors.json",
    )
    args = parser.parse_args()
    path = Path(args.vectors)
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
        if payload.get("contract_version") != "trnm_world_shadow_golden_vectors_v1":
            raise ContractError("unexpected shadow vector contract version")
        vectors = payload.get("vectors")
        if not isinstance(vectors, list) or len(vectors) < 6:
            raise ContractError("shadow vector set is incomplete")
        valid = 0
        invalid = 0
        divergence_codes: set[str] = set()
        for vector in vectors:
            name = vector.get("name", "unnamed")
            if "expected_error" in vector:
                try:
                    compare(vector["input"])
                except ContractError as error:
                    if vector["expected_error"] not in str(error):
                        raise ContractError(f"{name}: wrong error: {error}") from error
                    invalid += 1
                else:
                    raise ContractError(f"{name}: invalid vector unexpectedly passed")
                continue
            result = compare(vector["input"])
            expected_codes = vector.get("expected_divergence_codes")
            if result["equivalent"] is not vector.get("expected_equivalent"):
                raise ContractError(f"{name}: equivalent mismatch")
            if result["divergence_codes"] != expected_codes:
                raise ContractError(
                    f"{name}: divergence mismatch: {result['divergence_codes']} != {expected_codes}"
                )
            valid += 1
            divergence_codes.update(result["divergence_codes"])
        report = {
            "contract_version": "trnm_world_shadow_verifier_report_v1",
            "status": "ok",
            "valid_vectors": valid,
            "invalid_vectors": invalid,
            "typed_divergence_codes": sorted(divergence_codes),
            "authority": {
                "participant_admission": False,
                "global_ordering": False,
                "canonical_roots": False,
                "completion_signing": False,
                "chain_finality": False,
                "cex_custody": False,
            },
            "limitations": [
                "This is independent source-level shadow comparator evidence only.",
                "A Nakama consumer and Integration component lock remain required.",
                "Public online and public player markets remain disabled.",
            ],
        }
        print(json.dumps(report, sort_keys=True))
        return 0
    except (OSError, json.JSONDecodeError, ContractError) as error:
        print(json.dumps({"status": "blocked", "error": str(error)}), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
