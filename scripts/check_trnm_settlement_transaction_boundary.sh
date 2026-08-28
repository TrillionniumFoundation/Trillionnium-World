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
outbox_migration="trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql"
worker_migration="trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql"
identity_test="trillionnium/crates/trnm-game-server/tests/settlement_remote_identity_contract.rs"

for required in \
  "$legacy_file" \
  "$adr" \
  "$design" \
  "$contract" \
  "$outbox_migration" \
  "$worker_migration" \
  "$identity_test"; do
  [[ -s "$required" ]] || fail "missing required settlement boundary file: $required"
done

# One compatibility call site remains in lib.rs, but the synchronous CexClient
# implementation is fail-closed and cannot perform transport. The dedicated
# worker owns external settlement. This debt must still be deleted before P0
# closure, and no second compatibility caller may appear.
mapfile -t legacy_calls < <(
  grep -RInF \
    --include='*.rs' \
    'reconcile_economy(&state.cex' \
    trillionnium/crates 2>/dev/null || true
)

[[ "${#legacy_calls[@]}" == "1" ]] || {
  printf '%s\n' "${legacy_calls[@]:-}" >&2
  fail "expected exactly one inert compatibility settlement call; found ${#legacy_calls[@]}"
}
[[ "${legacy_calls[0]}" == "$legacy_file:"* ]] || {
  printf '%s\n' "${legacy_calls[0]}" >&2
  fail "registered compatibility settlement debt moved outside its reviewed file"
}

grep -Fq 'External signer, wallet, ledger, custody, webhook or other network I/O is' "$adr" \
  || fail "ADR-0002 no longer states the external-I/O transaction prohibition"
grep -Fq 'runtime_status: integrated-pending-p0-evidence' "$design" \
  || fail "settlement design no longer reports its exact runtime/evidence posture"
grep -Fq 'remote_request_id' "$design" \
  || fail "settlement design lost the stable remote identity contract"
grep -Fq 'remote_succeeded' "$design" \
  || fail "settlement design aliases remote success with campaign application"
grep -Fq 'pub const SETTLEMENT_OUTBOX_CONTRACT: &str = "trnm_settlement_outbox_v1"' "$contract" \
  || fail "settlement outbox invariant contract is missing or renamed without migration"

grep -Fq 'on delete restrict' "$outbox_migration" \
  || fail "settlement evidence is not protected from upstream deletion"
! grep -Fq 'on delete cascade' "$outbox_migration" \
  || fail "settlement evidence must not use upstream cascade deletion"

grep -Fq "set remote_request_id = 'trnm-settlement-remote-v1:'" "$worker_migration" \
  || fail "worker migration lost stable remote request identity"
grep -Fq 'entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id)' "$worker_migration" \
  || fail "entitlement nonce no longer uses stable remote identity"
grep -Fq 'p_authorization_request_id = remote_request_id' "$worker_migration" \
  || fail "authorization persistence no longer binds the stable remote identity"
grep -Fq 'create or replace view public.trnm_online_settlement_job_status_v1' "$worker_migration" \
  || fail "settlement operator projection is missing"
grep -Fq "when job.state = 'succeeded' then 'pending_apply'" "$worker_migration" \
  || fail "remote success is being presented as completed campaign application"

lease_expiry_fences="$(grep -Fc 'lease_expires_at > pg_catalog.clock_timestamp()' "$worker_migration")"
[[ "$lease_expiry_fences" -ge 5 ]] \
  || fail "expected live-lease fences on authorization/attempt/complete/retry/dead-letter; found $lease_expiry_fences"

grep -Fq 'WORLD-P0-001' \
  docs/development/trillionnium-world-development-plan-2026-08-27.json \
  || fail "the registered migration debt has no machine-readable P0 work item"

printf '%s\n' \
  'TRNM settlement transaction boundary: amber (runtime split, stable remote identity, live-lease fencing and evidence retention implemented; legacy caller and remote fault evidence remain open)'
