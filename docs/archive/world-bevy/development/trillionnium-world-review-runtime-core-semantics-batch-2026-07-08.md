# Trillionnium World Review Runtime-Core Semantics Batch - 2026-07-08

Purpose: review execution batch 3 sub-batch 1,
`runtime_core_semantics`, across the 55 OpenRA-like RTS core commits before
continuing deeper runtime/data-boundary review.

## Boundary

- Status: local review runtime-core semantics sub-batch 1.
- This consumes the batch 3 runtime-boundary shard plan, the
  `bevy-classic-rts-openra-like-core.json` local evidence, and the current
  release-review packet integrity artifact.
- It reviews only the 55 `runtime_core_semantics` commits from batch 3.
- It does not mark batch 3 complete and does not unblock batch 4.
- It does not stage, commit, push, rebase, reset, squash, force-push, delete,
  upload, publish, rewrite history, perform external collection, launch public
  traffic, or capture Android S5 real-device evidence.
- Do not convert this local review into OpenRA runtime compatibility, OpenRA
  replay/network compatibility, public-launch, Android S5 real-device, beta,
  production-ready UI, commercial, multi-node, live-traffic, or public-network
  credit.

## Source Inputs

- Runtime-boundary batch 3 shard plan:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json`
- Local OpenRA-like RTS core evidence:
  `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-like-core.json`
- Release-review packet integrity:
  `acceptance/S6_public_launch/latest/release-review-packet-integrity.json`

## Review Groups

| Group | Commit count | Sub-batch 1 conclusion |
| --- | ---: | --- |
| World model, shroud, and command resolver | 3 | The local map/rule/order/shroud command model is evidence-green and Trillionnium-owned, but final runtime-core source ownership remains a boundary follow-up. |
| Economy, build, and production semantics | 13 | Local economy/build/production gates are evidence-green; they remain local semantics evidence, not public or compatibility proof. |
| Combat, repair, stance, and targeting | 8 | Combat and unit-order semantics are evidence-green and no OpenRA engine code is copied. |
| Movement, pathing, and order queue | 11 | Pathfinding, queue, formation, traffic, and obstruction semantics are evidence-green but still live in the current Bevy-world evidence surface. |
| Objective capture semantics | 2 | Capture/contested capture semantics are evidence-green and remain local First Contact evidence only. |
| Control-group lifecycle | 18 | Control-group assignment, pruning, validation, recall, rebuild, and formation queue semantics are evidence-green. |

## Reviewed Commit Range

- First commit: `aafb959a79` / `feat: add openra-like rts core`
- Last commit: `1d86e9865d` / `feat: add openra-like control group recall formation queue core`
- Reviewed commit count: `55`
- Per-commit unresolved count: `0`

## Boundary Finding

The local semantic evidence is strong: it exercises the First Contact RTS core
with 75 green gates, 13 order kinds, 11 rules, 341 deterministic ticks, accepted
and rejected commands, harvesting, production, pathing, combat, repair,
capture, shroud memory, and control groups.

The remaining blocker is a systemic runtime-core source boundary follow-up
rather than a per-commit unresolved review: the reviewed OpenRA-like core
semantics still route through
`trillionnium/crates/trnm-world-bevy/src/lib.rs` and the
`classic-rts-openra-like-core` local evidence surface. That is acceptable for a
local review artifact, but it is not enough to close the whole runtime/data
boundary. Later batch 3 work must continue through adapter/data extraction
review before batch 3 can claim the runtime core boundary closed.

## Exit Rule

Sub-batch 1 local review is complete only when the generated artifact reports
all 55 commits reviewed, zero per-commit unresolved reviews, local
OpenRA-like-core evidence green, source-policy claims safe, no public/S5/beta
or OpenRA compatibility credit, and exactly one systemic runtime-core source
boundary follow-up.

Batch 3 remains open until the systemic follow-up is resolved and every batch 3
sub-batch has commit-level review.

## Done When

The generated artifact reports
`review_runtime_core_semantics_sub_batch_1_reviewed_with_boundary_followup`,
`reviewed_commit_count=55`, `unresolved_commit_review_count=0`,
`systemic_runtime_core_boundary_followup_count=1`,
`sub_batch_1_local_review_complete=true`,
`sub_batch_1_exit_rule_satisfied=false`,
`batch_3_exit_rule_satisfied=false`, and
`batch_4_unblocked_for_local_review=false` while preserving the
public-launch/Android S5 blocker boundary.
