# Trillionnium World Operations

This manual covers the World game-product repository and its bounded compatibility runtime. It does not describe Chain validators, consensus, PoUW, CEX custody administration or Nakama production operations.

## 1. Preconditions and authority boundary

Read, in order:

1. `PROJECT_BOUNDARY.md` and `PROJECT_BOUNDARY.json`;
2. `CURRENT_PLAN.md`;
3. `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`;
4. `docs/adr/0002-transaction-free-external-settlement.md`;
5. `docs/adr/0003-world-authority-service-boundary.md`;
6. `RELEASE_READINESS.md`.

World owns deterministic game behavior. Nakama owns canonical online admission/order/completion. CEX owns wallet/ledger custody. Chain owns finality. Integration owns exact cross-repository locks. The World-local server remains a compatibility and migration enclave.

## 2. Repository preflight

Run before any edit, build, commit or push:

```bash
bash scripts/project-preflight.sh
```

A mismatch in project ID, lane, remote, branch policy or forbidden path is a stop condition. Do not encode a personal home directory or sibling checkout path in source, documentation, service units or evidence.

## 3. Source validation

From the repository root:

```bash
python3 scripts/check-trnm-world-module-documentation.py
python3 scripts/test-trnm-world-module-documentation-negative.py
python3 scripts/check-trnm-world-documentation.py
python3 scripts/check-trnm-world-transition-conformance.py
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh
```

Then validate Rust:

```bash
cd trillionnium
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

A missing tool, skipped test or empty result is not a pass.

## 4. PostgreSQL test profile

The current exact database profile is PostgreSQL 16.4. Use a disposable database that contains no production or shared development data.

```bash
export TRNM_SETTLEMENT_TEST_DATABASE_URL='postgres://trnm:trnm@127.0.0.1:5432/trnm_test'
export TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST=1
cd trillionnium
cargo test -p trnm-game-server --test settlement_database_contract --locked -- --nocapture
cargo test -p trnm-game-server --test settlement_operator_controls_database --locked -- --nocapture
cargo test -p trnm-game-server --test settlement_fault_model --locked -- --nocapture
cargo test -p trnm-game-server --test settlement_worker_contract --locked -- --nocapture
cargo test -p trnm-game-server --test settlement_runtime_v2_contract --locked -- --nocapture
```

Never point these tests at a production endpoint. Database evidence is accepted only when it binds the exact commit, migration checksums, PostgreSQL image/version and retained logs.

## 5. Compatibility-enclave startup order

The supported laboratory order is:

```text
PostgreSQL
  -> immutable migrations and privilege checks
  -> signer/CEX test doubles or approved development endpoints
  -> settlement worker
  -> World compatibility server
  -> native client
```

Production startup is not authorized by this document. Every process uses a distinct service identity and audience. Development fallbacks must be explicit; a missing production selector fails closed.

## 6. Settlement lifecycle

Settlement follows exactly:

```text
capture transaction
  -> commit immutable capture/jobs
transaction-free remote execution
  -> exact signer lookup/submit
  -> exact CEX lookup/submit
  -> persist remote outcome under live lease
apply transaction
  -> revalidate terminal/campaign/capture/job fences
  -> apply verified receipt with CAS
  -> commit
```

No signer, CEX, wallet, ledger or network operation may execute while mutable match or campaign rows are locked. Timeout, malformed success, conflict and response loss remain ambiguous until exact lookup resolves them.

## 7. Shutdown and drain

On SIGINT/SIGTERM:

1. mark readiness false;
2. stop new player admission, settlement capture and claim;
3. stop accepting new stream connections;
4. drain bounded in-flight work until the configured grace deadline;
5. cancel local work after grace without forging completion;
6. leave durable jobs recoverable by lease expiry/takeover;
7. emit a final bounded shutdown summary.

SIGKILL receives no graceful-drain credit. Recovery must prove journal/database convergence and stale-owner rejection.

## 8. Incident triage

Fail readiness and quarantine the smallest exact scope for:

- migration or catalogue fingerprint drift;
- lost physical-host/instance fence;
- unsupported protocol/build pair;
- hot/cold journal ambiguity or corruption;
- altered duplicate command/intent/receipt;
- uncertain remote settlement success;
- stale lease or old-primary write attempt;
- state/hash/cursor divergence;
- resource budget exhaustion that could violate correctness.

Do not hand-edit lease owners, migration checksums, immutable receipts, terminal ACKs or append-only operator evidence.

## 9. Evidence capture

An accepted record follows `docs/release/trnm-world-evidence-record-v1.md` and includes:

- claim ID and evidence class;
- exact commit/tree/binary/toolchain/component digests;
- database image/version and runtime topology;
- start/end UTC time;
- command and fault injection;
- thresholds and exit code;
- raw artifact hashes;
- limitations, expiry and independent reviewer.

Local source tests cannot close cross-repository, cross-host, public-network, human, custody or commercial rows.

## 10. Rollback and disablement

Rollback is versioned, rehearsed and evidence-bound. It must preserve database compatibility, migration immutability, journal/receipt lineage and single-authority ownership. For online cutover, stop new admission before drain; Nakama remains the only intended canonical admission/order/completion authority. Re-enabling World-local canonical admission is forbidden without a new ADR and independent multi-repository approval.

## 11. Release and deployment stop conditions

Keep the candidate draft and unmerged while any required check is missing/failed/pending, any independent change request remains, or any external denominator lacks evidence. This manual does not authorize merge, tag, release, deployment, public ingress, custody or player-market enablement.
