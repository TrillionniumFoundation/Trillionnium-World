#!/usr/bin/env python3
"""Independent strict conformance for trnm_world_transition_v1.

This checker deliberately does not import or execute the Rust reference package.
It validates the published vectors with Python's standard library plus a separate
canonical encoder and object-pair model.
"""

from __future__ import annotations

import hashlib
import json
import pathlib
import re
import sys
from dataclasses import dataclass
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
POSITIVE = ROOT / "docs/protocol/vectors/trnm-world-transition-v1.json"
NEGATIVE = ROOT / "docs/protocol/vectors/trnm-world-transition-negative-v1.json"
SCHEMA = ROOT / "docs/protocol/schemas/trnm-world-transition-v1.schema.json"
RUST = ROOT / "trillionnium/contracts/trnm-world-transition-v1/src/canonical.rs"
LIB = ROOT / "trillionnium/contracts/trnm-world-transition-v1/src/lib.rs"

MAX_DEPTH = 128
MIN_I64 = -(2**63)
MAX_I64 = 2**63 - 1
INTEGER = re.compile(r"^-?(0|[1-9][0-9]*)$")
FORBIDDEN_KEYS = {
    "nakama_session_token",
    "nakama_private_key",
    "match_authority_private_key",
    "canonical_archive_root",
    "chain_finality",
    "chain_app_hash",
    "match_completed_v1",
    "participant_admission_receipt",
    "global_event_cursor",
}


class CanonicalFailure(ValueError):
    pass


@dataclass(frozen=True)
class CanonicalObject:
    pairs: tuple[tuple[str, Any], ...]


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World transition conformance: FAIL: {message}")


def parse_integer(text: str) -> int:
    if text == "-0" or INTEGER.fullmatch(text) is None:
        raise CanonicalFailure("number is not canonical signed-i64")
    value = int(text, 10)
    if value < MIN_I64 or value > MAX_I64:
        raise CanonicalFailure("integer exceeds signed-i64")
    return value


def reject_float(text: str) -> float:
    raise CanonicalFailure(f"floating-point number is forbidden: {text}")


def reject_constant(text: str) -> None:
    raise CanonicalFailure(f"non-finite number is forbidden: {text}")


def object_pairs(pairs: list[tuple[str, Any]]) -> CanonicalObject:
    previous: str | None = None
    for key, _ in pairs:
        if previous is not None and key <= previous:
            raise CanonicalFailure(f"object key duplicated or unsorted: {key}")
        previous = key
    return CanonicalObject(tuple(pairs))


def check_value(value: Any, depth: int = 0) -> None:
    if depth > MAX_DEPTH:
        raise CanonicalFailure("nesting exceeds 128")
    if isinstance(value, CanonicalObject):
        for key, child in value.pairs:
            if key in FORBIDDEN_KEYS:
                raise CanonicalFailure(f"forbidden authority key: {key}")
            check_string(key)
            check_value(child, depth + 1)
    elif isinstance(value, list):
        for child in value:
            check_value(child, depth + 1)
    elif isinstance(value, str):
        check_string(value)
    elif value is None or isinstance(value, (bool, int)):
        return
    else:
        raise CanonicalFailure(f"unsupported value type: {type(value).__name__}")


def check_string(value: str) -> None:
    for character in value:
        codepoint = ord(character)
        if codepoint == 0x7F or (codepoint < 0x20 and character not in "\b\f\n\r\t"):
            raise CanonicalFailure("string contains forbidden control character")


def encode_string(value: str) -> str:
    # ensure_ascii=False gives UTF-8 characters directly and uses the short JSON
    # escapes for quote, backslash, and the named control characters.
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def encode(value: Any) -> str:
    if isinstance(value, CanonicalObject):
        return "{" + ",".join(
            f"{encode_string(key)}:{encode(child)}" for key, child in value.pairs
        ) + "}"
    if isinstance(value, list):
        return "[" + ",".join(encode(child) for child in value) + "]"
    if isinstance(value, str):
        return encode_string(value)
    if value is None:
        return "null"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, int):
        if value < MIN_I64 or value > MAX_I64:
            raise CanonicalFailure("integer exceeds signed-i64")
        return str(value)
    raise CanonicalFailure(f"unsupported value type: {type(value).__name__}")


def parse_canonical(raw: str) -> Any:
    if not raw:
        raise CanonicalFailure("empty canonical JSON")
    try:
        value = json.loads(
            raw,
            object_pairs_hook=object_pairs,
            parse_int=parse_integer,
            parse_float=reject_float,
            parse_constant=reject_constant,
        )
    except (json.JSONDecodeError, RecursionError, UnicodeError, CanonicalFailure) as error:
        raise CanonicalFailure(str(error)) from error
    if not isinstance(value, (CanonicalObject, list)):
        raise CanonicalFailure("root must be object or array")
    check_value(value)
    if encode(value) != raw:
        raise CanonicalFailure("input bytes are not the exact canonical encoding")
    return value


def sha256_hex(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def domain_hash(domain: str, material: str) -> str:
    return sha256_hex(f"{domain}\n{material}".encode("utf-8"))


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must be a JSON object")
    return value


def main() -> None:
    positive = load_json(POSITIVE)
    negative = load_json(NEGATIVE)
    schema = load_json(SCHEMA)

    if positive.get("vector_contract") != "trnm_world_transition_vectors_v1":
        fail("wrong positive vector contract")
    if negative.get("vector_contract") != "trnm_world_transition_negative_vectors_v1":
        fail("wrong negative vector contract")
    if schema.get("$defs", {}).get("request", {}).get("additionalProperties") is not False:
        fail("request schema must reject unknown fields")

    for vector in positive.get("sha256_core_vectors", []):
        actual = sha256_hex(vector["input_utf8"].encode("utf-8"))
        if actual != vector["expected_sha256"]:
            fail(f"SHA-256 vector {vector['name']} drifted: {actual}")

    for vector in positive.get("canonical_vectors", []):
        raw = vector["canonical_utf8"]
        parse_canonical(raw)
        actual = sha256_hex(raw.encode("utf-8"))
        if actual != vector["expected_sha256"]:
            fail(f"canonical vector {vector['name']} hash drifted: {actual}")

    request_domain = positive["hash_domains"]["request"]
    for vector in positive.get("request_vectors", []):
        raw = vector["canonical_utf8"]
        parse_canonical(raw)
        actual = domain_hash(request_domain, raw)
        if actual != vector["expected_request_hash"]:
            fail(f"request vector {vector['name']} hash drifted: {actual}")

    negative_count = 0
    for vector in negative.get("vectors", []):
        try:
            parse_canonical(vector["utf8"])
        except CanonicalFailure:
            negative_count += 1
        else:
            fail(f"negative vector unexpectedly passed: {vector['name']}")
    if negative_count < 20:
        fail("negative vector corpus is unexpectedly small")

    rust = RUST.read_text(encoding="utf-8")
    lib = LIB.read_text(encoding="utf-8")
    required_rust = (
        "DuplicateOrUnsortedKey",
        "IntegerOutOfRange",
        "NonMinimalEscape",
        "ForbiddenAuthorityKey",
        "MAX_CANONICAL_DEPTH",
        "encode_canonical(&value) != raw",
    )
    for marker in required_rust:
        if marker not in rust:
            fail(f"Rust canonical parser missing {marker}")
    for marker in (
        "parse_canonical(&self.canonical_json",
        "REQUEST_HASH_DOMAIN",
        "TRANSITION_HASH_DOMAIN",
        "OUTCOME_HASH_DOMAIN",
        "WorldRulesAdapterV1",
    ):
        if marker not in lib:
            fail(f"Rust transition API missing {marker}")
    if "validate_minified_json" in lib or "validate_minified_json" in rust:
        fail("weak bracket-only JSON validation returned")

    print(
        "TRNM World transition conformance: PASS "
        f"({len(positive.get('canonical_vectors', []))} positive canonical, "
        f"{negative_count} negative, "
        f"{len(positive.get('request_vectors', []))} request vectors)"
    )


if __name__ == "__main__":
    main()
