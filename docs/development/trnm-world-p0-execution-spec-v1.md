# TRNM World P0 Execution Specification v1

Status: **current / execution-blocking**  
Owner: World runtime and repository governance  
Updated: 2026-08-27  
Parent plan: `docs/development/trnm-world-development-plan-v2.md`  
Decision: `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`

## Purpose

This document turns the P0 section of Development Plan v2 into an ordered, reviewable implementation contract. It does not create a new roadmap or widen product scope. A task may move to `done` only when its exact source commit has passed remote required checks and its residual limitations are recorded.

## Global stop-the-line rules

Development stops and the current slice remains blocked when any of the following is true:

1. World and Nakama can both be externally authoritative for the same match.
2. A signer, CEX, DNS, HTTP or other external transport is called while a mutable PostgreSQL business transaction remains open.
3. A stale campaign revision, state hash, terminal marker or component lock can be applied.
4. A retry changes a durable intent or command identity after an ambiguous remote outcome.
5. Production mode can execute an unverified source-tree binary or derive several roles from one root credential.
6. A release/status claim lacks exact commit, workflow run, environment, limitation and review/expiry metadata.
7. Current CI does not run the product-boundary, authority-boundary and settlement-boundary negative fixtures.

## Ordered P0 slices

### P0.1 — authority ownership convergence

Deliverables:

- ADR-0001 is the only current authority decision in World.
- Existing World Online Authority is labeled `legacy_local_alpha` in current documentation and status output.
- World publishes only deterministic game-domain runtime/output contracts.
- Nakama owns participant framing, global ordering, idempotency, restart recovery, canonical roots and completion signing.
- Chain owns ingress/finality/inclusion proof; CEX owns wallet ledger/custody; Integration owns exact cross-repository locks and E2E evidence.
- Repository checks reject a World-owned Nakama authority key, completion signer, canonical match root or sibling Chain path dependency.

Acceptance evidence:

- one accountable owner per capability;
- positive repository-boundary check;
- at least one negative fixture per forbidden capability;
- exact World/Nakama/Integration component lock;
- no contradictory current document.

Rollback boundary:

- revert adapters and route new matches back to the previously selected single authority;
- never enable both authorities as a rollback mechanism.

### P0.2 — settlement capture / execute / apply

Required production flow:

```text
short capture transaction
  -> commit and release every business lock
  -> bounded external execution using original intent IDs
  -> short apply transaction with exact optimistic revalidation
```

Capture token must bind:

- match ID;
- exact terminal publication/evidence tuple;
- member campaign IDs in deterministic order;
- campaign revision and persisted state hash;
- serialized campaign value used for execution;
- pending intent and compensation identities;
- capture schema version.

Execute phase requirements:

- no `PgConnection`, `PgPoolConnection`, `Transaction` or row guard is owned by the executing task;
- synchronous legacy economy work runs on a bounded blocking lane;
- timeout/cancellation does not mutate local durable state;
- ambiguous signer/CEX outcomes retain the original intent IDs;
- secrets and player sessions never enter logs.

Apply phase requirements:

- re-lock the exact match and campaign rows;
- revalidate phase, terminal ACK, cold seal and publication tuple;
- compare campaign revision and persisted state hash with capture;
- reject the complete apply on any stale member;
- persist only results derived from the captured value;
- advance terminal ACK and match settlement atomically only when all required campaigns are reconciled;
- return a typed outcome: `applied_settled`, `applied_pending`, `stale_capture`, `not_claimable` or `retryable_failure`.

Mandatory tests:

| Test | Required assertion |
| --- | --- |
| external visibility | mock signer/CEX receives no request before capture commit |
| stale revision | concurrent campaign update rejects apply |
| stale state hash | same revision with changed persisted hash rejects apply |
| terminal drift | changed/missing exact publication marker rejects apply |
| ambiguous CEX commit | retry reuses intent ID and receives the existing receipt |
| signer success / CEX failure | match remains pending without duplicating entitlement |
| apply rollback | remote success is safely replayable |
| two workers | at most one apply succeeds for a capture |
| partial member | match remains pending and unrelated match is not blocked |
| process kill | every phase boundary is recoverable |
| shutdown | no database lock survives external cancellation |

The old transaction-spanning path must be deleted, not retained as an automatic fallback.

### P0.3 — remote merge governance

Required mainline controls:

- pull request required;
- direct push and deletion blocked;
- one approving code-owner review;
- stale review dismissed after relevant changes;
- required current game CI checks;
- unresolved review conversations blocked;
- branch must be up to date or merge queue must revalidate it;
- release claims require a successful exact-commit run.

Required CI jobs:

1. product and changed-path boundary;
2. authority ownership boundary plus negative fixtures;
3. settlement transaction boundary plus executable fault/reference tests;
4. format, locked all-target tests and strict Clippy;
5. dependency audit/deny with owned, expiring exceptions;
6. package verification, SBOM and provenance;
7. current-document metadata/link/schema validation.

Legacy L1, validator and Web4 workflows are archive evidence and cannot be required World product checks.

### P0.4 — current truth and evidence registry

Every current document must declare:

- status;
- accountable owner;
- applicable protocol/schema/release;
- verified commit or explicit `unverified` state;
- last review and next review/expiry;
- superseded/current relationships;
- evidence and limitations.

Machine-readable evidence must distinguish:

- `source_implemented`;
- `remote_ci_verified`;
- `deployed`;
- `operationally_observed`;
- `release_approved`.

No transition is implicit. Public online and public player market remain blocked until their own denominators pass.

## P0 dependency graph

```text
P0.1 authority decision + guard
  |\
  | +--> deterministic World contract / Nakama consumer
  |
  +----> P0.2 settlement split
            |
            +--> fault and ambiguous-commit evidence

P0.3 CI/governance validates P0.1 and P0.2
P0.4 status/evidence projects only validated outcomes
```

## Pull-request slicing

To keep reviews bounded:

1. **Plan and authority guard PR** — documents, CODEOWNERS, repository checks and negative fixtures.
2. **Settlement runtime PR** — capture/execute/apply production code and focused tests only.
3. **Settlement fault PR** — black-box signer/CEX/database/process failure harness and evidence schema.
4. **CI/governance PR** — current workflows, immutable action pins, package/provenance and mainline rules.
5. **Runtime-contract PRs** — exact World contract, Integration lock and Nakama consumer, one repository per PR.

Do not mix unrelated gameplay breadth, public-market work or historical document cleanup into a P0 runtime PR.

## Completion record

For every slice record:

```yaml
slice_id: P0.x
status: source_implemented | remote_ci_verified | deployed | blocked
repository: TrillionniumFoundation/Trillionnium-World
commit: <40-hex SHA>
workflow_run: <immutable run reference or null>
checks:
  - <check name and result>
limitations:
  - <remaining limitation>
reviewed_at: <RFC3339>
review_due: <RFC3339>
```

A missing value is represented as `null` or `blocked`; it is never inferred from prose.
