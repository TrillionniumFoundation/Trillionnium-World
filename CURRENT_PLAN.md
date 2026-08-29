# Current Trillionnium World Plan

The current executable product plan remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

Binding convergence and current execution interpretation:

- `docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md`
- `docs/development/TRILLIONNIUM_WORLD_PLAN_V4_EXECUTION_UPDATE_2026-08-30.md`

Machine-readable execution and gap truth:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`
- `docs/status/world-v4-convergence-state-2026-08-30.json`

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
- **Chain** owns ingress/finality, **CEX** owns wallet/ledger settlement and custody, and **Integration** owns exact cross-repository component locks and release evidence.
- The World-local online server is a `world_legacy_local_alpha` compatibility enclave and must not expand into a second canonical public authority.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer/CEX/network I/O under mutable match or campaign locks is prohibited.
- CI may validate and upload evidence but may not rewrite, commit, push, tag, merge, or promote candidate source.
- Source, workflow, CODEOWNERS, or ruleset documentation is not remote/server evidence.
- The CEX transport and settlement worker are directly compiled source.
- `build.rs` still semantically rewrites only the game-server library from `src/lib.rs.in`; `WORLD-P0-009` remains `source_open` until that final transform is removed and exact-head tests pass.
- `WORLD-P1-001` remains `source_open` until correctness-critical module ownership and invariant tests replace the catch-all library boundary.
- PR `#39` on `fix/world-plan-v4-convergence-2026-08-30` is the single current V4 truth-source candidate. It must remain independently reviewed and must not be self-merged.

Public online and public player markets remain **NO-GO / disabled** until every dependency row in the release matrix has independently verified exact evidence. Commercial release remains **NO-GO**.
