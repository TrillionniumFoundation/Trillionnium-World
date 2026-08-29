---
status: current
owner: trillionnium-world
plan_id: trillionnium-world-development-v4-2026-08-29
source_base_commit: 1d4dee6d5add45a64f5c138f424e3bdab369ecd4
verified_commit: pending-exact-head-ci
last_reviewed: 2026-08-29
review_due: 2026-09-12
supersedes:
  - TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md
  - trnm-world-development-plan-v3.md
---

# Trillionnium World Development Plan V4

## 1. Executive decision

Trillionnium World will converge on one reviewable game-domain implementation,
one canonical online authority, one transaction-free settlement path and one
machine-readable release truth.

The target accountability model is fixed:

- **World** owns authored content, deterministic rules/simulation, World outcome
  facts, unsigned replay material, native client behavior and player-facing
  economic intents.
- **Nakama** owns target online admission, canonical event order, online command
  idempotency, restart recovery, canonical archive roots and
  `MatchCompletedV1` signing.
- **Chain** owns canonical ingress, consensus, inclusion and finality.
- **CEX** owns wallet/ledger settlement and custody.
- **Integration** owns exact cross-repository component locks and release
  evidence spanning those systems.

The existing World-local game server is a compatibility enclave, not the target
public authority. Public online, public player markets, custody and commercial
release remain NO-GO until their separate evidence denominators pass.

## 2. Why V4 exists

V3 correctly froze authority ownership and introduced the settlement outbox,
but subsequent audit found four gaps that must be explicit in the plan:

1. correctness source is partly reconstructed by build-time text transforms;
2. CI contains workflows that can modify and push the candidate they validate;
3. settlement worker lifecycle, poison isolation, unrelated concurrency and
   malformed-success ambiguity need additional fail-closed handling;
4. the isolated World transition contract must prove actual canonical JSON
   parsing and cross-language equivalence, not delimiter/whitespace heuristics.

V4 also turns Campaign/RTS error-path atomicity, protocol publication, database
API documentation, security, release provenance and evidence lifecycle into
first-class workstreams rather than implicit follow-up.

## 3. Truth hierarchy

Conflicts are resolved in this order:

1. `PROJECT_BOUNDARY.json` and `PROJECT_BOUNDARY.md`;
2. `CURRENT_PLAN.md` and this V4 plan;
3. accepted ADRs and versioned protocol contracts;
4. `docs/status/world-gap-registry-v2.json` and exact evidence manifests;
5. source/tests at the exact candidate commit;
6. historical status narratives and archived reports.

No generated status page or narrative can override failed, missing, stale,
skipped or environment-unbound evidence.

## 4. Release denominators

The project has independent release denominators:

| Denominator | Minimum proof |
| --- | --- |
| Source implemented | reviewed source, tests and negative fixtures |
| Exact-head validated | required CI checks on exact commit/tree |
| Deployed single-host | immutable artifacts plus deployed black-box evidence |
| Cross-host/public online | multi-host fencing/recovery, public edge and capacity |
| Human usable | real consented participant sessions |
| Trusted settlement | custody, ambiguity, rollback, reconciliation and operator approval |
| Commercial release | support, moderation, privacy, legal and distribution approval |

Passing one denominator never implies another.

## 5. Non-negotiable invariants

### 5.1 Authority

1. Exactly one system is accountable for every canonical cursor, root, receipt
   and signature.
2. World never loads or derives the Nakama match-authority private key.
3. A World outcome hash proves deterministic game material only; it does not
   prove admission, global order, archive completeness, Chain finality or wallet
   settlement.
4. New canonical online admission is not added to the World compatibility
   enclave.
5. Active-match cross-generation takeover is prohibited unless a dedicated
   recovery matrix passes.

### 5.2 Settlement

1. No external HTTP, signer, wallet or ledger call executes while mutable
   match/campaign rows are locked.
2. Capture commits before remote-attempt permission.
3. `job_id`, `intent_id`, intent hash and remote request identity have distinct,
   immutable meanings.
4. Authorization, remote attempt, completion, retry and dead-letter writes
   require a live exact lease owner/generation.
5. A malformed or lost success response is ambiguous and must converge through
   lookup-before-submit; it cannot become an unrecoverable permanent failure by
   decoding accident.
6. Remote success and Campaign application are separate durable states.
7. Same-account work is serialized; unrelated accounts have bounded concurrent
   progress.
8. Poison captures/jobs are quarantined without stopping unrelated work.
9. SIGINT, SIGTERM, cancellation and process kill leave recoverable durable
   state.
10. Operator replay is exact-identity-bound, append-only, rate-bounded and
    independently approved.

### 5.3 Determinism and state integrity

1. Canonical JSON is fully parsed, depth-bounded and byte-reencoded.
2. Object keys are strictly ascending after decoding; duplicates fail.
3. Numbers are signed-i64 decimal integers only; floats, exponent forms,
   leading zero, `-0` and overflow fail.
4. Forbidden authority fields are checked after decoding, recursively.
5. A failed Campaign or RTS command preserves state hash, resource counters,
   random cursor, replay log and event sequence.
6. Canonical gameplay state, replay log, checkpoint metadata and transport
   cursors are separate data domains.
7. Cross-platform and cross-language golden vectors are release inputs.

### 5.4 Source, CI and governance

1. Correctness-critical compiled source is directly reviewable.
2. Build scripts cannot silently patch security semantics from template files.
3. CI is read-only for source and candidate refs; no `--fix`, commit, push or
   self-issued verified tag.
4. `main` requires PRs, current required checks, code-owner review, stale-review
   dismissal, conversation resolution and force-push/deletion protection.
5. Every workflow action and toolchain used for release evidence is immutable or
   exact-version pinned.
6. No agent merges its own PR or changes production activation/release truth.

## 6. Target architecture

```text
Native Client
  |
  | player command / product intent
  v
Nakama canonical online authority
  | admission, total order, idempotency, recovery, archive roots
  |
  | trnm_world_transition_v1
  v
World deterministic rules service/package
  | next state, replay material, optional outcome, World hashes
  v
Nakama MatchCompletedV1 + signature
  |                 \
  |                  +--> Integration exact component/evidence lock
  +--> Chain ingress/finality
  +--> World settlement capture --> settlement worker --> signer/CEX --> exact apply
```

The transition boundary is unsigned and authority-neutral. Settlement never
executes in the deterministic transition function or inside locked game-state
transactions.

## 7. Workstreams

### W0 — Governance and truth convergence

Deliverables:

- one current plan and machine-readable gap registry;
- current README/docs index/status;
- valid CODEOWNERS principals;
- read-only exact-head workflows;
- server-side protected `main` snapshot;
- explicit issue/PR ownership and dependency graph.

Exit gate:

- old plans are marked superseded;
- no current index points to legacy L1/Web4 material as product truth;
- direct `main` push and force-push are rejected;
- required checks are observed server-side.

### W1 — Deterministic World transition contract

Deliverables:

- strict canonical JSON parser and canonical encoder;
- JSON Schema, error catalogue and positive/negative vectors;
- dependency-bounded reference package;
- Go/Rust byte-exact conformance corpus;
- version/deprecation/retirement policy;
- production rules adapter separated from the finite HTTPS fixture.

Exit gate:

- independent implementations reproduce exact accepted/rejected bytes and
  hashes;
- every malformed/ambiguous encoding fails with a stable code;
- no authority/custody material crosses the boundary.

### W2 — Transaction-free settlement

Deliverables:

- directly reviewable outbox worker source;
- migrations and stored-procedure contracts;
- capture/claim/remote/apply fault harness;
- SIGINT/SIGTERM drain and kill recovery;
- poison isolation and durable quarantine;
- bounded unrelated-account concurrency;
- lookup recovery for lost/malformed success and duplicate/conflict responses;
- account serialization, telemetry, retention and operator replay;
- backup/PITR/old-primary recovery contract.

Exit gate:

- no external I/O under mutable row locks;
- every crash/ambiguity point converges without duplicate value;
- one poisoned item cannot block unrelated settlement;
- exact-head PostgreSQL and deployed fault evidence pass.

### W3 — Campaign and RTS mutation atomicity

Deliverables:

- uniform validate/candidate/commit command pattern;
- property tests asserting error-state preservation;
- fixes for idempotency tombstones, queue-full, pending-purchase, budget and
  failed RTS command mutation paths;
- explicit random/replay cursor invariants;
- fuzz/property corpus for malformed and adversarial commands.

Exit gate:

- every rejected command preserves complete pre-command state;
- retry and restart produce one deterministic result;
- failures consume no resource, cooldown, queue slot, replay event or RNG step.

### W4 — Correctness-oriented module decomposition

Server target boundaries:

```text
http/
identity/
authority/{actor,admission,command_lane,publication}/
persistence/{migrations,commands,checkpoints,terminal}/
journal/{hot,cold,manifest,recovery}/
settlement/
readiness/
fleet/
operations/
```

Client target boundaries:

```text
campaign/
ui/
online/{transport,journal,reconciliation}/
render/
simulation_adapter/
```

Each module documents owned state, concurrency, lock order, durable/public
boundary, retry/idempotency and fail-open/fail-closed behavior.

Exit gate:

- no catch-all file owns unrelated HTTP, actor, persistence, journal,
  settlement and operations behavior;
- invariant-focused interfaces replace text-source scanners as primary proof.

### W5 — Protocol and database contracts

Deliverables:

- OpenAPI for current HTTP surfaces;
- schemas for WebSocket frames and reconnect/replay cursors;
- stable error/reason/retry metadata;
- compatibility matrix and retirement dates;
- canonical hash specification and vectors;
- ER model, procedure catalogue and privilege model;
- isolation levels, global lock graph and supported PostgreSQL versions;
- rolling writer/migration/rollback/PITR matrix;
- query-plan baselines for hot paths.

Exit gate:

- examples round-trip through implementation and schemas;
- every stored procedure has pre/postconditions, result codes and retry rules;
- protocol version and build provenance are separate.

### W6 — Runtime security and supply chain

Deliverables:

- portable verified-release installer;
- distinct service identities and credentials;
- mTLS/workload identity plan and implementation;
- KMS/HSM custody migration;
- exact Rust and action revisions;
- SBOM, licence inventory, provenance and signatures;
- secret scanning, vulnerability exception owner/reason/expiry;
- least-privilege systemd/container profiles.

Exit gate:

- clean-host install needs no sibling checkout;
- role compromise does not cross signer/ledger authority;
- missing release selector fails closed;
- build and dependency inputs are reproducibly identified.

### W7 — Reliability and evidence

Required matrices:

- authority command/commit/publication crash points;
- signer/CEX response loss, malformed success and duplicate conflict;
- PostgreSQL kill-before-ACK, rollback, failover, timeline/PITR and old-primary
  isolation;
- journal corruption and settlement lease takeover;
- Nakama shadow divergence and cutover rollback;
- multi-client concurrency and settlement backlog;
- clean isolated 24-hour endurance;
- cross-host/public-edge capacity;
- Windows/macOS/Linux package/signing.

Every record binds claim ID, commit/tree/binary/toolchain/environment, topology,
timestamps, thresholds, raw hashes, limitations, reviewer and expiry.

### W8 — Product and human validation

Deliverables:

- coherent 10–15 minute NEW → RPG → RTS → debrief → town slice;
- three independent five-second observers;
- non-developer unguided session;
- keyboard/mouse/accessibility/viewport matrix;
- multiplayer comprehension and recovery sessions;
- moderation/support/appeal drills;
- privacy, custody, fraud, dispute, chargeback, commercial and legal approval.

Automation cannot satisfy these rows.

### W9 — Retirement and repository cleanup

Deliverables:

- retire duplicate plans, workflows and legacy product claims;
- remove temporary source-rewrite scripts and build transforms;
- remove World-local canonical admission/signing after cutover;
- archive superseded protocols with readers only where required;
- define data and evidence retention before deletion.

## 8. Ordered execution tranches

### T0 — Truth and review integrity

1. Publish V4 plan, gap registry, architecture and evidence rules.
2. Remove self-modifying/push/tag CI.
3. Restore concise current README/docs index/status.
4. Correct repository visibility and truth hierarchy.
5. Establish server-side branch rules after exact named checks exist.

Exit: source truth is unambiguous and review cannot be bypassed.

### T1 — Source-manageable P0 safety

1. Harden settlement worker lifecycle, isolation, concurrency and ambiguity.
2. Make compiled settlement source directly reviewable.
3. Complete canonical JSON parser/conformance.
4. Fix Campaign/RTS state-preservation defects and add properties.
5. Run exact-head Rust/PostgreSQL/static checks.

Exit: all repository-owned P0 source invariants pass on one exact head.

### T2 — Cross-repository authority convergence

1. Merge/freeze World transition contract.
2. Implement Nakama adapter and shadow runner in the owning repository.
3. Bind exact World/Nakama/CEX/Integration components.
4. Drain World-local matches; rehearse cutover/rollback.
5. Disable new canonical admission/signing in the compatibility enclave.

Exit: Nakama is the sole canonical online completion signer.

### T3 — Decomposition, protocols and release provenance

1. Split correctness modules without semantic change.
2. Publish OpenAPI/WebSocket/database contracts.
3. Pin toolchains/actions and attach SBOM/provenance/signatures.
4. Complete portable clean-host installation and credential rotation.

Exit: code and operational contracts are independently reviewable and
reproducible.

### T4 — Deployed, human and commercial evidence

1. Execute deployed ambiguity/crash/backup/PITR matrices.
2. Complete clean 24-hour run and multi-host/public-edge evidence.
3. Complete multi-OS distribution.
4. Complete human, support, moderation, privacy, custody and legal gates.

Exit: only explicitly passed product denominators may be enabled or advertised.

## 9. Prioritized backlog

| ID | Priority | Owner | Current state | Closure gate |
| --- | --- | --- | --- | --- |
| WORLD-P0-001 | P0 | World | implemented, unverified | exact-head and deployed settlement matrix |
| WORLD-P0-002 | P0 | World | isolated Draft PR | strict cross-language contract CI and merge |
| WORLD-P0-003 | P0 | Nakama | blocked upstream | zero unexplained shadow divergence |
| WORLD-P0-004 | P0 | Integration | blocked upstream | exact component lock and cutover/rollback |
| WORLD-P0-005 | P0 | repo admins | server configuration required | protected `main` snapshot and negative rehearsal |
| WORLD-P0-006 | P0 | World | source gap | worker shutdown/isolation/concurrency/ambiguity matrix |
| WORLD-P0-007 | P0 | World | source gap on contract PR | full canonical JSON parser and shared vectors |
| WORLD-P0-008 | P0 | World | source gap | Campaign/RTS rejected-command state preservation |
| WORLD-P0-009 | P0 | World | source gap | no source-rewriting or self-modifying CI |
| WORLD-P1-001 | P1 | World | planned | correctness module decomposition |
| WORLD-P1-002 | P1 | World | planned | OpenAPI/WebSocket/error contracts |
| WORLD-P1-003 | P1 | World | planned | procedure catalogue and lock graph |
| WORLD-P1-004 | P1 | World | partial | portable clean-host installation |
| WORLD-P1-005 | P1 | World/CEX | partial | role identity, rotation, mTLS/KMS/HSM |
| WORLD-P1-006 | P1 | World | partial | pinned reproducible signed release |
| WORLD-P1-007 | P1 | World/Integration | blocked on P0 | complete deployed fault matrices |
| WORLD-P1-008 | P1 | World | blocked on P1-007 | clean isolated 24-hour evidence |
| WORLD-P2-001 | P2 | Integration/Ops | blocked | public edge, cross-host, regional evidence |
| WORLD-P2-002 | P2 | World/Product | external evidence required | signed human sessions |
| WORLD-P2-003 | P2 | CEX/Business | blocked | custody/support/legal market approval |

The machine-readable registry is authoritative for exact statuses and
dependencies.

## 10. Definition of done

A work item closes only when:

1. source, docs, schema, tests, runbooks and evidence describe one boundary;
2. negative tests reject invalid authority, identity, encoding and evidence;
3. required CI runs on the exact candidate commit/tree;
4. artifacts are immutable and checksummed;
5. rollback or disablement is rehearsed where applicable;
6. limitations and expiry remain explicit;
7. independent reviewer approval is recorded;
8. machine-readable plan/gap/evidence truth is updated;
9. no stronger release denominator is inferred from weaker evidence.

## 11. Stop and resume outcomes

Use an honest terminal outcome when work cannot continue safely:

- `BASE_DRIFT` — exact base/head/tree changed;
- `BLOCKED_UPSTREAM` — owning repository dependency is absent;
- `SERVER_CONFIGURATION_REQUIRED` — GitHub or deployment control is not a
  source-file change;
- `EXTERNAL_EVIDENCE_REQUIRED` — human, custody, public-network or long-running
  evidence is required;
- `STOP_CONDITION` — an invariant or safety boundary would be violated;
- `RESUME_REQUIRED` — source package is reviewable but validation/approval is
  incomplete.

These are valid fail-closed outcomes, not permission to overclaim closure.

## 12. Plan maintenance

- Review at least every two weeks during active migration.
- Architecture changes require an ADR with migration, compatibility and evidence
  impact.
- Closed rows link commit, tree, PR, checks, artifacts and reviewer.
- Deferred rows retain owner, dependency, reason and release effect.
- Historical plans remain in Git history and must be labelled superseded in the
  current tree.
