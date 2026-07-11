# Trillionnium World Next Execution Plan v1

> 2026-07-10 status: superseded for product implementation by
> `trillionnium-rpg-rts-closed-loop-v1.md`. This July 9 review/evidence queue is
> retained as audit history; it must not route the default client back to
> `trnm-world-bevy` or treat pre-reset packet counts as closure evidence.

Purpose: keep the post-audit execution stream focused on the highest-return
work after the local release-review packet became green with public-launch
blockers.

## Current Truth

- Native playable client: `trillionnium/crates/trnm-first-contact`.
- Campaign authority: `trillionnium/crates/trnm-campaign-core`.
- Battle authority: `trillionnium/crates/trnm-rts-sim`.
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
     `docs/archive/world-review-2026-07/trillionnium-world-first-contact-readability-review-2026-07-07.md`
2. Human playtest path:
   - start campaign
   - select units
   - secure beacon
   - read command queue
   - recover from blocked route
   - observation log:
     `docs/archive/world-review-2026-07/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md`
   - observer runbook:
     `docs/archive/world-review-2026-07/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md`
   - packet binding: `bevy-classic-playtest-handoff-packet` must carry the
     five-step local task path without beta, S5, or public-launch credit
3. Truth-source hygiene:
   - keep packet artifact counts synchronized
   - keep local-review and public-launch wording separate
   - keep S5 real-device claims false without device evidence
   - evidence-volume curation:
     `docs/archive/world-review-2026-07/trillionnium-world-evidence-volume-curation-2026-07-07.md`
   - reviewer handoff index:
     `docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md`
4. Commit and review strategy:
   - group the local backlog into reviewable slices
   - review-slice strategy:
     `docs/archive/world-review-2026-07/trillionnium-world-review-slice-strategy-2026-07-07.md`
   - review-slice manifest:
     `docs/archive/world-review-2026-07/trillionnium-world-review-slice-manifest-2026-07-07.md`
   - review triage queue:
     `docs/archive/world-review-2026-07/trillionnium-world-review-triage-queue-2026-07-07.md`
   - review primary-owner plan:
     `docs/archive/world-review-2026-07/trillionnium-world-review-primary-owner-plan-2026-07-07.md`
   - release/public-boundary owner queue:
     `docs/archive/world-review-2026-07/trillionnium-world-review-release-owner-queue-2026-07-07.md`
   - RTS runtime/data-boundary owner queue:
     `docs/archive/world-review-2026-07/trillionnium-world-review-runtime-owner-queue-2026-07-07.md`
   - residual owner-resolution queue:
     `docs/archive/world-review-2026-07/trillionnium-world-review-residual-queue-2026-07-08.md`
   - review execution batches:
     `docs/archive/world-review-2026-07/trillionnium-world-review-execution-batches-2026-07-08.md`
   - public-boundary batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-public-boundary-batch-2026-07-08.md`
   - release-native handoff batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-release-native-handoff-batch-2026-07-08.md`
   - runtime-boundary batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-runtime-boundary-batch-2026-07-08.md`
   - runtime-core semantics batch review:
     `docs/development/trillionnium-world-review-runtime-core-semantics-batch-2026-07-08.md`
   - runtime-adapter/online batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-runtime-adapter-online-batch-2026-07-08.md`
   - OpenRA parity/claim batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-openra-parity-claim-batch-2026-07-08.md`
   - First Contact RTS data batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-first-contact-rts-data-batch-2026-07-09.md`
   - RTS evidence crate batch review:
     `docs/development/trillionnium-world-review-rts-evidence-crate-batch-2026-07-09.md`
   - review evidence exposure batch:
     `docs/archive/world-review-2026-07/trillionnium-world-review-evidence-exposure-batch-2026-07-09.md`
   - Bevy runtime renderer batch review:
     `docs/development/trillionnium-world-review-bevy-runtime-renderer-batch-2026-07-09.md`
   - First Contact player-surface cues batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-first-contact-player-surface-cues-batch-2026-07-09.md`
   - generated count surface batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-generated-count-surface-batch-2026-07-09.md`
   - docs/plan truth-source batch review:
     `docs/development/trillionnium-world-review-docs-plan-truth-source-batch-2026-07-09.md`
   - bot/executor surface batch review:
     `docs/archive/world-review-2026-07/trillionnium-world-review-bot-executor-surface-batch-2026-07-09.md`
   - avoid external push/public actions until explicitly routed
5. Public launch evidence:
   - S5 real-device matrix
   - production map-pack public evidence
   - first beta cohort evidence
   - commercial launch drill evidence
   - multi-node or live-traffic latency evidence
   - public-network live exposure evidence
   - blocker execution ledger:
     `docs/archive/world-review-2026-07/trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md`

## Operating Rule

For local work, prefer product-quality improvements and truth-source guards. Do not keep shrinking already-gated micro cues unless a fresh screenshot-visible issue proves the cue is still harming readability.
