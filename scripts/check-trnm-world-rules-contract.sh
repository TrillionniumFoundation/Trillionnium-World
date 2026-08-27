#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
CONTRACT_ROOT="$ROOT_DIR/trillionnium/contracts/trnm-world-rules-contract-v1"
SHADOW_ROOT="$ROOT_DIR/tools/trnm-world-shadow-diff"
LOCK="$ROOT_DIR/integration/component-locks/trnm-world-rules-v1.lock.json"
RUNBOOK="$ROOT_DIR/docs/runbooks/trnm-world-nakama-authority-cutover-v1.md"

fail() {
  printf 'TRNM World rules contract failed: %s\n' "$*" >&2
  exit 1
}

required=(
  "$CONTRACT_ROOT/Cargo.toml"
  "$CONTRACT_ROOT/Cargo.lock"
  "$CONTRACT_ROOT/contract-manifest-v1.json"
  "$CONTRACT_ROOT/src/lib.rs"
  "$CONTRACT_ROOT/src/canonical.rs"
  "$CONTRACT_ROOT/src/digest.rs"
  "$CONTRACT_ROOT/src/engine.rs"
  "$CONTRACT_ROOT/src/error.rs"
  "$CONTRACT_ROOT/src/model.rs"
  "$CONTRACT_ROOT/src/bin/trnm-world-rules-vector.rs"
  "$CONTRACT_ROOT/schema/transition-request-v1.schema.json"
  "$CONTRACT_ROOT/schema/transition-receipt-v1.schema.json"
  "$CONTRACT_ROOT/schema/error-catalog-v1.json"
  "$CONTRACT_ROOT/vectors/first-contact-vector-0001.json"
  "$SHADOW_ROOT/reference_contract.py"
  "$SHADOW_ROOT/trnm_world_shadow_diff.py"
  "$SHADOW_ROOT/test_reference_contract.py"
  "$SHADOW_ROOT/test_shadow_diff.py"
  "$LOCK"
  "$RUNBOOK"
)
for file in "${required[@]}"; do
  [[ -f "$file" ]] || fail "required artifact is missing: ${file#$ROOT_DIR/}"
done

if grep -RniE --include='*.rs' \
  'use (reqwest|sqlx|axum|tokio|uuid|ring|ed25519|secp256k1)|extern crate (reqwest|sqlx|axum|tokio)' \
  "$CONTRACT_ROOT/src"; then
  fail 'deterministic contract imports network, persistence or signing dependencies'
fi

if grep -q '^\[dependencies\]' "$CONTRACT_ROOT/Cargo.toml"; then
  fail 'standalone contract unexpectedly gained third-party dependencies'
fi

grep -q 'AUTHORITY_SCOPE: &str = "deterministic_world_rules_only"' \
  "$CONTRACT_ROOT/src/lib.rs" || fail 'authority scope is missing or changed'
grep -q 'TRNM-WORLD-RULES-REQUEST/1' "$CONTRACT_ROOT/src/canonical.rs" \
  || fail 'request domain separator is missing'
grep -q 'TRNM-WORLD-RULES-RESULT/1' "$CONTRACT_ROOT/src/canonical.rs" \
  || fail 'result domain separator is missing'
grep -q 'execute_transition_verified' "$CONTRACT_ROOT/src/engine.rs" \
  || fail 'determinism verifier is missing'
grep -q 'NondeterministicResult' "$CONTRACT_ROOT/src/engine.rs" \
  || fail 'nondeterminism does not fail closed'
grep -q 'There is no automatic cross-generation live takeover' "$RUNBOOK" \
  || fail 'cutover runbook must explicitly reject automatic live takeover'

python3 - "$ROOT_DIR" <<'PY'
from __future__ import annotations

import json
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
contract = root / "trillionnium/contracts/trnm-world-rules-contract-v1"
manifest = json.loads((contract / "contract-manifest-v1.json").read_text())
request_schema = json.loads((contract / "schema/transition-request-v1.schema.json").read_text())
receipt_schema = json.loads((contract / "schema/transition-receipt-v1.schema.json").read_text())
error_catalog = json.loads((contract / "schema/error-catalog-v1.json").read_text())
vector = json.loads((contract / "vectors/first-contact-vector-0001.json").read_text())
lock = json.loads((root / "integration/component-locks/trnm-world-rules-v1.lock.json").read_text())
source = (contract / "src/model.rs").read_text()
error_source = (contract / "src/error.rs").read_text()


def expect(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


expect(manifest.get("contract_id") == "trnm_world_rules_v1", "contract ID drift")
expect(manifest.get("release_id") == "trnm_world_rules_v1@1.0.0-alpha.1", "release drift")
expect(manifest.get("authority_scope") == "deterministic_world_rules_only", "authority scope drift")
expect(manifest.get("canonical_encoding") == "trnm-canonical-lines-v1", "canonical encoding drift")
expect(manifest.get("hash_algorithm") == "sha256", "hash algorithm drift")
expect(manifest.get("compatibility", {}).get("unknown_contract") == "reject", "unknown contract must reject")
expect(manifest.get("compatibility", {}).get("unknown_ruleset") == "reject", "unknown ruleset must reject")
expect(manifest.get("compatibility", {}).get("diagnostics_committed") is False, "diagnostics must not be committed")

request_fields = {
    "contract_version",
    "ruleset_revision",
    "content_revision",
    "transition_id",
    "state_canonical_hex",
    "command_canonical_hex",
    "resource_budget",
}
expect(request_schema.get("additionalProperties") is False, "request schema must reject unknown fields")
expect(set(request_schema.get("required", [])) == request_fields, "request required fields drift")
expect(set(request_schema.get("properties", {})) == request_fields, "request properties drift")
for forbidden in {
    "player_id",
    "account_id",
    "session_token",
    "admission",
    "global_sequence",
    "archive_root",
    "signature",
    "private_key",
    "wallet",
    "chain_finality",
}:
    expect(forbidden not in request_schema.get("properties", {}), f"forbidden request authority field: {forbidden}")

receipt_fields = {
    "contract_version",
    "ruleset_revision",
    "content_revision",
    "transition_id",
    "request_hash",
    "state_before_hash",
    "command_hash",
    "disposition",
    "error_code",
    "state_after_hash",
    "outcome_hash",
    "replay_hash",
    "steps_used",
    "output_bytes",
    "replay_bytes",
    "transition_hash",
}
expect(receipt_schema.get("additionalProperties") is False, "receipt schema must reject unknown fields")
expect(set(receipt_schema.get("required", [])) == receipt_fields, "receipt required fields drift")
expect(set(receipt_schema.get("properties", {})) == receipt_fields, "receipt properties drift")

match = re.search(r"pub struct TransitionRequest \{(?P<body>.*?)\n\}", source, re.S)
expect(match is not None, "TransitionRequest declaration missing")
rust_fields = set(re.findall(r"pub\s+([a-z_][a-z0-9_]*)\s*:", match.group("body")))
expected_rust_fields = {
    "contract_version",
    "ruleset_revision",
    "content_revision",
    "transition_id",
    "state_canonical",
    "command_canonical",
    "budget",
}
expect(rust_fields == expected_rust_fields, f"Rust request surface drift: {sorted(rust_fields)}")

catalog_codes = [item.get("code") for item in error_catalog.get("errors", [])]
expect(len(catalog_codes) == 12 and len(catalog_codes) == len(set(catalog_codes)), "error catalogue is incomplete or duplicated")
rust_codes = set(re.findall(r'=>\s+"([a-z0-9_]+)"', error_source))
expect(set(catalog_codes) == rust_codes, f"Rust/schema error catalogue drift: catalog={catalog_codes} rust={sorted(rust_codes)}")
expect(error_catalog.get("unknown_error_policy") == "fail_closed_as_internal_contract_error", "unknown errors must fail closed")

expect(vector.get("contract_release") == manifest.get("release_id"), "vector release drift")
expect(vector.get("request", {}).get("contract_version") == manifest.get("contract_id"), "vector contract drift")
expect(vector.get("standard_hash_vector", {}).get("sha256") == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", "standard SHA-256 vector drift")

expect(lock.get("lock_version") == "trnm_integration_component_lock_v1", "component lock version drift")
expect(lock.get("status") == "candidate", "repository lock must remain candidate before Integration binding")
expect(lock.get("activation") == "shadow_only", "component lock may only activate shadow mode here")
expect(lock.get("producer", {}).get("immutable_release") == manifest.get("release_id"), "component lock release drift")
expect(lock.get("producer", {}).get("authority_scope") == "deterministic_world_rules_only", "component lock authority drift")
expect(lock.get("consumer", {}).get("required_contract_version") == manifest.get("contract_id"), "consumer contract drift")
expect(lock.get("compatibility", {}).get("unexplained_shadow_divergence") == "block_promotion", "divergence must block promotion")
expect(lock.get("compatibility", {}).get("world_local_authority_public") is False, "compatibility enclave may not become public")
lock_text = json.dumps(lock)
for forbidden_path in ("/home/", "/Users/", "../CEX", "../Nakama", "file://"):
    expect(forbidden_path not in lock_text, f"component lock contains local path coupling: {forbidden_path}")
PY

python3 -m unittest discover -s "$SHADOW_ROOT" -p 'test_*.py'

printf '%s\n' 'TRNM World deterministic rules contract and shadow conformance passed.'
