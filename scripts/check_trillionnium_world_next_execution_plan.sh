#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC="$ROOT/docs/development/trillionnium-world-next-execution-plan-v1.md"
READABILITY_REVIEW_DOC="$ROOT/docs/development/trillionnium-world-first-contact-readability-review-2026-07-07.md"
READABILITY_REVIEW_DOC_REL="docs/development/trillionnium-world-first-contact-readability-review-2026-07-07.md"
PLAYTEST_OBSERVATION_LOG_DOC="$ROOT/docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
PLAYTEST_OBSERVATION_LOG_DOC_REL="docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
PLAYTEST_OBSERVATION_LOG_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-observation-log.json"
PLAYTEST_RUNBOOK_DOC="$ROOT/docs/development/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
PLAYTEST_RUNBOOK_DOC_REL="docs/development/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
PLAYTEST_RUNBOOK_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-runbook.json"
EVIDENCE_VOLUME_CURATION_DOC="$ROOT/docs/development/trillionnium-world-evidence-volume-curation-2026-07-07.md"
EVIDENCE_VOLUME_CURATION_DOC_REL="docs/development/trillionnium-world-evidence-volume-curation-2026-07-07.md"
EVIDENCE_VOLUME_CURATION_JSON="$ACCEPTANCE_DIR/trillionnium-world-evidence-volume-curation.json"
REVIEWER_HANDOFF_INDEX_DOC="$ROOT/docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md"
REVIEWER_HANDOFF_INDEX_DOC_REL="docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md"
REVIEWER_HANDOFF_INDEX_JSON="$ACCEPTANCE_DIR/trillionnium-world-reviewer-handoff-index.json"
REVIEW_SLICE_STRATEGY_DOC="$ROOT/docs/development/trillionnium-world-review-slice-strategy-2026-07-07.md"
REVIEW_SLICE_STRATEGY_DOC_REL="docs/development/trillionnium-world-review-slice-strategy-2026-07-07.md"
REVIEW_SLICE_STRATEGY_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-slice-strategy.json"
REVIEW_SLICE_MANIFEST_DOC="$ROOT/docs/development/trillionnium-world-review-slice-manifest-2026-07-07.md"
REVIEW_SLICE_MANIFEST_DOC_REL="docs/development/trillionnium-world-review-slice-manifest-2026-07-07.md"
REVIEW_SLICE_MANIFEST_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-slice-manifest.json"
REVIEW_TRIAGE_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-triage-queue-2026-07-07.md"
REVIEW_TRIAGE_QUEUE_DOC_REL="docs/development/trillionnium-world-review-triage-queue-2026-07-07.md"
REVIEW_TRIAGE_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-triage-queue.json"
REVIEW_PRIMARY_OWNER_PLAN_DOC="$ROOT/docs/development/trillionnium-world-review-primary-owner-plan-2026-07-07.md"
REVIEW_PRIMARY_OWNER_PLAN_DOC_REL="docs/development/trillionnium-world-review-primary-owner-plan-2026-07-07.md"
REVIEW_PRIMARY_OWNER_PLAN_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-primary-owner-plan.json"
REVIEW_RELEASE_OWNER_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-release-owner-queue-2026-07-07.md"
REVIEW_RELEASE_OWNER_QUEUE_DOC_REL="docs/development/trillionnium-world-review-release-owner-queue-2026-07-07.md"
REVIEW_RELEASE_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
REVIEW_RUNTIME_OWNER_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-runtime-owner-queue-2026-07-07.md"
REVIEW_RUNTIME_OWNER_QUEUE_DOC_REL="docs/development/trillionnium-world-review-runtime-owner-queue-2026-07-07.md"
REVIEW_RUNTIME_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-owner-queue.json"
REVIEW_RESIDUAL_QUEUE_DOC="$ROOT/docs/development/trillionnium-world-review-residual-queue-2026-07-08.md"
REVIEW_RESIDUAL_QUEUE_DOC_REL="docs/development/trillionnium-world-review-residual-queue-2026-07-08.md"
REVIEW_RESIDUAL_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-residual-queue.json"
REVIEW_EXECUTION_BATCHES_DOC="$ROOT/docs/development/trillionnium-world-review-execution-batches-2026-07-08.md"
REVIEW_EXECUTION_BATCHES_DOC_REL="docs/development/trillionnium-world-review-execution-batches-2026-07-08.md"
REVIEW_EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
REVIEW_PUBLIC_BOUNDARY_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-public-boundary-batch-2026-07-08.md"
REVIEW_PUBLIC_BOUNDARY_BATCH_DOC_REL="docs/development/trillionnium-world-review-public-boundary-batch-2026-07-08.md"
REVIEW_PUBLIC_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-public-boundary-batch.json"
REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-release-native-handoff-batch-2026-07-08.md"
REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC_REL="docs/development/trillionnium-world-review-release-native-handoff-batch-2026-07-08.md"
REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-native-handoff-batch.json"
REVIEW_RUNTIME_BOUNDARY_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-runtime-boundary-batch-2026-07-08.md"
REVIEW_RUNTIME_BOUNDARY_BATCH_DOC_REL="docs/development/trillionnium-world-review-runtime-boundary-batch-2026-07-08.md"
REVIEW_RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-runtime-core-semantics-batch-2026-07-08.md"
REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC_REL="docs/development/trillionnium-world-review-runtime-core-semantics-batch-2026-07-08.md"
REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-core-semantics-batch.json"
REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-runtime-adapter-online-batch-2026-07-08.md"
REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC_REL="docs/development/trillionnium-world-review-runtime-adapter-online-batch-2026-07-08.md"
REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-adapter-online-batch.json"
REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC="$ROOT/docs/development/trillionnium-world-review-openra-parity-claim-batch-2026-07-08.md"
REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC_REL="docs/development/trillionnium-world-review-openra-parity-claim-batch-2026-07-08.md"
REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-openra-parity-claim-batch.json"
PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC="$ROOT/docs/development/trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md"
PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC_REL="docs/development/trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md"
PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON="$ACCEPTANCE_DIR/trillionnium-world-public-launch-blocker-execution-ledger.json"
PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
PUBLIC_LAUNCH_JSON="$ACCEPTANCE_DIR/public-launch-readiness.json"
RUNNER_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json"
SUMMARY_JSON="$ACCEPTANCE_DIR/trillionnium-world-next-execution-plan.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-next-execution-plan.md"

if [[ -v TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_SUMMARY && -n "$TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_SUMMARY" ]]; then
  SUMMARY_JSON="$TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_SUMMARY"
fi

mkdir -p "$ACCEPTANCE_DIR"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "[FAIL] missing required file: $path" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

require_file "$DOC"
require_file "$READABILITY_REVIEW_DOC"
require_file "$PLAYTEST_OBSERVATION_LOG_DOC"
require_file "$PLAYTEST_RUNBOOK_DOC"
require_file "$EVIDENCE_VOLUME_CURATION_DOC"
require_file "$REVIEWER_HANDOFF_INDEX_DOC"
require_file "$REVIEW_SLICE_STRATEGY_DOC"
require_file "$REVIEW_SLICE_MANIFEST_DOC"
require_file "$REVIEW_TRIAGE_QUEUE_DOC"
require_file "$REVIEW_PRIMARY_OWNER_PLAN_DOC"
require_file "$REVIEW_RELEASE_OWNER_QUEUE_DOC"
require_file "$REVIEW_RUNTIME_OWNER_QUEUE_DOC"
require_file "$REVIEW_RESIDUAL_QUEUE_DOC"
require_file "$REVIEW_EXECUTION_BATCHES_DOC"
require_file "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC"
require_file "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC"
require_file "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC"
require_file "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC"
require_file "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC"
require_file "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC"
require_file "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC"
require_file "$PACKET_JSON"
require_file "$PUBLIC_LAUNCH_JSON"
require_file "$RUNNER_JSON"

require_text "$DOC" "Whole-screen First Contact readability review"
require_text "$DOC" "Local review state: green with public-launch blockers."
require_text "$DOC" "Public launch state: blocked until real external evidence exists."
require_text "$DOC" 'packet binding: `bevy-classic-playtest-handoff-packet`'
require_text "$DOC" "Do not keep shrinking already-gated micro cues"
require_text "$DOC" "trillionnium-world-first-contact-readability-review-2026-07-07.md"
require_text "$DOC" "trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
require_text "$DOC" "trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
require_text "$DOC" "trillionnium-world-evidence-volume-curation-2026-07-07.md"
require_text "$DOC" "trillionnium-world-reviewer-handoff-index-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-slice-strategy-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-slice-manifest-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-triage-queue-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-primary-owner-plan-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-release-owner-queue-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-runtime-owner-queue-2026-07-07.md"
require_text "$DOC" "trillionnium-world-review-residual-queue-2026-07-08.md"
require_text "$DOC" "trillionnium-world-review-execution-batches-2026-07-08.md"
require_text "$DOC" "trillionnium-world-review-public-boundary-batch-2026-07-08.md"
require_text "$DOC" "trillionnium-world-review-release-native-handoff-batch-2026-07-08.md"
require_text "$DOC" "trillionnium-world-review-runtime-boundary-batch-2026-07-08.md"
require_text "$DOC" "trillionnium-world-review-runtime-core-semantics-batch-2026-07-08.md"
require_text "$DOC" "trillionnium-world-review-runtime-adapter-online-batch-2026-07-08.md"
require_text "$DOC" "trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md"
require_text "$READABILITY_REVIEW_DOC" "The central beacon fight is still the dominant whole-screen readability risk."
require_text "$READABILITY_REVIEW_DOC" "Do a product-level silhouette and composition pass around the active center"
require_text "$READABILITY_REVIEW_DOC" "Use the five-step human playtest path to log the first three confusion points"
require_text "$READABILITY_REVIEW_DOC" "trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
require_text "$READABILITY_REVIEW_DOC" "trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
require_text "$PLAYTEST_OBSERVATION_LOG_DOC" "Status: pre-human-playtest observation seed."
require_text "$PLAYTEST_OBSERVATION_LOG_DOC" "Record the first three moments where the tester hesitates"
require_text "$PLAYTEST_OBSERVATION_LOG_DOC" '| 3 | `secure_beacon` |'
require_text "$PLAYTEST_OBSERVATION_LOG_DOC" '| 5 | `recover_blocked_route` |'
require_text "$PLAYTEST_OBSERVATION_LOG_DOC" "This log has three recorded human-observed confusion points"
require_text "$PLAYTEST_OBSERVATION_LOG_DOC" "trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
require_text "$PLAYTEST_RUNBOOK_DOC" "Status: pre-human-playtest runbook."
require_text "$PLAYTEST_RUNBOOK_DOC" "One observer, one local tester, one five-step path."
require_text "$PLAYTEST_RUNBOOK_DOC" "Read only the fixed prompt for each task"
require_text "$PLAYTEST_RUNBOOK_DOC" "Stop after the first three confusion points are recorded."
require_text "$PLAYTEST_RUNBOOK_DOC" '| 5 | `recover_blocked_route` |'
require_text "$EVIDENCE_VOLUME_CURATION_DOC" "Status: local evidence-volume curation plan."
require_text "$EVIDENCE_VOLUME_CURATION_DOC" "Do not delete, compress, move, archive, rewrite, or prune acceptance evidence"
require_text "$EVIDENCE_VOLUME_CURATION_DOC" '| `raw_visual_archive_candidate` |'
require_text "$REVIEWER_HANDOFF_INDEX_DOC" "Status: local reviewer handoff index."
require_text "$REVIEWER_HANDOFF_INDEX_DOC" "This is an index over existing local evidence, not a new evidence claim."
require_text "$REVIEWER_HANDOFF_INDEX_DOC" '| `representative_visuals` |'
require_text "$REVIEWER_HANDOFF_INDEX_DOC" '| `raw_visual_archive_candidates` |'
require_text "$REVIEW_SLICE_STRATEGY_DOC" "Status: local review-slice strategy."
require_text "$REVIEW_SLICE_STRATEGY_DOC" "Do not push, rebase, force-push, reset, squash, or delete commits"
require_text "$REVIEW_SLICE_STRATEGY_DOC" '| `release_truth_and_public_boundary` |'
require_text "$REVIEW_SLICE_STRATEGY_DOC" '| `first_contact_product_readability` |'
require_text "$REVIEW_SLICE_STRATEGY_DOC" '| `external_evidence_collection_blockers` |'
require_text "$REVIEW_SLICE_MANIFEST_DOC" "Status: local review-slice commit-range manifest."
require_text "$REVIEW_SLICE_MANIFEST_DOC" "Unclassified commits remain manual-review risk"
require_text "$REVIEW_SLICE_MANIFEST_DOC" '| `first_contact_renderer_micro_cues` |'
require_text "$REVIEW_SLICE_MANIFEST_DOC" '| `rts_runtime_data_boundaries` |'
require_text "$REVIEW_TRIAGE_QUEUE_DOC" "Status: local review triage queue."
require_text "$REVIEW_TRIAGE_QUEUE_DOC" "Unclassified commits are bucketed for review"
require_text "$REVIEW_TRIAGE_QUEUE_DOC" "Multi-slice commits remain overlap risk"
require_text "$REVIEW_TRIAGE_QUEUE_DOC" '| `unclassified_generated_count_surface` |'
require_text "$REVIEW_TRIAGE_QUEUE_DOC" '| `multi_native_bevy_rts_boundary_overlap` |'
require_text "$REVIEW_PRIMARY_OWNER_PLAN_DOC" "Status: local review primary-owner plan."
require_text "$REVIEW_PRIMARY_OWNER_PLAN_DOC" "Bucket primary owners are review routing defaults"
require_text "$REVIEW_PRIMARY_OWNER_PLAN_DOC" "Multi-slice and manual buckets still need commit-level reviewer judgment"
require_text "$REVIEW_PRIMARY_OWNER_PLAN_DOC" '| `multi_native_bevy_rts_boundary_overlap` |'
require_text "$REVIEW_PRIMARY_OWNER_PLAN_DOC" '| `multi_manual_overlap` |'
require_text "$REVIEW_RELEASE_OWNER_QUEUE_DOC" "Status: local release/public-boundary owner queue."
require_text "$REVIEW_RELEASE_OWNER_QUEUE_DOC" "release_truth_and_public_boundary"
require_text "$REVIEW_RELEASE_OWNER_QUEUE_DOC" '| `multi_public_boundary_overlap` |'
require_text "$REVIEW_RELEASE_OWNER_QUEUE_DOC" '| `unclassified_generated_count_surface` |'
require_text "$REVIEW_RUNTIME_OWNER_QUEUE_DOC" "Status: local RTS runtime/data-boundary owner queue."
require_text "$REVIEW_RUNTIME_OWNER_QUEUE_DOC" "rts_runtime_data_boundaries"
require_text "$REVIEW_RUNTIME_OWNER_QUEUE_DOC" '| `multi_native_bevy_rts_boundary_overlap` |'
require_text "$REVIEW_RUNTIME_OWNER_QUEUE_DOC" '| `unclassified_bot_executor_surface` |'
require_text "$REVIEW_RUNTIME_OWNER_QUEUE_DOC" '| `unclassified_map_or_modeling_surface` |'
require_text "$REVIEW_RESIDUAL_QUEUE_DOC" "Status: local residual owner-resolution queue."
require_text "$REVIEW_RESIDUAL_QUEUE_DOC" "release/public-boundary or RTS runtime/data-boundary queues"
require_text "$REVIEW_RESIDUAL_QUEUE_DOC" '| `unclassified_classic_evidence_surface` |'
require_text "$REVIEW_RESIDUAL_QUEUE_DOC" '| `multi_first_contact_readability_renderer_overlap` |'
require_text "$REVIEW_RESIDUAL_QUEUE_DOC" '| `unclassified_manual_other` |'
require_text "$REVIEW_RESIDUAL_QUEUE_DOC" '| `multi_manual_overlap` |'
require_text "$REVIEW_EXECUTION_BATCHES_DOC" "Status: local review execution batches."
require_text "$REVIEW_EXECUTION_BATCHES_DOC" "release, runtime, and residual owner queues"
require_text "$REVIEW_EXECUTION_BATCHES_DOC" '| 1 | `multi_public_boundary_overlap` |'
require_text "$REVIEW_EXECUTION_BATCHES_DOC" '| 11 | `multi_manual_overlap` |'
require_text "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC" "Status: local review public-boundary batch 1."
require_text "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC" "multi_public_boundary_overlap"
require_text "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC" "unresolved_public_boundary_review_count=0"
require_text "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC" "Status: local review release-native handoff batch 2."
require_text "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC" "multi_release_native_handoff_overlap"
require_text "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC" "unresolved_release_native_handoff_review_count=0"
require_text "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC" "Status: local review runtime-boundary batch 3 shard plan."
require_text "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC" "multi_native_bevy_rts_boundary_overlap"
require_text "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC" "batch_3_exit_rule_satisfied=false"
require_text "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC" "batch_4_unblocked_for_local_review=false"
require_text "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC" "Status: local review runtime-core semantics sub-batch 1."
require_text "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC" "systemic runtime-core source boundary follow-up"
require_text "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC" "sub_batch_1_exit_rule_satisfied=false"
require_text "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC" "batch_4_unblocked_for_local_review=false"
require_text "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC" "Status: local review runtime adapter/online sub-batch 2."
require_text "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC" "adapter-path part of the prior runtime-core source-boundary"
require_text "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC" "sub_batch_2_exit_rule_satisfied=true"
require_text "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC" "batch_4_unblocked_for_local_review=false"
require_text "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC" "Status: local review OpenRA parity/claim sub-batch 3."
require_text "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC" "openra_parity_and_claim_boundary"
require_text "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC" "sub_batch_3_exit_rule_satisfied=true"
require_text "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC" "batch_4_unblocked_for_local_review=false"
require_text "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC" "Status: local blocker execution ledger."
require_text "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC" "Do not use templates, status-only files, host-side screenshots"
require_text "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC" '| `s5_real_device_matrix` |'
require_text "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC" '| `public_network_live_exposure_evidence` |'

"$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_observation_log.sh" >/dev/null
require_file "$PLAYTEST_OBSERVATION_LOG_JSON"
jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_observation_log_v1"
  and .status == "pre_human_playtest_observation_seed"
  and .recorded_confusion_point_count == 0
  and .unrecorded_slot_count == 3
  and .first_three_confusion_points_recorded == false
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_evidence_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$PLAYTEST_OBSERVATION_LOG_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_runbook.sh" >/dev/null
require_file "$PLAYTEST_RUNBOOK_JSON"
jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_runbook_v1"
  and .status == "pre_human_playtest_runbook_ready"
  and .task_count == 5
  and .required_confusion_point_count == 3
  and .runbook_prompts_bound == true
  and .pass_signals_bound == true
  and .confusion_triggers_bound == true
  and .recording_schema_bound == true
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_completion_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$PLAYTEST_RUNBOOK_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_evidence_volume_curation.sh" >/dev/null
require_file "$EVIDENCE_VOLUME_CURATION_JSON"
jq -e '
  .contract_version == "trillionnium_world_evidence_volume_curation_v1"
  and .status == "evidence_volume_curation_ready"
  and .s5_latest_kib > 10000000
  and .large_file_count > 100
  and .evidence_volume_risk_active == true
  and .deletion_performed == false
  and .compression_performed == false
  and .archive_movement_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$EVIDENCE_VOLUME_CURATION_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_reviewer_handoff_index.sh" >/dev/null
require_file "$REVIEWER_HANDOFF_INDEX_JSON"
jq -e '
  .contract_version == "trillionnium_world_reviewer_handoff_index_v1"
  and .status == "reviewer_handoff_index_green_with_public_launch_blockers"
  and .artifact_count == 38
  and .reviewer_summary_count == 24
  and .live_player_screen_count == 3
  and .representative_visual_count == 5
  and .raw_visual_archive_candidate_count == 6
  and .all_sha256_valid == true
  and .deletion_performed == false
  and .archive_movement_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$REVIEWER_HANDOFF_INDEX_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_slice_strategy.sh" >/dev/null
require_file "$REVIEW_SLICE_STRATEGY_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_slice_strategy_v1"
  and .status == "review_slice_strategy_ready"
  and .review_slice_count == 6
  and .local_backlog_risk_active == true
  and .external_action_performed == false
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_SLICE_STRATEGY_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_slice_manifest.sh" >/dev/null
require_file "$REVIEW_SLICE_MANIFEST_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_slice_manifest_v1"
  and .status == "review_slice_manifest_ready"
  and .review_slice_count == 6
  and .total_ahead_count >= 1
  and ((.manifested_commit_count + .unclassified_commit_count) == .total_ahead_count)
  and .slice_match_total_count >= .manifested_commit_count
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_SLICE_MANIFEST_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_triage_queue.sh" >/dev/null
require_file "$REVIEW_TRIAGE_QUEUE_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_triage_queue_v1"
  and .status == "review_triage_queue_ready"
  and .triage_bucket_count == 11
  and .unclassified_bucketed_count == .unclassified_commit_count
  and .multi_slice_bucketed_count == .multi_slice_commit_count
  and .manual_review_required == true
  and .primary_owner_assignment_required == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_TRIAGE_QUEUE_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_primary_owner_plan.sh" >/dev/null
require_file "$REVIEW_PRIMARY_OWNER_PLAN_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_primary_owner_plan_v1"
  and .status == "review_primary_owner_plan_ready"
  and .owner_bucket_count == 11
  and .bucket_primary_owner_assigned_count == 11
  and .bucket_primary_owner_assignment_complete == true
  and .commit_level_primary_owner_review_required == true
  and .commit_level_primary_owner_review_required_count >= 1
  and .review_order_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_PRIMARY_OWNER_PLAN_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
require_file "$REVIEW_RELEASE_OWNER_QUEUE_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .primary_owner == "release_truth_and_public_boundary"
  and .lane_bucket_count == 4
  and .release_queue_item_count == .owner_plan_release_commit_count
  and .queue_matches_owner_plan == true
  and .commit_level_primary_owner_review_required_count == .owner_plan_commit_level_required_count
  and .bucket_coverage_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RELEASE_OWNER_QUEUE_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh" >/dev/null
require_file "$REVIEW_RUNTIME_OWNER_QUEUE_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_runtime_owner_queue_v1"
  and .status == "review_runtime_owner_queue_ready"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .lane_bucket_count == 3
  and .runtime_queue_item_count == .owner_plan_runtime_commit_count
  and .queue_matches_owner_plan == true
  and .commit_level_primary_owner_review_required_count == .owner_plan_commit_level_required_count
  and .runtime_boundary_review_item_count >= 1
  and .bucket_coverage_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_residual_queue.sh" >/dev/null
require_file "$REVIEW_RESIDUAL_QUEUE_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_residual_queue_v1"
  and .status == "review_residual_queue_ready"
  and .queue_scope == "remaining_owner_resolution"
  and .remaining_bucket_count == 4
  and .residual_queue_item_count == .owner_plan_remaining_commit_count
  and .queue_matches_owner_plan == true
  and .all_owner_queue_coverage_complete == true
  and .manual_assignment_review_item_count >= 1
  and .overlap_resolution_review_item_count >= 1
  and .native_bevy_evidence_review_item_count >= 1
  and .zero_count_bucket_count >= 1
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RESIDUAL_QUEUE_JSON" >/dev/null

TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
require_file "$REVIEW_EXECUTION_BATCHES_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and .owner_batch_count == 11
  and .total_queue_item_count == .owner_plan_total_commit_count
  and .queue_item_coverage_complete == true
  and .all_owner_batches_match_plan == true
  and .first_batch_bucket_id == "multi_public_boundary_overlap"
  and .final_batch_bucket_id == "multi_manual_overlap"
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_EXECUTION_BATCHES_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_review_public_boundary_batch.sh" >/dev/null
require_file "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_public_boundary_batch_v1"
  and .status == "review_public_boundary_batch_1_ready"
  and .batch_order == 1
  and .bucket_id == "multi_public_boundary_overlap"
  and .reviewed_commit_count == 6
  and .unresolved_public_boundary_review_count == 0
  and .batch_1_exit_rule_satisfied == true
  and .batch_2_unblocked_for_local_review == true
  and .public_launch_blocker_count == 6
  and .blocker_ledger_needs_collection_count == 6
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON" >/dev/null

TRNM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_release_native_handoff_batch.sh" >/dev/null
require_file "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_release_native_handoff_batch_v1"
  and .status == "review_release_native_handoff_batch_2_ready"
  and .batch_order == 2
  and .bucket_id == "multi_release_native_handoff_overlap"
  and .reviewed_commit_count == 29
  and .unresolved_release_native_handoff_review_count == 0
  and .review_group_count == 4
  and .prior_public_boundary_batch_closed == true
  and .packet_integrity_failed_check_count == 0
  and .batch_2_exit_rule_satisfied == true
  and .batch_3_unblocked_for_local_review == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON" >/dev/null

TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
require_file "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_runtime_boundary_batch_v1"
  and .status == "review_runtime_boundary_batch_3_sharded"
  and .batch_order == 3
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .runtime_overlap_commit_count == 273
  and .sharded_commit_count == 273
  and .sub_batch_count == 8
  and .remaining_commit_level_review_count == 273
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "runtime_core_semantics"
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

TRNM_WORLD_REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_runtime_core_semantics_batch.sh" >/dev/null
require_file "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_runtime_core_semantics_batch_v1"
  and .status == "review_runtime_core_semantics_sub_batch_1_reviewed_with_boundary_followup"
  and .batch_order == 3
  and .sub_batch_order == 1
  and .sub_batch_id == "runtime_core_semantics"
  and .reviewed_commit_count == 55
  and .unresolved_commit_review_count == 0
  and .systemic_runtime_core_boundary_followup_count == 1
  and .sub_batch_1_local_review_complete == true
  and .sub_batch_1_exit_rule_satisfied == false
  and .sub_batch_2_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "runtime_adapter_and_online_boundary"
  and .openra_like_core_all_gates_green == true
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_compatibility_claimed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON" >/dev/null

TRNM_WORLD_REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_runtime_adapter_online_batch.sh" >/dev/null
require_file "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_runtime_adapter_online_batch_v1"
  and .status == "review_runtime_adapter_online_sub_batch_2_reviewed"
  and .batch_order == 3
  and .sub_batch_order == 2
  and .sub_batch_id == "runtime_adapter_and_online_boundary"
  and .reviewed_commit_count == 57
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 112
  and .batch_3_remaining_commit_level_review_count == 161
  and .adapter_path_resolves_runtime_core_source_boundary_followup == true
  and .sub_batch_2_local_review_complete == true
  and .sub_batch_2_exit_rule_satisfied == true
  and .sub_batch_3_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "openra_parity_and_claim_boundary"
  and .online_offline_adapter_green == true
  and .socket_opened == false
  and .hosted_service_claimed == false
  and .client_prediction_claimed == false
  and .rollback_netcode_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON" >/dev/null

TRNM_WORLD_REVIEW_OPENRA_PARITY_CLAIM_BATCH_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_openra_parity_claim_batch.sh" >/dev/null
require_file "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON"
jq -e '
  .contract_version == "trillionnium_world_review_openra_parity_claim_batch_v1"
  and .status == "review_openra_parity_claim_sub_batch_3_reviewed"
  and .batch_order == 3
  and .sub_batch_order == 3
  and .sub_batch_id == "openra_parity_and_claim_boundary"
  and .reviewed_commit_count == 35
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 147
  and .batch_3_remaining_commit_level_review_count == 126
  and .sub_batch_3_local_review_complete == true
  and .sub_batch_3_exit_rule_satisfied == true
  and .sub_batch_4_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "first_contact_rts_data_extraction"
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_order_stream_claimed == false
  and .openra_engine_port_claimed == false
  and .openra_pixel_perfect_asset_parity_claimed == false
  and .third_party_asset_copied == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON" >/dev/null

"$ROOT/scripts/check_trillionnium_world_public_launch_blocker_execution_ledger.sh" >/dev/null
require_file "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON"
jq -e '
  .contract_version == "trillionnium_world_public_launch_blocker_execution_ledger_v1"
  and .status == "public_launch_blocker_execution_ledger_ready_for_real_evidence_collection"
  and .blocker_count == 6
  and .evidence_item_count == 6
  and .needs_collection_count == 6
  and .green_evidence_item_count == 0
  and .blocker_consistency_failed_check_count == 0
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .live_map_ingestion_performed == false
  and .live_public_exposure_performed == false
  and .android_device_capture_performed == false
  and .local_substitutes_rejected == true
' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON" >/dev/null

packet_status="$(jq -r '.status // "missing"' "$PACKET_JSON")"
packet_green="$(jq -r '.green // false' "$PACKET_JSON")"
packet_artifact_count="$(jq -r '.artifact_count // 0' "$PACKET_JSON")"
packet_failed_check_count="$(jq -r '.failed_check_count // 999' "$PACKET_JSON")"
packet_check_count="$(jq -r '(.checks // []) | length' "$PACKET_JSON")"

runner_green="$(jq -r '.green // false' "$RUNNER_JSON")"
runner_gate_count="$(jq -r '.gate_count // 0' "$RUNNER_JSON")"
runner_failed_gate_count="$(jq -r '.failed_gate_count // 999' "$RUNNER_JSON")"
runner_pid="$(jq -r '.service.main_pid // "unknown"' "$RUNNER_JSON")"
runner_screenshot_path="$(jq -r '.live_player_screen.screenshot_path // ""' "$RUNNER_JSON")"
observation_status="$(jq -r '.status // "missing"' "$PLAYTEST_OBSERVATION_LOG_JSON")"
recorded_confusion_point_count="$(jq -r '.recorded_confusion_point_count // 0' "$PLAYTEST_OBSERVATION_LOG_JSON")"
unrecorded_slot_count="$(jq -r '.unrecorded_slot_count // 0' "$PLAYTEST_OBSERVATION_LOG_JSON")"
first_three_confusion_points_recorded="$(jq -r '.first_three_confusion_points_recorded // false' "$PLAYTEST_OBSERVATION_LOG_JSON")"
ready_for_renderer_change_from_human_observation="$(jq -r '.ready_for_renderer_change_from_human_observation // false' "$PLAYTEST_OBSERVATION_LOG_JSON")"
runbook_status="$(jq -r '.status // "missing"' "$PLAYTEST_RUNBOOK_JSON")"
runbook_prompts_bound="$(jq -r '.runbook_prompts_bound // false' "$PLAYTEST_RUNBOOK_JSON")"
runbook_confusion_triggers_bound="$(jq -r '.confusion_triggers_bound // false' "$PLAYTEST_RUNBOOK_JSON")"
runbook_recording_schema_bound="$(jq -r '.recording_schema_bound // false' "$PLAYTEST_RUNBOOK_JSON")"
evidence_volume_status="$(jq -r '.status // "missing"' "$EVIDENCE_VOLUME_CURATION_JSON")"
evidence_volume_large_file_count="$(jq -r '.large_file_count // 0' "$EVIDENCE_VOLUME_CURATION_JSON")"
evidence_volume_deletion_performed="$(jq -r 'if has("deletion_performed") then .deletion_performed else true end' "$EVIDENCE_VOLUME_CURATION_JSON")"
evidence_volume_archive_movement_performed="$(jq -r 'if has("archive_movement_performed") then .archive_movement_performed else true end' "$EVIDENCE_VOLUME_CURATION_JSON")"
reviewer_handoff_index_status="$(jq -r '.status // "missing"' "$REVIEWER_HANDOFF_INDEX_JSON")"
reviewer_handoff_index_artifact_count="$(jq -r '.artifact_count // 0' "$REVIEWER_HANDOFF_INDEX_JSON")"
reviewer_handoff_index_representative_visual_count="$(jq -r '.representative_visual_count // 0' "$REVIEWER_HANDOFF_INDEX_JSON")"
reviewer_handoff_index_upload_performed="$(jq -r 'if has("upload_performed") then .upload_performed else true end' "$REVIEWER_HANDOFF_INDEX_JSON")"
reviewer_handoff_index_publish_performed="$(jq -r 'if has("publish_performed") then .publish_performed else true end' "$REVIEWER_HANDOFF_INDEX_JSON")"
review_slice_strategy_status="$(jq -r '.status // "missing"' "$REVIEW_SLICE_STRATEGY_JSON")"
review_slice_count="$(jq -r '.review_slice_count // 0' "$REVIEW_SLICE_STRATEGY_JSON")"
review_slice_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_SLICE_STRATEGY_JSON")"
review_slice_manifest_status="$(jq -r '.status // "missing"' "$REVIEW_SLICE_MANIFEST_JSON")"
review_slice_manifest_total_ahead_count="$(jq -r '.total_ahead_count // 0' "$REVIEW_SLICE_MANIFEST_JSON")"
review_slice_manifest_manifested_commit_count="$(jq -r '.manifested_commit_count // 0' "$REVIEW_SLICE_MANIFEST_JSON")"
review_slice_manifest_unclassified_commit_count="$(jq -r '.unclassified_commit_count // 0' "$REVIEW_SLICE_MANIFEST_JSON")"
review_slice_manifest_multi_slice_commit_count="$(jq -r '.multi_slice_commit_count // 0' "$REVIEW_SLICE_MANIFEST_JSON")"
review_slice_manifest_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_SLICE_MANIFEST_JSON")"
review_slice_manifest_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_SLICE_MANIFEST_JSON")"
review_triage_queue_status="$(jq -r '.status // "missing"' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_queue_item_count="$(jq -r '.triage_queue_item_count // 0' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_bucket_count="$(jq -r '.triage_bucket_count // 0' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_unclassified_bucketed_count="$(jq -r '.unclassified_bucketed_count // 0' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_multi_slice_bucketed_count="$(jq -r '.multi_slice_bucketed_count // 0' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_manual_review_required="$(jq -r 'if has("manual_review_required") then .manual_review_required else false end' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_primary_owner_assignment_required="$(jq -r 'if has("primary_owner_assignment_required") then .primary_owner_assignment_required else false end' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_triage_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_TRIAGE_QUEUE_JSON")"
review_primary_owner_plan_status="$(jq -r '.status // "missing"' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_owner_bucket_count="$(jq -r '.owner_bucket_count // 0' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_assigned_count="$(jq -r '.bucket_primary_owner_assigned_count // 0' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_assignment_complete="$(jq -r 'if has("bucket_primary_owner_assignment_complete") then .bucket_primary_owner_assignment_complete else false end' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_commit_level_required="$(jq -r 'if has("commit_level_primary_owner_review_required") then .commit_level_primary_owner_review_required else false end' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_commit_level_required_count="$(jq -r '.commit_level_primary_owner_review_required_count // 0' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_review_order_complete="$(jq -r 'if has("review_order_complete") then .review_order_complete else false end' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_primary_owner_plan_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_PRIMARY_OWNER_PLAN_JSON")"
review_release_owner_queue_status="$(jq -r '.status // "missing"' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_item_count="$(jq -r '.release_queue_item_count // 0' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_lane_bucket_count="$(jq -r '.lane_bucket_count // 0' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_matches_owner_plan="$(jq -r 'if has("queue_matches_owner_plan") then .queue_matches_owner_plan else false end' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_commit_level_required_count="$(jq -r '.commit_level_primary_owner_review_required_count // 0' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_truth_source_count="$(jq -r '.truth_source_review_item_count // 0' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_release_owner_queue_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RELEASE_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_status="$(jq -r '.status // "missing"' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_item_count="$(jq -r '.runtime_queue_item_count // 0' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_lane_bucket_count="$(jq -r '.lane_bucket_count // 0' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_matches_owner_plan="$(jq -r 'if has("queue_matches_owner_plan") then .queue_matches_owner_plan else false end' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_commit_level_required_count="$(jq -r '.commit_level_primary_owner_review_required_count // 0' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_boundary_review_count="$(jq -r '.runtime_boundary_review_item_count // 0' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_zero_count_bucket_count="$(jq -r '.zero_count_bucket_count // 0' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_runtime_owner_queue_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON")"
review_residual_queue_status="$(jq -r '.status // "missing"' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_scope="$(jq -r '.queue_scope // "missing"' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_item_count="$(jq -r '.residual_queue_item_count // 0' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_bucket_count="$(jq -r '.remaining_bucket_count // 0' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_matches_owner_plan="$(jq -r 'if has("queue_matches_owner_plan") then .queue_matches_owner_plan else false end' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_coverage_complete="$(jq -r 'if has("all_owner_queue_coverage_complete") then .all_owner_queue_coverage_complete else false end' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_manual_assignment_count="$(jq -r '.manual_assignment_review_item_count // 0' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_overlap_resolution_count="$(jq -r '.overlap_resolution_review_item_count // 0' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_zero_count_bucket_count="$(jq -r '.zero_count_bucket_count // 0' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_residual_queue_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RESIDUAL_QUEUE_JSON")"
review_execution_batches_status="$(jq -r '.status // "missing"' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_owner_batch_count="$(jq -r '.owner_batch_count // 0' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_nonempty_batch_count="$(jq -r '.nonempty_batch_count // 0' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_reserved_zero_count_batch_count="$(jq -r '.reserved_zero_count_batch_count // 0' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_total_queue_item_count="$(jq -r '.total_queue_item_count // 0' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_owner_plan_total_commit_count="$(jq -r '.owner_plan_total_commit_count // 0' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_coverage_complete="$(jq -r 'if has("queue_item_coverage_complete") then .queue_item_coverage_complete else false end' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_all_match_plan="$(jq -r 'if has("all_owner_batches_match_plan") then .all_owner_batches_match_plan else false end' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_first_batch_bucket_id="$(jq -r '.first_batch_bucket_id // "missing"' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_final_batch_bucket_id="$(jq -r '.final_batch_bucket_id // "missing"' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_execution_batches_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_EXECUTION_BATCHES_JSON")"
review_public_boundary_batch_status="$(jq -r '.status // "missing"' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_reviewed_commit_count="$(jq -r '.reviewed_commit_count // 0' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_unresolved_count="$(jq -r '.unresolved_public_boundary_review_count // 999' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_exit_rule_satisfied="$(jq -r 'if has("batch_1_exit_rule_satisfied") then .batch_1_exit_rule_satisfied else false end' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_next_unblocked="$(jq -r 'if has("batch_2_unblocked_for_local_review") then .batch_2_unblocked_for_local_review else false end' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_blocker_count="$(jq -r '.public_launch_blocker_count // 0' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_needs_collection_count="$(jq -r '.blocker_ledger_needs_collection_count // 0' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_public_boundary_batch_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_PUBLIC_BOUNDARY_BATCH_JSON")"
review_release_native_handoff_batch_status="$(jq -r '.status // "missing"' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_reviewed_commit_count="$(jq -r '.reviewed_commit_count // 0' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_unresolved_count="$(jq -r '.unresolved_release_native_handoff_review_count // 999' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_review_group_count="$(jq -r '.review_group_count // 0' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_prior_closed="$(jq -r 'if has("prior_public_boundary_batch_closed") then .prior_public_boundary_batch_closed else false end' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_packet_failed_count="$(jq -r '.packet_integrity_failed_check_count // 999' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_exit_rule_satisfied="$(jq -r 'if has("batch_2_exit_rule_satisfied") then .batch_2_exit_rule_satisfied else false end' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_next_unblocked="$(jq -r 'if has("batch_3_unblocked_for_local_review") then .batch_3_unblocked_for_local_review else false end' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_release_native_handoff_batch_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_JSON")"
review_runtime_boundary_batch_status="$(jq -r '.status // "missing"' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_runtime_overlap_count="$(jq -r '.runtime_overlap_commit_count // 0' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_sharded_count="$(jq -r '.sharded_commit_count // 0' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_sub_batch_count="$(jq -r '.sub_batch_count // 0' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_remaining_review_count="$(jq -r '.remaining_commit_level_review_count // 0' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_entry_rule_satisfied="$(jq -r 'if has("batch_3_entry_rule_satisfied") then .batch_3_entry_rule_satisfied else false end' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_exit_rule_satisfied="$(jq -r 'if has("batch_3_exit_rule_satisfied") then .batch_3_exit_rule_satisfied else true end' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_batch4_unblocked="$(jq -r 'if has("batch_4_unblocked_for_local_review") then .batch_4_unblocked_for_local_review else true end' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_next_sub_batch_id="$(jq -r '.next_sub_batch_id // "missing"' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_boundary_batch_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RUNTIME_BOUNDARY_BATCH_JSON")"
review_runtime_core_semantics_batch_status="$(jq -r '.status // "missing"' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_reviewed_commit_count="$(jq -r '.reviewed_commit_count // 0' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_unresolved_count="$(jq -r '.unresolved_commit_review_count // 999' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_followup_count="$(jq -r '.systemic_runtime_core_boundary_followup_count // 999' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_local_review_complete="$(jq -r 'if has("sub_batch_1_local_review_complete") then .sub_batch_1_local_review_complete else false end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_exit_rule_satisfied="$(jq -r 'if has("sub_batch_1_exit_rule_satisfied") then .sub_batch_1_exit_rule_satisfied else true end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_next_unblocked="$(jq -r 'if has("sub_batch_2_unblocked_for_local_review") then .sub_batch_2_unblocked_for_local_review else false end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_batch3_exit_rule_satisfied="$(jq -r 'if has("batch_3_exit_rule_satisfied") then .batch_3_exit_rule_satisfied else true end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_batch4_unblocked="$(jq -r 'if has("batch_4_unblocked_for_local_review") then .batch_4_unblocked_for_local_review else true end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_next_sub_batch_id="$(jq -r '.next_sub_batch_id // "missing"' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_openra_core_green="$(jq -r 'if has("openra_like_core_all_gates_green") then .openra_like_core_all_gates_green else false end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_core_semantics_batch_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_JSON")"
review_runtime_adapter_online_batch_status="$(jq -r '.status // "missing"' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_reviewed_commit_count="$(jq -r '.reviewed_commit_count // 0' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_unresolved_count="$(jq -r '.unresolved_commit_review_count // 999' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_batch3_reviewed_count="$(jq -r '.batch_3_reviewed_commit_count // 0' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_batch3_remaining_count="$(jq -r '.batch_3_remaining_commit_level_review_count // 999' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_adapter_followup_resolved="$(jq -r 'if has("adapter_path_resolves_runtime_core_source_boundary_followup") then .adapter_path_resolves_runtime_core_source_boundary_followup else false end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_local_review_complete="$(jq -r 'if has("sub_batch_2_local_review_complete") then .sub_batch_2_local_review_complete else false end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_exit_rule_satisfied="$(jq -r 'if has("sub_batch_2_exit_rule_satisfied") then .sub_batch_2_exit_rule_satisfied else false end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_next_unblocked="$(jq -r 'if has("sub_batch_3_unblocked_for_local_review") then .sub_batch_3_unblocked_for_local_review else false end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_batch3_exit_rule_satisfied="$(jq -r 'if has("batch_3_exit_rule_satisfied") then .batch_3_exit_rule_satisfied else true end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_batch4_unblocked="$(jq -r 'if has("batch_4_unblocked_for_local_review") then .batch_4_unblocked_for_local_review else true end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_next_sub_batch_id="$(jq -r '.next_sub_batch_id // "missing"' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_online_green="$(jq -r 'if has("online_offline_adapter_green") then .online_offline_adapter_green else false end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_runtime_adapter_online_batch_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_JSON")"
review_openra_parity_claim_batch_status="$(jq -r '.status // "missing"' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_reviewed_commit_count="$(jq -r '.reviewed_commit_count // 0' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_unresolved_count="$(jq -r '.unresolved_commit_review_count // 999' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_batch3_reviewed_count="$(jq -r '.batch_3_reviewed_commit_count // 0' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_batch3_remaining_count="$(jq -r '.batch_3_remaining_commit_level_review_count // 999' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_local_review_complete="$(jq -r 'if has("sub_batch_3_local_review_complete") then .sub_batch_3_local_review_complete else false end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_exit_rule_satisfied="$(jq -r 'if has("sub_batch_3_exit_rule_satisfied") then .sub_batch_3_exit_rule_satisfied else false end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_next_unblocked="$(jq -r 'if has("sub_batch_4_unblocked_for_local_review") then .sub_batch_4_unblocked_for_local_review else false end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_batch3_exit_rule_satisfied="$(jq -r 'if has("batch_3_exit_rule_satisfied") then .batch_3_exit_rule_satisfied else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_batch4_unblocked="$(jq -r 'if has("batch_4_unblocked_for_local_review") then .batch_4_unblocked_for_local_review else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_next_sub_batch_id="$(jq -r '.next_sub_batch_id // "missing"' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_runtime_claimed="$(jq -r 'if has("openra_runtime_compatibility_claimed") then .openra_runtime_compatibility_claimed else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_replay_claimed="$(jq -r 'if has("openra_replay_compatibility_claimed") then .openra_replay_compatibility_claimed else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_network_claimed="$(jq -r 'if has("openra_network_order_stream_claimed") then .openra_network_order_stream_claimed else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_asset_copy_claimed="$(jq -r 'if has("third_party_asset_copied") then .third_party_asset_copied else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_external_action_performed="$(jq -r 'if has("external_action_performed") then .external_action_performed else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
review_openra_parity_claim_batch_history_rewrite_performed="$(jq -r 'if has("history_rewrite_performed") then .history_rewrite_performed else true end' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_JSON")"
blocker_execution_ledger_status="$(jq -r '.status // "missing"' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON")"
blocker_execution_ledger_needs_collection_count="$(jq -r '.needs_collection_count // 0' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON")"
blocker_execution_ledger_green_evidence_item_count="$(jq -r '.green_evidence_item_count // 0' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON")"
blocker_execution_ledger_consistency_failed_check_count="$(jq -r '.blocker_consistency_failed_check_count // 999' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON")"
blocker_execution_ledger_live_public_exposure_performed="$(jq -r 'if has("live_public_exposure_performed") then .live_public_exposure_performed else true end' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON")"
blocker_execution_ledger_device_capture_performed="$(jq -r 'if has("android_device_capture_performed") then .android_device_capture_performed else true end' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_JSON")"

public_launch_ready="$(jq -r '.public_launch_ready // false' "$PUBLIC_LAUNCH_JSON")"
android_s5_real_device_claimed="$(jq -r '.android_s5_real_device_claimed // false' "$PUBLIC_LAUNCH_JSON")"
blocker_count="$(jq -r '.known_public_launch_blocker_count // ((.known_public_launch_blockers // []) | length)' "$PUBLIC_LAUNCH_JSON")"
blockers_json="$(jq -c '.known_public_launch_blockers // []' "$PUBLIC_LAUNCH_JSON")"

head_commit="$(git -C "$ROOT" rev-parse HEAD)"
origin_commit="$(git -C "$ROOT" rev-parse origin/main)"
ahead_count="$(git -C "$ROOT" rev-list --count origin/main..HEAD)"
dirty_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"
s5_acceptance_kib="$(du -sk "$ROOT/acceptance/S5_native_bevy_device/latest" | awk '{print $1}')"

packet_gate=false
if [[ "$packet_green" == "true" && "$packet_status" == "release_review_packet_integrity_green_with_public_launch_blockers" && "$packet_artifact_count" -ge 128 && "$packet_failed_check_count" -eq 0 ]]; then
  packet_gate=true
fi

runner_gate=false
if [[ "$runner_green" == "true" && "$runner_gate_count" -ge 21 && "$runner_failed_gate_count" -eq 0 ]]; then
  runner_gate=true
fi

public_launch_blocker_gate=false
if [[ "$public_launch_ready" == "false" && "$android_s5_real_device_claimed" == "false" && "$blocker_count" -eq 6 ]]; then
  public_launch_blocker_gate=true
fi

green=false
status="next_execution_plan_blocked"
if [[ "$packet_gate" == "true" && "$runner_gate" == "true" && "$public_launch_blocker_gate" == "true" ]]; then
  green=true
  status="next_execution_plan_green_with_public_launch_blockers"
fi

risks_json="$(jq -nc '[
  {
    id: "local_commit_backlog",
    severity: "high",
    status: "active",
    next_action: "group local commits into reviewable slices before any external push"
  },
  {
    id: "external_public_launch_evidence_gap",
    severity: "blocking",
    status: "blocked_on_real_evidence",
    next_action: "execute the six-row blocker ledger with real external evidence, without granting template or host-side credit"
  },
  {
    id: "documentation_truth_source_drift",
    severity: "medium",
    status: "mitigating",
    next_action: "keep README, RELEASE_READINESS, and development docs synchronized with packet artifacts"
  },
  {
    id: "acceptance_evidence_volume",
    severity: "medium",
    status: "active",
    next_action: "curate large S5/Bevy evidence before handoff"
  },
  {
    id: "first_contact_central_battlefield_readability",
    severity: "high",
    status: "active",
    next_action: "shift from isolated micro-cue shaving to whole-screen product readability"
  }
]')"

work_queue_json="$(jq -nc '[
  {
    id: "whole_screen_first_contact_readability",
    priority: 1,
    scope: "local_product_quality",
    done_when: "unit silhouettes, building hierarchy, terrain grouping, objective focus, and combat flow are readable in the live player screen"
  },
  {
    id: "human_playtest_path",
    priority: 2,
    scope: "local_playtest",
    packet_binding: "bevy-classic-playtest-handoff-packet.human_playtest_task_path",
    done_when: "a tester can start campaign, select units, secure beacon, read command queue, and recover from blocked route"
  },
  {
    id: "truth_source_hygiene",
    priority: 3,
    scope: "local_docs_and_guards",
    done_when: "artifact counts, readiness dates, and no-claim boundaries remain synchronized"
  },
  {
    id: "review_slice_strategy",
    priority: 4,
    scope: "repository_hygiene",
    done_when: "local backlog is grouped and commit-range-manifested into reviewable slices without changing public/external state or history"
  },
  {
    id: "real_external_evidence_collection",
    priority: 5,
    scope: "external_evidence",
    done_when: "the blocker execution ledger reaches zero needs_collection rows and all six validators pass on real non-template artifacts"
  }
]')"

jq -n \
  --arg contract_version "trillionnium_world_next_execution_plan_v1" \
  --arg status "$status" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg head_commit "$head_commit" \
  --arg origin_commit "$origin_commit" \
  --arg packet_status "$packet_status" \
  --arg runner_pid "$runner_pid" \
  --arg runner_screenshot_path "$runner_screenshot_path" \
  --arg readability_review_doc "$READABILITY_REVIEW_DOC_REL" \
  --arg playtest_observation_log_doc "$PLAYTEST_OBSERVATION_LOG_DOC_REL" \
  --arg playtest_runbook_doc "$PLAYTEST_RUNBOOK_DOC_REL" \
  --arg evidence_volume_curation_doc "$EVIDENCE_VOLUME_CURATION_DOC_REL" \
  --arg reviewer_handoff_index_doc "$REVIEWER_HANDOFF_INDEX_DOC_REL" \
  --arg review_slice_strategy_doc "$REVIEW_SLICE_STRATEGY_DOC_REL" \
  --arg review_slice_manifest_doc "$REVIEW_SLICE_MANIFEST_DOC_REL" \
  --arg review_triage_queue_doc "$REVIEW_TRIAGE_QUEUE_DOC_REL" \
  --arg review_primary_owner_plan_doc "$REVIEW_PRIMARY_OWNER_PLAN_DOC_REL" \
  --arg review_release_owner_queue_doc "$REVIEW_RELEASE_OWNER_QUEUE_DOC_REL" \
  --arg review_runtime_owner_queue_doc "$REVIEW_RUNTIME_OWNER_QUEUE_DOC_REL" \
  --arg review_residual_queue_doc "$REVIEW_RESIDUAL_QUEUE_DOC_REL" \
  --arg review_execution_batches_doc "$REVIEW_EXECUTION_BATCHES_DOC_REL" \
  --arg review_public_boundary_batch_doc "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC_REL" \
  --arg review_release_native_handoff_batch_doc "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC_REL" \
  --arg review_runtime_boundary_batch_doc "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC_REL" \
  --arg review_runtime_core_semantics_batch_doc "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC_REL" \
  --arg review_runtime_adapter_online_batch_doc "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC_REL" \
  --arg review_openra_parity_claim_batch_doc "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC_REL" \
  --arg public_launch_blocker_ledger_doc "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC_REL" \
  --arg observation_status "$observation_status" \
  --arg runbook_status "$runbook_status" \
  --arg evidence_volume_status "$evidence_volume_status" \
  --arg reviewer_handoff_index_status "$reviewer_handoff_index_status" \
  --arg review_slice_strategy_status "$review_slice_strategy_status" \
  --arg review_slice_manifest_status "$review_slice_manifest_status" \
  --arg review_triage_queue_status "$review_triage_queue_status" \
  --arg review_primary_owner_plan_status "$review_primary_owner_plan_status" \
  --arg review_release_owner_queue_status "$review_release_owner_queue_status" \
  --arg review_runtime_owner_queue_status "$review_runtime_owner_queue_status" \
  --arg review_residual_queue_status "$review_residual_queue_status" \
  --arg review_residual_queue_scope "$review_residual_queue_scope" \
  --arg review_execution_batches_status "$review_execution_batches_status" \
  --arg review_execution_batches_first_batch_bucket_id "$review_execution_batches_first_batch_bucket_id" \
  --arg review_execution_batches_final_batch_bucket_id "$review_execution_batches_final_batch_bucket_id" \
  --arg review_public_boundary_batch_status "$review_public_boundary_batch_status" \
  --arg review_release_native_handoff_batch_status "$review_release_native_handoff_batch_status" \
  --arg review_runtime_boundary_batch_status "$review_runtime_boundary_batch_status" \
  --arg review_runtime_boundary_batch_next_sub_batch_id "$review_runtime_boundary_batch_next_sub_batch_id" \
  --arg review_runtime_core_semantics_batch_status "$review_runtime_core_semantics_batch_status" \
  --arg review_runtime_core_semantics_batch_next_sub_batch_id "$review_runtime_core_semantics_batch_next_sub_batch_id" \
  --arg review_runtime_adapter_online_batch_status "$review_runtime_adapter_online_batch_status" \
  --arg review_runtime_adapter_online_batch_next_sub_batch_id "$review_runtime_adapter_online_batch_next_sub_batch_id" \
  --arg review_openra_parity_claim_batch_status "$review_openra_parity_claim_batch_status" \
  --arg review_openra_parity_claim_batch_next_sub_batch_id "$review_openra_parity_claim_batch_next_sub_batch_id" \
  --arg blocker_execution_ledger_status "$blocker_execution_ledger_status" \
  --argjson green "$green" \
  --argjson packet_gate "$packet_gate" \
  --argjson packet_artifact_count "$packet_artifact_count" \
  --argjson packet_failed_check_count "$packet_failed_check_count" \
  --argjson packet_check_count "$packet_check_count" \
  --argjson runner_gate "$runner_gate" \
  --argjson runner_gate_count "$runner_gate_count" \
  --argjson runner_failed_gate_count "$runner_failed_gate_count" \
  --argjson recorded_confusion_point_count "$recorded_confusion_point_count" \
  --argjson unrecorded_slot_count "$unrecorded_slot_count" \
  --argjson first_three_confusion_points_recorded "$first_three_confusion_points_recorded" \
  --argjson ready_for_renderer_change_from_human_observation "$ready_for_renderer_change_from_human_observation" \
  --argjson runbook_prompts_bound "$runbook_prompts_bound" \
  --argjson runbook_confusion_triggers_bound "$runbook_confusion_triggers_bound" \
  --argjson runbook_recording_schema_bound "$runbook_recording_schema_bound" \
  --argjson evidence_volume_large_file_count "$evidence_volume_large_file_count" \
  --argjson evidence_volume_deletion_performed "$evidence_volume_deletion_performed" \
  --argjson evidence_volume_archive_movement_performed "$evidence_volume_archive_movement_performed" \
  --argjson reviewer_handoff_index_artifact_count "$reviewer_handoff_index_artifact_count" \
  --argjson reviewer_handoff_index_representative_visual_count "$reviewer_handoff_index_representative_visual_count" \
  --argjson reviewer_handoff_index_upload_performed "$reviewer_handoff_index_upload_performed" \
  --argjson reviewer_handoff_index_publish_performed "$reviewer_handoff_index_publish_performed" \
  --argjson review_slice_count "$review_slice_count" \
  --argjson review_slice_external_action_performed "$review_slice_external_action_performed" \
  --argjson review_slice_manifest_total_ahead_count "$review_slice_manifest_total_ahead_count" \
  --argjson review_slice_manifest_manifested_commit_count "$review_slice_manifest_manifested_commit_count" \
  --argjson review_slice_manifest_unclassified_commit_count "$review_slice_manifest_unclassified_commit_count" \
  --argjson review_slice_manifest_multi_slice_commit_count "$review_slice_manifest_multi_slice_commit_count" \
  --argjson review_slice_manifest_external_action_performed "$review_slice_manifest_external_action_performed" \
  --argjson review_slice_manifest_history_rewrite_performed "$review_slice_manifest_history_rewrite_performed" \
  --argjson review_triage_queue_item_count "$review_triage_queue_item_count" \
  --argjson review_triage_bucket_count "$review_triage_bucket_count" \
  --argjson review_triage_unclassified_bucketed_count "$review_triage_unclassified_bucketed_count" \
  --argjson review_triage_multi_slice_bucketed_count "$review_triage_multi_slice_bucketed_count" \
  --argjson review_triage_manual_review_required "$review_triage_manual_review_required" \
  --argjson review_triage_primary_owner_assignment_required "$review_triage_primary_owner_assignment_required" \
  --argjson review_triage_external_action_performed "$review_triage_external_action_performed" \
  --argjson review_triage_history_rewrite_performed "$review_triage_history_rewrite_performed" \
  --argjson review_primary_owner_plan_owner_bucket_count "$review_primary_owner_plan_owner_bucket_count" \
  --argjson review_primary_owner_plan_assigned_count "$review_primary_owner_plan_assigned_count" \
  --argjson review_primary_owner_plan_assignment_complete "$review_primary_owner_plan_assignment_complete" \
  --argjson review_primary_owner_plan_commit_level_required "$review_primary_owner_plan_commit_level_required" \
  --argjson review_primary_owner_plan_commit_level_required_count "$review_primary_owner_plan_commit_level_required_count" \
  --argjson review_primary_owner_plan_review_order_complete "$review_primary_owner_plan_review_order_complete" \
  --argjson review_primary_owner_plan_external_action_performed "$review_primary_owner_plan_external_action_performed" \
  --argjson review_primary_owner_plan_history_rewrite_performed "$review_primary_owner_plan_history_rewrite_performed" \
  --argjson review_release_owner_queue_item_count "$review_release_owner_queue_item_count" \
  --argjson review_release_owner_queue_lane_bucket_count "$review_release_owner_queue_lane_bucket_count" \
  --argjson review_release_owner_queue_matches_owner_plan "$review_release_owner_queue_matches_owner_plan" \
  --argjson review_release_owner_queue_commit_level_required_count "$review_release_owner_queue_commit_level_required_count" \
  --argjson review_release_owner_queue_truth_source_count "$review_release_owner_queue_truth_source_count" \
  --argjson review_release_owner_queue_external_action_performed "$review_release_owner_queue_external_action_performed" \
  --argjson review_release_owner_queue_history_rewrite_performed "$review_release_owner_queue_history_rewrite_performed" \
  --argjson review_runtime_owner_queue_item_count "$review_runtime_owner_queue_item_count" \
  --argjson review_runtime_owner_queue_lane_bucket_count "$review_runtime_owner_queue_lane_bucket_count" \
  --argjson review_runtime_owner_queue_matches_owner_plan "$review_runtime_owner_queue_matches_owner_plan" \
  --argjson review_runtime_owner_queue_commit_level_required_count "$review_runtime_owner_queue_commit_level_required_count" \
  --argjson review_runtime_owner_queue_boundary_review_count "$review_runtime_owner_queue_boundary_review_count" \
  --argjson review_runtime_owner_queue_zero_count_bucket_count "$review_runtime_owner_queue_zero_count_bucket_count" \
  --argjson review_runtime_owner_queue_external_action_performed "$review_runtime_owner_queue_external_action_performed" \
  --argjson review_runtime_owner_queue_history_rewrite_performed "$review_runtime_owner_queue_history_rewrite_performed" \
  --argjson review_residual_queue_item_count "$review_residual_queue_item_count" \
  --argjson review_residual_queue_bucket_count "$review_residual_queue_bucket_count" \
  --argjson review_residual_queue_matches_owner_plan "$review_residual_queue_matches_owner_plan" \
  --argjson review_residual_queue_coverage_complete "$review_residual_queue_coverage_complete" \
  --argjson review_residual_queue_manual_assignment_count "$review_residual_queue_manual_assignment_count" \
  --argjson review_residual_queue_overlap_resolution_count "$review_residual_queue_overlap_resolution_count" \
  --argjson review_residual_queue_zero_count_bucket_count "$review_residual_queue_zero_count_bucket_count" \
  --argjson review_residual_queue_external_action_performed "$review_residual_queue_external_action_performed" \
  --argjson review_residual_queue_history_rewrite_performed "$review_residual_queue_history_rewrite_performed" \
  --argjson review_execution_batches_owner_batch_count "$review_execution_batches_owner_batch_count" \
  --argjson review_execution_batches_nonempty_batch_count "$review_execution_batches_nonempty_batch_count" \
  --argjson review_execution_batches_reserved_zero_count_batch_count "$review_execution_batches_reserved_zero_count_batch_count" \
  --argjson review_execution_batches_total_queue_item_count "$review_execution_batches_total_queue_item_count" \
  --argjson review_execution_batches_owner_plan_total_commit_count "$review_execution_batches_owner_plan_total_commit_count" \
  --argjson review_execution_batches_coverage_complete "$review_execution_batches_coverage_complete" \
  --argjson review_execution_batches_all_match_plan "$review_execution_batches_all_match_plan" \
  --argjson review_execution_batches_external_action_performed "$review_execution_batches_external_action_performed" \
  --argjson review_execution_batches_history_rewrite_performed "$review_execution_batches_history_rewrite_performed" \
  --argjson review_public_boundary_batch_reviewed_commit_count "$review_public_boundary_batch_reviewed_commit_count" \
  --argjson review_public_boundary_batch_unresolved_count "$review_public_boundary_batch_unresolved_count" \
  --argjson review_public_boundary_batch_exit_rule_satisfied "$review_public_boundary_batch_exit_rule_satisfied" \
  --argjson review_public_boundary_batch_next_unblocked "$review_public_boundary_batch_next_unblocked" \
  --argjson review_public_boundary_batch_blocker_count "$review_public_boundary_batch_blocker_count" \
  --argjson review_public_boundary_batch_needs_collection_count "$review_public_boundary_batch_needs_collection_count" \
  --argjson review_public_boundary_batch_external_action_performed "$review_public_boundary_batch_external_action_performed" \
  --argjson review_public_boundary_batch_history_rewrite_performed "$review_public_boundary_batch_history_rewrite_performed" \
  --argjson review_release_native_handoff_batch_reviewed_commit_count "$review_release_native_handoff_batch_reviewed_commit_count" \
  --argjson review_release_native_handoff_batch_unresolved_count "$review_release_native_handoff_batch_unresolved_count" \
  --argjson review_release_native_handoff_batch_review_group_count "$review_release_native_handoff_batch_review_group_count" \
  --argjson review_release_native_handoff_batch_prior_closed "$review_release_native_handoff_batch_prior_closed" \
  --argjson review_release_native_handoff_batch_packet_failed_count "$review_release_native_handoff_batch_packet_failed_count" \
  --argjson review_release_native_handoff_batch_exit_rule_satisfied "$review_release_native_handoff_batch_exit_rule_satisfied" \
  --argjson review_release_native_handoff_batch_next_unblocked "$review_release_native_handoff_batch_next_unblocked" \
  --argjson review_release_native_handoff_batch_external_action_performed "$review_release_native_handoff_batch_external_action_performed" \
  --argjson review_release_native_handoff_batch_history_rewrite_performed "$review_release_native_handoff_batch_history_rewrite_performed" \
  --argjson review_runtime_boundary_batch_runtime_overlap_count "$review_runtime_boundary_batch_runtime_overlap_count" \
  --argjson review_runtime_boundary_batch_sharded_count "$review_runtime_boundary_batch_sharded_count" \
  --argjson review_runtime_boundary_batch_sub_batch_count "$review_runtime_boundary_batch_sub_batch_count" \
  --argjson review_runtime_boundary_batch_remaining_review_count "$review_runtime_boundary_batch_remaining_review_count" \
  --argjson review_runtime_boundary_batch_entry_rule_satisfied "$review_runtime_boundary_batch_entry_rule_satisfied" \
  --argjson review_runtime_boundary_batch_exit_rule_satisfied "$review_runtime_boundary_batch_exit_rule_satisfied" \
  --argjson review_runtime_boundary_batch_batch4_unblocked "$review_runtime_boundary_batch_batch4_unblocked" \
  --argjson review_runtime_boundary_batch_external_action_performed "$review_runtime_boundary_batch_external_action_performed" \
  --argjson review_runtime_boundary_batch_history_rewrite_performed "$review_runtime_boundary_batch_history_rewrite_performed" \
  --argjson review_runtime_core_semantics_batch_reviewed_commit_count "$review_runtime_core_semantics_batch_reviewed_commit_count" \
  --argjson review_runtime_core_semantics_batch_unresolved_count "$review_runtime_core_semantics_batch_unresolved_count" \
  --argjson review_runtime_core_semantics_batch_followup_count "$review_runtime_core_semantics_batch_followup_count" \
  --argjson review_runtime_core_semantics_batch_local_review_complete "$review_runtime_core_semantics_batch_local_review_complete" \
  --argjson review_runtime_core_semantics_batch_exit_rule_satisfied "$review_runtime_core_semantics_batch_exit_rule_satisfied" \
  --argjson review_runtime_core_semantics_batch_next_unblocked "$review_runtime_core_semantics_batch_next_unblocked" \
  --argjson review_runtime_core_semantics_batch_batch3_exit_rule_satisfied "$review_runtime_core_semantics_batch_batch3_exit_rule_satisfied" \
  --argjson review_runtime_core_semantics_batch_batch4_unblocked "$review_runtime_core_semantics_batch_batch4_unblocked" \
  --argjson review_runtime_core_semantics_batch_openra_core_green "$review_runtime_core_semantics_batch_openra_core_green" \
  --argjson review_runtime_core_semantics_batch_external_action_performed "$review_runtime_core_semantics_batch_external_action_performed" \
  --argjson review_runtime_core_semantics_batch_history_rewrite_performed "$review_runtime_core_semantics_batch_history_rewrite_performed" \
  --argjson review_runtime_adapter_online_batch_reviewed_commit_count "$review_runtime_adapter_online_batch_reviewed_commit_count" \
  --argjson review_runtime_adapter_online_batch_unresolved_count "$review_runtime_adapter_online_batch_unresolved_count" \
  --argjson review_runtime_adapter_online_batch_batch3_reviewed_count "$review_runtime_adapter_online_batch_batch3_reviewed_count" \
  --argjson review_runtime_adapter_online_batch_batch3_remaining_count "$review_runtime_adapter_online_batch_batch3_remaining_count" \
  --argjson review_runtime_adapter_online_batch_adapter_followup_resolved "$review_runtime_adapter_online_batch_adapter_followup_resolved" \
  --argjson review_runtime_adapter_online_batch_local_review_complete "$review_runtime_adapter_online_batch_local_review_complete" \
  --argjson review_runtime_adapter_online_batch_exit_rule_satisfied "$review_runtime_adapter_online_batch_exit_rule_satisfied" \
  --argjson review_runtime_adapter_online_batch_next_unblocked "$review_runtime_adapter_online_batch_next_unblocked" \
  --argjson review_runtime_adapter_online_batch_batch3_exit_rule_satisfied "$review_runtime_adapter_online_batch_batch3_exit_rule_satisfied" \
  --argjson review_runtime_adapter_online_batch_batch4_unblocked "$review_runtime_adapter_online_batch_batch4_unblocked" \
  --argjson review_runtime_adapter_online_batch_online_green "$review_runtime_adapter_online_batch_online_green" \
  --argjson review_runtime_adapter_online_batch_external_action_performed "$review_runtime_adapter_online_batch_external_action_performed" \
  --argjson review_runtime_adapter_online_batch_history_rewrite_performed "$review_runtime_adapter_online_batch_history_rewrite_performed" \
  --argjson review_openra_parity_claim_batch_reviewed_commit_count "$review_openra_parity_claim_batch_reviewed_commit_count" \
  --argjson review_openra_parity_claim_batch_unresolved_count "$review_openra_parity_claim_batch_unresolved_count" \
  --argjson review_openra_parity_claim_batch_batch3_reviewed_count "$review_openra_parity_claim_batch_batch3_reviewed_count" \
  --argjson review_openra_parity_claim_batch_batch3_remaining_count "$review_openra_parity_claim_batch_batch3_remaining_count" \
  --argjson review_openra_parity_claim_batch_local_review_complete "$review_openra_parity_claim_batch_local_review_complete" \
  --argjson review_openra_parity_claim_batch_exit_rule_satisfied "$review_openra_parity_claim_batch_exit_rule_satisfied" \
  --argjson review_openra_parity_claim_batch_next_unblocked "$review_openra_parity_claim_batch_next_unblocked" \
  --argjson review_openra_parity_claim_batch_batch3_exit_rule_satisfied "$review_openra_parity_claim_batch_batch3_exit_rule_satisfied" \
  --argjson review_openra_parity_claim_batch_batch4_unblocked "$review_openra_parity_claim_batch_batch4_unblocked" \
  --argjson review_openra_parity_claim_batch_runtime_claimed "$review_openra_parity_claim_batch_runtime_claimed" \
  --argjson review_openra_parity_claim_batch_replay_claimed "$review_openra_parity_claim_batch_replay_claimed" \
  --argjson review_openra_parity_claim_batch_network_claimed "$review_openra_parity_claim_batch_network_claimed" \
  --argjson review_openra_parity_claim_batch_asset_copy_claimed "$review_openra_parity_claim_batch_asset_copy_claimed" \
  --argjson review_openra_parity_claim_batch_external_action_performed "$review_openra_parity_claim_batch_external_action_performed" \
  --argjson review_openra_parity_claim_batch_history_rewrite_performed "$review_openra_parity_claim_batch_history_rewrite_performed" \
  --argjson blocker_execution_ledger_needs_collection_count "$blocker_execution_ledger_needs_collection_count" \
  --argjson blocker_execution_ledger_green_evidence_item_count "$blocker_execution_ledger_green_evidence_item_count" \
  --argjson blocker_execution_ledger_consistency_failed_check_count "$blocker_execution_ledger_consistency_failed_check_count" \
  --argjson blocker_execution_ledger_live_public_exposure_performed "$blocker_execution_ledger_live_public_exposure_performed" \
  --argjson blocker_execution_ledger_device_capture_performed "$blocker_execution_ledger_device_capture_performed" \
  --argjson public_launch_blocker_gate "$public_launch_blocker_gate" \
  --argjson public_launch_ready "$public_launch_ready" \
  --argjson android_s5_real_device_claimed "$android_s5_real_device_claimed" \
  --argjson blocker_count "$blocker_count" \
  --argjson blockers "$blockers_json" \
  --argjson ahead_count "$ahead_count" \
  --argjson dirty_count "$dirty_count" \
  --argjson s5_acceptance_kib "$s5_acceptance_kib" \
  --argjson risks "$risks_json" \
  --argjson work_queue "$work_queue_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_next_execution_plan",
    green: $green,
    gates: {
      release_review_packet_integrity: $packet_gate,
      live_runner: $runner_gate,
      public_launch_blockers_preserved: $public_launch_blocker_gate
    },
    repository: {
      head_commit: $head_commit,
      origin_main_commit: $origin_commit,
      ahead_count: $ahead_count,
      dirty_count_at_generation: $dirty_count
    },
    release_review_packet: {
      status: $packet_status,
      artifact_count: $packet_artifact_count,
      failed_check_count: $packet_failed_check_count,
      check_count: $packet_check_count
    },
    runner: {
      main_pid: $runner_pid,
      gate_count: $runner_gate_count,
      failed_gate_count: $runner_failed_gate_count,
      screenshot_path: $runner_screenshot_path
    },
    readability_review: {
      doc_path: $readability_review_doc,
      current_product_risk: "central beacon fight has too many similarly bright micro accents competing inside the same objective area",
      next_slice: "product-level silhouette and composition pass around the active center objective before further micro-cue shaving"
    },
    human_playtest_observation: {
      doc_path: $playtest_observation_log_doc,
      artifact_path: "acceptance/S6_public_launch/latest/first-contact-human-playtest-observation-log.json",
      status: $observation_status,
      recorded_confusion_point_count: $recorded_confusion_point_count,
      unrecorded_slot_count: $unrecorded_slot_count,
      first_three_confusion_points_recorded: $first_three_confusion_points_recorded,
      ready_for_renderer_change_from_human_observation: $ready_for_renderer_change_from_human_observation,
      task_ids: ["start_campaign", "select_units", "secure_beacon", "read_command_queue", "recover_blocked_route"],
      no_credit_boundary: "not beta, public launch, Android S5 real-device, production-ready UI, or commercial launch evidence"
    },
    human_playtest_runbook: {
      doc_path: $playtest_runbook_doc,
      artifact_path: "acceptance/S6_public_launch/latest/first-contact-human-playtest-runbook.json",
      status: $runbook_status,
      prompts_bound: $runbook_prompts_bound,
      confusion_triggers_bound: $runbook_confusion_triggers_bound,
      recording_schema_bound: $runbook_recording_schema_bound,
      ready_for_renderer_change_from_human_observation: false,
      no_credit_boundary: "runbook only; not beta, public launch, Android S5 real-device, production-ready UI, commercial launch, or human tester completion evidence"
    },
    evidence_volume_curation: {
      doc_path: $evidence_volume_curation_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-evidence-volume-curation.json",
      status: $evidence_volume_status,
      s5_native_bevy_latest_kib: $s5_acceptance_kib,
      large_file_count: $evidence_volume_large_file_count,
      deletion_performed: $evidence_volume_deletion_performed,
      archive_movement_performed: $evidence_volume_archive_movement_performed,
      no_credit_boundary: "local evidence-volume inventory only; no delete, compress, archive, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, or public-network credit"
    },
    reviewer_handoff_index: {
      doc_path: $reviewer_handoff_index_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-reviewer-handoff-index.json",
      status: $reviewer_handoff_index_status,
      artifact_count: $reviewer_handoff_index_artifact_count,
      representative_visual_count: $reviewer_handoff_index_representative_visual_count,
      upload_performed: $reviewer_handoff_index_upload_performed,
      publish_performed: $reviewer_handoff_index_publish_performed,
      no_credit_boundary: "local reviewer handoff index only; no delete, compress, archive, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, or public-network credit"
    },
    review_slice_strategy: {
      doc_path: $review_slice_strategy_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-slice-strategy.json",
      status: $review_slice_strategy_status,
      review_slice_count: $review_slice_count,
      external_action_performed: $review_slice_external_action_performed,
      no_credit_boundary: "local review slicing only; no push, rebase, reset, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, or public-network credit"
    },
    review_slice_manifest: {
      doc_path: $review_slice_manifest_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-slice-manifest.json",
      status: $review_slice_manifest_status,
      total_ahead_count: $review_slice_manifest_total_ahead_count,
      manifested_commit_count: $review_slice_manifest_manifested_commit_count,
      unclassified_commit_count: $review_slice_manifest_unclassified_commit_count,
      multi_slice_commit_count: $review_slice_manifest_multi_slice_commit_count,
      external_action_performed: $review_slice_manifest_external_action_performed,
      history_rewrite_performed: $review_slice_manifest_history_rewrite_performed,
      no_credit_boundary: "local review-slice commit-range manifest only; no push, rebase, reset, squash, history rewrite, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_triage_queue: {
      doc_path: $review_triage_queue_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-triage-queue.json",
      status: $review_triage_queue_status,
      triage_queue_item_count: $review_triage_queue_item_count,
      triage_bucket_count: $review_triage_bucket_count,
      unclassified_bucketed_count: $review_triage_unclassified_bucketed_count,
      multi_slice_bucketed_count: $review_triage_multi_slice_bucketed_count,
      manual_review_required: $review_triage_manual_review_required,
      primary_owner_assignment_required: $review_triage_primary_owner_assignment_required,
      external_action_performed: $review_triage_external_action_performed,
      history_rewrite_performed: $review_triage_history_rewrite_performed,
      no_credit_boundary: "local review triage queue only; no push, rebase, reset, squash, history rewrite, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_primary_owner_plan: {
      doc_path: $review_primary_owner_plan_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json",
      status: $review_primary_owner_plan_status,
      owner_bucket_count: $review_primary_owner_plan_owner_bucket_count,
      bucket_primary_owner_assigned_count: $review_primary_owner_plan_assigned_count,
      bucket_primary_owner_assignment_complete: $review_primary_owner_plan_assignment_complete,
      commit_level_primary_owner_review_required: $review_primary_owner_plan_commit_level_required,
      commit_level_primary_owner_review_required_count: $review_primary_owner_plan_commit_level_required_count,
      review_order_complete: $review_primary_owner_plan_review_order_complete,
      external_action_performed: $review_primary_owner_plan_external_action_performed,
      history_rewrite_performed: $review_primary_owner_plan_history_rewrite_performed,
      no_credit_boundary: "local review primary-owner plan only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_release_owner_queue: {
      doc_path: $review_release_owner_queue_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json",
      status: $review_release_owner_queue_status,
      primary_owner: "release_truth_and_public_boundary",
      lane_bucket_count: $review_release_owner_queue_lane_bucket_count,
      release_queue_item_count: $review_release_owner_queue_item_count,
      queue_matches_owner_plan: $review_release_owner_queue_matches_owner_plan,
      commit_level_primary_owner_review_required_count: $review_release_owner_queue_commit_level_required_count,
      truth_source_review_item_count: $review_release_owner_queue_truth_source_count,
      external_action_performed: $review_release_owner_queue_external_action_performed,
      history_rewrite_performed: $review_release_owner_queue_history_rewrite_performed,
      no_credit_boundary: "local release/public-boundary owner queue only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_runtime_owner_queue: {
      doc_path: $review_runtime_owner_queue_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json",
      status: $review_runtime_owner_queue_status,
      primary_owner: "rts_runtime_data_boundaries",
      lane_bucket_count: $review_runtime_owner_queue_lane_bucket_count,
      runtime_queue_item_count: $review_runtime_owner_queue_item_count,
      queue_matches_owner_plan: $review_runtime_owner_queue_matches_owner_plan,
      commit_level_primary_owner_review_required_count: $review_runtime_owner_queue_commit_level_required_count,
      runtime_boundary_review_item_count: $review_runtime_owner_queue_boundary_review_count,
      zero_count_bucket_count: $review_runtime_owner_queue_zero_count_bucket_count,
      external_action_performed: $review_runtime_owner_queue_external_action_performed,
      history_rewrite_performed: $review_runtime_owner_queue_history_rewrite_performed,
      no_credit_boundary: "local RTS runtime/data-boundary owner queue only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_residual_queue: {
      doc_path: $review_residual_queue_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-residual-queue.json",
      status: $review_residual_queue_status,
      queue_scope: $review_residual_queue_scope,
      remaining_bucket_count: $review_residual_queue_bucket_count,
      residual_queue_item_count: $review_residual_queue_item_count,
      queue_matches_owner_plan: $review_residual_queue_matches_owner_plan,
      all_owner_queue_coverage_complete: $review_residual_queue_coverage_complete,
      manual_assignment_review_item_count: $review_residual_queue_manual_assignment_count,
      overlap_resolution_review_item_count: $review_residual_queue_overlap_resolution_count,
      zero_count_bucket_count: $review_residual_queue_zero_count_bucket_count,
      external_action_performed: $review_residual_queue_external_action_performed,
      history_rewrite_performed: $review_residual_queue_history_rewrite_performed,
      no_credit_boundary: "local residual owner-resolution queue only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_execution_batches: {
      doc_path: $review_execution_batches_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-execution-batches.json",
      status: $review_execution_batches_status,
      owner_batch_count: $review_execution_batches_owner_batch_count,
      nonempty_batch_count: $review_execution_batches_nonempty_batch_count,
      reserved_zero_count_batch_count: $review_execution_batches_reserved_zero_count_batch_count,
      total_queue_item_count: $review_execution_batches_total_queue_item_count,
      owner_plan_total_commit_count: $review_execution_batches_owner_plan_total_commit_count,
      queue_item_coverage_complete: $review_execution_batches_coverage_complete,
      all_owner_batches_match_plan: $review_execution_batches_all_match_plan,
      first_batch_bucket_id: $review_execution_batches_first_batch_bucket_id,
      final_batch_bucket_id: $review_execution_batches_final_batch_bucket_id,
      external_action_performed: $review_execution_batches_external_action_performed,
      history_rewrite_performed: $review_execution_batches_history_rewrite_performed,
      no_credit_boundary: "local review execution batches only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_public_boundary_batch: {
      doc_path: $review_public_boundary_batch_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-public-boundary-batch.json",
      status: $review_public_boundary_batch_status,
      batch_order: 1,
      bucket_id: "multi_public_boundary_overlap",
      reviewed_commit_count: $review_public_boundary_batch_reviewed_commit_count,
      unresolved_public_boundary_review_count: $review_public_boundary_batch_unresolved_count,
      batch_1_exit_rule_satisfied: $review_public_boundary_batch_exit_rule_satisfied,
      batch_2_unblocked_for_local_review: $review_public_boundary_batch_next_unblocked,
      public_launch_blocker_count: $review_public_boundary_batch_blocker_count,
      blocker_ledger_needs_collection_count: $review_public_boundary_batch_needs_collection_count,
      external_action_performed: $review_public_boundary_batch_external_action_performed,
      history_rewrite_performed: $review_public_boundary_batch_history_rewrite_performed,
      no_credit_boundary: "local public-boundary batch review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_release_native_handoff_batch: {
      doc_path: $review_release_native_handoff_batch_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-native-handoff-batch.json",
      status: $review_release_native_handoff_batch_status,
      batch_order: 2,
      bucket_id: "multi_release_native_handoff_overlap",
      reviewed_commit_count: $review_release_native_handoff_batch_reviewed_commit_count,
      unresolved_release_native_handoff_review_count: $review_release_native_handoff_batch_unresolved_count,
      review_group_count: $review_release_native_handoff_batch_review_group_count,
      prior_public_boundary_batch_closed: $review_release_native_handoff_batch_prior_closed,
      packet_integrity_failed_check_count: $review_release_native_handoff_batch_packet_failed_count,
      batch_2_exit_rule_satisfied: $review_release_native_handoff_batch_exit_rule_satisfied,
      batch_3_unblocked_for_local_review: $review_release_native_handoff_batch_next_unblocked,
      external_action_performed: $review_release_native_handoff_batch_external_action_performed,
      history_rewrite_performed: $review_release_native_handoff_batch_history_rewrite_performed,
      no_credit_boundary: "local release-native handoff batch review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_runtime_boundary_batch: {
      doc_path: $review_runtime_boundary_batch_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      status: $review_runtime_boundary_batch_status,
      batch_order: 3,
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      runtime_overlap_commit_count: $review_runtime_boundary_batch_runtime_overlap_count,
      sharded_commit_count: $review_runtime_boundary_batch_sharded_count,
      sub_batch_count: $review_runtime_boundary_batch_sub_batch_count,
      remaining_commit_level_review_count: $review_runtime_boundary_batch_remaining_review_count,
      batch_3_entry_rule_satisfied: $review_runtime_boundary_batch_entry_rule_satisfied,
      batch_3_exit_rule_satisfied: $review_runtime_boundary_batch_exit_rule_satisfied,
      batch_4_unblocked_for_local_review: $review_runtime_boundary_batch_batch4_unblocked,
      next_sub_batch_id: $review_runtime_boundary_batch_next_sub_batch_id,
      external_action_performed: $review_runtime_boundary_batch_external_action_performed,
      history_rewrite_performed: $review_runtime_boundary_batch_history_rewrite_performed,
      no_credit_boundary: "local runtime-boundary batch 3 shard plan only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
    },
    review_runtime_core_semantics_batch: {
      doc_path: $review_runtime_core_semantics_batch_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-core-semantics-batch.json",
      status: $review_runtime_core_semantics_batch_status,
      batch_order: 3,
      sub_batch_order: 1,
      sub_batch_id: "runtime_core_semantics",
      reviewed_commit_count: $review_runtime_core_semantics_batch_reviewed_commit_count,
      unresolved_commit_review_count: $review_runtime_core_semantics_batch_unresolved_count,
      systemic_runtime_core_boundary_followup_count: $review_runtime_core_semantics_batch_followup_count,
      sub_batch_1_local_review_complete: $review_runtime_core_semantics_batch_local_review_complete,
      sub_batch_1_exit_rule_satisfied: $review_runtime_core_semantics_batch_exit_rule_satisfied,
      sub_batch_2_unblocked_for_local_review: $review_runtime_core_semantics_batch_next_unblocked,
      batch_3_exit_rule_satisfied: $review_runtime_core_semantics_batch_batch3_exit_rule_satisfied,
      batch_4_unblocked_for_local_review: $review_runtime_core_semantics_batch_batch4_unblocked,
      next_sub_batch_id: $review_runtime_core_semantics_batch_next_sub_batch_id,
      openra_like_core_all_gates_green: $review_runtime_core_semantics_batch_openra_core_green,
      external_action_performed: $review_runtime_core_semantics_batch_external_action_performed,
      history_rewrite_performed: $review_runtime_core_semantics_batch_history_rewrite_performed,
      no_credit_boundary: "local runtime-core semantics sub-batch 1 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, OpenRA runtime/replay/network compatibility, multi-node, live-traffic, or public-network credit"
    },
    review_runtime_adapter_online_batch: {
      doc_path: $review_runtime_adapter_online_batch_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-adapter-online-batch.json",
      status: $review_runtime_adapter_online_batch_status,
      batch_order: 3,
      sub_batch_order: 2,
      sub_batch_id: "runtime_adapter_and_online_boundary",
      reviewed_commit_count: $review_runtime_adapter_online_batch_reviewed_commit_count,
      unresolved_commit_review_count: $review_runtime_adapter_online_batch_unresolved_count,
      batch_3_reviewed_commit_count: $review_runtime_adapter_online_batch_batch3_reviewed_count,
      batch_3_remaining_commit_level_review_count: $review_runtime_adapter_online_batch_batch3_remaining_count,
      adapter_path_resolves_runtime_core_source_boundary_followup: $review_runtime_adapter_online_batch_adapter_followup_resolved,
      sub_batch_2_local_review_complete: $review_runtime_adapter_online_batch_local_review_complete,
      sub_batch_2_exit_rule_satisfied: $review_runtime_adapter_online_batch_exit_rule_satisfied,
      sub_batch_3_unblocked_for_local_review: $review_runtime_adapter_online_batch_next_unblocked,
      batch_3_exit_rule_satisfied: $review_runtime_adapter_online_batch_batch3_exit_rule_satisfied,
      batch_4_unblocked_for_local_review: $review_runtime_adapter_online_batch_batch4_unblocked,
      next_sub_batch_id: $review_runtime_adapter_online_batch_next_sub_batch_id,
      online_offline_adapter_green: $review_runtime_adapter_online_batch_online_green,
      external_action_performed: $review_runtime_adapter_online_batch_external_action_performed,
      history_rewrite_performed: $review_runtime_adapter_online_batch_history_rewrite_performed,
      no_credit_boundary: "local runtime-adapter/online sub-batch 2 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, client prediction, rollback netcode, live multiplayer, OpenRA runtime/replay/network compatibility, multi-node, live-traffic, or public-network credit"
    },
    review_openra_parity_claim_batch: {
      doc_path: $review_openra_parity_claim_batch_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-openra-parity-claim-batch.json",
      status: $review_openra_parity_claim_batch_status,
      batch_order: 3,
      sub_batch_order: 3,
      sub_batch_id: "openra_parity_and_claim_boundary",
      reviewed_commit_count: $review_openra_parity_claim_batch_reviewed_commit_count,
      unresolved_commit_review_count: $review_openra_parity_claim_batch_unresolved_count,
      batch_3_reviewed_commit_count: $review_openra_parity_claim_batch_batch3_reviewed_count,
      batch_3_remaining_commit_level_review_count: $review_openra_parity_claim_batch_batch3_remaining_count,
      sub_batch_3_local_review_complete: $review_openra_parity_claim_batch_local_review_complete,
      sub_batch_3_exit_rule_satisfied: $review_openra_parity_claim_batch_exit_rule_satisfied,
      sub_batch_4_unblocked_for_local_review: $review_openra_parity_claim_batch_next_unblocked,
      batch_3_exit_rule_satisfied: $review_openra_parity_claim_batch_batch3_exit_rule_satisfied,
      batch_4_unblocked_for_local_review: $review_openra_parity_claim_batch_batch4_unblocked,
      next_sub_batch_id: $review_openra_parity_claim_batch_next_sub_batch_id,
      openra_runtime_compatibility_claimed: $review_openra_parity_claim_batch_runtime_claimed,
      openra_replay_compatibility_claimed: $review_openra_parity_claim_batch_replay_claimed,
      openra_network_order_stream_claimed: $review_openra_parity_claim_batch_network_claimed,
      third_party_asset_copied: $review_openra_parity_claim_batch_asset_copy_claimed,
      external_action_performed: $review_openra_parity_claim_batch_external_action_performed,
      history_rewrite_performed: $review_openra_parity_claim_batch_history_rewrite_performed,
      no_credit_boundary: "local OpenRA parity/claim sub-batch 3 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, OpenRA runtime/replay/network/binary/headless compatibility, OpenRA engine/full-engine/pixel-perfect/Westwood asset parity, third-party asset copy, multi-node, live-traffic, or public-network credit"
    },
    public_launch_blocker_execution_ledger: {
      doc_path: $public_launch_blocker_ledger_doc,
      artifact_path: "acceptance/S6_public_launch/latest/trillionnium-world-public-launch-blocker-execution-ledger.json",
      status: $blocker_execution_ledger_status,
      needs_collection_count: $blocker_execution_ledger_needs_collection_count,
      green_evidence_item_count: $blocker_execution_ledger_green_evidence_item_count,
      blocker_consistency_failed_check_count: $blocker_execution_ledger_consistency_failed_check_count,
      live_public_exposure_performed: $blocker_execution_ledger_live_public_exposure_performed,
      android_device_capture_performed: $blocker_execution_ledger_device_capture_performed,
      no_credit_boundary: "local blocker execution ledger only; no public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, public-network, live-ingestion, or device-capture credit"
    },
    public_launch: {
      public_launch_ready: $public_launch_ready,
      android_s5_real_device_claimed: $android_s5_real_device_claimed,
      blocker_count: $blocker_count,
      blockers: $blockers
    },
    evidence_volume: {
      s5_native_bevy_latest_kib: $s5_acceptance_kib
    },
    risks: $risks,
    work_queue: $work_queue,
    operating_rule: "prefer whole-screen product quality and truth-source guards; do not shrink already-gated micro cues without a fresh screenshot-visible issue"
  }' >"$SUMMARY_JSON"

jq -e '
  .contract_version == "trillionnium_world_next_execution_plan_v1"
  and .green == true
  and .human_playtest_observation.first_three_confusion_points_recorded == false
  and .human_playtest_observation.ready_for_renderer_change_from_human_observation == false
  and .human_playtest_runbook.ready_for_renderer_change_from_human_observation == false
  and .human_playtest_runbook.prompts_bound == true
  and .human_playtest_runbook.confusion_triggers_bound == true
  and .human_playtest_runbook.recording_schema_bound == true
  and .evidence_volume_curation.large_file_count > 100
  and .evidence_volume_curation.deletion_performed == false
  and .evidence_volume_curation.archive_movement_performed == false
  and .reviewer_handoff_index.artifact_count == 38
  and .reviewer_handoff_index.representative_visual_count == 5
  and .reviewer_handoff_index.upload_performed == false
  and .reviewer_handoff_index.publish_performed == false
  and .review_slice_strategy.review_slice_count == 6
  and .review_slice_strategy.external_action_performed == false
  and .review_slice_manifest.total_ahead_count >= 1
  and ((.review_slice_manifest.manifested_commit_count + .review_slice_manifest.unclassified_commit_count) == .review_slice_manifest.total_ahead_count)
  and .review_slice_manifest.external_action_performed == false
  and .review_slice_manifest.history_rewrite_performed == false
  and .review_triage_queue.triage_bucket_count == 11
  and .review_triage_queue.unclassified_bucketed_count == .review_slice_manifest.unclassified_commit_count
  and .review_triage_queue.multi_slice_bucketed_count == .review_slice_manifest.multi_slice_commit_count
  and .review_triage_queue.manual_review_required == true
  and .review_triage_queue.primary_owner_assignment_required == true
  and .review_triage_queue.external_action_performed == false
  and .review_triage_queue.history_rewrite_performed == false
  and .review_primary_owner_plan.owner_bucket_count == 11
  and .review_primary_owner_plan.bucket_primary_owner_assigned_count == 11
  and .review_primary_owner_plan.bucket_primary_owner_assignment_complete == true
  and .review_primary_owner_plan.commit_level_primary_owner_review_required == true
  and .review_primary_owner_plan.commit_level_primary_owner_review_required_count >= 1
  and .review_primary_owner_plan.review_order_complete == true
  and .review_primary_owner_plan.external_action_performed == false
  and .review_primary_owner_plan.history_rewrite_performed == false
  and .review_release_owner_queue.primary_owner == "release_truth_and_public_boundary"
  and .review_release_owner_queue.lane_bucket_count == 4
  and .review_release_owner_queue.release_queue_item_count >= 1
  and .review_release_owner_queue.queue_matches_owner_plan == true
  and .review_release_owner_queue.commit_level_primary_owner_review_required_count >= 1
  and .review_release_owner_queue.truth_source_review_item_count >= 1
  and .review_release_owner_queue.external_action_performed == false
  and .review_release_owner_queue.history_rewrite_performed == false
  and .review_runtime_owner_queue.primary_owner == "rts_runtime_data_boundaries"
  and .review_runtime_owner_queue.lane_bucket_count == 3
  and .review_runtime_owner_queue.runtime_queue_item_count >= 1
  and .review_runtime_owner_queue.queue_matches_owner_plan == true
  and .review_runtime_owner_queue.commit_level_primary_owner_review_required_count >= 1
  and .review_runtime_owner_queue.runtime_boundary_review_item_count >= 1
  and .review_runtime_owner_queue.zero_count_bucket_count >= 1
  and .review_runtime_owner_queue.external_action_performed == false
  and .review_runtime_owner_queue.history_rewrite_performed == false
  and .review_residual_queue.queue_scope == "remaining_owner_resolution"
  and .review_residual_queue.remaining_bucket_count == 4
  and .review_residual_queue.residual_queue_item_count >= 1
  and .review_residual_queue.queue_matches_owner_plan == true
  and .review_residual_queue.all_owner_queue_coverage_complete == true
  and .review_residual_queue.manual_assignment_review_item_count >= 1
  and .review_residual_queue.overlap_resolution_review_item_count >= 1
  and .review_residual_queue.zero_count_bucket_count >= 1
  and .review_residual_queue.external_action_performed == false
  and .review_residual_queue.history_rewrite_performed == false
  and .review_execution_batches.owner_batch_count == 11
  and .review_execution_batches.nonempty_batch_count >= 9
  and .review_execution_batches.reserved_zero_count_batch_count >= 1
  and .review_execution_batches.total_queue_item_count == .review_execution_batches.owner_plan_total_commit_count
  and .review_execution_batches.queue_item_coverage_complete == true
  and .review_execution_batches.all_owner_batches_match_plan == true
  and .review_execution_batches.first_batch_bucket_id == "multi_public_boundary_overlap"
  and .review_execution_batches.final_batch_bucket_id == "multi_manual_overlap"
  and .review_execution_batches.external_action_performed == false
  and .review_execution_batches.history_rewrite_performed == false
  and .review_public_boundary_batch.status == "review_public_boundary_batch_1_ready"
  and .review_public_boundary_batch.batch_order == 1
  and .review_public_boundary_batch.bucket_id == "multi_public_boundary_overlap"
  and .review_public_boundary_batch.reviewed_commit_count == 6
  and .review_public_boundary_batch.unresolved_public_boundary_review_count == 0
  and .review_public_boundary_batch.batch_1_exit_rule_satisfied == true
  and .review_public_boundary_batch.batch_2_unblocked_for_local_review == true
  and .review_public_boundary_batch.public_launch_blocker_count == 6
  and .review_public_boundary_batch.blocker_ledger_needs_collection_count == 6
  and .review_public_boundary_batch.external_action_performed == false
  and .review_public_boundary_batch.history_rewrite_performed == false
  and .review_release_native_handoff_batch.status == "review_release_native_handoff_batch_2_ready"
  and .review_release_native_handoff_batch.batch_order == 2
  and .review_release_native_handoff_batch.bucket_id == "multi_release_native_handoff_overlap"
  and .review_release_native_handoff_batch.reviewed_commit_count == 29
  and .review_release_native_handoff_batch.unresolved_release_native_handoff_review_count == 0
  and .review_release_native_handoff_batch.review_group_count == 4
  and .review_release_native_handoff_batch.prior_public_boundary_batch_closed == true
  and .review_release_native_handoff_batch.packet_integrity_failed_check_count == 0
  and .review_release_native_handoff_batch.batch_2_exit_rule_satisfied == true
  and .review_release_native_handoff_batch.batch_3_unblocked_for_local_review == true
  and .review_release_native_handoff_batch.external_action_performed == false
  and .review_release_native_handoff_batch.history_rewrite_performed == false
  and .review_runtime_boundary_batch.status == "review_runtime_boundary_batch_3_sharded"
  and .review_runtime_boundary_batch.batch_order == 3
  and .review_runtime_boundary_batch.bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .review_runtime_boundary_batch.runtime_overlap_commit_count == 273
  and .review_runtime_boundary_batch.sharded_commit_count == 273
  and .review_runtime_boundary_batch.sub_batch_count == 8
  and .review_runtime_boundary_batch.remaining_commit_level_review_count == 273
  and .review_runtime_boundary_batch.batch_3_entry_rule_satisfied == true
  and .review_runtime_boundary_batch.batch_3_exit_rule_satisfied == false
  and .review_runtime_boundary_batch.batch_4_unblocked_for_local_review == false
  and .review_runtime_boundary_batch.next_sub_batch_id == "runtime_core_semantics"
  and .review_runtime_boundary_batch.external_action_performed == false
  and .review_runtime_boundary_batch.history_rewrite_performed == false
  and .review_runtime_core_semantics_batch.status == "review_runtime_core_semantics_sub_batch_1_reviewed_with_boundary_followup"
  and .review_runtime_core_semantics_batch.batch_order == 3
  and .review_runtime_core_semantics_batch.sub_batch_order == 1
  and .review_runtime_core_semantics_batch.sub_batch_id == "runtime_core_semantics"
  and .review_runtime_core_semantics_batch.reviewed_commit_count == 55
  and .review_runtime_core_semantics_batch.unresolved_commit_review_count == 0
  and .review_runtime_core_semantics_batch.systemic_runtime_core_boundary_followup_count == 1
  and .review_runtime_core_semantics_batch.sub_batch_1_local_review_complete == true
  and .review_runtime_core_semantics_batch.sub_batch_1_exit_rule_satisfied == false
  and .review_runtime_core_semantics_batch.sub_batch_2_unblocked_for_local_review == true
  and .review_runtime_core_semantics_batch.batch_3_exit_rule_satisfied == false
  and .review_runtime_core_semantics_batch.batch_4_unblocked_for_local_review == false
  and .review_runtime_core_semantics_batch.next_sub_batch_id == "runtime_adapter_and_online_boundary"
  and .review_runtime_core_semantics_batch.openra_like_core_all_gates_green == true
  and .review_runtime_core_semantics_batch.external_action_performed == false
  and .review_runtime_core_semantics_batch.history_rewrite_performed == false
  and .review_runtime_adapter_online_batch.status == "review_runtime_adapter_online_sub_batch_2_reviewed"
  and .review_runtime_adapter_online_batch.batch_order == 3
  and .review_runtime_adapter_online_batch.sub_batch_order == 2
  and .review_runtime_adapter_online_batch.sub_batch_id == "runtime_adapter_and_online_boundary"
  and .review_runtime_adapter_online_batch.reviewed_commit_count == 57
  and .review_runtime_adapter_online_batch.unresolved_commit_review_count == 0
  and .review_runtime_adapter_online_batch.batch_3_reviewed_commit_count == 112
  and .review_runtime_adapter_online_batch.batch_3_remaining_commit_level_review_count == 161
  and .review_runtime_adapter_online_batch.adapter_path_resolves_runtime_core_source_boundary_followup == true
  and .review_runtime_adapter_online_batch.sub_batch_2_local_review_complete == true
  and .review_runtime_adapter_online_batch.sub_batch_2_exit_rule_satisfied == true
  and .review_runtime_adapter_online_batch.sub_batch_3_unblocked_for_local_review == true
  and .review_runtime_adapter_online_batch.batch_3_exit_rule_satisfied == false
  and .review_runtime_adapter_online_batch.batch_4_unblocked_for_local_review == false
  and .review_runtime_adapter_online_batch.next_sub_batch_id == "openra_parity_and_claim_boundary"
  and .review_runtime_adapter_online_batch.online_offline_adapter_green == true
  and .review_runtime_adapter_online_batch.external_action_performed == false
  and .review_runtime_adapter_online_batch.history_rewrite_performed == false
  and .review_openra_parity_claim_batch.status == "review_openra_parity_claim_sub_batch_3_reviewed"
  and .review_openra_parity_claim_batch.batch_order == 3
  and .review_openra_parity_claim_batch.sub_batch_order == 3
  and .review_openra_parity_claim_batch.sub_batch_id == "openra_parity_and_claim_boundary"
  and .review_openra_parity_claim_batch.reviewed_commit_count == 35
  and .review_openra_parity_claim_batch.unresolved_commit_review_count == 0
  and .review_openra_parity_claim_batch.batch_3_reviewed_commit_count == 147
  and .review_openra_parity_claim_batch.batch_3_remaining_commit_level_review_count == 126
  and .review_openra_parity_claim_batch.sub_batch_3_local_review_complete == true
  and .review_openra_parity_claim_batch.sub_batch_3_exit_rule_satisfied == true
  and .review_openra_parity_claim_batch.sub_batch_4_unblocked_for_local_review == true
  and .review_openra_parity_claim_batch.batch_3_exit_rule_satisfied == false
  and .review_openra_parity_claim_batch.batch_4_unblocked_for_local_review == false
  and .review_openra_parity_claim_batch.next_sub_batch_id == "first_contact_rts_data_extraction"
  and .review_openra_parity_claim_batch.openra_runtime_compatibility_claimed == false
  and .review_openra_parity_claim_batch.openra_replay_compatibility_claimed == false
  and .review_openra_parity_claim_batch.openra_network_order_stream_claimed == false
  and .review_openra_parity_claim_batch.third_party_asset_copied == false
  and .review_openra_parity_claim_batch.external_action_performed == false
  and .review_openra_parity_claim_batch.history_rewrite_performed == false
  and .public_launch_blocker_execution_ledger.needs_collection_count == 6
  and .public_launch_blocker_execution_ledger.green_evidence_item_count == 0
  and .public_launch_blocker_execution_ledger.blocker_consistency_failed_check_count == 0
  and .public_launch_blocker_execution_ledger.live_public_exposure_performed == false
  and .public_launch_blocker_execution_ledger.android_device_capture_performed == false
  and .public_launch.public_launch_ready == false
  and .public_launch.android_s5_real_device_claimed == false
' "$SUMMARY_JSON" >/dev/null

{
  printf '# Trillionnium World Next Execution Plan\n\n'
  printf -- '- status: `%s`\n' "$status"
  printf -- '- green: `%s`\n' "$green"
  printf -- '- local commits ahead of origin/main: `%s`\n' "$ahead_count"
  printf -- '- packet artifacts: `%s`, failed checks: `%s`\n' "$packet_artifact_count" "$packet_failed_check_count"
  printf -- '- public launch ready: `%s`\n' "$public_launch_ready"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$android_s5_real_device_claimed"
  printf -- '- readability review: `%s`\n\n' "$READABILITY_REVIEW_DOC_REL"
  printf -- '- playtest observation log: `%s`\n\n' "$PLAYTEST_OBSERVATION_LOG_DOC_REL"
  printf -- '- playtest runbook: `%s`\n\n' "$PLAYTEST_RUNBOOK_DOC_REL"
  printf -- '- evidence-volume curation: `%s`\n\n' "$EVIDENCE_VOLUME_CURATION_DOC_REL"
  printf -- '- reviewer handoff index: `%s`\n\n' "$REVIEWER_HANDOFF_INDEX_DOC_REL"
  printf -- '- review-slice strategy: `%s`\n\n' "$REVIEW_SLICE_STRATEGY_DOC_REL"
  printf -- '- review-slice manifest: `%s`\n\n' "$REVIEW_SLICE_MANIFEST_DOC_REL"
  printf -- '- review triage queue: `%s`\n\n' "$REVIEW_TRIAGE_QUEUE_DOC_REL"
  printf -- '- review primary-owner plan: `%s`\n\n' "$REVIEW_PRIMARY_OWNER_PLAN_DOC_REL"
  printf -- '- release/public-boundary owner queue: `%s`\n\n' "$REVIEW_RELEASE_OWNER_QUEUE_DOC_REL"
  printf -- '- RTS runtime/data-boundary owner queue: `%s`\n\n' "$REVIEW_RUNTIME_OWNER_QUEUE_DOC_REL"
  printf -- '- residual owner-resolution queue: `%s`\n\n' "$REVIEW_RESIDUAL_QUEUE_DOC_REL"
  printf -- '- review execution batches: `%s`\n\n' "$REVIEW_EXECUTION_BATCHES_DOC_REL"
  printf -- '- public-boundary batch review: `%s`\n\n' "$REVIEW_PUBLIC_BOUNDARY_BATCH_DOC_REL"
  printf -- '- release-native handoff batch review: `%s`\n\n' "$REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_DOC_REL"
  printf -- '- runtime-boundary batch review: `%s`\n\n' "$REVIEW_RUNTIME_BOUNDARY_BATCH_DOC_REL"
  printf -- '- runtime-core semantics batch review: `%s`\n\n' "$REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_DOC_REL"
  printf -- '- runtime-adapter/online batch review: `%s`\n\n' "$REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_DOC_REL"
  printf -- '- OpenRA parity/claim batch review: `%s`\n\n' "$REVIEW_OPENRA_PARITY_CLAIM_BATCH_DOC_REL"
  printf -- '- public-launch blocker execution ledger: `%s`\n\n' "$PUBLIC_LAUNCH_BLOCKER_LEDGER_DOC_REL"
  printf '## Risks\n\n'
  jq -r '.risks[] | "- `\(.id)`: \(.next_action)"' "$SUMMARY_JSON"
  printf '\n## Work Queue\n\n'
  jq -r '.work_queue[] | "- \(.priority). `\(.id)`: \(.done_when)"' "$SUMMARY_JSON"
  printf '\n## Public Launch Blockers\n\n'
  jq -r '.public_launch.blockers[] | "- `\(.)`"' "$SUMMARY_JSON"
} >"$SUMMARY_MD"

if [[ "$green" == "true" ]]; then
  printf 'TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_JSON"
else
  printf 'TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_BLOCKED %s\n' "$SUMMARY_JSON" >&2
  exit 1
fi
