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
  docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md \
  docs/development/trillionnium-world-development-plan-2026-08-29.json \
  docs/adr/0001-realtime-authority-and-match-evidence-ownership.md \
  docs/adr/0002-transaction-free-external-settlement.md \
  docs/protocol/trnm-match-evidence-commitment-v1.md \
  docs/development/trnm-settlement-outbox-v1.md; do
  [[ -s "$required" ]] || fail "missing required current boundary document: $required"
done

jq -e '
  .schema == 3 and
  .project_id == "trillionnium-world" and
  .lane == "game-product" and
  .remote.canonical_slug == "TrillionniumFoundation/Trillionnium-World" and
  .authority.online_match_authority == "Trillionnium-Nakama" and
  .authority.chain_finality_authority == "Trillionnium-Chain" and
  .authority.wallet_settlement_authority == "CEX" and
  .authority.cross_repository_evidence_authority == "Trillionnium-Integration" and
  (.authority.world | index("deterministic-simulation") != null) and
  (.authority.world | index("game-outcome-hash") != null) and
  (.authority.forbidden_world_claims | index("canonical-match-completion-signature") != null) and
  (.compatibility_enclaves | any(
    .path == "trillionnium/crates/trnm-game-server" and
    .profile == "world_legacy_local_alpha" and
    .new_public_authority_contracts == "forbid" and
    .canonical_match_evidence == false
  )) and
  .documentation.current_plan ==
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md" and
  .documentation.current_plan_manifest ==
    "docs/development/trillionnium-world-development-plan-2026-08-29.json" and
  .documentation.authority_adr ==
    "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md" and
  .runtime.external_settlement_under_mutable_transaction == "forbid" and
  .ci.validation_repository_permissions == "read_only" and
  .ci.self_modifying_candidate_source == "forbid" and
  .ci.exact_head_required_for_credit == true and
  .ci.missing_skipped_cancelled_stale_checks == "block" and
  .release.public_online == "no_go" and
  .release.public_player_market == "disabled"
' PROJECT_BOUNDARY.json >/dev/null \
  || fail "PROJECT_BOUNDARY.json authority contract is incomplete or stale"

jq -e '
  .schema == "trnm_world_development_plan_v4" and
  .plan_id == "trillionnium-world-development-2026-08-29-v4" and
  .status == "current_candidate" and
  .decision.online_match_authority == "Trillionnium-Nakama" and
  .decision.chain_finality_authority == "Trillionnium-Chain" and
  .decision.wallet_settlement_authority == "CEX" and
  .decision.cross_repository_evidence_authority == "Trillionnium-Integration" and
  .decision.world_local_game_server == "world_legacy_local_alpha_compatibility_enclave" and
  (.immediate_work_items | any(.id == "WORLD-P0-001" and .owner == "Trillionnium-World")) and
  (.immediate_work_items | any(.id == "WORLD-P0-003" and .owner == "Trillionnium-Nakama")) and
  (.immediate_work_items | any(.id == "WORLD-P0-005" and .state == "server_configuration_required")) and
  (.no_go | index("remote_settlement_before_capture_commit") != null) and
  (.no_go | index("missing_or_bypassed_required_checks") != null)
' docs/development/trillionnium-world-development-plan-2026-08-29.json >/dev/null \
  || fail "machine-readable development plan is incomplete or stale"

for expected in \
  'Nakama is the canonical online match authority' \
  'World must not load, derive, proxy or back up the Nakama authority private key' \
  'World-local authority is `world_legacy_local_alpha`' \
  'Exactly one component is externally authoritative'; do
  grep -Fq "$expected" docs/adr/0001-realtime-authority-and-match-evidence-ownership.md \
    || fail "authority ADR is missing: $expected"
done

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
grep -Fq 'TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md' docs/README.md \
  || fail "docs index does not expose the current development plan"

bash scripts/check-trnm-world-authority-boundary.sh full

printf '%s\n' \
  'TRNM authority boundary: green (World deterministic domain / Nakama online authority / Chain finality / CEX ledger / Integration evidence)'
