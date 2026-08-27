# Project Boundary (binding)

This Git root is **Trillionnium World** (`trillionnium-world`), lane
`game-product`. Before any write, build, commit, branch, remote or dependency
change, run:

```bash
bash scripts/project-preflight.sh
```

Stop on a project ID, lane, remote, branch, topic or dependency mismatch. The
repository may be checked out at any operator-approved path; do not encode a
personal home directory in source, docs, units or scripts.

Read these current truth sources before development:

1. `PROJECT_BOUNDARY.md`
2. `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md`
3. `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`
4. `docs/adr/0002-transaction-free-external-settlement.md`

Binding ownership rule:

- World owns deterministic game rules, simulation, outcomes, unsigned replay
  material, the native client and player-facing economy intents.
- Nakama owns canonical online participant admission, event order, command
  idempotency, restart recovery, archive roots and `MatchCompletedV1` signing.
- Chain owns ingress/finality, CEX owns wallet settlement, and Integration owns
  cross-repository evidence locks.

`trillionnium/crates/trnm-game-server` is a compatibility authority enclave
pending the World-to-Nakama adapter. Do not add a new public authority protocol,
Nakama private-key surface, canonical match-completion signature, direct Chain
research command or public/cross-host authority claim there.

No external signer/CEX/network call may run while a mutable match or campaign
PostgreSQL transaction holds row locks. New settlement work must follow the
outbox contract and ADR-0002. Until WORLD-P0-001 removes the registered legacy
path, CI must prove that debt remains confined to its one reviewed location.

The excluded `trillionnium/crates/platform` tree is legacy material and is not
an active workspace. New sibling filesystem dependencies on Chain, Nakama, CEX
or Integration are forbidden; use published or exact-revision contracts.

Before review, run the applicable gates:

```bash
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh
```
