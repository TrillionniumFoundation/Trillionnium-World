# Trillionnium World Boundary

- Project ID: `trillionnium-world`
- Canonical repository: `TrillionniumFoundation/Trillionnium-World`
- Visibility: public repository with an internal workspace licence policy
- Lane: `game-product`
- Lifecycle: active development
- Current plan pointer: `CURRENT_PLAN.md`
- Current execution snapshot: `docs/status/world-plan-v4-execution-truth-2026-09-08.json`
- Planning foundation: `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`
- Historical machine plan: `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- Historical gap ledger: `docs/development/trnm-world-gap-closure-ledger-v4.json`
- Authority ADRs: `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md` and `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md`
- Settlement ADR: `docs/adr/0002-transaction-free-external-settlement.md`

The August machine plan and gap ledger remain planning/provenance inputs. They are not allowed to override the operative pull request, exact head or execution state selected by `CURRENT_PLAN.md` and the current September snapshot.

## World owns

- authored RPG/RTS content, rulesets and content digests;
- deterministic command validation, simulation and state transition;
- World transition/outcome hashes and unsigned replay/outcome material;
- campaign/save/progression behavior owned by the game product;
- the native client, accessibility, packaging and human-play evidence;
- player-facing economic intents and campaign projection of verified receipts;
- the bounded World-domain service contract and deterministic state mutation under an admitted writer epoch.

## World does not own

- canonical online participant admission, global event order, command idempotency, match generation, restart recovery or archive roots;
- construction/signing of `MatchCompletedV1` or the Nakama authority private key;
- Chain ingress, consensus, inclusion, AppHash or finality;
- CEX wallet/ledger settlement or custody;
- cross-repository component locks and release evidence;
- public player-market enablement.

Those responsibilities belong to Trillionnium Nakama, Trillionnium Chain, CEX and Trillionnium Integration as defined by the accepted ADRs.

## World-domain service boundary

`trillionnium/crates/world-authority` is the bounded World-domain service workspace. Its historical origin and reviewed successors are separately bound by the authority provenance contract. Fixture identity, session, ledger, repository, evidence and metrics adapters provide deterministic tests only. The current production adapter contract is `trillionnium_world_production_adapter_v1`; implementations and exact-head qualification are absent, and production authorization is `not_granted`.

The standalone fixture listener requires explicit `--allow-fixture-runtime`, a literal loopback socket and a nonzero port. It cannot bind wildcard, hostname, private-network or public addresses and must never be publicly routed.

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
- Historical source identity and reviewed successor identity must never be collapsed into one false exact-byte claim.

## Release boundary

The progression is:

```text
implemented -> independently validated -> release eligible
```

A missing, skipped, cancelled, stale or unbound check is a blocker. Automated/local evidence cannot satisfy human, public-network, cross-host, custody, legal or commercial gates. Public online remains NO-GO, public player markets remain disabled and production authorization remains `not_granted` until every dependency in the release matrix is independently green.
