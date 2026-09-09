#!/usr/bin/env python3
"""Independent raw-JSON reference for the opt-in RTS intake v1 profile.

This validates frozen vectors only. It does not execute Rust or authorize an
online endpoint, player, simulation transition, or release.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys
import unittest

CONTRACT = "trnm_rts_order_intake_v1"
ORDER_CONTRACT = "trnm_rts_order_protocol_v1"
MAX_INPUT = 128 * 1024
MAX_ID = 160
MAX_SUBJECTS = 256
MAX_LABEL = 256
FIELDS = {
    "contract", "frame", "player_id", "subject_actor_ids", "kind", "queued",
    "target_tile", "target_actor_id", "target_rule_id", "queue_id", "formation_id",
    "source", "raw_command_label",
}
REQUIRED = {"contract", "frame", "player_id", "subject_actor_ids", "kind", "source"}
KINDS = {
    "move", "attack_move", "harvest", "build", "capture", "extract", "attack",
    "focus_fire", "ability", "repair", "recon", "train", "research", "upgrade",
    "assign_group", "append_group", "remove_group", "recall_group", "cancel_queued_order",
    "cancel_job", "pause_job", "resume_job", "promote_job", "set_rally", "patrol",
    "stop", "set_stance", "hold",
}


class Rejected(ValueError):
    pass


def reject_noninteger(value: str):
    raise Rejected("noninteger or nonfinite JSON number")


def integer(value: str) -> int:
    if value == "-0":
        raise Rejected("negative zero")
    return int(value)


def unique_object(pairs: list[tuple[str, object]]) -> dict:
    result = {}
    for key, value in pairs:
        if key in result:
            raise Rejected("duplicate JSON member")
        result[key] = value
    return result


def strict_json(raw: bytes):
    value = json.loads(raw.decode("utf-8"), object_pairs_hook=unique_object,
                       parse_int=integer, parse_float=reject_noninteger,
                       parse_constant=reject_noninteger)
    def unicode_valid(item):
        if isinstance(item, str):
            item.encode("utf-8", errors="strict")
        elif isinstance(item, dict):
            for key, child in item.items():
                unicode_valid(key)
                unicode_valid(child)
        elif isinstance(item, list):
            for child in item:
                unicode_valid(child)
    unicode_valid(value)
    return value


def object_fields(value, allowed: set[str], required: set[str]) -> bool:
    return isinstance(value, dict) and set(value) <= allowed and required <= set(value)


def wire_valid(value) -> bool:
    if not object_fields(value, {"intake_contract", "order"}, {"intake_contract", "order"}):
        return False
    if not isinstance(value["intake_contract"], str):
        return False
    order = value["order"]
    if not object_fields(order, FIELDS, REQUIRED):
        return False
    for name in ("contract", "player_id", "kind", "source"):
        if not isinstance(order[name], str):
            return False
    if type(order["frame"]) is not int or not 0 <= order["frame"] <= 2**32 - 1:
        return False
    if order["kind"] not in KINDS or order["source"] not in {"local_input", "replay"}:
        return False
    if not isinstance(order["subject_actor_ids"], list) or not all(isinstance(v, str) for v in order["subject_actor_ids"]):
        return False
    if "queued" in order and type(order["queued"]) is not bool:
        return False
    for name in ("target_actor_id", "target_rule_id", "queue_id", "formation_id", "raw_command_label"):
        if order.get(name) is not None and not isinstance(order[name], str):
            return False
    tile = order.get("target_tile")
    if tile is not None:
        if not object_fields(tile, {"x", "y"}, {"x", "y"}):
            return False
        if any(type(tile[c]) is not int or not -(2**31) <= tile[c] <= 2**31 - 1 for c in ("x", "y")):
            return False
    return True


def identifier(value: str) -> str | None:
    if len(value.encode("utf-8")) > MAX_ID:
        return "resource_budget_exceeded"
    if not value or any(ord(c) <= 0x20 or ord(c) == 0x7f for c in value):
        return "invalid_identifier"
    return None


def classify(raw: bytes) -> str:
    if len(raw) > MAX_INPUT:
        return "resource_budget_exceeded"
    try:
        value = strict_json(raw)
        if not wire_valid(value):
            return "invalid_encoding"
    except (ValueError, UnicodeError, RecursionError, OverflowError):
        return "invalid_encoding"
    if value["intake_contract"] != CONTRACT:
        return "unsupported_intake_contract"
    order = value["order"]
    if order["contract"] != ORDER_CONTRACT:
        return "unsupported_order_contract"
    subjects = order["subject_actor_ids"]
    if len(subjects) > MAX_SUBJECTS:
        return "resource_budget_exceeded"
    error = identifier(order["player_id"])
    if error:
        return error
    seen = set()
    for subject in subjects:
        error = identifier(subject)
        if error:
            return error
        if subject in seen:
            return "duplicate_subject"
        seen.add(subject)
    for name in ("target_actor_id", "target_rule_id", "queue_id", "formation_id"):
        if order.get(name) is not None:
            error = identifier(order[name])
            if error:
                return error
    label = order.get("raw_command_label")
    if label is not None and len(label.encode("utf-8")) > MAX_LABEL:
        return "resource_budget_exceeded"
    if not subjects or (order.get("queued", False) and not order.get("queue_id")):
        return "invalid_shape"
    kind = order["kind"]
    tile, actor, rule, queue = (order.get(f) is not None for f in ("target_tile", "target_actor_id", "target_rule_id", "queue_id"))
    valid = True
    if kind in {"move", "attack_move", "patrol", "recon"}:
        valid = tile
    elif kind in {"attack", "focus_fire", "harvest", "capture", "extract", "repair"}:
        valid = actor or tile
    elif kind == "build":
        valid = rule and tile
    elif kind in {"ability", "assign_group", "append_group", "remove_group", "recall_group"}:
        valid = rule
    elif kind in {"train", "research", "upgrade"}:
        valid = rule and queue
    elif kind in {"cancel_queued_order", "cancel_job", "pause_job", "resume_job", "promote_job"}:
        valid = queue
    elif kind == "set_rally":
        valid = queue and tile
    elif kind == "set_stance":
        valid = order.get("target_rule_id") in {"hold_fire", "guard", "aggressive"}
    return "accepted" if valid else "invalid_shape"


def verify_corpus(path: Path) -> dict:
    raw = path.read_bytes()
    if len(raw) > 2 * 1024 * 1024:
        raise Rejected("vector corpus is oversized")
    corpus = strict_json(raw)
    if not object_fields(corpus, {"schema", "cases"}, {"schema", "cases"}) or corpus["schema"] != "trnm_rts_order_intake_vectors_v1":
        raise Rejected("invalid vector corpus schema")
    cases = corpus["cases"]
    if not isinstance(cases, list) or len(cases) != 114:
        raise Rejected("incomplete or oversized vector corpus")
    seen, codes = set(), {}
    for case in cases:
        if not object_fields(case, {"id", "raw", "expected"}, {"id", "raw", "expected"}) or not all(isinstance(v, str) for v in case.values()):
            raise Rejected("invalid vector record")
        if case["id"] in seen:
            raise Rejected("duplicate vector identity")
        seen.add(case["id"])
        actual = classify(case["raw"].encode("utf-8"))
        if actual != case["expected"]:
            raise Rejected(f"vector {case['id']}: expected {case['expected']}, observed {actual}")
        codes[actual] = codes.get(actual, 0) + 1
    if not {"kind-" + kind for kind in KINDS} <= seen:
        raise Rejected("missing command-kind coverage")
    if set(codes) != {"accepted", "resource_budget_exceeded", "invalid_encoding", "unsupported_intake_contract",
                      "unsupported_order_contract", "invalid_identifier", "duplicate_subject", "invalid_shape"}:
        raise Rejected("missing stable-code family coverage")
    return {"schema": "trnm_rts_intake_reference_result_v1", "status": "passed",
            "cases": len(cases), "outcomes": codes, "rust_executed": False,
            "runtime_wiring_proven": False, "production_authorization": "not_granted"}


def sample() -> bytes:
    return json.dumps({"intake_contract": CONTRACT, "order": {"contract": ORDER_CONTRACT,
        "frame": 1, "player_id": "p", "subject_actor_ids": ["u"], "kind": "hold", "source": "local_input"}}).encode()


class ReferenceTests(unittest.TestCase):
    def test_input_byte_budget(self):
        raw = sample()
        self.assertEqual(classify(raw + b" " * (MAX_INPUT - len(raw))), "accepted")
        self.assertEqual(classify(raw + b" " * (MAX_INPUT + 1 - len(raw))), "resource_budget_exceeded")

    def test_unencoded_invalid_utf8(self):
        self.assertEqual(classify(b"\xff"), "invalid_encoding")

    def test_utf8_id_boundary(self):
        for count, expected in [(53, "accepted"), (54, "resource_budget_exceeded")]:
            value = json.loads(sample())
            value["order"]["player_id"] = "界" * count
            self.assertEqual(classify(json.dumps(value, ensure_ascii=False).encode()), expected)

    def test_subject_boundary(self):
        for count, expected in [(256, "accepted"), (257, "resource_budget_exceeded")]:
            value = json.loads(sample())
            value["order"]["subject_actor_ids"] = [f"u{i}" for i in range(count)]
            self.assertEqual(classify(json.dumps(value).encode()), expected)

    def test_bad_expected_outcome_rejected(self):
        import tempfile
        value = {"schema": "trnm_rts_order_intake_vectors_v1", "cases": [
            {"id": str(i), "raw": sample().decode(), "expected": "invalid_shape"} for i in range(114)]}
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "vectors.json"
            path.write_text(json.dumps(value))
            with self.assertRaises(Rejected):
                verify_corpus(path)

    def test_duplicate_vector_ids_rejected(self):
        import tempfile
        value = {"schema": "trnm_rts_order_intake_vectors_v1", "cases": [
            {"id": "same", "raw": sample().decode(), "expected": "accepted"}] * 114}
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "vectors.json"
            path.write_text(json.dumps(value))
            with self.assertRaises(Rejected):
                verify_corpus(path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vectors", type=Path, default=Path(__file__).resolve().parents[1] / "docs/protocol/vectors/trnm-rts-order-intake-v1.json")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        result = unittest.TextTestRunner().run(unittest.defaultTestLoader.loadTestsFromTestCase(ReferenceTests))
        if not result.wasSuccessful():
            raise SystemExit(1)
    try:
        print(json.dumps(verify_corpus(args.vectors), sort_keys=True))
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"RTS intake reference failed: {error}", file=sys.stderr)
        raise SystemExit(1)
