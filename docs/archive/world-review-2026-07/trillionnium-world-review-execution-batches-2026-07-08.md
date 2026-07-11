# Trillionnium World Review Execution Batches - 2026-07-08

Purpose: turn the release/runtime/residual owner queues into one ordered local
execution plan for reviewer batches before any push, history operation, or
external handoff.

## Boundary

- Status: local review execution batches.
- This consumes the current review primary-owner plan plus the release,
  runtime, and residual owner queues.
- The release, runtime, and residual owner queues remain the source of batch
  membership.
- It does not stage, commit, push, rebase, reset, squash, force-push, delete,
  upload, publish, or rewrite history.
- It is a local reviewer execution order only; it is not proof that any batch is
  ready to merge.
- Do not convert these local batches into public-launch, Android S5
  real-device, beta, production-ready UI, commercial, multi-node,
  live-traffic, or public-network credit.

## Source Queues

- Release/public-boundary owner queue:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json`
- RTS runtime/data-boundary owner queue:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json`
- Residual owner-resolution queue:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-residual-queue.json`
- Primary-owner plan:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json`

## Execution Order

| Batch | Bucket | Source queue | Gate before moving on |
| ---: | --- | --- | --- |
| 1 | `multi_public_boundary_overlap` | `review_release_owner_queue` | Public/S5/no-credit boundaries reviewed first. |
| 2 | `multi_release_native_handoff_overlap` | `review_release_owner_queue` | Release handoff wording reviewed before playable-client details. |
| 3 | `multi_native_bevy_rts_boundary_overlap` | `review_runtime_owner_queue` | Renderer-neutral RTS ownership reviewed before Bevy/runtime integration. |
| 4 | `unclassified_generated_count_surface` | `review_release_owner_queue` | Count surfaces tied to owning checker/artifact contracts. |
| 5 | `unclassified_docs_plan_truth_source` | `review_release_owner_queue` | Docs confirmed current truth or routed to archive/reference-only. |
| 6 | `unclassified_bot_executor_surface` | `review_runtime_owner_queue` | Bot/executor changes assigned to runtime/data, Bevy, or release evidence. |
| 7 | `unclassified_classic_evidence_surface` | `review_residual_queue` | Classic evidence surfaces routed to playable, renderer, or release review. |
| 8 | `unclassified_map_or_modeling_surface` | `review_runtime_owner_queue` | Reserved map/modeling lane remains explicit even at zero count. |
| 9 | `multi_first_contact_readability_renderer_overlap` | `review_residual_queue` | Reserved readability/renderer lane remains human-playtest-first. |
| 10 | `unclassified_manual_other` | `review_residual_queue` | Manual commits receive a primary reviewer slice. |
| 11 | `multi_manual_overlap` | `review_residual_queue` | Manual overlaps receive a primary owner or later split strategy. |

## Done When

The generated artifact reports all 11 owner batches in review order, ties each
batch back to its source queue, confirms queue counts match the primary-owner
plan, records nonempty versus reserved zero-count batches, exposes the first
batch to review, and preserves the no-push/no-history-rewrite/no-public-credit
boundary.
