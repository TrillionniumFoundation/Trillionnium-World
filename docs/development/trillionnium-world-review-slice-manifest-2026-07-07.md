# Trillionnium World Review Slice Manifest - 2026-07-07

Purpose: bind the large local `origin/main..HEAD` backlog to the six review
slices before any external push, rebase, reset, squash, or public handoff.

## Boundary

- Status: local review-slice commit-range manifest.
- This is a read-only manifest over the current git range; it does not rewrite,
  stage, commit, push, rebase, reset, squash, delete, or force-push history.
- Slice matches are routing hints for review, not a replacement for reviewer
  judgment on every commit.
- Unclassified commits remain manual-review risk; do not hide them or treat
  this manifest as a complete squash plan.
- Do not convert this local manifest into public-launch, Android S5
  real-device, beta, production-ready UI, commercial, multi-node, live-traffic,
  or public-network credit.

## Source Range

- Base: `origin/main`.
- Head: local `HEAD`.
- Commit list: `git log --format='%H%x09%s' origin/main..HEAD`.
- File hints: `git show --name-only --format= <commit>`.

## Slice Routing Rules

| Slice ID | Match Hints |
| --- | --- |
| `release_truth_and_public_boundary` | Release packet, readiness, signoff, public-launch, no-credit, blocker, evidence, README, and release-review guard work. |
| `native_bevy_playable_client` | Native Bevy runner, playtest, live screenshot, texture/render, action coach, player HUD, and S5 host-side playable evidence. |
| `first_contact_product_readability` | Whole-screen First Contact readability review, human playtest path, observation log, observer runbook, active path, queue, objective, and blocked-route work. |
| `first_contact_renderer_micro_cues` | First Contact renderer passes that shrink, mute, suppress, stagger, taper, or otherwise turn status-like bars/rings/glints/trails into micro cues. |
| `rts_runtime_data_boundaries` | Renderer-neutral RTS data/runtime/evidence crates, adapter contracts, asset-boundary, model-catalog, OpenRA boundary, and simulation/data separation work. |
| `external_evidence_collection_blockers` | S5 real device, production map-pack, beta cohort, commercial drill, multi-node/live-traffic latency, public-network exposure, and evidence collection/validation blockers. |

## Done When

The generated artifact reports the current base/head commits, total ahead count,
six slice summaries, sample commits per slice, multi-slice commit count,
unclassified commit count, and the rule that this manifest performs no external
action, no history rewrite, and grants no public/S5/beta/commercial credit.
