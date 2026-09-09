#!/usr/bin/env python3
"""Test the independent checker and byte-bound Rust fixture; does not run Rust."""
from __future__ import annotations

import ast
import contextlib
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-transition-conformance.py"
spec = importlib.util.spec_from_file_location("world_transition_checker", CHECKER)
checker = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = checker
spec.loader.exec_module(checker)


def raw_object(key: str) -> str:
    return json.dumps({key: "x"}, ensure_ascii=False, separators=(",", ":"))


class TransitionConformanceTests(unittest.TestCase):
    def setUp(self):
        self.raw = checker.NEGATIVE.read_bytes()
        self.document = json.loads(self.raw)

    def test_lowercase_authority_keys_rejected(self):
        for key in checker.FORBIDDEN_KEYS:
            with self.subTest(key=key), self.assertRaises(checker.CanonicalFailure):
                checker.parse_canonical(raw_object(key))

    def test_uppercase_authority_keys_rejected(self):
        for key in checker.FORBIDDEN_KEYS:
            with self.subTest(key=key), self.assertRaises(checker.CanonicalFailure):
                checker.parse_canonical(raw_object(key.upper()))

    def test_alternating_case_authority_keys_rejected(self):
        for key in checker.FORBIDDEN_KEYS:
            mixed = "".join(c.upper() if i % 2 else c for i, c in enumerate(key))
            with self.subTest(key=key), self.assertRaises(checker.CanonicalFailure):
                checker.parse_canonical(raw_object(mixed))

    def test_nested_authority_keys_rejected(self):
        for key in checker.FORBIDDEN_KEYS:
            with self.subTest(key=key), self.assertRaises(checker.CanonicalFailure):
                checker.parse_canonical('{"a":' + raw_object(key.upper()) + '}')

    def test_array_authority_keys_rejected(self):
        for key in checker.FORBIDDEN_KEYS:
            with self.subTest(key=key), self.assertRaises(checker.CanonicalFailure):
                checker.parse_canonical('[' + raw_object(key.upper()) + ']')

    def test_authority_words_in_values_are_not_keys(self):
        for key in checker.FORBIDDEN_KEYS:
            raw = json.dumps({"a": key.upper()}, separators=(",", ":"))
            with self.subTest(key=key):
                self.assertEqual(checker.encode(checker.parse_canonical(raw)), raw)

    def test_ascii_lower_does_not_unicode_casefold(self):
        self.assertEqual(checker.ascii_lower("NAKAMA_KEY_İß"), "nakama_Key_İß")
        raw = raw_object("naKama_private_key")
        self.assertEqual(checker.encode(checker.parse_canonical(raw)), raw)

    def test_published_negative_corpus_is_rejected(self):
        for vector in self.document["vectors"]:
            with self.subTest(name=vector["name"]), self.assertRaises(checker.CanonicalFailure):
                checker.parse_canonical(vector["utf8"])

    def test_positive_canonical_corpus_is_preserved(self):
        positive = json.loads(checker.POSITIVE.read_bytes())
        for vector in positive["canonical_vectors"]:
            raw = vector["canonical_utf8"]
            with self.subTest(name=vector["name"]):
                self.assertEqual(checker.encode(checker.parse_canonical(raw)), raw)

    def test_render_is_deterministic(self):
        a = checker.render_negative_fixture(self.document, self.raw)
        self.assertEqual(a, checker.render_negative_fixture(copy.deepcopy(self.document), self.raw))

    def test_fixture_matches_checked_in_bytes(self):
        checker.validate_negative_fixture(self.document, self.raw, checker.RUST_NEGATIVE_FIXTURE)

    def test_fixture_binds_raw_json_digest(self):
        rendered = checker.render_negative_fixture(self.document, self.raw)
        self.assertIn(hashlib.sha256(self.raw).hexdigest(), rendered)
        self.assertNotEqual(rendered, checker.render_negative_fixture(self.document, self.raw + b"\n"))

    def test_fixture_contains_all_names_and_exact_bytes(self):
        rendered = checker.render_negative_fixture(self.document, self.raw)
        rows = [line.strip().rstrip(",") for line in rendered.splitlines() if line.startswith('    ("')]
        decoded = [ast.literal_eval(row) for row in rows]
        self.assertEqual(decoded, [(v["name"], v["utf8"].encode("utf-8")) for v in self.document["vectors"]])

    def test_rust_byte_encoding_preserves_controls_and_unicode(self):
        examples = [chr(n) for n in range(128)] + ['é', '世', '😀', '"\\\n\t\r', '\u2028', '"#\\u001f']
        for value in examples:
            with self.subTest(value=repr(value)):
                literal = checker.rust_bytes_literal(value)
                self.assertTrue(literal.isascii())
                self.assertEqual(ast.literal_eval(literal), value.encode("utf-8"))

    def test_render_rejects_wrong_contract(self):
        self.document["vector_contract"] = "other"
        with self.assertRaises(checker.CanonicalFailure):
            checker.render_negative_fixture(self.document, self.raw)

    def test_render_rejects_wrong_profile(self):
        self.document["canonical_profile"] = "other"
        with self.assertRaises(checker.CanonicalFailure):
            checker.render_negative_fixture(self.document, self.raw)

    def test_render_rejects_small_corpus(self):
        self.document["vectors"] = self.document["vectors"][:19]
        with self.assertRaises(checker.CanonicalFailure):
            checker.render_negative_fixture(self.document, self.raw)

    def test_render_rejects_duplicate_names(self):
        self.document["vectors"][-1]["name"] = self.document["vectors"][0]["name"]
        with self.assertRaises(checker.CanonicalFailure):
            checker.render_negative_fixture(self.document, self.raw)

    def test_render_rejects_unsafe_or_non_ascii_names(self):
        for name in ['bad"name', 'bad\nname', 'é', '', '0first']:
            self.document["vectors"][0]["name"] = name
            with self.subTest(name=name), self.assertRaises(checker.CanonicalFailure):
                checker.render_negative_fixture(self.document, self.raw)

    def test_render_rejects_missing_or_extra_fields(self):
        original = copy.deepcopy(self.document)
        for field in ("name", "utf8", "error"):
            broken = copy.deepcopy(original)
            del broken["vectors"][0][field]
            with self.subTest(field=field), self.assertRaises(checker.CanonicalFailure):
                checker.render_negative_fixture(broken, self.raw)
        original["vectors"][0]["extra"] = True
        with self.assertRaises(checker.CanonicalFailure):
            checker.render_negative_fixture(original, self.raw)

    def test_render_rejects_wrong_field_types(self):
        for field, value in [("utf8", None), ("error", False), ("error", ""), ("name", 1)]:
            document = copy.deepcopy(self.document)
            document["vectors"][0][field] = value
            with self.subTest(field=field, value=value), self.assertRaises(checker.CanonicalFailure):
                checker.render_negative_fixture(document, self.raw)

    def test_render_rejects_surrogate_payload(self):
        self.document["vectors"][0]["utf8"] = '\ud800'
        with self.assertRaises(checker.CanonicalFailure):
            checker.render_negative_fixture(self.document, self.raw)

    def test_missing_fixture_fails(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaises(SystemExit):
                checker.validate_negative_fixture(self.document, self.raw, Path(directory) / "missing.rs")

    def test_changed_fixture_fails(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "negative.rs"
            path.write_bytes(checker.RUST_NEGATIVE_FIXTURE.read_bytes() + b"// drift\n")
            with self.assertRaises(SystemExit):
                checker.validate_negative_fixture(self.document, self.raw, path)

    def test_print_mode_is_not_validation_success(self):
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            checker.main(["--print-negative-fixture"])
        self.assertEqual(output.getvalue().encode(), checker.RUST_NEGATIVE_FIXTURE.read_bytes())
        self.assertNotIn("conformance: PASS", output.getvalue())

    def test_default_cli_pass_is_read_only(self):
        watched = (checker.NEGATIVE, checker.RUST_NEGATIVE_FIXTURE, CHECKER)
        before = {p: p.read_bytes() for p in watched}
        result = subprocess.run([sys.executable, str(CHECKER)], capture_output=True, text=True, timeout=20)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("24 negative", result.stdout)
        self.assertEqual(before, {p: p.read_bytes() for p in watched})

    def test_cli_rejects_fixture_drift(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for source in (CHECKER, checker.POSITIVE, checker.NEGATIVE, checker.SCHEMA,
                           checker.RUST, checker.LIB, checker.RUST_NEGATIVE_FIXTURE):
                destination = root / source.relative_to(ROOT)
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, destination)
            fixture = root / checker.RUST_NEGATIVE_FIXTURE.relative_to(ROOT)
            fixture.write_bytes(fixture.read_bytes().replace(b'("empty", b"")', b'("empty", b"{}")'))
            result = subprocess.run([sys.executable, str(root / CHECKER.relative_to(ROOT))],
                                    capture_output=True, text=True, timeout=20)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("fixture differs", result.stderr)
            self.assertNotIn("conformance: PASS", result.stdout)

    def test_rust_harness_consumes_bound_fixture(self):
        harness = (checker.RUST_NEGATIVE_FIXTURE.parent.parent / "vectors.rs").read_text()
        self.assertIn('mod negative_vectors;', harness)
        self.assertIn('negative_vectors::CASES', harness)
        self.assertIn('negative_vectors::SOURCE_SHA256', harness)
        self.assertIn('parse_canonical_bytes(raw, 4096)', harness)
        self.assertIn('trnm-world-transition-negative-v1.json', harness)
        self.assertNotIn('let negatives = [', harness)


if __name__ == "__main__":
    unittest.main(verbosity=2)
