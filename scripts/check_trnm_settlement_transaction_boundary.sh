#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

fail() {
  printf 'settlement-transaction-boundary: %s\n' "$*" >&2
  exit 1
}

entry="trillionnium/crates/trnm-game-server/src/lib.rs"
template="trillionnium/crates/trnm-game-server/src/lib.rs.in"
worker_entry="trillionnium/crates/trnm-game-server/src/settlement_worker.rs"
worker_template="trillionnium/crates/trnm-game-server/src/settlement_worker.rs.in"
build="trillionnium/crates/trnm-game-server/build.rs"
adr="docs/adr/0002-transaction-free-external-settlement.md"
design="docs/development/trnm-settlement-outbox-v1.md"
recovery="docs/protocol/trnm-settlement-receipt-recovery-v1.md"
runbook="docs/runbooks/trnm-settlement-operations-v1.md"
status="docs/status/settlement-runtime-v1.json"
contract="trillionnium/tools/trnm-settlement-outbox-contract/src/lib.rs"
outbox_migration="trillionnium/crates/trnm-game-server/migrations/0016_online_settlement_outbox_v1.sql"
worker_migration="trillionnium/crates/trnm-game-server/migrations/0017_online_settlement_worker_runtime_v1.sql"
operator_migration="trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql"
cex_source="trillionnium/crates/trnm-game-server/src/cex.rs"
signer_protocol="trillionnium/crates/trnm-game-server/src/signer_protocol.rs"
signer_binary="trillionnium/crates/trnm-game-server/src/bin/trnm-entitlement-signer.rs"
identity_test="trillionnium/crates/trnm-game-server/tests/settlement_remote_identity_contract.rs"
serialization_test="trillionnium/crates/trnm-game-server/tests/settlement_serialization_database.rs"
capture_test="trillionnium/crates/trnm-game-server/tests/settlement_capture_commit_boundary.rs"
operator_test="trillionnium/crates/trnm-game-server/tests/settlement_operator_controls_database.rs"
boundary_test="trillionnium/crates/trnm-game-server/tests/settlement_game_server_boundary.rs"
workflow=".github/workflows/trnm-settlement-fencing.yml"

for required in \
  "$entry" "$template" "$worker_entry" "$worker_template" "$build" \
  "$adr" "$design" "$recovery" "$runbook" "$status" "$contract" \
  "$outbox_migration" "$worker_migration" "$operator_migration" \
  "$cex_source" "$signer_protocol" "$signer_binary" "$identity_test" \
  "$serialization_test" "$capture_test" "$operator_test" "$boundary_test" \
  "$workflow"; do
  [[ -s "$required" ]] || fail "missing required settlement boundary file: $required"
done

# The compiled entrypoints are generated from reviewed templates. The template
# may retain exactly one source transformation anchor, but the compiled source
# must contain no synchronous CEX settlement call.
grep -Fq 'trnm_game_server_lib_generated.rs' "$entry" \
  || fail "game-server entrypoint no longer includes generated runtime source"
grep -Fq 'trnm_settlement_worker_generated.rs' "$worker_entry" \
  || fail "settlement-worker entrypoint no longer includes generated runtime source"
! grep -Fq 'reconcile_economy(&state.cex' "$entry" \
  || fail "compiled game-server entrypoint reintroduced synchronous settlement"
[[ "$(grep -Fc 'reconcile_economy(&state.cex' "$template")" == "1" ]] \
  || fail "reviewed game-server template must retain exactly one fail-closed transform anchor"
for marker in \
  'source.contains("reconcile_economy(&state.cex")' \
  'source.contains("settle_pending_matches(&settlement_state")' \
  'WORLD-P0-001 source transform failed closed' \
  'terminal settlement is owned by trnm-settlement-worker' \
  '0018_online_settlement_operator_controls_v1'; do
  grep -Fq "$marker" "$build" \
    || fail "generated runtime transform lost marker: $marker"
done

# Run the structural parser, which scans both .rs and .rs.in sources and proves
# no remote transport occurs in a transaction-owning function.
bash scripts/check-trnm-settlement-transaction-boundary.sh

grep -Fq 'External signer, wallet, ledger, custody, webhook or other network I/O is' "$adr" \
  || fail "ADR-0002 no longer states the external-I/O transaction prohibition"
grep -Fq 'runtime_status: integrated-pending-p0-evidence' "$design" \
  || fail "settlement design no longer reports its bounded runtime posture"
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
  || fail "settlement outbox invariant contract is missing or renamed"

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
  'create or replace view public.trnm_online_settlement_job_status_v1' \
  'create or replace view public.trnm_online_settlement_metrics_v1' \
  "when job.state = 'succeeded' then 'pending_apply'" \
  'oldest_eligible_age' \
  'oldest_pending_apply_age'; do
  grep -Fq "$marker" "$worker_migration" \
    || fail "worker migration lost settlement marker: $marker"
done

for marker in \
  'trnm_online_settlement_operator_replay' \
  'trnm_online_settlement_operator_replay_requests' \
  'before update or delete' \
  'before truncate' \
  'remote_attempts' \
  'retention'; do
  grep -Fiq "$marker" "$operator_migration" \
    || fail "operator migration lost replay/append-only marker: $marker"
done

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
  || fail "signer request identity is not bound through entitlement nonce"
! grep -Fq 'entitlement.intent_id != request.request_id' "$signer_binary" \
  || fail "signer transport identity is aliased to economic intent identity"

grep -Fq 'account_serialization_does_not_block_unrelated_work' "$serialization_test" \
  || fail "PostgreSQL serialization test is missing"
grep -Fq 'settlement_operator_replay_is_exact_audited_one_attempt_and_append_only' "$operator_test" \
  || fail "PostgreSQL operator replay test is missing"
grep -Fq 'GENERATED_GAME_SERVER_SOURCE' "$boundary_test" \
  || fail "generated game-server ownership test is missing"
grep -Fq "TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST: '1'" "$workflow" \
  || fail "PostgreSQL settlement tests are optional in CI"
for target in \
  'settlement_game_server_boundary' \
  'settlement_serialization_database' \
  'settlement_operator_controls_database'; do
  grep -Fq -- "--test $target" "$workflow" \
    || fail "workflow does not run $target"
done
grep -Fq -- '--bin trnm-entitlement-signer' "$workflow" \
  || fail "workflow does not test signer lookup route"
grep -Fq 'cargo clippy -p trnm-game-server --all-targets --locked -- -D warnings' "$workflow" \
  || fail "workflow lost complete game-server lint"

python3 - <<'PY'
import json
from pathlib import Path
status = json.loads(Path('docs/status/settlement-runtime-v1.json').read_text())
required = {
    'merge_cex_owner_repository_pull_request',
    'bind_exact_cex_build_and_deployment_artifact',
    'prove_deployed_signer_and_cex_response_loss_recovery',
    'prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix',
    'approve_backup_pitr_restore_and_receipt_retention',
    'obtain_exact_commit_github_actions_evidence',
    'obtain_reviewer_signoff',
}
if set(status['open_gates']) != required:
    raise SystemExit('settlement status hid or invented final external blockers')
if status['public_online'] != 'no_go' or status['public_player_market'] != 'disabled':
    raise SystemExit('settlement status overclaimed public release')
PY

printf '%s\n' \
  'TRNM settlement transaction boundary: PASS (compiled synchronous caller removed; generated source, worker phases, identity, lease, replay and evidence controls are fail-closed; external deployment/governance gates remain explicit)'
