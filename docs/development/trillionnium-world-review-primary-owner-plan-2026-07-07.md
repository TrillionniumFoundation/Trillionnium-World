# Trillionnium World Review Primary-Owner Plan - 2026-07-07

Purpose: turn the review triage queue into a bucket-level primary-owner plan
before any push, rebase, reset, squash, history rewrite, or external handoff.

## Boundary

- Status: local review primary-owner plan.
- This consumes the current review triage queue only; it does not rewrite,
  stage, commit, push, rebase, reset, squash, delete, force-push, upload, or
  publish history.
- Bucket primary owners are review routing defaults, not proof that individual
  commits are ready to merge.
- Multi-slice and manual buckets still need commit-level reviewer judgment
  before any external push or history operation.
- Do not convert this local owner plan into public-launch, Android S5
  real-device, beta, production-ready UI, commercial, multi-node, live-traffic,
  or public-network credit.

## Inputs

- Review triage queue:
  `acceptance/S6_public_launch/latest/trillionnium-world-review-triage-queue.json`.
- Review triage queue doc:
  `docs/development/trillionnium-world-review-triage-queue-2026-07-07.md`.

## Owner Routing

| Bucket | Primary owner | Review order | Exit rule |
| --- | --- | ---: | --- |
| `multi_public_boundary_overlap` | `release_truth_and_public_boundary` | 1 | No-credit and public/S5 boundaries must be reviewed before product/runtime details. |
| `multi_release_native_handoff_overlap` | `release_truth_and_public_boundary` | 2 | Release truth and handoff language must be resolved before Bevy client review. |
| `multi_native_bevy_rts_boundary_overlap` | `rts_runtime_data_boundaries` | 3 | Renderer-neutral RTS contracts must be reviewed before Bevy draw/runtime integration. |
| `unclassified_generated_count_surface` | `release_truth_and_public_boundary` | 4 | Each count exposure must have an owning artifact/checker. |
| `unclassified_docs_plan_truth_source` | `release_truth_and_public_boundary` | 5 | Each doc must be confirmed as current truth or routed to archive/reference-only. |
| `unclassified_bot_executor_surface` | `rts_runtime_data_boundaries` | 6 | Bot/executor changes must be assigned to runtime/data, Bevy integration, or release evidence. |
| `unclassified_classic_evidence_surface` | `native_bevy_playable_client` | 7 | Classic evidence surfaces must be routed to playable-client, renderer, or release-truth review. |
| `unclassified_map_or_modeling_surface` | `rts_runtime_data_boundaries` | 8 | Map/modeling changes must prove no live-ingestion or public map-pack credit is implied. |
| `multi_first_contact_readability_renderer_overlap` | `first_contact_product_readability` | 9 | Human-playtest/product readability owns the first pass before renderer micro-cue ownership. |
| `unclassified_manual_other` | `manual_triage_required` | 10 | Read each commit and assign a primary reviewer slice manually. |
| `multi_manual_overlap` | `manual_triage_required` | 11 | Read each overlap and decide primary owner or later split strategy manually. |

## Done When

The generated artifact reports all 11 triage buckets with a bucket-level primary
owner, review order, exit rule, source commit counts, remaining commit-level
review count, and the rule that this plan performs no external action, no
history rewrite, and grants no public/S5/beta/commercial credit.
