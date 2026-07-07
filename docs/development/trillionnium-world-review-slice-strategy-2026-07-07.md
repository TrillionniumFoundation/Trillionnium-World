# Trillionnium World Review Slice Strategy - 2026-07-07

Purpose: turn the large local `main...origin/main` backlog into reviewable
topics before any external push or public handoff.

## Boundary

- Status: local review-slice strategy.
- This is a grouping plan over existing local commits, not a history rewrite.
- Do not push, rebase, force-push, reset, squash, or delete commits from this
  plan alone.
- Do not convert local review readiness into public-launch, Android S5
  real-device, beta, production-ready UI, commercial, multi-node, or
  public-network credit.

## Current Backlog Risk

The local branch is hundreds of commits ahead of `origin/main`. Even when all
local gates are green, this is still a review and integration risk because
product, evidence, renderer, and release-boundary work are mixed in one long
linear history.

## Review Slices

| Slice ID | Review Topic | Primary Question | Do Not Mix With |
| --- | --- | --- | --- |
| `release_truth_and_public_boundary` | Release-review packet, public-launch blockers, no-credit evidence validators, readiness docs. | Does every public/S5/beta/commercial claim stay tied to real evidence? | Renderer tuning or product art changes. |
| `native_bevy_playable_client` | Bevy runner, playtest launcher, handoff packet, live screenshot, desktop review packet, local playable gates. | Can the local native client be reviewed and replayed without CEX as the product client? | External launch evidence or mobile S5 claims. |
| `first_contact_product_readability` | Whole-screen First Contact readability, human playtest path, observation log, observer runbook. | Can a reviewer understand selected group, objective, queue, and blocked route? | More isolated exact-color shaving without fresh observation. |
| `first_contact_renderer_micro_cues` | Prior focused renderer passes that shrank status-like bars, rings, trails, glints, and structure/unit accents. | Are the micro-cue gates preserving product readability without changing simulation? | Runtime-core/gameplay/network changes. |
| `rts_runtime_data_boundaries` | Renderer-neutral data, online/offline adapter contracts, runtime/evidence split, no OpenRA-copy boundary. | Are simulation/data contracts independent from Bevy draw math and proprietary assets? | Pixel-art-only review or public launch operations. |
| `external_evidence_collection_blockers` | S5 real device, production map-pack, beta cohort, commercial drill, multi-node/live latency, public exposure. | What real non-template artifacts are still missing? | Local host-side proof or placeholder templates. |

## Ordering

1. Review `release_truth_and_public_boundary` first so all later slices inherit
   honest public/S5 boundaries.
2. Review `native_bevy_playable_client` before product art tuning so reviewers
   agree on the playable client and local runner.
3. Review `first_contact_product_readability` before more renderer work.
4. Review `first_contact_renderer_micro_cues` as focused proof that visual
   cleanup did not mutate runtime behavior.
5. Review `rts_runtime_data_boundaries` before any broader architecture claims.
6. Keep `external_evidence_collection_blockers` blocked until real evidence
   exists.

## Done When

The next-plan artifact can report the current ahead count, clean/dirty state,
six review slices, and the rule that this strategy performs no external action
and grants no public/S5/beta/commercial credit.
