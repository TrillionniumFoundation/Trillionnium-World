#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
CONTRACT_ROOT="$ROOT_DIR/trillionnium/contracts/trnm-world-transition-v1"
SOURCE="$CONTRACT_ROOT/src/contract.rs"
CANONICAL_SOURCE="$CONTRACT_ROOT/src/canonical_json.rs"
LEGACY_SOURCE="$CONTRACT_ROOT/src/lib.rs"
CARGO="$CONTRACT_ROOT/Cargo.toml"
LOCK="$CONTRACT_ROOT/Cargo.lock"
DOC="$ROOT_DIR/docs/protocol/trnm-world-transition-v1.md"
SCHEMA="$ROOT_DIR/docs/protocol/schemas/trnm-world-transition-v1.schema.json"
VECTORS="$ROOT_DIR/docs/protocol/vectors/trnm-world-transition-v1.json"
STATUS="$ROOT_DIR/docs/development/trnm-world-transition-contract-v1.json"

fail() {
  printf 'TRNM World transition contract failed: %s\n' "$*" >&2
  exit 1
}

for file in "$SOURCE" "$CANONICAL_SOURCE" "$CARGO" "$LOCK" "$DOC" "$SCHEMA" "$VECTORS" "$STATUS"; do
  [[ -f "$file" ]] || fail "missing required artifact: ${file#$ROOT_DIR/}"
done
[[ ! -e "$LEGACY_SOURCE" ]] \
  || fail 'duplicate legacy src/lib.rs would let checks and builds drift'
grep -q '^path = "src/contract.rs"$' "$CARGO" \
  || fail 'Cargo must compile the same strict source root that is checked and packaged'

if grep -q '^\[dependencies\]' "$CARGO"; then
  fail 'reference contract package must remain dependency-free'
fi
grep -q '^\[workspace\]$' "$CARGO" \
  || fail 'reference package must remain an independently testable workspace'
grep -q 'pub const CONTRACT_VERSION: &str = "trnm_world_transition_v1"' "$SOURCE" \
  || fail 'Rust contract version drift'
grep -q 'pub trait WorldRulesAdapterV1' "$SOURCE" \
  || fail 'deterministic adapter boundary is missing'
grep -q 'pub fn execute_transition' "$SOURCE" \
  || fail 'deterministic transition entry point is missing'
grep -q 'REQUEST_HASH_DOMAIN' "$SOURCE" \
  || fail 'request hash domain is missing'
grep -q 'TRANSITION_HASH_DOMAIN' "$SOURCE" \
  || fail 'transition hash domain is missing'
grep -q 'OUTCOME_HASH_DOMAIN' "$SOURCE" \
  || fail 'outcome hash domain is missing'

if grep -nE '(^|[^A-Za-z0-9_])(struct|enum|type)[[:space:]]+MatchCompletedV1|pub[[:space:]]+[A-Za-z0-9_]*(session_token|private_key|archive_root|finality|app_hash|global_event_cursor)[A-Za-z0-9_]*[[:space:]]*:' "$SOURCE"; then
  fail 'reference package acquired a forbidden authority field or completion type'
fi

python3 - "$SCHEMA" "$VECTORS" "$SOURCE" "$STATUS" <<'PY'
from __future__ import annotations

import hashlib
import json
import pathlib
import re
import sys
from typing import Any

schema_path = pathlib.Path(sys.argv[1])
vectors_path = pathlib.Path(sys.argv[2])
source_path = pathlib.Path(sys.argv[3])
status_path = pathlib.Path(sys.argv[4])


def reject(message: str) -> None:
    raise SystemExit(message)


def expect(condition: bool, message: str) -> None:
    if not condition:
        reject(message)


def no_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    output: dict[str, Any] = {}
    for key, value in pairs:
        if key in output:
            reject(f"duplicate JSON key: {key}")
        output[key] = value
    return output


def load_json(path: pathlib.Path) -> Any:
    return json.loads(
        path.read_text(encoding="utf-8"),
        object_pairs_hook=no_duplicates,
        parse_constant=lambda value: reject(f"non-finite JSON number: {value}"),
    )


def canonical(value: Any) -> str:
    def reject_floats(node: Any) -> None:
        if isinstance(node, float):
            reject("floating-point numbers are forbidden in transition vectors")
        if isinstance(node, dict):
            for child in node.values():
                reject_floats(child)
        elif isinstance(node, list):
            for child in node:
                reject_floats(child)

    reject_floats(value)
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def digest(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def property_names(node: Any) -> set[str]:
    names: set[str] = set()
    if isinstance(node, dict):
        properties = node.get("properties")
        if isinstance(properties, dict):
            names.update(properties)
        for child in node.values():
            names.update(property_names(child))
    elif isinstance(node, list):
        for child in node:
            names.update(property_names(child))
    return names


schema = load_json(schema_path)
vectors = load_json(vectors_path)
status = load_json(status_path)
source = source_path.read_text(encoding="utf-8")

expect(schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema", "unexpected JSON Schema draft")
expect(schema.get("$defs", {}).get("request", {}).get("additionalProperties") is False, "request must reject unknown fields")
expect(schema.get("$defs", {}).get("accepted", {}).get("additionalProperties") is False, "accepted result must reject unknown fields")
expect(schema.get("$defs", {}).get("rejected", {}).get("additionalProperties") is False, "rejected result must reject unknown fields")
expect(schema.get("$defs", {}).get("canonicalPayload", {}).get("additionalProperties") is False, "canonical payload must reject unknown fields")
expect(vectors.get("vector_contract") == "trnm_world_transition_vectors_v1", "unexpected vector contract")
expect(vectors.get("schema_contract_version") == "trnm_world_transition_v1", "vector/schema contract mismatch")

domains = vectors.get("hash_domains")
expect(domains == {
    "request": "trnm.world.transition.request.v1",
    "transition": "trnm.world.transition.accepted.v1",
    "outcome": "trnm.world.outcome.v1",
}, "hash-domain vector drift")
for domain in domains.values():
    expect(domain in source, f"Rust source is missing hash domain {domain}")

for vector in vectors.get("sha256_core_vectors", []):
    expect(digest(vector["input_utf8"]) == vector["expected_sha256"], f"SHA-256 core vector failed: {vector.get('name')}")

for vector in vectors.get("payload_vectors", []):
    encoded = canonical(vector["canonical_json"])
    expect(encoded == vector["expected_canonical_utf8"], f"payload canonicalization drift: {vector.get('name')}")
    expect(digest(encoded) == vector["expected_sha256"], f"payload hash drift: {vector.get('name')}")

sha_pattern = re.compile(r"^[0-9a-f]{64}$")
for vector in vectors.get("request_vectors", []):
    request = vector["request"]
    expect(request.get("contract_version") == "trnm_world_transition_v1", f"request version drift: {vector.get('name')}")
    encoded = canonical(request)
    expect(encoded == vector["expected_canonical_json"], f"request canonicalization drift: {vector.get('name')}")
    for payload in (request["previous_state"], request["command"]["payload"]):
        payload_json = canonical(payload["canonical_json"])
        expect(digest(payload_json) == payload["sha256"], f"request payload hash mismatch: {vector.get('name')}")
        expect(sha_pattern.fullmatch(payload["sha256"]) is not None, "request payload SHA-256 must be lower-case hex")

for vector in vectors.get("accepted_facts_vectors", []):
    accepted = vector["accepted_facts"]
    encoded = canonical(accepted)
    expect(encoded == vector["expected_canonical_json"], f"accepted-facts canonicalization drift: {vector.get('name')}")
    for key in ("next_state", "replay_material"):
        payload = accepted[key]
        expect(digest(canonical(payload["canonical_json"])) == payload["sha256"], f"accepted payload hash mismatch: {vector.get('name')}:{key}")

expected_codes = vectors.get("stable_error_codes")
schema_codes = schema.get("$defs", {}).get("errorCode", {}).get("enum")
expect(isinstance(expected_codes, list) and len(expected_codes) == len(set(expected_codes)), "error catalogue contains duplicates")
expect(expected_codes == schema_codes, "schema/vector error catalogue drift")
for code in expected_codes:
    expect(f'"{code}"' in source, f"Rust source is missing stable error code {code}")

forbidden = set(vectors.get("forbidden_authority_keys", []))
expect(forbidden, "forbidden authority key set is empty")
properties = property_names(schema)
intersection = forbidden.intersection(properties)
expect(not intersection, f"schema exposes forbidden authority properties: {sorted(intersection)}")
for key in forbidden:
    expect(f'"{key}"' in source, f"Rust defense-in-depth denylist is missing {key}")

required_request = set(schema["$defs"]["request"]["required"])
expect(required_request == {
    "command",
    "content_revision",
    "contract_version",
    "expected_tick",
    "previous_state",
    "ruleset_revision",
    "transition_id",
}, "request required-field contract drift")

expect(schema["$defs"]["statePayload"]["x-trnm-max-canonical-bytes"] == 2 * 1024 * 1024, "state budget drift")
expect(schema["$defs"]["commandPayload"]["x-trnm-max-canonical-bytes"] == 128 * 1024, "command budget drift")
expect(schema["$defs"]["replayPayload"]["x-trnm-max-canonical-bytes"] == 2 * 1024 * 1024, "replay budget drift")
expect(schema["$defs"]["outcomePayload"]["x-trnm-max-canonical-bytes"] == 512 * 1024, "outcome budget drift")

expect(status.get("contract_version") == "trnm_world_transition_delivery_v1", "delivery contract version drift")
expect(status.get("backlog_id") == "WORLD-P0-002", "delivery backlog identity drift")
expect(status.get("status") == "implemented_pending_exact_commit_ci", "delivery status overclaim")
expect(status.get("production_candidate") is False, "transition contract may not become a production candidate here")
artifacts = status.get("artifacts", {})
expect(artifacts.get("compiled_source") == "trillionnium/contracts/trnm-world-transition-v1/src/contract.rs", "delivery truth points at the wrong compiled source")
expect(artifacts.get("canonical_json_source") == "trillionnium/contracts/trnm-world-transition-v1/src/canonical_json.rs", "delivery truth points at the wrong canonical source")
expect(artifacts.get("package_builder") == "scripts/package-trnm-world-transition-contract.py", "delivery truth omits the package builder")
expect(artifacts.get("package_check") == "scripts/check-trnm-world-transition-package.sh", "delivery truth omits the package gate")
verification = status.get("verification", {})
expect(verification.get("github_actions_runs") == "missing", "missing exact-head CI may not be promoted")
expect(verification.get("remote_ci_covers_current_head") is False, "delivery truth falsely claims current-head CI")
expect(verification.get("independent_exact_head_review") == "pending", "delivery truth falsely claims independent review")
acceptance = status.get("acceptance", [])
expect(isinstance(acceptance, list) and {item.get("id") for item in acceptance} == {f"WORLD-P0-002-A{index}" for index in range(1, 8)}, "delivery acceptance catalogue drift")
expect(all(item.get("state") in {"implemented_pending_ci", "pending_WORLD-P0-003"} for item in acceptance), "delivery acceptance contains a promoted state")
blockers = set(status.get("promotion_blockers", []))
expect({
    "exact_head_workflow_runs_missing_or_not_green",
    "nakama_cross_repository_shadow_not_yet_green",
    "integration_exact_commit_lock_not_yet_published",
    "independent_exact_head_review_pending",
}.issubset(blockers), "delivery promotion blockers were weakened")
not_owned = set(status.get("authority_scope", {}).get("world_does_not_own", []))
expect({
    "online_participant_admission",
    "canonical_global_event_order",
    "online_command_idempotency",
    "canonical_archive_root",
    "match_completed_v1",
    "match_evidence_signing_key",
    "chain_finality",
    "wallet_or_ledger_settlement",
}.issubset(not_owned), "delivery authority exclusions were weakened")

print("TRNM World transition schema, vectors, hashes, delivery truth and authority boundary passed.")
PY

printf '%s\n' 'TRNM World deterministic transition contract passed.'
