---
status: current-candidate
plan_id: trillionnium-world-development-2026-08-29-v4
owner: trillionnium-world
base_commit: 1d4dee6d5add45a64f5c138f424e3bdab369ecd4
working_branch: fix/world-plan-gap-closure-v4
verified_commit: pending_exact_head_ci
applies_to:
  - game-product
  - deterministic-world-runtime
  - native-client
  - world-local-compatibility-server
  - world-to-nakama-contract
  - settlement-outbox
supersedes:
  - TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md
last_reviewed: 2026-08-29
review_due: 2026-09-12
release_effect: none
public_online: no-go
public_player_market: disabled
---

# Trillionnium World Development Plan — 2026-08-29 (v4)

Machine-readable companions:

- `trillionnium-world-development-plan-2026-08-29.json`
- `trnm-world-gap-closure-ledger-v4.json`

## 1. Executive decision

Trillionnium World will be completed as a **deterministic game-domain component and native game product**, not as a second canonical online authority.

The target architecture is fixed:

- **World** owns authored content, deterministic rules and simulation, campaign/save/progression behavior, World transition and outcome hashes, unsigned replay/outcome material, the native client, and player-facing economy intents.
- **Nakama** owns online participant admission, canonical global order, command idempotency, match version, restart recovery, canonical archive roots, and `MatchCompletedV1` signing.
- **Chain** owns ingress, consensus, inclusion and finality.
- **CEX** owns wallet/ledger settlement and custody.
- **Integration** owns exact cross-repository component locks, compatibility matrices and release evidence spanning repositories.

The existing `trnm-game-server` remains a bounded `world_legacy_local_alpha` compatibility enclave until the Nakama adapter is promoted. It may support laboratory, migration and rollback evidence. It must not acquire a new public authority protocol, a Nakama private-key surface, a canonical completion signature, Chain-finality claims or public-market authority.

Public online and public player markets remain **NO-GO / disabled**. No source implementation, local test, generated status document or single-host fixture may change that posture.

## 2. Truth hierarchy

The following order is binding:

1. `PROJECT_BOUNDARY.md` and `PROJECT_BOUNDARY.json`;
2. accepted ADRs;
3. this plan and its machine manifest;
4. normative protocol/database contracts;
5. machine-readable gate status and exact evidence records;
6. implementation and tests;
7. operational reports;
8. historical material.

A contradiction between layers is a blocker. Historical documents do not become current through directory placement or repeated citation.

## 3. Release denominators

Progress is not expressed as one percentage. Each denominator advances independently:

| Denominator | Current source posture | Promotion boundary |
| --- | --- | --- |
| Deterministic World runtime | implemented candidate | strict canonical contract, cross-language vectors and exact-head CI |
| Native software alpha | technical alpha | exact package/runtime evidence on supported target matrix |
| Commercial single-player | blocked | independent human, accessibility, distribution, support, privacy and legal evidence |
| Trusted CEX settlement | source candidate | deployed ambiguity/failure matrix, custody, PITR, retention and reviewer approval |
| Closed Nakama online | blocked | sole Nakama authority, shadow convergence, exact component lock, drain/cutover/rollback |
| Public online | no-go | closed-online plus public edge, cross-host durability, capacity, endurance, moderation and human evidence |
| Public player market | disabled | public-online plus custody, fraud, dispute, commercial, privacy, legal and governance approval |

The allowed state progression is:

```text
planned -> implemented -> independently_validated -> release_eligible
```

`implemented` never implies deployment or release eligibility.

## 4. Non-negotiable invariants

### 4.1 Authority

1. Exactly one accountable system owns each canonical cursor, root, receipt and signature.
2. World must never load, derive, proxy or persist the Nakama match-authority private key.
3. A World transition/outcome hash proves only exact deterministic game-domain material under named ruleset/content revisions.
4. A World artifact cannot claim participant admission, canonical total order, archive completeness, Chain finality or wallet settlement.
5. New online admission enters only the eventual Nakama authority after cutover; the World-local enclave is frozen to compatibility and rollback evidence.

### 4.2 Determinism and canonical encoding

1. Contract payloads are strict UTF-8 canonical JSON with object/array roots.
2. Object keys are decoded and strictly ascending; duplicates are rejected.
3. Numbers are signed-i64 decimal integers only; floats, exponents, leading zeros, `-0`, NaN and overflow are rejected.
4. Insignificant whitespace, trailing data, nonminimal escapes and depth above the published limit are rejected.
5. Authority-field rejection operates on decoded keys recursively, not substring matching over raw bytes.
6. Identical exact inputs under one component lock produce byte-identical accept/reject output and hashes.
7. Rust, Go/Python reference implementations and Nakama conformance must reproduce the same positive and negative vectors.

### 4.3 Settlement

1. No signer, CEX, wallet, ledger or other remote operation runs while mutable match/campaign rows are locked.
2. Capture commits before a job is visible or a remote attempt can begin.
3. Remote request identity is stable across local capture generations and binds immutable intent identity.
4. Every mutation after claim is fenced by live lease owner, generation and expiry.
5. Signer and CEX lookup precede submit; timeout, malformed success response, response loss and conflict are ambiguous, not silent permanent failure.
6. Remote success and campaign application are distinct durable states.
7. Campaign apply revalidates terminal identity, campaign revision/hash, head intent, receipt and exact CAS in one transaction.
8. A poison match, capture or job is quarantined with bounded retry evidence and cannot block unrelated accounts.
9. Same-account/campaign mutations remain serialized; unrelated accounts make bounded concurrent progress.
10. SIGINT/SIGTERM stop new capture/claim, boundedly drain in-flight work, and leave unfinished jobs recoverable by lease expiry.
11. Operator replay is exact-identity-bound, append-only, one additional remote attempt, retained and audited.

### 4.4 Repository governance and CI

1. Development never occurs directly on `main`.
2. CI has read-only repository permissions for validation workflows.
3. CI may upload immutable evidence but must not rewrite, commit, push, tag, merge or promote candidate source.
4. Server-side ruleset/branch protection, required checks and review controls must be observed through GitHub API; source files cannot self-assert them.
5. A missing, empty, skipped, cancelled, stale or environment-unbound check is a blocker.
6. Exact-head evidence records commit, tree, source manifest, toolchain, dependency lock, environment/topology, raw artifact hashes and reviewer decision.

### 4.5 Runtime and supply chain

1. Production artifacts contain no personal path or sibling checkout dependency.
2. Production startup requires a verified release selector; development fallback is explicit opt-in only.
3. Game authority, moderator, settlement worker and signer credentials are distinct by audience and privilege.
4. Toolchains and GitHub Actions are pinned; release artifacts include SBOM, licence inventory, provenance and signatures.
5. KMS/HSM, mTLS/workload identity, public ingress and cross-host durability are separate evidence gates.

## 5. Target flows

### 5.1 Canonical online command

```text
Native Client
    -> Nakama admission/session
    -> Nakama canonical sequence and idempotency
    -> versioned World transition request
    -> World deterministic transition/rejection
    -> Nakama persistence, archive roots and restart recovery
    -> Nakama MatchCompletedV1 signature
    -> Integration exact component lock
    -> Chain/CEX consumers under their own contracts
```

### 5.2 Terminal settlement

```text
transaction A: capture
    lock terminal match and campaigns
    bind terminal identity, revisions, hashes and head intents
    insert immutable capture/jobs
    commit

transaction-free execution
    claim one live lease
    lookup signer receipt -> sign only on exact 404
    lookup CEX receipt -> submit only on exact 404
    persist remote success or retry/dead-letter under live lease fence

transaction B: apply
    relock terminal match/campaigns/capture/jobs
    revalidate exact fences and receipt
    apply campaign mutation using captured receipt backend only
    CAS revisions/hashes
    mark campaign_applied_at and finalize when all heads drain
    commit
```

### 5.3 Evidence promotion

```text
source + schema + tests
    -> exact-head CI artifact
    -> black-box/environment evidence
    -> independent review
    -> gate registry update
    -> release decision by accountable owner
```

## 6. Gap taxonomy

Every gap has one type:

- `source`: closeable in this repository by code/docs/tests;
- `repository_control`: requires GitHub server configuration;
- `cross_repository`: requires Nakama, CEX, Chain or Integration owner changes;
- `environment`: requires deployed infrastructure or physical hosts;
- `human`: requires consented human sessions;
- `custody_security`: requires key/custody control and independent security approval;
- `commercial_legal`: requires business, privacy and legal approval.

A task closes only in the evidence class assigned to it. Source work cannot close external classes.

## 7. Workstreams

## W0 — Governance, truth and documentation

Deliverables:

- canonical README and documentation index;
- Plan v4 and machine gap ledger;
- current authority and settlement ADRs;
- release denominator matrix;
- documentation metadata and link/path validator;
- read-only CI policy;
- removal of self-modifying source workflows;
- observed server-side ruleset snapshot.

Exit criteria:

- one current plan is discoverable from the repository root;
- current docs name exactly one owner for every canonical artifact;
- current and historical documents are visibly separated;
- documentation CI rejects missing paths, stale plan pointers, overclaims and write-enabled validation workflows;
- branch protection and required checks are verified server-side.

## W1 — Strict World deterministic transition contract

Deliverables:

- `trnm_world_transition_v1` package;
- strict canonical JSON parser and encoder;
- schema, positive vectors and adversarial negative vectors;
- stable typed error catalogue and resource budgets;
- independent Python/Go conformance implementation;
- exact accept/reject bytes and domain-separated hashes;
- compatibility and retirement policy.

Exit criteria:

- malformed syntax, duplicate/unsorted keys, noncanonical numbers/escapes, excessive depth and decoded authority keys fail closed;
- Rust and independent implementation reproduce every vector;
- contract has no network, database, wall-clock, randomness, signer, wallet or authority credential dependency;
- exact-head CI is green.

## W2 — Settlement runtime closure

Deliverables:

- migrations 16–19 and checksum ledger registration;
- capture/execute/apply split;
- strict remote identity and lease fencing;
- lookup-before-submit and ambiguous-success recovery;
- bounded error-body handling;
- poison match/capture/job quarantine and backoff;
- same-key serialization and unrelated-key concurrency;
- SIGINT/SIGTERM bounded drain;
- exact operator replay/retention/alert controls;
- source, PostgreSQL and deployed fault matrix.

Exit criteria:

- game server contains no in-process settlement scheduler and synchronous CEX backend fails closed;
- no remote operation begins before capture commit;
- stale or expired worker cannot mutate the job;
- malformed 2xx, response loss and conflict converge through exact lookup without duplicate value;
- poison work is quarantined and unrelated work progresses;
- shutdown at every phase leaves a recoverable durable state;
- exact-head Rust/PostgreSQL/deployed matrix is green and independently reviewed.

## W3 — Nakama adapter, shadowing and cutover

Owner repositories: World for contract behavior, Nakama for canonical runtime, Integration for cross-repository evidence.

Deliverables:

- authenticated Nakama adapter consuming the exact World contract;
- shadow runner comparing World-local and Nakama-driven deterministic outputs;
- representative accepted/rejected/restart/load corpus;
- divergence classification and promotion block;
- active-match drain, admission cutover, rollback and enclave-disablement runbooks;
- exact Integration component lock.

Exit criteria:

- zero unexplained divergence;
- Nakama is sole canonical admission/order/recovery/root/completion signer;
- active World-local matches are drained or a separately reviewed takeover matrix passes;
- rollback and authority disablement are rehearsed;
- World-local public endpoints are disabled or laboratory-only.

## W4 — Correctness-oriented decomposition

Deliverables:

```text
server/
  http/
  identity/
  authority/{actor,admission,command_lane,publication}
  persistence/{migrations,command_store,checkpoint_store,terminal_store}
  journal/{hot,cold,manifest,recovery}
  settlement/
  readiness/
  fleet/
  operations/

client/
  campaign/
  ui/
  online/{transport,journal,reconciliation}
  render/
  simulation_adapter/
```

Each module documents owned state, concurrency, lock order, durable/public boundary, retry/idempotency, failure posture and tests.

Exit criteria:

- no catch-all correctness file owns unrelated responsibilities;
- no circular dependency;
- module APIs have invariant-focused tests;
- source generation does not hide semantic code changes from review.

## W5 — Protocol and database formalization

Deliverables:

- OpenAPI for HTTP surfaces;
- JSON Schema for WebSocket and evidence messages;
- stable machine errors and retry metadata;
- capability negotiation and retirement dates;
- canonical hashing specification;
- ER diagram and stored-procedure catalogue;
- transaction isolation and global lock-order graph;
- role/privilege matrix;
- rolling writer, migration, rollback, PITR and old-primary matrix;
- hot-query index and plan baselines.

Exit criteria:

- examples round-trip through schemas and implementation;
- every procedure states preconditions, postconditions, privileges, result codes and retry behavior;
- supported PostgreSQL versions are explicit;
- compatibility-breaking changes require an ADR and evidence.

## W6 — Portable runtime, secrets and supply chain

Deliverables:

- self-contained clean-host installer and uninstall/rollback;
- dedicated service identities and private state/config directories;
- credential issuance/TTL/rotation/revocation/break-glass runbooks;
- verified-release-only production startup;
- pinned toolchains and action SHAs;
- SBOM, licence report, provenance, signatures and reproducibility record;
- KMS/HSM and mTLS/workload-identity migration.

Exit criteria:

- clean host needs no sibling source checkout;
- role compromise cannot cross signer/ledger/moderator boundaries;
- release inputs and binaries are reproducibly identified;
- secrets never appear in source, logs, artifacts or player UI.

## W7 — Reliability and evidence

Required matrices:

- command submit/commit/publication crash points;
- signer/CEX malformed-success and response-loss;
- cancellation, SIGTERM, SIGKILL and lease takeover;
- PostgreSQL kill-before-ACK, rollback, failover, PITR/timeline and old-primary isolation;
- journal hot/cold/manifest corruption;
- compatibility-window upgrade/downgrade;
- Nakama shadow divergence;
- multi-client concurrency and settlement backlog;
- isolated clean 24-hour active-match endurance;
- multi-OS package/install/upgrade/rollback.

Exit criteria:

- every evidence record matches `release/trnm-world-evidence-record-v1.md`;
- partial/stale/unbound evidence fails closed;
- no partial endurance credit;
- local evidence cannot satisfy public-network, cross-host, human or commercial rows.

## W8 — Product and human validation

Deliverables:

- coherent 10–15 minute `NEW -> RPG -> RTS -> debrief -> town` slice;
- three independent five-second observers;
- non-developer unguided session;
- keyboard-only, mouse-only, high-contrast, subtitles, low-motion and viewport matrix;
- multiplayer comprehension, disconnect and recovery sessions;
- staffed support/moderation/appeal drills.

Exit criteria:

- evidence is real, consented, privacy-bounded and exact-build-bound;
- no gameplay hint contaminates unguided rows;
- accessibility failures have owners and regression tests;
- automated evidence never substitutes for human evidence.

## W9 — Public and commercial readiness

Deliverables:

- public TLS/mTLS edge, WAF/DDoS, abuse controls and capacity;
- multi-host/regional topology, backup, retention and disaster recovery;
- KMS/HSM custody and independent security review;
- privacy, data retention, incident and vulnerability response;
- payment/refund/chargeback/dispute/support operations;
- fraud, market abuse, listing ownership and economic governance;
- explicit commercial/legal approvals.

Exit criteria:

- every dependency in the public-online and public-market gate graph is independently green;
- enablement is an explicit owner decision, never a side effect of game source;
- rollback, disablement and incident rehearsals pass.

## 8. Ordered execution tranches

### T0 — Plan/document/governance convergence

1. Publish Plan v4, machine manifest and gap ledger.
2. Rebuild README/docs/status/release truth.
3. Remove self-modifying workflows.
4. Add documentation and CI-integrity checks.
5. Establish valid CODEOWNERS and observe server rules.

Exit: source truth converges; exact-head documentation/static checks pass; server governance is observed or the outcome is `SERVER_CONFIGURATION_REQUIRED`.

### T1 — Source-manageable P0 closure

1. Harden settlement worker shutdown, isolation, ambiguity and database constraints.
2. Integrate strict World transition contract and negative vectors.
3. Run Rust, Python and PostgreSQL exact-head gates.
4. Capture immutable artifacts and review results.

Exit: source P0 rows are independently validated; no external class is overclaimed.

### T2 — Cross-repository authority and settlement integration

1. Merge CEX receipt lookup/identity contract in owner repository.
2. Implement Nakama adapter and shadow runner.
3. Bind World/Nakama/CEX/Integration exact revisions.
4. Execute deployed fault, drain, cutover and rollback matrices.

Exit: Nakama is sole canonical online authority and settlement ambiguity converges across deployed components.

### T3 — Decomposition, protocol and database formalization

1. Split correctness modules without changing authority.
2. Publish OpenAPI/WebSocket/database contracts.
3. Pin supply chain and produce signed artifacts.

Exit: correctness ownership is reviewable and release provenance is reproducible.

### T4 — Reliability and portable distribution

1. Complete multi-OS install/upgrade/rollback.
2. Complete cross-host recovery and 24-hour endurance.
3. Complete backup/PITR/retention and incident drills.

Exit: operational denominator rows are green for their stated topology.

### T5 — Human/product validation

Complete all W8 sessions against an exact candidate build.

Exit: human rows are signed and accepted; failures return to owning workstream.

### T6 — Public/commercial decision

Complete W9 and obtain explicit owner approvals.

Exit: enable only the exact denominator approved. Otherwise remain NO-GO.

## 9. Immediate backlog

| ID | Priority | Owner | Deliverable | Current state |
| --- | --- | --- | --- | --- |
| WORLD-P0-001 | P0 | World | durable settlement capture/execute/apply | implemented candidate; exact CI/deployed matrix pending |
| WORLD-P0-001A | P0 | World | SIGTERM/SIGINT bounded drain | source gap |
| WORLD-P0-001B | P0 | World | poison work quarantine and unrelated progress | source/database gap |
| WORLD-P0-001C | P0 | World/CEX | malformed-success/conflict ambiguity recovery | source and cross-repo gap |
| WORLD-P0-002 | P0 | World | strict deterministic transition contract | separate candidate; integration and strict parser closure required |
| WORLD-P0-002A | P0 | World | canonical JSON adversarial conformance | source gap |
| WORLD-P0-003 | P0 | Nakama | adapter and shadow runner | blocked upstream |
| WORLD-P0-004 | P0 | Integration | exact component lock and cutover evidence | blocked upstream |
| WORLD-P0-005 | P0 | repository admin | protected main and required checks | server configuration required |
| WORLD-P0-006 | P0 | World | remove CI source self-modification | source gap |
| WORLD-P1-001 | P1 | World | server/client module decomposition | pending P0 freeze |
| WORLD-P1-002 | P1 | World | OpenAPI/WebSocket/error catalogue | pending authority contract |
| WORLD-P1-003 | P1 | World | stored-procedure and lock-order contract | pending settlement schema freeze |
| WORLD-P1-004 | P1 | World | portable clean-host installation | partial source foundation |
| WORLD-P1-005 | P1 | World/CEX | service identity and credential rotation | environment/custody pending |
| WORLD-P1-006 | P1 | World | reproducible signed release | source/environment pending |
| WORLD-P1-007 | P1 | World/CEX | full settlement/authority fault matrix | deployed environment pending |
| WORLD-P1-008 | P1 | World | clean 24-hour endurance | environment pending |
| WORLD-P2-001 | P2 | Integration/Ops | public edge/cross-host/regional evidence | blocked by T2–T4 |
| WORLD-P2-002 | P2 | World/Product | human single/multiplayer evidence | exact candidate and participants pending |
| WORLD-P2-003 | P2 | CEX/Business/Legal | public market decision | disabled |

Exact state, dependencies and acceptance checks live in the gap ledger.

## 10. Evidence contract

Every accepted evidence record includes:

- claim and gate ID;
- evidence class and scope;
- repository, commit and Git tree;
- binary/image/package digest and source manifest;
- toolchain/dependency/component-lock revisions;
- environment, topology and security boundaries;
- start/end timestamps and timezone;
- thresholds, observed metrics and final result;
- raw artifact paths, sizes and hashes;
- injected faults and expected/observed recovery;
- limitations, expiry and next review;
- reviewer identity, role and decision.

Evidence is rejected when any required binding is absent, a dependency is stale, the run is partial, the environment does not match the claim, or the producer attempts to satisfy a higher evidence class.

## 11. Definition of done

A work item is closed only when:

1. code, docs, schemas, tests, runbooks and status describe the same boundary;
2. positive and negative tests cover the invariant;
3. exact-head CI runs and passes;
4. immutable evidence is checksummed and retained;
5. rollback, fail-close or disablement is rehearsed where applicable;
6. independent owner/reviewer approval is recorded;
7. all dependencies are closed in their own evidence class;
8. the gap ledger and current status are updated without overclaim;
9. no historical or generated document is acting as unreviewed truth;
10. the release effect is explicitly stated.

## 12. Valid stop outcomes

When a task cannot close in this repository, use one of:

- `MODULE_CLOSED_CANDIDATE` — source/docs/tests complete; exact independent promotion still pending;
- `BLOCKED_UPSTREAM` — another owner repository must change;
- `SERVER_CONFIGURATION_REQUIRED` — repository settings/control plane required;
- `EXTERNAL_EVIDENCE_REQUIRED` — deployed, physical, human, custody or commercial evidence required;
- `BASE_DRIFT` — exact base changed and revalidation is required;
- `RESUME_REQUIRED` — execution channel lacks a required capability;
- `STOP_CONDITION` — a no-go invariant is violated.

These outcomes are not failures to report progress; they prevent false closure.

## 13. No-go conditions

Stop promotion immediately when:

- World and Nakama both claim one canonical cursor/root/signature;
- canonical JSON acceptance differs across promoted implementations;
- remote settlement begins before capture commit;
- timeout, malformed success or conflict can create duplicate value;
- stale/expired lease can mutate a job;
- poison work blocks unrelated accounts without bounded quarantine;
- SIGTERM can continue admitting new settlement work indefinitely;
- CI modifies or pushes candidate source;
- production falls back to an unverified binary;
- role credentials share a root value or audience;
- evidence lacks exact source/binary/environment binding;
- active-match takeover is assumed rather than demonstrated;
- local/automated evidence is used as human/public/cross-host/custody/commercial proof;
- required checks, review or server controls are absent or bypassed.

## 14. Plan maintenance

- This plan becomes canonical when its branch is reviewed and merged.
- Architectural changes require an ADR with migration, compatibility and evidence impact.
- Review every two weeks during active closure and immediately after base drift.
- Closed work links exact commit, PR, checks, artifacts and reviewer decision.
- Deferred work retains owner, dependency, reason, evidence class and no-go effect.
- The machine manifest and gap ledger must remain parseable and semantically consistent with this document.