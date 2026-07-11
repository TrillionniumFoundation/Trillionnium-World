# Trillionnium World Review First Contact RTS Data Batch - 2026-07-09

Purpose: review execution batch 3 sub-batch 4,
`first_contact_rts_data_extraction`, across the 24 First Contact data
extraction commits before continuing into RTS evidence crate ownership.

## Boundary

- Status: local review First Contact RTS data extraction sub-batch 4.
- This consumes the batch 3 runtime-boundary shard plan, the completed OpenRA
  parity/claim sub-batch review, the First Contact Basin local RTS data/runtime
  evidence, and the current release-review packet integrity artifact.
- It reviews only the 24 `first_contact_rts_data_extraction` commits from
  batch 3.
- It treats First Contact authored profiles, player-screen defaults, chrome,
  renderer-projection inputs, preview actors, samples, and labels as
  renderer-neutral data/evidence inputs. It does not move draw math, live Bevy
  renderer behavior, GPU upload proof, public launch evidence, or Android S5
  real-device evidence into `trnm-rts-data`.
- It does not stage, commit, push, rebase, reset, squash, force-push, delete,
  upload, publish, rewrite history, perform external collection, launch public
  traffic, open sockets, start hosted services, or capture Android S5
  real-device evidence.
- Do not convert this local review into renderer ownership transfer, render
  extraction completion, GPU upload, production-ready UI, public-launch,
  Android S5 real-device, beta, commercial, multi-node, live-traffic,
  hosted-service, socket, or public-network credit.

## Source Inputs

- Runtime-boundary batch 3 shard plan:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json`
- OpenRA parity/claim sub-batch 3 review:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-openra-parity-claim-batch.json`
- First Contact Basin local RTS data/runtime evidence:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json`
- Release-review packet integrity:
  `acceptance/S6_public_launch/latest/release-review-packet-integrity.json`

## Review Groups

| Group | Commit count | Sub-batch 4 conclusion |
| --- | ---: | --- |
| Data profile extraction | 6 | Terrain, opening-loop, player-startup, actor-presentation/glyph, and visual-telemetry profiles live in RTS data as authored inputs. |
| Player-screen chrome and runtime defaults | 14 | Player-screen profile, layout, chrome, command queue/cards/slots, production queues, selection health, active command, and cooldown defaults are data-owned inputs consumed by runtime adapters. |
| Renderer projection boundary | 2 | Renderer model and preview-actor projection are derived from RTS data while Bevy draw math and live renderer behavior stay renderer-owned. |
| Samples and labels readability boundary | 2 | First Contact samples and labels move to RTS data as reusable authored/readability inputs, not as player-screen proof or launch evidence. |

## Reviewed Commit Range

- First commit: `a53d815eec` / `feat: move First Contact terrain profile into RTS data`
- Last commit: `b65b4a44cd` / `refactor: move First Contact labels to RTS data`
- Reviewed commit count: `24`
- Per-commit unresolved count: `0`

## Boundary Finding

The local First Contact Basin evidence is green enough for this review slice:
`trnm-rts-data` owns the First Contact map model, terrain/opening/player
startup profiles, actor presentation/glyph profiles, visual telemetry,
player-screen profile/layout/chrome, renderer projection inputs, preview actor
projection, samples, and labels. The Bevy runtime/evidence adapters consume the
same profiles and expose equality gates for player-screen data, layout, chrome,
actor presentation, visual telemetry, renderer projection, and preview actor
projection.

The review does not grant renderer or release credit. Renderer draw math,
pixel budgets, live Bevy rendering behavior, GPU upload/extraction proof,
production-ready UI, public launch, Android S5 real-device, beta, commercial,
socket, hosted service, multi-node, and live-traffic claims all remain false.
The reviewed commits are data-boundary closure only.

## Exit Rule

Sub-batch 4 local review is complete only when the generated artifact reports
all 24 commits reviewed, zero per-commit unresolved reviews, First Contact Basin
RTS data/runtime gates green, RTS data and evidence adapter profiles aligned,
no renderer draw-math/live-renderer/public/S5/beta/commercial credit, and the
next sub-batch set to `rts_evidence_crate_boundary`.

Batch 3 remains open until every batch 3 sub-batch has commit-level review and
the unresolved runtime/data-boundary review count is zero.

## Done When

The generated artifact reports
`review_first_contact_rts_data_sub_batch_4_reviewed`,
`reviewed_commit_count=24`, `unresolved_commit_review_count=0`,
`sub_batch_4_local_review_complete=true`,
`sub_batch_4_exit_rule_satisfied=true`,
`sub_batch_5_unblocked_for_local_review=true`,
`batch_3_exit_rule_satisfied=false`, and
`batch_4_unblocked_for_local_review=false` while preserving the
public-launch/Android S5 blocker boundary.
