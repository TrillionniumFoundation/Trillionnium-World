# Trillionnium World Development Plan v3

Status: **current executable plan**  
Owner: Trillionnium World maintainers  
Effective date: 2026-08-27  
Applies to: `TrillionniumFoundation/Trillionnium-World`  
Supersedes for execution: earlier narrative plans that do not explicitly defer to this document  
Required companion decisions: `PROJECT_BOUNDARY.md`, ADR-0001 realtime authority ownership, the World match-evidence commitment boundary, and ADR-0002 settlement external-I/O boundary.

## 1. Purpose

This plan converts the current audit findings into an ordered implementation program. It is intentionally narrower than a product wish list. Until the P0 program is closed, new gameplay breadth, public-market scope, cross-chain claims, and additional evidence-only scripts are frozen.

A work item is not complete because source code exists. Completion requires an exact commit, remote required checks, bounded rollback, current documentation, and evidence whose limitations are explicit.

## 2. Product and release posture

The repository is a technically advanced local game-product alpha. It contains a deterministic RPG/RTS vertical slice, a native client, an economy protocol, a migration-era World-local online authority, PostgreSQL recovery/fencing, replay and publication evidence, and local deployment material.

The following claims remain distinct:

| Denominator | Current posture | Promotion requirement |
| --- | --- | --- |
| Deterministic runtime alpha | Implemented locally | Exact golden vectors and remote CI |
| Native software alpha | Candidate | Clean package, platform matrix, usability evidence |
| Commercial single-player | Blocked | Distribution, support, accessibility, legal and human evidence |
| Closed online alpha | Migration in progress | Nakama-only external authority and cross-repository compatibility evidence |
| Trusted CEX settlement | Local profile only | Transaction-safe settlement, custody and fault evidence |
| Public online beta | No-go | Security, durability, operations, capacity, endurance and human gates |
| Public player market | Disabled | Separate governance, custody, abuse, legal and economic approval |

No passing test in one denominator implicitly promotes another.

## 3. Target authority model

Exactly one component is accountable for each distributed-system capability.

| Capability | Accountable owner | World responsibility |
| --- | --- | --- |
| Authenticated online match admission | Nakama | Supply selected game rules and validate game-domain inputs |
| Participant roster and online roles | Nakama | Validate role compatibility with a ruleset |
| Global command/event sequence | Nakama | Deterministically execute the ordered command payload |
| Command idempotency and online restart recovery | Nakama | Supply deterministic replayable transitions and golden vectors |
| Canonical event, roster and archive roots | Nakama | Supply game-domain digests and unsigned outcome facts |
| Signed `MatchCompletedV1` evidence | Nakama | Produce unsigned game outcome material only |
| Game rules, authored content and simulation | World | Authoritative owner |
| Game-owned outcome serialization and hash | World | Authoritative owner |
| Wallet/ledger mutation and economic receipts | CEX | Emit typed intents and verify receipts |
| Chain ingress, finality and inclusion proof | Chain | Use a published adapter; make no local finality claim |
| Cross-repository component lock and E2E evidence | Integration | Publish exact World contract/version and fixtures |

The existing World-local Online Authority is a migration-era local-alpha implementation. It may receive safety, correctness, migration and maintainability fixes. It is scope-frozen and may not acquire target Nakama signing, canonical-root, global-ordering or finality responsibilities.

## 4. Non-negotiable invariants

1. One externally visible match has one realtime authority.
2. External DNS, signer, CEX or HTTP work never executes while a mutable PostgreSQL business transaction is open.
3. Retries retain the original durable command or intent identity.
4. Speculative state is not public, settled or release-credit eligible.
5. Terminal state is exposed only after the exact durable publication/evidence boundary.
6. Captured state is applied only when revision, state hash and terminal evidence remain exact.
7. Production execution never silently falls back to a source-tree binary or sibling source checkout.
8. Player, game-authority, signer, moderator and database credentials are independent principals.
9. Protocol compatibility has an owner, an explicit window, negative tests and an expiry condition.
10. Current documentation is version-bound and machine-checkable; archived material cannot become a current gate by linkage accident.
11. Mainline release credit requires protected review and remote required checks for the exact commit.
12. A failed, invalid or interrupted fault/endurance run earns no partial credit.

## 5. Definition of done

Every implementation slice must include:

- a named accountable owner and reviewer;
- an ADR when authority, custody, durable state or compatibility ownership changes;
- code and schema changes with a bounded rollback path;
- positive, negative, concurrency, cancellation and restart tests appropriate to the slice;
- a remote CI run bound to the exact commit;
- updated current-status and operator documentation;
- a machine-readable evidence record when the slice changes a release gate;
- explicit residual limitations and follow-on work;
- no contradictory current document or duplicate authority owner.

Allowed status vocabulary:

- `planned`: design exists, no accepted implementation;
- `implemented`: source and focused tests exist;
- `verified-local`: a reproducible local gate passed;
- `verified-remote`: required remote checks passed for the exact commit;
- `deployed`: a named environment accepted the exact artifact;
- `operational`: monitored service-level evidence exists;
- `release-ready`: all denominator-specific gates passed.

A source change may not jump directly to `operational` or `release-ready`.

## 6. Program structure

### W0 — Governance and truth source (P0)

Goal: make repository scope, plans, ownership and checks enforceable.

Deliverables:

- this plan and the P0 execution backlog are the current execution source;
- ADR-0001 names one authority owner per capability;
- CODEOWNERS covers authority, economy, protocol, deployment and release evidence;
- `main` requires PR review and current World checks;
- active workflows describe the World game-product lane only;
- current/archived documentation is machine-classified;
- release claims are generated from evidence records, not hand-edited percentages.

Exit criteria:

- boundary-negative fixtures fail in CI;
- direct pushes cannot receive release credit;
- every current document has owner, status, review date and applicable contract/release;
- no current document assigns online global order, restart recovery or completion signing to World.

### W1 — Settlement safety (P0)

Goal: eliminate external transport from locked database transactions and make ambiguous outcomes replay-safe.

Required architecture:

1. **Capture transaction** locks and validates the exact pending terminal tuple, reads campaign revision/state hash and immutable pending intent state, then commits.
2. **External execution** runs signer/CEX reconciliation outside every PostgreSQL transaction on a bounded blocking pool while the legacy synchronous backend exists.
3. **Apply transaction** re-locks the exact rows, revalidates terminal evidence plus captured revision/hash, applies reconciled campaign state idempotently and advances settlement only when all members are complete.

Required behaviors:

- stale capture performs no local write;
- remote success followed by stale apply reuses the same intent ID on fresh capture;
- two workers cannot double-apply one match;
- one member may remain pending without holding locks or blocking unrelated matches;
- cancellation and shutdown retain recoverable pending work;
- signer success/CEX ambiguity and CEX success/local rollback are covered by tests;
- observability distinguishes capture, execution, stale apply, retry, ambiguity and final settlement.

Exit criteria:

- source boundary checker proves `reconcile_economy` is absent from transaction-owning settlement code;
- database integration tests prove no external request occurs before capture commit;
- exact-once and ambiguous-commit fault suites pass remotely;
- old transaction-spanning path is removed.

### W2 — Online authority migration (P0/P1)

Goal: move target online authority to Nakama without a dual-authority interval.

Sequence:

1. publish a Bevy-free World deterministic runtime contract;
2. freeze canonical serialization, content/ruleset digest, command input and outcome hash with golden vectors;
3. Integration locks exact World/Nakama revisions and independently verifies fixtures;
4. Nakama consumes the contract and owns admission, roster, global sequence and idempotency;
5. Nakama owns restart recovery and canonical replay/archive roots;
6. Nakama constructs/signs completion evidence;
7. CEX reward authorization consumes Nakama-bound completion evidence through a versioned adapter;
8. new online matches route only through Nakama;
9. old World matches drain or enter explicit quarantine;
10. legacy World authority keys/endpoints are revoked and retired while historical evidence remains verifiable.

A dual-run verifier is allowed only when it is unsigned, cannot settle, cannot publish externally, and cannot become authority after a mismatch.

Exit criteria:

- one externally authoritative owner is demonstrable for every migration phase;
- cross-repository golden vectors and component lock pass;
- no supported client starts a new match on World-local authority;
- old active matches have a documented terminal disposition;
- legacy credentials are revoked and data retention is approved.

### W3 — Runtime modularization (P1)

Goal: decompose the server and client by invariant rather than file size.

Server target boundaries:

```text
trnm-game-server/src/
  http/
  identity/
  authority_legacy/
    actor.rs
    admission.rs
    command_lane.rs
    publication.rs
  persistence/
    connection.rs
    migrations.rs
    command_store.rs
    checkpoint_store.rs
    terminal_store.rs
  journal/
    hot.rs
    cold.rs
    manifest.rs
    recovery.rs
  settlement/
    capture.rs
    execute.rs
    apply.rs
  readiness/
  fleet/
  operations/
```

Each module documents owned state, concurrency model, lock order, durable boundary, retry/idempotency contract and failure policy. Public interfaces are narrow; dependency direction is enforced.

Client work separates domain state machines, application orchestration and Bevy adapters. Bevy systems may render or schedule domain actions but must not own durable semantics.

Exit criteria:

- unrelated state machines no longer share an implicit owner module;
- lock-order/state-transition tests exist;
- module-level API and dependency checks pass;
- behavior/golden vectors remain unchanged unless a versioned contract explicitly changes.

### W4 — Protocol and data contracts (P1)

Deliverables:

- generated OpenAPI for HTTP surfaces;
- generated JSON Schema or equivalent for WebSocket messages;
- stable error codes, reason codes, retry metadata and resync guidance;
- canonical hashing and serialization specification with golden vectors;
- protocol/build/capability compatibility matrix and retirement dates;
- SDK fixtures and contract tests;
- PostgreSQL stored-procedure contracts, permissions, isolation/lock order and query-plan baselines;
- rolling writer/reader and migration checksum policy.

Exact build IDs remain release provenance and admission information; they are not the sole protocol-negotiation mechanism.

### W5 — Deployment and security (P1)

Deliverables:

- self-contained signed release bundle with binaries, assets, migrations, schema and configuration templates;
- no runtime dependency on sibling CEX/Chain/Nakama source checkouts;
- generated systemd units for the selected installation path and dedicated service users;
- production mode rejects source-tree binary fallback;
- separate role credentials with audience, TTL, rotation and revocation;
- mTLS or short-lived workload identity between services;
- pinned Rust/tool/action versions, SBOM, license inventory and provenance;
- reproducible-build comparison in a clean environment;
- KMS/HSM-backed signer before public value credit;
- TLS, WAF/DDoS, rate-limit and abuse evidence before public ingress.

### W6 — Reliability, capacity and evidence (P1)

Evidence families:

- PostgreSQL kill-before-ACK, rollback, PITR, old-primary isolation and ambiguous commit;
- signer/CEX timeout, replay, partial failure and credential rotation;
- journal fsync timeout, corruption, disk full, manifest rollback and retention;
- same-host replacement and, only after replicated durability, cross-host RPO/RTO;
- clean isolated 24-hour active-match endurance;
- production-like latency/concurrency/capacity matrix;
- off-host backup and restore drills;
- Windows/macOS packaging, signing, update and rollback;
- human single-player/multiplayer comprehension and accessibility;
- staffed moderation, appeals, support and commercial/legal drills.

Every evidence record binds commit, tree, artifact hash, toolchain, environment, timestamps, commands/checks, result, limitations and review/expiry date.

### W7 — Product validation (P2)

Product promotion proceeds independently through deterministic alpha, native alpha, commercial single-player candidate, Nakama closed-online alpha, trusted CEX settlement candidate, public online beta and separately approved public market.

## 7. Ordered implementation phases

### Phase 0A — converge truth and enforcement

- land plan v3 and P0 backlog;
- accept/reaffirm ADR-0001;
- add authority-boundary and settlement-boundary required checks;
- classify legacy documents/workflows as archived;
- configure CODEOWNERS and mainline protection;
- establish machine-readable gate/evidence registry.

### Phase 0B — close settlement safety

- review the existing settlement-runtime branch against the three-phase contract;
- add stale-capture, concurrent-worker, cancellation and ambiguous-commit tests;
- make external execution bounded and observable;
- remove old transaction-spanning reconciliation;
- run existing CEX/restart/exact-once gates plus the new fault matrix.

### Phase 1 — publish deterministic runtime contract

- define canonical request/result types independent of Bevy and online authority;
- freeze canonical JSON/hash rules and resource limits;
- publish schema and golden vectors;
- add independent consumer implementation in Integration/Nakama;
- reject participant, global sequence, completion signature and finality fields at the World boundary.

### Phase 2 — migrate Nakama authority

- admission/roster;
- global order/idempotency;
- restart recovery/replay archive;
- canonical roots;
- completion signing;
- CEX and Chain adapters;
- route new matches;
- drain/quarantine legacy World matches;
- revoke/retire old authority.

### Phase 3 — modularize and formalize

- extract settlement first, then persistence/journal, then legacy actor/publication;
- publish HTTP/WS/error/database contracts;
- add compatibility windows and generated SDK fixtures;
- reduce current status to generated projections.

### Phase 4 — harden deployment and evidence

- self-contained installation and role isolation;
- workload identity, artifact signing, SBOM/provenance;
- fault, endurance, capacity, backup, platform and human gates;
- denominator-specific release decisions.

## 8. Pull-request slicing

Large rewrites are prohibited. The expected PR sequence is:

1. **P0 truth and guards** — plan, ADR, CODEOWNERS, active CI and negative fixtures.
2. **Settlement capture model** — pure types, capture query/transaction and focused tests.
3. **Settlement external execution** — bounded execution outside transactions and transport tests.
4. **Settlement apply model** — exact revalidation, stale rejection, concurrency and state transition.
5. **Settlement old-path removal** — no compatibility fallback, full fault suite.
6. **World runtime contract** — canonical types/schema/hash/golden vectors.
7. **Integration reference verifier** — exact component lock and independent hash verification.
8. **Nakama consumer** — authoritative framing without World authority leakage.
9. **Authority migration slices** — one ownership capability per PR.
10. **Legacy drain/retirement** — data disposition, key revocation and endpoint removal.

A PR that changes authority ownership may not also add unrelated product features.

## 9. P0 acceptance matrix

| P0 control | Required proof | Release effect |
| --- | --- | --- |
| Authority ownership | ADR, boundary checker and negative fixture | Prevents dual-authority implementation |
| Settlement transaction split | Source gate, DB integration and fault tests | Required for trusted settlement credit |
| Mainline governance | Protected PR and required checks | Required for remote verification credit |
| Current-document convergence | Metadata/link checker and archive index | Prevents stale truth claims |
| Exact component lock | World/Nakama/Integration commit and fixture binding | Required before closed-online promotion |
| Legacy authority scope freeze | No forbidden keys/roots/signing/finality code | Prevents migration scope expansion |

P0 is closed only when all rows are remotely verified on the integration commit. Partial closure does not lift the feature freeze.

## 10. Risk register

| Risk | Consequence | Mandatory control |
| --- | --- | --- |
| Dual World/Nakama authority | Divergent order, double execution/signing/settlement | ADR, repo guard, one external owner and migration phase gates |
| External I/O under DB locks | Lock convoy, pool starvation and tail collapse | Three-phase settlement and fault tests |
| Remote success/local conflict | Duplicate or lost value | Stable intent IDs, optimistic exact apply and idempotent remote receipts |
| Stale release evidence | False readiness | Exact-commit remote evidence with review/expiry |
| Monolithic state machines | Unsafe changes and weak reviewability | Invariant-based module extraction |
| Shared root credentials | Cross-role compromise | Separate principals, audience, TTL and rotation |
| Source-tree deployment fallback | Unverified production artifact | Production fail-closed release selector |
| Floating tools/actions | Non-reproducible or compromised build | Immutable pinning, SBOM and provenance |
| Historical docs as current truth | Wrong implementation/release decisions | Current/archive metadata and link checks |
| Migration flag becomes permanent | Two architectures indefinitely | Exit date, owner, telemetry and removal PR required for every flag |

## 11. Change control

- Plan changes are reviewed like code.
- Reordering a P0 item requires an explicit risk decision in the PR.
- Changing an authority owner requires a replacement cross-repository ADR, custody plan, migration plan and evidence-retention plan.
- A feature flag that changes authority or settlement behavior must have an owner, default, metrics, rollback boundary and removal condition.
- A release claim changes only through accepted evidence; source statements and local screenshots do not promote a gate.
- Current limitations are first-class output and may not be omitted from generated status.

## 12. Immediate next actions

1. Integrate the plan/boundary and settlement-runtime work on one review branch.
2. Verify the settlement implementation against capture/execute/apply, then add missing concurrency and ambiguity tests.
3. Make authority and settlement boundary checks active on every relevant PR.
4. Create an exact P0 status/evidence registry for the integration commit.
5. Only after P0 checks pass, begin the World deterministic runtime contract and Nakama compatibility work.

Until these actions are remotely verified, public online remains **NO-GO** and the public player market remains **disabled**.
