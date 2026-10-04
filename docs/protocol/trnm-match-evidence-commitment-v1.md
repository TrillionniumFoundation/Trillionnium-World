---
status: current
date: 2026-08-27
owner: Trillionnium-World
canonical_authority_owner: Trillionnium-Nakama
related_adr: ../adr/0001-realtime-authority-and-match-evidence-ownership.md
---

# World Match Evidence Boundary v1

## Status

World is not the Nakama match-evidence signer and does not construct a Chain
research command.

The earlier World-local signer/root direction is not the target architecture:
it created a second realtime authority and used authority material that could
diverge from Nakama's participant, ordering and archive contracts. Existing
World-local online-authority code is retained only as the compatibility enclave
defined by ADR-0001.

The authoritative match contract is owned and versioned by
`Trillionnium-Nakama`. Chain ingress, finality, inclusion proofs and research
command semantics are owned by `Trillionnium-Chain`. Cross-repository evidence
and component locks are owned by `Trillionnium-Integration`. Wallet settlement
is owned by CEX.

## World-owned deterministic output

World may produce unsigned, deterministic game-domain material for a match:

- ruleset and content revision identifiers;
- game commands and payloads accepted by the selected ruleset;
- deterministic state transitions;
- result and outcome facts;
- replay material and game-owned archive locations;
- content, ruleset and replay digests;
- a World-owned outcome hash whose canonical serialization is versioned here.

These values are inputs to the authoritative runtime. They do not independently
prove:

- participant admission or role assignment;
- global event order or match version;
- command idempotency or restart recovery;
- archive completeness;
- canonical roster/event/archive roots;
- authority signature;
- Chain inclusion or finality;
- wallet settlement.

## Required World outcome envelope

A future implementation of this contract must bind at least:

```text
contract_version
ruleset_id
ruleset_revision
content_digest
match_domain_id
initial_state_digest
accepted_command_digest
final_state_digest
outcome_digest
replay_material_digest
world_outcome_hash
```

Canonical serialization must specify:

- field order and encoding;
- integer widths and endianness;
- string normalization;
- collection ordering;
- absent/optional field representation;
- hash algorithm and domain separator;
- maximum sizes;
- golden vectors.

No credential, session token, authority private key, Chain key or CEX custody
material is part of the World envelope.

## Nakama-owned authority

Nakama is the canonical online match authority. Nakama alone owns:

- one-time signed match authorization consumption;
- participant roster identity and role framing;
- global event sequence and match version;
- canonical command idempotency and runtime restart recovery;
- canonical event, roster and archive roots;
- `MatchCompletedV1` construction and authority signing.

World must never load, derive or re-sign with the Nakama authority private key.
Consumers retrieve the durable archive through Nakama's authenticated contract
and independently reproduce the canonical roots before accepting signed
completion evidence.

## Chain boundary

World does not import Chain crates through sibling filesystem paths. A
game-to-Chain adapter consumes an immutable published or exact-revision
contract and submits through canonical Chain ingress.

Until ingress and an AppHash/finality-bound receipt are versioned and verified,
no World or Nakama flow may claim canonical Chain settlement or finality.

## CEX boundary

A World result can produce a typed game-owned economic intent. CEX alone owns
wallet/ledger mutation and returns a versioned receipt. A successful World
outcome, Nakama completion or Chain inclusion is not itself a wallet receipt.

## Integration boundary

Cross-repository release credit requires an Integration-owned lock that binds:

- exact World revision and rules/content digest;
- exact Nakama authority contract/build;
- exact Chain ingress/finality contract/build where applicable;
- exact CEX settlement contract/build where applicable;
- schemas, fixtures, evidence artifacts and their hashes.

Repository-local green tests cannot satisfy this cross-repository gate.

## Compatibility enclave restrictions

The existing World-local `trnm-game-server` may be used for local regression,
rollback and shadow comparison while migration is active. It must not:

- add a new public authority protocol generation;
- produce canonical `MatchCompletedV1`;
- introduce a second canonical event/roster/archive root;
- load Nakama private authority material;
- claim public-network, cross-host, regional or Chain-finalized authority.

## Promotion gates

This boundary is implementation-complete only when:

1. the World outcome schema and golden vectors are published;
2. Nakama validates and reproduces the World outcome deterministically;
3. shadow comparison has no unexplained divergence;
4. Integration binds exact component revisions;
5. active World-local matches are drained or an explicit takeover matrix passes;
6. Nakama is the sole canonical completion signer;
7. rollback and disablement are rehearsed.
