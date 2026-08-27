# Trillionnium World P0 Execution Backlog v1

Status: **current**  
Owner: World maintainers  
Plan: `trnm-world-development-plan-v3.md`  
Scope: P0 only; source implementation does not imply deployment or release readiness.

## Operating rules

- Execute in the order below unless a PR records an explicit risk decision.
- Every item has one accountable owner, one reviewer, an exact branch/commit and an acceptance command.
- A checkbox is closed only after remote required checks pass for the exact integration commit.
- Any failed or skipped negative/fault test keeps the item open.
- Authority changes require cross-repository review; settlement changes require economy and persistence review.

## P0-01 — authority decision and scope freeze

**Goal:** World cannot become a second Nakama authority by implementation drift.

Tasks:

- [ ] Reaffirm ADR-0001 with World/Nakama/Chain/CEX/Integration owners.
- [ ] Classify the current World Online Authority as `legacy_local_alpha` in current documentation and readiness/status surfaces.
- [ ] Add a production-code boundary checker for Nakama private keys, World-owned signed completion evidence, competing canonical roster/event/archive roots and local Chain-finality claims.
- [ ] Add negative fixtures for every forbidden category.
- [ ] Ensure active Cargo manifests have no sibling Chain/Nakama filesystem path dependency.
- [ ] Add CODEOWNERS for authority/protocol/economy/deployment/evidence paths.

Acceptance:

```bash
scripts/check-trnm-world-authority-boundary.sh
scripts/test-trnm-world-authority-boundary-negative.sh
```

Evidence: remote workflow URL, exact commit/tree and checker output.

## P0-02 — settlement capture transaction

**Goal:** capture exact settlement work without external transport.

Tasks:

- [ ] Define `SettlementCapture`, `CapturedCampaign` and typed capture outcome.
- [ ] Select pending terminal matches with bounded `FOR UPDATE SKIP LOCKED` behavior.
- [ ] Verify exact acknowledged/cold-sealed terminal publication before capture.
- [ ] Capture campaign ID, DB revision, stored state hash, serialized campaign and immutable intent identity.
- [ ] Commit before any signer/CEX call.
- [ ] Unit-test negative revision/range/shape cases.
- [ ] Integration-test that an external probe receives no request before capture commit.

Acceptance:

- capture transaction has a bounded statement/lock timeout;
- capture performs no network/DNS/blocking work;
- capture returns no write authority after commit;
- malformed or quarantined progression fails closed.

## P0-03 — external settlement execution

**Goal:** execute legacy synchronous economy reconciliation outside PostgreSQL transactions.

Tasks:

- [ ] Execute on a bounded `spawn_blocking`/dedicated worker lane while `EconomyBackend` remains synchronous.
- [ ] Preserve original economic intent IDs and authoritative metadata.
- [ ] Bound concurrency, queue depth, operation timeout and shutdown behavior.
- [ ] Classify retryable transport, ambiguous commit, typed rejection and dead-letter outcomes.
- [ ] Add structured metrics/logging without credentials or player-session tokens.
- [ ] Test signer timeout, signer success/CEX failure, CEX commit/response loss and cancellation.

Acceptance:

- no SQL transaction object enters the execution function;
- the worker queue is bounded;
- retry reuses the same request/intent identity;
- cancellation leaves durable pending work.

## P0-04 — settlement apply transaction

**Goal:** apply only an exact current capture and settle once.

Tasks:

- [ ] Re-lock exact match and all member campaigns in deterministic order.
- [ ] Revalidate terminal publication marker and cold seal.
- [ ] Compare captured campaign revision and state hash before any write.
- [ ] Reject stale capture atomically with zero campaign/marker/match writes.
- [ ] Persist every accepted reconciled campaign in one short transaction.
- [ ] Advance ACK and match settlement from `pending` to `settled` only when every member is complete.
- [ ] Test concurrent workers, campaign mutation during external execution and apply rollback after remote success.

Acceptance:

- two workers cannot double-apply;
- stale apply is observable and safely recaptured;
- partial member completion remains pending;
- remote success plus local conflict never duplicates value.

## P0-05 — remove legacy transaction-spanning path

**Goal:** one settlement implementation and one transaction boundary.

Tasks:

- [ ] Delete direct `reconcile_economy(&state.cex, ...)` calls from transaction-owning settlement code.
- [ ] Delete temporary compatibility flag after fault gates pass.
- [ ] Add source checker preventing regression.
- [ ] Run existing CEX exact-once/restart gates and new failure matrix.

Acceptance:

```bash
scripts/check-trnm-settlement-transaction-boundary.sh
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

## P0-06 — active World CI convergence

**Goal:** current checks match the current game-product lane.

Tasks:

- [ ] Run product-boundary, authority-boundary, settlement-boundary and document checks first.
- [ ] Run format, all-target tests, strict Clippy, audit, deny and package verification.
- [ ] Preserve reports and exact release artifacts.
- [ ] Move legacy L1/validator workflows under an archive index or disable their active triggers.
- [ ] Pin third-party Actions to immutable SHAs.
- [ ] Add negative-fixture jobs that must fail for the intended reason.

Acceptance: all required checks are visible on the integration PR and cannot be bypassed for release credit.

## P0-07 — mainline review governance

**Goal:** source and documentation claims require remote review.

Tasks:

- [ ] Protect `main` against deletion and force push.
- [ ] Require pull requests, one approving review and code-owner review.
- [ ] Require current World P0/CI checks and conversation resolution.
- [ ] Define emergency bypass principals and mandatory incident record.
- [ ] Verify branch protection through the GitHub API and record the result.

If repository-plan restrictions prevent a server-side rule, the item remains open; CODEOWNERS and workflow checks do not substitute for enforcement.

## P0-08 — current documentation convergence

**Goal:** one current implementation/release truth source.

Tasks:

- [ ] Put current plan, status, ADR, protocols, operations and release evidence first in `docs/README.md`.
- [ ] Add status/owner/applicability/review metadata to every current document.
- [ ] Archive legacy Chain/Web4/validator and superseded World plans.
- [ ] Add a current-document link/metadata checker.
- [ ] Generate `GAME_STATUS.md` or reduce it to a concise projection of machine-readable records.

Acceptance: active docs contain no unresolved links or contradictory authority owner.

## P0-09 — machine-readable status and evidence

**Goal:** prevent prose from widening product claims.

Tasks:

- [ ] Define gate/evidence JSON Schema.
- [ ] Register exact commit, tree, artifact, toolchain, environment, checks, result, limitations and review date.
- [ ] Generate current status from accepted records.
- [ ] Reject `public_online=ready`, `public_market=enabled` or equivalent without the complete denominator-specific evidence set.
- [ ] Distinguish local, remote, deployed and operational evidence.

Acceptance: a tampered status fixture fails CI and public online remains `no_go` until all required evidence is accepted.

## P0-10 — deterministic runtime contract preparation

**Goal:** create the first migration artifact without moving online authority into World.

Tasks:

- [ ] Define Bevy-free request/result/outcome types.
- [ ] Specify canonical serialization, integer/resource/depth limits and hashes.
- [ ] Publish JSON Schema and golden vectors.
- [ ] Prohibit participant roster, global sequence, canonical roots, signature and finality fields.
- [ ] Add independent reference verification in Integration.
- [ ] Add Nakama consumer contract tests bound to exact revisions.

This item may start only after P0-01 through P0-06 pass on the same integration line.

## Integration exit decision

The P0 integration PR may become ready for main only when:

- P0-01 through P0-06 are remotely verified;
- P0-07 is enforced or explicitly remains a documented repository-level blocker;
- P0-08 and P0-09 prevent stale release claims;
- the settlement old path is absent;
- residual limitations include no public ingress, no public market, no cross-host RPO=0 and no production custody claim.

Merging P0 does not authorize public online launch. It only establishes a safe development baseline for the Nakama migration.
