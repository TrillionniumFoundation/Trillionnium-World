#!/usr/bin/env python3
"""Offline module-resolution regressions. No Rust, database or deployment credit."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name('check-trnm-settlement-transaction-boundary.py')
spec = importlib.util.spec_from_file_location('settlement_module_boundary', SCRIPT)
assert spec and spec.loader
checker = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = checker
spec.loader.exec_module(checker)
GOOD = '''pub(crate) async fn capture_match(pool: &Pool) {
    let tx = pool.begin().await.unwrap(); tx.commit().await.unwrap();
}
pub(super) async fn process_claimed_job(cex: &Cex, intent: &Intent) {
    let a = cex.authorize_settlement_intent(intent).await.unwrap();
    cex.submit_authorized_settlement_intent(&a.intent).await.unwrap();
}
pub async fn apply_capture(pool: &Pool) {
    let tx = pool.begin().await.unwrap(); tx.commit().await.unwrap();
}
'''


class ModuleResolution(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix='world-module-boundary-')
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.write('src/lib.rs', 'mod phases;')
        self.write('src/phases.rs', GOOD)

    def write(self, name, text):
        p = self.root / name
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding='utf-8')

    def scan(self, entry='src/lib.rs'):
        reader = checker.SourceBundle(self.root)
        result = reader.bundle(entry)
        checker.scan_phases(result)
        return reader, result

    def reject(self, pattern=None, entry='src/lib.rs'):
        with self.assertRaises(checker.BoundaryFailure) as raised:
            self.scan(entry)
        if pattern:
            self.assertIn(pattern, str(raised.exception))

    def test_plain_file_module(self):
        reader, _ = self.scan()
        self.assertEqual(reader.seen, {'src/lib.rs', 'src/phases.rs'})

    def test_mod_rs_layout(self):
        (self.root / 'src/phases.rs').unlink()
        self.write('src/phases/mod.rs', GOOD)
        self.scan()

    def test_restricted_visibility_and_raw_identifier(self):
        for visibility in ('pub', 'pub(crate)', 'pub(super)', 'pub(in crate::inner)'):
            with self.subTest(visibility=visibility):
                self.write('src/lib.rs', f'{visibility} mod r#phases;')
                self.scan()

    def test_explicit_path_and_inner_parts(self):
        self.write('src/lib.rs', '#[path = "parts/owner/mod.rs"] mod owner;')
        self.write('src/parts/owner/mod.rs', 'pub(crate) mod part_01;')
        self.write('src/parts/owner/part_01.rs', GOOD)
        reader, _ = self.scan()
        self.assertEqual(len(reader.seen), 3)

    def test_non_mod_rs_default_child_directory(self):
        self.write('src/phases.rs', 'mod child;')
        self.write('src/phases/child.rs', GOOD)
        self.scan()

    def test_non_mod_rs_path_is_relative_to_source_directory(self):
        self.write('src/phases.rs', '#[path = "other/body.rs"] mod child;')
        self.write('src/other/body.rs', GOOD)
        self.scan()

    def test_inline_module_directory(self):
        self.write('src/lib.rs', 'mod owner { mod child; }')
        self.write('src/owner/child.rs', GOOD)
        self.scan()

    def test_inline_module_path_override(self):
        self.write('src/lib.rs', '#[path = "selected"] mod owner { #[path = "body.rs"] mod child; }')
        self.write('src/selected/body.rs', GOOD)
        self.scan()

    def test_non_mod_rs_inline_path(self):
        self.write('src/phases.rs', 'mod inline { #[path="body.rs"] mod child; }')
        self.write('src/phases/inline/body.rs', GOOD)
        self.scan()

    def test_path_literal_after_unrelated_attributes(self):
        self.write('src/lib.rs', '#![allow(dead_code)]\n#[allow(unused)]\n#[path="phases.rs"]\npub(crate) mod body;')
        self.scan()

    def test_include_compatibility(self):
        for text in ('include!("phases.rs");', 'include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/phases.rs"));'):
            with self.subTest(text=text):
                self.write('src/lib.rs', text)
                self.scan()

    def test_module_with_legacy_include_fragment(self):
        self.write('src/phases.rs', 'include!("body.rs");')
        self.write('src/body.rs', GOOD)
        self.scan()

    def test_module_looking_comments_literals_and_macros(self):
        self.write('src/lib.rs', '''// mod absent;
const S: &str = r##"#[path="missing.rs"] mod fake;"##;
macro_rules! fake { () => { mod not_a_module; } }
ignore!{ mod also_not_a_module; }
mod phases;''')
        self.scan()

    def test_macro_cannot_supply_ordinary_phase(self):
        self.write('src/phases.rs', 'fake!{' + GOOD + '}')
        self.reject('expected one ordinary phase implementation')

    def test_missing_module_rejected(self):
        (self.root / 'src/phases.rs').unlink()
        self.reject('missing or ambiguous module')

    def test_ambiguous_module_rejected(self):
        self.write('src/phases/mod.rs', GOOD)
        self.reject('missing or ambiguous module')

    def test_path_escape_and_absolute_rejected(self):
        for path in ('../escape.rs', '/tmp/escape.rs', '.git/config', 'x//y.rs', 'C:/file.rs'):
            with self.subTest(path=path):
                self.write('src/lib.rs', f'#[path="{path}"] mod phases;')
                self.reject('unsafe source path')

    def test_duplicate_and_computed_path_rejected(self):
        for attrs in ('#[path="phases.rs"] #[path="phases.rs"]', '#[path=concat!("phases", ".rs")]'):
            with self.subTest(attrs=attrs):
                self.write('src/lib.rs', attrs + ' mod phases;')
                self.reject('module path')

    def test_conditional_production_path_rejected(self):
        for attrs in ('#[cfg(any())]', '#[cfg(unix)]', '#[cfg_attr(unix, path="phases.rs")]'):
            with self.subTest(attrs=attrs):
                self.write('src/lib.rs', attrs + ' mod phases;')
                self.reject('conditional')

    def test_test_only_module_cannot_supply_phase(self):
        for source in ('#[cfg(test)] mod phases;', '#[cfg(test)] mod tests {' + GOOD + '}'):
            with self.subTest(source=source[:40]):
                self.write('src/lib.rs', source)
                self.reject('expected one ordinary phase implementation')

    def test_test_helper_duplicates_do_not_supply_production_implementations(self):
        self.write('src/lib.rs', 'mod phases; #[cfg(test)] mod tests {' + GOOD + '}')
        self.scan()

    def test_block_local_module_is_not_silently_ignored(self):
        self.write('src/lib.rs', 'fn helper() { mod hidden; } mod phases;')
        self.reject('block-local module')

    def test_linked_module_and_parent_rejected(self):
        (self.root / 'src/phases.rs').unlink()
        for target in ('body.rs', 'missing.rs'):
            self.write('src/body.rs', GOOD)
            p = self.root / 'src/phases.rs'
            p.symlink_to(target)
            self.reject('linked')
            p.unlink()
        self.write('src/actual/body.rs', GOOD)
        (self.root / 'src/alias').symlink_to('actual', target_is_directory=True)
        self.write('src/lib.rs', '#[path="alias/body.rs"] mod phases;')
        self.reject('linked')

    def test_duplicate_and_cyclic_module_rejected(self):
        for source in ('#[path="phases.rs"] mod a; #[path="phases.rs"] mod b;', '#[path="lib.rs"] mod cyclic;'):
            self.write('src/lib.rs', source)
            self.reject('duplicate or cyclic')

    def test_nested_adapter_name_is_not_excluded(self):
        self.write('src/lib.rs', 'mod owner { mod cex; }')
        self.write('src/owner/cex.rs', GOOD)
        reader, _ = self.scan()
        self.assertIn('src/owner/cex.rs', reader.seen)

    def test_historical_root_adapter_exclusion_is_exact(self):
        self.write('src/lib.rs', 'mod cex; mod phases;')
        self.write('src/cex.rs', 'const SHARED: &str = "separate adapter";')
        reader, _ = self.scan()
        self.assertNotIn('src/cex.rs', reader.seen)
        self.write('src/lib.rs', '#[path="other.rs"] mod cex; mod phases;')
        self.write('src/other.rs', 'fn bad() { cex.authorize_settlement_intent(intent); }')
        reader, stream = self.scan()
        self.assertIn('src/other.rs', reader.seen)
        self.assertTrue(checker.remote_calls(stream))

    def test_module_resource_budgets_preserved(self):
        for name, value in (('MAX_FILES', 1), ('MAX_FILE_BYTES', 20), ('MAX_BUNDLE_BYTES', 50), ('MAX_TOKENS', 20)):
            with self.subTest(name=name), patch.object(checker, name, value):
                self.reject()
        self.write('src/lib.rs', 'mod a { mod b { mod c { } } } mod phases;')
        with patch.object(checker, 'MAX_NESTING', 2):
            self.reject()

    def test_remote_calls_under_capture_and_apply_still_rejected(self):
        for method in ('authorize_settlement_intent', 'submit_authorized_settlement_intent'):
            for phase in ('capture_match', 'apply_capture'):
                with self.subTest(method=method, phase=phase):
                    source = GOOD.replace(f'fn {phase}(pool: &Pool) {{', f'fn {phase}(pool: &Pool) {{ cex.{method}(intent);')
                    self.write('src/phases.rs', source)
                    self.reject('contains direct signer/CEX execution')

    def test_remote_phase_transaction_and_lock_still_rejected(self):
        for statement in ('pool.begin();', 'let q = "SELECT 1 FOR UPDATE";', 'let tx: Transaction = value;'):
            with self.subTest(statement=statement):
                self.write('src/phases.rs', GOOD.replace('let a = cex', statement + ' let a = cex'))
                self.reject()

    def test_missing_remote_and_duplicate_phase_still_rejected(self):
        for source in (GOOD.replace('submit_authorized_settlement_intent', 'not_remote'), GOOD + 'fn capture_match() {pool.begin();}'):
            self.write('src/phases.rs', source)
            self.reject()

    def test_worker_path_layout_regression(self):
        self.write('src/settlement_worker.rs', '#[path="settlement_worker_legacy.rs"] mod implementation;')
        self.write('src/settlement_worker_legacy.rs', '#[path="settlement_worker_runtime_v2.rs"] mod runtime_v2;\n' + GOOD)
        self.write('src/settlement_worker_runtime_v2.rs', 'pub async fn run_v2() {}')
        reader, _ = self.scan('src/settlement_worker.rs')
        self.assertEqual(len(reader.seen), 3)

    def test_actor_runtime_path_layout_regression(self):
        self.write('src/lib.rs', '#[path="lib_parts/actor_runtime/mod.rs"] mod actor_runtime;')
        self.write('src/lib_parts/actor_runtime/mod.rs', 'mod part_03; pub use part_03::settle_pending_matches;')
        self.write('src/lib_parts/actor_runtime/part_03.rs', GOOD + '''
pub async fn settle_pending_matches() -> Result<(), String> {
    Err("terminal settlement is owned by trnm-settlement-worker".to_owned())
}
''')
        reader, stream = self.scan()
        body = checker.function(stream, 'settle_pending_matches')
        self.assertTrue(any(t.kind == 'string' and 'trnm-settlement-worker' in t.value for t in body))
        self.assertFalse(checker.remote_calls(body))
        self.assertEqual(len(reader.seen), 3)

    def test_cli_rejects_unsafe_nested_phase(self):
        self.write('src/phases.rs', GOOD.replace('let tx = pool.begin()', 'cex.authorize_settlement_intent(intent); let tx = pool.begin()', 1))
        result = subprocess.run([sys.executable, str(SCRIPT), 'scan-only', str(self.root / 'src')],
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 1)
        self.assertNotIn('PASS', result.stdout)
        self.assertIn('contains direct signer/CEX execution', result.stderr)


if __name__ == '__main__':
    unittest.main(verbosity=2)
