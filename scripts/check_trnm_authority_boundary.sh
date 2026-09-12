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
  docs/development/trnm-world-gap-closure-ledger-v4.json \
  docs/development/trnm-world-gap-closure-ledger-v6.json \
  docs/status/world-plan-v4-execution-truth-2026-09-08.json \
  docs/adr/0001-realtime-authority-and-match-evidence-ownership.md \
  docs/adr/0002-transaction-free-external-settlement.md \
  docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md \
  docs/protocol/trnm-match-evidence-commitment-v1.md \
  docs/development/trnm-settlement-outbox-v1.md; do
  [[ -s "$required" ]] || fail "missing required boundary document: $required"
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
  .authority.world_domain_service_workspace ==
    "trillionnium/crates/world-authority/Cargo.toml" and
  .authority.world_production_adapter_contract ==
    "trillionnium_world_production_adapter_v1" and
  .authority.world_production_authorization == "not_granted" and
  (.authority.world | index("deterministic-simulation") != null) and
  (.authority.world | index("game-outcome-hash") != null) and
  (.authority.forbidden_world_claims | index("canonical-match-completion-signature") != null) and
  (.compatibility_enclaves | any(
    .path == "trillionnium/crates/trnm-game-server" and
    .profile == "world_legacy_local_alpha" and
    .new_public_authority_contracts == "forbid" and
    .canonical_match_evidence == false
  )) and
  .documentation.current_plan == "CURRENT_PLAN.md" and
  .documentation.current_execution_snapshot ==
    "docs/status/world-plan-v4-execution-truth-2026-09-08.json" and
  .documentation.current_gap_ledger ==
    "docs/development/trnm-world-gap-closure-ledger-v6.json" and
  .documentation.planning_foundation ==
    "docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md" and
  .documentation.historical_plan_manifest ==
    "docs/development/trillionnium-world-development-plan-2026-08-29.json" and
  .documentation.historical_gap_ledger ==
    "docs/development/trnm-world-gap-closure-ledger-v4.json" and
  .documentation.historical_machine_inputs_are_execution_authority == false and
  .documentation.authority_adr ==
    "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md" and
  .documentation.domain_service_adr ==
    "docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md" and
  .runtime.external_settlement_under_mutable_transaction == "forbid" and
  .runtime.fixture_listener == "explicit-opt-in-loopback-only" and
  .ci.validation_repository_permissions == "read_only" and
  .ci.self_modifying_candidate_source == "forbid" and
  .ci.exact_head_required_for_credit == true and
  .ci.missing_skipped_cancelled_stale_checks == "block" and
  .release.public_online == "no_go" and
  .release.public_player_market == "disabled" and
  .release.production_authorization == "not_granted"
' PROJECT_BOUNDARY.json >/dev/null \
  || fail "PROJECT_BOUNDARY.json authority contract is incomplete or stale"

# The 2026-08-29 JSON remains a historical planning input. Validate its
# provenance and durable authority decisions without promoting it above
# CURRENT_PLAN.md, the V6 ledger or the selected September execution snapshot.
jq -e '
  .schema == "trnm_world_development_plan_v4" and
  .plan_id == "trillionnium-world-development-2026-08-29-v4" and
  .as_of == "2026-08-29" and
  .working_branch == "fix/world-plan-gap-closure-v4" and
  .verified_commit == null and
  .release_effect == "none" and
  .public_online == "no_go" and
  .public_player_market == "disabled" and
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
  || fail "historical machine-readable development plan integrity drift"

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
  || fail "docs index does not expose the planning foundation"
grep -Fq 'trillionnium-world-development-plan-2026-08-29.json' CURRENT_PLAN.md \
  || fail "current plan does not expose the historical machine plan"
grep -Fq 'trnm-world-gap-closure-ledger-v4.json' CURRENT_PLAN.md \
  || fail "current plan does not expose the historical gap ledger"

bash scripts/check-trnm-world-authority-boundary.sh full

printf '%s\n' \
  'TRNM authority boundary: green (World deterministic domain / Nakama online authority / Chain finality / CEX ledger / Integration evidence)'
