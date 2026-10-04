from __future__ import annotations
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[1]
RECORDER = ROOT/'scripts/run-trnm-world-recorded-command.py'

@unittest.skipUnless(os.name == 'posix', 'POSIX process-group recorder')
class RecorderTests(unittest.TestCase):
    def command(self, directory, code, *opts):
        return [sys.executable,str(RECORDER),'--log',str(directory/'output.log'),
                '--metadata',str(directory/'execution.json'),'--kill-grace-seconds','0.15',
                *opts,'--',sys.executable,'-S','-c',code]

    def invoke(self, code, *opts):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            proc = subprocess.run(self.command(directory,code,*opts), cwd=tmp,capture_output=True,timeout=10)
            metadata = json.loads((directory/'execution.json').read_text())
            log = (directory/'output.log').read_bytes()
            self.assertEqual(metadata['retained_log_sha256'],hashlib.sha256(log).hexdigest())
            self.assertFalse(metadata['independent_acceptance'])
            self.assertEqual(metadata['production_authorization'],'not_granted')
            return proc, metadata, log

    def test_success_and_output_hash(self):
        proc,meta,log=self.invoke("print('checked')")
        self.assertEqual(proc.returncode,0)
        self.assertEqual(log,b'checked\n')
        self.assertTrue(meta['raw_command_output_complete'])
        self.assertFalse(meta['source_identity_bound'])

    def test_failure_retains_output_and_original_exit(self):
        proc,meta,log=self.invoke("import sys; print('failure details',flush=True); sys.exit(7)")
        self.assertEqual(proc.returncode,7)
        self.assertEqual(meta['child_exit_code'],7)
        self.assertIn(b'failure details',log)
        self.assertFalse(meta['command_completed_successfully'])

    def test_stdout_and_stderr_are_both_retained(self):
        proc,_,log=self.invoke("import sys; print('out',flush=True); print('err',file=sys.stderr,flush=True)")
        self.assertEqual(proc.returncode,0)
        self.assertIn(b'out',log);self.assertIn(b'err',log)

    def test_truncated_success_cannot_receive_full_evidence_credit(self):
        proc,meta,log=self.invoke("print('a'*2000)",'--max-bytes','100')
        self.assertEqual(proc.returncode,125)
        self.assertEqual(len(log),100)
        self.assertEqual(meta['output_bytes_observed'],2001)
        self.assertTrue(meta['output_truncated'])
        self.assertFalse(meta['raw_command_output_complete'])

    def test_failure_exit_is_preserved_when_log_is_truncated(self):
        proc,meta,_=self.invoke("import sys;print('a'*2000);sys.exit(9)",'--max-bytes','100')
        self.assertEqual(proc.returncode,9)
        self.assertTrue(meta['output_truncated'])

    def test_timeout_retains_partial_output(self):
        proc,meta,log=self.invoke("import time;print('started',flush=True);time.sleep(20)",'--timeout-seconds','1')
        self.assertEqual(proc.returncode,124)
        self.assertTrue(meta['timed_out']);self.assertIn(b'started',log)

    def test_closed_stdout_does_not_terminate_a_still_running_child(self):
        proc,meta,_=self.invoke("import os,time;os.close(1);os.close(2);time.sleep(0.3)",'--timeout-seconds','2')
        self.assertEqual(proc.returncode,0)
        self.assertFalse(meta['timed_out'])

    def test_duplicate_output_path_is_refused_without_overwrite(self):
        with tempfile.TemporaryDirectory() as tmp:
            d=Path(tmp);(d/'output.log').write_bytes(b'prior-evidence')
            proc=subprocess.run(self.command(d,"print('new')"),capture_output=True,timeout=10)
            self.assertNotEqual(proc.returncode,0)
            self.assertEqual((d/'output.log').read_bytes(),b'prior-evidence')

    def test_symlink_log_is_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            d=Path(tmp);(d/'target').write_bytes(b'prior')
            (d/'output.log').symlink_to(d/'target')
            proc=subprocess.run(self.command(d,"print('new')"),capture_output=True,timeout=10)
            self.assertNotEqual(proc.returncode,0)
            self.assertEqual((d/'target').read_bytes(),b'prior')

    def test_child_signal_is_not_converted_to_success(self):
        proc,meta,_=self.invoke("import os,signal;os.kill(os.getpid(),signal.SIGTERM)")
        self.assertEqual(proc.returncode,143)
        self.assertFalse(meta['command_completed_successfully'])

    def test_recorder_signal_is_propagated_and_recorded(self):
        with tempfile.TemporaryDirectory() as tmp:
            d=Path(tmp)
            proc=subprocess.Popen(self.command(d,"import time;print('ready',flush=True);time.sleep(20)"),
                                  cwd=tmp,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
            deadline=time.monotonic()+3
            while time.monotonic()<deadline:
                if (d/'output.log').exists() and (d/'output.log').stat().st_size:
                    break
                time.sleep(0.02)
            # The file buffer may not flush until completion; wait at least long
            # enough for the signal handler to have been installed.
            proc.send_signal(signal.SIGTERM)
            proc.communicate(timeout=5)
            meta=json.loads((d/'execution.json').read_text())
            self.assertEqual(proc.returncode,143)
            self.assertEqual(meta['interrupted_by'],signal.SIGTERM)
            self.assertFalse(meta['raw_command_output_complete'])

    def test_spawn_failure_is_not_a_pass(self):
        with tempfile.TemporaryDirectory() as tmp:
            d=Path(tmp)
            cmd=self.command(d,'pass')
            cmd=cmd[:cmd.index('--')+1]+['/definitely/nonexistent-program']
            proc=subprocess.run(cmd,capture_output=True,timeout=10)
            self.assertNotEqual(proc.returncode,0)
            self.assertNotIn(b'nonexistent-program',proc.stderr)



@unittest.skipUnless(os.name == 'posix', 'POSIX evidence recorder')
class ExactSourceRecorderTests(unittest.TestCase):
    command = RecorderTests.command
    # Reuse helpers without counting the inherited tests a second time.
    def git(self, root, *args):
        return subprocess.check_output(['git', *args], cwd=root, stderr=subprocess.DEVNULL).decode().strip()

    def repository(self, root):
        self.git(root, 'init', '-q')
        self.git(root, 'config', 'user.name', 'Evidence fixture')
        self.git(root, 'config', 'user.email', 'fixture@example.invalid')
        (root/'tracked').write_text('before')
        self.git(root, 'add', 'tracked')
        self.git(root, 'commit', '-qm', 'fixture before')
        return self.git(root, 'rev-parse', 'HEAD'), self.git(root, 'rev-parse', 'HEAD^{tree}')

    def execute_in(self, root, code, *opts):
        proc = subprocess.run(self.command(root, code, *opts), cwd=root,
                              capture_output=True, timeout=8)
        meta = json.loads((root/'execution.json').read_text())
        self.assertEqual(meta['recorder_exit_code'], proc.returncode)
        self.assertEqual(meta['retained_log_sha256'],
                         hashlib.sha256((root/'output.log').read_bytes()).hexdigest())
        self.assertEqual(meta['production_authorization'], 'not_granted')
        return proc, meta

    def test_required_source_without_git_never_starts_command(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            proc, meta = self.execute_in(root, "open('marker','w').write('bad')", '--require-source-binding')
            self.assertEqual(proc.returncode, 125)
            self.assertTrue(meta['source_precondition_failed'])
            self.assertFalse(meta['command_started'])
            self.assertFalse((root/'marker').exists())

    def test_clean_expected_commit_and_tree_are_bound(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); head, tree = self.repository(root)
            proc, meta = self.execute_in(root, "print('bound')", '--require-source-binding',
                                        '--expected-commit', head, '--expected-tree', tree)
            self.assertEqual(proc.returncode, 0)
            self.assertTrue(meta['source_identity_bound'])
            self.assertTrue(meta['source_identity_required'])
            self.assertEqual(meta['source_before']['commit'], head)
            self.assertEqual(meta['source_before']['tree'], tree)
            self.assertEqual(meta['source_before'], meta['source_after'])

    def test_expected_tuple_mismatch_does_not_execute(self):
        for field in ('--expected-commit', '--expected-tree'):
            with self.subTest(field=field), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp); self.repository(root)
                proc, meta = self.execute_in(root, "open('marker','w').write('bad')", field, '0'*40)
                self.assertEqual(proc.returncode, 125)
                self.assertFalse(meta['command_started'])
                self.assertFalse((root/'marker').exists())

    def test_dirty_initial_tree_does_not_execute(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); self.repository(root)
            (root/'tracked').write_text('already dirty')
            proc, meta = self.execute_in(root, "open('marker','w').write('bad')", '--require-source-binding')
            self.assertEqual(proc.returncode, 125)
            self.assertFalse(meta['command_started'])
            self.assertFalse((root/'marker').exists())

    def test_child_commit_movement_is_not_green(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); self.repository(root)
            code = "from pathlib import Path;import subprocess;Path('tracked').write_text('after');subprocess.run(['git','commit','-qam','after'],check=True)"
            proc, meta = self.execute_in(root, code)
            self.assertEqual(proc.returncode, 125)
            self.assertEqual(meta['child_exit_code'], 0)
            self.assertTrue(meta['source_changed'])
            self.assertFalse(meta['source_identity_bound'])
            self.assertFalse(meta['command_completed_successfully'])

    def test_child_tracked_mutation_without_commit_is_not_green(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); self.repository(root)
            proc, meta = self.execute_in(root, "open('tracked','w').write('after')", '--require-source-binding')
            self.assertEqual(proc.returncode, 125)
            self.assertEqual(meta['source_before']['commit'], meta['source_after']['commit'])
            self.assertFalse(meta['source_after']['tracked_clean'])

    def test_nonzero_child_exit_survives_source_change(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); self.repository(root)
            proc, meta = self.execute_in(root, "import sys;open('tracked','w').write('after');sys.exit(9)", '--require-source-binding')
            self.assertEqual(proc.returncode, 9)
            self.assertTrue(meta['source_changed'])
            self.assertFalse(meta['source_identity_bound'])

    def test_untracked_evidence_does_not_change_tracked_source(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); self.repository(root)
            proc, meta = self.execute_in(root, "open('untracked-output','w').write('evidence')", '--require-source-binding')
            self.assertEqual(proc.returncode, 0)
            self.assertTrue(meta['source_identity_bound'])

    def test_missing_executable_still_has_failure_record(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cmd = self.command(root, 'pass')
            cmd = cmd[:cmd.index('--')+1] + ['/definitely/missing-executable']
            proc = subprocess.run(cmd, cwd=root, capture_output=True, timeout=8)
            meta = json.loads((root/'execution.json').read_text())
            self.assertEqual(proc.returncode, 70)
            self.assertFalse(meta['command_started'])
            self.assertEqual(meta['failure_class'], 'FileNotFoundError')
            self.assertFalse(meta['raw_command_output_complete'])
            self.assertNotIn('missing-executable', json.dumps(meta))

    def test_escaped_descendant_cannot_hold_recorder_forever(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            code = "import subprocess,sys;p=subprocess.Popen([sys.executable,'-S','-c','import time;time.sleep(20)'],start_new_session=True);print(p.pid,flush=True)"
            start = time.monotonic()
            try:
                proc, meta = self.execute_in(root, code, '--timeout-seconds', '0.5')
                self.assertLess(time.monotonic()-start, 5)
                self.assertIn(proc.returncode, (124, 125))
                self.assertTrue(meta['output_drain_forced_closed'])
                self.assertFalse(meta['raw_command_output_complete'])
                self.assertEqual(meta['descendant_cleanup'], 'not_proved_without_runner_containment')
            finally:
                path = root/'output.log'
                if path.is_file() and path.read_text().strip().isdigit():
                    try: os.kill(int(path.read_text().strip()), signal.SIGKILL)
                    except ProcessLookupError: pass

    def test_linked_ancestor_directory_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); (root/'real').mkdir()
            (root/'alias').symlink_to(root/'real', target_is_directory=True)
            cmd = self.command(root/'alias', "print('bad')")
            proc = subprocess.run(cmd, cwd=root, capture_output=True, timeout=8)
            self.assertNotEqual(proc.returncode, 0)
            self.assertFalse((root/'real/output.log').exists())

    def test_metadata_is_not_overwritten(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); (root/'execution.json').write_text('original')
            proc = subprocess.run(self.command(root, 'pass'), cwd=root, capture_output=True, timeout=8)
            self.assertNotEqual(proc.returncode, 0)
            self.assertEqual((root/'execution.json').read_text(), 'original')

    def test_malformed_expected_hash_never_runs(self):
        for value in ('abc', 'A'*40, 'g'*40):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                proc = subprocess.run(self.command(root, "open('marker','w').write('bad')", '--expected-commit', value),
                                      cwd=root, capture_output=True, timeout=8)
                self.assertNotEqual(proc.returncode, 0)
                self.assertFalse((root/'marker').exists())

    def test_invalid_timeout_and_log_limits_rejected(self):
        for opt, value in (('--timeout-seconds', 'nan'), ('--timeout-seconds', 'inf'),
                           ('--timeout-seconds', '0'), ('--kill-grace-seconds', '-1'),
                           ('--max-bytes', '0')):
            with self.subTest(opt=opt, value=value), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                proc = subprocess.run(self.command(root, "open('marker','w').write('bad')", opt, value),
                                      cwd=root, capture_output=True, timeout=8)
                self.assertNotEqual(proc.returncode, 0)
                self.assertFalse((root/'marker').exists())


if __name__ == '__main__':
    unittest.main()
