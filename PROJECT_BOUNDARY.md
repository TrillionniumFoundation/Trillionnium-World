# Trillionnium World Boundary

- Project ID: `trillionnium-world`
- Canonical repository: `TrillionniumFoundation/Trillionnium-World`
- Visibility: public source repository with project-specific licence terms
- Lane: `game-product`
- Lifecycle: active development
- Current plan: `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md`
- Current gaps: `docs/status/world-gap-registry-v2.json`

## World owns

World is the source of truth for game-domain behavior and deterministic game
artifacts:

- authored RPG/RTS content, rulesets and content digests;
- deterministic simulation, order validation and game outcome calculation;
- campaign/save/progression behavior owned by the game product;
- player-facing economy intents and reconciliation presentation;
- unsigned replay material, result facts and a versioned World outcome hash;
- native client behavior, accessibility, packaging and human-play evidence.

## World does not own

World is not the canonical authority for:

- online participant admission, global event ordering or match-version history;
- canonical command idempotency, runtime restart recovery or archive roots;
- `MatchCompletedV1` construction or match-evidence signing;
- Chain ingress, consensus, finality, inclusion proofs or research commands;
- CEX wallet/ledger settlement or custody;
- cross-repository component locks and end-to-end release evidence.

Those responsibilities belong respectively to Trillionnium Nakama,
Trillionnium Chain, CEX and Trillionnium Integration as defined by ADR-0001.

## Compatibility authority enclave

`trillionnium/crates/trnm-game-server` contains the existing World-local online
authority implementation. It is retained as a bounded compatibility and
migration enclave while the canonical Nakama adapter is built. It may support
local laboratory and rollback evidence, but it must not:

- introduce a new public authority protocol or second canonical evidence root;
- load a Nakama authority private key;
- sign or claim canonical `MatchCompletedV1` evidence;
- submit a Chain research command directly;
- be described as cross-host, public-network or finality-backed authority.

## Repository guardrails

- Active game workspace: `trillionnium/Cargo.toml`.
- `trillionnium/crates/platform` is excluded legacy material; new code is
  forbidden there.
- New sibling filesystem dependencies on Chain, Nakama, CEX or Integration are
  forbidden. Consume immutable packages, generated schemas or exact-revision
  artifacts.
- Production launch scripts must not source sibling repositories, derive
  multiple role credentials from one root secret, contain personal home paths,
  or silently fall back to a development binary.
- No external settlement I/O may occur inside a mutable match/campaign
  transaction.
- CI must be read-only with respect to source and candidate refs.

## Truth hierarchy

When documentation conflicts, use this order:

1. `PROJECT_BOUNDARY.json` and this boundary document;
2. `CURRENT_PLAN.md` and the V4 executable plan;
3. accepted ADRs and versioned protocol contracts;
4. machine-readable gap/evidence registries;
5. source and tests at the exact candidate commit;
6. historical status reports and archived evidence.

No document may grant production, public, human or commercial credit without
the evidence type required by the release evidence contract.
