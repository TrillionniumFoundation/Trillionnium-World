# Trillionnium World Boundary

- Project ID: `trillionnium-world`
- Canonical repository: `TrillionniumFoundation/Trillionnium-World`
- Visibility: public repository with an internal workspace licence policy
- Lane: `game-product`
- Lifecycle: active development
- Current plan: `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`
- Machine plan: `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- Gap ledger: `docs/development/trnm-world-gap-closure-ledger-v4.json`
- Authority ADR: `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`
- Settlement ADR: `docs/adr/0002-transaction-free-external-settlement.md`

## World owns

- authored RPG/RTS content, rulesets and content digests;
- deterministic command validation, simulation and state transition;
- World transition/outcome hashes and unsigned replay/outcome material;
- campaign/save/progression behavior owned by the game product;
- the native client, accessibility, packaging and human-play evidence;
- player-facing economic intents and campaign projection of verified receipts.

## World does not own

- canonical online participant admission, global event order, command idempotency, match generation, restart recovery or archive roots;
- construction/signing of `MatchCompletedV1` or the Nakama authority private key;
- Chain ingress, consensus, inclusion, AppHash or finality;
- CEX wallet/ledger settlement or custody;
- cross-repository component locks and release evidence;
- public player-market enablement.

Those responsibilities belong to Trillionnium Nakama, Trillionnium Chain, CEX and Trillionnium Integration as defined by ADR-0001.

## Compatibility authority enclave

`trillionnium/crates/trnm-game-server` is retained under the explicit profile `world_legacy_local_alpha` for local laboratory, migration, drain and rollback evidence. It must not:

- introduce a new public authority generation;
- load or proxy a Nakama private key;
- sign or claim canonical online completion evidence;
- claim cross-host, public-network or Chain-finality authority;
- enable a public player market;
- execute signer/CEX/network work while mutable match or campaign rows are locked.

Any expansion requires an ADR, migration/rollback plan and explicit owner/evidence impact review.

## Deterministic transition boundary

World-to-Nakama communication uses `trnm_world_transition_v1` or a later explicitly versioned contract. The contract carries deterministic game-domain material only. Canonical JSON, resource budgets, hash domains and negative vectors are normative. A sibling filesystem path or release label is not an integration contract.

## Settlement boundary

Settlement follows:

```text
capture transaction -> transaction-free signer/CEX execution -> fenced apply transaction
```

Stable remote identity, lookup-before-submit, live lease fencing, exact receipt binding, quarantine, bounded shutdown and operator audit are mandatory. Source implementation alone grants no trusted-settlement or custody credit.

## Repository guardrails

- Active workspace: `trillionnium/Cargo.toml`.
- `trillionnium/crates/platform` is excluded legacy material; new code must not enter it.
- New sibling filesystem dependencies on Chain, Nakama, CEX or Integration are forbidden.
- Production launch assets contain no personal paths or silent development-binary fallback.
- Runtime role credentials are distinct by audience and privilege.
- Development never occurs directly on `main`.
- Validation workflows use read-only repository permissions and may upload evidence but may not modify, commit, push, tag, merge or promote candidate source.
- GitHub ruleset/branch protection and required checks are server-side facts and must be observed through the API.
- New semantic code must not be hidden in build-time source rewrite machinery.

## Release boundary

The progression is:

```text
implemented -> independently validated -> release eligible
```

A missing, skipped, cancelled, stale or unbound check is a blocker. Automated/local evidence cannot satisfy human, public-network, cross-host, custody, legal or commercial gates. Public online remains NO-GO and public player markets remain disabled until every dependency in the release matrix is independently green.