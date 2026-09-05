#!/usr/bin/env python3
"""Positive and hostile fixtures for the bounded settlement lock-order source guard."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("trnm_lock_order_guard", ROOT / "scripts/check-trnm-settlement-lock-order.py")
if spec is None or spec.loader is None:
    raise RuntimeError("lock-order checker is unavailable")
guard = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = guard
spec.loader.exec_module(guard)


class LockOrderTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="trnm-lock-order-fixture-")
        self.root = Path(self.temp.name)
        self.addCleanup(self.temp.cleanup)
        for name in (guard.SOURCE, guard.TEST_SOURCE, guard.CONTRACT, *guard.DOCS):
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, path)

    def replace(self, path, old, new, count=1):
        target = self.root / path
        text = target.read_text()
        self.assertIn(old, text, "fixture target drifted")
        target.write_text(text.replace(old, new, count))

    def source(self, old, new, count=1):
        self.replace(guard.SOURCE, old, new, count)

    def reject(self):
        with self.assertRaises((guard.LockOrderFailure, guard.lexer.BoundaryFailure, OSError, ValueError, TypeError, KeyError)):
            guard.validate(self.root)

    def mutate_contract(self, change):
        path = self.root / guard.CONTRACT
        data = json.loads(path.read_text())
        change(data)
        path.write_text(json.dumps(data))

    def test_current_source_passes(self):
        guard.validate(self.root)

    def test_comment_cannot_restore_campaign_order(self):
        self.source('order by campaign.campaign_id, member.player_id', 'order by member.player_id')
        path = self.root / guard.SOURCE
        path.write_text(path.read_text() + '\n// order by campaign.campaign_id, member.player_id for update of campaign\n')
        self.reject()

    def test_campaign_player_order_rejected(self):
        self.source('order by campaign.campaign_id, member.player_id', 'order by member.player_id')
        self.reject()

    def test_missing_job_tie_break_rejected(self):
        self.source('order by campaign_id, job_id\n          for update', 'order by campaign_id\n          for update')
        self.reject()

    def test_early_capture_lock_rejected(self):
        self.source("where capture_id = $1 and state = 'active'\"", "where capture_id = $1 and state = 'active' for update\"")
        self.reject()

    def test_missing_locked_match_binding_rejected(self):
        self.source("where capture_id = $1 and match_id = $2 and state = 'active'", "where capture_id = $1 and state = 'active'")
        self.reject()

    def test_missing_capture_match_parameter_rejected(self):
        self.source('.bind(capture_id)\n    .bind(match_id)\n    .fetch_optional', '.bind(capture_id)\n    .fetch_optional')
        self.reject()

    def test_inactive_capture_acceptance_rejected(self):
        self.source("where capture_id = $1 and match_id = $2 and state = 'active'", 'where capture_id = $1 and match_id = $2')
        self.reject()

    def test_match_lock_removed_rejected(self):
        self.source('where match_id = $1 for update\",\n    )\n    .bind(match_id)\n    .fetch_optional(&mut *transaction)', 'where match_id = $1\",\n    )\n    .bind(match_id)\n    .fetch_optional(&mut *transaction)')
        self.reject()

    def test_capture_ack_call_removed_rejected(self):
        self.source('lock_terminal_rows(&mut transaction, match_id).await?;', '// lock_terminal_rows(&mut transaction, match_id).await?;')
        self.reject()

    def test_apply_ack_call_removed_rejected(self):
        self.source('lock_terminal_rows(&mut transaction, match_id).await?;\n    let mut campaigns', '// lock_terminal_rows(&mut transaction, match_id).await?;\n    let mut campaigns')
        self.reject()

    def test_apply_campaigns_before_ack_rejected(self):
        self.source('lock_terminal_rows(&mut transaction, match_id).await?;\n    let mut campaigns = load_campaign_rows(&mut transaction, match_id).await?;', 'let mut campaigns = load_campaign_rows(&mut transaction, match_id).await?;\n    lock_terminal_rows(&mut transaction, match_id).await?;')
        self.reject()

    def test_member_order_removed_rejected(self):
        self.source('where match_id = $1 order by player_id for update', 'where match_id = $1 for update')
        self.reject()

    def test_ack_lock_removed_rejected(self):
        self.source('where match_id = $1 for update\",\n    )\n    .bind(match_id)\n    .fetch_optional(&mut **transaction)', 'where match_id = $1\",\n    )\n    .bind(match_id)\n    .fetch_optional(&mut **transaction)')
        self.reject()

    def test_extra_lock_query_rejected(self):
        self.source('// The initial capture read is a routing hint', 'sqlx::query::query("select job_id from public.trnm_online_settlement_jobs for update").execute(&mut *transaction).await.unwrap();\n    // The initial capture read is a routing hint')
        self.reject()

    def test_duplicate_phase_rejected(self):
        path = self.root / guard.SOURCE
        path.write_text(path.read_text() + '\nasync fn apply_capture() {}\n')
        self.reject()

    def test_remote_call_under_locks_rejected(self):
        self.source('// The initial capture read is a routing hint', 'authorize_settlement_intent().await?;\n    // The initial capture read is a routing hint')
        self.reject()

    def test_database_test_module_missing_rejected(self):
        self.source('mod lock_order_database_tests;', '// mod lock_order_database_tests;')
        self.reject()

    def test_database_test_file_missing_rejected(self):
        (self.root / guard.TEST_SOURCE).unlink()
        self.reject()

    def test_linked_source_rejected(self):
        path = self.root / guard.SOURCE
        path.unlink()
        path.symlink_to(ROOT / guard.SOURCE)
        self.reject()

    def test_empty_source_rejected(self):
        (self.root / guard.SOURCE).write_text('')
        self.reject()

    def test_oversized_source_rejected(self):
        (self.root / guard.SOURCE).write_bytes(b' ' * (guard.lexer.MAX_FILE_BYTES + 1))
        self.reject()

    def test_rank_reversal_rejected(self):
        self.mutate_contract(lambda d: d['order'][3].update(rank=8))
        self.reject()

    def test_boolean_rank_rejected(self):
        self.mutate_contract(lambda d: d['order'][0].update(rank=True))
        self.reject()

    def test_unknown_contract_field_rejected(self):
        self.mutate_contract(lambda d: d.update(verified=True))
        self.reject()

    def test_missing_evidence_boundaries_rejected(self):
        self.mutate_contract(lambda d: d.update(remaining_evidence=[]))
        self.reject()

    def test_runtime_overclaim_rejected(self):
        self.mutate_contract(lambda d: d.update(status='independently_verified'))
        self.reject()

    def test_production_overclaim_rejected(self):
        self.mutate_contract(lambda d: d.update(production_authorization='granted'))
        self.reject()

    def test_phase_trace_reversal_rejected(self):
        self.mutate_contract(lambda d: d['checked_phases'].update(apply_capture=list(reversed(guard.PHASE))))
        self.reject()

    def test_duplicate_json_key_rejected(self):
        path = self.root / guard.CONTRACT
        path.write_text('{"status":"shadowed",' + path.read_text()[1:])
        self.reject()

    def test_nonfinite_json_rejected(self):
        path = self.root / guard.CONTRACT
        path.write_text(path.read_text().replace('"rank": 1,', '"rank": NaN,', 1))
        self.reject()

    def test_doc_block_drift_rejected(self):
        self.replace(guard.DOCS[0], '8. settlement capture row', '8. match row')
        self.reject()

    def test_second_doc_block_drift_rejected(self):
        self.replace(guard.DOCS[1], '8. settlement capture row', '8. match row')
        self.reject()

    def test_old_apply_example_rejected(self):
        path = self.root / guard.DOCS[0]
        path.write_text(path.read_text() + '\ncapture -> match -> campaigns\n')
        self.reject()

    def test_actual_cli_success_and_failure(self):
        command = [sys.executable, str(ROOT / 'scripts/check-trnm-settlement-lock-order.py'), '--root', str(self.root)]
        good = subprocess.run(command, capture_output=True, text=True, timeout=10)
        self.assertEqual(good.returncode, 0, good.stderr)
        self.assertIn('no Rust/PostgreSQL/hosted credit', good.stdout)
        self.source('order by campaign.campaign_id, member.player_id', 'order by member.player_id')
        bad = subprocess.run(command, capture_output=True, text=True, timeout=10)
        self.assertNotEqual(bad.returncode, 0)
        self.assertIn('FAIL', bad.stderr)


if __name__ == '__main__':
    unittest.main(verbosity=2)
