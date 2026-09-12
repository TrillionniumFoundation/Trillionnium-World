#!/usr/bin/env python3
"""Apply WORLD-P0-001 gap closure and normalize generated source checks."""

from __future__ import annotations

import pathlib
import runpy

ROOT = pathlib.Path(__file__).resolve().parents[1]
runpy.run_path(str(ROOT / "scripts/apply-world-settlement-gap-closure-v1.py"), run_name="__main__")

checker_path = ROOT / "scripts/check-trnm-settlement-runtime-status.py"
checker = checker_path.read_text(encoding="utf-8")
old = '''        operator_test,
        (
            "settlement_operator_replay_is_exact_audited_one_attempt_and_append_only",
            "remote_attempts\\\"), 15",
            "append-only receipt",
        ),
        "operator PostgreSQL test",
    )'''
new = '''        operator_test,
        (
            "settlement_operator_replay_is_exact_audited_one_attempt_and_append_only",
            'get::<i32, _>("remote_attempts"), 15',
            "truncate public.trnm_online_settlement_operator_replay_requests",
        ),
        "operator PostgreSQL test",
    )'''
if old in checker:
    checker = checker.replace(old, new, 1)
elif new not in checker:
    raise SystemExit("generated operator-test checker markers drifted")
checker_path.write_text(checker, encoding="utf-8")

print("WORLD-P0-001 mechanical gap-closure patch v2: APPLIED")
