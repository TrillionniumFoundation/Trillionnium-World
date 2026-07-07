# Trillionnium World Review Triage Queue - 2026-07-07

Purpose: turn the review-slice manifest's unclassified and multi-slice commit
risk into an explicit local triage queue before any push, rebase, reset,
squash, history rewrite, or external/public handoff.

## Boundary

- Status: local review triage queue.
- This consumes the current review-slice manifest and git range; it does not
  rewrite, stage, commit, push, rebase, reset, squash, delete, force-push, or
  publish history.
- Triage buckets are reviewer work queues, not proof that individual commits
  are ready to merge.
- Unclassified commits are bucketed for review, but still require manual
  commit-level judgment before any external push or history operation.
- Multi-slice commits remain overlap risk until a reviewer decides their primary
  owner and whether they should stay together or be split in a later review.
- Do not convert this local queue into public-launch, Android S5 real-device,
  beta, production-ready UI, commercial, multi-node, live-traffic, or
  public-network credit.

## Inputs

- Review-slice manifest:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-slice-manifest.json`.
- Commit range: `git log --format='%H%x09%s' origin/main..HEAD`.
- File hints: `git show --name-only --format= <commit>`.

## Triage Buckets

| Bucket | Purpose |
| --- | --- |
| `unclassified_docs_plan_truth_source` | Docs/plan commits that need truth-source review before they can be routed. |
| `unclassified_generated_count_surface` | Count-exposure commits that may belong to existing artifact wrappers or guard surfaces. |
| `unclassified_classic_evidence_surface` | Classic evidence/UI/replication commits that need a primary reviewer slice. |
| `unclassified_bot_executor_surface` | Bot/executor commits that may belong to RTS runtime/data or Bevy playable review. |
| `unclassified_map_or_modeling_surface` | Map/modeling commits that need release-boundary and runtime/data ownership review. |
| `unclassified_manual_other` | Remaining unclassified commits that require manual owner assignment. |
| `multi_public_boundary_overlap` | Commits that touch public/release/external evidence boundaries plus another slice. |
| `multi_first_contact_readability_renderer_overlap` | Commits that mix First Contact readability and renderer micro-cue ownership. |
| `multi_native_bevy_rts_boundary_overlap` | Commits that overlap native Bevy and renderer-neutral RTS boundaries. |
| `multi_release_native_handoff_overlap` | Commits that overlap release truth and native Bevy handoff/review surfaces. |
| `multi_manual_overlap` | Remaining multi-slice commits that need manual primary-owner review. |

## Done When

The generated artifact reports the current base/head commits, manifest counts,
unclassified bucket coverage, multi-slice overlap bucket coverage, sample commits
per bucket, and the rule that this queue performs no external action, no history
rewrite, and grants no public/S5/beta/commercial credit.
