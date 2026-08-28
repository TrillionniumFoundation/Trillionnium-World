---
status: implemented-pending-exact-commit-ci
owner: trillionnium-world
work_item: WORLD-P0-001
applies_to:
  - trnm_online_settlement_outbox_v1
  - trnm_online_settlement_worker_runtime_v1
  - trusted-cex-settlement-candidate
verified_commit: null
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# TRNM Settlement PostgreSQL Contract v1

## Purpose

`trillionnium/crates/trnm-game-server/tests/settlement_database_contract.rs`
executes the actual settlement migrations against an isolated PostgreSQL
database. It exists to prove database behavior that static source checks cannot
establish.

The test is mandatory in the dedicated `trnm-settlement-fencing` workflow. CI
sets:

```text
TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST=1
TRNM_SETTLEMENT_TEST_DATABASE_URL=postgres://.../trnm_settlement_test
```

Without the require flag, a developer machine that has no isolated database may
skip this one integration test. A promoted workflow may not skip it.

## Database setup

The test drops and recreates the `public` schema in a dedicated disposable
database, builds the minimum prerequisite match/campaign/member/publication-ACK
schema, and then executes the complete contents of:

- `0016_online_settlement_outbox_v1.sql`;
- `0017_online_settlement_worker_runtime_v1.sql`.

The minimum scaffold is intentional. It tests the settlement migration contract
without pretending to be a full online-authority environment. Full migration
compatibility and end-to-end match settlement remain separate evidence rows.

## Current black-box assertions

### Stable remote identity

Three capture generations are inserted for the same match, campaign and intent.
One row deliberately uses a different `intent_hash`.

The test requires all three rows to receive the same `remote_request_id`. This
proves that:

- recapture does not mint a new signer/CEX request identity;
- capture generation is not part of remote idempotency;
- payload fingerprint remains a separate conflict/integrity fence rather than a
  remote identity input.

### Database ownership of identity

The test requires PostgreSQL to reject with SQLSTATE `23514`:

- direct replacement of `remote_request_id`;
- rebinding `intent_id` after insertion;
- a divergent `authorization_request_id`;
- a divergent entitlement nonce.

The source contract additionally freezes match, campaign, intent, capture,
expected campaign fence, queue lane, contract, and payload identity fields.

### Claim-version retirement

Calling `trnm_online_claim_settlement_job_v1` must fail with SQLSTATE `0A000`.
Only the v2 claim path may lease settlement work.

### Live lease and takeover

The test:

1. claims one active job as worker A;
2. expires its lease;
3. proves authorization, attempt start, completion, retry and dead-letter writes
   by worker A no longer mutate the row;
4. reclaims the row as worker B with a strictly higher generation;
5. proves worker A still cannot retry after takeover;
6. proves worker B can begin the attempt and persist remote success.

### Remote success versus application

After receipt persistence, the operator view must report:

```text
remote_state      = remote_succeeded
application_state = pending_apply
```

Only after `campaign_applied_at` is written may the application projection become
`applied`.

### Evidence retention

Deleting the parent match or campaign while settlement rows exist must fail with
SQLSTATE `23503`. Settlement intent, receipt, compensation, retry and dead-letter
history cannot disappear through an upstream cascade.

## What this test does not prove

This database contract does not yet prove:

- the complete historical migration chain from an existing production-like
  database;
- that external signer/CEX transport starts only after capture commit;
- actual signer response-loss or CEX ambiguous-commit convergence;
- exact remote receipt lookup after a lost response;
- full campaign apply/rollback behavior through the production worker;
- process kill, cancellation, shutdown and queue backpressure;
- multi-worker account/campaign ordering under sustained contention;
- operator dashboards, retention approval or replay procedures;
- deployment, custody, KMS/HSM, public online or public player-market readiness.

Those remain explicit open gates in
`docs/status/settlement-runtime-v1.json` and Issue #1.

## Promotion rule

The existence of this test is `implemented` source evidence only. Database
verification requires an exact-head workflow run whose logs bind:

- repository commit and tree;
- Rust and Cargo versions;
- PostgreSQL image/version;
- exact test command and result;
- limitations;
- reviewer signoff.

An absent, skipped, cancelled or stale run grants no credit. Until all
`WORLD-P0-001` gates close, trusted CEX settlement remains blocked and public
player markets remain disabled.
