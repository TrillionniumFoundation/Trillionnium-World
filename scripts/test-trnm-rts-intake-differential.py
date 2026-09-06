#!/usr/bin/env python3
"""Exercise differential-runner failure handling. Fixture children are NOT Rust."""
from __future__ import annotations
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("rts_differential", Path(__file__).with_name("check-trnm-rts-intake-differential.py"))
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)


def records(rows):
    result = []
    for sequence, (_, raw) in enumerate(rows):
        code, digest = M.expected(raw)
        result.append({"schema": "trnm_rts_intake_oracle_v1", "sequence": sequence,
                       "result": code, "order_sha256": digest})
    return result


def output(value):
    return b"".join(M.encode(row) + b"\n" for row in value)


class DifferentialTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="world-oracle-fixtures-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        value = json.loads(M.REF.sample())
        value["order"]["kind"] = {"hold": None}
        self.rows = [("valid", M.REF.sample()), ("enum-alias", M.encode(value))]

    def child(self, data=None, exit_code=0, body=None):
        path = self.root / "fixture-oracle"
        if body is None:
            payload = output(records(self.rows)) if data is None else data
            body = f"import sys\nsys.stdin.buffer.read()\nsys.stdout.buffer.write({payload!r})\nsys.exit({exit_code})\n"
        path.write_text(f"#!{sys.executable}\n" + body)
        path.chmod(0o700)
        return path

    def test_actual_fixture_process_success_is_bound(self):
        path = self.child()
        report = M.execute(path, self.rows)
        self.assertTrue(report["oracle_executed"])
        self.assertEqual(report["oracle_sha256"], M.binary_digest(path))
        self.assertNotIn("production_ready", report)

    def test_enum_alias_regression_is_detected(self):
        value = records(self.rows)
        value[1]["result"] = "accepted"
        value[1]["order_sha256"] = value[0]["order_sha256"]
        with self.assertRaisesRegex(M.DifferentialFailure, "oracle_divergence:enum-alias"):
            M.execute(self.child(output(value)), self.rows)

    def test_exit_nonzero_even_with_correct_results(self):
        with self.assertRaisesRegex(M.DifferentialFailure, "oracle_exit_nonzero"):
            M.execute(self.child(exit_code=7), self.rows)

    def test_zero_records_is_not_pass(self):
        with self.assertRaises(M.DifferentialFailure):
            M.execute(self.child(b""), self.rows)

    def test_partial_records_is_not_pass(self):
        with self.assertRaisesRegex(M.DifferentialFailure, "count"):
            M.execute(self.child(output(records(self.rows)[:1])), self.rows)

    def test_extra_records_is_not_pass(self):
        value = records(self.rows)
        with self.assertRaisesRegex(M.DifferentialFailure, "count"):
            M.execute(self.child(output(value + value)), self.rows)

    def test_wrong_sequence(self):
        value = records(self.rows)
        value[0]["sequence"] = 1
        with self.assertRaisesRegex(M.DifferentialFailure, "identity"):
            M.parse_output(output(value), self.rows)

    def test_boolean_sequence(self):
        value = records(self.rows)
        value[0]["sequence"] = False
        with self.assertRaises(M.DifferentialFailure):
            M.parse_output(output(value), self.rows)

    def test_crossed_schema(self):
        value = records(self.rows)
        value[0]["schema"] = "trnm_rts_intake_oracle_v2"
        with self.assertRaises(M.DifferentialFailure):
            M.parse_output(output(value), self.rows)

    def test_unknown_field(self):
        value = records(self.rows)
        value[0]["production_authorization"] = "granted"
        with self.assertRaisesRegex(M.DifferentialFailure, "shape"):
            M.parse_output(output(value), self.rows)

    def test_wrong_normalized_hash(self):
        value = records(self.rows)
        value[0]["order_sha256"] = "0" * 64
        with self.assertRaisesRegex(M.DifferentialFailure, "divergence"):
            M.parse_output(output(value), self.rows)

    def test_rejection_cannot_carry_hash(self):
        value = records(self.rows)
        value[1]["order_sha256"] = "0" * 64
        with self.assertRaisesRegex(M.DifferentialFailure, "divergence"):
            M.parse_output(output(value), self.rows)

    def test_duplicate_json_keys(self):
        data = output(records(self.rows)).replace(b'"sequence":0', b'"sequence":0,"sequence":0', 1)
        with self.assertRaisesRegex(M.DifferentialFailure, "encoding"):
            M.parse_output(data, self.rows)

    def test_malformed_utf8(self):
        data = output(records(self.rows)).replace(b'"sequence":0', b'"sequence":"\xff"', 1)
        with self.assertRaises(M.DifferentialFailure):
            M.parse_output(data, self.rows)

    def test_missing_final_newline(self):
        with self.assertRaisesRegex(M.DifferentialFailure, "framing"):
            M.parse_output(output(records(self.rows)).rstrip(b"\n"), self.rows)

    def test_diagnostic_not_reflected(self):
        with self.assertRaises(M.DifferentialFailure) as caught:
            M.execute(self.child(b"private-reflection\n"), self.rows)
        self.assertNotIn("private-reflection", str(caught.exception))

    def test_timeout_terminates_child(self):
        with self.assertRaisesRegex(M.DifferentialFailure, "timeout"):
            M.execute(self.child(body="import time\ntime.sleep(20)\n"), self.rows, timeout=0.05)

    def test_excess_output_is_failure(self):
        path = self.child(body=f"import sys\nsys.stdout.buffer.write(b'x'*{M.MAX_OUTPUT + 1})\n")
        with self.assertRaisesRegex(M.DifferentialFailure, "budget"):
            M.execute(path, self.rows)

    def test_excess_stderr_is_failure(self):
        path = self.child(body=f"import sys\nsys.stderr.buffer.write(b'x'*{M.MAX_OUTPUT + 1})\n")
        with self.assertRaisesRegex(M.DifferentialFailure, "budget"):
            M.execute(path, self.rows)

    def test_nonexecutable_is_failure(self):
        path = self.child()
        path.chmod(0o600)
        with self.assertRaisesRegex(M.DifferentialFailure, "start"):
            M.execute(path, self.rows)

    def test_symlink_oracle_rejected(self):
        target = self.child()
        link = self.root / "link"
        link.symlink_to(target)
        with self.assertRaisesRegex(M.DifferentialFailure, "binary"):
            M.execute(link, self.rows)

    def test_empty_case_set(self):
        with self.assertRaises(M.DifferentialFailure):
            M.execute(self.child(), [])

    def test_duplicate_case_identity(self):
        with self.assertRaises(M.DifferentialFailure):
            M.validate_cases(self.rows + self.rows)

    def test_case_byte_budget(self):
        with self.assertRaises(M.DifferentialFailure):
            M.validate_cases([("huge", b"x" * (M.REF.MAX_INPUT + 2))])

    def test_empty_identifier_in_corpus_is_rejected(self):
        with self.assertRaises(M.DifferentialFailure):
            M.validate_cases([("", b"{}")])

    def test_binary_change_is_failure(self):
        path = self.child()
        original = M.binary_digest
        called = 0
        def changed(p):
            nonlocal called
            called += 1
            return original(p) if called == 1 else "0" * 64
        with patch.object(M, "binary_digest", side_effect=changed), self.assertRaisesRegex(M.DifferentialFailure, "changed"):
            M.execute(path, self.rows)

    def test_order_defaults_match_contract_shape(self):
        code, digest = M.expected(M.REF.sample())
        self.assertEqual(code, "accepted")
        value = json.loads(M.REF.sample())
        for field in M.ORDER_FIELDS:
            if field not in value["order"]:
                value["order"][field] = False if field == "queued" else None
        self.assertEqual(M.expected(M.encode(value)), (code, digest))

    def test_matrix_digest_is_order_and_byte_sensitive(self):
        self.assertNotEqual(M.matrix_digest(self.rows), M.matrix_digest(list(reversed(self.rows))))
        self.assertNotEqual(M.matrix_digest(self.rows), M.matrix_digest([("valid", self.rows[0][1] + b" ")] + self.rows[1:]))

    def test_generated_matrix_is_reproducible_and_covers_aliases(self):
        path = M.ROOT / "docs/protocol/vectors/trnm-rts-order-intake-v1.json"
        first, second = M.cases(path), M.cases(path)
        self.assertEqual(first, second)
        self.assertEqual(len(first), 361)
        tagged = [(key, raw) for key, raw in first if key.startswith("enum-map/")]
        self.assertEqual(len(tagged), 30)
        self.assertTrue(all(M.expected(raw) == ("invalid_encoding", None) for _, raw in tagged))

    def test_reference_only_never_claims_execution(self):
        result = subprocess.run([sys.executable, str(Path(M.__file__)), "--reference-only"], capture_output=True, text=True, check=False, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        record = json.loads(result.stdout)
        self.assertEqual(record["status"], "reference_checked_only")
        self.assertFalse(record["oracle_executed"])

    def test_missing_oracle_is_not_reference_fallback(self):
        result = subprocess.run([sys.executable, str(Path(M.__file__))], capture_output=True, text=True, check=False, timeout=10)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(json.loads(result.stdout)["status"], "failed")


if __name__ == "__main__":
    unittest.main()
