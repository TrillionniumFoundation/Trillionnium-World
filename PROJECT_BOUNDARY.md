# Trillionnium World Boundary

- Project ID: `trillionnium-world`
- Canonical repository: `TrillionniumFoundation/Trillionnium-World`
- Lane: `game-product`
- Lifecycle: active development
- Current development plan: `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md`
- Authority ADR: `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`

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

Those responsibilities belong respectively to `Trillionnium-Nakama`,
`Trillionnium-Chain`, `CEX`, and `Trillionnium-Integration` as defined by the
authority ADR.

## Compatibility authority enclave

`trillionnium/crates/trnm-game-server` contains the existing World-local online
authority implementation. It is retained as a bounded compatibility and
migration enclave while the canonical Nakama adapter is built. It may continue
to support local laboratory and rollback evidence, but it must not:

- introduce a new public authority protocol or a second canonical evidence root;
- load a Nakama authority private key;
- sign or claim canonical `MatchCompletedV1` evidence;
- submit a Chain research command directly;
- be described as cross-host, public-network or finality-backed authority.

Any change that expands this enclave requires an ADR update and an explicit
migration justification. New online product work should target the versioned
World-to-Nakama contract instead.

## Repository guardrails

- Active game workspace: `trillionnium/Cargo.toml`.
- The excluded `trillionnium/crates/platform` tree is legacy migration content;
  new code must not enter it.
- New sibling filesystem dependencies on Chain, Nakama, CEX or Integration are
  forbidden. Consume immutable published packages, generated schemas or
  exact-revision artifacts instead.
- Production launch scripts must not source sibling repositories, derive
  multiple role credentials from one root secret, contain personal home paths,
  or silently fall back to a development binary.
- Current architecture and runtime-boundary checks are enforced by
  `scripts/check_trnm_authority_boundary.sh` and
  `scripts/check_trnm_runtime_configuration.sh`.
