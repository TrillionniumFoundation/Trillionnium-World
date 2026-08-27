# Current Trillionnium World Plan

The current executable development plan is:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md`

Machine-readable backlog and gates:

`docs/development/trillionnium-world-development-plan-2026-08-27.json`

The current architecture decision is:

`docs/adr/0001-realtime-authority-and-match-evidence-ownership.md`

Key decision: Nakama is the canonical online match/evidence authority. World
owns deterministic game rules, simulation, outcomes and unsigned game-domain
material. The existing World-local online authority is a compatibility enclave
pending adapter migration and must not expand into a second public authority.

Public online, public player market and commercial-release claims remain gated
by the exact evidence rows described in the plan and `GAME_STATUS.md`.
