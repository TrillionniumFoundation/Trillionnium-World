#!/usr/bin/env python3
"""Reference verifier for the unsigned TRNM World runtime v1 contract.

The implementation intentionally uses only the Python standard library so
Nakama/Integration can run the same vectors without inheriting the World Rust
workspace.  It is a conformance oracle, not an online authority or signer.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import unicodedata
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

CONTRACT_VERSION = "trnm_world_runtime_v1"
VECTOR_VERSION = "trnm_world_runtime_golden_vectors_v1"
MAX_DEPTH = 64
MAX_NODES = 100_000
MAX_BYTES = 16 * 1024 * 1024
MIN_I64 = -(2**63)
MAX_I64 = 2**63 - 1
HEX64 = set("0123456789abcdef")
ALLOWED_TOP_LEVEL = {
    "contract_version",
    "message_type",
    "ruleset",
    "content_digest",
    "initial_state",
    "commands",
}
ALLOWED_RULESET = {"id", "version", "digest"}
ALLOWED_COMMAND = {"batch_ordinal", "kind", "payload"}
FORBIDDEN_AUTHORITY_FIELDS = {
    "participant_roster",
    "global_sequence",
    "event_root",
    "roster_root",
    "archive_root",
    "completion_signature",
    "authority_key_id",
    "chain_finality",
    "inclusion_proof",
    "wallet_balance",
}


class ContractError(ValueError):
    pass


@dataclass
class Budget:
    nodes: int = 0

    def visit(self, depth: int) -> None:
        if depth > MAX_DEPTH:
            raise ContractError(f"canonical value exceeds maximum depth {MAX_DEPTH}")
        self.nodes += 1
        if self.nodes > MAX_NODES:
            raise ContractError(f"canonical value exceeds maximum node count {MAX_NODES}")


def parse_i64(raw: str) -> int:
    value = int(raw, 10)
    if not MIN_I64 <= value <= MAX_I64:
        raise ContractError("integer is outside signed 64-bit range")
    return value


def reject_float(raw: str) -> None:
    raise ContractError(f"floating-point numbers are forbidden: {raw}")


def strict_object(pairs: Iterable[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    raw_seen: set[str] = set()
    for raw_key, value in pairs:
        if raw_key in raw_seen:
            raise ContractError(f"duplicate object key: {raw_key!r}")
        raw_seen.add(raw_key)
        key = unicodedata.normalize("NFC", raw_key)
        if key in result:
            raise ContractError(f"normalized object key collision: {key!r}")
        result[key] = value
    return result


def loads_strict(text: str) -> Any:
    try:
        return json.loads(
            text,
            parse_int=parse_i64,
            parse_float=reject_float,
            parse_constant=lambda value: (_ for _ in ()).throw(
                ContractError(f"non-finite number is forbidden: {value}")
            ),
            object_pairs_hook=strict_object,
        )
    except ContractError:
        raise
    except json.JSONDecodeError as exc:
        raise ContractError(f"invalid JSON: {exc}") from exc


def normalize_value(value: Any, budget: Budget, depth: int = 0) -> Any:
    budget.visit(depth)
    if value is None or isinstance(value, bool):
        return value
    if isinstance(value, int) and not isinstance(value, bool):
        if not MIN_I64 <= value <= MAX_I64:
            raise ContractError("integer is outside signed 64-bit range")
        return value
    if isinstance(value, float):
        raise ContractError("floating-point numbers are forbidden")
    if isinstance(value, str):
        return unicodedata.normalize("NFC", value)
    if isinstance(value, list):
        return [normalize_value(item, budget, depth + 1) for item in value]
    if isinstance(value, dict):
        normalized: dict[str, Any] = {}
        for raw_key, item in value.items():
            if not isinstance(raw_key, str):
                raise ContractError("object key is not a string")
            key = unicodedata.normalize("NFC", raw_key)
            if key in normalized:
                raise ContractError(f"normalized object key collision: {key!r}")
            normalized[key] = normalize_value(item, budget, depth + 1)
        return {
            key: normalized[key]
            for key in sorted(normalized, key=lambda item: item.encode("utf-8"))
        }
    raise ContractError(f"unsupported canonical value type: {type(value).__name__}")


def canonical_bytes(value: Any) -> bytes:
    normalized = normalize_value(value, Budget())
    encoded = json.dumps(
        normalized,
        ensure_ascii=False,
        allow_nan=False,
        separators=(",", ":"),
    ).encode("utf-8")
    if len(encoded) > MAX_BYTES:
        raise ContractError(f"canonical value exceeds maximum byte size {MAX_BYTES}")
    return encoded


def sha256_hex(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def domain_hash(domain: str, value: Any) -> str:
    if not domain.isascii() or not domain or "\n" in domain:
        raise ContractError("hash domain must be non-empty single-line ASCII")
    return sha256_hex(domain.encode("ascii") + b"\n" + canonical_bytes(value))


def require_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise ContractError(f"{label} must be an object")
    return value


def require_exact_fields(value: dict[str, Any], allowed: set[str], label: str) -> None:
    unknown = sorted(set(value) - allowed)
    missing = sorted(allowed - set(value))
    if unknown:
        if any(field in FORBIDDEN_AUTHORITY_FIELDS for field in unknown):
            raise ContractError(f"additional authority field is forbidden in {label}: {unknown}")
        raise ContractError(f"unknown field in {label}: {unknown}")
    if missing:
        raise ContractError(f"missing field in {label}: {missing}")


def require_identifier(value: Any, label: str) -> str:
    if not isinstance(value, str) or not 1 <= len(value) <= 128:
        raise ContractError(f"{label} must be a 1..128 character identifier")
    if not value[0].isalnum() or value[0] != value[0].lower():
        raise ContractError(f"{label} has an invalid first character")
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789._:-")
    if any(character not in allowed for character in value):
        raise ContractError(f"{label} contains a non-portable character")
    return value


def require_hex64(value: Any, label: str) -> str:
    if not isinstance(value, str) or len(value) != 64 or any(ch not in HEX64 for ch in value):
        raise ContractError(f"{label} must be lowercase 64-hex")
    return value


def validate_request(value: Any) -> dict[str, Any]:
    request = require_object(value, "execute request")
    require_exact_fields(request, ALLOWED_TOP_LEVEL, "execute request")
    if request["contract_version"] != CONTRACT_VERSION:
        raise ContractError("unsupported World runtime contract version")
    if request["message_type"] != "execute_request":
        raise ContractError("message_type must be execute_request")

    ruleset = require_object(request["ruleset"], "ruleset")
    require_exact_fields(ruleset, ALLOWED_RULESET, "ruleset")
    require_identifier(ruleset["id"], "ruleset.id")
    require_identifier(ruleset["version"], "ruleset.version")
    require_hex64(ruleset["digest"], "ruleset.digest")
    require_hex64(request["content_digest"], "content_digest")
    canonical_bytes(request["initial_state"])

    commands = request["commands"]
    if not isinstance(commands, list) or len(commands) > MAX_NODES:
        raise ContractError("commands must be a bounded array")
    for expected_ordinal, raw_command in enumerate(commands):
        command = require_object(raw_command, f"commands[{expected_ordinal}]")
        require_exact_fields(command, ALLOWED_COMMAND, f"commands[{expected_ordinal}]")
        if command["batch_ordinal"] != expected_ordinal:
            raise ContractError("command ordinals must be contiguous from zero")
        require_identifier(command["kind"], f"commands[{expected_ordinal}].kind")
        canonical_bytes(command["payload"])
    canonical_bytes(request)
    return request


def verify_schema_shape(schema: Any) -> None:
    root = require_object(schema, "schema")
    if root.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise ContractError("runtime schema must use JSON Schema 2020-12")
    definitions = require_object(root.get("$defs"), "schema.$defs")
    request = require_object(definitions.get("executeRequest"), "executeRequest schema")
    result = require_object(definitions.get("executeResult"), "executeResult schema")
    for label, definition in (("request", request), ("result", result)):
        if definition.get("additionalProperties") is not False:
            raise ContractError(f"{label} envelope must fail closed on additional properties")
    request_properties = require_object(request.get("properties"), "request properties")
    for forbidden in FORBIDDEN_AUTHORITY_FIELDS:
        if forbidden in request_properties:
            raise ContractError(f"schema assigns forbidden authority field to World: {forbidden}")


def expect_error(label: str, expected_fragment: str, operation: Any) -> None:
    try:
        operation()
    except ContractError as exc:
        if expected_fragment not in str(exc):
            raise ContractError(
                f"negative vector {label!r} returned {exc!r}, expected {expected_fragment!r}"
            ) from exc
        return
    raise ContractError(f"negative vector {label!r} unexpectedly succeeded")


def verify(contract_root: Path) -> dict[str, Any]:
    vectors_path = contract_root / "golden-vectors.json"
    schema_path = contract_root / "trnm-world-runtime-v1.schema.json"
    vectors = loads_strict(vectors_path.read_text(encoding="utf-8"))
    schema = loads_strict(schema_path.read_text(encoding="utf-8"))
    verify_schema_shape(schema)

    if vectors.get("contract_version") != VECTOR_VERSION:
        raise ContractError("unsupported golden-vector version")

    for vector in vectors.get("sha256_known_vectors", []):
        actual = sha256_hex(vector["bytes_utf8"].encode("utf-8"))
        if actual != vector["sha256"]:
            raise ContractError(f"SHA-256 known vector failed: {vector['name']}")

    for vector in vectors.get("canonicalization_vectors", []):
        actual = canonical_bytes(vector["value"]).decode("utf-8")
        if actual != vector["expected_canonical"]:
            raise ContractError(
                f"canonical vector {vector['name']!r} differs: {actual!r}"
            )
        expected_hash = vector.get("expected_sha256")
        if expected_hash is not None and sha256_hex(actual.encode("utf-8")) != expected_hash:
            raise ContractError(f"canonical SHA-256 vector failed: {vector['name']}")
        reparsed = loads_strict(actual)
        if canonical_bytes(reparsed).decode("utf-8") != actual:
            raise ContractError(f"canonical vector is not idempotent: {vector['name']}")

    runtime = require_object(vectors.get("runtime_request_vector"), "runtime vector")
    request = validate_request(runtime["value"])
    canonical = canonical_bytes(request).decode("utf-8")
    if canonical != runtime["expected_canonical"]:
        raise ContractError("runtime request canonical bytes differ from the golden vector")
    request_hash = domain_hash(runtime["hash_domain"], request)

    reordered = loads_strict(runtime["expected_canonical"])
    if domain_hash(runtime["hash_domain"], reordered) != request_hash:
        raise ContractError("object-key reordering changed the domain hash")
    tampered = loads_strict(runtime["expected_canonical"])
    tampered["commands"][0]["kind"] = "move"
    if domain_hash(runtime["hash_domain"], tampered) == request_hash:
        raise ContractError("command tampering did not change the domain hash")

    negatives = {item["name"]: item for item in vectors.get("negative_vectors", [])}
    expect_error(
        "float-forbidden",
        negatives["float-forbidden"]["expected_error"],
        lambda: loads_strict(negatives["float-forbidden"]["json"]),
    )
    expect_error(
        "duplicate-key-forbidden",
        negatives["duplicate-key-forbidden"]["expected_error"],
        lambda: loads_strict(negatives["duplicate-key-forbidden"]["json"]),
    )
    expect_error(
        "nfc-key-collision-forbidden",
        negatives["nfc-key-collision-forbidden"]["expected_error"],
        lambda: loads_strict(negatives["nfc-key-collision-forbidden"]["json"]),
    )

    ordinal_request = loads_strict(runtime["expected_canonical"])
    ordinal_request["commands"] = negatives["ordinal-gap-forbidden"]["request_commands"]
    expect_error(
        "ordinal-gap-forbidden",
        negatives["ordinal-gap-forbidden"]["expected_error"],
        lambda: validate_request(ordinal_request),
    )

    authority_request = loads_strict(runtime["expected_canonical"])
    authority_request.update(negatives["authority-field-forbidden"]["request_extension"])
    expect_error(
        "authority-field-forbidden",
        negatives["authority-field-forbidden"]["expected_error"],
        lambda: validate_request(authority_request),
    )

    return {
        "status": "ok",
        "contract_version": CONTRACT_VERSION,
        "vector_version": VECTOR_VERSION,
        "canonicalization_vectors": len(vectors.get("canonicalization_vectors", [])),
        "negative_vectors": len(vectors.get("negative_vectors", [])),
        "runtime_request_domain_hash": request_hash,
        "authority_signing": False,
        "global_ordering": False,
        "chain_finality": False,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--contract-root",
        type=Path,
        default=Path(__file__).resolve().parents[1] / "contracts/world-runtime/v1",
    )
    args = parser.parse_args()
    try:
        report = verify(args.contract_root.resolve())
    except (ContractError, OSError, KeyError, TypeError) as exc:
        print(json.dumps({"status": "blocked", "error": str(exc)}, ensure_ascii=False), file=sys.stderr)
        return 1
    print(json.dumps(report, ensure_ascii=False, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
