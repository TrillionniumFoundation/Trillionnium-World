# Trillionnium World Review Bevy Runtime Renderer Batch - 2026-07-09

Purpose: review execution batch 3 sub-batch 7,
`bevy_runtime_renderer_boundary`, across the 7 Bevy runtime/renderer boundary
commits before continuing into the First Contact player-surface cue batch.

## Boundary

- Status: local review Bevy runtime renderer boundary sub-batch 7.
- This consumes the batch 3 runtime-boundary shard plan, the completed local
  review-evidence exposure sub-batch, the current First Contact Basin runtime
  adapter artifact, the current classic model catalog and asset-boundary
  artifacts, and the current release-review packet integrity artifact.
- It reviews only the 7 `bevy_runtime_renderer_boundary` commits from batch 3.
- Bevy runtime and renderer splits may move player-screen runtime adapter
  helpers into `trnm-rts-bevy-runtime` and renderer helpers into
  `trnm-world-bevy` modules. They do not make Bevy renderer files the RTS data
  truth source, do not transfer live renderer behavior into data/evidence
  crates, and do not prove render-world extraction or GPU upload.
- It does not stage, commit, push, rebase, reset, squash, force-push, delete,
  upload, publish, rewrite history, perform external collection, launch public
  traffic, open sockets, start hosted services, or capture Android S5
  real-device evidence.
- Do not convert this local Bevy runtime/renderer boundary review into
  public-launch, Android S5 real-device, beta, commercial, production-ready UI,
  multi-node, live-traffic, hosted-service, socket, OpenRA runtime
  compatibility, render-world extraction, GPU upload, external evidence, or
  public-network credit.

## Source Inputs

- Runtime-boundary batch 3 shard plan:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json`
- Review evidence exposure sub-batch 6 review:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-evidence-exposure-batch.json`
- First Contact Basin local Bevy runtime adapter:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json`
- Classic model catalog:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json`
- Classic asset boundary artifacts:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-asset-slot-map.json`,
  `acceptance/S5_native_bevy_device/latest/bevy-classic-asset-pack.json`, and
  `acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json`
- Release-review packet integrity:
  `acceptance/S6_public_launch/latest/release-review-packet-integrity.json`

## Review Groups

| Group | Commit count | Sub-batch 7 conclusion |
| --- | ---: | --- |
| Classic model catalog gate | 1 | The model catalog gate renders project-owned manifest frames through the low-spec Bevy renderer path without wgpu, S5, public, or external art credit. |
| First Contact Bevy runtime modules | 4 | Tile, readout, command-grid, and bottom-panel runtime helpers live in `trnm-rts-bevy-runtime` as adapter/consumer surfaces over RTS data/evidence. |
| Classic renderer split modules | 2 | Model-catalog and asset-boundary renderer splits stay in `trnm-world-bevy`, keeping renderer draw helpers out of RTS data truth. |

## Reviewed Commit Range

- First commit: `b04ab8807f` / `test: gate classic model catalog`
- Last commit: `69c16cc223` / `refactor: split classic asset boundary renderer`
- Reviewed commit count: `7`
- Per-commit unresolved count: `0`

## Boundary Finding

The reviewed commits keep the Bevy side as consumer/adaptor/renderer surface:
First Contact runtime helper code is isolated in `trnm-rts-bevy-runtime`, while
classic model-catalog and asset-boundary renderer helpers remain in
`trnm-world-bevy`. The current First Contact Basin artifact still reports the
Bevy runtime adapter, map projection, player-screen application, command-grid,
bottom-panel, player label, data-renderer projection, and RTS data consumer
gates green. The classic model catalog, asset slot map, asset pack, and
manifest lint artifacts still mark the project-owned low-spec renderer boundary
green without CEX runtime, wgpu, copied asset, public-launch, or Android S5
credit.

The review does not grant release or external evidence credit. Public launch,
Android S5 real-device, beta, commercial, production-ready UI, socket, hosted
service, live multiplayer, live-traffic, public-network, render-world
extraction, GPU upload, OpenRA runtime/replay/network compatibility, and
external evidence claims all remain false.

## Exit Rule

Sub-batch 7 local review is complete only when the generated artifact reports
all 7 commits reviewed, zero per-commit unresolved reviews, the prior
review-evidence exposure sub-batch closed, Bevy runtime adapter gates green,
classic model catalog and asset-boundary renderer gates green, no Bevy renderer
ownership of RTS data truth, no render-world/GPU/public/S5/beta/commercial
credit, and the next sub-batch set to `first_contact_player_surface_cues`.

Batch 3 remains open until every batch 3 sub-batch has commit-level review and
the unresolved runtime/data-boundary review count is zero.

## Done When

The generated artifact reports
`review_bevy_runtime_renderer_sub_batch_7_reviewed`,
`reviewed_commit_count=7`, `unresolved_commit_review_count=0`,
`sub_batch_7_local_review_complete=true`,
`sub_batch_7_exit_rule_satisfied=true`,
`sub_batch_8_unblocked_for_local_review=true`,
`batch_3_reviewed_commit_count=210`,
`batch_3_remaining_commit_level_review_count=63`,
`batch_3_exit_rule_satisfied=false`, and
`batch_4_unblocked_for_local_review=false` while preserving the
public-launch/Android S5 blocker boundary.
