---
status: accepted
date: 2026-08-27
owners:
  - Trillionnium-World
  - Trillionnium-Nakama
  - Trillionnium-Chain
  - CEX
  - Trillionnium-Integration
supersedes:
  - implicit-world-and-nakama-dual-authority
---

# ADR-0001: Realtime Authority and Match-Evidence Ownership

## Context

World currently contains a sophisticated local online-authority implementation:
match actors, command sequencing, idempotency, reconnect, PostgreSQL recovery,
publication journals, terminal acknowledgements, fleet fencing, replay and
settlement projections. A newer cross-repository design assigns canonical
participant admission, event order, restart recovery, archive roots and signed
match completion to Nakama.

Allowing both designs to remain canonical would create two incompatible sources
of truth for the same match. A game outcome hash, a server database row and a
signed match-completion record would no longer have one accountable owner.
Recovery after network partitions, process death or rollback could select
different histories.

## Decision

### World

World owns:

- authored content and ruleset revisions;
- deterministic game-domain validation and state transitions;
- deterministic game outcomes;
- unsigned replay and archive material produced by the game;
- canonical serialization and versioned hash of the **World-owned outcome**;
- native client and player-facing game behavior;
- game-owned economy intents.

A World outcome hash attests only to the deterministic game-domain material
specified by its version. It does not attest participant admission, total event
order, archive completeness, signing authority, Chain inclusion or finality.

### Nakama

Nakama is the canonical online match authority and owns:

- one-time match authorization consumption;
- authenticated participant roster and role framing;
- global event sequence and match version;
- canonical command idempotency;
- runtime restart recovery;
- canonical event, roster and archive roots;
- `MatchCompletedV1` construction;
- the private key used to sign match-completion evidence.

World must not load, derive, proxy or back up the Nakama authority private key.

### Chain

Chain owns:

- canonical ingress;
- consensus and state transition;
- finality and AppHash binding;
- inclusion proofs;
- research command semantics.

No World or Nakama artifact is Chain-finalized until Chain returns the versioned
receipt required by the Chain contract.

### CEX

CEX owns wallet/ledger settlement, escrow, refunds, chargebacks, custody and
verified economic receipts. World emits typed intents and consumes receipts; it
does not own ledger balances.

### Integration

Integration owns cross-repository component locks and end-to-end evidence that
binds exact World, Nakama, Chain and CEX revisions. A repository-local test may
not claim a cross-repository gate without this binding.

## Compatibility enclave

The existing `trillionnium/crates/trnm-game-server` implementation is retained
as a bounded compatibility authority enclave during migration.

Allowed:

- local deterministic laboratory evidence;
- rollback fixtures and migration readers;
- compatibility tests for existing clients;
- shadow comparison against the target Nakama path;
- bug fixes that reduce an existing P0 without expanding public authority.

Forbidden without a replacement ADR:

- new public authority protocol generations;
- new canonical evidence roots or completion signatures;
- loading Nakama private authority material;
- direct Chain research-command construction;
- claims of public, cross-host, regional or finality-backed authority.

## Migration

1. Freeze authority expansion and mark the enclave.
2. Extract a versioned deterministic World transition/outcome contract.
3. Implement Nakama conformance and shadow-diff tests.
4. Drain World-local active matches; do not infer live cross-generation takeover.
5. Admit new canonical matches only through Nakama.
6. Remove duplicated authority responsibilities from World after rollback and
   compatibility obligations expire.

## Consequences

Positive:

- one accountable owner for every canonical cursor, root and signature;
- clearer security and key custody;
- deterministic World logic remains independently testable;
- Chain and CEX boundaries remain explicit;
- cross-repository evidence can bind exact components.

Costs:

- existing World-local authority work becomes migration debt;
- a new versioned adapter and shadow runner are required;
- active matches must be drained unless takeover is separately proven;
- some local evidence must be reclassified rather than promoted.

## Enforcement

- `PROJECT_BOUNDARY.json` records the ownership map and compatibility enclave.
- `scripts/check_trnm_authority_boundary.sh` rejects prohibited source surfaces
  and documentation drift.
- CI runs the boundary check for code, docs, protocol and deployment changes.
- Any exception requires a superseding ADR with security, recovery and evidence
  impact.
