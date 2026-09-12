"""Independent source/policy regression checks; hosted CI is not implied."""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
INTAKE = ROOT / 'trillionnium/crates/world-authority/trnm-world-api/src/production_contract/intake.rs'
STATE = ROOT / 'trillionnium/crates/world-authority/trnm-world-api/src/production_contract/state_intake.rs'
PREFLIGHT = ROOT / 'trillionnium/crates/world-authority/trnm-world-server/src/bin/production_preflight/mod.rs'
BINARY = ROOT / 'trillionnium/crates/world-authority/trnm-world-server/src/bin/trnm-world-production-preflight.rs'
BASE = ROOT / 'trillionnium/crates/world-authority/trnm-world-api/src/production_contract/base.rs'
GATE = ROOT / 'scripts/check-trnm-world-priority-contracts.sh'


class PriorityContractTests(unittest.TestCase):
    def test_adapter_scope_is_complete_for_selected_boundary(self):
        source = INTAKE.read_text()
        expected = {
            'WorldCredentialReference', 'WorldIdentityResolutionRequest',
            'WorldSessionAuthorizationRequest', 'WorldAccountOperationRequest',
            'WorldMutationContext', 'WorldAggregateLoadRequest',
            'WorldAggregateRequestLookup', 'WorldLedgerLookupRequest',
            'WorldLedgerSubmitRequest', 'WorldEvidenceAppendRequest',
            'WorldMetricRecordRequest', 'WorldRoutingAuthorizationRequest',
            'WorldRoutingDecision', 'WorldDeploymentIdentity',
            'WorldProductionAdapterError',
        }
        actual = set(re.findall(r'intake_value!\((\w+),', source))
        self.assertEqual(actual, expected)
        self.assertIn('MAX_PRODUCTION_INPUT_BYTES: usize = 128 * 1024', source)
        self.assertIn('duplicate JSON key', source)

    def test_error_enum_has_stable_snake_case_policy(self):
        source = BASE.read_text()
        self.assertIn('#[serde(rename_all = "snake_case")]', source)
        match = re.search(r'pub enum WorldProductionAdapterErrorCode \{(.*?)\n\}', source, re.S)
        self.assertIsNotNone(match)
        self.assertGreaterEqual(len(re.findall(r'^\s+[A-Z]\w+,', match.group(1), re.M)), 20)

    def test_state_boundary_has_explicit_budgets_and_references(self):
        source = STATE.read_text()
        for marker in (
            'MAX_WORLD_STATE_INPUT_BYTES: usize = 2 * 1024 * 1024',
            'MAX_WORLD_COMMAND_INPUT_BYTES: usize = 128 * 1024',
            'MissingReference', 'DuplicateIdentity', 'WORLD_DOMAIN_CONTRACT',
        ):
            self.assertIn(marker, source)
        for collection in ('nodes', 'edges', 'positions', 'npcs', 'tasks', 'receipts'):
            self.assertIn('"' + collection + '"', source)

    def test_preflight_rejects_known_url_confusions(self):
        source = PREFLIGHT.read_text()
        for marker in ('Ipv4Addr', 'Ipv6Addr', 'numeric_tail', 'localhost',
                       'is_multicast', 'file.take(cap)', 'file_type().is_symlink()'):
            self.assertIn(marker, source)
        block = source.split('for value in [', 1)[1].split('] {', 1)[0]
        self.assertGreaterEqual(len(re.findall(r'"https?://', block)), 35)

    def test_preflight_binary_does_not_claim_dns_or_filesystem_atomicity(self):
        source = BINARY.read_text()
        self.assertIn('"dns_egress_verified": false', source)
        self.assertIn('"filesystem_checks_are_point_in_time": true', source)
        self.assertIn('production_preflight::read_bounded_config', source)
        self.assertNotIn('fs::read(&path)', source)

    def test_no_adapter_implementation_or_production_credit_is_smuggled_in(self):
        source = INTAKE.read_text() + STATE.read_text()
        for forbidden in ('production_ready: true', 'fixture_adapter_credit_allowed: true',
                          'production_authorization: "granted"'):
            self.assertNotIn(forbidden, source)
        self.assertIn('does NOT authenticate callers', INTAKE.read_text())
        self.assertIn('No authority or persistence', STATE.read_text())

    def test_check_gate_requires_real_rust_execution(self):
        gate = GATE.read_text()
        for marker in ('cargo fmt', 'cargo test', 'cargo clippy', 'set -euo pipefail'):
            self.assertIn(marker, gate)
        self.assertNotIn('|| true', gate)


if __name__ == '__main__':
    unittest.main()
