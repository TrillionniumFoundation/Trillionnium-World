#!/usr/bin/env python3
"""Compare actual oracle execution with an independent RTS intake reference.

--reference-only NEVER proves Rust agreement. The default requires an executable
oracle built separately from the exact candidate. No runtime/release credit is
inferred. All case ordering, byte hashes and expected results are deterministic.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("rts_intake_reference", ROOT / "scripts/check-trnm-rts-intake-conformance.py")
assert SPEC and SPEC.loader
REF = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(REF)
MAX_CASES = 4096
MAX_INPUT_TOTAL = 16 * 1024 * 1024
MAX_OUTPUT = 1024 * 1024
MAX_BINARY = 64 * 1024 * 1024
ORDER_FIELDS = (
    "contract", "frame", "player_id", "subject_actor_ids", "kind", "queued",
    "target_tile", "target_actor_id", "target_rule_id", "queue_id", "formation_id",
    "source", "raw_command_label",
)


class DifferentialFailure(RuntimeError):
    pass


def encode(value) -> bytes:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), allow_nan=False).encode("utf-8")


def expected(raw: bytes) -> tuple[str, str | None]:
    result = REF.classify(raw)
    if result != "accepted":
        return result, None
    order = REF.strict_json(raw)["order"]
    # Deliberately independent from the Rust serializer: declaration order and
    # every missing optional/default field are explicit wire-contract material.
    complete = {key: order.get(key, False if key == "queued" else None) for key in ORDER_FIELDS}
    if complete["target_tile"] is not None:
        tile = complete["target_tile"]
        complete["target_tile"] = {"x": tile["x"], "y": tile["y"]}
    return result, hashlib.sha256(encode(complete)).hexdigest()


def cases(vector_path: Path) -> list[tuple[str, bytes]]:
    REF.verify_corpus(vector_path)
    frozen = REF.strict_json(vector_path.read_bytes())["cases"]
    rows: list[tuple[str, bytes]] = [("frozen/" + row["id"], row["raw"].encode()) for row in frozen]
    sample = REF.strict_json(REF.sample())
    kinds = sorted((row for row in frozen if row["id"].startswith("kind-")), key=lambda row: row["id"])
    for row in kinds:
        value = REF.strict_json(row["raw"].encode())
        value["order"]["kind"] = {value["order"]["kind"]: None}
        rows.append(("enum-map/" + row["id"], encode(value)))
    for spelling in ("local_input", "replay"):
        value = json.loads(encode(sample))
        value["order"]["source"] = {spelling: None}
        rows.append(("enum-map/source-" + spelling, encode(value)))
    mutations = [None, False, True, 0, -1, 1.0, "", [], {}, ["hold"], {"hold": None}]
    for field in ORDER_FIELDS:
        for index, mutation in enumerate(mutations):
            value = json.loads(encode(sample))
            value["order"][field] = mutation
            rows.append((f"type/{field}/{index}", encode(value)))
    for field in ("player_id", "target_actor_id", "target_rule_id", "queue_id", "formation_id"):
        for index, spelling in enumerate(("x" * 160, "x" * 161, "界" * 53, "界" * 54,
                                           "ok\x00bad", "ok\n", "ok\x7f", "a\u00a0b", "é", "e\u0301")):
            value = json.loads(encode(sample))
            value["order"][field] = spelling
            rows.append((f"identifier/{field}/{index}", encode(value)))
    for count in (0, 1, 256, 257):
        value = json.loads(encode(sample))
        value["order"]["subject_actor_ids"] = [f"unit-{i}" for i in range(count)]
        rows.append((f"subjects/{count}", encode(value)))
    for count in (0, 256, 257):
        value = json.loads(encode(sample))
        value["order"]["raw_command_label"] = "x" * count
        rows.append((f"label/{count}", encode(value)))
    for index, raw in enumerate((b"\xff", b"\xc0\xaf", b"\xef\xbb\xbf{}", b"\0", b"", b"\"scalar\"")):
        rows.append((f"raw-encoding/{index}", raw))
    raw = encode(sample)
    rows.extend([
        ("raw-limit/exact", raw + b" " * (REF.MAX_INPUT - len(raw))),
        ("raw-limit/over", raw + b" " * (REF.MAX_INPUT + 1 - len(raw))),
        ("raw/duplicate-escaped-key", raw.replace(b'"player_id":"p"', b'"player_id":"p","player_\\u0069d":"q"')),
        ("raw/surrogate", raw.replace(b'"player_id":"p"', b'"player_id":"\\ud800"')),
    ])
    # Exercise integer spelling separately from Python's normalized number model.
    for spelling in (b"-0", b"1.0", b"1e0", b"01", b"4294967295", b"4294967296", b"-1"):
        rows.append(("frame-wire/" + spelling.decode(), raw.replace(b'"frame":1', b'"frame":' + spelling)))
    validate_cases(rows)
    return rows


def validate_cases(rows: list[tuple[str, bytes]]) -> None:
    if not rows or len(rows) > MAX_CASES or len({key for key, _ in rows}) != len(rows):
        raise DifferentialFailure("case_identity_or_count_invalid")
    if any(not isinstance(key, str) or not re.fullmatch(r"[A-Za-z0-9_./+-]{1,160}", key) for key, _ in rows):
        raise DifferentialFailure("case_identity_invalid")
    if any(not isinstance(raw, bytes) or len(raw) > REF.MAX_INPUT + 1 for _, raw in rows):
        raise DifferentialFailure("case_bytes_invalid")
    if sum(len(raw) * 2 + 1 for _, raw in rows) > MAX_INPUT_TOTAL:
        raise DifferentialFailure("case_total_budget_exceeded")


def matrix_digest(rows: list[tuple[str, bytes]]) -> str:
    digest = hashlib.sha256()
    for key, raw in rows:
        for material in (key.encode("utf-8"), raw):
            digest.update(len(material).to_bytes(8, "big"))
            digest.update(material)
    return digest.hexdigest()


def parse_output(raw: bytes, rows: list[tuple[str, bytes]]) -> None:
    if len(raw) > MAX_OUTPUT or not raw or not raw.endswith(b"\n"):
        raise DifferentialFailure("oracle_output_size_or_framing_invalid")
    lines = raw.splitlines()
    if len(lines) != len(rows):
        raise DifferentialFailure("oracle_result_count_mismatch")
    for index, (line, (key, material)) in enumerate(zip(lines, rows)):
        try:
            record = REF.strict_json(line)
        except (ValueError, UnicodeError, RecursionError):
            raise DifferentialFailure("oracle_result_encoding_invalid") from None
        if not isinstance(record, dict) or set(record) != {"schema", "sequence", "result", "order_sha256"}:
            raise DifferentialFailure("oracle_result_shape_invalid")
        if record["schema"] != "trnm_rts_intake_oracle_v1" or type(record["sequence"]) is not int or record["sequence"] != index:
            raise DifferentialFailure("oracle_result_identity_mismatch")
        result, order_hash = expected(material)
        if (record["result"], record["order_sha256"]) != (result, order_hash):
            raise DifferentialFailure(f"oracle_divergence:{key}")


def binary_digest(path: Path) -> str:
    if path.is_symlink() or not path.is_file() or path.stat().st_size > MAX_BINARY:
        raise DifferentialFailure("oracle_binary_invalid")
    with path.open("rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()


def execute(oracle: Path, rows: list[tuple[str, bytes]], timeout: float = 30.0) -> dict:
    validate_cases(rows)
    if os.name != "posix":
        raise DifferentialFailure("bounded_process_group_runner_requires_posix")
    if timeout <= 0 or timeout > 120:
        raise DifferentialFailure("oracle_timeout_invalid")
    binary_hash = binary_digest(oracle)
    oracle = oracle.absolute()
    with tempfile.TemporaryDirectory(prefix="world-rts-differential-") as temporary:
        root = Path(temporary)
        with (root / "input").open("wb") as stream:
            for _, raw in rows:
                stream.write(raw.hex().encode("ascii") + b"\n")
        with (root / "input").open("rb") as stdin, (root / "stdout").open("wb") as stdout, (root / "stderr").open("wb") as stderr:
            try:
                process = subprocess.Popen([str(oracle)], stdin=stdin, stdout=stdout, stderr=stderr, start_new_session=True)
            except OSError:
                raise DifferentialFailure("oracle_start_failed") from None
            start = time.monotonic()
            try:
                while process.poll() is None:
                    if (root / "stdout").stat().st_size + (root / "stderr").stat().st_size > MAX_OUTPUT:
                        raise DifferentialFailure("oracle_output_budget_exceeded")
                    if time.monotonic() - start > timeout:
                        raise DifferentialFailure("oracle_timeout")
                    time.sleep(0.01)
                if process.returncode != 0:
                    raise DifferentialFailure("oracle_exit_nonzero")
            finally:
                # Also clean up descendants of a child that exits early.
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait(timeout=5)
        if (root / "stdout").stat().st_size + (root / "stderr").stat().st_size > MAX_OUTPUT:
            raise DifferentialFailure("oracle_output_budget_exceeded")
        output = (root / "stdout").read_bytes()
        parse_output(output, rows)
    if binary_digest(oracle) != binary_hash:
        raise DifferentialFailure("oracle_binary_changed_during_run")
    return {"oracle_executed": True, "oracle_sha256": binary_hash,
            "oracle_output_sha256": hashlib.sha256(output).hexdigest()}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vectors", type=Path, default=ROOT / "docs/protocol/vectors/trnm-rts-order-intake-v1.json")
    parser.add_argument("--oracle", type=Path)
    parser.add_argument("--reference-only", action="store_true")
    parser.add_argument("--result", type=Path)
    args = parser.parse_args()
    record = {"schema": "trnm_rts_intake_differential_v1", "status": "failed", "oracle_executed": False,
              "runtime_wiring_proven": False, "production_authorization": "not_granted"}
    try:
        if args.reference_only == (args.oracle is not None):
            raise DifferentialFailure("select_exactly_one_oracle_or_reference_only")
        rows = cases(args.vectors)
        counts: dict[str, int] = {}
        for _, raw in rows:
            result, _ = expected(raw)
            counts[result] = counts.get(result, 0) + 1
        record.update(cases=len(rows), matrix_sha256=matrix_digest(rows), outcomes=counts)
        if args.reference_only:
            record["status"] = "reference_checked_only"
        else:
            record.update(execute(args.oracle, rows))
            record["status"] = "differential_passed"
        code = 0
    except (DifferentialFailure, OSError, ValueError) as error:
        record["error"] = str(error) if isinstance(error, DifferentialFailure) else "input_or_environment_invalid"
        code = 1
    text = json.dumps(record, indent=2, sort_keys=True) + "\n"
    if args.result:
        args.result.parent.mkdir(parents=True, exist_ok=True)
        args.result.write_text(text, encoding="utf-8")
    print(text, end="")
    return code


if __name__ == "__main__":
    raise SystemExit(main())
