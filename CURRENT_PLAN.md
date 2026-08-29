# Current Trillionnium World Plan

The current executable product plan remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

The binding convergence and evidence interpretation for the current candidate is:

`docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md`

Machine-readable execution, convergence, and gap truth:

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
- **Chain** owns ingress/finality, **CEX** owns wallet/ledger settlement and custody, and **Integration** owns cross-repository component locks and release evidence.
- The existing World-local online authority is a `world_legacy_local_alpha` compatibility enclave. It must not expand into a second public authority.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. No signer/CEX/network I/O may run while mutable match or campaign rows are locked.
- CI may validate and upload evidence but must not rewrite, commit, push, tag, or promote candidate source.
- A source file or workflow definition is not remote evidence. On 2026-08-30, GitHub reported zero check runs for both V4 candidate heads, zero rulesets, and zero workflow runs for the dedicated Actions probe.
- `build.rs` still semantically rewrites the game-server, settlement-worker, and CEX client from `.rs.in` templates; directly reviewable compiled source therefore remains an open World-owned source blocker.
- The convergence branch is `fix/world-plan-v4-convergence-2026-08-30`; overlapping V4 candidates must be reconciled and superseded rather than merged as competing truth sources.

Public online and public player markets remain **NO-GO / disabled** until every dependency row in the release matrix has independently verified exact evidence. Commercial release remains **NO-GO**.
