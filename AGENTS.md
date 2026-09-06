# Project Boundary (binding)

This Git root is **Trillionnium World** (`trillionnium-world`), lane `game-product`.

Before any write, build, commit, branch, remote or dependency change, run:

```bash
bash scripts/project-preflight.sh
```

Stop on a project ID, lane, remote, branch, topic or dependency mismatch. The repository may be checked out at an operator-approved path; do not encode a personal home directory in source, docs, units or scripts.

## Current truth sources

Read in order:

1. `PROJECT_BOUNDARY.md`
2. `CURRENT_PLAN.md`
3. `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`
4. `docs/development/trnm-world-gap-closure-ledger-v4.json`
5. `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`
6. `docs/adr/0002-transaction-free-external-settlement.md`
7. `docs/status/CURRENT.md`

## Binding ownership

- World owns deterministic game rules, content, simulation, transition/outcome hashes, unsigned replay/outcome material, the native client and player-facing economy intents.
- Nakama owns canonical online admission, total order, idempotency, restart recovery, archive roots and `MatchCompletedV1` signing.
- Chain owns ingress/finality, CEX owns wallet/ledger settlement and custody, and Integration owns cross-repository component locks and release evidence.

`trillionnium/crates/trnm-game-server` is a `world_legacy_local_alpha` compatibility enclave. Do not add a new public authority protocol, Nakama private-key surface, canonical completion signature, Chain-finality claim, public/cross-host authority claim or public-market enablement.

## Settlement rules

No signer/CEX/network operation may run while a mutable match or campaign PostgreSQL transaction holds row locks. Settlement must use capture -> transaction-free remote execution -> fenced apply, with stable remote identity, lookup-before-submit, live lease fencing, poison quarantine, bounded shutdown and exact operator audit.

## Determinism rules

New World transition material follows the strict canonical JSON profile:

- parsed JSON, not bracket scanning;
- decoded strictly ascending unique keys;
- signed-i64 integers only;
- exact minimal encoding and byte equality;
- bounded depth and size;
- recursive decoded authority-key rejection;
- positive and adversarial negative vectors;
- cross-language conformance.

## Repository and CI rules

- Never develop directly on `main`.
- Use one lane-compliant branch and one reviewable PR for the current package.
- Revalidate exact base/head/tree before and after writes.
- Validation workflows use read-only repository permissions.
- CI may upload evidence but must not rewrite, commit, push, tag, merge or promote candidate source.
- New semantic code must not be hidden in build-time source rewrite machinery.
- Do not merge your own PR.
- Server-side branch protection/rulesets and required checks must be observed; source files cannot self-assert them.
- Missing, skipped, cancelled, stale or unbound checks are blockers.

## Active workspace

- `trillionnium/Cargo.toml` contains the active game crates.
- `trillionnium/crates/platform` is excluded legacy material; new code must not enter it.
- New sibling filesystem dependencies on Chain, Nakama, CEX or Integration are forbidden. Use published or exact-revision contracts.

## Required gates

Run the applicable subset, including:

```bash
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh
./scripts/check-trnm-world-documentation.py
./scripts/check-trnm-world-transition-conformance.py
```

Then run exact Rust/PostgreSQL/workstream-specific checks from Plan v4 and capture immutable evidence.

## Completion outcomes

Continue the gap loop until one valid outcome is reached:

- `MODULE_CLOSED_CANDIDATE`
- `BLOCKED_UPSTREAM`
- `SERVER_CONFIGURATION_REQUIRED`
- `EXTERNAL_EVIDENCE_REQUIRED`
- `BASE_DRIFT`
- `RESUME_REQUIRED`
- `STOP_CONDITION`

Never convert an external, server, human, custody or commercial dependency into a false source-level pass.