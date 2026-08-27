#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

fail() {
  printf 'authority-boundary: %s\n' "$*" >&2
  exit 1
}

command -v jq >/dev/null 2>&1 || fail "jq is required"

for required in \
  PROJECT_BOUNDARY.json \
  PROJECT_BOUNDARY.md \
  CURRENT_PLAN.md \
  docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md \
  docs/development/trillionnium-world-development-plan-2026-08-27.json \
  docs/adr/0001-realtime-authority-and-match-evidence-ownership.md \
  docs/adr/0002-transaction-free-external-settlement.md \
  docs/protocol/trnm-match-evidence-commitment-v1.md \
  docs/development/trnm-settlement-outbox-v1.md; do
  [[ -s "$required" ]] || fail "missing required current boundary document: $required"
done

jq -e '
  .schema == 2 and
  .project_id == "trillionnium-world" and
  .lane == "game-product" and
  .authority.online_match_authority == "Trillionnium-Nakama" and
  .authority.chain_finality_authority == "Trillionnium-Chain" and
  .authority.wallet_settlement_authority == "CEX" and
  .authority.cross_repository_evidence_authority == "Trillionnium-Integration" and
  (.authority.world | index("deterministic-simulation") != null) and
  (.authority.world | index("game-outcome-hash") != null) and
  (.authority.forbidden_world_claims | index("canonical-match-completion-signature") != null) and
  (.compatibility_enclaves | any(
    .path == "trillionnium/crates/trnm-game-server" and
    .new_public_authority_contracts == "forbid" and
    .canonical_match_evidence == false
  )) and
  .documentation.current_plan ==
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md" and
  .documentation.authority_adr ==
    "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md" and
  .documentation.current_plan_manifest ==
    "docs/development/trillionnium-world-development-plan-2026-08-27.json"
' PROJECT_BOUNDARY.json >/dev/null || fail "PROJECT_BOUNDARY.json authority contract is incomplete"

jq -e '
  .schema == 1 and
  .status == "current" and
  .decision.online_match_authority == "Trillionnium-Nakama" and
  .decision.world_local_game_server_status == "compatibility-authority-enclave" and
  (.work_items | any(.id == "WORLD-P0-001" and .priority == "P0")) and
  (.work_items | any(.id == "WORLD-P0-003" and .owner_repository == "Trillionnium-Nakama")) and
  (.no_go | index("external network I/O under mutable match or campaign row locks") != null)
' docs/development/trillionnium-world-development-plan-2026-08-27.json >/dev/null \
  || fail "machine-readable development plan is incomplete"

for expected in \
  'Nakama is the canonical online match authority' \
  'World must never load, derive or re-sign with the Nakama authority private key' \
  'World does not import Chain crates through sibling filesystem paths'; do
  grep -Fq "$expected" docs/protocol/trnm-match-evidence-commitment-v1.md \
    || fail "match-evidence contract is missing: $expected"
done

grep -Fiq 'compatibility authority enclave' PROJECT_BOUNDARY.md \
  || fail "PROJECT_BOUNDARY.md does not identify the compatibility enclave"
grep -Fq '0001-realtime-authority-and-match-evidence-ownership.md' docs/README.md \
  || fail "docs index does not expose the authority ADR"
grep -Fq 'TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md' docs/README.md \
  || fail "docs index does not expose the current development plan"

rust_authority_violation="$(
  grep -RInE \
    --include='*.rs' \
    'MatchCompletedV1|NAKAMA[^[:space:]]*(PRIVATE|SECRET)[_ -]?KEY|nakama[^[:space:]]*(private|secret)[_ -]?key' \
    trillionnium/crates 2>/dev/null || true
)"
[[ -z "$rust_authority_violation" ]] || {
  printf '%s\n' "$rust_authority_violation" >&2
  fail "World Rust source contains a prohibited Nakama/canonical-completion authority surface"
}

path_dependency_violation="$(
  grep -RInE \
    --include='Cargo.toml' \
    'path[[:space:]]*=[[:space:]]*"[^\"]*(Trillionnium-(Chain|Nakama|Integration)|/CEX|\.\./CEX|\.\./Chain|\.\./Nakama)' \
    trillionnium 2>/dev/null || true
)"
[[ -z "$path_dependency_violation" ]] || {
  printf '%s\n' "$path_dependency_violation" >&2
  fail "cross-repository sibling path dependency detected"
}

printf '%s\n' \
  'TRNM authority boundary: green (World deterministic domain / Nakama online authority / Chain finality / CEX ledger / Integration evidence)'
