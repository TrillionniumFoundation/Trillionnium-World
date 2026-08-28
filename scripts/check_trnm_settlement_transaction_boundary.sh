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
recovery="docs/protocol/trnm-settlement-receipt-recovery-v1.md"
runbook="docs/runbooks/trnm-settlement-operations-v1.md"
contract="trillionnium/tools/trnm-settlement-outbox-contract/src/lib.rs"
outbox_migration="trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql"
worker_migration="trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql"
cex_source="trillionnium/crates/trnm-game-server/src/cex.rs"
signer_protocol="trillionnium/crates/trnm-game-server/src/signer_protocol.rs"
signer_binary="trillionnium/crates/trnm-game-server/src/bin/trnm-entitlement-signer.rs"
identity_test="trillionnium/crates/trnm-game-server/tests/settlement_remote_identity_contract.rs"
serialization_test="trillionnium/crates/trnm-game-server/tests/settlement_serialization_database.rs"
workflow=".github/workflows/trnm-settlement-fencing.yml"

for required in \
  "$legacy_file" "$adr" "$design" "$recovery" "$runbook" "$contract" \
  "$outbox_migration" "$worker_migration" "$cex_source" "$signer_protocol" \
  "$signer_binary" "$identity_test" "$serialization_test" "$workflow"; do
  [[ -s "$required" ]] || fail "missing required settlement boundary file: $required"
done

# One compatibility call site remains in lib.rs, but the synchronous CexClient
# implementation is fail-closed and cannot perform transport. No second caller
# may appear, and this final caller remains an explicit P0 deletion gate.
mapfile -t legacy_calls < <(
  grep -RInF --include='*.rs' 'reconcile_economy(&state.cex' \
    trillionnium/crates 2>/dev/null || true
)
[[ "${#legacy_calls[@]}" == "1" ]] || {
  printf '%s\n' "${legacy_calls[@]:-}" >&2
  fail "expected exactly one inert compatibility settlement call; found ${#legacy_calls[@]}"
}
[[ "${legacy_calls[0]}" == "$legacy_file:"* ]] \
  || fail "registered compatibility settlement debt moved outside its reviewed file"

grep -Fq 'External signer, wallet, ledger, custody, webhook or other network I/O is' "$adr" \
  || fail "ADR-0002 no longer states the external-I/O transaction prohibition"
grep -Fq 'runtime_status: integrated-pending-p0-evidence' "$design" \
  || fail "settlement design no longer reports its exact runtime/evidence posture"
grep -Fq 'lookup-before-submit' "$design" \
  || fail "settlement design lost remote ambiguity recovery"
grep -Fq 'account/campaign serialization' "$design" \
  || fail "settlement design lost account/campaign serialization"
grep -Fq 'remote_succeeded' "$design" \
  || fail "settlement design aliases remote success with campaign application"
grep -Fq 'lookup before submit' "$recovery" \
  || fail "receipt recovery protocol lost lookup-before-submit"
grep -Fq 'trnm_cex_settlement_receipt_lookup_v1' "$recovery" \
  || fail "receipt recovery protocol lost CEX binding"
grep -Fq 'trnm_online_settlement_metrics_v1' "$runbook" \
  || fail "settlement runbook lost operator metrics"
grep -Fq 'pub const SETTLEMENT_OUTBOX_CONTRACT: &str = "trnm_settlement_outbox_v1"' "$contract" \
  || fail "settlement outbox invariant contract is missing or renamed without migration"

grep -Fq 'on delete restrict' "$outbox_migration" \
  || fail "settlement evidence is not protected from upstream deletion"
! grep -Fq 'on delete cascade' "$outbox_migration" \
  || fail "settlement evidence must not use upstream cascade deletion"

for marker in \
  'create or replace function public.trnm_online_remote_request_id_v1' \
  'pg_catalog.sha256(' \
  'remote_request_id must be an ordinary stored column' \
  'before update of match_id, campaign_id, intent_id, remote_request_id' \
  'settlement match, campaign and intent identity fields are immutable' \
  'remote_request_id does not match durable settlement identity' \
  'entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id)' \
  'p_authorization_request_id = remote_request_id' \
  'create or replace function public.trnm_online_settlement_serialization_key_v1' \
  'pg_catalog.pg_try_advisory_xact_lock' \
  'pg_catalog.hashtextextended' \
  'create or replace view public.trnm_online_settlement_job_status_v1' \
  'create or replace view public.trnm_online_settlement_metrics_v1' \
  "when job.state = 'succeeded' then 'pending_apply'" \
  'oldest_eligible_age' \
  'oldest_pending_apply_age'; do
  grep -Fq "$marker" "$worker_migration" \
    || fail "worker migration lost settlement marker: $marker"
done

lease_expiry_fences="$(grep -Fc 'lease_expires_at > pg_catalog.clock_timestamp()' "$worker_migration")"
[[ "$lease_expiry_fences" -ge 5 ]] \
  || fail "expected live-lease fences on authorization/attempt/complete/retry/dead-letter; found $lease_expiry_fences"

for marker in \
  'async fn lookup_signer_receipt' \
  'async fn lookup_authorized_settlement_receipt' \
  'CEX_SETTLEMENT_RECEIPT_LOOKUP_PATH' \
  'signer_response_loss_recovers_by_lookup_without_a_second_sign' \
  'cex_response_loss_recovers_by_lookup_without_a_second_submit' \
  'cex_lookup_with_a_mismatched_hash_fails_closed' \
  'Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())'; do
  grep -Fq "$marker" "$cex_source" \
    || fail "CEX/signer client lost recovery marker: $marker"
done

grep -Fq 'ENTITLEMENT_SIGNER_RECEIPT_PATH: &str = "/v1/signer/receipts"' "$signer_protocol" \
  || fail "signer protocol lost receipt lookup path"
grep -Fq '"/v1/signer/receipts/:request_id"' "$signer_binary" \
  || fail "signer service lost receipt lookup route"
grep -Fq 'entitlement.nonce != request.request_id' "$signer_binary" \
  || fail "signer request identity is not bound through the entitlement nonce"
! grep -Fq 'entitlement.intent_id != request.request_id' "$signer_binary" \
  || fail "signer transport identity is incorrectly aliased to economic intent identity"

grep -Fq 'account_serialization_does_not_block_unrelated_work' "$serialization_test" \
  || fail "PostgreSQL serialization test is missing"
grep -Fq "TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST: '1'" "$workflow" \
  || fail "PostgreSQL settlement tests are optional in CI"
grep -Fq -- '--test settlement_serialization_database' "$workflow" \
  || fail "workflow does not run account serialization database test"
grep -Fq -- '--bin trnm-entitlement-signer' "$workflow" \
  || fail "workflow does not test signer lookup route"

grep -Fq 'WORLD-P0-001' \
  docs/development/trillionnium-world-development-plan-2026-08-27.json \
  || fail "the registered migration debt has no machine-readable P0 work item"

printf '%s\n' \
  'TRNM settlement transaction boundary: amber (runtime split, stable identity, lookup-before-submit, account serialization, PostgreSQL tests and operator metrics implemented; legacy caller, CEX owner endpoint and deployed fault evidence remain open)'
