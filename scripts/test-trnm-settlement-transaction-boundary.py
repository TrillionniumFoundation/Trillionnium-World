#!/usr/bin/env python3
"""Offline hostile fixtures for source-static settlement checks, not Rust/DB proof."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name('check-trnm-settlement-transaction-boundary.py')
spec = importlib.util.spec_from_file_location('settlement_boundary', SCRIPT)
assert spec is not None and spec.loader is not None
checker = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = checker
spec.loader.exec_module(checker)
ROOT = SCRIPT.resolve().parents[1]
GOOD = '''async fn capture_match(pool: &Pool) {
    let tx = pool.begin().await.unwrap(); tx.commit().await.unwrap();
}
async fn process_claimed_job(cex: &Cex, intent: &Intent) {
    let a = cex.authorize_settlement_intent(intent).await.unwrap();
    cex.submit_authorized_settlement_intent(&a.intent).await.unwrap();
}
async fn apply_capture(pool: &Pool) {
    let tx = pool.begin().await.unwrap(); tx.commit().await.unwrap();
}
'''


def scan(source: str) -> None:
    checker.scan_phases(checker.tokens(source))


class PhaseFixtures(unittest.TestCase):
    def rejects(self, source: str) -> None:
        with self.assertRaises(checker.BoundaryFailure):
            scan(source)

    def test_good_three_phase_fixture(self):
        scan(GOOD)

    def test_multiline_comment_separated_calls(self):
        scan(GOOD.replace('.authorize_settlement_intent(', '. /* note */\n authorize_settlement_intent /* note */ ('))

    def test_raw_identifiers_preserve_required_calls(self):
        scan(GOOD.replace('.authorize_settlement_intent(', '.r#authorize_settlement_intent('))

    def test_transaction_openers(self):
        for name in ('begin', 'begin_with', 'transaction'):
            with self.subTest(name=name):
                scan(GOOD.replace('.begin()', f'.{name}()'))

    def test_remote_call_in_capture_rejected(self):
        self.rejects(GOOD.replace('let tx = pool.begin()', 'cex.authorize_settlement_intent(intent); let tx = pool.begin()', 1))

    def test_remote_call_in_apply_rejected(self):
        self.rejects(GOOD.replace('async fn apply_capture(pool: &Pool) {', 'async fn apply_capture(pool: &Pool) { cex.submit_authorized_settlement_intent(intent);'))

    def test_raw_identifier_remote_call_under_lock_rejected(self):
        self.rejects(GOOD.replace('let tx = pool.begin()', 'cex.r#authorize_settlement_intent(intent); let tx = pool.begin()', 1))

    def test_qualified_remote_symbol_under_lock_rejected(self):
        self.rejects(GOOD.replace('let tx = pool.begin()', 'Cex::authorize_settlement_intent(cex,intent); let tx = pool.begin()', 1))

    def test_transaction_in_remote_phase_rejected(self):
        for opener in ('let tx=pool.begin();', 'let tx=pool.begin_with(opts);', 'let tx=pool.transaction();', 'let tx: Transaction = value;'):
            with self.subTest(opener=opener):
                self.rejects(GOOD.replace('let a = cex', opener + '\n let a = cex'))

    def test_row_lock_variants_rejected(self):
        for clause in ('for update', 'FOR SHARE', 'for no key update', 'for key share', 'FOR\n UPDATE SKIP LOCKED'):
            with self.subTest(clause=clause):
                self.rejects(GOOD.replace('let a = cex', f'let q=r##"select * from t {clause}"##; let a = cex'))

    def test_escaped_row_locks_rejected(self):
        for literal in ('"select * from t for\\x20update"', '"select * from t for\\u{20}share"', 'b"for\\tupdate"'):
            with self.subTest(literal=literal):
                self.rejects(GOOD.replace('let a = cex', f'let q={literal}; let a = cex'))

    def test_comments_cannot_supply_missing_remote_call(self):
        self.rejects(GOOD.replace('cex.submit_authorized_settlement_intent(&a.intent).await.unwrap();', '// cex.submit_authorized_settlement_intent(&a.intent);'))

    def test_string_cannot_supply_missing_remote_call(self):
        self.rejects(GOOD.replace('cex.submit_authorized_settlement_intent(&a.intent).await.unwrap();', 'let s="cex.submit_authorized_settlement_intent(&a.intent)";'))

    def test_symbol_reference_is_not_invocation(self):
        self.rejects(GOOD.replace('cex.submit_authorized_settlement_intent(&a.intent).await.unwrap();', 'let f = Cex::submit_authorized_settlement_intent;'))

    def test_begin_field_is_not_transaction_call(self):
        self.rejects(GOOD.replace('pool.begin()', 'pool.begin'))

    def test_quoted_begin_does_not_own_transaction(self):
        self.rejects(GOOD.replace('pool.begin()', '"pool.begin()"'))

    def test_capture_without_transaction_rejected(self):
        self.rejects(GOOD.replace('let tx = pool.begin().await.unwrap(); tx.commit().await.unwrap();', 'let n=0;', 1))

    def test_missing_and_duplicate_phase_rejected(self):
        self.rejects(GOOD.replace('fn capture_match', 'fn other'))
        self.rejects(GOOD + 'async fn apply_capture(pool:&Pool) {pool.begin();}')

    def test_signature_without_implementation_rejected(self):
        self.rejects(GOOD + 'fn capture_match(pool: &Pool);')

    def test_comment_function_not_implementation(self):
        self.rejects('/*' + GOOD + '*/')

    def test_raw_string_function_not_implementation(self):
        self.rejects('const FAKE: &str = r####"' + GOOD + '"####;')

    def test_macro_function_not_ordinary_implementation(self):
        for wrapped in ('fake!{' + GOOD + '}', 'macro_rules! fake {() => {' + GOOD + '}}'):
            with self.subTest(wrapped=wrapped[:30]):
                self.rejects(wrapped)

    def test_nested_comments_fake_symbols_ignored(self):
        scan('/* outer /* nested } fn capture_match {} */ still } */\n' + GOOD)

    def test_strings_and_chars_cannot_end_phase(self):
        literal = '''let a="}\\\"{ "; let b=r###"} /* fake */ \" {"###; let c=b'}'; let d='{';'''
        scan(GOOD.replace('let tx = pool.begin()', literal + '\nlet tx = pool.begin()', 1))

    def test_lifetime_syntax_supported(self):
        scan(GOOD.replace('fn capture_match(pool: &Pool)', "fn capture_match<'a>(pool: &'a Pool)"))

    def test_string_remote_text_in_capture_not_call(self):
        scan(GOOD.replace('let tx = pool.begin()', 'let x=r#"cex.authorize_settlement_intent()"#; let tx = pool.begin()', 1))

    def test_unterminated_source_rejected(self):
        for broken in ('/* unterminated', 'r#"unterminated', '"unterminated', GOOD[:-2], GOOD.replace('pool.begin()', 'pool.begin(]')):
            with self.subTest(broken=broken[-40:]):
                self.rejects(broken)

    def test_invalid_or_unsupported_escape_rejected(self):
        for literal in ('"\\q"', '"\\xZZ"', '"\\u{D800}"', '"\\u{110000}"', '"\\u{___}"'):
            with self.subTest(literal=literal):
                with self.assertRaises(checker.BoundaryFailure):
                    checker.tokens(literal)

    def test_string_continuation_and_unicode_decode(self):
        self.assertEqual(checker.tokens('"a\\\n  b\\u{20}c"'), [checker.Token('string', 'ab c')])

    def test_depth_and_token_budgets(self):
        with patch.object(checker, 'MAX_NESTING', 2):
            with self.assertRaises(checker.BoundaryFailure):
                checker.tokens('/* /* /* x */ */ */')
            with self.assertRaises(checker.BoundaryFailure):
                checker.group_end(checker.tokens('((()))'), 0)
        with patch.object(checker, 'MAX_TOKENS', 2):
            with self.assertRaises(checker.BoundaryFailure):
                checker.tokens('a b c')


class IncludeFixtures(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='trnm-boundary-fixture-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root/'src').mkdir()
        (self.root/'src/body.rs').write_text(GOOD, encoding='utf-8')

    def entry(self, text: str):
        (self.root/'src/lib.rs').write_text(text, encoding='utf-8')

    def scan(self):
        scan_tokens = checker.SourceBundle(self.root).bundle('src/lib.rs')
        checker.scan_phases(scan_tokens)

    def rejects(self, text: str):
        self.entry(text)
        with self.assertRaises((checker.BoundaryFailure, UnicodeError)):
            self.scan()

    def test_direct_include(self):
        self.entry('include!("body.rs");')
        self.scan()

    def test_crate_bound_concat_include(self):
        self.entry('include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/body.rs"));')
        self.scan()

    def test_trailing_comma_include(self):
        self.entry('include!("body.rs",);')
        self.scan()

    def test_fake_include_in_comment_and_string_ignored(self):
        self.entry('// include!("missing.rs");\nconst S:&str=r#"include!(\"missing.rs\")"#;\ninclude!("body.rs");')
        self.scan()

    def test_unsafe_include_paths(self):
        for path in ('../outside.rs', '/tmp/outside.rs', '.git/config', 'src//body.rs', 'C:/bad.rs'):
            with self.subTest(path=path):
                self.rejects(f'include!("{path}");')

    def test_generated_and_unrecognized_include_expressions(self):
        for expr in ('concat!(env!("OUT_DIR"), "/generated.rs")', 'SOME_PATH', 'concat!("src", "/body.rs")', 'concat!(env!("CARGO_MANIFEST_DIR"), "/../outside.rs")'):
            with self.subTest(expr=expr):
                self.rejects(f'include!({expr});')

    def test_duplicate_and_cyclic_include(self):
        self.rejects('include!("body.rs"); include!("body.rs");')
        self.rejects('include!("lib.rs");')

    def test_missing_include(self):
        self.rejects('include!("absent.rs");')

    def test_empty_and_non_utf8_source(self):
        self.entry('include!("body.rs");')
        for data in (b'', b' \n', b'\xff'):
            (self.root/'src/body.rs').write_bytes(data)
            with self.subTest(data=data), self.assertRaises((checker.BoundaryFailure, UnicodeError)):
                self.scan()

    def test_symlink_and_dangling_include(self):
        self.entry('include!("link.rs");')
        p=self.root/'src/link.rs'
        for target in ('body.rs', 'missing.rs'):
            p.symlink_to(target)
            with self.subTest(target=target), self.assertRaises(checker.BoundaryFailure):
                self.scan()
            p.unlink()

    def test_linked_source_root(self):
        p=self.root/'alias'; p.symlink_to(self.root/'src', target_is_directory=True)
        with self.assertRaises(checker.BoundaryFailure):
            checker.SourceBundle(p)

    def test_file_and_bundle_resource_limits(self):
        self.entry('include!("body.rs");')
        for constant, value in (('MAX_FILE_BYTES', 12), ('MAX_FILES', 1), ('MAX_BUNDLE_BYTES', len(GOOD.encode())), ('MAX_TOKENS', 4)):
            with self.subTest(constant=constant), patch.object(checker, constant, value):
                with self.assertRaises(checker.BoundaryFailure):
                    self.scan()

    def test_cli_fixture_is_used_instead_of_repository(self):
        (self.root/'lib.rs').write_text(GOOD, encoding='utf-8')
        result=subprocess.run([sys.executable,str(SCRIPT),'scan-only',str(self.root),'--root','/definitely-absent'],capture_output=True,text=True,timeout=15)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertIn('explicit fixture source only',result.stdout)
        (self.root/'lib.rs').write_text(GOOD.replace('fn capture_match','fn other'))
        result=subprocess.run([sys.executable,str(SCRIPT),'scan-only',str(self.root)],capture_output=True,text=True,timeout=15)
        self.assertNotEqual(result.returncode,0)
        self.assertNotIn('PASS',result.stdout)

    def test_shell_wrapper_forwards_fixture(self):
        (self.root/'lib.rs').write_text(GOOD, encoding='utf-8')
        shell=SCRIPT.with_suffix('.sh')
        result=subprocess.run(['bash',str(shell),'scan-only',str(self.root)],capture_output=True,text=True,timeout=15)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertIn('explicit fixture',result.stdout)

    def test_cli_rejects_unknown_extra_missing_or_full_fixture(self):
        for args in (['unknown'],['full',str(self.root)],['scan-only',str(self.root),'extra'],['scan-only',str(self.root/'missing')]):
            with self.subTest(args=args):
                result=subprocess.run([sys.executable,str(SCRIPT),*args],capture_output=True,text=True,timeout=15)
                self.assertNotEqual(result.returncode,0)


class RepositoryFixtures(unittest.TestCase):
    """Test the full source scanner against a reduced temporary real-source copy.

    Copying source is not compilation, deployed testing or verification of the
    original repository head. Each mutation starts from the actual local files.
    """
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory(prefix='trnm-boundary-repo-')
        self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)
        self.relative='trillionnium/crates/trnm-game-server'
        shutil.copytree(ROOT/self.relative,self.root/self.relative)
        (self.root/'.github/workflows').mkdir(parents=True)
        for name in ('trnm-world-gap-closure-v4.yml','trnm-world-v4-final-gates.yml'):
            p=ROOT/'.github/workflows'/name
            if p.is_file():
                shutil.copyfile(p,self.root/'.github/workflows'/name)
        self.crate=self.root/self.relative

    def rejects(self):
        with self.assertRaises((checker.BoundaryFailure,UnicodeError,ValueError)):
            checker.check_repository(self.root,True)

    def test_real_direct_source_baseline(self):
        result=checker.check_repository(self.root,True)
        self.assertEqual(result['evidence_class'],'source_static')
        self.assertFalse(result['rust_execution'])
        self.assertFalse(result['database_execution'])
        self.assertFalse(result['remote_evidence'])

    def test_build_and_template_debt_rejected(self):
        for relative in ('build.rs','src/lib.rs.in','src/settlement_worker.rs.in'):
            p=self.crate/relative; p.write_text('not allowed')
            with self.subTest(relative=relative):self.rejects()
            p.unlink()

    def test_dangling_deleted_template_rejected(self):
        p=self.crate/'src/lib.rs.in';p.symlink_to('missing.rs')
        self.rejects()

    def test_cargo_build_opt_in_rejected(self):
        p=self.crate/'Cargo.toml';p.write_text(p.read_text().replace('[package]','[package]\nbuild="custom.rs"',1))
        self.rejects()

    def test_generated_environment_reference_rejected(self):
        p=self.crate/'src/lib.rs';p.write_text(p.read_text()+'\nconst EXTRA:&str=env!("OUT_DIR");\n')
        self.rejects()

    def test_game_direct_settlement_call_rejected(self):
        p=self.crate/'src/lib.rs';p.write_text(p.read_text()+'\nfn bad(){cex.authorize_settlement_intent(intent);}\n')
        self.rejects()

    def test_missing_direct_source_include_rejected(self):
        p=self.crate/'src/lib_parts/identity/part_01.rs';p.unlink()
        self.rejects()

    def test_missing_migration_registry_rejected(self):
        p=self.crate/'src/lib_parts/configuration_and_migrations/part_01.rs'
        p.write_text(p.read_text().replace('0019_online_settlement_quarantine_v1','removed_quarantine_registration'))
        self.rejects()

    def test_worker_v2_registration_not_satisfied_by_include(self):
        p=self.crate/'src/settlement_worker_runtime_v2.rs'
        source=p.read_text()
        # Retain include_str!(migration file), remove only runtime registration.
        p.write_text(source.replace('"0019_online_settlement_quarantine_v1",','"missing_migration",'))
        self.rejects()

    def test_blocking_cex_reference_rejected(self):
        p=self.crate/'src/cex.rs';p.write_text(p.read_text()+'\nfn bad(){reqwest::blocking::get(url);}\n')
        self.rejects()

    def test_missing_receipt_lookup_rejected(self):
        p=self.crate/'src/cex.rs';p.write_text(p.read_text().replace('fn lookup_signer_receipt','fn removed_lookup_signer_receipt'))
        self.rejects()

    def test_cascading_settlement_lineage_rejected(self):
        p=self.crate/'migrations/0016_online_settlement_outbox_v1.sql';p.write_text(p.read_text().replace('ON DELETE RESTRICT','ON DELETE CASCADE').replace('on delete restrict','on delete cascade'))
        self.rejects()

    def test_missing_lease_fence_rejected(self):
        p=self.crate/'migrations/0017_online_settlement_worker_runtime_v1.sql';p.write_text(p.read_text().replace('lease_expires_at > pg_catalog.clock_timestamp()','true'))
        self.rejects()

    def test_operator_append_only_marker_required(self):
        p=self.crate/'migrations/0018_online_settlement_operator_controls_v1.sql';p.write_text(p.read_text().replace('BEFORE TRUNCATE','AFTER TRUNCATE').replace('before truncate','after truncate'))
        self.rejects()

    def test_fake_test_function_comment_rejected(self):
        p=self.crate/'tests/settlement_game_server_boundary.rs'
        p.write_text(p.read_text().replace('fn direct_sources_never_restore_generated_authority','fn unrelated')+'\n// fn direct_sources_never_restore_generated_authority() {}\n')
        self.rejects()

    def test_workflow_must_exist(self):
        for p in (self.root/'.github/workflows').iterdir():p.unlink()
        self.rejects()

    def test_echoed_commands_cannot_supply_workflow_checks(self):
        for p in (self.root/'.github/workflows').iterdir():
            lines=[('  echo "'+line.strip()+'"') if line.strip().startswith('cargo ') else line for line in p.read_text().splitlines()]
            p.write_text('\n'.join(lines)+'\n')
        self.rejects()

    def test_workflow_write_permission_rejected(self):
        for p in (self.root/'.github/workflows').iterdir():p.write_text(p.read_text().replace('contents: read','contents: write'))
        self.rejects()

    def test_workflow_missing_strict_clippy_rejected(self):
        for p in (self.root/'.github/workflows').iterdir():p.write_text(p.read_text().replace('-- -D warnings','-- -A warnings'))
        self.rejects()


class RuntimeSourceFixtures(unittest.TestCase):
    def setUp(self):
        RepositoryFixtures.setUp(self)
        path = self.crate / 'Cargo.toml'
        source = path.read_text()
        # The negative fixture must remain hostile after the real source is repaired.
        lines = source.splitlines(keepends=True)
        for index, line in enumerate(lines):
            if line.startswith('reqwest = ') and '"blocking"' not in line:
                lines[index] = line.replace('features = [', 'features = ["blocking", ', 1)
        path.write_text(''.join(lines))

    def runtime(self):
        path = SCRIPT.with_name('check-trnm-settlement-runtime-status.py')
        spec = importlib.util.spec_from_file_location('runtime_status_fixture', path)
        assert spec is not None and spec.loader is not None
        module = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = module
        spec.loader.exec_module(module)
        return module

    def without_blocking_fixture(self):
        p=self.crate/'Cargo.toml'
        source=p.read_text()
        self.assertIn('"blocking", ',source)
        p.write_text(source.replace('"blocking", ',''))

    def test_fixed_artifact_blocking_feature_is_rejected(self):
        runtime=self.runtime()
        with self.assertRaisesRegex(runtime.Invalid,'blocking HTTP support returned'):
            runtime.validate_source(self.root)

    def test_synthetic_nonblocking_fixture_passes_source_inventory(self):
        self.without_blocking_fixture()
        self.runtime().validate_source(self.root)

    def test_shutdown_requirement_not_relaxed(self):
        self.without_blocking_fixture()
        p=self.crate/'src/settlement_worker_runtime_v2.rs'
        p.write_text(p.read_text().replace('SignalKind::terminate','SignalKind::other'))
        runtime=self.runtime()
        with self.assertRaisesRegex(runtime.Invalid,'runtime v2 lost markers'):
            runtime.validate_source(self.root)

    def test_quarantine_requirement_not_relaxed(self):
        self.without_blocking_fixture()
        p=self.crate/'migrations/0019_online_settlement_quarantine_v1.sql'
        p.write_text(p.read_text().replace('trnm_online_resolve_settlement_quarantine_v1','removed_quarantine_resolution'))
        runtime=self.runtime()
        with self.assertRaisesRegex(runtime.Invalid,'quarantine migration lost markers'):
            runtime.validate_source(self.root)

    def test_legacy_source_generation_still_rejected(self):
        self.without_blocking_fixture()
        (self.crate/'build.rs').write_text('fn main() {}')
        runtime=self.runtime()
        with self.assertRaisesRegex(runtime.Invalid,'semantic template/generator remains'):
            runtime.validate_source(self.root)

    def test_composite_gate_keeps_exact_ci_requirement(self):
        import json
        status=json.loads((ROOT/'docs/status/settlement-runtime-v1.json').read_text())
        self.assertIn('run_exact_head_v4_checks',status['open_gates'])
        wrapper=(ROOT/'scripts/check_trnm_settlement_transaction_boundary.sh').read_text()
        self.assertIn('"run_exact_head_v4_checks"',wrapper)
        self.assertIn('scripts/test-trnm-settlement-transaction-boundary-negative.sh',wrapper)
        self.assertIn('scripts/test-trnm-settlement-runtime-status-negative.py',wrapper)
        self.assertNotIn('"obtain_exact_commit_github_actions_evidence"',wrapper)


    def test_unpublished_inventory_remains_planned(self):
        import json
        status = json.loads((ROOT / 'docs/status/settlement-runtime-v1.json').read_text())
        self.assertEqual(status['status'], 'planned')
        self.assertIsNone(status['verified_commit'])
        gate = 'publish_reviewed_direct_source_and_successor_manifest'
        self.assertIn(gate, status['open_gates'])
        wrapper = (ROOT / 'scripts/check_trnm_settlement_transaction_boundary.sh').read_text()
        self.assertIn('"' + gate + '"', wrapper)


if __name__ == '__main__':
    unittest.main(verbosity=2)
