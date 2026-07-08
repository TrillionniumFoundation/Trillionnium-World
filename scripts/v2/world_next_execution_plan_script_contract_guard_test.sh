#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_next_execution_plan.sh"
OBSERVATION_LOG_SCRIPT="$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_observation_log.sh"
RUNBOOK_SCRIPT="$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_runbook.sh"
EVIDENCE_VOLUME_SCRIPT="$ROOT/scripts/check_trillionnium_world_evidence_volume_curation.sh"
REVIEWER_HANDOFF_SCRIPT="$ROOT/scripts/check_trillionnium_world_reviewer_handoff_index.sh"
REVIEW_SLICE_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_slice_strategy.sh"
REVIEW_SLICE_MANIFEST_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_slice_manifest.sh"
REVIEW_TRIAGE_QUEUE_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_triage_queue.sh"
REVIEW_PRIMARY_OWNER_PLAN_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_primary_owner_plan.sh"
REVIEW_RELEASE_OWNER_QUEUE_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh"
REVIEW_RUNTIME_OWNER_QUEUE_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh"
REVIEW_RESIDUAL_QUEUE_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_residual_queue.sh"
REVIEW_EXECUTION_BATCHES_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh"
REVIEW_PUBLIC_BOUNDARY_BATCH_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_public_boundary_batch.sh"
REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_release_native_handoff_batch.sh"
REVIEW_RUNTIME_BOUNDARY_BATCH_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh"
BLOCKER_LEDGER_SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_blocker_execution_ledger.sh"
DOC="$ROOT/docs/development/trillionnium-world-next-execution-plan-v1.md"
READABILITY_REVIEW_DOC="$ROOT/docs/development/trillionnium-world-first-contact-readability-review-2026-07-07.md"
PLAYTEST_OBSERVATION_LOG_DOC="$ROOT/docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
PLAYTEST_RUNBOOK_DOC="$ROOT/docs/development/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
EVIDENCE_VOLUME_DOC="$ROOT/docs/development/trillionnium-world-evidence-volume-curation-2026-07-07.md"
REVIEWER_HANDOFF_DOC="$ROOT/docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md"
REVIEW_SLICE_DOC="$ROOT/docs/development/trillionnium-world-review-slice-strategy-2026-07-07.md"
REVIEW_SLICE_MANIFEST_DOC="$ROOT/docs/development/trillionnium-world-review-slice-manifest-2026-07-07.md"
REVIEW_TRIAGE_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-triage-queue-2026-07-07.md"
REVIEW_PRIMARY_OWNER_PLAN_DOC="$ROOT/docs/development/trillionnium-world-review-primary-owner-plan-2026-07-07.md"
REVIEW_RELEASE_OWNER_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-release-owner-queue-2026-07-07.md"
REVIEW_RUNTIME_OWNER_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-runtime-owner-queue-2026-07-07.md"
REVIEW_RESIDUAL_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-residual-queue-2026-07-08.md"
REVIEW_EXECUTION_BATCHES_DOC="$ROOT/docs/development/trillionnium-world-review-execution-batches-2026-07-08.md"
REVIEW_PUBLIC_BOUNDARY_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-public-boundary-batch-2026-07-08.md"
REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-release-native-handoff-batch-2026-07-08.md"
REVIEW_RUNTIME_BOUNDARY_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-runtime-boundary-batch-2026-07-08.md"
BLOCKER_LEDGER_DOC="$ROOT/docs/development/trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md"

required_script_lines=(
  'trillionnium_world_next_execution_plan_v1'
  'release_review_packet_integrity_green_with_public_launch_blockers'
  'next_execution_plan_green_with_public_launch_blockers'
  'public_launch_blockers_preserved'
  'whole_screen_first_contact_readability'
  'trillionnium-world-first-contact-readability-review-2026-07-07.md'
  'readability_review'
  'trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md'
  'human_playtest_observation'
  'first-contact-human-playtest-observation-log.json'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
  'human_playtest_runbook'
  'first-contact-human-playtest-runbook.json'
  'pre_human_playtest_runbook_ready'
  'runbook_prompts_bound'
  'confusion_triggers_bound'
  'recording_schema_bound'
  'trillionnium-world-evidence-volume-curation-2026-07-07.md'
  'evidence_volume_curation'
  'trillionnium-world-evidence-volume-curation.json'
  'evidence_volume_curation_ready'
  'deletion_performed'
  'archive_movement_performed'
  'trillionnium-world-reviewer-handoff-index-2026-07-07.md'
  'reviewer_handoff_index'
  'trillionnium-world-reviewer-handoff-index.json'
  'reviewer_handoff_index_green_with_public_launch_blockers'
  'representative_visual_count'
  'upload_performed'
  'publish_performed'
  'trillionnium-world-review-slice-strategy-2026-07-07.md'
  'review_slice_strategy'
  'trillionnium-world-review-slice-strategy.json'
  'review_slice_strategy_ready'
  'trillionnium-world-review-slice-manifest-2026-07-07.md'
  'review_slice_manifest'
  'trillionnium-world-review-slice-manifest.json'
  'review_slice_manifest_ready'
  'manifested_commit_count'
  'unclassified_commit_count'
  'multi_slice_commit_count'
  'history_rewrite_performed'
  'trillionnium-world-review-triage-queue-2026-07-07.md'
  'review_triage_queue'
  'trillionnium-world-review-triage-queue.json'
  'review_triage_queue_ready'
  'triage_queue_item_count'
  'triage_bucket_count'
  'manual_review_required'
  'primary_owner_assignment_required'
  'trillionnium-world-review-primary-owner-plan-2026-07-07.md'
  'review_primary_owner_plan'
  'trillionnium-world-review-primary-owner-plan.json'
  'review_primary_owner_plan_ready'
  'owner_bucket_count'
  'bucket_primary_owner_assigned_count'
  'commit_level_primary_owner_review_required_count'
  'review_order_complete'
  'trillionnium-world-review-release-owner-queue-2026-07-07.md'
  'review_release_owner_queue'
  'trillionnium-world-review-release-owner-queue.json'
  'review_release_owner_queue_ready'
  'release_queue_item_count'
  'queue_matches_owner_plan'
  'truth_source_review_item_count'
  'trillionnium-world-review-runtime-owner-queue-2026-07-07.md'
  'review_runtime_owner_queue'
  'trillionnium-world-review-runtime-owner-queue.json'
  'review_runtime_owner_queue_ready'
  'runtime_queue_item_count'
  'runtime_boundary_review_item_count'
  'zero_count_bucket_count'
  'trillionnium-world-review-residual-queue-2026-07-08.md'
  'review_residual_queue'
  'trillionnium-world-review-residual-queue.json'
  'review_residual_queue_ready'
  'remaining_owner_resolution'
  'residual_queue_item_count'
  'all_owner_queue_coverage_complete'
  'manual_assignment_review_item_count'
  'overlap_resolution_review_item_count'
  'trillionnium-world-review-execution-batches-2026-07-08.md'
  'review_execution_batches'
  'trillionnium-world-review-execution-batches.json'
  'review_execution_batches_ready'
  'owner_batch_count'
  'total_queue_item_count'
  'queue_item_coverage_complete'
  'all_owner_batches_match_plan'
  'trillionnium-world-review-public-boundary-batch-2026-07-08.md'
  'review_public_boundary_batch'
  'trillionnium-world-review-public-boundary-batch.json'
  'review_public_boundary_batch_1_ready'
  'unresolved_public_boundary_review_count'
  'batch_2_unblocked_for_local_review'
  'trillionnium-world-review-release-native-handoff-batch-2026-07-08.md'
  'review_release_native_handoff_batch'
  'trillionnium-world-review-release-native-handoff-batch.json'
  'review_release_native_handoff_batch_2_ready'
  'unresolved_release_native_handoff_review_count'
  'TRNM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_REFRESH_INPUTS=0'
  'batch_3_unblocked_for_local_review'
  'trillionnium-world-review-runtime-boundary-batch-2026-07-08.md'
  'review_runtime_boundary_batch'
  'trillionnium-world-review-runtime-boundary-batch.json'
  'review_runtime_boundary_batch_3_sharded'
  'TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0'
  'batch_3_exit_rule_satisfied'
  'batch_4_unblocked_for_local_review'
  'trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md'
  'public_launch_blocker_execution_ledger'
  'trillionnium-world-public-launch-blocker-execution-ledger.json'
  'public_launch_blocker_execution_ledger_ready_for_real_evidence_collection'
  'blocker_consistency_failed_check_count'
  'live_public_exposure_performed'
  'android_device_capture_performed'
  'external_action_performed'
  '.review_slice_strategy.external_action_performed == false'
  '.review_slice_manifest.history_rewrite_performed == false'
  '.review_triage_queue.manual_review_required == true'
  '.review_triage_queue.primary_owner_assignment_required == true'
  '.review_primary_owner_plan.bucket_primary_owner_assignment_complete == true'
  '.review_primary_owner_plan.commit_level_primary_owner_review_required == true'
  '.review_release_owner_queue.queue_matches_owner_plan == true'
  '.review_release_owner_queue.external_action_performed == false'
  '.review_runtime_owner_queue.queue_matches_owner_plan == true'
  '.review_runtime_owner_queue.external_action_performed == false'
  '.review_residual_queue.queue_matches_owner_plan == true'
  '.review_residual_queue.all_owner_queue_coverage_complete == true'
  '.review_residual_queue.external_action_performed == false'
  '.review_execution_batches.queue_item_coverage_complete == true'
  '.review_execution_batches.external_action_performed == false'
  '.review_public_boundary_batch.batch_1_exit_rule_satisfied == true'
  '.review_public_boundary_batch.unresolved_public_boundary_review_count == 0'
  '.review_public_boundary_batch.external_action_performed == false'
  '.review_release_native_handoff_batch.batch_2_exit_rule_satisfied == true'
  '.review_release_native_handoff_batch.unresolved_release_native_handoff_review_count == 0'
  '.review_release_native_handoff_batch.external_action_performed == false'
  '.review_runtime_boundary_batch.batch_3_entry_rule_satisfied == true'
  '.review_runtime_boundary_batch.batch_3_exit_rule_satisfied == false'
  '.review_runtime_boundary_batch.batch_4_unblocked_for_local_review == false'
  '.review_runtime_boundary_batch.external_action_performed == false'
  '.human_playtest_runbook.prompts_bound == true'
  '.evidence_volume_curation.deletion_performed == false'
  '.reviewer_handoff_index.upload_performed == false'
  '.public_launch_blocker_execution_ledger.needs_collection_count == 6'
  'ready_for_renderer_change_from_human_observation'
  'pre_human_playtest_observation_seed'
  'recorded_confusion_point_count == 0'
  'unrecorded_slot_count == 3'
  'human_playtest_evidence_claimed == false'
  'beta_cohort_evidence_claimed == false'
  'human_playtest_path'
  'bevy-classic-playtest-handoff-packet.human_playtest_task_path'
  'real_external_evidence_collection'
  'do not shrink already-gated micro cues without a fresh screenshot-visible issue'
)

required_doc_lines=(
  'Whole-screen First Contact readability review'
  'Public launch state: blocked until real external evidence exists.'
  'Android S5 real-device state: unclaimed until device evidence is collected.'
  'packet binding: `bevy-classic-playtest-handoff-packet`'
  'trillionnium-world-first-contact-readability-review-2026-07-07.md'
  'trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
  'trillionnium-world-evidence-volume-curation-2026-07-07.md'
  'trillionnium-world-reviewer-handoff-index-2026-07-07.md'
  'trillionnium-world-review-slice-strategy-2026-07-07.md'
  'trillionnium-world-review-slice-manifest-2026-07-07.md'
  'trillionnium-world-review-triage-queue-2026-07-07.md'
  'trillionnium-world-review-primary-owner-plan-2026-07-07.md'
  'trillionnium-world-review-release-owner-queue-2026-07-07.md'
  'trillionnium-world-review-runtime-owner-queue-2026-07-07.md'
  'trillionnium-world-review-residual-queue-2026-07-08.md'
  'trillionnium-world-review-execution-batches-2026-07-08.md'
  'trillionnium-world-review-public-boundary-batch-2026-07-08.md'
  'trillionnium-world-review-release-native-handoff-batch-2026-07-08.md'
  'trillionnium-world-review-runtime-boundary-batch-2026-07-08.md'
  'trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md'
  'Do not keep shrinking already-gated micro cues'
)

required_readability_review_lines=(
  'The central beacon fight is still the dominant whole-screen readability risk.'
  'Do a product-level silhouette and composition pass around the active center'
  'Use the five-step human playtest path to log the first three confusion points'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
  'Do not keep shaving already-gated micro cues without a fresh screenshot-visible'
)

required_playtest_observation_log_lines=(
  'Status: pre-human-playtest observation seed.'
  'Record the first three moments where the tester hesitates'
  '| 3 | `secure_beacon` |'
  '| 5 | `recover_blocked_route` |'
  'This log has three recorded human-observed confusion points'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
)

required_playtest_runbook_lines=(
  'Status: pre-human-playtest runbook.'
  'One observer, one local tester, one five-step path.'
  'Read only the fixed prompt for each task'
  '| 1 | `start_campaign` |'
  '| 5 | `recover_blocked_route` |'
  'Each recorded confusion point should include:'
  'ready_for_renderer_change_from_human_observation'
)

required_review_slice_lines=(
  'Status: local review-slice strategy.'
  'This is a grouping plan over existing local commits, not a history rewrite.'
  'Do not push, rebase, force-push, reset, squash, or delete commits'
  '| `release_truth_and_public_boundary` |'
  '| `native_bevy_playable_client` |'
  '| `first_contact_product_readability` |'
  '| `external_evidence_collection_blockers` |'
)

required_evidence_volume_lines=(
  'Status: local evidence-volume curation plan.'
  'Do not delete, compress, move, archive, rewrite, or prune acceptance evidence'
  'Preserve `acceptance/S5_native_bevy_device/latest` as the source of truth.'
  '| `reviewer_summary` |'
  '| `raw_visual_archive_candidate` |'
  '| `external_evidence_blockers` |'
)

required_reviewer_handoff_lines=(
  'Status: local reviewer handoff index.'
  'This is an index over existing local evidence, not a new evidence claim.'
  'Do not delete, compress, move, archive, rewrite, upload, or publish evidence'
  'Review-slice manifest'
  'Review triage queue'
  'Review primary-owner plan'
  'Review release-owner queue'
  'Review runtime-owner queue'
  'Review residual queue'
  'Review execution batches'
  'Review public-boundary batch'
  'Review release-native handoff batch'
  'Public-launch blocker execution ledger'
  '| `reviewer_summary` |'
  '| `representative_visuals` |'
  '| `raw_visual_archive_candidates` |'
)

required_observation_log_script_lines=(
  'trillionnium_world_first_contact_human_playtest_observation_log_v1'
  'first-contact-human-playtest-observation-log.json'
  'recorded_confusion_point_count'
  'unrecorded_slot_count'
  'ready_for_renderer_change_from_human_observation'
  'human_playtest_evidence_claimed == false'
  'beta_cohort_evidence_claimed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_FIRST_CONTACT_HUMAN_PLAYTEST_OBSERVATION_LOG_GREEN'
)

required_runbook_script_lines=(
  'trillionnium_world_first_contact_human_playtest_runbook_v1'
  'first-contact-human-playtest-runbook.json'
  'pre_human_playtest_runbook_ready'
  'runbook_prompts_bound'
  'confusion_triggers_bound'
  'recording_schema_bound'
  'human_playtest_completion_claimed'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_FIRST_CONTACT_HUMAN_PLAYTEST_RUNBOOK_GREEN'
)

required_evidence_volume_script_lines=(
  'trillionnium_world_evidence_volume_curation_v1'
  'trillionnium-world-evidence-volume-curation.json'
  'evidence_volume_curation_ready'
  'large_file_count > 100'
  'deletion_performed == false'
  'compression_performed == false'
  'archive_movement_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_EVIDENCE_VOLUME_CURATION_GREEN'
)

required_reviewer_handoff_script_lines=(
  'trillionnium_world_reviewer_handoff_index_v1'
  'trillionnium-world-reviewer-handoff-index.json'
  'reviewer_handoff_index_green_with_public_launch_blockers'
  'artifact_count == 35'
  'reviewer_summary_count == 21'
  'trillionnium-world-review-triage-queue.json'
  'trillionnium-world-review-primary-owner-plan.json'
  'trillionnium-world-review-release-owner-queue.json'
  'trillionnium-world-review-runtime-owner-queue.json'
  'trillionnium-world-review-residual-queue.json'
  'trillionnium-world-review-execution-batches.json'
  'trillionnium-world-review-public-boundary-batch.json'
  'trillionnium-world-review-release-native-handoff-batch.json'
  'trillionnium-world-review-runtime-boundary-batch.json'
  'representative_visual_count == 5'
  'raw_visual_archive_candidate_count == 6'
  'upload_performed == false'
  'publish_performed == false'
  'public_launch_ready == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEWER_HANDOFF_INDEX_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS'
)

required_blocker_ledger_doc_lines=(
  'Status: local blocker execution ledger.'
  'This consumes existing readiness, evidence-intake, and blocker-consistency'
  'Do not use templates, status-only files, host-side screenshots'
  '| `s5_real_device_matrix` |'
  '| `production_map_pack_public_evidence` |'
  '| `first_beta_cohort_evidence` |'
  '| `commercial_launch_drill_evidence` |'
  '| `multi_node_or_live_traffic_latency_evidence` |'
  '| `public_network_live_exposure_evidence` |'
)

required_blocker_ledger_script_lines=(
  'trillionnium_world_public_launch_blocker_execution_ledger_v1'
  'trillionnium-world-public-launch-blocker-execution-ledger.json'
  'public_launch_blocker_execution_ledger_ready_for_real_evidence_collection'
  'needs_collection_count == 6'
  'green_evidence_item_count == 0'
  'blocker_consistency_failed_check_count == 0'
  'live_public_exposure_performed == false'
  'android_device_capture_performed == false'
  'local_substitutes_rejected == true'
  'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BLOCKER_EXECUTION_LEDGER_READY'
)

required_review_slice_script_lines=(
  'trillionnium_world_review_slice_strategy_v1'
  'trillionnium-world-review-slice-strategy.json'
  'review_slice_strategy_ready'
  'review_slice_count == 6'
  'external_action_performed == false'
  'push_performed == false'
  'rebase_performed == false'
  'reset_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_SLICE_STRATEGY_GREEN'
)

required_review_slice_manifest_lines=(
  'Status: local review-slice commit-range manifest.'
  'read-only manifest over the current git range'
  'Unclassified commits remain manual-review risk'
  '| `release_truth_and_public_boundary` |'
  '| `first_contact_renderer_micro_cues` |'
  '| `rts_runtime_data_boundaries` |'
)

required_review_slice_manifest_script_lines=(
  'trillionnium_world_review_slice_manifest_v1'
  'trillionnium-world-review-slice-manifest.json'
  'review_slice_manifest_ready'
  'manifested_commit_count'
  'unclassified_commit_count'
  'multi_slice_commit_count'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_SLICE_MANIFEST_GREEN'
)

required_review_triage_queue_lines=(
  'Status: local review triage queue.'
  'Unclassified commits are bucketed for review'
  'Multi-slice commits remain overlap risk'
  '| `unclassified_docs_plan_truth_source` |'
  '| `unclassified_generated_count_surface` |'
  '| `multi_public_boundary_overlap` |'
  '| `multi_native_bevy_rts_boundary_overlap` |'
)

required_review_triage_queue_script_lines=(
  'trillionnium_world_review_triage_queue_v1'
  'trillionnium-world-review-triage-queue.json'
  'review_triage_queue_ready'
  'triage_bucket_count == 11'
  'unclassified_bucketed_count == .unclassified_commit_count'
  'multi_slice_bucketed_count == .multi_slice_commit_count'
  'manual_review_required == true'
  'primary_owner_assignment_required == true'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_TRIAGE_QUEUE_GREEN'
)

required_review_primary_owner_plan_lines=(
  'Status: local review primary-owner plan.'
  'Bucket primary owners are review routing defaults'
  'Multi-slice and manual buckets still need commit-level reviewer judgment'
  '| `multi_public_boundary_overlap` |'
  '| `multi_native_bevy_rts_boundary_overlap` |'
  '| `multi_manual_overlap` |'
)

required_review_primary_owner_plan_script_lines=(
  'trillionnium_world_review_primary_owner_plan_v1'
  'trillionnium-world-review-primary-owner-plan.json'
  'review_primary_owner_plan_ready'
  'owner_bucket_count == 11'
  'bucket_primary_owner_assigned_count == 11'
  'bucket_primary_owner_assignment_complete == true'
  'commit_level_primary_owner_review_required == true'
  'commit_level_primary_owner_review_required_count'
  'review_order_complete == true'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_PRIMARY_OWNER_PLAN_GREEN'
)

required_review_release_owner_queue_lines=(
  'Status: local release/public-boundary owner queue.'
  'release_truth_and_public_boundary'
  'It does not stage, commit, push, rebase, reset, squash'
  '| `multi_public_boundary_overlap` |'
  '| `multi_release_native_handoff_overlap` |'
  '| `unclassified_generated_count_surface` |'
  '| `unclassified_docs_plan_truth_source` |'
)

required_review_release_owner_queue_script_lines=(
  'trillionnium_world_review_release_owner_queue_v1'
  'trillionnium-world-review-release-owner-queue.json'
  'review_release_owner_queue_ready'
  'release_truth_and_public_boundary'
  'lane_bucket_count == 4'
  'release_queue_item_count'
  'queue_matches_owner_plan == true'
  'commit_level_primary_owner_review_required_count'
  'truth_source_review_item_count'
  'bucket_coverage_complete == true'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_RELEASE_OWNER_QUEUE_GREEN'
)

required_review_runtime_owner_queue_lines=(
  'Status: local RTS runtime/data-boundary owner queue.'
  'rts_runtime_data_boundaries'
  'It does not stage, commit, push, rebase, reset, squash'
  '| `multi_native_bevy_rts_boundary_overlap` |'
  '| `unclassified_bot_executor_surface` |'
  '| `unclassified_map_or_modeling_surface` |'
)

required_review_runtime_owner_queue_script_lines=(
  'trillionnium_world_review_runtime_owner_queue_v1'
  'trillionnium-world-review-runtime-owner-queue.json'
  'review_runtime_owner_queue_ready'
  'rts_runtime_data_boundaries'
  'lane_bucket_count == 3'
  'runtime_queue_item_count'
  'queue_matches_owner_plan == true'
  'commit_level_primary_owner_review_required_count'
  'runtime_boundary_review_item_count'
  'zero_count_bucket_count'
  'bucket_coverage_complete == true'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_RUNTIME_OWNER_QUEUE_GREEN'
)

required_review_residual_queue_lines=(
  'Status: local residual owner-resolution queue.'
  'release/public-boundary or RTS runtime/data-boundary queues'
  'It does not reassign historical authorship, stage, commit, push, rebase'
  '| `unclassified_classic_evidence_surface` |'
  '| `multi_first_contact_readability_renderer_overlap` |'
  '| `unclassified_manual_other` |'
  '| `multi_manual_overlap` |'
)

required_review_residual_queue_script_lines=(
  'trillionnium_world_review_residual_queue_v1'
  'trillionnium-world-review-residual-queue.json'
  'review_residual_queue_ready'
  'remaining_owner_resolution'
  'remaining_bucket_count == 4'
  'residual_queue_item_count'
  'queue_matches_owner_plan == true'
  'all_owner_queue_coverage_complete == true'
  'manual_assignment_review_item_count'
  'overlap_resolution_review_item_count'
  'zero_count_bucket_count'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_RESIDUAL_QUEUE_GREEN'
)

required_review_execution_batches_lines=(
  'Status: local review execution batches.'
  'release, runtime, and residual owner queues'
  'It does not stage, commit, push, rebase, reset, squash'
  '| 1 | `multi_public_boundary_overlap` |'
  '| 3 | `multi_native_bevy_rts_boundary_overlap` |'
  '| 11 | `multi_manual_overlap` |'
)

required_review_execution_batches_script_lines=(
  'trillionnium_world_review_execution_batches_v1'
  'trillionnium-world-review-execution-batches.json'
  'TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS'
  'review_execution_batches_ready'
  'owner_batch_count == 11'
  'total_queue_item_count == .owner_plan_total_commit_count'
  'queue_item_coverage_complete == true'
  'all_owner_batches_match_plan == true'
  'first_batch_bucket_id == "multi_public_boundary_overlap"'
  'final_batch_bucket_id == "multi_manual_overlap"'
  'history_rewrite_performed == false'
  'external_action_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_EXECUTION_BATCHES_GREEN'
)

required_review_public_boundary_batch_lines=(
  'Status: local review public-boundary batch 1.'
  'multi_public_boundary_overlap'
  'It does not stage, commit, push, rebase, reset, squash'
  '`f5299b7e54`'
  '`b65c23a504`'
  'unresolved_public_boundary_review_count=0'
)

required_review_public_boundary_batch_script_lines=(
  'trillionnium_world_review_public_boundary_batch_v1'
  'trillionnium-world-review-public-boundary-batch.json'
  'review_public_boundary_batch_1_ready'
  'multi_public_boundary_overlap'
  'reviewed_commit_count == 6'
  'unresolved_public_boundary_review_count == 0'
  'batch_1_exit_rule_satisfied == true'
  'batch_2_unblocked_for_local_review == true'
  'public_launch_blocker_count == 6'
  'blocker_ledger_needs_collection_count == 6'
  'external_action_performed == false'
  'history_rewrite_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_PUBLIC_BOUNDARY_BATCH_GREEN'
)

required_review_release_native_handoff_batch_lines=(
  'Status: local review release-native handoff batch 2.'
  'multi_release_native_handoff_overlap'
  'Release-review packet integrity'
  'It does not stage, commit, push, rebase, reset, squash'
  '`bcc231f2fb`'
  '`4b53cd606b`'
  'unresolved_release_native_handoff_review_count=0'
)

required_review_release_native_handoff_batch_script_lines=(
  'trillionnium_world_review_release_native_handoff_batch_v1'
  'trillionnium-world-review-release-native-handoff-batch.json'
  'TRNM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_REFRESH_INPUTS'
  'review_release_native_handoff_batch_2_ready'
  'multi_release_native_handoff_overlap'
  'reviewed_commit_count == 29'
  'unresolved_release_native_handoff_review_count == 0'
  'review_group_count == 4'
  'prior_public_boundary_batch_closed == true'
  'packet_integrity_failed_check_count == 0'
  'batch_2_exit_rule_satisfied == true'
  'batch_3_unblocked_for_local_review == true'
  'external_action_performed == false'
  'history_rewrite_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_GREEN'
)

required_review_runtime_boundary_batch_lines=(
  'Status: local review runtime-boundary batch 3 shard plan.'
  'multi_native_bevy_rts_boundary_overlap'
  'Runtime Sub-Batches'
  '`runtime_core_semantics`'
  '`runtime_adapter_and_online_boundary`'
  '`first_contact_player_surface_cues`'
  'batch_3_exit_rule_satisfied=false'
  'batch_4_unblocked_for_local_review=false'
)

required_review_runtime_boundary_batch_script_lines=(
  'trillionnium_world_review_runtime_boundary_batch_v1'
  'trillionnium-world-review-runtime-boundary-batch.json'
  'TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS'
  'review_runtime_boundary_batch_3_sharded'
  'multi_native_bevy_rts_boundary_overlap'
  'runtime_overlap_commit_count == 273'
  'sharded_commit_count == 273'
  'sub_batch_count == 8'
  'remaining_commit_level_review_count == 273'
  'batch_3_entry_rule_satisfied == true'
  'batch_3_exit_rule_satisfied == false'
  'batch_4_unblocked_for_local_review == false'
  'TRILLIONNIUM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] next execution plan script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_doc_lines[@]}"; do
  if ! grep -Fq -- "$line" "$DOC"; then
    echo "[FAIL] next execution plan doc missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_readability_review_lines[@]}"; do
  if ! grep -Fq -- "$line" "$READABILITY_REVIEW_DOC"; then
    echo "[FAIL] readability review doc missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_playtest_observation_log_lines[@]}"; do
  if ! grep -Fq -- "$line" "$PLAYTEST_OBSERVATION_LOG_DOC"; then
    echo "[FAIL] playtest observation log missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_playtest_runbook_lines[@]}"; do
  if ! grep -Fq -- "$line" "$PLAYTEST_RUNBOOK_DOC"; then
    echo "[FAIL] playtest runbook missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_slice_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_SLICE_DOC"; then
    echo "[FAIL] review slice strategy missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_slice_manifest_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_SLICE_MANIFEST_DOC"; then
    echo "[FAIL] review slice manifest missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_triage_queue_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_TRIAGE_QUEUE_DOC"; then
    echo "[FAIL] review triage queue missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_primary_owner_plan_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_PRIMARY_OWNER_PLAN_DOC"; then
    echo "[FAIL] review primary-owner plan missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_release_owner_queue_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RELEASE_OWNER_QUEUE_DOC"; then
    echo "[FAIL] review release-owner queue missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_runtime_owner_queue_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RUNTIME_OWNER_QUEUE_DOC"; then
    echo "[FAIL] review runtime-owner queue missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_residual_queue_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RESIDUAL_QUEUE_DOC"; then
    echo "[FAIL] review residual queue missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_execution_batches_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_EXECUTION_BATCHES_DOC"; then
    echo "[FAIL] review execution batches missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_public_boundary_batch_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC"; then
    echo "[FAIL] review public-boundary batch missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_release_native_handoff_batch_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC"; then
    echo "[FAIL] review release-native handoff batch missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_runtime_boundary_batch_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC"; then
    echo "[FAIL] review runtime-boundary batch missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_evidence_volume_lines[@]}"; do
  if ! grep -Fq -- "$line" "$EVIDENCE_VOLUME_DOC"; then
    echo "[FAIL] evidence volume curation missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_reviewer_handoff_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEWER_HANDOFF_DOC"; then
    echo "[FAIL] reviewer handoff index missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_observation_log_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$OBSERVATION_LOG_SCRIPT"; then
    echo "[FAIL] playtest observation log script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_runbook_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$RUNBOOK_SCRIPT"; then
    echo "[FAIL] playtest runbook script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_slice_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_SLICE_SCRIPT"; then
    echo "[FAIL] review slice strategy script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_slice_manifest_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_SLICE_MANIFEST_SCRIPT"; then
    echo "[FAIL] review slice manifest script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_triage_queue_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_TRIAGE_QUEUE_SCRIPT"; then
    echo "[FAIL] review triage queue script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_primary_owner_plan_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_PRIMARY_OWNER_PLAN_SCRIPT"; then
    echo "[FAIL] review primary-owner plan script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_release_owner_queue_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RELEASE_OWNER_QUEUE_SCRIPT"; then
    echo "[FAIL] review release-owner queue script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_runtime_owner_queue_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RUNTIME_OWNER_QUEUE_SCRIPT"; then
    echo "[FAIL] review runtime-owner queue script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_residual_queue_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RESIDUAL_QUEUE_SCRIPT"; then
    echo "[FAIL] review residual queue script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_execution_batches_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_EXECUTION_BATCHES_SCRIPT"; then
    echo "[FAIL] review execution batches script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_public_boundary_batch_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_PUBLIC_BOUNDARY_BATCH_SCRIPT"; then
    echo "[FAIL] review public-boundary batch script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_release_native_handoff_batch_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_SCRIPT"; then
    echo "[FAIL] review release-native handoff batch script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_runtime_boundary_batch_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_RUNTIME_BOUNDARY_BATCH_SCRIPT"; then
    echo "[FAIL] review runtime-boundary batch script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_evidence_volume_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$EVIDENCE_VOLUME_SCRIPT"; then
    echo "[FAIL] evidence volume curation script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_reviewer_handoff_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEWER_HANDOFF_SCRIPT"; then
    echo "[FAIL] reviewer handoff index script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_blocker_ledger_doc_lines[@]}"; do
  if ! grep -Fq -- "$line" "$BLOCKER_LEDGER_DOC"; then
    echo "[FAIL] public launch blocker execution ledger doc missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_blocker_ledger_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$BLOCKER_LEDGER_SCRIPT"; then
    echo "[FAIL] public launch blocker execution ledger script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] next execution plan script/doc keep whole-screen product direction, public-launch blockers, and micro-cue restraint"
