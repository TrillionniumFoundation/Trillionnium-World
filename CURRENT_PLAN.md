# Current Trillionnium World Plan

The current executable development plan is:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md`

Machine-readable plan and gap truth:

- `docs/development/trillionnium-world-development-plan-v4-2026-08-29.json`
- `docs/status/world-gap-registry-v2.json`
- `docs/status/CURRENT.md`

Binding architecture decisions:

- `docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`
- `docs/adr/0002-transaction-free-external-settlement.md`
- `docs/adr/0003-reviewable-source-and-non-self-modifying-ci.md`

The target architecture has one accountable owner for every canonical cursor,
root, receipt and signature. World owns deterministic game-domain behavior;
Nakama owns target online authority; Chain owns finality; CEX owns wallet
settlement; Integration owns cross-repository release locks.

Public online, public player markets, custody and commercial-release claims
remain disabled until their distinct machine, deployed, human and approval
evidence rows are explicitly green.
