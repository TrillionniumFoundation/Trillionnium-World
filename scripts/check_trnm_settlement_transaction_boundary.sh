#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

fail() {
  printf 'settlement-transaction-boundary: %s\n' "$*" >&2
  exit 1
}

legacy_file="trillionnium/crates/trnm-game-server/src/lib.rs"
adr="docs/adr/0002-transaction-free-external-settlement.md"
design="docs/development/trnm-settlement-outbox-v1.md"
contract="trillionnium/tools/trnm-settlement-outbox-contract/src/lib.rs"

for required in "$legacy_file" "$adr" "$design" "$contract"; do
  [[ -s "$required" ]] || fail "missing required settlement boundary file: $required"
done

# The current compatibility server has one registered migration debt: its
# terminal settlement loop still calls the synchronous EconomyBackend while a
# mutable PostgreSQL transaction owns match/campaign row locks. This check does
# not grant that path credit. It prevents a second copy from being added while
# WORLD-P0-001 replaces it with the durable outbox.
mapfile -t legacy_calls < <(
  grep -RInF \
    --include='*.rs' \
    'reconcile_economy(&state.cex' \
    trillionnium/crates 2>/dev/null || true
)

[[ "${#legacy_calls[@]}" == "1" ]] || {
  printf '%s\n' "${legacy_calls[@]:-}" >&2
  fail "expected exactly one registered transaction-held CEX settlement call; found ${#legacy_calls[@]}"
}
[[ "${legacy_calls[0]}" == "$legacy_file:"* ]] || {
  printf '%s\n' "${legacy_calls[0]}" >&2
  fail "registered settlement debt moved outside its reviewed compatibility file"
}

grep -Fq 'External signer, wallet, ledger, custody, webhook or other network I/O is' "$adr" \
  || fail "ADR-0002 no longer states the external-I/O transaction prohibition"
grep -Fq 'runtime_status: foundation-only' "$design" \
  || fail "settlement outbox design must remain honest about runtime integration status"
grep -Fq 'pub const SETTLEMENT_OUTBOX_CONTRACT: &str = "trnm_settlement_outbox_v1"' "$contract" \
  || fail "settlement outbox invariant contract is missing or renamed without migration"

grep -Fq 'WORLD-P0-001' \
  docs/development/trillionnium-world-development-plan-2026-08-27.json \
  || fail "the registered migration debt has no machine-readable P0 work item"

printf '%s\n' \
  'TRNM settlement transaction boundary: amber (1 registered legacy call; expansion blocked pending WORLD-P0-001)'
