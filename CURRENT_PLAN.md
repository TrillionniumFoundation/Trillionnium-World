# Current Trillionnium World Plan

The current executable plan is:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

Machine-readable execution and gap ledger:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`

Current architecture decisions:

- `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`
- `docs/adr/0002-transaction-free-external-settlement.md`

Current status and release denominators:

- `docs/status/CURRENT.md`
- `docs/status/world-gates-v1.json`
- `docs/release/trnm-world-release-gate-matrix-v2.md`

## Binding decisions

- **World** owns deterministic game-domain behavior, authored content, the native client, player-facing economy intents, World outcome hashes, and unsigned replay/outcome material.
- **Nakama** owns target online admission, canonical total order, idempotency, restart recovery, archive roots, and `MatchCompletedV1` signing.
- **Chain** owns ingress/finality, **CEX** owns wallet/ledger settlement and custody, and **Integration** owns cross-repository component locks and release evidence.
- The existing World-local online authority is a `world_legacy_local_alpha` compatibility enclave. It must not expand into a second public authority.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. No signer/CEX/network I/O may run while mutable match or campaign rows are locked.
- CI may validate and upload evidence but must not rewrite, commit, push, tag, or promote candidate source.

Public online and public player markets remain **NO-GO / disabled** until every dependency row in the release matrix has independently verified exact evidence.