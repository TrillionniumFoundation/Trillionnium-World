---
status: current
owner: trillionnium-world
verified_commit: pending-merge
applies_to:
  - game-product
  - native-client
  - deterministic-simulation
  - world-to-nakama-integration
supersedes:
  - ad-hoc-online-authority-expansion
last_reviewed: 2026-08-27
review_due: 2026-09-10
---

# Trillionnium World Development Plan — 2026-08-27

Machine-readable execution manifest:
`trillionnium-world-development-plan-2026-08-27.json`.

## 1. Executive decision

Trillionnium World will stop evolving as a second canonical online-match
authority.

The target architecture is:

- **World** owns authored content, deterministic rules and simulation, game
  outcome facts, unsigned replay material, the native client, and player-facing
  economy intents.
- **Nakama** owns online participant admission, canonical global event order,
  match version, command idempotency, restart recovery, canonical archive roots,
  `MatchCompletedV1`, and the match-evidence signing key.
- **Chain** owns ingress, consensus, finality, inclusion proofs, and research
  command semantics.
- **CEX** owns wallet/ledger settlement and custody.
- **Integration** owns cross-repository component locks, compatibility matrices,
  and release evidence that spans those systems.

The existing World-local `trnm-game-server` remains temporarily available as a
**compatibility authority enclave**. It may support deterministic laboratory,
rollback, and migration evidence. It is not the destination architecture and
must not acquire new canonical public-authority responsibilities.

This decision resolves the previous contradiction where World and Nakama could
both claim ordering, idempotency, restart recovery, and completion evidence.
The authoritative record is ADR-0001.

## 2. Current product posture

- Native software and deterministic RPG/RTS work remain a technical alpha.
- Commercial single-player remains gated by external usability, packaging,
  accessibility, support, and distribution evidence.
- Local trusted-system-market CEX work remains a bounded single-host profile.
- Public player markets and public online RPG/RTS remain NO-GO.
- Existing local latency, journal, replay, and recovery evidence is useful
  migration evidence but does not establish Nakama authority, public network
  operation, cross-host RPO=0, Chain finality, or commercial readiness.

## 3. Non-negotiable invariants

### 3.1 Authority

1. Exactly one system is accountable for each canonical cursor, root, receipt,
   or signature.
2. World must never load or derive the Nakama match-authority private key.
3. World may compute a versioned game outcome hash, but that hash alone does not
   prove participant admission, event order, archive completeness, or finality.
4. A World artifact must never be described as Chain-finalized without an
   AppHash/finality-bound Chain receipt.
5. Existing World-local authority code is grandfathered only inside its named
   compatibility enclave; no second public authority protocol may be added.

### 3.2 Settlement

1. No external HTTP, signer, wallet, or ledger operation may execute while a
   PostgreSQL transaction holds mutable match or campaign row locks.
2. Every settlement attempt has a deterministic job identity and immutable
   intent fingerprint.
3. Signer success, ledger success, timeout, retry, process death, and ambiguous
   commit must all converge through one idempotent outbox state machine.
4. A receipt may update game progression only when it binds the exact job,
   intent, campaign revision, account, amount, and authoritative result facts.
5. Retryable failure never becomes silent success; exhausted attempts become an
   explicit dead letter with operator-visible evidence.

### 3.3 Runtime and release

1. Production units contain no developer home directory or machine-specific
   checkout path.
2. Production scripts do not source a sibling repository.
3. Game authority, moderator, and entitlement-signer credentials are distinct.
4. Production requires a verified release selector. Development-binary fallback
   is allowed only with an explicit opt-in environment flag.
5. Current documentation, schemas, source, binaries, and evidence identify the
   exact commit or component revision they apply to.

### 3.4 Engineering

1. Correctness-critical state machines expose narrow module boundaries and
   explicit invariants.
2. New wire contracts have schemas, stable error codes, compatibility windows,
   and golden vectors.
3. Every release claim distinguishes source/unit evidence, local black-box
   evidence, human evidence, public-network evidence, and commercial approval.
4. Historical documents cannot silently act as current truth sources.

## 4. Target system flow

```text
Native Client
    |
    | player intent / game command
    v
Nakama canonical match authority
    |  - admission
    |  - event total order
    |  - command idempotency
    |  - restart recovery
    |  - archive roots
    |
    | versioned World ruleset request
    v
World deterministic rules/simulation
    |  - validates game-domain command
    |  - advances deterministic state
    |  - emits outcome/replay material
    |  - computes World outcome hash
    v
Nakama MatchCompletedV1 + signature
    |
    +-----------------> Integration evidence lock
    |
    +-----------------> Chain canonical ingress/finality
    |
    +-----------------> CEX entitlement/ledger settlement
```

World-to-Nakama communication must use an immutable published contract,
generated schema, or exact-revision package. A sibling filesystem dependency is
not an integration contract.

## 5. Migration strategy for the current World-local authority

The migration is staged to preserve existing correctness evidence without
creating an unsafe flag day.

### Stage A — freeze and describe

- Mark `trnm-game-server` as a compatibility enclave.
- Prohibit new canonical authority claims and match-completion signing.
- Establish the World/Nakama/Chain/CEX/Integration responsibility matrix.
- Add static boundary checks to CI.
- Retain existing tests as regression evidence for deterministic simulation and
  migration behavior.

### Stage B — extract deterministic World contract

Create a versioned World rules contract that contains only:

- ruleset/content revision;
- deterministic command payload;
- initial state or state reference;
- deterministic transition result;
- game outcome and replay material;
- World outcome hash and canonical serialization version;
- domain validation errors.

It must not contain Nakama session tokens, authority private keys, canonical
archive roots, Chain credentials, or finality claims.

### Stage C — Nakama adapter and shadow verification

- Implement the adapter in the repository that owns the runtime side of the
  contract.
- Run World-local and Nakama-driven paths in shadow mode against identical
  fixtures.
- Compare state hashes, outcomes, replay material, rejection reasons, and
  resource budgets.
- Reject promotion on any unexplained divergence.
- Keep World-local output unsigned and noncanonical during shadowing.

### Stage D — authority cutover

- New online matches are admitted only by Nakama.
- Active World-local matches are drained; no cross-generation live takeover is
  claimed unless a specific recovery matrix passes.
- Nakama becomes the only producer of canonical completion evidence.
- World-local public endpoints are disabled or retained only behind an explicit
  laboratory profile.

### Stage E — enclave retirement

- Remove duplicated admission, ordering, idempotency, archive-root, and
  completion-signing responsibilities from World.
- Retain deterministic simulation and migration readers only where needed.
- Archive superseded protocols and runbooks.

## 6. Workstreams

## W0 — Governance and truth sources

**Goal:** make the target architecture and active plan impossible to confuse
with historical material.

Deliverables:

- ADR-0001 authority ownership;
- ADR-0002 external-settlement transaction boundary;
- schema-v2 `PROJECT_BOUNDARY.json`;
- current documentation index;
- CI boundary checks;
- issue-backed execution backlog;
- branch protection and required checks at repository settings level.

Acceptance:

- current docs name one accountable authority for every canonical artifact;
- a CI test fails when World source constructs `MatchCompletedV1` or introduces
  a Nakama private-key surface;
- no current index points to legacy Chain material as the World architecture;
- `main` cannot merge without current game and architecture checks.

## W1 — World-to-Nakama authority contract

**Goal:** expose deterministic game behavior without duplicating online
runtime authority.

Deliverables:

- contract package and JSON/Protobuf schemas;
- canonical serialization and hash vectors;
- version negotiation and deprecation policy;
- deterministic transition API;
- replay/outcome material API;
- typed error catalogue;
- adapter conformance tests;
- shadow-diff runner;
- cutover and rollback runbook.

Acceptance:

- World and Nakama independently reproduce the same transition/outcome hashes;
- malformed or unknown ruleset revisions fail closed;
- no private authority material crosses the interface;
- compatibility is tied to a published contract revision, not a sibling path.

## W2 — Transaction-free external settlement

**Goal:** remove signer/CEX network calls from locked PostgreSQL transactions.

Target state machine:

```text
Pending -> Leased -> Succeeded
             |
             +-> Retryable -> Leased
             +-> DeadLetter

Expired Leased -> Leased by a new generation
```

Deliverables:

- deterministic settlement job identity;
- immutable intent hash and expected campaign revision;
- durable lease owner/generation/expiry;
- transaction-one claim and transaction-two result application;
- async signer/CEX worker outside database transactions;
- exact receipt binding;
- retry/backoff/dead-letter policy;
- ambiguous-commit recovery;
- per-account/campaign serialization;
- backlog, age, retry, lease-expiry, and dead-letter telemetry;
- fault-injection matrix.

Acceptance:

- no external network call occurs while a mutable campaign/match transaction is
  open;
- process death at every transition converges without double payout;
- a stale lease owner cannot apply a result after lease takeover;
- exact duplicate receipts are idempotent and mismatched receipts fail closed;
- one poisoned job cannot block unrelated compensation or settlement jobs.

## W3 — Correctness-oriented module decomposition

**Goal:** reduce change blast radius in `trnm-game-server` and the native client.

Server target modules:

```text
http/
identity/
authority/
  actor.rs
  admission.rs
  command_lane.rs
  publication.rs
persistence/
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
readiness/
fleet/
operations/
```

Client target modules:

```text
campaign/
ui/
online/
  transport.rs
  journal.rs
  reconciliation.rs
render/
simulation_adapter/
```

Each module must document:

- owned state;
- concurrency model;
- lock order;
- durable/public boundary;
- retry and idempotency contract;
- fail-open/fail-closed behavior;
- unit and black-box evidence.

Acceptance:

- no correctness-critical source file remains a catch-all for unrelated HTTP,
  actor, persistence, journal, settlement, and operations behavior;
- circular dependencies are absent;
- module APIs are tested through invariant-focused tests rather than only happy
  path endpoint tests.

## W4 — Protocol and data contracts

**Goal:** replace implicit Rust/SQL coupling with reviewable versioned contracts.

Deliverables:

- OpenAPI for HTTP surfaces;
- schema for WebSocket messages;
- stable machine-readable error codes;
- compatibility and retirement matrix;
- canonical hashing specification;
- database ER diagram;
- stored-procedure API catalogue;
- transaction isolation and lock-order specification;
- rolling migration and old-writer matrix;
- schema conformance and golden-vector tests.

Acceptance:

- serialized examples round-trip through implementation and schema validators;
- protocol version and release provenance are distinct concepts;
- unknown enum/error behavior is defined;
- every PL/pgSQL procedure states preconditions, postconditions, privileges,
  result codes, retry behavior, and supported PostgreSQL versions.

## W5 — Runtime, secrets, and supply chain

**Goal:** create a portable, least-privilege, reproducible deployment.

Deliverables:

- rendered systemd units without committed personal paths;
- explicit environment files and examples;
- distinct role credentials and rotation runbooks;
- verified-release-only production startup;
- explicit development fallback;
- no sibling CEX source dependency;
- pinned Rust/toolchain and GitHub Actions revisions;
- SBOM, licence report, provenance and release signatures;
- KMS/HSM integration plan;
- mTLS/workload identity plan for service-to-service calls.

Acceptance:

- a clean host can install from a release bundle without another repository
  checkout;
- leaked moderator credentials cannot authorize settlement or signing;
- a missing or dangling production release selector fails closed;
- build inputs and produced binaries are reproducibly identified.

## W6 — Reliability, capacity, and release evidence

**Goal:** turn local evidence scripts into an auditable release matrix.

Required matrices:

- command submit/commit/publication crash points;
- signer success + response loss;
- CEX commit + response loss;
- PostgreSQL kill-before-ACK and ambiguous commit;
- rollback/PITR/timeline change and old-primary isolation;
- journal hot/cold/manifest corruption;
- v2/v3 or compatibility-window matrix while advertised;
- Nakama adapter shadow divergence;
- clean 24-hour active-match endurance;
- multi-client concurrency and settlement backlog;
- public-edge TLS/WAF/DDoS and regional capacity;
- Windows/macOS/Linux packaging and signing.

Every evidence record must identify:

- claim ID and scope;
- commit/tree/binary/toolchain digests;
- environment and topology;
- start/end timestamps;
- metrics and thresholds;
- raw artifact locations and hashes;
- limitations and expiry/review date;
- reviewer/signoff.

Acceptance:

- generated status pages derive from machine-readable evidence;
- local source tests cannot satisfy human, public-network, cross-host, or
  commercial rows;
- invalid, partial, stale, or environment-unbound evidence fails closed.

## W7 — Product and human validation

**Goal:** validate a coherent player experience rather than only technical
reachability.

Deliverables:

- 10–15 minute NEW -> RPG -> RTS -> debrief -> town vertical slice;
- three independent five-second-observer sessions;
- non-developer unguided session;
- accessibility and input-mode matrix;
- multiplayer comprehension and recovery sessions;
- support/moderation/appeal drills;
- commercial, privacy and legal signoff;
- public player-market custody, dispute and anti-fraud evidence before enablement.

Acceptance:

- participant evidence is real, consented, privacy-bounded, and not generated by
  automation;
- product claims match the gate actually passed;
- public market and public online flags remain disabled until all dependent
  rows are green.

## 7. Ordered execution plan

### Tranche 0 — P0 foundation (this change set)

1. Accept the authority ADR and update project boundaries.
2. Publish this executable plan and rebuild the docs index.
3. Add CI checks for authority and runtime configuration drift.
4. Add a standalone settlement-outbox contract and invariant tests.
5. Remove sibling-CEX loading, shared-root credential defaults, personal paths,
   and silent production-to-development binary fallback from launch assets.
6. Create issue-backed follow-up work.

**Exit gate:** architecture checks and the standalone contract suite pass; the
change is reviewed but does not claim runtime settlement migration complete.

### Tranche 1 — P0 runtime settlement migration

1. Add durable settlement-job schema and migration.
2. Split claim, network execution, and result application into separate phases.
3. Replace blocking CEX/signer operations with async clients.
4. Add account/campaign serialization and telemetry.
5. Execute crash and ambiguous-commit matrices.
6. Remove the legacy transaction-held external call path.

**Exit gate:** static and black-box evidence proves no external I/O under locked
campaign/match transactions and no duplicate payout across every tested crash
point.

### Tranche 2 — P0/P1 authority adapter

1. Freeze a versioned World deterministic transition contract.
2. Implement Nakama conformance and shadow comparison.
3. Add cross-repository Integration component lock.
4. Drain World-local active matches and cut new admission to Nakama.
5. Disable canonical claims from the compatibility enclave.

**Exit gate:** Nakama is the only canonical completion signer; World/Nakama
shadow evidence is deterministic and Integration binds exact revisions.

### Tranche 3 — P1 decomposition and protocol formalization

1. Split server/client correctness modules.
2. Publish OpenAPI/WebSocket/database procedure contracts.
3. Add generated schema and golden-vector CI.
4. Pin toolchains/actions and sign releases.

**Exit gate:** module and protocol ownership are reviewable without traversing
catch-all files, and release provenance is remotely reproducible.

### Tranche 4 — P1/P2 production evidence

1. Complete 24-hour endurance and fault matrices.
2. Complete cross-host, backup, public edge and capacity evidence.
3. Complete multi-OS distribution.
4. Complete human, support, moderation, commercial and legal gates.

**Exit gate:** only explicitly passed gates may be enabled or advertised.

## 8. Immediate prioritized backlog

| ID | Priority | Deliverable | Dependency | Acceptance gate |
| --- | --- | --- | --- | --- |
| WORLD-P0-001 | P0 | Runtime settlement outbox migration | ADR-0002 | no external I/O in locked DB transaction |
| WORLD-P0-002 | P0 | World deterministic transition contract | ADR-0001 | schema + golden vectors |
| WORLD-P0-003 | P0 | Nakama adapter shadow runner | WORLD-P0-002 | zero unexplained divergence |
| WORLD-P0-004 | P0 | Canonical authority cutover runbook | WORLD-P0-003 | drain/cutover/rollback rehearsal |
| WORLD-P0-005 | P0 | Branch protection and required checks | current CI | direct main push blocked |
| WORLD-P1-001 | P1 | Game-server module decomposition | P0 freeze | invariant-focused module tests |
| WORLD-P1-002 | P1 | OpenAPI/WebSocket/error catalogue | authority contract | schema conformance CI |
| WORLD-P1-003 | P1 | Stored-procedure contract and lock map | current DB API | migration/rollback matrix |
| WORLD-P1-004 | P1 | Portable release installation | runtime config | clean-host install proof |
| WORLD-P1-005 | P1 | Credential isolation and rotation | portable install | role-compromise negative tests |
| WORLD-P1-006 | P1 | Reproducible signed release | toolchain pin | provenance + SBOM + signature |
| WORLD-P1-007 | P1 | Fault/ambiguous-commit matrix | settlement migration | all crash points converge |
| WORLD-P1-008 | P1 | Clean 24-hour endurance | isolated host | complete valid summary |
| WORLD-P2-001 | P2 | Public-edge/cross-host evidence | architecture cutover | release matrix green |
| WORLD-P2-002 | P2 | Human multiplayer validation | stable candidate | signed human packet |
| WORLD-P2-003 | P2 | Public market enablement review | custody/support/legal | explicit approval only |

## 9. Definition of done for every tranche

A tranche is done only when:

1. source, docs, schema, tests, runbooks, and evidence describe the same
   boundary;
2. negative tests prove invalid evidence or authority crossings are rejected;
3. CI runs against the exact proposed commit;
4. generated artifacts are checksummed and attached to the run;
5. rollback or disablement is rehearsed;
6. open limitations are recorded without partial credit;
7. the current plan and evidence registry are updated;
8. no historical document is promoted to current status by implication.

## 10. No-go conditions

Stop promotion immediately when any of the following is true:

- World and Nakama both claim the same canonical cursor/root/signature;
- a signer or ledger call occurs while mutable game rows are locked;
- a stale lease owner can apply settlement after takeover;
- the production launcher falls back to an unverified development binary;
- role credentials share one root value or are missing rotation ownership;
- release evidence is not bound to commit/tree/binary/toolchain/environment;
- active-match cross-version takeover is assumed rather than demonstrated;
- a local or automated result is being used as public, human, or commercial
  evidence;
- required CI checks are absent or bypassed.

## 11. Plan maintenance

- This file is the current execution plan until explicitly superseded.
- Architectural changes require an ADR, owner, migration impact, compatibility
  impact, and evidence impact.
- Review at least every two weeks during active migration.
- Closed work must link its commit, PR, tests, and evidence record.
- Deferred work must retain an owner, reason, dependency, and no-go effect.
