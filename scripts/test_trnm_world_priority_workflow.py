"""Workflow source checks only; they never certify hosted execution."""
import os
from pathlib import Path
import re
import subprocess
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
WORKFLOW=ROOT/'.github/workflows/world-pr-native-admission-v1.yml'

class WorkflowSourceTests(unittest.TestCase):
    def test_read_only_permissions_and_required_contexts_are_preserved(self):
        source=WORKFLOW.read_text()
        self.assertIn('permissions:\n  contents: read',source)
        self.assertNotRegex(source,r'(?:contents|statuses|pull-requests|actions):\s*write')
        for name in ('trnm-game-ci','trnm-world-p0-boundaries','trnm-world-status-evidence'):
            self.assertIn('context: '+name,source)
        self.assertIn('test "$QUALIFICATION_RESULT" = success',source)
        self.assertIn('refs/pull/${PR_NUMBER}/merge',source)
        self.assertNotIn('continue-on-error',source)

    def test_all_scopes_are_recorded_and_collection_runs_on_failure(self):
        source=WORKFLOW.read_text()
        for scope in ('truth','source','postgres','package'):
            self.assertIn('recorded '+scope,source)
        block=source.split('- name: Collect exact-object artifacts after success or failure',1)[1]
        self.assertTrue(block.lstrip().startswith('if: always()'))
        self.assertIn('--metadata "$GITHUB_WORKSPACE/evidence/${label}/${scope}-execution.json"',source)

    def test_recorders_require_the_expected_head_or_merge_tuple(self):
        source=WORKFLOW.read_text()
        self.assertIn('expected_commit="$HEAD_SHA"',source)
        self.assertIn('expected_commit="$MERGE_SHA"',source)
        self.assertEqual(source.count('--require-source-binding --expected-commit "$expected_commit"'),2)
        self.assertEqual(source.count('--expected-tree "$expected_tree" --timeout-seconds'),2)
        self.assertIn("-p 'test_trnm_world_priority_*.py'",source)
        self.assertNotIn('check-trnm-world-priority-contracts.sh',source)

    def test_yaml_and_embedded_bash_syntax(self):
        try:import yaml
        except ImportError:self.fail('PyYAML is required for workflow validation')
        workflow=yaml.load(WORKFLOW.read_text(),Loader=yaml.BaseLoader)
        for job in workflow['jobs'].values():
            for step in job['steps']:
                if 'run' not in step:continue
                script=re.sub(r'\$\{\{.*?\}\}','source',step['run'])
                result=subprocess.run(['bash','-n'],input=script,text=True,capture_output=True)
                self.assertEqual(result.returncode,0,(step['name'],result.stderr))

    def test_post_failure_collector_keeps_partial_artifacts(self):
        try:import yaml
        except ImportError:self.fail('PyYAML is required for workflow validation')
        workflow=yaml.load(WORKFLOW.read_text(),Loader=yaml.BaseLoader)
        step=next(s for s in workflow['jobs']['qualification']['steps'] if s['name'].startswith('Collect exact-object'))
        script=re.sub(r'\$\{\{.*?\}\}','source',step['run'])
        with tempfile.TemporaryDirectory() as tmp:
            d=Path(tmp);partial=d/'runner/world-head/run/world-v6-v2/source'
            partial.mkdir(parents=True);(partial/'partial.json').write_text('{"finished":false}')
            env=dict(os.environ,RUNNER_TEMP=str(d/'runner'))
            result=subprocess.run(['bash','-c',script],cwd=d,env=env,capture_output=True,text=True)
            self.assertEqual(result.returncode,0,result.stderr)
            self.assertEqual((d/'evidence/head/source/partial.json').read_text(),'{"finished":false}')

if __name__=='__main__':unittest.main()
