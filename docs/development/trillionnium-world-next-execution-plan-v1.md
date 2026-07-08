# Trillionnium World Next Execution Plan v1

Purpose: keep the post-audit execution stream focused on the highest-return
work after the local release-review packet became green with public-launch
blockers.

## Current Truth

- Native playable client: `trillionnium/crates/trnm-world-bevy`.
- CEX role: legacy adapter/evidence reference, not the product client.
- Local review state: green with public-launch blockers.
- Public launch state: blocked until real external evidence exists.
- Android S5 real-device state: unclaimed until device evidence is collected.

## Highest Risks

1. Local commit backlog: `main` is hundreds of commits ahead of `origin/main`.
2. External evidence gap: six public-launch blockers still require real evidence.
3. Documentation drift: artifact counts and readiness dates can lag the packet.
4. Evidence volume: S5/Bevy acceptance evidence is large and needs handoff curation.
5. Product readability: First Contact is playable, but central battlefield clarity
   needs whole-screen product work rather than more isolated color shaving.

## Next Work Queue

1. Whole-screen First Contact readability review:
   - unit silhouettes
   - building hierarchy
   - terrain/material grouping
   - objective focus
   - combat flow
   - current snapshot review:
     `docs/development/trillionnium-world-first-contact-readability-review-2026-07-07.md`
2. Human playtest path:
   - start campaign
   - select units
   - secure beacon
   - read command queue
   - recover from blocked route
   - observation log:
     `docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md`
   - observer runbook:
     `docs/development/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md`
   - packet binding: `bevy-classic-playtest-handoff-packet` must carry the
     five-step local task path without beta, S5, or public-launch credit
3. Truth-source hygiene:
   - keep packet artifact counts synchronized
   - keep local-review and public-launch wording separate
   - keep S5 real-device claims false without device evidence
   - evidence-volume curation:
     `docs/development/trillionnium-world-evidence-volume-curation-2026-07-07.md`
   - reviewer handoff index:
     `docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md`
4. Commit and review strategy:
   - group the local backlog into reviewable slices
   - review-slice strategy:
     `docs/development/trillionnium-world-review-slice-strategy-2026-07-07.md`
   - review-slice manifest:
     `docs/development/trillionnium-world-review-slice-manifest-2026-07-07.md`
   - review triage queue:
     `docs/development/trillionnium-world-review-triage-queue-2026-07-07.md`
   - review primary-owner plan:
     `docs/development/trillionnium-world-review-primary-owner-plan-2026-07-07.md`
   - release/public-boundary owner queue:
     `docs/development/trillionnium-world-review-release-owner-queue-2026-07-07.md`
   - RTS runtime/data-boundary owner queue:
     `docs/development/trillionnium-world-review-runtime-owner-queue-2026-07-07.md`
   - avoid external push/public actions until explicitly routed
5. Public launch evidence:
   - S5 real-device matrix
   - production map-pack public evidence
   - first beta cohort evidence
   - commercial launch drill evidence
   - multi-node or live-traffic latency evidence
   - public-network live exposure evidence
   - blocker execution ledger:
     `docs/development/trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md`

## Operating Rule

For local work, prefer product-quality improvements and truth-source guards. Do not keep shrinking already-gated micro cues unless a fresh screenshot-visible issue proves the cue is still harming readability.
