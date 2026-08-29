# Project Boundary and Agent Contract (binding)

This Git root is **Trillionnium World** (`trillionnium-world`), lane
`game-product`.

Before any write, build, commit, branch, remote or dependency change, run:

```bash
bash scripts/project-preflight.sh
```

Stop on a project ID, lane, remote, branch, topic, dependency or denied-path
mismatch. Do not encode a personal home directory in source, docs, units or
scripts.

## Current truth sources

Read these in order before development:

1. `PROJECT_BOUNDARY.md`
2. `CURRENT_PLAN.md`
3. `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md`
4. `docs/status/world-gap-registry-v2.json`
5. `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`
6. `docs/adr/0002-transaction-free-external-settlement.md`
7. `docs/adr/0003-reviewable-source-and-non-self-modifying-ci.md`

Historical plans, status narratives and archived evidence do not override these
files.

## Binding ownership

- World owns deterministic game rules, simulation, outcomes, unsigned replay
  material, the native client and player-facing economy intents.
- Nakama owns canonical online admission, event order, command idempotency,
  restart recovery, archive roots and `MatchCompletedV1` signing.
- Chain owns ingress/finality, CEX owns wallet settlement/custody, and
  Integration owns cross-repository component locks and release evidence.

`trillionnium/crates/trnm-game-server` is a compatibility authority enclave.
Do not add a new public authority protocol, Nakama private-key surface,
canonical match-completion signature, direct Chain research command, or public
or cross-host authority claim there.

## Engineering invariants

1. No external signer, CEX, wallet, ledger or network call may run while a
   mutable match/campaign PostgreSQL transaction holds row locks.
2. Settlement capture, remote execution and exact apply are separate durable
   phases with immutable identity and live lease fencing.
3. A failed domain command must preserve the complete pre-command state and
   replay/event counters.
4. Canonical JSON must be parsed, not approximated by delimiter/whitespace
   checks; duplicate keys, unsorted keys, non-i64 numbers, nonminimal encoding
   and forbidden decoded authority fields fail closed.
5. Correctness-critical compiled source must be directly reviewable. Build-time
   generators may generate data or bindings, but must not silently rewrite
   security semantics from hidden source templates.
6. CI validates exact source and uploads evidence. It must not run `--fix`,
   commit changes, push the candidate branch, or create a verified tag for its
   own modifications.
7. Source/unit evidence cannot satisfy deployed, public-network, cross-host,
   human, custody, legal or commercial rows.

## Branch and PR discipline

- Use one lane-prefixed branch and one isolated PR for the owned package.
- Never develop directly on `main` or force-update protected refs.
- Revalidate the exact base/head/tree before each write batch.
- Do not merge your own PR.
- Request independent code-owner/security review for authority, settlement,
  protocol, migration, credential or release changes.
- Stop with an honest `BASE_DRIFT`, `BLOCKED_UPSTREAM`,
  `SERVER_CONFIGURATION_REQUIRED`, `EXTERNAL_EVIDENCE_REQUIRED` or
  `RESUME_REQUIRED` result when the next gate cannot be proven in this
  repository and environment.

## Denied scope

- `trillionnium/crates/platform/**`
- new sibling filesystem dependencies on Chain, Nakama, CEX or Integration
- production/release activation, public market enablement or truth overclaim
- fabricated, partial, stale or environment-unbound evidence

## Required local gates

Run the applicable subset before review:

```bash
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh

cd trillionnium
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```
